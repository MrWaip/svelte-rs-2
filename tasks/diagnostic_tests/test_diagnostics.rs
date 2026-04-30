use std::{fs, fs::read_to_string, path::Path};

use pretty_assertions::assert_eq;
use serde::{Deserialize, Serialize};
use svelte_compiler::{CompileOptions, Namespace, compile};
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
    let input = read_to_string(&path).expect("test invariant");

    let dir = path.parent().expect("test invariant");
    let config_path = dir.join("config.json");
    let mut opts = CompileOptions {
        name: Some("App".into()),
        ..Default::default()
    };
    if config_path.exists() {
        let config: serde_json::Value =
            serde_json::from_str(&read_to_string(&config_path).expect("test invariant"))
                .expect("test invariant");
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
        if let Some(exp) = config.get("experimental")
            && let Some(async_val) = exp.get("async").and_then(|v| v.as_bool())
        {
            opts.experimental.async_ = async_val;
        }
    }

    (input, opts)
}

fn expected_diagnostics(case: &str) -> Vec<ExpectedDiagnostic> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("cases")
        .join(case)
        .join("case-svelte.json");
    serde_json::from_str(&read_to_string(path).expect("test invariant")).expect("test invariant")
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
        a11y_role_whitespace_validates_each_token,
        "a11y/a11y_role_whitespace_validates_each_token"
    );
    diagnostic_case!(
        a11y_no_redundant_roles_warns_for_native_semantics,
        "a11y/a11y_no_redundant_roles_warns_for_native_semantics"
    );
    diagnostic_case!(
        a11y_no_redundant_roles_warns_for_top_level_header_banner,
        "a11y/a11y_no_redundant_roles_warns_for_top_level_header_banner"
    );
    diagnostic_case!(
        a11y_no_redundant_roles_no_warning_for_section_header_banner,
        "a11y/a11y_no_redundant_roles_no_warning_for_section_header_banner"
    );
    diagnostic_case!(
        a11y_no_redundant_roles_no_warning_for_anchor_without_href,
        "a11y/a11y_no_redundant_roles_no_warning_for_anchor_without_href"
    );
    diagnostic_case!(
        a11y_role_has_required_aria_props_no_warning_when_props_present,
        "a11y/a11y_role_has_required_aria_props_no_warning_when_props_present"
    );
    diagnostic_case!(
        a11y_role_has_required_aria_props_warns_for_missing_props,
        "a11y/a11y_role_has_required_aria_props_warns_for_missing_props"
    );
    diagnostic_case!(
        a11y_role_has_required_aria_props_no_warning_for_native_semantics,
        "a11y/a11y_role_has_required_aria_props_no_warning_for_native_semantics"
    );
    diagnostic_case!(
        a11y_role_has_required_aria_props_no_warning_with_spread,
        "a11y/a11y_role_has_required_aria_props_no_warning_with_spread"
    );
    diagnostic_case!(
        a11y_role_supports_aria_props_no_warning_for_supported_explicit_role_prop,
        "a11y/a11y_role_supports_aria_props_no_warning_for_supported_explicit_role_prop"
    );
    diagnostic_case!(
        a11y_role_supports_aria_props_warns_for_explicit_role,
        "a11y/a11y_role_supports_aria_props_warns_for_explicit_role"
    );
    diagnostic_case!(
        a11y_role_supports_aria_props_warns_for_implicit_role,
        "a11y/a11y_role_supports_aria_props_warns_for_implicit_role"
    );
    diagnostic_case!(
        a11y_role_supports_aria_props_no_warning_for_supported_implicit_role_prop,
        "a11y/a11y_role_supports_aria_props_no_warning_for_supported_implicit_role_prop"
    );
    diagnostic_case!(
        a11y_role_supports_aria_props_unknown_aria_attr_only_warns_once,
        "a11y/a11y_role_supports_aria_props_unknown_aria_attr_only_warns_once"
    );
    diagnostic_case!(
        a11y_role_supports_aria_props_no_warning_without_role,
        "a11y/a11y_role_supports_aria_props_no_warning_without_role"
    );
    diagnostic_case!(
        a11y_aria_activedescendant_has_tabindex_warns_without_tabindex,
        "a11y/a11y_aria_activedescendant_has_tabindex_warns_without_tabindex"
    );
    diagnostic_case!(
        a11y_aria_activedescendant_has_tabindex_no_warning_with_tabindex,
        "a11y/a11y_aria_activedescendant_has_tabindex_no_warning_with_tabindex"
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
        "a11y/a11y_interactive_supports_focus_no_warning_for_native_interactive_element"
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
        "a11y/a11y_click_events_have_key_events_no_warning_for_interactive_element"
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
        a11y_no_static_element_interactions_ignored_on_mouseenter_mouseleave,
        "a11y/a11y_no_static_element_interactions_ignored_on_mouseenter_mouseleave"
    );
    diagnostic_case!(
        a11y_mouse_events_have_key_events_warns_for_mouseover_without_focus,
        "a11y/a11y_mouse_events_have_key_events_warns_for_mouseover_without_focus"
    );
    diagnostic_case!(
        a11y_mouse_events_have_key_events_warns_for_mouseout_without_blur,
        "a11y/a11y_mouse_events_have_key_events_warns_for_mouseout_without_blur"
    );
    diagnostic_case!(
        a11y_mouse_events_have_key_events_no_warning_with_paired_handler,
        "a11y/a11y_mouse_events_have_key_events_no_warning_with_paired_handler"
    );
    diagnostic_case!(
        a11y_no_interactive_element_to_noninteractive_role_warns_for_button_role_presentation,
        "a11y/a11y_no_interactive_element_to_noninteractive_role_warns_for_button_role_presentation"
    );
    diagnostic_case!(
        a11y_no_interactive_element_to_noninteractive_role_warns_for_button_role_presentation_with_text,
        "a11y/a11y_no_interactive_element_to_noninteractive_role_warns_for_button_role_presentation_with_text"
    );
    diagnostic_case!(
        a11y_invalid_attribute_warns_for_anchor_hash_href,
        "a11y/a11y_invalid_attribute_warns_for_anchor_hash_href"
    );
    diagnostic_case!(
        a11y_autocomplete_valid_warns_for_invalid_input_token,
        "a11y/a11y_autocomplete_valid_warns_for_invalid_input_token"
    );
    diagnostic_case!(
        a11y_img_redundant_alt_warns_for_redundant_image_wording,
        "a11y/a11y_img_redundant_alt_warns_for_redundant_image_wording"
    );
    diagnostic_case!(
        a11y_misplaced_scope_warns_on_td,
        "a11y/a11y_misplaced_scope_warns_on_td"
    );
    diagnostic_case!(
        a11y_missing_content_warns_for_empty_h1,
        "a11y/a11y_missing_content_warns_for_empty_h1"
    );
    diagnostic_case!(
        a11y_media_has_caption_warns_for_video_without_caption_track,
        "a11y/a11y_media_has_caption_warns_for_video_without_caption_track"
    );
    diagnostic_case!(
        a11y_figcaption_parent_warns_outside_figure,
        "a11y/a11y_figcaption_parent_warns_outside_figure"
    );
    diagnostic_case!(
        a11y_figcaption_index_warns_for_middle_figcaption,
        "a11y/a11y_figcaption_index_warns_for_middle_figcaption"
    );
    diagnostic_case!(
        a11y_label_has_associated_control_warns_without_for_or_control,
        "a11y/a11y_label_has_associated_control_warns_without_for_or_control"
    );
    diagnostic_case!(
        a11y_consider_explicit_label_warns_for_icon_button,
        "a11y/a11y_consider_explicit_label_warns_for_icon_button"
    );
    diagnostic_case!(
        a11y_no_noninteractive_element_to_interactive_role_warns_for_div_role_button,
        "a11y/a11y_no_noninteractive_element_to_interactive_role_warns_for_div_role_button"
    );
    diagnostic_case!(
        a11y_no_noninteractive_element_to_interactive_role_warns_for_footer_role_button,
        "a11y/a11y_no_noninteractive_element_to_interactive_role_warns_for_footer_role_button"
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
    diagnostic_case!(
        a11y_tabindex_zero_no_warning,
        "a11y/a11y_tabindex_zero_no_warning"
    );
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
    diagnostic_case!(
        attribute_quoted_on_component,
        "attributes/attribute_quoted_on_component"
    );
    diagnostic_case!(
        attribute_quoted_custom_element,
        "attributes/attribute_quoted_custom_element"
    );
    diagnostic_case!(
        attribute_quoted_regular_element_no_warn,
        "attributes/attribute_quoted_regular_element_no_warn"
    );
    diagnostic_case!(
        component_attribute_illegal_colon_warns,
        "attributes/component_attribute_illegal_colon_warns"
    );
    diagnostic_case!(
        component_attribute_unquoted_sequence_errors,
        "attributes/component_attribute_unquoted_sequence_errors"
    );
    diagnostic_case!(
        regular_element_attribute_unquoted_sequence_errors,
        "attributes/regular_element_attribute_unquoted_sequence_errors"
    );
    diagnostic_case!(
        custom_element_attribute_unquoted_sequence_errors,
        "attributes/custom_element_attribute_unquoted_sequence_errors"
    );
    diagnostic_case!(
        svelte_element_attribute_unquoted_sequence_errors,
        "attributes/svelte_element_attribute_unquoted_sequence_errors"
    );
}

