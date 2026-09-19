use logos::Span;

pub enum ErrorKind {
    Parse(ParseError)
}

pub enum ParseError {
    UnexpectedToken,
    UnexpectedTypeDuplicate
}

pub struct Error {
    pub(crate) message: String,
    pub(crate) kind: ErrorKind,
    pub(crate) span: Span,
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
    
    pub fn dump(&mut self) -> bool {
        if self.errors.is_empty() {
            return false;
        }
        for error in self.errors.iter() {
            eprintln!("error: {}", error.message);
        }
        true
    }
}