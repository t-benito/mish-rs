use logos::Span;

pub enum ErrorKind {
    Parse(ParseError)
}

pub enum ParseError {
    UnexpectedToken,
}

pub struct Error {
    message: String,
    kind: ErrorKind,
    span: Span,
}

pub struct ErrorLogger<'e> {
    errors: Vec<Error>,
    source: &'e str,
}

impl<'e> ErrorLogger<'e> {
    pub fn new(source: &'e str) -> Self {
        Self {
            errors: vec![],
            source,
        }
    }

    pub fn add_error(&mut self, error: Error) {
        self.errors.push(error);
    }
}