mod css {
    use super::*;

    diagnostic_case!(
        css_global_block_invalid_placement,
        "css/css_global_block_invalid_placement"
    );
    diagnostic_case!(
        css_global_invalid_placement,
        "css/css_global_invalid_placement"
    );
    diagnostic_case!(
        css_global_invalid_placement_multiple_non_global_after,
        "css/css_global_invalid_placement_multiple_non_global_after"
    );
    diagnostic_case!(
        css_global_invalid_placement_end_ok,
        "css/css_global_invalid_placement_end_ok"
    );
    diagnostic_case!(
        css_global_invalid_placement_start_ok,
        "css/css_global_invalid_placement_start_ok"
    );
    diagnostic_case!(
        css_global_invalid_selector_list,
        "css/css_global_invalid_selector_list"
    );
    diagnostic_case!(
        css_type_selector_invalid_placement,
        "css/css_type_selector_invalid_placement"
    );
    diagnostic_case!(
        css_global_invalid_selector,
        "css/css_global_invalid_selector"
    );
    diagnostic_case!(
        css_global_block_invalid_modifier_start,
        "css/css_global_block_invalid_modifier_start"
    );
    diagnostic_case!(
        css_global_block_invalid_combinator,
        "css/css_global_block_invalid_combinator"
    );
    diagnostic_case!(
        css_global_block_invalid_list,
        "css/css_global_block_invalid_list"
    );
    diagnostic_case!(
        css_global_block_invalid_declaration,
        "css/css_global_block_invalid_declaration"
    );
    diagnostic_case!(
        css_global_block_invalid_modifier,
        "css/css_global_block_invalid_modifier"
    );
    diagnostic_case!(
        css_nesting_selector_invalid_placement,
        "css/css_nesting_selector_invalid_placement"
    );
    diagnostic_case!(
        css_nesting_selector_valid_in_global,
        "css/css_nesting_selector_valid_in_global"
    );
    diagnostic_case!(css_selector_invalid, "css/css_selector_invalid");
    diagnostic_case!(
        css_global_block_with_nested_rules_ok,
        "css/css_global_block_with_nested_rules_ok"
    );
    diagnostic_case!(
        css_global_block_descendant_ok,
        "css/css_global_block_descendant_ok"
    );
    diagnostic_case!(
        css_global_nesting_modifier_start_in_global_block,
        "css/css_global_nesting_modifier_start_in_global_block"
    );
    diagnostic_case!(
        css_global_block_invalid_list_mixed,
        "css/css_global_block_invalid_list_mixed"
    );
    diagnostic_case!(
        css_nesting_in_compound_global_block_ok,
        "css/css_nesting_in_compound_global_block_ok"
    );
    diagnostic_case!(
        valid_scoped_css_no_diagnostics,
        "css/valid_scoped_css_no_diagnostics"
    );
}

mod css_prune {
    use super::*;

