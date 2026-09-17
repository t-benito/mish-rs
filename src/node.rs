use logos::Span;
use crate::token::TokenKind;

#[derive(Debug)]
pub enum Data {
    // Usizes here are "pointers" to values on the token vector
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
    },
    VarDeclaration {
        name: usize,
        mutable: bool,
        ty: Box<Node>,
        value: Box<Node>
    },
    FnDeclaration {
        name: usize,
        params: Option< Vec<Node> >,
        return_type: Box<Node>,
        body: Vec<Node>,
    },
    FnParameter {
        ty: Box<Node>,
        name: usize
    }
}

#[derive(Debug)]
pub struct Node {
    pub(crate) data: Data,
    pub(crate) span: Span,
}