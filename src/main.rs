fn main() {
    println!("Hello, world!");
}

pub struct Scanner {
    pub source: String,
}

pub enum Token {
    Ident(String),
    Assign,
    Define,
    Equal,
    LParen,
    RParen,
    Add,
    AddAssign,
    Sub,
    SubAssign,
    Mul,
    MulAssign,
    Div,
    DivAssign,
    Mod,
    Return,
    Auto,
    Exp,
    ExpAssign,
    LBrace,
    RBrace,
    Number(f64),
}
