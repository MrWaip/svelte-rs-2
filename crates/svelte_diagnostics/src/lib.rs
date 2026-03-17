use std::fmt;

use svelte_span::Span;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum Severity {
    Error,
    Warning,
}

#[derive(Debug, PartialEq, serde::Serialize)]
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
    OnlyOneTopLevelStyle,
    UnknownDirective,
    NoEachBlockToClose,
    NoKeyBlockToClose,
    VoidElementInvalidContent,
    // svelte:options errors
    SvelteOptionsUnknownAttribute(String),
    SvelteOptionsInvalidAttributeValue(String),
    SvelteOptionsInvalidCustomElementTag,
    SvelteOptionsReservedTagName,
    SvelteOptionsNoChildren,
    SvelteOptionsInvalidAttribute,
    SvelteOptionsDuplicate,
    /// LEGACY(svelte4): `tag` attribute renamed to `customElement`.
    SvelteOptionsDeprecatedTag,
    // Internal compiler errors
    InternalError(String),
}

impl DiagnosticKind {
    /// Returns the snake_case error code for this diagnostic.
    /// Matches official Svelte error codes where applicable.
    pub fn code(&self) -> &'static str {
        match self {
            Self::UnexpectedEndOfFile => "unexpected_eof",
            Self::InvalidTagName => "tag_invalid_name",
            Self::UnterminatedStartTag => "unterminated_start_tag",
            Self::InvalidAttributeName => "attribute_invalid_name",
            Self::UnexpectedToken => "unexpected_token",
            Self::UnexpectedKeyword => "unexpected_reserved_word",
            Self::NoElementToClose => "element_invalid_closing_tag",
            Self::UnclosedNode => "element_unclosed",
            Self::InvalidExpression => "invalid_expression",
            Self::NoIfBlockToClose => "block_unexpected_close",
            Self::NoIfBlockForElse => "block_unexpected_close",
            Self::OnlyOneTopLevelScript => "script_duplicate",
            Self::OnlyOneTopLevelStyle => "style_duplicate",
            Self::UnknownDirective => "unknown_directive",
            Self::NoEachBlockToClose => "block_unexpected_close",
            Self::NoKeyBlockToClose => "block_unexpected_close",
            Self::VoidElementInvalidContent => "void_element_invalid_content",
            Self::SvelteOptionsUnknownAttribute(_) => "svelte_options_unknown_attribute",
            Self::SvelteOptionsInvalidAttributeValue(_) => "svelte_options_invalid_attribute_value",
            Self::SvelteOptionsInvalidCustomElementTag => "svelte_options_invalid_customelement",
            Self::SvelteOptionsReservedTagName => "svelte_options_reserved_tagname",
            Self::SvelteOptionsNoChildren => "svelte_options_children_forbidden",
            Self::SvelteOptionsInvalidAttribute => "svelte_options_invalid_attribute",
            Self::SvelteOptionsDuplicate => "svelte_options_duplicate",
            Self::SvelteOptionsDeprecatedTag => "svelte_options_deprecated_tag",
            Self::InternalError(_) => "internal_error",
        }
    }

    /// Returns a human-readable error message.
    pub fn message(&self) -> String {
        match self {
            Self::UnexpectedEndOfFile => "Unexpected end of input".into(),
            Self::InvalidTagName => {
                "Expected a valid element or component name".into()
            }
            Self::UnterminatedStartTag => "Start tag is not terminated".into(),
            Self::InvalidAttributeName => "Invalid attribute name".into(),
            Self::UnexpectedToken => "Unexpected token".into(),
            Self::UnexpectedKeyword => "Unexpected reserved word".into(),
            Self::NoElementToClose => {
                "Attempted to close an element that was not open".into()
            }
            Self::UnclosedNode => "Element was left open".into(),
            Self::InvalidExpression => "Invalid expression".into(),
            Self::NoIfBlockToClose => {
                "Unexpected {/if} \u{2014} there is no matching {#if}".into()
            }
            Self::NoIfBlockForElse => {
                "Unexpected {:else} \u{2014} there is no matching {#if}".into()
            }
            Self::OnlyOneTopLevelScript => {
                "A component can have a single top-level <script> element".into()
            }
            Self::OnlyOneTopLevelStyle => {
                "A component can have a single top-level <style> element".into()
            }
            Self::UnknownDirective => "Unknown directive".into(),
            Self::NoEachBlockToClose => {
                "Unexpected {/each} \u{2014} there is no matching {#each}".into()
            }
            Self::NoKeyBlockToClose => {
                "Unexpected {/key} \u{2014} there is no matching {#key}".into()
            }
            Self::VoidElementInvalidContent => {
                "Void elements cannot have children or closing tags".into()
            }
            Self::SvelteOptionsUnknownAttribute(name) => {
                format!("<svelte:options> unknown attribute '{name}'")
            }
            Self::SvelteOptionsInvalidAttributeValue(expected) => {
                format!("Value must be {expected}")
            }
            Self::SvelteOptionsInvalidCustomElementTag => {
                "\"tag\" must be a valid custom element name".into()
            }
            Self::SvelteOptionsReservedTagName => {
                "\"tag\" cannot be a reserved custom element name".into()
            }
            Self::SvelteOptionsNoChildren => {
                "<svelte:options> cannot have children".into()
            }
            Self::SvelteOptionsInvalidAttribute => {
                "<svelte:options> can only have static attributes".into()
            }
            Self::SvelteOptionsDuplicate => {
                "A component can have a single <svelte:options> element".into()
            }
            Self::SvelteOptionsDeprecatedTag => {
                "\"tag\" option is deprecated \u{2014} use \"customElement\" instead".into()
            }
            Self::InternalError(msg) => format!("Internal compiler error: {msg}"),
        }
    }

    /// Returns a link to the Svelte documentation for this error, if one exists.
    pub fn svelte_doc_url(&self) -> Option<String> {
        let code = self.code();
        match self {
            // These codes have matching pages on svelte.dev
            Self::UnexpectedEndOfFile
            | Self::InvalidTagName
            | Self::InvalidAttributeName
            | Self::UnexpectedKeyword
            | Self::NoElementToClose
            | Self::UnclosedNode
            | Self::NoIfBlockToClose
            | Self::NoIfBlockForElse
            | Self::OnlyOneTopLevelScript
            | Self::OnlyOneTopLevelStyle
            | Self::NoEachBlockToClose
            | Self::NoKeyBlockToClose
            | Self::VoidElementInvalidContent
            | Self::SvelteOptionsUnknownAttribute(_)
            | Self::SvelteOptionsInvalidAttributeValue(_)
            | Self::SvelteOptionsInvalidCustomElementTag
            | Self::SvelteOptionsReservedTagName
            | Self::SvelteOptionsNoChildren
            | Self::SvelteOptionsInvalidAttribute
            | Self::SvelteOptionsDuplicate
            | Self::SvelteOptionsDeprecatedTag => {
                Some(format!("https://svelte.dev/e/{code}"))
            }
            _ => None,
        }
    }
}

