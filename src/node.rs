use logos::Span;
use crate::token::TokenKind;

#[derive(Debug)]
pub enum Data {
    /* These are indices for the token vector */
    Identifier(usize),
    String(usize),
    Integer(usize),
    Float(usize),

    BinaryOp {
        lhs: Box<Node>,
        op: TokenKind,
        rhs: Box<Node>,
    },
    UnaryOp {
        op: TokenKind,
        target: Box<Node>,
    }
}

#[derive(Debug)]
pub struct Node {
    pub(crate) data: Data,
    pub(crate) span: Span,
}