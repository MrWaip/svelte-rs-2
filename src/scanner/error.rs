#[derive(Debug)]
pub enum ScannerErrorType {
    UnexpectedEndOfFile,
    UnterminatedStartTag,
    UnterminatedEndTag,
}

#[derive(Debug)]
pub struct ScannerError {
    error_type: ScannerErrorType,
    line: usize,
    context: Option<String>,
}

impl ScannerError {
    pub fn new(error_type: ScannerErrorType, line: usize, context: Option<String>) -> Self {
        ScannerError {
            error_type,
            line,
            context,
        }
    }
}
