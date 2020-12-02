use bigdecimal::BigDecimal;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Statement {
    Assign(bool, String, Expr),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Expr {
    Var(String),
    Num(BigDecimal),
    Plus(Box<Expr>, Box<Expr>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Type {
    Float,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Block {
    Space(Vec<(String,Type)>, Vec<Statement>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Program(pub Vec<Block>);