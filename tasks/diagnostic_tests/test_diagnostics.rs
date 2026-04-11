use std::{fs, fs::read_to_string, path::Path};

use pretty_assertions::assert_eq;
use serde::{Deserialize, Serialize};
use svelte_compiler::{compile, CompileOptions, Namespace};
use svelte_diagnostics::Severity;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
struct ExpectedDiagnostic {
    severity: String,
    code: String,
    start: u32,
    end: u32,
}

fn case_input_and_options(case: &str) -> (String, CompileOptions) {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("cases")
        .join(case)
        .join("case.svelte");
    let input = read_to_string(&path).unwrap();

    let dir = path.parent().unwrap();
    let config_path = dir.join("config.json");
    let mut opts = CompileOptions {
        name: Some("App".into()),
        ..Default::default()
    };
    if config_path.exists() {
        let config: serde_json::Value =
            serde_json::from_str(&read_to_string(&config_path).unwrap()).unwrap();
        if let Some(dev) = config.get("dev").and_then(|v| v.as_bool()) {
            opts.dev = dev;
        }
        if let Some(runes) = config.get("runes").and_then(|v| v.as_bool()) {
            opts.runes = Some(runes);
        }
        if let Some(ce) = config.get("customElement").and_then(|v| v.as_bool()) {
            opts.custom_element = ce;
        }
        if let Some(filename) = config.get("filename").and_then(|v| v.as_str()) {
            opts.filename = filename.to_string();
        }
        if let Some(ns) = config.get("namespace").and_then(|v| v.as_str()) {
            opts.namespace = match ns {
                "svg" => Namespace::Svg,
                "mathml" => Namespace::MathMl,
                _ => Namespace::Html,
            };
        }
        if let Some(exp) = config.get("experimental") {
            if let Some(async_val) = exp.get("async").and_then(|v| v.as_bool()) {
                opts.experimental.async_ = async_val;
            }
        }
    }

    (input, opts)
}

fn expected_diagnostics(case: &str) -> Vec<ExpectedDiagnostic> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("cases")
        .join(case)
        .join("case-svelte.json");
    serde_json::from_str(&read_to_string(path).unwrap()).unwrap()
}

fn normalize_actual_diagnostics(case: &str) -> Vec<ExpectedDiagnostic> {
    let (input, opts) = case_input_and_options(case);
    compile(&input, &opts)
        .diagnostics
        .into_iter()
        .map(|diagnostic| ExpectedDiagnostic {
            severity: match diagnostic.severity {
                Severity::Error => "error".into(),
                Severity::Warning => "warning".into(),
            },
            code: diagnostic.kind.code().to_string(),
            start: diagnostic.span.start,
            end: diagnostic.span.end,
        })
        .collect()
}

fn sort_diagnostics(diags: &mut [ExpectedDiagnostic]) {
    diags.sort_by(|left, right| {
        severity_rank(&left.severity)
            .cmp(&severity_rank(&right.severity))
            .then_with(|| left.code.cmp(&right.code))
            .then_with(|| left.start.cmp(&right.start))
            .then_with(|| left.end.cmp(&right.end))
    });
}

fn severity_rank(severity: &str) -> u8 {
    match severity {
        "error" => 0,
        "warning" => 1,
        other => panic!("unexpected severity '{other}'"),
    }
}

fn spans_intersect(left: &ExpectedDiagnostic, right: &ExpectedDiagnostic) -> bool {
    match (
        left.start.cmp(&left.end),
        right.start.cmp(&right.end),
        left.start == left.end,
        right.start == right.end,
    ) {
        (_, _, true, true) => left.start == right.start,
        (_, _, true, false) => point_in_span(left.start, right.start, right.end),
        (_, _, false, true) => point_in_span(right.start, left.start, left.end),
        _ => std::cmp::max(left.start, right.start) < std::cmp::min(left.end, right.end),
    }
}

fn point_in_span(point: u32, start: u32, end: u32) -> bool {
    if start == end {
        point == start
    } else {
        start <= point && point < end
    }
}

