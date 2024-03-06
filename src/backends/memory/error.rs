use std::{
    backtrace::Backtrace,
    cell::{BorrowError, BorrowMutError},
    error::Error,
    fmt::{Debug, Display},
};

#[derive(Clone, Debug, Default)]
pub struct MemoryError {
    pub source: String,
    pub backtrace: Box<String>,
    pub message: String,
}

impl Error for MemoryError {}

impl Display for MemoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{source}] {message}",
            source = self.source,
            message = self.message
        )
    }
}

impl From<Box<bincode::ErrorKind>> for MemoryError {
    fn from(value: Box<bincode::ErrorKind>) -> Self {
        Self {
            source: "std::error::Error".to_string(),
            backtrace: Box::new(Backtrace::capture().to_string()),
            message: value.to_string(),
        }
    }
}

impl From<BorrowError> for MemoryError {
    fn from(value: BorrowError) -> Self {
        Self::default()
            .with_source("borrow_error")
            .with_message(&value.to_string())
    }
}

impl From<BorrowMutError> for MemoryError {
    fn from(value: BorrowMutError) -> Self {
        Self::default()
            .with_source("borrow_mut_error")
            .with_message(&value.to_string())
    }
}

impl MemoryError {
    pub fn new(src: &str, msg: &str) -> Self {
        Self {
            source: src.to_string(),
            backtrace: Box::new(Backtrace::capture().to_string()),
            message: msg.to_string(),
        }
    }

    #[inline]
    pub fn with_source(mut self, src: &str) -> Self {
        self.source = src.to_string();
        self
    }

    #[inline]
    pub fn with_message(mut self, msg: &str) -> Self {
        self.message = msg.to_string();
        self
    }

    #[inline]
    pub fn with_backtrace(mut self) -> Self {
        self.backtrace = Box::new(Backtrace::capture().to_string());
        self
    }
}
