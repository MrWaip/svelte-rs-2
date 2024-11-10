#[derive(Debug, PartialEq)]
pub enum ScannerErrorType {
    UnexpectedEndOfFile,
    InvalidTagName,
    UnterminatedStartTag,
    InvalidAttributeName,
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

    pub fn invalid_tag_name(line: usize) -> ScannerError {
        return ScannerError::new(ScannerErrorType::InvalidTagName, line);
    }

    pub fn unterminated_start_tag(line: usize) -> ScannerError {
        return ScannerError::new(ScannerErrorType::UnterminatedStartTag, line);
    }

    pub fn invalid_attribute_name(line: usize) -> ScannerError {
        return ScannerError::new(ScannerErrorType::UnterminatedStartTag, line);
    }
}
