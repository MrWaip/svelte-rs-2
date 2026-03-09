use std::fmt;

use span::Span;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
}

#[derive(Debug, PartialEq)]
pub enum DiagnosticKind {
    // Parser errors
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
    NoEachBlockToClose,
    // Analysis errors (future)
    // Semantic warnings (future)
}

#[derive(Debug)]
pub struct Diagnostic {
    pub kind: DiagnosticKind,
    pub span: Span,
    pub severity: Severity,
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.kind)
    }
}

impl Diagnostic {
    pub fn error(kind: DiagnosticKind, span: Span) -> Self {
        Diagnostic {
            kind,
            span,
            severity: Severity::Error,
        }
    }

    pub fn unexpected_end_of_file(span: Span) -> Self {
        Self::error(DiagnosticKind::UnexpectedEndOfFile, span)
    }

    pub fn invalid_tag_name(span: Span) -> Self {
        Self::error(DiagnosticKind::InvalidTagName, span)
    }

    pub fn unterminated_start_tag(span: Span) -> Self {
        Self::error(DiagnosticKind::UnterminatedStartTag, span)
    }

    pub fn invalid_attribute_name(span: Span) -> Self {
        Self::error(DiagnosticKind::InvalidAttributeName, span)
    }

    pub fn unexpected_token(span: Span) -> Self {
        Self::error(DiagnosticKind::UnexpectedToken, span)
    }

    pub fn unexpected_keyword(span: Span) -> Self {
        Self::error(DiagnosticKind::UnexpectedKeyword, span)
    }

    pub fn no_element_to_close(span: Span) -> Self {
        Self::error(DiagnosticKind::NoElementToClose, span)
    }

    pub fn no_if_block_to_close(span: Span) -> Self {
        Self::error(DiagnosticKind::NoIfBlockToClose, span)
    }

    pub fn no_if_block_for_else(span: Span) -> Self {
        Self::error(DiagnosticKind::NoIfBlockForElse, span)
    }

    pub fn unclosed_node(span: Span) -> Self {
        Self::error(DiagnosticKind::UnclosedNode, span)
    }

    pub fn invalid_expression(span: Span) -> Self {
        Self::error(DiagnosticKind::InvalidExpression, span)
    }

    pub fn only_single_top_level_script(span: Span) -> Self {
        Self::error(DiagnosticKind::OnlyOneTopLevelScript, span)
    }

    pub fn unknown_directive(span: Span) -> Self {
        Self::error(DiagnosticKind::UnknownDirective, span)
    }

    pub fn no_each_block_to_close(span: Span) -> Self {
        Self::error(DiagnosticKind::NoEachBlockToClose, span)
    }

    pub fn as_err<T>(self) -> Result<T, Diagnostic> {
        Err(self)
    }
}

/// Converts byte offset to (line, column) pair.
/// Lines and columns are 0-based.
pub struct LineIndex {
    line_starts: Vec<usize>,
}

impl LineIndex {
    pub fn new(source: &str) -> Self {
        let mut line_starts = vec![0];
        for (i, ch) in source.char_indices() {
            if ch == '\n' {
                line_starts.push(i + 1);
            }
        }
        LineIndex { line_starts }
    }

    /// Returns (line, column) for a byte offset. Both 0-based.
    pub fn line_col(&self, offset: usize) -> (usize, usize) {
        let line = self
            .line_starts
            .partition_point(|&start| start <= offset)
            .saturating_sub(1);
        let col = offset - self.line_starts[line];
        (line, col)
    }
}
