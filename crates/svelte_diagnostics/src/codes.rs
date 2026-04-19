use crate::DiagnosticKind;

/// Returns the new warning code for a legacy (Svelte 4) svelte-ignore code.
/// Legacy codes used kebab-case; Svelte 5 uses snake_case.
pub fn legacy_replacement(code: &str) -> Option<&'static str> {
    match code {
        "non-top-level-reactive-declaration" => Some("reactive_declaration_invalid_placement"),
        "module-script-reactive-declaration" => {
            Some("reactive_declaration_module_script_dependency")
        }
        "empty-block" => Some("block_empty"),
        "avoid-is" => Some("attribute_avoid_is"),
        "invalid-html-attribute" => Some("attribute_invalid_property_name"),
        "a11y-structure" => Some("a11y_figcaption_parent"),
        "illegal-attribute-character" => Some("attribute_illegal_colon"),
        "invalid-rest-eachblock-binding" => Some("bind_invalid_each_rest"),
        "unused-export-let" => Some("export_let_unused"),
        _ => None,
    }
}

/// Runtime-only warnings that can be suppressed via `svelte-ignore` but are not
/// emitted by the compiler. Matches Svelte's `IGNORABLE_RUNTIME_WARNINGS`.
const IGNORABLE_RUNTIME_WARNINGS: &[&str] = &[
    "await_waterfall",
    "await_reactivity_loss",
    "state_snapshot_uncloneable",
];

/// Returns true if the code is a valid warning code (compile-time or runtime-ignorable).
pub fn is_valid_warning_code(code: &str) -> bool {
    DiagnosticKind::all_warning_codes().contains(&code)
        || IGNORABLE_RUNTIME_WARNINGS.contains(&code)
}

/// Finds the closest match for `input` among `candidates` using Levenshtein distance.
/// Returns `None` if no candidate is close enough (threshold: len/3, min 2).
pub fn fuzzymatch<'a>(input: &str, candidates: &[&'a str]) -> Option<&'a str> {
    let threshold = (input.len() / 3).max(2);
    let mut best: Option<(&'a str, usize)> = None;

    for &candidate in candidates {
        let dist = levenshtein(input, candidate);
        if dist <= threshold && best.is_none_or(|(_, d)| dist < d) {
            best = Some((candidate, dist));
        }
    }

    best.map(|(s, _)| s)
}

fn levenshtein(a: &str, b: &str) -> usize {
    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();
    let m = a_bytes.len();
    let n = b_bytes.len();

    let mut prev = (0..=n).collect::<Vec<_>>();
    let mut curr = vec![0; n + 1];

    for i in 1..=m {
        curr[0] = i;
        for j in 1..=n {
            let cost = if a_bytes[i - 1] == b_bytes[j - 1] {
                0
            } else {
                1
            };
            curr[j] = (prev[j] + 1).min(curr[j - 1] + 1).min(prev[j - 1] + cost);
        }
        std::mem::swap(&mut prev, &mut curr);
    }

    prev[n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legacy_replacements() {
        assert_eq!(legacy_replacement("empty-block"), Some("block_empty"));
        assert_eq!(legacy_replacement("avoid-is"), Some("attribute_avoid_is"));
        assert_eq!(
            legacy_replacement("non-top-level-reactive-declaration"),
            Some("reactive_declaration_invalid_placement")
        );
        assert_eq!(legacy_replacement("unknown-code"), None);
        assert_eq!(legacy_replacement("block_empty"), None);
    }

    #[test]
    fn valid_codes() {
        assert!(is_valid_warning_code("block_empty"));
        assert!(is_valid_warning_code("a11y_accesskey"));
        assert!(is_valid_warning_code("css_unused_selector"));
        assert!(!is_valid_warning_code("not_a_code"));
        assert!(!is_valid_warning_code("empty-block"));
    }

    #[test]
    fn fuzzymatch_finds_close() {
        let codes = DiagnosticKind::all_warning_codes();
        assert_eq!(fuzzymatch("block_emtpy", codes), Some("block_empty"));
        assert_eq!(fuzzymatch("a11y_acceeskey", codes), Some("a11y_accesskey"));
    }

    #[test]
    fn fuzzymatch_returns_none_for_distant() {
        let codes = DiagnosticKind::all_warning_codes();
        assert_eq!(fuzzymatch("completely_unrelated_thing", codes), None);
    }

    #[test]
    fn levenshtein_basic() {
        assert_eq!(levenshtein("kitten", "sitting"), 3);
        assert_eq!(levenshtein("", "abc"), 3);
        assert_eq!(levenshtein("abc", "abc"), 0);
        assert_eq!(levenshtein("abc", "abd"), 1);
    }
}
