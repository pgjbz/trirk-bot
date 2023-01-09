use std::{error::Error, fmt::Display};

use parser::trirk_parser::error::UnparsableError;

#[derive(Debug)]
pub struct TrirkError {
    message: String,
    kind: TrirkErrorKind,
}

impl Display for TrirkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} - {}", self.kind, self.message)
    }
}

#[derive(Debug)]
pub enum TrirkErrorKind {
    Parse,
    Io,
}

impl From<UnparsableError> for TrirkError {
    fn from(value: UnparsableError) -> Self {
        Self {
            message: value.to_string(),
            kind: TrirkErrorKind::Parse,
        }
    }
}

impl From<std::io::Error> for TrirkError {
    fn from(value: std::io::Error) -> Self {
        Self {
            message: value.to_string(),
            kind: TrirkErrorKind::Io,
        }
    }
}

impl Error for TrirkError {}
