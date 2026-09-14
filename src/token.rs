use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\r]+")]
pub enum Token {
    #[token("=")]
    Assign,
    #[token("+")]
    Add,
    #[token("+=")]
    AddAssign,
    #[token("++")]
    Increment,
    #[token("-")]
    Sub,
    #[token("-=")]
    SubAssign,
    #[token("--")]
    Decrement,
    #[token("*")]
    Mul,
    #[token("*=")]
    MulAssign,
    #[token("/")]
    Div,
    #[token("/=")]
    DivAssign,

    #[token(";")]
    Semicolon,

    #[token("(")]
    OpenParen,
    #[token(")")]
    CloseParen,
    #[token("[")]
    OpenBracket,
    #[token("]")]
    CloseBracket,
    #[token("{")]
    OpenBody,
    #[token("}")]
    CloseBody,

    #[token("let")]
    KwLet,
    #[token("const")]
    KwConst,
    #[token("fn")]
    KwFn,
    #[token("if")]
    KwIf,

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,
    #[regex("[0-9]+")]
    Number,
}