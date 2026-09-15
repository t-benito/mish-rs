use crate::error::{ErrorKind, ErrorLogger};
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
    
    fn advance(&mut self) -> &Token {
        if self.is_at_end() {
            return self.tokens.last().unwrap();
        }
        let token = &self.tokens[self.current];
        self.current += 1;
        token 
    }
    // fn previous(&self) -> Option<&Token> {
    //     if self.current == 0 {
    //         return None;
    //     }
    //     self.tokens.get(self.current - 1)
    // }
    fn expect(&mut self, expected: TokenKind) -> Result<&Token, ErrorKind> {
        let token = self.advance();
        if token.kind == expected {
            Ok(token)
        }
        self.logger.add_error({
            
        });
    }
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }
    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    pub fn run(&mut self) {

    }
}