use svelte_span::Span;

use crate::codes::{fuzzymatch, is_valid_warning_code, legacy_replacement};
use crate::{Diagnostic, DiagnosticKind};

/// Result of parsing a `<!-- svelte-ignore ... -->` comment.
#[derive(Debug, Default)]
pub struct ExtractResult {
    /// Valid warning codes extracted from the comment.
    pub codes: Vec<String>,
    /// Diagnostics emitted during parsing (e.g., LegacyCode, UnknownCode warnings).
    pub warnings: Vec<Diagnostic>,
}

/// Extracts svelte-ignore codes from comment inner text.
///
/// `offset` is the byte offset of `text` in the source (for diagnostic spans).
/// `text` is the content between `<!--` and `-->`, NOT including delimiters.
/// `runes` selects strict (comma-separated) vs lenient (space-separated) parsing.
pub fn extract_svelte_ignore(offset: u32, text: &str, runes: bool) -> ExtractResult {
    let prefix = "svelte-ignore";
    let trimmed = text.trim_start();
    let leading_ws = text.len() - trimmed.len();

    if !trimmed.starts_with(prefix) {
        return ExtractResult::default();
    }

    // Must have whitespace after "svelte-ignore"
    let after_prefix = &trimmed[prefix.len()..];
    if !after_prefix.starts_with(|c: char| c.is_whitespace()) {
        return ExtractResult::default();
    }

    let codes_text = after_prefix.trim_start();
    let codes_offset = offset + (leading_ws + prefix.len() + (after_prefix.len() - codes_text.len())) as u32;

    if runes {
        extract_runes_mode(codes_offset, codes_text)
    } else {
        extract_legacy_mode(codes_offset, codes_text)
    }
}

fn is_code_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '$' || c == '-'
}

/// Runes mode: comma-separated, strict validation.
/// Stops at first code without trailing comma (rest is prose).
fn extract_runes_mode(base_offset: u32, text: &str) -> ExtractResult {
    let mut result = ExtractResult::default();
    let all_codes = DiagnosticKind::all_warning_codes();
    let mut pos = 0;
    let bytes = text.as_bytes();

    loop {
        // Skip whitespace
        while pos < bytes.len() && bytes[pos].is_ascii_whitespace() {
            pos += 1;
        }
        if pos >= bytes.len() {
            break;
        }

        // Read a word
        let word_start = pos;
        while pos < bytes.len() && is_code_char(bytes[pos] as char) {
            pos += 1;
        }
        if pos == word_start {
            break;
        }

        let code = &text[word_start..pos];
        let span = Span::new(base_offset + word_start as u32, base_offset + pos as u32);

        if is_valid_warning_code(code) {
            result.codes.push(code.to_string());
        } else {
            let replacement = legacy_replacement(code)
                .map(String::from)
                .unwrap_or_else(|| code.replace('-', "_"));

            if is_valid_warning_code(&replacement) {
                result.warnings.push(Diagnostic::warning(
                    DiagnosticKind::LegacyCode {
                        code: code.to_string(),
                        suggestion: replacement.clone(),
                    },
                    span,
                ));
                result.codes.push(replacement);
            } else {
                let suggestion = fuzzymatch(code, all_codes).map(String::from);
                result.warnings.push(Diagnostic::warning(
                    DiagnosticKind::UnknownCode {
                        code: code.to_string(),
                        suggestion,
                    },
                    span,
                ));
            }
        }

        // Skip whitespace after word
        while pos < bytes.len() && bytes[pos].is_ascii_whitespace() {
            pos += 1;
        }

        // Check for comma — if none, stop (rest is prose)
        if pos < bytes.len() && bytes[pos] == b',' {
            pos += 1;
        } else {
            break;
        }
    }

    result
}

