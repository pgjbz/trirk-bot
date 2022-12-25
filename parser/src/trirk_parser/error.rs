use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct UnparsableError {
    message: String,
}

impl UnparsableError {
    #[inline(always)]
    pub fn new<T: Into<String>>(message: T) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl Display for UnparsableError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for UnparsableError {}
