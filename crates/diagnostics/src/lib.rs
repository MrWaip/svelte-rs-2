use std::fmt;

use span::Span;

#[derive(Debug, PartialEq)]
pub enum DiagnosticType {
    UnexpectedEndOfFile,
    InvalidTagName,
    UnterminatedStartTag,
    InvalidAttributeName,
    UnexpectedToken,
    UnexpectedKeyword,
    NoElementToClose,
    UnclosedNode,
    InvalidExpression,
    NoIfBlockToClose,
    NoIfBlockForElse,
    OnlyOneTopLevelScript,
    UnknownDirective,
}

#[derive(Debug)]
pub struct Diagnostic {
    pub error_type: DiagnosticType,
    pub span: Span,
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error_type = &self.error_type;
        f.write_str(&format!("{error_type:?}"))?;
        Ok(())
    }
}

impl Diagnostic {
    pub fn new(error_type: DiagnosticType, span: Span) -> Self {
        Diagnostic { error_type, span }
    }

    pub fn unexpected_end_of_file(span: Span) -> Diagnostic {
        Diagnostic::new(DiagnosticType::UnexpectedEndOfFile, span)
    }

    pub fn invalid_tag_name(span: Span) -> Diagnostic {
        Diagnostic::new(DiagnosticType::InvalidTagName, span)
    }

    pub fn unterminated_start_tag(span: Span) -> Diagnostic {
        Diagnostic::new(DiagnosticType::UnterminatedStartTag, span)
    }

    pub fn invalid_attribute_name(span: Span) -> Diagnostic {
        Diagnostic::new(DiagnosticType::InvalidAttributeName, span)
    }

    pub fn unexpected_token(span: Span) -> Diagnostic {
        Diagnostic::new(DiagnosticType::UnexpectedToken, span)
    }

    pub fn unexpected_keyword(span: Span) -> Diagnostic {
        Diagnostic::new(DiagnosticType::UnexpectedKeyword, span)
    }

    pub fn no_element_to_close(span: Span) -> Diagnostic {
        Diagnostic::new(DiagnosticType::NoElementToClose, span)
    }

    pub fn no_if_block_to_close(span: Span) -> Diagnostic {
        Diagnostic::new(DiagnosticType::NoIfBlockToClose, span)
    }

    pub fn no_if_block_for_else(span: Span) -> Diagnostic {
        Diagnostic::new(DiagnosticType::NoIfBlockForElse, span)
    }

    pub fn unclosed_node(span: Span) -> Diagnostic {
        Diagnostic::new(DiagnosticType::UnclosedNode, span)
    }

    pub fn invalid_expression(span: Span) -> Diagnostic {
        Diagnostic::new(DiagnosticType::InvalidExpression, span)
    }

    pub fn as_err<T>(self) -> Result<T, Diagnostic> {
        Err(self)
    }

    pub fn only_single_top_level_script(span: Span) -> Diagnostic {
        Diagnostic::new(DiagnosticType::OnlyOneTopLevelScript, span)
    }

    pub fn unknown_directive(span: Span) -> Diagnostic {
        Diagnostic::new(DiagnosticType::UnknownDirective, span)
    }
}
