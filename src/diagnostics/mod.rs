#[derive(Debug, PartialEq)]
pub enum DiagnosticType {
    UnexpectedEndOfFile,
    InvalidTagName,
    UnterminatedStartTag,
    InvalidAttributeName,
    UnexpectedToken,
    UnexpectedKeyword,
    NoElementToClose,
}

#[derive(Debug)]
pub struct Diagnostic {
    pub error_type: DiagnosticType,
    pub line: usize,
}

impl Diagnostic {
    pub fn new(error_type: DiagnosticType, line: usize) -> Self {
        Diagnostic { error_type, line }
    }

    pub fn unexpected_end_of_file(line: usize) -> Diagnostic {
        return Diagnostic::new(DiagnosticType::UnexpectedEndOfFile, line);
    }

    pub fn invalid_tag_name(line: usize) -> Diagnostic {
        return Diagnostic::new(DiagnosticType::InvalidTagName, line);
    }

    pub fn unterminated_start_tag(line: usize) -> Diagnostic {
        return Diagnostic::new(DiagnosticType::UnterminatedStartTag, line);
    }

    pub fn invalid_attribute_name(line: usize) -> Diagnostic {
        return Diagnostic::new(DiagnosticType::UnterminatedStartTag, line);
    }

    pub fn unexpected_token(line: usize) -> Diagnostic {
        return Diagnostic::new(DiagnosticType::UnexpectedToken, line);
    }

    pub fn unexpected_keyword(line: usize) -> Diagnostic {
        return Diagnostic::new(DiagnosticType::UnexpectedKeyword, line);
    }

    pub fn no_element_to_close(line: usize) -> Diagnostic {
        return Diagnostic::new(DiagnosticType::NoElementToClose, line);
    }
}
