use anyhow::bail;
use bigdecimal::{BigDecimal,Num};
use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, digit1, multispace0},
    combinator::{all_consuming, cut, opt, map, map_res, recognize, verify, value},
    multi::{many0, separated_list0},
    sequence::{pair, preceded, separated_pair, terminated},
};

use crate::ast::{Block, Expr, Program, Statement, Type};

/////////////
// Low-level
/////////////

fn sym(s: &'static str) -> impl Fn(&str) -> IResult<&str, ()> {
    move |input| {
        let (input, _) = tag(s)(input)?;
        let (input, _) = multispace0(input)?;
        Ok((input, ()))
    }
}

fn decimal(input: &str) -> IResult<&str, &str> {
    terminated(recognize(pair(digit1, opt(pair(tag("."), digit1)))), multispace0)(input)
}

fn var(input: &str) -> IResult<&str, &str> {
    terminated(recognize(pair(alt((alpha1, tag("_"))), many0(alt((alphanumeric1, tag("_")))))), multispace0)(input)
}

fn var_owned(input: &str) -> IResult<&str, String> {
    map(var, str::to_owned)(input)
}

fn keyword(k: &'static str) -> impl Fn(&str) -> IResult<&str, ()> {
    move |input| {
        value((), verify(var, |s:&str|s==k))(input)
    }
}

/////////////
// Expressions
/////////////

fn str_to_numexpr(s: &str) -> anyhow::Result<Expr> {
    Ok(Expr::Num(BigDecimal::from_str_radix(s,10)?))
}

fn call(input: &str) -> IResult<&str, Expr> {
    let (input, f) = var_owned(input)?;
    let (input, _) = sym("(")(input)?;
    let (input, args) = cut(separated_list0(sym(","), expr))(input)?;
    let (input, _) = sym(")")(input)?;
    let res = Expr::Call(f, args);
    Ok((input, res))
}

fn atom(input: &str) -> IResult<&str, Expr> {
    alt((
        call,
        map_res(decimal, str_to_numexpr),
        map(preceded(tag("'"), cut(var_owned)), Expr::FreeVar),
        map(var_owned, Expr::Var),
    ))(input)
}

fn expr(input: &str) -> IResult<&str, Expr> {
    let (input, first) = atom(input)?;
    let (input, rest) = many0(preceded(sym("+"), atom))(input)?;
    let res = rest.into_iter().fold(first, |a,b|Expr::Plus(Box::new(a), Box::new(b)));
    Ok((input, res))
}

/////////////
// Statements
/////////////

fn assign(input: &str) -> IResult<&str, Statement> {
    let (input, x) = var_owned(input)?;
    let (input, ()) = sym("=")(input)?;
    let (input, e) = cut(expr)(input)?;
    let res = Statement::Assign(x, e);
    Ok((input, res))
}

fn statement(input: &str) -> IResult<&str, Statement> {
    terminated(assign, cut(sym(";")))(input)
}

/////////////
// Types
/////////////

fn typ(input: &str) -> IResult<&str, Type> {
    value(Type::Float, keyword("float"))(input)
}

/////////////
// Blocks
/////////////

fn arg(input: &str) -> IResult<&str, (String,Type)> {
    separated_pair(var_owned, sym(":"), typ)(input)
}

fn fun(input: &str) -> IResult<&str, Block> {
    let (input, name) = var_owned(input)?;
    let (input, ()) = sym("(")(input)?;
    let (input, args) = separated_list0(sym(","), arg)(input)?;
    let (input, ()) = sym(")")(input)?;
    let (input, ()) = sym("{")(input)?;
    let (input, stmts) = many0(statement)(input)?;
    let (input, ()) = sym("}")(input)?;
    Ok((input, Block::Fun(name, args, stmts)))
}

fn block(input: &str) -> IResult<&str, Block> {
    preceded(keyword("fn"), cut(fun))(input)
}


/////////////
// Programs
/////////////

pub fn program(input: &str) -> anyhow::Result<Program> {
    match map(all_consuming(preceded(multispace0, many0(block))), Program)(input) {
        Ok((_, res)) => Ok(res),
        Err(nom::Err::Incomplete(_)) => bail!("Incomplete"),
        Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => {
            let lines:Vec<_> = input[..input.len() - e.input.len()].lines().collect();
            if lines.len() == 0 {
                bail!("Parse error at start: {:?}", e.code);
            }
            bail!("Parse error on line {} char {}: {:?}", lines.len(), lines[lines.len()-1].len()+1, e.code);
        }
    }
}

