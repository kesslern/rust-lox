use std::fmt::Display;
use crate::token::Token;

#[derive(Clone, Debug)]
pub enum ErrorType {
    ScannerError,
    ParseError,
    RuntimeError,
}

#[derive(Clone, Debug)]
pub struct Error {
    pub error_type: ErrorType,
    pub message: String,
    pub token: Option<String>,
    pub line: Option<usize>,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(line) = self.line {
            write!(f, "[line {}] Error", line)?;
        } else {
            write!(f, "Error")?;
        }

        if let Some(token) = &self.token {
            write!(f, " at '{}': ", token)?;
        } else {
            write!(f, ": ")?;
        }

        eprintln!("{}", self.message);

        Ok(())
    }
}

pub struct ErrorBuilder {
    error_type: ErrorType,
    message: String,
    line: Option<usize>,
    token: Option<String>,
}

impl ErrorBuilder {
    pub fn new(error_type: ErrorType, message: String) -> ErrorBuilder {
        ErrorBuilder { error_type, message, line: None, token: None }
    }

    pub fn token(mut self, token: Token) -> ErrorBuilder {
        self.token = Some(token.lexeme);
        self.line = Some(token.line);
        self
    }

    pub fn build(self) -> Error {
        Error { error_type: self.error_type, message: self.message, line: self.line, token: self.token }
    }
}