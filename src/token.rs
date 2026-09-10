use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
pub enum TokenKind {
    #[token("+")]
    Add,
    #[token("+=")]
    AddAssign,
    #[token("++")]
    Adds,
    #[token("-")]
    Sub,
    #[token("-=")]
    SubAssign,
    #[token("--")]
    Subs,
    #[token("*")]
    Mul,
    #[token("*=")]
    MulAssign,
    #[token("/")]
    Div,
    #[token("/=")]
    DivAssign,

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

    #[regex("[a-zA-Z]+")]
    Identifier,
}