fn assert_diagnostics(case: &str) {
    let mut expected = expected_diagnostics(case);
    let mut actual = normalize_actual_diagnostics(case);
    sort_diagnostics(&mut expected);
    sort_diagnostics(&mut actual);
    write_actual_diagnostics(case, &actual);

    assert_eq!(
        actual.len(),
        expected.len(),
        "[{case}] diagnostic count mismatch\nexpected: {expected:#?}\nactual: {actual:#?}"
    );

    for (index, (actual_diag, expected_diag)) in actual.iter().zip(&expected).enumerate() {
        assert_eq!(
            actual_diag.severity, expected_diag.severity,
            "[{case}] severity mismatch at index {index}"
        );
        assert_eq!(
            actual_diag.code, expected_diag.code,
            "[{case}] code mismatch at index {index}"
        );
        assert!(
            spans_intersect(actual_diag, expected_diag),
            "[{case}] span mismatch at index {index}\nexpected: {expected_diag:#?}\nactual: {actual_diag:#?}"
        );
    }
}

fn write_actual_diagnostics(case: &str, actual: &[ExpectedDiagnostic]) {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("cases")
        .join(case)
        .join("case-rust.json");
    let json =
        serde_json::to_string_pretty(actual).expect("failed to serialize actual diagnostics");
    fs::write(path, json).expect("failed to write actual diagnostics snapshot");
}

macro_rules! diagnostic_case {
    ($name:ident, $path:literal) => {
        #[test]
        fn $name() {
            assert_diagnostics($path);
        }
    };
    ($name:ident, $path:literal, ignore = $reason:literal) => {
        #[test]
        #[ignore = $reason]
        fn $name() {
            assert_diagnostics($path);
        }
    };
}

mod a11y {
    use super::*;

