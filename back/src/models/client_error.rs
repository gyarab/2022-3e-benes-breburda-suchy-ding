use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct ClientError {
    pub status_code: u16,
    pub code: String,
    pub message: String,
}

impl ClientError {
    pub fn new(status_code: u16, code: &str, message: &str) -> Self {
        Self {
            status_code,
            code: code.to_owned(),
            message: message.to_owned(),
        }
    }
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {} ({})", self.status_code, self.code, self.message)
    }
}

impl Error for ClientError {}
