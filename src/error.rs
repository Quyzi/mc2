use std::{backtrace::Backtrace, error::Error, fmt::Display};


#[derive(Clone, Default, Debug)]
pub struct StorageError {
    pub source: String,
    pub trace: Box<String>,
    pub message: String,
}

impl Error for StorageError {}
impl Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{src}: {msg}", src=self.source, msg=self.message)
    }
}

impl StorageError {
    pub fn new(src: &str, msg: &str) -> Self {
        Self {
            source: format!("{src}"),
            trace: Box::new(Backtrace::capture().to_string()),
            message: format!("{msg}"),
        }
    }
}