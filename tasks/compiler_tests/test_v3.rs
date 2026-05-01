use std::{
    fs::{File, read_to_string},
    io::Write,
    path::Path,
};

use pretty_assertions::assert_eq;
use rstest::rstest;
use svelte_compiler::{CompileOptions, ModuleCompileOptions, Namespace, compile, compile_module};

fn normalize_css(s: &str) -> String {
    s.lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .filter(|l| !is_css_comment_line(l))
        .flat_map(|line| line.split_whitespace())
        .collect::<Vec<_>>()
        .join(" ")
}

fn is_css_comment_line(line: &str) -> bool {
    let s = line.trim();
    s.starts_with("/*") && s.ends_with("*/")
}

fn case_input_and_options(case: &str) -> (String, CompileOptions) {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("cases2")
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

fn assert_compiler(case: &str) {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("cases2")
        .join(case)
        .join("case.svelte");
    let (input, opts) = case_input_and_options(case);
    let result = compile(&input, &opts);
    let js = result
        .js
        .unwrap_or_else(|| panic!("[{case}] compile produced no JS"));

    let dir = path.parent().expect("test invariant");
    let expected_js = read_to_string(dir.join("case-svelte.js")).expect("test invariant");

    File::create(dir.join("case-rust.js"))
        .expect("test invariant")
        .write_all(js.as_bytes())
        .expect("test invariant");

    assert_eq!(js, expected_js, "[{case}] JS mismatch");

    let expected_css_path = dir.join("case-svelte.css");
    if expected_css_path.exists() {
        let expected_css = read_to_string(&expected_css_path).expect("test invariant");
        let actual_css = result.css.unwrap_or_default();
        File::create(dir.join("case-rust.css"))
            .expect("test invariant")
            .write_all(actual_css.as_bytes())
            .expect("test invariant");
        assert_eq!(
            normalize_css(&actual_css),
            normalize_css(&expected_css),
            "[{case}] CSS mismatch"
        );
    }
}

#[rstest]
fn css_scope_class_in_snippet() {
    assert_compiler("css_scope_class_in_snippet");
}

#[rstest]
fn css_scope_svelte_element_class() {
    assert_compiler("css_scope_svelte_element_class");
}

#[rstest]
fn css_scope_class_object() {
    assert_compiler("css_scope_class_object");
}

#[rstest]
fn css_scope_spread_attribute() {
    assert_compiler("css_scope_spread_attribute");
}

#[rstest]
fn css_unused_external() {
    assert_compiler("css_unused_external");
}

#[rstest]
fn css_unused_injected() {
    assert_compiler("css_unused_injected");
}

#[rstest]
fn css_nested_style() {
    assert_compiler("css_nested_style");
}

#[rstest]
fn css_scoped_id_selector() {
    assert_compiler("css_scoped_id_selector");
}

#[rstest]
fn css_scoped_attr_presence() {
    assert_compiler("css_scoped_attr_presence");
}

#[rstest]
fn css_scoped_attr_value_selector() {
    assert_compiler("css_scoped_attr_value_selector");
}

#[rstest]
fn css_scoped_attr_matcher_operators() {
    assert_compiler("css_scoped_attr_matcher_operators");
}

#[rstest]
fn css_scoped_attr_name_casefolding() {
    assert_compiler("css_scoped_attr_name_casefolding");
}

#[rstest]
fn css_pseudo_compound_unused_but_scoped() {
    assert_compiler("css_pseudo_compound_unused_but_scoped");
}

#[rstest]
fn css_snippet_descendant_scope_boundary() {
    assert_compiler("css_snippet_descendant_scope_boundary");
}

#[rstest]
fn css_snippet_sibling_boundary() {
    assert_compiler("css_snippet_sibling_boundary");
}

#[rstest]
fn css_component_snippet_descendant_boundary() {
    assert_compiler("css_component_snippet_descendant_boundary");
}

#[rstest]
fn css_pseudo_has() {
    assert_compiler("css_pseudo_has");
}

#[rstest]
fn css_pseudo_not_scoped() {
    assert_compiler("css_pseudo_not_scoped");
}

#[rstest]
fn css_nesting_selector_scoped() {
    assert_compiler("css_nesting_selector_scoped");
}

#[rstest]
fn css_root_has_scoped() {
    assert_compiler("css_root_has_scoped");
}

#[rstest]
fn css_escaped_selector_scoped() {
    assert_compiler("css_escaped_selector_scoped");
}

#[rstest]
fn css_dynamic_attr_selector_match() {
    assert_compiler("css_dynamic_attr_selector_match");
}

#[rstest]
fn css_comments_preserved() {
    assert_compiler("css_comments_preserved");
}

#[rstest]
fn script_module_exports_ordering_with_snippets() {
    assert_compiler("script_module_exports_ordering_with_snippets");
}

#[rstest]
fn warn_attr_avoid_is() {
    assert_compiler("warn_attr_avoid_is");
}

#[rstest]
fn warn_attr_illegal_colon() {
    assert_compiler("warn_attr_illegal_colon");
}

#[rstest]
fn warn_attr_invalid_prop_name() {
    assert_compiler("warn_attr_invalid_prop_name");
}

#[rstest]
fn warn_slot_deprecated() {
    assert_compiler("warn_slot_deprecated");
}

#[rstest]
fn slot_named_fallback() {
    assert_compiler("slot_named_fallback");
}

#[rstest]
fn legacy_slot_dev_mixed() {
    assert_compiler("legacy_slot_dev_mixed");
}

#[rstest]
fn slot_props_default() {
    assert_compiler("slot_props_default");
}

#[rstest]
fn slot_props_spread() {
    assert_compiler("slot_props_spread");
}

#[test]
fn slot_props_dynamic_state() {
    assert_compiler("slot_props_dynamic_state");
}

#[test]
fn slot_props_dynamic_call() {
    assert_compiler("slot_props_dynamic_call");
}

#[rstest]
fn warn_script_context_deprecated() {
    assert_compiler("warn_script_context_deprecated");
}

#[rstest]
fn head_with_special_elements() {
    assert_compiler("head_with_special_elements");
}

#[rstest]
fn head_with_snippets() {
    assert_compiler("head_with_snippets");
}

#[rstest]
fn push_binding_group_order() {
    assert_compiler("push_binding_group_order");
}

#[rstest]
fn bind_group_order_with_stores() {
    assert_compiler("bind_group_order_with_stores");
}

#[rstest]
#[ignore = "diagnose: pending fix"]
fn css_injected_append_styles_with_stores_order() {
    assert_compiler("css_injected_append_styles_with_stores_order");
}

#[rstest]
fn css_scoped_basic() {
    assert_compiler("css_scoped_basic");
}

#[rstest]
fn css_injected() {
    assert_compiler("css_injected");
}

#[rstest]
fn css_global_basic() {
    assert_compiler("css_global_basic");
}

#[rstest]
fn css_global_block() {
    assert_compiler("css_global_block");
}

#[rstest]
fn css_global_compound() {
    assert_compiler("css_global_compound");
}

#[rstest]
fn css_global_in_pseudo() {
    assert_compiler("css_global_in_pseudo");
}

#[rstest]
fn css_keyframes_scoped() {
    assert_compiler("css_keyframes_scoped");
}

#[rstest]
fn css_keyframes_percentage_scopes_all() {
    assert_compiler("css_keyframes_percentage_scopes_all");
}

#[rstest]
fn bind_this_with_children_and_class_directive() {
    assert_compiler("bind_this_with_children_and_class_directive");
}

#[rstest]
fn head_position_with_body() {
    assert_compiler("head_position_with_body");
}

#[rstest]
fn special_elements_all() {
    assert_compiler("special_elements_all");
}

#[rstest]
fn empty() {
    assert_compiler("empty");
}

#[rstest]
fn simple() {
    assert_compiler("hello_state");
}

#[rstest]
fn single_text_node() {
    assert_compiler("single_text_node");
}

#[rstest]
fn single_element() {
    assert_compiler("single_element");
}

#[rstest]
fn single_interpolation() {
    assert_compiler("single_interpolation");
}

#[rstest]
fn text_entity_decoding() {
    assert_compiler("text_entity_decoding");
}

#[rstest]
fn text_entity_decoding_root() {
    assert_compiler("text_entity_decoding_root");
}

#[rstest]
fn title_entity_decoding() {
    assert_compiler("title_entity_decoding");
}

#[rstest]
fn single_if_block() {
    assert_compiler("single_if_block");
}

#[rstest]
fn single_if_else_block() {
    assert_compiler("single_if_else_block");
}

#[test]
fn if_call_condition() {
    assert_compiler("if_call_condition");
}

#[rstest]
fn if_block_empty_consequent() {
    assert_compiler("if_block_empty_consequent");
}

#[rstest]
fn if_block_empty_alternate() {
    assert_compiler("if_block_empty_alternate");
}

#[rstest]
fn element_attributes() {
    assert_compiler("element_attributes");
}

#[rstest]
fn element_autofocus() {
    assert_compiler("element_autofocus");
}

#[rstest]
fn textarea_child_value_dynamic() {
    assert_compiler("textarea_child_value_dynamic");
}

#[rstest]
fn option_expr_child_value() {
    assert_compiler("option_expr_child_value");
}

#[rstest]
fn option_expr_value() {
    assert_compiler("option_expr_value");
}

#[rstest]
fn option_concat_value() {
    assert_compiler("option_concat_value");
}

#[rstest]
fn option_expr_value_multi() {
    assert_compiler("option_expr_value_multi");
}

#[rstest]
fn bind_value_dev_named_fns() {
    assert_compiler("bind_value_dev_named_fns");
}

#[rstest]
fn bind_component_prop_dev_ownership() {
    assert_compiler("bind_component_prop_dev_ownership");
}

#[rstest]
fn bind_component_plain_prop_dev_ownership() {
    assert_compiler("bind_component_plain_prop_dev_ownership");
}

#[rstest]
fn bind_dynamic_component_dev_ownership() {
    assert_compiler("bind_dynamic_component_dev_ownership");
}

#[rstest]
fn bind_component_dev_ownership_ignore() {
    assert_compiler("bind_component_dev_ownership_ignore");
}

#[rstest]
fn bind_component_explicit_source() {
    assert_compiler("bind_component_explicit_source");
}

#[rstest]
fn customizable_select_option_el() {
    assert_compiler("customizable_select_option_el");
}

#[rstest]
fn customizable_select_select_div() {
    assert_compiler("customizable_select_select_div");
}

#[rstest]
fn selectedcontent_basic() {
    assert_compiler("selectedcontent_basic");
}

#[rstest]
fn state_runes() {
    assert_compiler("state_runes");
}

#[rstest]
fn state_raw() {
    assert_compiler("state_raw");
}

#[rstest]
fn state_eager_basic() {
    assert_compiler("state_eager_basic");
}

#[rstest]
fn state_eager_reactive() {
    assert_compiler("state_eager_reactive");
}

#[rstest]
fn state_eager_template() {
    assert_compiler("state_eager_template");
}

#[rstest]
fn state_snapshot_basic() {
    assert_compiler("state_snapshot_basic");
}

#[rstest]
fn state_snapshot_expression() {
    assert_compiler("state_snapshot_expression");
}

#[rstest]
fn state_snapshot_reactive() {
    assert_compiler("state_snapshot_reactive");
}

#[rstest]
fn each_block() {
    assert_compiler("each_block");
}

#[rstest]
fn each_inner_shadow() {
    assert_compiler("each_inner_shadow");
}

#[rstest]
fn bind_directives() {
    assert_compiler("bind_directives");
}

#[rstest]
fn nested_elements() {
    assert_compiler("nested_elements");
}

#[rstest]
fn nested_resets() {
    assert_compiler("nested_resets");
}

#[rstest]
fn single_concatenation() {
    assert_compiler("single_concatenation");
}

#[rstest]
fn elements_childs() {
    assert_compiler("elements_childs");
}

#[rstest]
fn generic_root_sequence() {
    assert_compiler("generic_root_sequence");
}

#[rstest]
fn spread_attribute() {
    assert_compiler("spread_attribute");
}

#[rstest]
fn spread_class_directive() {
    assert_compiler("spread_class_directive");
}

#[rstest]
fn spread_style_directive() {
    assert_compiler("spread_style_directive");
}

#[rstest]
fn utf8() {
    assert_compiler("utf8");
}

#[rstest]
fn smoke() {
    assert_compiler("smoke");
}

#[rstest]
fn class_directive() {
    assert_compiler("class_directive");
}

#[rstest]
fn class_concat() {
    assert_compiler("class_concat");
}

#[rstest]
fn class_concat_literal_fold() {
    assert_compiler("class_concat_literal_fold");
}

#[rstest]
fn attribute_concat_literal_fold() {
    assert_compiler("attribute_concat_literal_fold");
}

#[rstest]
fn component_prop_concat_literal_fold() {
    assert_compiler("component_prop_concat_literal_fold");
}

#[rstest]
fn rune_update() {
    assert_compiler("rune_update");
}

#[rstest]
fn assign_in_template() {
    assert_compiler("assign_in_template");
}

#[rstest]
fn only_script() {
    assert_compiler("only_script");
}

#[rstest]
fn hoist_imports() {
    assert_compiler("hoist_imports");
}

#[rstest]
fn bind_directives_extended() {
    assert_compiler("bind_directives_extended");
}

#[rstest]
fn mutated_state_rune() {
    assert_compiler("mutated_state_rune");
}

#[rstest]
fn static_interpolation() {
    assert_compiler("static_interpolation");
}

#[rstest]
fn props_basic() {
    assert_compiler("props_basic");
}

#[rstest]
fn props_rest() {
    assert_compiler("props_rest");
}

#[rstest]
fn props_renamed() {
    assert_compiler("props_renamed");
}

#[rstest]
fn props_renamed_bindable() {
    assert_compiler("props_renamed_bindable");
}

#[rstest]
fn props_bindable() {
    assert_compiler("props_bindable");
}

#[rstest]
fn props_lazy_default() {
    assert_compiler("props_lazy_default");
}

#[rstest]
fn props_mutated() {
    assert_compiler("props_mutated");
}

#[rstest]
fn props_member_mutation_computed() {
    assert_compiler("props_member_mutation_computed");
}

#[rstest]
fn props_renamed_member_update_computed() {
    assert_compiler("props_renamed_member_update_computed");
}

#[rstest]
fn props_mixed() {
    assert_compiler("props_mixed");
}

#[rstest]
fn exports() {
    assert_compiler("exports");
}

#[rstest]
fn snippet_basic() {
    assert_compiler("snippet_basic");
}

#[rstest]
fn component_basic() {
    assert_compiler("component_basic");
}

#[rstest]
fn svelte_component_basic() {
    assert_compiler("svelte_component_basic");
}

#[rstest]
fn svelte_component_children() {
    assert_compiler("svelte_component_children");
}

#[rstest]
fn component_non_self_closing() {
    assert_compiler("component_non_self_closing");
}

#[rstest]
fn component_in_element() {
    assert_compiler("component_in_element");
}

#[rstest]
fn component_mixed() {
    assert_compiler("component_mixed");
}

#[rstest]
fn component_props() {
    assert_compiler("component_props");
}

#[rstest]
fn component_children() {
    assert_compiler("component_children");
}

#[rstest]
fn component_events() {
    assert_compiler("component_events");
}

#[rstest]
fn component_events_dev_apply() {
    assert_compiler("component_events_dev_apply");
}

#[rstest]
fn component_element_children() {
    assert_compiler("component_element_children");
}

#[rstest]
fn component_named_slot() {
    assert_compiler("component_named_slot");
}

#[rstest]
fn component_default_slot_let() {
    assert_compiler("component_default_slot_let");
}

#[rstest]
fn component_default_slot_let_alias() {
    assert_compiler("component_default_slot_let_alias");
}

#[rstest]
fn component_named_slot_let_element() {
    assert_compiler("component_named_slot_let_element");
}

#[rstest]
fn component_named_slot_let_element_destructure() {
    assert_compiler("component_named_slot_let_element_destructure");
}

#[rstest]
fn component_named_slot_let_element_multiple() {
    assert_compiler("component_named_slot_let_element_multiple");
}

#[rstest]
fn component_child_slot_attribute() {
    assert_compiler("component_child_slot_attribute");
}

#[rstest]
fn smoke_all() {
    assert_compiler("smoke_all");
}

#[rstest]
fn derived_basic() {
    assert_compiler("derived_basic");
}

#[rstest]
fn derived_by() {
    assert_compiler("derived_by");
}

#[rstest]
fn derived_dynamic() {
    assert_compiler("derived_dynamic");
}

#[rstest]
fn unmutated_state_optimization() {
    assert_compiler("unmutated_state_optimization");
}

#[rstest]
fn effect_runes() {
    assert_compiler("effect_runes");
}

#[rstest]
fn effect_root_basic() {
    assert_compiler("effect_root_basic");
}

#[rstest]
fn effect_root_cleanup() {
    assert_compiler("effect_root_cleanup");
}

#[rstest]
fn effect_tracking() {
    assert_compiler("effect_tracking");
}

#[rstest]
fn effect_pending() {
    assert_compiler("effect_pending");
}

#[rstest]
fn effect_pending_script_init() {
    assert_compiler("effect_pending_script_init");
}

#[rstest]
fn effect_pending_script_derived() {
    assert_compiler("effect_pending_script_derived");
}

#[rstest]
fn host_basic() {
    assert_compiler("host_basic");
}

#[rstest]
fn host_props_rest() {
    assert_compiler("host_props_rest");
}

#[rstest]
fn custom_element_props() {
    assert_compiler("custom_element_props");
}

#[rstest]
fn custom_element_props_config() {
    assert_compiler("custom_element_props_config");
}

#[rstest]
fn custom_element_boolean_default() {
    assert_compiler("custom_element_boolean_default");
}

#[rstest]
fn custom_element_exports() {
    assert_compiler("custom_element_exports");
}

#[rstest]
fn custom_element_shadow_none() {
    assert_compiler("custom_element_shadow_none");
}

#[rstest]
fn custom_element_object_full() {
    assert_compiler("custom_element_object_full");
}

#[rstest]
fn custom_element_shadow_open() {
    assert_compiler("custom_element_shadow_open");
}

#[rstest]
fn custom_element_extend() {
    assert_compiler("custom_element_extend");
}

#[rstest]
fn custom_element_no_tag() {
    assert_compiler("custom_element_no_tag");
}

#[rstest]
fn custom_element_prop_alias() {
    assert_compiler("custom_element_prop_alias");
}

#[rstest]
fn custom_element_compile_option_default() {
    assert_compiler("custom_element_compile_option_default");
}

#[rstest]
fn custom_element_dev_exports_legacy_api() {
    assert_compiler("custom_element_dev_exports_legacy_api");
}

#[rstest]
fn custom_element_slots() {
    assert_compiler("custom_element_slots");
}

#[rstest]
fn legacy_props_basic() {
    assert_compiler("legacy_props_basic");
}

#[rstest]
fn legacy_reactivity_let_basic() {
    assert_compiler("legacy_reactivity_let_basic");
}

#[rstest]
fn legacy_reactivity_var_basic() {
    assert_compiler("legacy_reactivity_var_basic");
}

#[rstest]
fn legacy_reactivity_member_mutation() {
    assert_compiler("legacy_reactivity_member_mutation");
}

#[rstest]
fn legacy_reactivity_array_self_assign() {
    assert_compiler("legacy_reactivity_array_self_assign");
}

#[rstest]
fn legacy_reactivity_destructure() {
    assert_compiler("legacy_reactivity_destructure");
}

#[rstest]
fn legacy_reactive_assignment_basic() {
    assert_compiler("legacy_reactive_assignment_basic");
}

#[rstest]
fn legacy_reactive_assignment_declared_dependency() {
    assert_compiler("legacy_reactive_assignment_declared_dependency");
}

#[rstest]
fn legacy_reactive_assignment_block_destructure() {
    assert_compiler("legacy_reactive_assignment_block_destructure");
}

#[rstest]
fn legacy_reactive_assignment_coarse_deps() {
    assert_compiler("legacy_reactive_assignment_coarse_deps");
}

#[rstest]
fn legacy_reactive_assignment_import_topology() {
    assert_compiler("legacy_reactive_assignment_import_topology");
}

#[rstest]
fn legacy_rest_props_basic() {
    assert_compiler("legacy_rest_props_basic");
}

#[rstest]
fn legacy_slots_if() {
    assert_compiler("legacy_slots_if");
}

#[rstest]
fn legacy_slots_script_basic() {
    assert_compiler("legacy_slots_script_basic");
}

#[rstest]
fn legacy_before_after_update_basic() {
    assert_compiler("legacy_before_after_update_basic");
}

#[rstest]
fn legacy_before_after_update_alias() {
    assert_compiler("legacy_before_after_update_alias");
}

#[rstest]
fn custom_element_css_default_injected() {
    assert_compiler("custom_element_css_default_injected");
}

#[rstest]
fn custom_element_shadow_object() {
    assert_compiler("custom_element_shadow_object");
}

#[rstest]
fn html_tag() {
    assert_compiler("html_tag");
}

#[rstest]
fn html_tag_mathml() {
    assert_compiler("html_tag_mathml");
}

#[rstest]
fn svg_foreignobject_fragment_html() {
    assert_compiler("svg_foreignobject_fragment_html");
}

#[rstest]
fn mathml_root_html_fragment() {
    assert_compiler("mathml_root_html_fragment");
}

#[rstest]
fn mathml_annotation_xml_fragment_html() {
    assert_compiler("mathml_annotation_xml_fragment_html");
}

#[rstest]
fn key_block() {
    assert_compiler("key_block");
}

#[rstest]
fn key_block_nested() {
    assert_compiler("key_block_nested");
}

#[rstest]
fn style_directive() {
    assert_compiler("style_directive");
}

#[rstest]
fn css_custom_prop_component() {
    assert_compiler("css_custom_prop_component");
}

#[rstest]
fn css_custom_prop_component_svg() {
    assert_compiler("css_custom_prop_component_svg");
}

#[rstest]
fn style_directive_important() {
    assert_compiler("style_directive_important");
}

#[rstest]
fn style_directive_string() {
    assert_compiler("style_directive_string");
}

#[rstest]
fn style_directive_concat() {
    assert_compiler("style_directive_concat");
}

#[rstest]
fn on_directive() {
    assert_compiler("on_directive");
}

#[rstest]
fn on_directive_modifiers() {
    assert_compiler("on_directive_modifiers");
}

#[rstest]
fn on_directive_nonpassive() {
    assert_compiler("on_directive_nonpassive");
}

#[rstest]
fn on_directive_dev_apply() {
    assert_compiler("on_directive_dev_apply");
}

#[rstest]
fn use_action_basic() {
    assert_compiler("use_action_basic");
}

#[rstest]
fn use_action_expression() {
    assert_compiler("use_action_expression");
}

#[rstest]
fn use_action_reactive() {
    assert_compiler("use_action_reactive");
}

#[rstest]
fn use_action_dotted() {
    assert_compiler("use_action_dotted");
}

#[rstest]
fn use_action_dotted_hyphen() {
    assert_compiler("use_action_dotted_hyphen");
}

#[rstest]
fn use_action_multiple() {
    assert_compiler("use_action_multiple");
}

#[rstest]
fn use_action_in_if() {
    assert_compiler("use_action_in_if");
}

#[rstest]
fn use_action_in_each() {
    assert_compiler("use_action_in_each");
}

#[rstest]
fn void_elements() {
    assert_compiler("void_elements");
}

#[rstest]
fn non_void_self_closing() {
    assert_compiler("non_void_self_closing");
}

#[rstest]
fn mixed_html_elements() {
    assert_compiler("mixed_html_elements");
}

#[rstest]
fn store_basic() {
    assert_compiler("store_basic");
}

#[rstest]
fn store_write() {
    assert_compiler("store_write");
}

#[rstest]
fn store_validate_dev() {
    assert_compiler("store_validate_dev");
}

#[rstest]
fn store_reassign_unsub() {
    assert_compiler("store_reassign_unsub");
}

#[rstest]
fn store_each_invalidate() {
    assert_compiler("store_each_invalidate");
}

#[rstest]
fn store_mark_binding() {
    assert_compiler("store_mark_binding");
}

#[rstest]
fn const_tag() {
    assert_compiler("const_tag");
}

#[rstest]
fn const_tag_destructured() {
    assert_compiler("const_tag_destructured");
}

#[rstest]
fn const_tag_destructured_multi() {
    assert_compiler("const_tag_destructured_multi");
}

#[rstest]
fn const_tag_destructured_if() {
    assert_compiler("const_tag_destructured_if");
}

#[rstest]
fn const_tag_key_block() {
    assert_compiler("const_tag_key_block");
}

#[rstest]
fn const_tag_await() {
    assert_compiler("const_tag_await");
}

#[rstest]
fn const_tag_component() {
    assert_compiler("const_tag_component");
}

#[rstest]
fn class_array() {
    assert_compiler("class_array");
}

#[rstest]
fn class_object() {
    assert_compiler("class_object");
}

#[rstest]
fn class_variable() {
    assert_compiler("class_variable");
}

#[rstest]
fn class_expr_with_directives() {
    assert_compiler("class_expr_with_directives");
}

#[rstest]
fn bind_select_value() {
    assert_compiler("bind_select_value");
}

#[rstest]
fn bind_files() {
    assert_compiler("bind_files");
}

#[rstest]
fn bind_property() {
    assert_compiler("bind_property");
}

#[rstest]
fn bind_content_editable() {
    assert_compiler("bind_content_editable");
}

#[rstest]
fn bind_element_size() {
    assert_compiler("bind_element_size");
}

#[rstest]
fn bind_resize_observer() {
    assert_compiler("bind_resize_observer");
}

#[rstest]
fn bind_resize_observer_border_box_size() {
    assert_compiler("bind_resize_observer_border_box_size");
}

#[rstest]
fn bind_resize_observer_device_pixel_content_box_size() {
    assert_compiler("bind_resize_observer_device_pixel_content_box_size");
}

#[rstest]
fn bind_textarea_value() {
    assert_compiler("bind_textarea_value");
}

#[rstest]
fn bind_media_rw() {
    assert_compiler("bind_media_rw");
}

#[rstest]
fn bind_media_ro() {
    assert_compiler("bind_media_ro");
}

#[rstest]
fn bind_media_property() {
    assert_compiler("bind_media_property");
}

#[rstest]
fn bind_img() {
    assert_compiler("bind_img");
}

#[rstest]
fn bind_this() {
    assert_compiler("bind_this");
}

#[rstest]
fn component_bind_this() {
    assert_compiler("component_bind_this");
}

#[rstest]
fn component_bind_this_variants() {
    assert_compiler("component_bind_this_variants");
}

#[rstest]
fn svelte_self_if() {
    assert_compiler("svelte_self_if");
}

#[rstest]
fn svelte_self_each() {
    assert_compiler("svelte_self_each");
}

#[rstest]
fn svelte_self_snippet() {
    assert_compiler("svelte_self_snippet");
}

#[rstest]
fn svelte_self_slot() {
    assert_compiler("svelte_self_slot");
}

#[rstest]
fn svelte_self_props() {
    assert_compiler("svelte_self_props");
}

#[rstest]
fn svelte_self_bind_this() {
    assert_compiler("svelte_self_bind_this");
}

#[rstest]
fn bind_focused() {
    assert_compiler("bind_focused");
}

// ---------------------------------------------------------------------------
// Transition tests
// ---------------------------------------------------------------------------

#[rstest]
fn transition_basic() {
    assert_compiler("transition_basic");
}

#[rstest]
fn transition_params() {
    assert_compiler("transition_params");
}

#[rstest]
fn transition_in() {
    assert_compiler("transition_in");
}

#[rstest]
fn transition_out() {
    assert_compiler("transition_out");
}

#[rstest]
fn transition_in_out_separate() {
    assert_compiler("transition_in_out_separate");
}

#[rstest]
fn transition_local() {
    assert_compiler("transition_local");
}

#[rstest]
fn transition_global() {
    assert_compiler("transition_global");
}

#[rstest]
fn transition_dotted_name() {
    assert_compiler("transition_dotted_name");
}

#[rstest]
fn transition_in_if() {
    assert_compiler("transition_in_if");
}

#[rstest]
fn transition_reactive_params() {
    assert_compiler("transition_reactive_params");
}

#[rstest]
fn transition_elseif_local() {
    assert_compiler("transition_elseif_local");
}

// ---------------------------------------------------------------------------
// Animate directive tests
// ---------------------------------------------------------------------------

#[rstest]
fn animate_basic() {
    assert_compiler("animate_basic");
}

#[rstest]
fn animate_params() {
    assert_compiler("animate_params");
}

#[rstest]
fn animate_dotted_name() {
    assert_compiler("animate_dotted_name");
}

#[rstest]
fn animate_reactive_params() {
    assert_compiler("animate_reactive_params");
}

#[rstest]
fn animate_svelte_element() {
    assert_compiler("animate_svelte_element");
}

#[rstest]
fn animate_with_const_tag() {
    assert_compiler("animate_with_const_tag");
}

// ---------------------------------------------------------------------------
// Attach tag tests
// ---------------------------------------------------------------------------

#[rstest]
fn attach_basic() {
    assert_compiler("attach_basic");
}

#[rstest]
fn attach_inline_arrow() {
    assert_compiler("attach_inline_arrow");
}

#[rstest]
fn attach_conditional() {
    assert_compiler("attach_conditional");
}

#[rstest]
fn attach_multiple() {
    assert_compiler("attach_multiple");
}

#[rstest]
fn attach_with_directives() {
    assert_compiler("attach_with_directives");
}

#[rstest]
fn attach_in_if() {
    assert_compiler("attach_in_if");
}

#[rstest]
fn attach_in_each() {
    assert_compiler("attach_in_each");
}

// ---------------------------------------------------------------------------
// $state/$state.raw destructuring
// ---------------------------------------------------------------------------

#[rstest]
fn state_destructure() {
    assert_compiler("state_destructure");
}

#[rstest]
fn state_raw_destructure_object() {
    assert_compiler("state_raw_destructure_object");
}

#[rstest]
fn state_raw_destructure_array() {
    assert_compiler("state_raw_destructure_array");
}

// ---------------------------------------------------------------------------
// $state/$state.raw class fields
// ---------------------------------------------------------------------------

#[rstest]
fn state_class_field() {
    assert_compiler("state_class_field");
}

#[rstest]
fn state_raw_class_field() {
    assert_compiler("state_raw_class_field");
}

#[rstest]
fn state_private_class_field() {
    assert_compiler("state_private_class_field");
}

#[rstest]
fn state_class_constructor() {
    assert_compiler("state_class_constructor");
}

#[rstest]
fn state_class_multiple() {
    assert_compiler("state_class_multiple");
}

#[rstest]
fn state_constructor_private_read() {
    assert_compiler("state_constructor_private_read");
}

#[rstest]
fn state_constructor_read_v() {
    assert_compiler("state_constructor_read_v");
}

#[rstest]
fn state_constructor_read_derived() {
    assert_compiler("state_constructor_read_derived");
}

#[rstest]
fn state_class_raw_field() {
    assert_compiler("state_class_raw_field");
}

#[rstest]
fn state_no_init() {
    assert_compiler("state_no_init");
}

#[rstest]
fn state_snapshot_in_template() {
    assert_compiler("state_snapshot_in_template");
}

#[rstest]
fn state_snapshot_ignored() {
    assert_compiler("state_snapshot_ignored");
}

#[rstest]
fn state_snapshot_not_ignored() {
    assert_compiler("state_snapshot_not_ignored");
}

#[rstest]
fn state_snapshot_ignored_return() {
    assert_compiler("state_snapshot_ignored_return");
}

#[rstest]
fn for_await_ignored() {
    assert_compiler("for_await_ignored");
}

#[rstest]
fn await_reactivity_ignored() {
    assert_compiler("await_reactivity_ignored");
}

#[rstest]
fn state_class_constructor_proxy() {
    assert_compiler("state_class_constructor_proxy");
}

#[rstest]
fn derived_class_field() {
    assert_compiler("derived_class_field");
}

#[rstest]
fn derived_by_class_fields() {
    assert_compiler("derived_by_class_fields");
}

#[rstest]
fn derived_by_class_constructor_only() {
    assert_compiler("derived_by_class_constructor_only");
}

#[rstest]
fn derived_by_class_placeholder_preserves_plain_fields() {
    assert_compiler("derived_by_class_placeholder_preserves_plain_fields");
}

#[rstest]
fn state_class_field_constructor_assign() {
    assert_compiler("state_class_field_constructor_assign");
}

#[rstest]
fn svg_inner_whitespace_trimming() {
    assert_compiler("svg_inner_whitespace_trimming");
}

#[rstest]
fn svg_inner_template_from_svg() {
    assert_compiler("svg_inner_template_from_svg");
}

#[rstest]
fn template_effect_call_deps() {
    assert_compiler("template_effect_call_deps");
}

#[rstest]
fn svg_text_preserves_whitespace() {
    assert_compiler("svg_text_preserves_whitespace");
}

#[rstest]
fn template_effect_multiple_call_deps() {
    assert_compiler("template_effect_multiple_call_deps");
}

#[rstest]
fn component_local_underscored_bind_this() {
    assert_compiler("component_local_underscored_bind_this");
}

#[rstest]
fn component_dynamic_dotted_identifier_root() {
    assert_compiler("component_dynamic_dotted_identifier_root");
}

#[rstest]
fn component_dynamic_props_access() {
    assert_compiler("component_dynamic_props_access");
}

#[rstest]
fn component_dynamic_dotted_props_root() {
    assert_compiler("component_dynamic_dotted_props_root");
}

// ---------------------------------------------------------------------------
// Module compilation tests
// ---------------------------------------------------------------------------

fn assert_compiler_module(case: &str) {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("cases2")
        .join(case)
        .join("case.svelte.js");
    let input = read_to_string(&path).expect("test invariant");

    let dir = path.parent().expect("test invariant");
    let config_path = dir.join("config.json");
    let mut opts = ModuleCompileOptions::default();
    if config_path.exists() {
        let config: serde_json::Value =
            serde_json::from_str(&read_to_string(&config_path).expect("test invariant"))
                .expect("test invariant");
        if let Some(dev) = config.get("dev").and_then(|v| v.as_bool()) {
            opts.dev = dev;
        }
        if let Some(filename) = config.get("filename").and_then(|v| v.as_str()) {
            opts.filename = filename.to_string();
        }
    }

    let result = compile_module(&input, &opts);
    let js = result
        .js
        .unwrap_or_else(|| panic!("[{case}] compile_module produced no JS"));

    let expected = read_to_string(dir.join("case-svelte.js")).expect("test invariant");

    File::create(dir.join("case-rust.js"))
        .expect("test invariant")
        .write_all(js.as_bytes())
        .expect("test invariant");
    assert_eq!(js, expected);
}

#[rstest]
fn module_compilation() {
    assert_compiler_module("module_compilation");
}

#[rstest]
fn module_dev_state_tag() {
    assert_compiler_module("module_dev_state_tag");
}

#[rstest]
fn module_dev_derived_tag() {
    assert_compiler_module("module_dev_derived_tag");
}

#[rstest]
fn module_dev_console_log_wrap() {
    assert_compiler_module("module_dev_console_log_wrap");
}

#[rstest]
fn script_module_exports() {
    assert_compiler("script_module_exports");
}

#[rstest]
fn script_module_export_specifiers() {
    assert_compiler("script_module_export_specifiers");
}

#[rstest]
fn script_module_imports() {
    assert_compiler("script_module_imports");
}

#[rstest]
fn script_module_empty() {
    assert_compiler("script_module_empty");
}

#[rstest]
fn script_module_runes() {
    assert_compiler("script_module_runes");
}

#[rstest]
fn script_module_instance_ref() {
    assert_compiler("script_module_instance_ref");
}

#[rstest]
fn script_module_only() {
    assert_compiler("script_module_only");
}

#[rstest]
fn script_module_with_instance() {
    assert_compiler("script_module_with_instance");
}

#[rstest]
fn svelte_options_basic() {
    assert_compiler("svelte_options_basic");
}

#[rstest]
fn svelte_options_runes_false_override() {
    assert_compiler("svelte_options_runes_false_override");
}

#[rstest]
fn svelte_options_accessors_legacy() {
    assert_compiler("svelte_options_accessors_legacy");
}

#[rstest]
fn svelte_options_immutable_legacy() {
    assert_compiler("svelte_options_immutable_legacy");
}

#[rstest]
fn legacy_export_let_required() {
    assert_compiler("legacy_export_let_required");
}

#[rstest]
fn legacy_export_var_basic() {
    assert_compiler("legacy_export_var_basic");
}

#[rstest]
fn legacy_export_specifier() {
    assert_compiler("legacy_export_specifier");
}

#[rstest]
fn legacy_export_specifier_alias() {
    assert_compiler("legacy_export_specifier_alias");
}

#[rstest]
fn legacy_export_destructure() {
    assert_compiler("legacy_export_destructure");
}

#[rstest]
fn legacy_export_let_typed() {
    assert_compiler("legacy_export_let_typed");
}

#[rstest]
fn legacy_export_let_member_mutation() {
    assert_compiler("legacy_export_let_member_mutation");
}

#[rstest]
fn legacy_export_let_bind_to_inner() {
    assert_compiler("legacy_export_let_bind_to_inner");
}

#[rstest]
fn legacy_export_let_compound_assign_prop() {
    assert_compiler("legacy_export_let_compound_assign_prop");
}

#[rstest]
fn legacy_export_let_update_prop_in_template() {
    assert_compiler("legacy_export_let_update_prop_in_template");
}

#[rstest]
fn legacy_export_let_assign_prop_in_template() {
    assert_compiler("legacy_export_let_assign_prop_in_template");
}

#[rstest]
fn legacy_state_member_update_in_template() {
    assert_compiler("legacy_state_member_update_in_template");
}

#[rstest]
fn legacy_state_member_compound_in_template() {
    assert_compiler("legacy_state_member_compound_in_template");
}

#[rstest]
fn legacy_export_let_member_update_in_template() {
    assert_compiler("legacy_export_let_member_update_in_template");
}

#[rstest]
fn runes_prop_member_update_in_template() {
    assert_compiler("runes_prop_member_update_in_template");
}

#[rstest]
fn runes_prop_member_compound_in_template() {
    assert_compiler("runes_prop_member_compound_in_template");
}

#[rstest]
fn smoke_legacy_reactive_mutations_all() {
    assert_compiler("smoke_legacy_reactive_mutations_all");
}

#[rstest]
fn smoke_runes_reactive_mutations_all() {
    assert_compiler("smoke_runes_reactive_mutations_all");
}

#[rstest]
fn smoke_legacy_contextual_mutations_all() {
    assert_compiler("smoke_legacy_contextual_mutations_all");
}

#[rstest]
fn smoke_runes_declarator_gaps_all() {
    assert_compiler("smoke_runes_declarator_gaps_all");
}

#[rstest]
fn smoke_runes_state_eager_panic() {
    assert_compiler("smoke_runes_state_eager_panic");
}

#[rstest]
fn smoke_ts_non_null_assertion_mutations() {
    assert_compiler("smoke_ts_non_null_assertion_mutations");
}

#[rstest]
fn text_expression_binary_no_nullish_fallback() {
    assert_compiler("text_expression_binary_no_nullish_fallback");
}

#[rstest]
fn svelte_options_preserve_whitespace() {
    assert_compiler("svelte_options_preserve_whitespace");
}

// ---------------------------------------------------------------------------
// svelte:head tests
// ---------------------------------------------------------------------------

#[rstest]
fn svelte_head_basic() {
    assert_compiler("svelte_head_basic");
}

#[rstest]
fn svelte_head_reactive() {
    assert_compiler("svelte_head_reactive");
}

#[rstest]
fn svelte_head_with_content() {
    assert_compiler("svelte_head_with_content");
}

// <title> in <svelte:head> tests
#[rstest]
fn title_variants() {
    assert_compiler("title_variants");
}

#[rstest]
fn async_title_basic() {
    assert_compiler("async_title_basic");
}

// svelte:window tests
#[rstest]
fn svelte_window_event_legacy() {
    assert_compiler("svelte_window_event_legacy");
}

#[rstest]
fn svelte_window_event_attr() {
    assert_compiler("svelte_window_event_attr");
}

#[rstest]
fn svelte_window_bind_scroll() {
    assert_compiler("svelte_window_bind_scroll");
}

#[rstest]
fn svelte_window_bind_size() {
    assert_compiler("svelte_window_bind_size");
}

#[rstest]
fn svelte_window_bind_online() {
    assert_compiler("svelte_window_bind_online");
}

#[rstest]
fn svelte_window_combined() {
    assert_compiler("svelte_window_combined");
}

#[rstest]
fn svelte_window_reactive() {
    assert_compiler("svelte_window_reactive");
}

#[rstest]
fn svelte_document_bindings() {
    assert_compiler("svelte_document_bindings");
}

#[rstest]
fn svelte_document_events() {
    assert_compiler("svelte_document_events");
}

#[rstest]
fn svelte_document_bubble() {
    assert_compiler("svelte_document_bubble");
}

#[rstest]
fn svelte_document_combined() {
    assert_compiler("svelte_document_combined");
}

#[rstest]
fn svelte_element_basic() {
    assert_compiler("svelte_element_basic");
}

#[rstest]
fn svelte_element_self_closing() {
    assert_compiler("svelte_element_self_closing");
}

#[rstest]
fn svelte_fragment_named_slot() {
    assert_compiler("svelte_fragment_named_slot");
}

#[rstest]
fn component_named_slot_let_fragment() {
    assert_compiler("component_named_slot_let_fragment");
}

#[rstest]
fn component_named_slot_let_fragment_destructure() {
    assert_compiler("component_named_slot_let_fragment_destructure");
}

#[rstest]
fn svelte_element_static_tag() {
    assert_compiler("svelte_element_static_tag");
}

#[rstest]
fn svelte_element_attributes() {
    assert_compiler("svelte_element_attributes");
}

#[rstest]
fn svelte_element_spread() {
    assert_compiler("svelte_element_spread");
}

#[rstest]
fn svelte_element_onclick() {
    assert_compiler("svelte_element_onclick");
}

#[rstest]
fn svelte_element_bind() {
    assert_compiler("svelte_element_bind");
}

#[rstest]
fn svelte_element_null_tag() {
    assert_compiler("svelte_element_null_tag");
}

#[rstest]
fn svelte_element_xmlns() {
    assert_compiler("svelte_element_xmlns");
}

#[rstest]
fn svelte_element_dynamic_xmlns() {
    assert_compiler("svelte_element_dynamic_xmlns");
}

#[rstest]
fn svelte_element_children_expr() {
    assert_compiler("svelte_element_children_expr");
}

#[rstest]
fn svelte_body_event_attr() {
    assert_compiler("svelte_body_event_attr");
}

#[rstest]
fn svelte_body_event_legacy() {
    assert_compiler("svelte_body_event_legacy");
}

#[rstest]
fn svelte_body_action() {
    assert_compiler("svelte_body_action");
}

#[rstest]
fn svelte_body_combined() {
    assert_compiler("svelte_body_combined");
}

#[rstest]
fn boundary_basic() {
    assert_compiler("boundary_basic");
}

#[rstest]
fn boundary_failed_snippet() {
    assert_compiler("boundary_failed_snippet");
}

#[rstest]
fn boundary_onerror() {
    assert_compiler("boundary_onerror");
}

#[rstest]
fn boundary_pending_snippet() {
    assert_compiler("boundary_pending_snippet");
}

#[rstest]
fn boundary_failed_onerror() {
    assert_compiler("boundary_failed_onerror");
}

#[rstest]
fn boundary_failed_attribute() {
    assert_compiler("boundary_failed_attribute");
}

#[rstest]
fn boundary_all_three() {
    assert_compiler("boundary_all_three");
}

#[rstest]
fn boundary_reactive_onerror() {
    assert_compiler("boundary_reactive_onerror");
}

#[rstest]
fn boundary_nested() {
    assert_compiler("boundary_nested");
}

#[rstest]
fn boundary_const_tag() {
    assert_compiler("boundary_const_tag");
}

#[rstest]
fn boundary_in_if() {
    assert_compiler("boundary_in_if");
}

#[rstest]
fn boundary_other_snippets() {
    assert_compiler("boundary_other_snippets");
}

#[rstest]
fn boundary_pending_attribute() {
    assert_compiler("boundary_pending_attribute");
}

#[rstest]
fn boundary_pending_imported() {
    assert_compiler("boundary_pending_imported");
}

#[rstest]
fn boundary_failed_attribute_override() {
    assert_compiler("boundary_failed_attribute_override");
}

#[rstest]
fn boundary_pending_attribute_override() {
    assert_compiler("boundary_pending_attribute_override");
}

#[rstest]
fn await_basic() {
    assert_compiler("await_basic");
}

#[rstest]
fn await_short_then() {
    assert_compiler("await_short_then");
}

#[rstest]
fn await_short_catch() {
    assert_compiler("await_short_catch");
}

#[rstest]
fn await_then_catch() {
    assert_compiler("await_then_catch");
}

#[rstest]
fn await_no_bindings() {
    assert_compiler("await_no_bindings");
}

#[rstest]
fn await_pending_only() {
    assert_compiler("await_pending_only");
}

#[rstest]
fn await_destructured() {
    assert_compiler("await_destructured");
}

#[rstest]
fn await_in_if() {
    assert_compiler("await_in_if");
}

#[rstest]
fn await_in_each() {
    assert_compiler("await_in_each");
}

#[rstest]
fn await_reactive() {
    assert_compiler("await_reactive");
}

#[rstest]
fn await_nested_content() {
    assert_compiler("await_nested_content");
}

// ---------------------------------------------------------------------------
// Event attribute tests (Svelte 5)
// ---------------------------------------------------------------------------

#[rstest]
fn event_attr_non_delegatable() {
    assert_compiler("event_attr_non_delegatable");
}

#[rstest]
fn event_attr_capture() {
    assert_compiler("event_attr_capture");
}

#[rstest]
fn event_attr_capture_non_deleg() {
    assert_compiler("event_attr_capture_non_deleg");
}

#[rstest]
fn event_attr_gotpointercapture() {
    assert_compiler("event_attr_gotpointercapture");
}

#[rstest]
fn event_attr_passive() {
    assert_compiler("event_attr_passive");
}

#[rstest]
fn event_attr_passive_window() {
    assert_compiler("event_attr_passive_window");
}

#[rstest]
fn event_attr_import_handler() {
    assert_compiler("event_attr_import_handler");
}

#[rstest]
fn event_attr_member_handler() {
    assert_compiler("event_attr_member_handler");
}

#[rstest]
fn event_attr_has_call() {
    assert_compiler("event_attr_has_call");
}

#[rstest]
fn event_attr_dev_apply() {
    assert_compiler("event_attr_dev_apply");
}

// ---------------------------------------------------------------------------
// Expression memoization tests
// ---------------------------------------------------------------------------

#[rstest]
fn component_prop_has_call() {
    assert_compiler("component_prop_has_call");
}

#[rstest]
fn component_prop_has_call_multi() {
    assert_compiler("component_prop_has_call_multi");
}

#[rstest]
fn component_prop_has_call_mixed() {
    assert_compiler("component_prop_has_call_mixed");
}

#[rstest]
fn component_dynamic_dotted() {
    assert_compiler("component_dynamic_dotted");
}

#[rstest]
fn component_prop_memo_state() {
    assert_compiler("component_prop_memo_state");
}

#[rstest]
fn render_tag_arg_has_call() {
    assert_compiler("render_tag_arg_has_call");
}

#[rstest]
fn render_tag_arg_has_call_multi() {
    assert_compiler("render_tag_arg_has_call_multi");
}

#[rstest]
fn render_tag_arg_mixed() {
    assert_compiler("render_tag_arg_mixed");
}

#[rstest]
fn render_tag_dynamic_prop() {
    assert_compiler("render_tag_dynamic_prop");
}

#[rstest]
fn render_tag_dynamic_state() {
    assert_compiler("render_tag_dynamic_state");
}

#[rstest]
fn render_tag_dynamic_snippet_param() {
    assert_compiler("render_tag_dynamic_snippet_param");
}

#[rstest]
fn render_tag_optional() {
    assert_compiler("render_tag_optional");
}

#[rstest]
fn render_tag_optional_dynamic() {
    assert_compiler("render_tag_optional_dynamic");
}

// ---------------------------------------------------------------------------
// $inspect rune tests
// ---------------------------------------------------------------------------

#[rstest]
fn inspect_basic() {
    assert_compiler("inspect_basic");
}

#[rstest]
fn inspect_with_callback() {
    assert_compiler("inspect_with_callback");
}

#[rstest]
fn inspect_prod_strip() {
    assert_compiler("inspect_prod_strip");
}

// ---------------------------------------------------------------------------
// $inspect.trace() rune tests
// ---------------------------------------------------------------------------

#[rstest]
fn inspect_trace_basic() {
    assert_compiler("inspect_trace_basic");
}

#[rstest]
fn inspect_trace_contexts() {
    assert_compiler("inspect_trace_contexts");
}

#[rstest]
fn inspect_trace_prod_strip() {
    assert_compiler("inspect_trace_prod_strip");
}

#[rstest]
fn inspect_trace_reactive_contexts() {
    assert_compiler("inspect_trace_reactive_contexts");
}

// ---------------------------------------------------------------------------
// $props.id() rune tests
// ---------------------------------------------------------------------------

#[rstest]
fn props_id_basic() {
    assert_compiler("props_id_basic");
}

#[rstest]
fn props_id_with_props() {
    assert_compiler("props_id_with_props");
}

// ---------------------------------------------------------------------------
// {@debug} tests
// ---------------------------------------------------------------------------

#[rstest]
fn debug_basic() {
    assert_compiler("debug_basic");
}

#[rstest]
fn debug_in_blocks() {
    assert_compiler("debug_in_blocks");
}

// ---------------------------------------------------------------------------
// TypeScript stripping tests
// ---------------------------------------------------------------------------

#[rstest]
fn ts_strip_expression_tag() {
    assert_compiler("ts_strip_expression_tag");
}

#[rstest]
fn ts_strip_satisfies() {
    assert_compiler("ts_strip_satisfies");
}

#[rstest]
fn ts_strip_non_null() {
    assert_compiler("ts_strip_non_null");
}

#[rstest]
fn ts_strip_const_tag() {
    assert_compiler("ts_strip_const_tag");
}

#[rstest]
fn ts_strip_attribute() {
    assert_compiler("ts_strip_attribute");
}

#[rstest]
fn ts_strip_script_types() {
    assert_compiler("ts_strip_script_types");
}

#[rstest]
fn namespace_svg() {
    assert_compiler("namespace_svg");
}

#[rstest]
fn namespace_mathml() {
    assert_compiler("namespace_mathml");
}

#[rstest]
fn svg_fragment_ambiguous_a() {
    assert_compiler("svg_fragment_ambiguous_a");
}

#[rstest]
fn svg_fragment_ambiguous_title() {
    assert_compiler("svg_fragment_ambiguous_title");
}

#[rstest]
fn svelte_element_in_if() {
    assert_compiler("svelte_element_in_if");
}

#[rstest]
fn svelte_element_class_directive() {
    assert_compiler("svelte_element_class_directive");
}

#[rstest]
fn svelte_element_style_directive() {
    assert_compiler("svelte_element_style_directive");
}

#[rstest]
fn svelte_element_dev_invalid_tag() {
    assert_compiler("svelte_element_dev_invalid_tag");
}

#[rstest]
fn svelte_element_dev_void_children() {
    assert_compiler("svelte_element_dev_void_children");
}

#[rstest]
fn boundary_const_in_snippet() {
    assert_compiler("boundary_const_in_snippet");
}

#[rstest]
fn boundary_imported_handler() {
    assert_compiler("boundary_imported_handler");
}

#[rstest]
fn bind_this_sequence() {
    assert_compiler("bind_this_sequence");
}

// ---------------------------------------------------------------------------
// Tier 2b — Template Tags
// ---------------------------------------------------------------------------

#[rstest]
fn await_array_destructured() {
    assert_compiler("await_array_destructured");
}

#[rstest]
fn html_tag_controlled() {
    assert_compiler("html_tag_controlled");
}

#[rstest]
fn html_tag_svg() {
    assert_compiler("html_tag_svg");
}

#[rstest]
fn html_tag_nested_svg() {
    assert_compiler("html_tag_nested_svg");
}

#[rstest]
fn html_tag_nested_mathml() {
    assert_compiler("html_tag_nested_mathml");
}

#[rstest]
fn html_tag_hydration_ignore() {
    assert_compiler("html_tag_hydration_ignore");
}

#[rstest]
fn const_tag_dev() {
    assert_compiler("const_tag_dev");
}

#[rstest]
fn rune_compound_template() {
    assert_compiler("rune_compound_template");
}

#[rstest]
fn store_assign_template() {
    assert_compiler("store_assign_template");
}

#[rstest]
fn store_compound_template() {
    assert_compiler("store_compound_template");
}

#[rstest]
fn store_update_template() {
    assert_compiler("store_update_template");
}

#[rstest]
fn store_deep_mutation() {
    assert_compiler("store_deep_mutation");
}

#[rstest]
fn store_deep_update() {
    assert_compiler("store_deep_update");
}

// ---------------------------------------------------------------------------
// Tier 2c — Bind Directive Edge Cases
// ---------------------------------------------------------------------------

#[rstest]
fn bind_function_value() {
    assert_compiler("bind_function_value");
}

#[rstest]
fn bind_function_checked() {
    assert_compiler("bind_function_checked");
}

#[rstest]
fn bind_use_deferral() {
    assert_compiler("bind_use_deferral");
}

#[rstest]
fn bind_contenteditable_flag() {
    assert_compiler("bind_contenteditable_flag");
}

#[rstest]
fn bind_group_each() {
    assert_compiler("bind_group_each");
}

#[rstest]
fn bind_group_keyed_each() {
    assert_compiler("bind_group_keyed_each");
}

#[rstest]
fn bind_group_nested_each() {
    assert_compiler("bind_group_nested_each");
}

#[rstest]
fn bind_group_value_attr() {
    assert_compiler("bind_group_value_attr");
}

#[rstest]
fn bind_group_each_var() {
    assert_compiler("bind_group_each_var");
}

#[rstest]
fn bind_group_each_var_keyed() {
    assert_compiler("bind_group_each_var_keyed");
}

#[rstest]
fn each_fallback() {
    assert_compiler("each_fallback");
}

#[rstest]
fn each_keyed_index() {
    assert_compiler("each_keyed_index");
}

#[rstest]
fn each_key_is_index_literal_diagnose() {
    assert_compiler("each_key_is_index_literal_diagnose");
}

#[rstest]
fn each_key_uses_index() {
    assert_compiler("each_key_uses_index");
}

#[rstest]
fn each_key_is_item() {
    assert_compiler("each_key_is_item");
}

#[rstest]
fn each_destructured_obj() {
    assert_compiler("each_destructured_obj");
}

#[rstest]
fn each_destructured_default() {
    assert_compiler("each_destructured_default");
}

#[rstest]
fn each_destructured_array() {
    assert_compiler("each_destructured_array");
}

#[rstest]
fn style_attr_object() {
    assert_compiler("style_attr_object");
}

#[rstest]
fn style_attr_dynamic() {
    assert_compiler("style_attr_dynamic");
}

#[rstest]
fn script_jsdoc_comment() {
    assert_compiler("script_jsdoc_comment");
}

#[rstest]
fn svelte_head_title_meta() {
    assert_compiler("svelte_head_title_meta");
}

#[rstest]
fn snippet_ident_conflict_with_script() {
    assert_compiler("snippet_ident_conflict_with_script");
}

#[rstest]
fn debug_non_dev() {
    assert_compiler("debug_non_dev");
}

#[rstest]
fn debug_non_runes_untrack() {
    assert_compiler("debug_non_runes_untrack");
}

#[rstest]
fn non_runes_simple_snapshot() {
    assert_compiler("non_runes_simple_snapshot");
}

#[rstest]
fn animate_with_spread() {
    assert_compiler("animate_with_spread");
}

#[rstest]
fn svelte_element_static_class_attr() {
    assert_compiler("svelte_element_static_class_attr");
}

#[rstest]
fn root_with_special_elements() {
    assert_compiler("root_with_special_elements");
}

#[rstest]
fn needs_context_method_chain() {
    assert_compiler("needs_context_method_chain");
}

#[rstest]
fn event_handler_derived_with_class_directives() {
    assert_compiler("event_handler_derived_with_class_directives");
}

#[rstest]
fn event_handler_derived_with_class_object() {
    assert_compiler("event_handler_derived_with_class_object");
}

#[rstest]
fn derived_inside_function() {
    assert_compiler("derived_inside_function");
}

#[rstest]
fn derived_nested_getter() {
    assert_compiler("derived_nested_getter");
}

#[rstest]
fn derived_shorthand_property() {
    assert_compiler("derived_shorthand_property");
}

#[rstest]
fn state_inside_function() {
    assert_compiler("state_inside_function");
}

#[rstest]
fn derived_by_inside_function() {
    assert_compiler("derived_by_inside_function");
}

#[rstest]
fn component_snippet_prop() {
    assert_compiler("component_snippet_prop");
}

#[rstest]
fn component_snippet_with_children() {
    assert_compiler("component_snippet_with_children");
}

#[rstest]
fn component_multiple_snippets() {
    assert_compiler("component_multiple_snippets");
}

#[rstest]
fn component_snippet_only() {
    assert_compiler("component_snippet_only");
}

// ---------------------------------------------------------------------------
// Diagnose: TypeScript import + spread + bind:prop tests
// ---------------------------------------------------------------------------

#[rstest]
fn ts_type_import_comment() {
    assert_compiler("ts_type_import_comment");
}

#[rstest]
fn rest_props_member_access() {
    assert_compiler("rest_props_member_access");
}

#[rstest]
fn component_spread_props() {
    assert_compiler("component_spread_props");
}

#[rstest]
fn component_bind_prop_forward() {
    assert_compiler("component_bind_prop_forward");
}

// ---------------------------------------------------------------------------
// Diagnose: svelte import patterns
// ---------------------------------------------------------------------------

#[rstest]
fn needs_context_nested_fn() {
    assert_compiler("needs_context_nested_fn");
}

#[rstest]
fn member_expr_dynamic_local() {
    assert_compiler("member_expr_dynamic_local");
}

#[rstest]
fn import_type_mixed() {
    assert_compiler("import_type_mixed");
}

#[rstest]
fn derived_in_nested_function() {
    assert_compiler("derived_in_nested_function");
}

#[rstest]
fn derived_local_signal_get() {
    assert_compiler("derived_local_signal_get");
}

#[rstest]
fn svelte_element_duplicate_naming() {
    assert_compiler("svelte_element_duplicate_naming");
}

#[rstest]
fn each_block_no_item() {
    assert_compiler("each_block_no_item");
}

#[rstest]
fn each_block_no_item_multi() {
    assert_compiler("each_block_no_item_multi");
}

#[rstest]
fn each_block_no_item_with_index() {
    assert_compiler("each_block_no_item_with_index");
}

#[rstest]
fn async_if_basic() {
    assert_compiler("async_if_basic");
}

#[test]
fn async_if_else_if_condition() {
    assert_compiler("async_if_else_if_condition");
}

#[test]
fn if_elseif_new_blockers() {
    assert_compiler("if_elseif_new_blockers");
}

#[rstest]
fn async_each_basic() {
    assert_compiler("async_each_basic");
}

#[rstest]
fn async_html_basic() {
    assert_compiler("async_html_basic");
}

#[rstest]
fn async_key_basic() {
    assert_compiler("async_key_basic");
}

#[rstest]
fn async_await_has_await() {
    assert_compiler("async_await_has_await");
}

#[rstest]
fn async_flag_import() {
    assert_compiler("async_flag_import");
}

#[rstest]
fn async_blockers_basic() {
    assert_compiler("async_blockers_basic");
}

#[rstest]
fn async_bind_basic() {
    assert_compiler("async_bind_basic");
}

#[rstest]
fn action_blockers() {
    assert_compiler("action_blockers");
}

#[rstest]
fn attach_blockers() {
    assert_compiler("attach_blockers");
}

#[rstest]
fn transition_blockers() {
    assert_compiler("transition_blockers");
}

#[rstest]
fn animate_blockers() {
    assert_compiler("animate_blockers");
}

#[rstest]
fn async_svelte_element() {
    assert_compiler("async_svelte_element");
}

#[rstest]
fn async_const_tag() {
    assert_compiler("async_const_tag");
}

#[rstest]
fn async_derived_basic() {
    assert_compiler("async_derived_basic");
}

#[rstest]
fn async_derived_destructured() {
    assert_compiler("async_derived_destructured");
}

#[rstest]
fn async_derived_dev() {
    assert_compiler("async_derived_dev");
}

#[rstest]
fn async_derived_dev_ignored() {
    assert_compiler("async_derived_dev_ignored");
}

#[rstest]
fn async_derived_dev_ignored_destructured() {
    assert_compiler("async_derived_dev_ignored_destructured");
}

#[rstest]
fn async_derived_nested_function() {
    assert_compiler("async_derived_nested_function");
}

#[rstest]
fn async_derived_nested_function_destructured() {
    assert_compiler("async_derived_nested_function_destructured");
}

#[rstest]
fn async_for_await_dev() {
    assert_compiler("async_for_await_dev");
}

#[rstest]
fn inline_await_basic() {
    assert_compiler("inline_await_basic");
}

#[rstest]
fn inline_await_global_callee() {
    assert_compiler("inline_await_global_callee");
}

#[rstest]
fn inline_await_text_concat() {
    assert_compiler("inline_await_text_concat");
}

#[rstest]
fn inline_await_attr() {
    assert_compiler("inline_await_attr");
}

#[test]
fn async_pickled_await_template() {
    assert_compiler("async_pickled_await_template");
}

#[rstest]
fn async_render_tag() {
    assert_compiler("async_render_tag");
}

#[rstest]
fn async_render_tag_complex_args() {
    assert_compiler("async_render_tag_complex_args");
}

#[rstest]
fn async_boundary_const() {
    assert_compiler("async_boundary_const");
}

#[rstest]
fn async_const_derived_chain() {
    assert_compiler("async_const_derived_chain");
}

#[rstest]
fn attach_on_component() {
    assert_compiler("attach_on_component");
}

#[rstest]
fn attach_on_component_dynamic() {
    assert_compiler("attach_on_component_dynamic");
}

#[rstest]
fn attach_on_document() {
    assert_compiler("attach_on_document");
}

#[rstest]
fn each_keyed_destructure() {
    assert_compiler("each_keyed_destructure");
}

#[rstest]
fn await_then_text_before_element() {
    assert_compiler("await_then_text_before_element");
}

#[rstest]
fn await_thunk_optimization() {
    assert_compiler("await_thunk_optimization");
}

#[rstest]
fn await_each_nested() {
    assert_compiler("await_each_nested");
}

#[rstest]
fn await_pending_then() {
    assert_compiler("await_pending_then");
}

#[rstest]
fn await_pending_catch() {
    assert_compiler("await_pending_catch");
}

#[rstest]
fn await_short_catch_no_binding() {
    assert_compiler("await_short_catch_no_binding");
}

#[rstest]
fn await_nested_await() {
    assert_compiler("await_nested_await");
}

#[rstest]
fn fragment_counter_with_nested_if() {
    assert_compiler("fragment_counter_with_nested_if");
}

#[rstest]
fn bind_group_radio_basic() {
    assert_compiler("bind_group_radio_basic");
}

#[rstest]
fn bind_multiple_on_element() {
    assert_compiler("bind_multiple_on_element");
}

#[rstest]
fn if_else_chain_with_const() {
    assert_compiler("if_else_chain_with_const");
}

#[rstest]
fn event_mixed_delegation() {
    assert_compiler("event_mixed_delegation");
}

#[rstest]
fn props_identifier_basic() {
    assert_compiler("props_identifier_basic");
}

#[rstest]
fn props_identifier_await_expression() {
    assert_compiler("props_identifier_await_expression");
}

#[rstest]
fn call_expr_local_method_dynamic() {
    assert_compiler("call_expr_local_method_dynamic");
}

#[rstest]
fn call_expr_nested_fn_dynamic() {
    assert_compiler("call_expr_nested_fn_dynamic");
}

#[rstest]
fn effect_cleanup_return() {
    assert_compiler("effect_cleanup_return");
}

#[rstest]
fn tag_derived_basic() {
    assert_compiler("tag_derived_basic");
}

#[rstest]
fn tag_derived_by() {
    assert_compiler("tag_derived_by");
}

#[rstest]
fn derived_destructured_object() {
    assert_compiler("derived_destructured_object");
}

#[rstest]
fn derived_destructured_array() {
    assert_compiler("derived_destructured_array");
}

#[rstest]
fn derived_destructured_by() {
    assert_compiler("derived_destructured_by");
}

#[rstest]
fn tag_state_unmutated() {
    assert_compiler("tag_state_unmutated");
}

#[rstest]
fn tag_state_unmutated_no_proxy() {
    assert_compiler("tag_state_unmutated_no_proxy");
}

#[rstest]
fn tag_bindable_proxy() {
    assert_compiler("tag_bindable_proxy");
}

#[rstest]
fn tag_class_field_public() {
    assert_compiler("tag_class_field_public");
}

#[rstest]
fn tag_class_field_private() {
    assert_compiler("tag_class_field_private");
}

#[rstest]
fn tag_class_constructor() {
    assert_compiler("tag_class_constructor");
}

#[rstest]
fn tag_snippet_dev() {
    assert_compiler("tag_snippet_dev");
}

#[rstest]
fn tag_render_dev() {
    assert_compiler("tag_render_dev");
}

#[rstest]
fn snippet_destructure_dev() {
    assert_compiler("snippet_destructure_dev");
}

#[rstest]
fn snippet_object_destructure() {
    assert_compiler("snippet_object_destructure");
}

#[rstest]
fn snippet_array_destructure() {
    assert_compiler("snippet_array_destructure");
}

#[rstest]
fn snippet_mixed_params() {
    assert_compiler("snippet_mixed_params");
}

#[rstest]
fn snippet_nested_destructure() {
    assert_compiler("snippet_nested_destructure");
}

#[rstest]
fn snippet_computed_key_destructure() {
    assert_compiler("snippet_computed_key_destructure");
}

#[rstest]
fn tag_state_destructured_array() {
    assert_compiler("tag_state_destructured_array");
}

#[rstest]
fn tag_state_destructured_object() {
    assert_compiler("tag_state_destructured_object");
}

#[rstest]
fn state_var_safe_get() {
    assert_compiler("state_var_safe_get");
}

#[rstest]
fn state_assign_dev() {
    assert_compiler("state_assign_dev");
}

#[rstest]
fn css_scoped_class_selector() {
    assert_compiler("css_scoped_class_selector");
}

#[rstest]
fn bind_select_static_option_value() {
    assert_compiler("bind_select_static_option_value");
}

#[rstest]
fn input_dynamic_special_attrs() {
    assert_compiler("input_dynamic_special_attrs");
}

#[rstest]
fn svg_dynamic_special_attrs() {
    assert_compiler("svg_dynamic_special_attrs");
}

#[rstest]
fn each_index_text_no_coalesce() {
    assert_compiler("each_index_text_no_coalesce");
}

#[rstest]
fn snippet_destructure_default_state_ref() {
    assert_compiler("snippet_destructure_default_state_ref");
}

#[rstest]
fn snippet_destructure_default_mutated_state_ref() {
    assert_compiler("snippet_destructure_default_mutated_state_ref");
}

#[rstest]
fn diagnose_props_bindable_icon_component() {
    assert_compiler("diagnose_props_bindable_icon_component");
}

#[rstest]
fn props_bindable_checkbox_disabled_shorthand_ts() {
    assert_compiler("props_bindable_checkbox_disabled_shorthand_ts");
}

#[rstest]
fn diagnose_component_onclick_state() {
    assert_compiler("diagnose_component_onclick_state");
}

#[rstest]
fn diagnose_svg_city_icon() {
    assert_compiler("diagnose_svg_city_icon");
}

#[rstest]
fn clock_svg_derived_onmount() {
    assert_compiler("clock_svg_derived_onmount");
}

#[rstest]
fn diagnose_component_default_and_named_slot_expr() {
    assert_compiler("diagnose_component_default_and_named_slot_expr");
}

#[rstest]
#[ignore = "diagnose: pending fix"]
fn diagnose_runes_dev_ce_benchmark() {
    assert_compiler("diagnose_runes_dev_ce_benchmark");
}

#[rstest]
fn component_dev_default_children_wrap_snippet() {
    assert_compiler("component_dev_default_children_wrap_snippet");
}
