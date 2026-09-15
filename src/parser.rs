use logos::Span;
use crate::error::{Error, ErrorKind, ErrorLogger, ParseError};
use crate::error::ErrorKind::Parse;
use crate::node::{Data, Node};
use crate::token::{Token, TokenKind};

pub struct Parser<'a> {
    tokens: Vec<Token>,
    source: &'a str,
    logger: ErrorLogger<'a>,

    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token>, source: &'a str) -> Self {
        Self {
            tokens,
            source,
            logger: ErrorLogger::new(source),
            current: 0,
        }
    }
    
    fn advance(&mut self) -> Token {
        if self.is_at_end() {
            return self.tokens.last().unwrap().clone();
        }
        let token = self.tokens.get(self.current).unwrap();
        self.current += 1;
        token.clone()
    }
    // fn previous(&self) -> Option<&Token> {
    //     if self.current == 0 {
    //         return None;
    //     }
    //     self.tokens.get(self.current - 1)
    // }
    fn expect(&mut self, expected: TokenKind) -> Result<Token, ParseError> {
        let token = self.advance();
        if token.kind == expected {
            return Ok(token);
        }
        self.logger.add_error(Error{
            message: format!("expected {:?}", expected),
            kind: ErrorKind::Parse(ParseError::UnexpectedToken),
            span: token.span.clone()
        });
        Err(ParseError::UnexpectedToken)
    }
    fn peek(&self) -> Token {
        if self.is_at_end() {
            return self.tokens.last().unwrap().clone()
        }
        self.tokens[self.current].clone()
    }
    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    pub fn run(&mut self) -> Result<Vec<Node>, ParseError> {
        let mut nodes = Vec::new();

        while !self.is_at_end() {
            let maybenode = self.parse_item();
            if let Ok(node) = maybenode {

            } else {
                //self.synchronize()
                // though im too lazy to make that now....
            }
        }
        Ok(nodes)
    }

    pub fn parse_item(&mut self) -> Result<Node, ParseError> {
        match self.peek().kind {
            TokenKind::KwConst => self.parse_var(false),
            TokenKind::KwFn => self.parse_fn(),
            TokenKind::KwLet => self.parse_var(true),
            _ => {
                self.logger.add_error(Error{
                    message: "unexpected top-level statement".to_string(),
                    kind: Parse(ParseError::UnexpectedToken),
                    span: self.peek().span,
                });
                Err(ParseError::UnexpectedToken)
            }
        }
    }

    pub fn parse_expr(&mut self, min_importance: u8) -> Result<Node, ParseError> {
        let lhs_token = self.advance();
        let lhs = match lhs_token.kind {
            TokenKind::Identifier => Node {
                data: Data::Identifier(self.current - 1),
                span: lhs_token.span,
            },
            TokenKind::IntegerLiteral => Node {
                data: Data::Integer(self.current - 1),
                span: lhs_token.span,
            },
            TokenKind::FloatLiteral => Node {
                data: Data::Integer(self.current - 1),
                span: lhs_token.span,
            },
            TokenKind::StringLiteral => Node {
                data: Data::String(self.current - 1),
                span: lhs_token.span,
            },
            TokenKind::OpenParen => {
                let mut expr = self.parse_expr(0)?;
                let close = self.expect(TokenKind::CloseParen)?;
                
                expr.span = Span { start: expr.span.start, end: close.span.end };
                expr
            },
            _ => {
                self.logger.add_error(Error{
                    message: "expected expression".to_string(),
                    kind: Parse(ParseError::UnexpectedToken),
                    span: lhs_token.span
                });
                return Err(ParseError::UnexpectedToken);
            }
        };
        
        Ok(lhs)
    }

    pub fn parse_var(&mut self, mutable: bool) -> Result<Node, ParseError> {

    }

    pub fn parse_fn(&mut self) -> Result<Node, ParseError> {

    }
}