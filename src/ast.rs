use crate::lexer::Span;

#[derive(Debug)]
pub struct Program {
    pub rules: Vec<Rule>,
}

#[derive(Debug)]
pub enum RuleType {
    BeginRule,
    DefaultRule,
    EndRule
}

#[derive(Debug)]
pub struct Rule {
    pub rule_type: RuleType,
    pub statements: Vec<Statement> 
}

#[derive(Debug)]
pub struct Statement {
    pub expressions: Vec<Expr>
}

#[derive(Debug)]
pub struct Expr {
    pub span: Span,
    pub node: Expr_,
}

#[derive(Debug)]
pub enum Expr_ {
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Var(String),
    Assign(String, Box<Expr>),
    Print(Box<Expr>),
    Integer(i64),
    Text(String)
}

