#[derive(Debug, PartialEq)]
pub enum ScannerErrorType {
    UnexpectedEndOfFile,
    UnterminatedStartTag,
    UnterminatedEndTag,
}

#[derive(Debug)]
pub struct ScannerError {
    pub error_type: ScannerErrorType,
    pub line: usize,
}

impl ScannerError {
    pub fn new(error_type: ScannerErrorType, line: usize) -> Self {
        ScannerError { error_type, line }
    }

    pub fn unexpected_end_of_file(line: usize) -> ScannerError {
        return ScannerError::new(ScannerErrorType::UnexpectedEndOfFile, line);
    }
}