    diagnostic_case!(a11y_accesskey, "a11y/a11y_accesskey");
    diagnostic_case!(a11y_autofocus_warns, "a11y/a11y_autofocus_warns");
    diagnostic_case!(
        a11y_autofocus_on_dialog_no_warning,
        "a11y/a11y_autofocus_on_dialog_no_warning"
    );
    diagnostic_case!(
        a11y_autofocus_inside_dialog_no_warning,
        "a11y/a11y_autofocus_inside_dialog_no_warning"
    );
    diagnostic_case!(
        a11y_distracting_elements_marquee,
        "a11y/a11y_distracting_elements_marquee"
    );
    diagnostic_case!(
        a11y_distracting_elements_blink,
        "a11y/a11y_distracting_elements_blink"
    );
    diagnostic_case!(
        a11y_hidden_warns_on_heading_tags,
        "a11y/a11y_hidden_warns_on_heading_tags"
    );
    diagnostic_case!(
        a11y_hidden_no_warning_on_non_heading_tags,
        "a11y/a11y_hidden_no_warning_on_non_heading_tags"
    );
    diagnostic_case!(
        a11y_missing_attribute_img_no_alt,
        "a11y/a11y_missing_attribute_img_no_alt"
    );
    diagnostic_case!(
        a11y_missing_attribute_img_with_alt_no_warning,
        "a11y/a11y_missing_attribute_img_with_alt_no_warning"
    );
    diagnostic_case!(
        a11y_missing_attribute_img_spread_no_warning,
        "a11y/a11y_missing_attribute_img_spread_no_warning"
    );
    diagnostic_case!(
        a11y_missing_attribute_area_no_alt,
        "a11y/a11y_missing_attribute_area_no_alt"
    );
    diagnostic_case!(
        a11y_missing_attribute_area_with_aria_label_no_warning,
        "a11y/a11y_missing_attribute_area_with_aria_label_no_warning"
    );
    diagnostic_case!(
        a11y_missing_attribute_iframe_no_title,
        "a11y/a11y_missing_attribute_iframe_no_title"
    );
    diagnostic_case!(
        a11y_missing_attribute_iframe_with_title_no_warning,
        "a11y/a11y_missing_attribute_iframe_with_title_no_warning"
    );
    diagnostic_case!(
        a11y_missing_attribute_object_no_title,
        "a11y/a11y_missing_attribute_object_no_title"
    );
    diagnostic_case!(
        a11y_missing_attribute_object_with_aria_labelledby_no_warning,
        "a11y/a11y_missing_attribute_object_with_aria_labelledby_no_warning"
    );
    diagnostic_case!(
        a11y_missing_attribute_anchor_no_href,
        "a11y/a11y_missing_attribute_anchor_no_href"
    );
    diagnostic_case!(
        a11y_missing_attribute_anchor_with_href_no_warning,
        "a11y/a11y_missing_attribute_anchor_with_href_no_warning"
    );
    diagnostic_case!(
        a11y_missing_attribute_anchor_with_id_no_warning,
        "a11y/a11y_missing_attribute_anchor_with_id_no_warning"
    );
    diagnostic_case!(
        a11y_missing_attribute_anchor_with_name_no_warning,
        "a11y/a11y_missing_attribute_anchor_with_name_no_warning"
    );
    diagnostic_case!(
        a11y_missing_attribute_anchor_aria_disabled_no_warning,
        "a11y/a11y_missing_attribute_anchor_aria_disabled_no_warning"
    );
    diagnostic_case!(
        a11y_missing_attribute_anchor_spread_no_warning,
        "a11y/a11y_missing_attribute_anchor_spread_no_warning"
    );
    diagnostic_case!(
        a11y_unknown_aria_attribute_warns,
        "a11y/a11y_unknown_aria_attribute_warns"
    );
    diagnostic_case!(
        a11y_known_aria_attribute_no_unknown_warning,
        "a11y/a11y_known_aria_attribute_no_unknown_warning"
    );
    diagnostic_case!(
        a11y_aria_attributes_warn_on_invisible_elements,
        "a11y/a11y_aria_attributes_warn_on_invisible_elements"
    );
    diagnostic_case!(
        a11y_incorrect_aria_attribute_type_idlist_warns_on_empty_value,
        "a11y/a11y_incorrect_aria_attribute_type_idlist_warns_on_empty_value"
    );
    diagnostic_case!(
        a11y_incorrect_aria_attribute_type_warns_for_invalid_number,
        "a11y/a11y_incorrect_aria_attribute_type_warns_for_invalid_number"
    );
    diagnostic_case!(
        a11y_incorrect_aria_attribute_type_integer_warns_for_non_integer,
        "a11y/a11y_incorrect_aria_attribute_type_integer_warns_for_non_integer"
    );
    diagnostic_case!(
        a11y_incorrect_aria_attribute_type_boolean_warns_for_invalid_boolean,
        "a11y/a11y_incorrect_aria_attribute_type_boolean_warns_for_invalid_boolean"
    );
    diagnostic_case!(
        a11y_incorrect_aria_attribute_type_token_warns_with_allowed_values,
        "a11y/a11y_incorrect_aria_attribute_type_token_warns_with_allowed_values"
    );
    diagnostic_case!(
        a11y_incorrect_aria_attribute_type_tokenlist_warns_for_invalid_token,
        "a11y/a11y_incorrect_aria_attribute_type_tokenlist_warns_for_invalid_token"
    );
    diagnostic_case!(
        a11y_incorrect_aria_attribute_type_tristate_accepts_mixed,
        "a11y/a11y_incorrect_aria_attribute_type_tristate_accepts_mixed"
    );
    diagnostic_case!(
        a11y_incorrect_aria_attribute_type_no_warning_for_valid_known_value,
        "a11y/a11y_incorrect_aria_attribute_type_no_warning_for_valid_known_value"
    );
    diagnostic_case!(
        a11y_incorrect_aria_attribute_type_no_warning_for_dynamic_value,
        "a11y/a11y_incorrect_aria_attribute_type_no_warning_for_dynamic_value"
    );
    diagnostic_case!(
        a11y_incorrect_aria_attribute_type_unknown_attribute_only_warns_once,
        "a11y/a11y_incorrect_aria_attribute_type_unknown_attribute_only_warns_once"
    );
    diagnostic_case!(
        a11y_valid_concrete_role_no_name_warning,
        "a11y/a11y_valid_concrete_role_no_name_warning"
    );
    diagnostic_case!(
        a11y_misplaced_role_warns_on_invisible_elements,
        "a11y/a11y_misplaced_role_warns_on_invisible_elements"
    );
    diagnostic_case!(
        a11y_interactive_supports_focus_warns_for_interactive_role_with_handler,
        "a11y/a11y_interactive_supports_focus_warns_for_interactive_role_with_handler"
    );
    diagnostic_case!(
        a11y_interactive_supports_focus_no_warning_with_tabindex,
        "a11y/a11y_interactive_supports_focus_no_warning_with_tabindex"
    );
    diagnostic_case!(
        a11y_interactive_supports_focus_no_warning_when_disabled,
        "a11y/a11y_interactive_supports_focus_no_warning_when_disabled"
    );
    diagnostic_case!(
        a11y_interactive_supports_focus_no_warning_for_native_interactive_element,
        "a11y/a11y_interactive_supports_focus_no_warning_for_native_interactive_element",
        ignore = "known mismatch: npm svelte/compiler reports a11y_consider_explicit_label for unlabeled interactive button"
    );
    diagnostic_case!(
        a11y_click_events_have_key_events_warns_for_visible_noninteractive_element,
        "a11y/a11y_click_events_have_key_events_warns_for_visible_noninteractive_element"
    );
    diagnostic_case!(
        a11y_click_events_have_key_events_no_warning_with_keyboard_handler,
        "a11y/a11y_click_events_have_key_events_no_warning_with_keyboard_handler"
    );
    diagnostic_case!(
        a11y_click_events_have_key_events_no_warning_for_interactive_element,
        "a11y/a11y_click_events_have_key_events_no_warning_for_interactive_element",
        ignore = "known mismatch: npm svelte/compiler reports a11y_consider_explicit_label for unlabeled interactive button"
    );
    diagnostic_case!(
        a11y_no_noninteractive_element_interactions_warns_for_noninteractive_role,
        "a11y/a11y_no_noninteractive_element_interactions_warns_for_noninteractive_role"
    );
    diagnostic_case!(
        a11y_no_noninteractive_element_interactions_no_warning_with_contenteditable,
        "a11y/a11y_no_noninteractive_element_interactions_no_warning_with_contenteditable"
    );
    diagnostic_case!(
        a11y_no_static_element_interactions_warns_without_role,
        "a11y/a11y_no_static_element_interactions_warns_without_role"
    );
    diagnostic_case!(
        a11y_no_static_element_interactions_no_warning_with_explicit_role,
        "a11y/a11y_no_static_element_interactions_no_warning_with_explicit_role"
    );
    diagnostic_case!(
        a11y_no_noninteractive_tabindex_warns_for_div,
        "a11y/a11y_no_noninteractive_tabindex_warns_for_div"
    );
    diagnostic_case!(
        a11y_no_noninteractive_tabindex_no_warning_for_negative_tabindex,
        "a11y/a11y_no_noninteractive_tabindex_no_warning_for_negative_tabindex"
    );
    diagnostic_case!(
        a11y_no_noninteractive_tabindex_no_warning_for_interactive_element,
        "a11y/a11y_no_noninteractive_tabindex_no_warning_for_interactive_element"
    );
    diagnostic_case!(
        a11y_positive_tabindex_warns,
        "a11y/a11y_positive_tabindex_warns"
    );
    diagnostic_case!(a11y_unknown_role_warns, "a11y/a11y_unknown_role_warns");
    diagnostic_case!(a11y_abstract_role_warns, "a11y/a11y_abstract_role_warns");
    diagnostic_case!(a11y_tabindex_zero_no_warning, "a11y/a11y_tabindex_zero_no_warning");
    diagnostic_case!(
        a11y_tabindex_negative_no_warning,
        "a11y/a11y_tabindex_negative_no_warning"
    );
    diagnostic_case!(
        a11y_tabindex_dynamic_no_warning,
        "a11y/a11y_tabindex_dynamic_no_warning"
    );
}