#[derive(Debug, serde::Serialize)]
pub struct Diagnostic {
    pub kind: DiagnosticKind,
    pub span: Span,
    pub severity: Severity,
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind.message())?;
        if let Some(url) = self.kind.svelte_doc_url() {
            write!(f, "\n{url}")?;
        }
        Ok(())
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

    pub fn only_single_top_level_style(span: Span) -> Self {
        Self::error(DiagnosticKind::OnlyOneTopLevelStyle, span)
    }

    pub fn unknown_directive(span: Span) -> Self {
        Self::error(DiagnosticKind::UnknownDirective, span)
    }

    pub fn no_each_block_to_close(span: Span) -> Self {
        Self::error(DiagnosticKind::NoEachBlockToClose, span)
    }

    pub fn no_key_block_to_close(span: Span) -> Self {
        Self::error(DiagnosticKind::NoKeyBlockToClose, span)
    }

    pub fn void_element_invalid_content(span: Span) -> Self {
        Self::error(DiagnosticKind::VoidElementInvalidContent, span)
    }

    pub fn svelte_options_unknown_attribute(span: Span, name: String) -> Self {
        Diagnostic { kind: DiagnosticKind::SvelteOptionsUnknownAttribute(name), span, severity: Severity::Error }
    }

    pub fn svelte_options_invalid_attribute_value(span: Span, expected: String) -> Self {
        Diagnostic { kind: DiagnosticKind::SvelteOptionsInvalidAttributeValue(expected), span, severity: Severity::Error }
    }

    pub fn svelte_options_invalid_custom_element_tag(span: Span) -> Self {
        Diagnostic { kind: DiagnosticKind::SvelteOptionsInvalidCustomElementTag, span, severity: Severity::Error }
    }

    pub fn svelte_options_reserved_tag_name(span: Span) -> Self {
        Diagnostic { kind: DiagnosticKind::SvelteOptionsReservedTagName, span, severity: Severity::Error }
    }

    pub fn svelte_options_no_children(span: Span) -> Self {
        Diagnostic { kind: DiagnosticKind::SvelteOptionsNoChildren, span, severity: Severity::Error }
    }

    pub fn svelte_options_invalid_attribute(span: Span) -> Self {
        Diagnostic { kind: DiagnosticKind::SvelteOptionsInvalidAttribute, span, severity: Severity::Error }
    }

    pub fn svelte_options_duplicate(span: Span) -> Self {
        Diagnostic { kind: DiagnosticKind::SvelteOptionsDuplicate, span, severity: Severity::Error }
    }

    /// LEGACY(svelte4): `tag` attribute renamed to `customElement`.
    pub fn svelte_options_deprecated_tag(span: Span) -> Self {
        Diagnostic { kind: DiagnosticKind::SvelteOptionsDeprecatedTag, span, severity: Severity::Warning }
    }

    pub fn internal_error(message: String) -> Self {
        Diagnostic {
            kind: DiagnosticKind::InternalError(message),
            span: Span::new(0, 0),
            severity: Severity::Error,
        }
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

    /// Renders a code frame showing ±2 lines of context around the error.
    /// Returns `None` if the span is out of bounds.
    pub fn code_frame(&self, source: &str, span: Span) -> Option<String> {
        let total_lines = self.line_starts.len();
        if total_lines == 0 {
            return None;
        }

        let (error_line, error_col) = self.line_col(span.start as usize);

        let frame_start = error_line.saturating_sub(2);
        let frame_end = (error_line + 3).min(total_lines);

        let lines: Vec<&str> = source.split('\n').collect();
        if error_line >= lines.len() {
            return None;
        }

        let max_line_num = frame_end; // 1-based
        let gutter_width = max_line_num.to_string().len();

        let mut out = String::new();
        for i in frame_start..frame_end {
            if i >= lines.len() {
                break;
            }
            let line_num = i + 1; // 1-based
            let display_line = lines[i].replace('\t', "  ");

            if i == error_line {
                out.push_str(&format!(
                    "{:>width$} | {}\n",
                    line_num,
                    display_line,
                    width = gutter_width
                ));
                // Add pointer line
                let pointer_col = lines[i][..error_col.min(lines[i].len())]
                    .chars()
                    .map(|c| if c == '\t' { 2 } else { 1 })
                    .sum::<usize>();
                out.push_str(&format!(
                    "{:>width$} | {}^\n",
                    "",
                    " ".repeat(pointer_col),
                    width = gutter_width
                ));
            } else {
                out.push_str(&format!(
                    "{:>width$} | {}\n",
                    line_num,
                    display_line,
                    width = gutter_width
                ));
            }
        }

        // Remove trailing newline
        if out.ends_with('\n') {
            out.pop();
        }

        Some(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_codes() {
        assert_eq!(DiagnosticKind::UnexpectedEndOfFile.code(), "unexpected_eof");
        assert_eq!(DiagnosticKind::InvalidTagName.code(), "tag_invalid_name");
        assert_eq!(
            DiagnosticKind::NoElementToClose.code(),
            "element_invalid_closing_tag"
        );
        assert_eq!(DiagnosticKind::UnclosedNode.code(), "element_unclosed");
        assert_eq!(DiagnosticKind::OnlyOneTopLevelScript.code(), "script_duplicate");
        assert_eq!(DiagnosticKind::OnlyOneTopLevelStyle.code(), "style_duplicate");
        assert_eq!(
            DiagnosticKind::VoidElementInvalidContent.code(),
            "void_element_invalid_content"
        );
        assert_eq!(
            DiagnosticKind::InternalError("test".into()).code(),
            "internal_error"
        );
    }

    #[test]
    fn error_messages() {
        assert_eq!(
            DiagnosticKind::UnexpectedEndOfFile.message(),
            "Unexpected end of input"
        );
        assert_eq!(
            DiagnosticKind::NoIfBlockToClose.message(),
            "Unexpected {/if} \u{2014} there is no matching {#if}"
        );
        assert_eq!(
            DiagnosticKind::InternalError("oops".into()).message(),
            "Internal compiler error: oops"
        );
    }

    #[test]
    fn svelte_doc_urls() {
        // Has Svelte doc page
        assert_eq!(
            DiagnosticKind::UnexpectedEndOfFile.svelte_doc_url(),
            Some("https://svelte.dev/e/unexpected_eof".into())
        );
        assert_eq!(
            DiagnosticKind::VoidElementInvalidContent.svelte_doc_url(),
            Some("https://svelte.dev/e/void_element_invalid_content".into())
        );

        // No Svelte doc page
        assert_eq!(DiagnosticKind::UnexpectedToken.svelte_doc_url(), None);
        assert_eq!(DiagnosticKind::UnknownDirective.svelte_doc_url(), None);
        assert_eq!(
            DiagnosticKind::InternalError("x".into()).svelte_doc_url(),
            None
        );
    }

    #[test]
    fn display_with_url() {
        let d = Diagnostic::unexpected_end_of_file(Span::new(0, 0));
        let output = format!("{d}");
        assert!(output.contains("Unexpected end of input"));
        assert!(output.contains("https://svelte.dev/e/unexpected_eof"));
    }

    #[test]
    fn display_without_url() {
        let d = Diagnostic::error(DiagnosticKind::UnexpectedToken, Span::new(0, 0));
        let output = format!("{d}");
        assert_eq!(output, "Unexpected token");
    }

    #[test]
    fn code_frame_basic() {
        let source = "line1\nline2\nline3\nline4\nline5";
        let idx = LineIndex::new(source);
        // Error at start of line3 (byte offset 12)
        let frame = idx.code_frame(source, Span::new(12, 17)).unwrap();
        assert!(frame.contains("1 | line1"));
        assert!(frame.contains("3 | line3"));
        assert!(frame.contains("5 | line5"));
        assert!(frame.contains("^"));
    }

    #[test]
    fn code_frame_first_line() {
        let source = "error_here\nline2\nline3";
        let idx = LineIndex::new(source);
        let frame = idx.code_frame(source, Span::new(0, 5)).unwrap();
        assert!(frame.contains("1 | error_here"));
        assert!(frame.contains("^"));
        assert!(frame.contains("3 | line3"));
    }

    #[test]
    fn code_frame_last_line() {
        let source = "line1\nline2\nline3\nline4\nerror_here";
        let idx = LineIndex::new(source);
        // Error at start of line5 (byte offset = 24)
        let frame = idx.code_frame(source, Span::new(24, 34)).unwrap();
        assert!(frame.contains("5 | error_here"));
        assert!(frame.contains("^"));
        assert!(frame.contains("3 | line3"));
    }

    #[test]
    fn code_frame_with_tabs() {
        let source = "\tindented";
        let idx = LineIndex::new(source);
        let frame = idx.code_frame(source, Span::new(1, 5)).unwrap();
        // Tab should be replaced with 2 spaces in display
        assert!(frame.contains("  indented"));
        // Pointer should account for tab width
        assert!(frame.contains("  ^"));
    }
}
