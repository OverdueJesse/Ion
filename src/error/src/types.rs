use crate::error;

#[derive(Debug)]
pub enum ErrorTypes {
    GenericCompilerError,
    TokenizationError,
    FileIOError,
}

pub struct CompilerError {
    pub line: usize,
    pub col: usize,
    pub message: String,
    pub location: String,
    pub error_type: ErrorTypes,
}

impl CompilerError {
    pub fn new(line: usize, col: usize, message: String, error_type: ErrorTypes, location: Option<String>) -> Self {
        CompilerError {
            line,
            col,
            message,
            location: location.unwrap_or_else(|| String::new()),
            error_type,
        }
    }

    pub fn throw_new(line: usize, col: usize, message: &str, error_type: ErrorTypes, location: Option<String>) {
        Self::throw(
            &Self::new(
                line,
                col,
                message.to_string(),
                error_type,
                location,
            )
        )
    }

    pub fn throw(&self) { error(self) }
}