mod attributes {
    use super::*;

    diagnostic_case!(
        attribute_global_event_reference_missing_binding,
        "attributes/attribute_global_event_reference_missing_binding",
        ignore = "known mismatch: reference repro includes extra/missing warnings beyond attribute_global_event_reference in current fixture"
    );
    diagnostic_case!(
        attribute_global_event_reference_local_binding,
        "attributes/attribute_global_event_reference_local_binding",
        ignore = "known mismatch: reference repro includes extra/missing warnings beyond attribute_global_event_reference in current fixture"
    );
}

mod components {
    use super::*;

    diagnostic_case!(
        component_name_lowercase_unused_import,
        "components/component_name_lowercase_unused_import"
    );
    diagnostic_case!(
        component_name_lowercase_plain_html_element,
        "components/component_name_lowercase_plain_html_element"
    );
}

mod options {
    use super::*;

    diagnostic_case!(
        options_deprecated_accessors_runes,
        "options/options_deprecated_accessors_runes",
        ignore = "known mismatch: Rust warning span is 0..0 while reference spans the accessors option"
    );
    diagnostic_case!(
        options_deprecated_accessors_legacy,
        "options/options_deprecated_accessors_legacy"
    );
    diagnostic_case!(
        options_deprecated_immutable_runes,
        "options/options_deprecated_immutable_runes",
        ignore = "known mismatch: Rust warning span is 0..0 while reference spans the immutable option"
    );
    diagnostic_case!(
        options_deprecated_immutable_legacy,
        "options/options_deprecated_immutable_legacy"
    );
    diagnostic_case!(
        validate_options_custom_element_warns_without_compiler_flag,
        "options/validate_options_custom_element_warns_without_compiler_flag",
        ignore = "known mismatch: Rust warning span is 0..0 while reference spans the customElement option"
    );
    diagnostic_case!(
        validate_options_custom_element_no_warn_with_compiler_flag,
        "options/validate_options_custom_element_no_warn_with_compiler_flag"
    );
}

