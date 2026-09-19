use logos::Span;
use crate::error::{Error, ErrorLogger, ParseError};
use crate::error::ErrorKind::Parse;
use crate::node::{Data, Node, ParseType, ParseTypeBase};
use crate::token::{Token, TokenKind};

pub struct Parser<'a> {
    tokens: &'a [Token],
    source: &'a str,
    logger: ErrorLogger<'a>,

    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token], source: &'a str) -> Self {
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
                self.synchronize()
            }
        }
        Ok(nodes)
    }

    pub fn synchronize(&mut self) {
        while !self.is_at_end() {
            match self.peek().kind {
                TokenKind::KwLet
                | TokenKind::KwConst
                | TokenKind::KwElse
                | TokenKind::KwIf
                | TokenKind::KwAlias
                | TokenKind::KwStruct
                | TokenKind::KwUnion
                | TokenKind::KwEnum
                | TokenKind::KwRtn
                | TokenKind::EOF
                | TokenKind::CompKwDefine
                | TokenKind::CompKwSizeof
                | TokenKind::KwFn => break,
                _ => {}
            }
            self.advance();
        }
        return;
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
        match self.peek().kind {
            TokenKind::KwConst => self.parse_var(false),
            TokenKind::KwLet => self.parse_var(true),
            _ => {
                self.logger.add_error(Error {
                    message: "expected statement".to_string(),
                    kind: Parse(ParseError::UnexpectedToken),
                    span: self.peek().span,
                });
                Err(ParseError::UnexpectedToken)
            }
        }
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

            TokenKind::Star
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
            self.advance();

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
    
    pub fn parse_type(&mut self) -> Result<ParseType, ParseError> {
        let first_token = self.advance();

        let mut origin = match first_token.kind {
            TokenKind::KwLong => {
                let mut next = self.parse_type()?;
                if next.long {
                    self.logger.add_error(Error {
                        message: "unexpected duplicate 'long'".to_string(),
                        kind: Parse(ParseError::UnexpectedTypeDuplicate),
                        span: first_token.span,
                    });
                    return Err(ParseError::UnexpectedTypeDuplicate);
                }
                next.long = true;
                next
            }
            TokenKind::KwUnsigned => {
                let mut next = self.parse_type()?;
                if next.unsigned {
                    self.logger.add_error(Error {
                        message: "unexpected duplicate 'unsigned'".to_string(),
                        kind: Parse(ParseError::UnexpectedTypeDuplicate),
                        span: first_token.span,
                    });
                    return Err(ParseError::UnexpectedTypeDuplicate);
                }
                next.unsigned = true;
                next
            }
            TokenKind::KwInt => {
                ParseType {
                    base: ParseTypeBase::Int,
                    long: false,
                    unsigned: false,
                }
            }
            TokenKind::KwChar => {
                ParseType {
                    base: ParseTypeBase::Char,
                    long: false,
                    unsigned: false,
                }
            }
            _ => {
                self.logger.add_error(Error{
                    message: "expected type statement".to_string(),
                    kind: Parse(ParseError::UnexpectedToken),
                    span: first_token.span,
                });
                return Err(ParseError::UnexpectedToken);
              }
        };

        loop {
            match self.peek().kind {
                TokenKind::Star => {
                    self.advance();
                    origin = ParseType {
                        unsigned: false,
                        long: false,
                        base: ParseTypeBase::Pointer(Box::new(origin)),
                    }
                }
                TokenKind::OpenBracket => {
                    self.advance();
                    let len = self.parse_expr(0)?;
                    self.expect(TokenKind::CloseBracket)?;
                    origin = ParseType {
                        unsigned: false,
                        long: false,
                        base: ParseTypeBase::Array(Box::new(origin), len),
                    }
                }
                _ => break,
            }
        }

        Ok(origin)
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
                let start = self.peek().span.start;
                let p_ty = self.parse_type()?;
                let p_name = self.current;
                self.expect(TokenKind::Identifier)?;

                params.get_or_insert_with(Vec::new).push(Node {
                    span: Span { start, end: self.peek().span.end },
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