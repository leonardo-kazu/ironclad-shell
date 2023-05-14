use core::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct MyError {
    message: String,
}

impl Error for MyError {
    fn description(&self) -> &str {
        &self.message
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl MyError {
    pub fn new(message_value: String) -> Self {
        Self {
            message: message_value,
        }
    }
}