/// Legacy mode: space-separated, lenient — accepts all codes.
fn extract_legacy_mode(_base_offset: u32, text: &str) -> ExtractResult {
    let mut result = ExtractResult::default();
    let mut pos = 0;
    let bytes = text.as_bytes();

    loop {
        while pos < bytes.len() && !is_code_char(bytes[pos] as char) {
            pos += 1;
        }
        if pos >= bytes.len() {
            break;
        }

        let word_start = pos;
        while pos < bytes.len() && is_code_char(bytes[pos] as char) {
            pos += 1;
        }

        let code = &text[word_start..pos];
        result.codes.push(code.to_string());

        if !is_valid_warning_code(code) {
            let replacement = legacy_replacement(code)
                .map(String::from)
                .unwrap_or_else(|| code.replace('-', "_"));

            if is_valid_warning_code(&replacement) {
                result.codes.push(replacement);
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn not_svelte_ignore() {
        let r = extract_svelte_ignore(0, " some comment ", true);
        assert!(r.codes.is_empty());
        assert!(r.warnings.is_empty());
    }

    #[test]
    fn runes_single_code() {
        let r = extract_svelte_ignore(0, " svelte-ignore block_empty ", true);
        assert_eq!(r.codes, vec!["block_empty"]);
        assert!(r.warnings.is_empty());
    }

    #[test]
    fn runes_comma_separated() {
        let r = extract_svelte_ignore(0, " svelte-ignore a11y_accesskey, block_empty ", true);
        assert_eq!(r.codes, vec!["a11y_accesskey", "block_empty"]);
        assert!(r.warnings.is_empty());
    }

    #[test]
    fn runes_stops_at_no_comma() {
        let r = extract_svelte_ignore(0, " svelte-ignore block_empty some prose here ", true);
        assert_eq!(r.codes, vec!["block_empty"]);
    }

    #[test]
    fn runes_legacy_code_emits_warning() {
        let r = extract_svelte_ignore(0, " svelte-ignore empty-block ", true);
        assert_eq!(r.codes, vec!["block_empty"]);
        assert_eq!(r.warnings.len(), 1);
        assert_eq!(r.warnings[0].kind.code(), "legacy_code");
    }

    #[test]
    fn runes_unknown_code_emits_warning() {
        let r = extract_svelte_ignore(0, " svelte-ignore totally_invalid_code ", true);
        assert!(r.codes.is_empty());
        assert_eq!(r.warnings.len(), 1);
        assert_eq!(r.warnings[0].kind.code(), "unknown_code");
    }

    #[test]
    fn runes_unknown_with_suggestion() {
        let r = extract_svelte_ignore(0, " svelte-ignore block_emtpy ", true);
        assert_eq!(r.warnings.len(), 1);
        match &r.warnings[0].kind {
            DiagnosticKind::UnknownCode { suggestion, .. } => {
                assert_eq!(suggestion.as_deref(), Some("block_empty"));
            }
            other => panic!("expected UnknownCode, got {other:?}"),
        }
    }

    #[test]
    fn legacy_space_separated() {
        let r = extract_svelte_ignore(0, " svelte-ignore a11y_accesskey block_empty ", false);
        assert_eq!(r.codes, vec!["a11y_accesskey", "block_empty"]);
        assert!(r.warnings.is_empty());
    }

    #[test]
    fn legacy_maps_old_codes() {
        let r = extract_svelte_ignore(0, " svelte-ignore empty-block ", false);
        assert!(r.codes.contains(&"empty-block".to_string()));
        assert!(r.codes.contains(&"block_empty".to_string()));
    }

    #[test]
    fn legacy_accepts_unknown() {
        let r = extract_svelte_ignore(0, " svelte-ignore whatever ", false);
        assert_eq!(r.codes, vec!["whatever"]);
        assert!(r.warnings.is_empty());
    }

    #[test]
    fn no_space_after_prefix() {
        let r = extract_svelte_ignore(0, "svelte-ignorefoo", true);
        assert!(r.codes.is_empty());
    }

    #[test]
    fn offset_tracking() {
        let r = extract_svelte_ignore(4, " svelte-ignore bad_code ", true);
        assert_eq!(r.warnings.len(), 1);
        // "bad_code" starts at position 15 in " svelte-ignore bad_code "
        assert_eq!(r.warnings[0].span.start, 4 + 15);
    }
}