    diagnostic_case!(
        type_selector_matches_element,
        "css_prune/type_selector_matches_element"
    );
    diagnostic_case!(type_selector_no_match, "css_prune/type_selector_no_match");
    diagnostic_case!(
        type_selector_case_insensitive_css,
        "css_prune/type_selector_case_insensitive_css"
    );
    diagnostic_case!(
        universal_selector_always_matches,
        "css_prune/universal_selector_always_matches"
    );
    diagnostic_case!(
        multiple_selectors_mixed,
        "css_prune/multiple_selectors_mixed"
    );
    diagnostic_case!(
        class_selector_static_match,
        "css_prune/class_selector_static_match"
    );
    diagnostic_case!(class_selector_no_match, "css_prune/class_selector_no_match");
    diagnostic_case!(class_directive_match, "css_prune/class_directive_match");
    diagnostic_case!(id_selector_match, "css_prune/id_selector_match");
    diagnostic_case!(id_selector_no_match, "css_prune/id_selector_no_match");
    diagnostic_case!(
        global_functional_always_used,
        "css_prune/global_functional_always_used"
    );
    diagnostic_case!(
        global_block_always_used,
        "css_prune/global_block_always_used"
    );
    diagnostic_case!(
        descendant_combinator_match,
        "css_prune/descendant_combinator_match"
    );
    diagnostic_case!(
        descendant_combinator_deep_match,
        "css_prune/descendant_combinator_deep_match"
    );
    diagnostic_case!(
        descendant_combinator_no_match,
        "css_prune/descendant_combinator_no_match"
    );
    diagnostic_case!(
        child_combinator_direct_match,
        "css_prune/child_combinator_direct_match"
    );
    diagnostic_case!(
        child_combinator_indirect_no_match,
        "css_prune/child_combinator_indirect_no_match"
    );
    diagnostic_case!(
        adjacent_sibling_combinator_match,
        "css_prune/adjacent_sibling_combinator_match"
    );
    diagnostic_case!(
        adjacent_sibling_combinator_no_match,
        "css_prune/adjacent_sibling_combinator_no_match"
    );
    diagnostic_case!(
        general_sibling_combinator_match,
        "css_prune/general_sibling_combinator_match"
    );
    diagnostic_case!(
        general_sibling_combinator_no_match,
        "css_prune/general_sibling_combinator_no_match"
    );
    diagnostic_case!(no_elements_all_unused, "css_prune/no_elements_all_unused");
    diagnostic_case!(keyframes_not_warned, "css_prune/keyframes_not_warned");
    diagnostic_case!(
        media_query_unused_selector,
        "css_prune/media_query_unused_selector"
    );
    diagnostic_case!(
        media_query_used_selector,
        "css_prune/media_query_used_selector"
    );
    diagnostic_case!(
        spread_attribute_conservative_class_match,
        "css_prune/spread_attribute_conservative_class_match"
    );
    diagnostic_case!(is_selector_match, "css_prune/is_selector_match");
    diagnostic_case!(is_selector_no_match, "css_prune/is_selector_no_match");
    diagnostic_case!(
        is_selector_compound_no_match,
        "css_prune/is_selector_compound_no_match"
    );
    diagnostic_case!(where_selector_match, "css_prune/where_selector_match");
    diagnostic_case!(
        where_selector_complex_branch_conservative,
        "css_prune/where_selector_complex_branch_conservative"
    );
    diagnostic_case!(
        concat_attribute_selector_no_match,
        "css_prune/concat_attribute_selector_no_match"
    );
    diagnostic_case!(nested_element_match, "css_prune/nested_element_match");
    diagnostic_case!(
        deeply_nested_descendant,
        "css_prune/deeply_nested_descendant"
    );
    diagnostic_case!(implicit_nesting_match, "css_prune/implicit_nesting_match");
    diagnostic_case!(root_has_match, "css_prune/root_has_match");
    diagnostic_case!(escaped_selector_match, "css_prune/escaped_selector_match");
}

mod bind {
    use super::*;

    diagnostic_case!(
        validate_bind_invalid_name,
        "bind/validate_bind_invalid_name"
    );
    diagnostic_case!(
        validate_bind_invalid_expression,
        "bind/validate_bind_invalid_expression",
        ignore = "known mismatch: analyzer reports extra non_reactive_update alongside bind_invalid_expression"
    );
    diagnostic_case!(
        validate_bind_invalid_value,
        "bind/validate_bind_invalid_value",
        ignore = "known mismatch: analyzer reports bind_invalid_value while npm svelte/compiler reports no diagnostic"
    );
    diagnostic_case!(
        validate_bind_plain_let_is_valid,
        "bind/validate_bind_plain_let_is_valid",
        ignore = "known mismatch: analyzer and npm svelte/compiler report non_reactive_update on different spans"
    );
    diagnostic_case!(
        validate_attribute_contenteditable_missing,
        "bind/validate_attribute_contenteditable_missing"
    );
    diagnostic_case!(
        validate_bind_invalid_target,
        "bind/validate_bind_invalid_target"
    );
    diagnostic_case!(
        validate_bind_invalid_name_with_special_element_candidates,
        "bind/validate_bind_invalid_name_with_special_element_candidates",
        ignore = "known mismatch: analyzer reports extra attribute_contenteditable_missing alongside bind_invalid_name"
    );
    diagnostic_case!(
        validate_bind_invalid_parens,
        "bind/validate_bind_invalid_parens"
    );
    diagnostic_case!(
        validate_bind_getter_setter_without_parens,
        "bind/validate_bind_getter_setter_without_parens"
    );
    diagnostic_case!(
        validate_bind_group_invalid_expression,
        "bind/validate_bind_group_invalid_expression"
    );
    diagnostic_case!(
        validate_bind_sequence_reports_all_relevant_errors,
        "bind/validate_bind_sequence_reports_all_relevant_errors",
        ignore = "known mismatch: analyzer reports extra bind_invalid_expression and bind_invalid_parens beyond npm svelte/compiler"
    );
    diagnostic_case!(
        validate_bind_group_invalid_snippet_parameter,
        "bind/validate_bind_group_invalid_snippet_parameter",
        ignore = "known mismatch: analyzer reports bind_group_invalid_snippet_parameter in addition to npm svelte/compiler snippet_parameter_assignment"
    );
    diagnostic_case!(
        validate_attribute_contenteditable_dynamic,
        "bind/validate_attribute_contenteditable_dynamic"
    );
    diagnostic_case!(
        validate_attribute_invalid_type,
        "bind/validate_attribute_invalid_type"
    );
    diagnostic_case!(
        validate_attribute_invalid_multiple,
        "bind/validate_attribute_invalid_multiple"
    );
    diagnostic_case!(
        validate_bind_invalid_each_rest,
        "bind/validate_bind_invalid_each_rest",
        ignore = "known mismatch: analyzer reports bind_invalid_each_rest as a warning while npm svelte/compiler reports an error"
    );
    diagnostic_case!(
        validate_bind_checked_radio_target,
        "bind/validate_bind_checked_radio_target"
    );
    diagnostic_case!(
        validate_bind_files_wrong_input_type,
        "bind/validate_bind_files_wrong_input_type"
    );
    diagnostic_case!(
        validate_bind_member_expression_no_error,
        "bind/validate_bind_member_expression_no_error",
        ignore = "known mismatch: analyzer reports extra store_rune_conflict warning"
    );
    diagnostic_case!(
        validate_bind_getter_setter_no_error,
        "bind/validate_bind_getter_setter_no_error"
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
    diagnostic_case!(
        svelte_self_deprecated_warns_with_default_self_import_hint,
        "components/svelte_self_deprecated_warns_with_default_self_import_hint",
        ignore = "known mismatch: npm svelte/compiler currently reports an error while analyzer reports a warning"
    );
    diagnostic_case!(
        svelte_self_deprecated_warns_with_configured_self_import_hint,
        "components/svelte_self_deprecated_warns_with_configured_self_import_hint",
        ignore = "known mismatch: npm svelte/compiler currently reports an error while analyzer reports a warning"
    );
    diagnostic_case!(
        svelte_self_deprecated_uses_deconflicted_component_name,
        "components/svelte_self_deprecated_uses_deconflicted_component_name",
        ignore = "known mismatch: npm svelte/compiler currently reports an error while analyzer reports a warning"
    );
    diagnostic_case!(
        svelte_self_deprecated_uses_reserved_word_deconflicted_component_name,
        "components/svelte_self_deprecated_uses_reserved_word_deconflicted_component_name",
        ignore = "known mismatch: npm svelte/compiler currently reports an error while analyzer reports a warning"
    );
    diagnostic_case!(
        svelte_component_deprecated_warns_in_runes_mode,
        "components/svelte_component_deprecated_warns_in_runes_mode"
    );
    diagnostic_case!(
        svelte_component_deprecated_no_warn_in_legacy_mode,
        "components/svelte_component_deprecated_no_warn_in_legacy_mode"
    );
    diagnostic_case!(
        svelte_component_missing_this,
        "components/svelte_component_missing_this",
        ignore = "missing: parser-owned <svelte:component> missing this diagnostic (parser)"
    );
    diagnostic_case!(
        svelte_component_invalid_this_string,
        "components/svelte_component_invalid_this_string",
        ignore = "missing: parser-owned <svelte:component> invalid this diagnostic (parser)"
    );
    diagnostic_case!(
        svelte_self_deprecated_no_warn_in_legacy_mode,
        "components/svelte_self_deprecated_no_warn_in_legacy_mode",
        ignore = "known mismatch: npm svelte/compiler reports svelte_self_invalid_placement while analyzer emits no diagnostic"
    );
    diagnostic_case!(
        component_invalid_directive_use,
        "components/component_invalid_directive_use"
    );
    diagnostic_case!(
        component_on_modifier_only_allows_once,
        "components/component_on_modifier_only_allows_once"
    );
}

mod events {
    use super::*;

