use bigdecimal::BigDecimal;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Statement {
    Assign(String, Expr),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Expr {
    Var(String),
    FreeVar(String),
    Num(BigDecimal),
    Call(String, Vec<Expr>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Type {
    Float,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Block {
    Fun(String, Vec<(String,Type)>, Vec<Statement>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Program(pub Vec<Block>);
