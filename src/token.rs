use logos::{Logos, Span};

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\n\r]+")]
pub enum TokenKind {
    #[token("==")]
    Equal,
    #[token("!=")]
    NotEqual,
    #[token(">")]
    Greater,
    #[token(">=")]
    GreaterEqual,
    #[token("<")]
    Lesser,
    #[token("<=")]
    LesserEqual,
    
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
    IntegerLiteral,
    #[regex(r"-?[0-9]+\.[0-9]+")]
    FloatLiteral,
    #[regex(r#""[^"]*""#)]
    StringLiteral,

    EOF,
}

#[derive(Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}