    diagnostic_case!(
        on_directive_invalid_modifier,
        "events/on_directive_invalid_modifier",
        ignore = "known mismatch: analyzer reports extra event_directive_deprecated and a11y warnings for legacy on: syntax"
    );
    diagnostic_case!(
        on_directive_passive_nonpassive_conflict,
        "events/on_directive_passive_nonpassive_conflict",
        ignore = "known mismatch: analyzer reports extra event_directive_deprecated and a11y warnings for legacy on: syntax"
    );
    diagnostic_case!(
        on_directive_mixed_syntax,
        "events/on_directive_mixed_syntax",
        ignore = "known mismatch: analyzer reports extra event_directive_deprecated and a11y warnings for legacy on: syntax"
    );
    diagnostic_case!(
        on_directive_mixed_syntax_svelte_element,
        "events/on_directive_mixed_syntax_svelte_element",
        ignore = "known mismatch: analyzer reports extra event_directive_deprecated warning for legacy on: syntax"
    );
    diagnostic_case!(
        on_directive_deprecated_in_runes_mode,
        "events/on_directive_deprecated_in_runes_mode"
    );
    diagnostic_case!(
        on_directive_not_deprecated_in_non_runes_mode,
        "events/on_directive_not_deprecated_in_non_runes_mode"
    );
    diagnostic_case!(
        on_directive_not_deprecated_in_non_runes_mode_svelte_window,
        "events/on_directive_not_deprecated_in_non_runes_mode_svelte_window",
        ignore = "diagnose-diagnostics: pending fix"
    );
}

mod directives {
    use super::*;

    diagnostic_case!(
        validate_transition_duplicate_transition,
        "directives/validate_transition_duplicate_transition"
    );
    diagnostic_case!(
        validate_transition_duplicate_in,
        "directives/validate_transition_duplicate_in"
    );
    diagnostic_case!(
        validate_transition_duplicate_out,
        "directives/validate_transition_duplicate_out"
    );
    diagnostic_case!(
        validate_transition_conflict_in,
        "directives/validate_transition_conflict_in"
    );
    diagnostic_case!(
        validate_transition_conflict_out,
        "directives/validate_transition_conflict_out"
    );
    diagnostic_case!(
        validate_use_directive_illegal_await_expression,
        "directives/validate_use_directive_illegal_await_expression",
        ignore = "known mismatch: npm svelte/compiler reports experimental_async while analyzer reports illegal_await_expression"
    );
    diagnostic_case!(
        validate_transition_illegal_await_expression,
        "directives/validate_transition_illegal_await_expression",
        ignore = "known mismatch: npm svelte/compiler reports experimental_async while analyzer reports illegal_await_expression"
    );
    diagnostic_case!(
        validate_animate_directive_illegal_await_expression,
        "directives/validate_animate_directive_illegal_await_expression",
        ignore = "known mismatch: npm svelte/compiler reports experimental_async while analyzer reports illegal_await_expression"
    );
}

mod options {
    use super::*;

