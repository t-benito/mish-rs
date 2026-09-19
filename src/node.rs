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
        ty: Box<ParseType>,
        value: Box<Node>
    },
    FnDeclaration {
        name: usize,
        params: Option< Vec<Node> >,
        return_type: Box<ParseType>,
        body: Vec<Node>,
    },
    FnParameter {
        ty: Box<ParseType>,
        name: usize
    }
}

#[derive(Debug)]
pub struct Node {
    pub data: Data,
    pub span: Span,
}

#[derive(Debug)]
pub enum ParseTypeBase {
    Char,       // 8 bit
    Void,

    Byte,       // 8 bit
    Short,      // 16 bit
    Int,        // 32 bit

    Float,      // 32 bit floating

    Pointer(Box<ParseType>),
    Array(Box<ParseType>, Node),
    Function {
        params: Vec<ParseType>,
        return_type: Box<ParseType>,
        variadic: bool,
    },

    Struct(Span),
    Union(Span),
    Enum(Span),

    Alias(Span),
}

#[derive(Debug)]
pub struct ParseType {
    pub base: ParseTypeBase,

    pub unsigned: bool,
    pub long: bool,
}