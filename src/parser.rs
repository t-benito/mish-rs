use logos::Span;
use crate::error::{Error, ErrorLogger, ParseError};
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
            kind: Parse(ParseError::UnexpectedToken),
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
                nodes.push(node);
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
    
    pub fn parse_stmt(&mut self) -> Result<Node, ParseError> {
        
    }

    pub fn get_importance(&self, kind: &TokenKind) -> u8 {
        match kind {
            TokenKind::Assign
            | TokenKind::AddAssign
            | TokenKind::SubAssign
            | TokenKind::MulAssign
            | TokenKind::DivAssign => 1,

            TokenKind::Equal
            | TokenKind::NotEqual
            | TokenKind::Greater
            | TokenKind::GreaterEqual
            | TokenKind::Lesser
            | TokenKind::LesserEqual => 4,

            TokenKind::Add
            | TokenKind::Sub => 5,

            TokenKind::Mul
            | TokenKind::Div => 6,

            _ => 0,
        }
    }

    pub fn parse_expr(&mut self, min_importance: u8) -> Result<Node, ParseError> {
        let lhs_token = self.advance();
        let mut lhs = match lhs_token.kind {
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

        while self.peek().kind != TokenKind::EOF {
            let op = self.peek();

            let importance = self.get_importance(&op.kind);
            if importance == 0 || importance <= min_importance {
                break;
            }
            _ = self.advance();

            match op.kind {
                _ => {
                    let rhs = self.parse_expr(importance)?;
                    lhs = Node { 
                        // put span first because of borrow checker
                        span: Span { start: lhs.span.start, end: rhs.span.end },
                        data: Data::BinaryOp {
                            lhs: Box::new(lhs),
                            op: op.kind,
                            rhs: Box::new(rhs)
                        },
                    }
                }
            }
        }

        Ok(lhs)
    }
    
    pub fn parse_type(&mut self) -> Result<Node, ParseError> {
        
    }

    pub fn parse_var(&mut self, mutable: bool) -> Result<Node, ParseError> {
        let kw = self.advance();
        let ty = self.parse_type()?;

        let name = self.current;
        _ = self.expect(TokenKind::Identifier)?;
        
        _ = self.expect(TokenKind::Assign)?;
        let value = self.parse_expr(0)?;
        Ok(Node {
            // here again...
            span: Span { start: kw.span.start, end: value.span.end },
            data: Data::VarDeclaration {
                name,
                mutable,
                ty: Box::new(ty),
                value: Box::new(value),
            },
        })
    }

    pub fn parse_fn(&mut self) -> Result<Node, ParseError> {
        let kw = self.advance();
        let return_type = self.parse_type()?;

        let name = self.current;
        _ = self.expect(TokenKind::Identifier);

        let mut params: Option< Vec<Node> > = None;
        if self.peek().kind == TokenKind::OpenParen {
            _ = self.advance();

            while !self.is_at_end() {
                if self.peek().kind == TokenKind::CloseParen {
                    break;
                }
                let p_ty = self.parse_type()?;
                let p_name = self.current;
                _  = self.expect(TokenKind::Identifier)?;

                params.get_or_insert_with(Vec::new).push(Node {
                    span: Span { start: p_ty.span.start, end: self.peek().span.end },
                    data: Data::FnParameter {
                        name: p_name,
                        ty: Box::new(p_ty),
                    }
                });
            };
            _ = self.expect(TokenKind::CloseParen)?;
        }
        _ = self.expect(TokenKind::OpenBody)?;

        let mut body = Vec::new();
        while !self.is_at_end() {
            if self.peek().kind == TokenKind::CloseBody {
                break;
            }

            let stmt = self.parse_stmt()?;
            body.push(stmt);
        };
        let end = self.expect(TokenKind::CloseBody)?;
        
        Ok(Node {
            data: Data::FnDeclaration {
                name,
                params,
                return_type: Box::new(return_type),
                body,
            },
            span: Span { start: kw.span.start, end: end.span.end }
        })
    }
}