    diagnostic_case!(
        options_deprecated_accessors_runes,
        "options/options_deprecated_accessors_runes",
        ignore =
            "known mismatch: Rust warning span is 0..0 while reference spans the accessors option"
    );
    diagnostic_case!(
        options_deprecated_accessors_legacy,
        "options/options_deprecated_accessors_legacy"
    );
    diagnostic_case!(
        options_deprecated_immutable_runes,
        "options/options_deprecated_immutable_runes",
        ignore =
            "known mismatch: Rust warning span is 0..0 while reference spans the immutable option"
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
    diagnostic_case!(
        validate_custom_element_props_identifier_warns,
        "options/validate_custom_element_props_identifier_warns"
    );
    diagnostic_case!(
        validate_custom_element_props_rest_warns,
        "options/validate_custom_element_props_rest_warns",
        ignore = "known mismatch: Rust warning span highlights the rest binding while npm svelte/compiler highlights the rest identifier usage"
    );
    diagnostic_case!(
        validate_custom_element_props_destructured_no_warn,
        "options/validate_custom_element_props_destructured_no_warn"
    );
    diagnostic_case!(
        validate_custom_element_with_explicit_props_config_no_warn,
        "options/validate_custom_element_with_explicit_props_config_no_warn"
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
    diagnostic_case!(
        validate_props_invalid_placement_inside_function,
        "props/validate_props_invalid_placement_inside_function"
    );
    diagnostic_case!(
        validate_props_invalid_placement_in_module_script,
        "props/validate_props_invalid_placement_in_module_script"
    );
    diagnostic_case!(
        validate_props_invalid_arguments_in_module_script,
        "props/validate_props_invalid_arguments_in_module_script"
    );
    diagnostic_case!(validate_props_duplicate, "props/validate_props_duplicate");
    diagnostic_case!(
        validate_props_and_props_id_coexist,
        "props/validate_props_and_props_id_coexist"
    );
    diagnostic_case!(
        validate_props_invalid_pattern_computed_key,
        "props/validate_props_invalid_pattern_computed_key"
    );
    diagnostic_case!(
        validate_props_id_invalid_placement_inside_function,
        "props/validate_props_id_invalid_placement_inside_function"
    );
    diagnostic_case!(
        validate_props_id_invalid_placement_in_module_script,
        "props/validate_props_id_invalid_placement_in_module_script"
    );
    diagnostic_case!(
        validate_props_id_invalid_arguments_in_module_script,
        "props/validate_props_id_invalid_arguments_in_module_script"
    );
    diagnostic_case!(
        validate_props_illegal_name_rest_member_access,
        "props/validate_props_illegal_name_rest_member_access"
    );
    diagnostic_case!(
        validate_props_illegal_name_identifier_pattern_member_access,
        "props/validate_props_illegal_name_identifier_pattern_member_access"
    );
    diagnostic_case!(
        validate_legacy_props_invalid_in_runes_mode,
        "props/validate_legacy_props_invalid_in_runes_mode"
    );
    diagnostic_case!(
        validate_legacy_rest_props_invalid_in_runes_mode,
        "props/validate_legacy_rest_props_invalid_in_runes_mode"
    );
    diagnostic_case!(
        validate_props_normal_member_access_no_error,
        "props/validate_props_normal_member_access_no_error",
        ignore = "known mismatch: npm svelte/compiler reports state_referenced_locally while analyzer reports no diagnostics"
    );
    diagnostic_case!(
        validate_props_typed_children_snippet_no_diagnostic,
        "props/validate_props_typed_children_snippet_no_diagnostic"
    );
    diagnostic_case!(
        validate_export_let_unused,
        "props/validate_export_let_unused"
    );
}

mod runes {
    use super::*;

    diagnostic_case!(
        validate_effect_invalid_placement_fn_arg,
        "runes/validate_effect_invalid_placement_fn_arg"
    );
    diagnostic_case!(
        validate_effect_pre_invalid_placement_assignment,
        "runes/validate_effect_pre_invalid_placement_assignment"
    );
    diagnostic_case!(
        validate_effect_wrong_arg_count,
        "runes/validate_effect_wrong_arg_count"
    );
    diagnostic_case!(
        validate_before_after_update_invalid_import,
        "runes/validate_before_after_update_invalid_import"
    );
    diagnostic_case!(
        validate_before_after_update_invalid_import_split_statements,
        "runes/validate_before_after_update_invalid_import_split_statements"
    );
    diagnostic_case!(
        validate_derived_wrong_arg_count,
        "runes/validate_derived_wrong_arg_count"
    );
    diagnostic_case!(
        validate_derived_by_wrong_arg_count,
        "runes/validate_derived_by_wrong_arg_count"
    );
    diagnostic_case!(
        validate_derived_rune_invalid_usage_in_non_runes_mode,
        "runes/validate_derived_rune_invalid_usage_in_non_runes_mode",
        ignore = "known mismatch: npm svelte/compiler does not report rune_invalid_usage for $derived in non-runes mode on this repro"
    );
    diagnostic_case!(
        validate_derived_destructured_rune_invalid_usage_in_non_runes_mode,
        "runes/validate_derived_destructured_rune_invalid_usage_in_non_runes_mode",
        ignore = "known mismatch: npm svelte/compiler does not report rune_invalid_usage for destructured $derived in non-runes mode on this repro"
    );
    diagnostic_case!(
        validate_derived_rune_allowed_in_runes_mode,
        "runes/validate_derived_rune_allowed_in_runes_mode"
    );
    diagnostic_case!(
        validate_derived_invalid_export,
        "runes/validate_derived_invalid_export"
    );
    diagnostic_case!(
        validate_derived_invalid_export_specifier,
        "runes/validate_derived_invalid_export_specifier",
        ignore = "known mismatch: analyzer reports derived_invalid_export for export specifier but npm svelte/compiler reports no diagnostic"
    );
    diagnostic_case!(
        validate_derived_invalid_default_export,
        "runes/validate_derived_invalid_default_export",
        ignore = "known mismatch: npm svelte/compiler reports module_illegal_default_export while analyzer reports derived_invalid_export plus state_referenced_locally"
    );
    diagnostic_case!(
        validate_state_invalid_placement_bare_expr,
        "runes/validate_state_invalid_placement_bare_expr"
    );
    diagnostic_case!(
        validate_state_invalid_placement_fn_arg,
        "runes/validate_state_invalid_placement_fn_arg"
    );
    diagnostic_case!(
        validate_state_too_many_args,
        "runes/validate_state_too_many_args"
    );
    diagnostic_case!(
        validate_state_frozen_renamed,
        "runes/validate_state_frozen_renamed"
    );
    diagnostic_case!(validate_state_is_removed, "runes/validate_state_is_removed");
    diagnostic_case!(
        validate_state_valid_positions,
        "runes/validate_state_valid_positions"
    );
    diagnostic_case!(
        validate_state_constructor_private_field,
        "runes/validate_state_constructor_private_field"
    );
    diagnostic_case!(
        validate_state_nested_class_in_constructor,
        "runes/validate_state_nested_class_in_constructor"
    );
    diagnostic_case!(
        validate_state_raw_too_many_args,
        "runes/validate_state_raw_too_many_args"
    );
    diagnostic_case!(
        validate_state_referenced_locally_for_derived,
        "runes/validate_state_referenced_locally_for_derived"
    );
    diagnostic_case!(
        validate_state_referenced_locally_derived_type_is_derived_inside_state_arg,
        "runes/validate_state_referenced_locally_derived_type_is_derived_inside_state_arg"
    );
    diagnostic_case!(
        validate_state_referenced_locally_derived_no_warning_across_fn_boundary,
        "runes/validate_state_referenced_locally_derived_no_warning_across_fn_boundary"
    );
    diagnostic_case!(
        validate_state_referenced_locally_for_reassigned_state,
        "runes/validate_state_referenced_locally_for_reassigned_state"
    );
    diagnostic_case!(
        validate_state_referenced_locally_for_primitive_state,
        "runes/validate_state_referenced_locally_for_primitive_state"
    );
    diagnostic_case!(
        validate_state_referenced_locally_no_warning_for_proxy_state,
        "runes/validate_state_referenced_locally_no_warning_for_proxy_state"
    );
    diagnostic_case!(
        validate_state_referenced_locally_for_state_raw,
        "runes/validate_state_referenced_locally_for_state_raw"
    );
    diagnostic_case!(
        validate_state_referenced_locally_no_warning_across_fn_boundary_state,
        "runes/validate_state_referenced_locally_no_warning_across_fn_boundary_state"
    );
    diagnostic_case!(
        validate_state_invalid_export_for_reassigned_state,
        "runes/validate_state_invalid_export_for_reassigned_state"
    );
    diagnostic_case!(
        validate_state_invalid_export_for_reassigned_state_raw,
        "runes/validate_state_invalid_export_for_reassigned_state_raw"
    );
    diagnostic_case!(
        validate_state_invalid_export_no_error_without_reassignment,
        "runes/validate_state_invalid_export_no_error_without_reassignment"
    );
    diagnostic_case!(
        validate_state_invalid_export_for_reassigned_state_export_specifier,
        "runes/validate_state_invalid_export_for_reassigned_state_export_specifier",
        ignore = "known mismatch: analyzer reports state_invalid_export for module export specifier while npm svelte/compiler reports no diagnostic"
    );
    diagnostic_case!(
        validate_state_invalid_export_for_reassigned_state_default_export,
        "runes/validate_state_invalid_export_for_reassigned_state_default_export"
    );
    diagnostic_case!(
        validate_state_invalid_export_no_error_for_default_export_without_reassignment,
        "runes/validate_state_invalid_export_no_error_for_default_export_without_reassignment"
    );
    diagnostic_case!(
        validate_effect_pre_wrong_arg_count,
        "runes/validate_effect_pre_wrong_arg_count"
    );
    diagnostic_case!(
        validate_effect_root_wrong_arg_count,
        "runes/validate_effect_root_wrong_arg_count"
    );
    diagnostic_case!(
        validate_effect_tracking_with_argument,
        "runes/validate_effect_tracking_with_argument"
    );
    diagnostic_case!(
        validate_state_eager_no_args,
        "runes/validate_state_eager_no_args"
    );
    diagnostic_case!(
        validate_state_eager_too_many_args,
        "runes/validate_state_eager_too_many_args"
    );
    diagnostic_case!(
        validate_bindable_invalid_location,
        "runes/validate_bindable_invalid_location"
    );
    diagnostic_case!(
        validate_reactive_declaration_invalid_placement,
        "runes/validate_reactive_declaration_invalid_placement"
    );
    diagnostic_case!(
        validate_reactive_declaration_cycle,
        "runes/validate_reactive_declaration_cycle"
    );
    diagnostic_case!(
        validate_reactive_declaration_module_script_dependency,
        "runes/validate_reactive_declaration_module_script_dependency"
    );
    diagnostic_case!(
        validate_bindable_invalid_location_inside_arrow,
        "runes/validate_bindable_invalid_location_inside_arrow"
    );
    diagnostic_case!(
        validate_bindable_too_many_args,
        "runes/validate_bindable_too_many_args"
    );
    diagnostic_case!(
        validate_inspect_requires_arguments,
        "runes/validate_inspect_requires_arguments"
    );
    diagnostic_case!(
        validate_inspect_zero_args,
        "runes/validate_inspect_zero_args"
    );
    diagnostic_case!(
        validate_inspect_one_or_more_args_ok,
        "runes/validate_inspect_one_or_more_args_ok"
    );
    diagnostic_case!(
        validate_inspect_with_requires_callback,
        "runes/validate_inspect_with_requires_callback"
    );
    diagnostic_case!(
        validate_inspect_with_wrong_arg_count_zero,
        "runes/validate_inspect_with_wrong_arg_count_zero"
    );
    diagnostic_case!(
        validate_inspect_with_wrong_arg_count_two,
        "runes/validate_inspect_with_wrong_arg_count_two"
    );
    diagnostic_case!(
        validate_inspect_with_one_arg_ok,
        "runes/validate_inspect_with_one_arg_ok"
    );
    diagnostic_case!(
        validate_inspect_derived_no_state_referenced_locally_warning,
        "runes/validate_inspect_derived_no_state_referenced_locally_warning"
    );
    diagnostic_case!(
        validate_inspect_trace_wrong_arg_count,
        "runes/validate_inspect_trace_wrong_arg_count"
    );
    diagnostic_case!(
        validate_inspect_trace_too_many_args,
        "runes/validate_inspect_trace_too_many_args"
    );
    diagnostic_case!(
        validate_inspect_trace_invalid_placement,
        "runes/validate_inspect_trace_invalid_placement"
    );
    diagnostic_case!(
        validate_inspect_trace_invalid_placement_top_level,
        "runes/validate_inspect_trace_invalid_placement_top_level"
    );
    diagnostic_case!(
        validate_inspect_trace_invalid_placement_not_first_stmt,
        "runes/validate_inspect_trace_invalid_placement_not_first_stmt"
    );
    diagnostic_case!(
        validate_inspect_trace_zero_args_ok,
        "runes/validate_inspect_trace_zero_args_ok"
    );
    diagnostic_case!(
        validate_inspect_trace_one_arg_ok,
        "runes/validate_inspect_trace_one_arg_ok"
    );
    diagnostic_case!(
        validate_inspect_trace_valid_in_arrow,
        "runes/validate_inspect_trace_valid_in_arrow"
    );
    diagnostic_case!(
        validate_inspect_trace_generator_invalid,
        "runes/validate_inspect_trace_generator_invalid"
    );
    diagnostic_case!(
        validate_inspect_trace_generator_rejected,
        "runes/validate_inspect_trace_generator_rejected"
    );
}

mod stores {
    use super::*;

    diagnostic_case!(
        state_rune_no_store_rune_conflict,
        "stores/state_rune_no_store_rune_conflict"
    );
    diagnostic_case!(
        state_rune_no_conflict_with_other_rune_calls,
        "stores/state_rune_no_conflict_with_other_rune_calls",
        ignore = "diagnose-diagnostics: pending fix"
    );
    diagnostic_case!(
        validate_store_rune_conflict,
        "stores/validate_store_rune_conflict"
    );
    diagnostic_case!(
        validate_store_invalid_scoped_subscription,
        "stores/validate_store_invalid_scoped_subscription"
    );
    diagnostic_case!(
        validate_store_invalid_subscription_in_module,
        "stores/validate_store_invalid_subscription_in_module"
    );
}

mod host {
    use super::*;

    diagnostic_case!(
        validate_host_invalid_placement_without_custom_element,
        "host/validate_host_invalid_placement_without_custom_element",
        ignore = "known mismatch: analyzer reports extra store_rune_conflict warning alongside host_invalid_placement"
    );
    diagnostic_case!(
        validate_host_invalid_arguments,
        "host/validate_host_invalid_arguments",
        ignore = "known mismatch: analyzer reports extra store_rune_conflict warning alongside rune_invalid_arguments"
    );
}

mod special {
    use super::*;

    diagnostic_case!(
        svelte_head_illegal_attribute,
        "special/svelte_head_illegal_attribute",
        ignore = "known mismatch: analyzer is missing svelte_head_illegal_attribute"
    );
    diagnostic_case!(
        svelte_window_illegal_attribute_class,
        "special/svelte_window_illegal_attribute_class"
    );
    diagnostic_case!(
        svelte_window_illegal_attribute_spread,
        "special/svelte_window_illegal_attribute_spread"
    );
    diagnostic_case!(
        svelte_document_illegal_attribute_class,
        "special/svelte_document_illegal_attribute_class"
    );
    diagnostic_case!(
        svelte_document_illegal_attribute_spread,
        "special/svelte_document_illegal_attribute_spread"
    );
    diagnostic_case!(
        svelte_body_illegal_attribute_class,
        "special/svelte_body_illegal_attribute_class"
    );
    diagnostic_case!(
        svelte_body_illegal_attribute_spread,
        "special/svelte_body_illegal_attribute_spread"
    );
    diagnostic_case!(
        svelte_window_invalid_content,
        "special/svelte_window_invalid_content"
    );
    diagnostic_case!(
        svelte_document_invalid_content,
        "special/svelte_document_invalid_content"
    );
    diagnostic_case!(
        svelte_body_invalid_content,
        "special/svelte_body_invalid_content"
    );
    diagnostic_case!(
        title_illegal_attribute,
        "special/title_illegal_attribute",
        ignore = "known mismatch: analyzer is missing title_illegal_attribute"
    );
    diagnostic_case!(
        title_invalid_content,
        "special/title_invalid_content",
        ignore = "known mismatch: analyzer is missing title_invalid_content"
    );
}

mod module {
    use super::*;

    diagnostic_case!(
        validate_module_illegal_default_export,
        "module/validate_module_illegal_default_export"
    );
    diagnostic_case!(
        validate_module_illegal_default_export_function,
        "module/validate_module_illegal_default_export_function"
    );
    diagnostic_case!(
        validate_module_illegal_default_export_specifier,
        "module/validate_module_illegal_default_export_specifier"
    );
}

mod slots {
    use super::*;

    diagnostic_case!(
        slot_attribute_invalid_expression_value,
        "slots/slot_attribute_invalid_expression_value"
    );
    diagnostic_case!(
        slot_attribute_static_value_ok,
        "slots/slot_attribute_static_value_ok"
    );
    diagnostic_case!(
        slot_attribute_duplicate_reports_second_named_slot,
        "slots/slot_attribute_duplicate_reports_second_named_slot"
    );
    diagnostic_case!(
        slot_attribute_duplicate_component_child_reports_second_named_slot,
        "slots/slot_attribute_duplicate_component_child_reports_second_named_slot"
    );
    diagnostic_case!(
        slot_default_duplicate_reports_implicit_default_content,
        "slots/slot_default_duplicate_reports_implicit_default_content"
    );
    diagnostic_case!(
        slot_default_duplicate_component_child_reports_slotted_component_conflict,
        "slots/slot_default_duplicate_component_child_reports_slotted_component_conflict"
    );
    diagnostic_case!(
        slot_distinct_named_slots_do_not_conflict,
        "slots/slot_distinct_named_slots_do_not_conflict"
    );
    diagnostic_case!(
        slot_default_duplicate_ignores_whitespace_and_other_named_slots,
        "slots/slot_default_duplicate_ignores_whitespace_and_other_named_slots"
    );
    diagnostic_case!(
        const_tag_inside_slotted_element_is_allowed,
        "slots/const_tag_inside_slotted_element_is_allowed"
    );
    diagnostic_case!(
        slot_attribute_invalid_placement_root,
        "slots/slot_attribute_invalid_placement_root"
    );
    diagnostic_case!(
        slot_attribute_invalid_placement_nested_inside_component,
        "slots/slot_attribute_invalid_placement_nested_inside_component"
    );
    diagnostic_case!(
        slot_element_invalid_name_dynamic,
        "slots/slot_element_invalid_name_dynamic"
    );
    diagnostic_case!(
        slot_element_invalid_name_default,
        "slots/slot_element_invalid_name_default"
    );
    diagnostic_case!(
        slot_element_invalid_attribute_class,
        "slots/slot_element_invalid_attribute_class"
    );
    diagnostic_case!(slot_snippet_conflict, "slots/slot_snippet_conflict");
    diagnostic_case!(
        svelte_fragment_invalid_placement_root,
        "slots/svelte_fragment_invalid_placement_root"
    );
    diagnostic_case!(
        svelte_fragment_invalid_attribute_class,
        "slots/svelte_fragment_invalid_attribute_class"
    );
    diagnostic_case!(
        let_directive_invalid_placement_svelte_window,
        "slots/let_directive_invalid_placement_svelte_window"
    );
    diagnostic_case!(
        let_directive_invalid_placement_svelte_body,
        "slots/let_directive_invalid_placement_svelte_body"
    );
}

mod const_tag {
    use super::*;

    diagnostic_case!(
        validate_const_tag_invalid_placement_root,
        "const_tag/validate_const_tag_invalid_placement_root"
    );
    diagnostic_case!(
        validate_const_tag_invalid_placement_inside_element,
        "const_tag/validate_const_tag_invalid_placement_inside_element"
    );
    diagnostic_case!(
        validate_const_tag_valid_placement_each,
        "const_tag/validate_const_tag_valid_placement_each"
    );
    diagnostic_case!(
        validate_const_tag_valid_placement_if,
        "const_tag/validate_const_tag_valid_placement_if"
    );
    diagnostic_case!(
        validate_const_tag_valid_placement_key,
        "const_tag/validate_const_tag_valid_placement_key"
    );
    diagnostic_case!(
        validate_const_tag_invalid_expression,
        "const_tag/validate_const_tag_invalid_expression"
    );
    diagnostic_case!(
        validate_const_tag_parenthesized_sequence_ok,
        "const_tag/validate_const_tag_parenthesized_sequence_ok"
    );
    diagnostic_case!(
        validate_const_tag_invalid_reference_component_children_async,
        "const_tag/validate_const_tag_invalid_reference_component_children_async",
        ignore = "known mismatch: analyzer reports extra snippet_conflict alongside const_tag_invalid_reference"
    );
    diagnostic_case!(
        validate_const_tag_invalid_reference_boundary_failed_async,
        "const_tag/validate_const_tag_invalid_reference_boundary_failed_async"
    );
    diagnostic_case!(
        validate_const_tag_invalid_reference_boundary_pending_async,
        "const_tag/validate_const_tag_invalid_reference_boundary_pending_async"
    );
    diagnostic_case!(
        validate_const_tag_invalid_reference_skipped_without_async,
        "const_tag/validate_const_tag_invalid_reference_skipped_without_async"
    );
    diagnostic_case!(
        validate_const_tag_reference_inside_snippet_scope_is_allowed_async,
        "const_tag/validate_const_tag_reference_inside_snippet_scope_is_allowed_async"
    );
}

mod each {
    use super::*;

    diagnostic_case!(
        validate_each_animation_missing_key,
        "each/validate_each_animation_missing_key"
    );
    diagnostic_case!(
        validate_each_animation_invalid_placement,
        "each/validate_each_animation_invalid_placement"
    );
    diagnostic_case!(
        validate_each_animation_duplicate,
        "each/validate_each_animation_duplicate"
    );
    diagnostic_case!(
        validate_each_item_invalid_assignment,
        "each/validate_each_item_invalid_assignment"
    );
    diagnostic_case!(
        each_block_shadowing_does_not_mutate_rune,
        "each/each_block_shadowing_does_not_mutate_rune"
    );
    diagnostic_case!(
        validate_each_item_invalid_assignment_bind_identifier,
        "each/validate_each_item_invalid_assignment_bind_identifier"
    );
    diagnostic_case!(
        validate_each_item_bind_member_expression_no_invalid_assignment,
        "each/validate_each_item_bind_member_expression_no_invalid_assignment"
    );
    diagnostic_case!(
        validate_each_item_invalid_assignment_array_destructure,
        "each/validate_each_item_invalid_assignment_array_destructure",
        ignore = "known mismatch: analyzer reports each_item_invalid_assignment for array destructure while npm svelte/compiler reports no diagnostic"
    );
    diagnostic_case!(
        validate_each_item_invalid_assignment_nested_object_destructure,
        "each/validate_each_item_invalid_assignment_nested_object_destructure",
        ignore = "known mismatch: analyzer reports each_item_invalid_assignment for nested object destructure while npm svelte/compiler reports no diagnostic"
    );
    diagnostic_case!(
        validate_each_key_without_as,
        "each/validate_each_key_without_as"
    );
}

mod snippets {
    use super::*;

    diagnostic_case!(
        validate_snippet_parameter_assignment,
        "snippets/validate_snippet_parameter_assignment"
    );
    diagnostic_case!(
        validate_snippet_parameter_assignment_in_nested_target,
        "snippets/validate_snippet_parameter_assignment_in_nested_target",
        ignore = "known mismatch: analyzer reports snippet_parameter_assignment for nested destructuring target while npm svelte/compiler reports no diagnostic"
    );
    diagnostic_case!(
        validate_snippet_invalid_rest_parameter,
        "snippets/validate_snippet_invalid_rest_parameter"
    );
    diagnostic_case!(
        validate_snippet_shadowing_prop,
        "snippets/validate_snippet_shadowing_prop"
    );
    diagnostic_case!(
        validate_snippet_conflict,
        "snippets/validate_snippet_conflict"
    );
    diagnostic_case!(
        fragment_facts_track_non_trivial_child_counts,
        "snippets/fragment_facts_track_non_trivial_child_counts"
    );
    diagnostic_case!(
        validate_snippet_children_without_other_content_has_no_conflict,
        "snippets/validate_snippet_children_without_other_content_has_no_conflict"
    );
    diagnostic_case!(
        validate_snippet_invalid_export,
        "snippets/validate_snippet_invalid_export",
        ignore = "known mismatch: analyzer and npm svelte/compiler report snippet_invalid_export on different spans"
    );
    diagnostic_case!(
        validate_snippet_invalid_export_no_false_positive,
        "snippets/validate_snippet_invalid_export_no_false_positive"
    );
    diagnostic_case!(
        validate_snippet_invalid_export_module_bound_no_fire,
        "snippets/validate_snippet_invalid_export_module_bound_no_fire"
    );
}

mod template {
    use super::*;

    diagnostic_case!(
        invalid_text_parent_uses_topology_ancestor_lookup,
        "template/invalid_text_parent_uses_topology_ancestor_lookup"
    );
    diagnostic_case!(
        await_valid_then_catch_no_unexpected_character,
        "template/await_valid_then_catch_no_unexpected_character"
    );
    diagnostic_case!(
        debug_tag_valid_runes_no_unexpected_character,
        "template/debug_tag_valid_runes_no_unexpected_character"
    );
    diagnostic_case!(
        debug_tag_non_runes_skips_opening_tag_check,
        "template/debug_tag_non_runes_skips_opening_tag_check"
    );
    diagnostic_case!(
        validate_key_block_empty_warns,
        "template/validate_key_block_empty_warns"
    );
    diagnostic_case!(
        fragment_facts_single_child_supports_block_empty,
        "template/fragment_facts_single_child_supports_block_empty"
    );
    diagnostic_case!(
        validate_text_invalid_placement,
        "template/validate_text_invalid_placement"
    );
    diagnostic_case!(
        validate_expression_tag_invalid_placement,
        "template/validate_expression_tag_invalid_placement"
    );
    diagnostic_case!(
        validate_text_bidirectional_control_warning,
        "template/validate_text_bidirectional_control_warning"
    );
    diagnostic_case!(
        validate_text_bidirectional_control_warning_ignored,
        "template/validate_text_bidirectional_control_warning_ignored"
    );
    diagnostic_case!(
        textarea_invalid_content_fires,
        "template/textarea_invalid_content_fires"
    );
    diagnostic_case!(
        textarea_no_conflict_without_value_attr,
        "template/textarea_no_conflict_without_value_attr"
    );
    diagnostic_case!(
        validate_non_reactive_update_for_direct_template_read,
        "template/validate_non_reactive_update_for_direct_template_read",
        ignore = "known mismatch: analyzer reports non_reactive_update on a different span than npm svelte/compiler"
    );
    diagnostic_case!(
        validate_non_reactive_update_no_warning_across_function_boundary,
        "template/validate_non_reactive_update_no_warning_across_function_boundary",
        ignore = "known mismatch: npm svelte/compiler reports a11y_consider_explicit_label for unlabeled button while analyzer reports no diagnostic"
    );
    diagnostic_case!(
        validate_non_reactive_update_bind_this_no_warning_without_dynamic_block,
        "template/validate_non_reactive_update_bind_this_no_warning_without_dynamic_block"
    );
    diagnostic_case!(
        validate_non_reactive_update_bind_this_warns_inside_if_block,
        "template/validate_non_reactive_update_bind_this_warns_inside_if_block",
        ignore = "known mismatch: analyzer reports non_reactive_update on a different span than npm svelte/compiler"
    );
}
