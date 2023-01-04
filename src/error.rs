use std::fmt::Display;
use crate::token::Token;

#[derive(Clone, Debug)]
pub enum ErrorType {
    Scanner,
    Parse,
    Runtime,
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

        writeln!(f, "{}", self.message)?;

        Ok(())
    }
}

pub struct Builder {
    error_type: ErrorType,
    message: String,
    line: Option<usize>,
    token: Option<String>,
}

impl Builder {
    pub fn new(error_type: ErrorType, message: String) -> Builder {
        Builder { error_type, message, line: None, token: None }
    }

    pub fn token(mut self, token: &Token) -> Builder {
        self.line = Some(token.span().line);
        self
    }

    pub fn build(self) -> Error {
        Error { error_type: self.error_type, message: self.message, line: self.line, token: self.token }
    }
}