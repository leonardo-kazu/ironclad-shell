use crate::SHELL_NAME;

pub struct ErrorHandler {}

impl ErrorHandler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn handle_error(&self, info: String) {
        eprintln!("{SHELL_NAME} {info}");
    }
}