mod perf {
    use super::*;

    diagnostic_case!(
        validate_perf_avoid_nested_class_no_warning_at_instance_top_level,
        "perf/validate_perf_avoid_nested_class_no_warning_at_instance_top_level"
    );
    diagnostic_case!(
        validate_perf_avoid_nested_class_warns_in_instance_nested_function,
        "perf/validate_perf_avoid_nested_class_warns_in_instance_nested_function"
    );
    diagnostic_case!(
        validate_perf_avoid_nested_class_no_warning_at_module_top_level,
        "perf/validate_perf_avoid_nested_class_no_warning_at_module_top_level"
    );
    diagnostic_case!(
        validate_perf_avoid_nested_class_warns_in_module_nested_function,
        "perf/validate_perf_avoid_nested_class_warns_in_module_nested_function"
    );
    diagnostic_case!(
        validate_perf_avoid_inline_class_warns_at_instance_top_level,
        "perf/validate_perf_avoid_inline_class_warns_at_instance_top_level"
    );
    diagnostic_case!(
        validate_perf_avoid_inline_class_no_warning_at_module_top_level,
        "perf/validate_perf_avoid_inline_class_no_warning_at_module_top_level"
    );
    diagnostic_case!(
        validate_perf_avoid_inline_class_warns_in_nested_function,
        "perf/validate_perf_avoid_inline_class_warns_in_nested_function"
    );
}

mod props {
    use super::*;

    diagnostic_case!(
        props_identifier_no_store_rune_conflict,
        "props/props_identifier_no_store_rune_conflict"
    );
}
