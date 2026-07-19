use std::mem;

use crate::DiagnosticKind;

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

const IGNORABLE_RUNTIME_WARNINGS: &[&str] = &[
    "await_waterfall",
    "await_reactivity_loss",
    "state_snapshot_uncloneable",
    "binding_property_non_reactive",
    "hydration_attribute_changed",
    "hydration_html_changed",
    "ownership_invalid_binding",
    "ownership_invalid_mutation",
];

pub fn is_valid_warning_code(code: &str) -> bool {
    DiagnosticKind::all_warning_codes().contains(&code)
        || IGNORABLE_RUNTIME_WARNINGS.contains(&code)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WarningCode {
    AwaitWaterfall,
    AwaitReactivityLoss,
    StateSnapshotUncloneable,
    BindingPropertyNonReactive,
    HydrationAttributeChanged,
    HydrationHtmlChanged,
    OwnershipInvalidBinding,
    OwnershipInvalidMutation,
    BidirectionalControlCharacters,
}

impl WarningCode {
    pub const fn as_str(self) -> &'static str {
        match self {
            WarningCode::AwaitWaterfall => "await_waterfall",
            WarningCode::AwaitReactivityLoss => "await_reactivity_loss",
            WarningCode::StateSnapshotUncloneable => "state_snapshot_uncloneable",
            WarningCode::BindingPropertyNonReactive => "binding_property_non_reactive",
            WarningCode::HydrationAttributeChanged => "hydration_attribute_changed",
            WarningCode::HydrationHtmlChanged => "hydration_html_changed",
            WarningCode::OwnershipInvalidBinding => "ownership_invalid_binding",
            WarningCode::OwnershipInvalidMutation => "ownership_invalid_mutation",
            WarningCode::BidirectionalControlCharacters => "bidirectional_control_characters",
        }
    }
}

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
        mem::swap(&mut prev, &mut curr);
    }

    prev[n]
}

#[cfg(test)]
mod tests {
    use std::{fs, path::Path};

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
    fn ignorable_runtime_warnings_in_sync_with_reference() {
        let candidates = [
            "../../tasks/generate_test_cases/node_modules/svelte/src/constants.js",
            "../../tasks/benchmark/node_modules/svelte/src/constants.js",
        ];
        let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let path = candidates
            .iter()
            .map(|c| manifest_dir.join(c))
            .find(|p| p.exists());
        let Some(path) = path else {
            return;
        };
        let content = fs::read_to_string(&path).expect("read constants.js");
        let start = content
            .find("IGNORABLE_RUNTIME_WARNINGS")
            .expect("constant present");
        let body_start = content[start..].find('[').expect("array start");
        let body_end = content[start + body_start..].find(']').expect("array end");
        let body = &content[start + body_start + 1..start + body_start + body_end];
        let mut reference: Vec<&str> = body
            .split(',')
            .map(str::trim)
            .filter(|s| s.starts_with('\'') && s.ends_with('\''))
            .map(|s| &s[1..s.len() - 1])
            .collect();
        reference.sort();
        let mut ours: Vec<&str> = IGNORABLE_RUNTIME_WARNINGS.to_vec();
        ours.sort();
        assert_eq!(
            ours,
            reference,
            "IGNORABLE_RUNTIME_WARNINGS drifted from svelte/src/constants.js at {}",
            path.display()
        );
    }

    fn warning_codes_in_dts(content: &str) -> Vec<String> {
        let start = content
            .find("export type WarningCode =")
            .expect("WarningCode union present");
        let rest = &content[start..];
        let end = rest.find(';').expect("union terminator");
        let body = &rest[..end];
        let parts: Vec<&str> = body.split('\'').collect();
        let mut codes = Vec::new();
        let mut i = 1;
        while i < parts.len() {
            codes.push(parts[i].to_string());
            i += 2;
        }
        codes
    }

    #[test]
    fn warning_code_union_in_sync_with_all_warning_codes() {
        let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let path = manifest_dir.join("../../packages/svelte-rs2/compiler/index.d.ts");
        let Ok(content) = fs::read_to_string(&path) else {
            return;
        };
        let mut union = warning_codes_in_dts(&content);
        union.sort();
        let mut reference: Vec<String> = DiagnosticKind::all_warning_codes()
            .iter()
            .map(|code| code.to_string())
            .collect();
        reference.sort();
        assert_eq!(
            union,
            reference,
            "WarningCode union in {} drifted from DiagnosticKind::all_warning_codes(); regenerate it",
            path.display()
        );
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
