use std::{
    fs::{read_to_string, File},
    io::Write,
    path::Path,
};

use pretty_assertions::assert_eq;
use rstest::rstest;
use svelte_compiler::{compile, compile_module, CompileOptions, ModuleCompileOptions};

fn assert_compiler(case: &str) {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("cases2")
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
        if let Some(ce) = config.get("customElement").and_then(|v| v.as_bool()) {
            opts.custom_element = ce;
        }
        if let Some(filename) = config.get("filename").and_then(|v| v.as_str()) {
            opts.filename = filename.to_string();
        }
        if let Some(exp) = config.get("experimental") {
            if let Some(async_val) = exp.get("async").and_then(|v| v.as_bool()) {
                opts.experimental.async_ = async_val;
            }
        }
    }
    let result = compile(&input, &opts);
    let js = result
        .js
        .unwrap_or_else(|| panic!("[{case}] compile produced no JS"));

    let dir = path.parent().unwrap();
    let expected = read_to_string(dir.join("case-svelte.js")).unwrap();

    File::create(dir.join("case-rust.js"))
        .unwrap()
        .write_all(js.as_bytes())
        .unwrap();

    assert_eq!(js, expected);
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
#[ignore = "missing: known v3 parity gap"]
fn css_scoped_basic() {
    assert_compiler("css_scoped_basic");
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
#[ignore = "missing: known v3 parity gap"]
fn text_entity_decoding() {
    assert_compiler("text_entity_decoding");
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
#[ignore = "missing: known v3 parity gap"]
fn if_call_condition() {
    assert_compiler("if_call_condition");
}

#[rstest]
fn element_attributes() {
    assert_compiler("element_attributes");
}

#[rstest]
#[ignore = "missing: known v3 parity gap"]
fn element_autofocus() {
    assert_compiler("element_autofocus");
}

#[rstest]
#[ignore = "missing: known v3 parity gap"]
fn textarea_child_value_dynamic() {
    assert_compiler("textarea_child_value_dynamic");
}

#[rstest]
#[ignore = "missing: known v3 parity gap"]
fn option_expr_child_value() {
    assert_compiler("option_expr_child_value");
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
#[ignore = "missing: known v3 parity gap"]
fn spread_class_directive() {
    assert_compiler("spread_class_directive");
}

#[rstest]
#[ignore = "missing: known v3 parity gap"]
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
#[ignore = "missing: known v3 parity gap"]
fn svelte_component_basic() {
    assert_compiler("svelte_component_basic");
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
#[ignore = "missing: known v3 parity gap"]
fn component_events() {
    assert_compiler("component_events");
}

#[rstest]
fn component_element_children() {
    assert_compiler("component_element_children");
}

#[rstest]
#[ignore = "missing: known v3 parity gap"]
fn component_named_slot() {
    assert_compiler("component_named_slot");
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
fn host_basic() {
    assert_compiler("host_basic");
}

#[rstest]
#[ignore = "missing: $host rest props should exclude $$host"]
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
fn html_tag() {
    assert_compiler("html_tag");
}

#[rstest]
fn html_tag_mathml() {
    assert_compiler("html_tag_mathml");
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
#[ignore = "missing: known v3 parity gap"]
fn on_directive_nonpassive() {
    assert_compiler("on_directive_nonpassive");
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
#[ignore = "missing: known v3 parity gap"]
fn store_validate_dev() {
    assert_compiler("store_validate_dev");
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
#[ignore = "missing: known v3 parity gap"]
fn svelte_self_if() {
    assert_compiler("svelte_self_if");
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
#[ignore = "missing: constructor-assigned $derived.by class field rewrite drops original field declaration"]
fn derived_by_class_fields() {
    assert_compiler("derived_by_class_fields");
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

// ---------------------------------------------------------------------------
// Module compilation tests
// ---------------------------------------------------------------------------

fn assert_compiler_module(case: &str) {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("cases2")
        .join(case)
        .join("case.svelte.js");
    let input = read_to_string(&path).unwrap();

    let result = compile_module(&input, &ModuleCompileOptions::default());
    let js = result
        .js
        .unwrap_or_else(|| panic!("[{case}] compile_module produced no JS"));

    let dir = path.parent().unwrap();
    let expected = read_to_string(dir.join("case-svelte.js")).unwrap();

    File::create(dir.join("case-rust.js"))
        .unwrap()
        .write_all(js.as_bytes())
        .unwrap();

    assert_eq!(js, expected);
}

#[rstest]
fn module_compilation() {
    assert_compiler_module("module_compilation");
}

#[rstest]
fn svelte_options_basic() {
    assert_compiler("svelte_options_basic");
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
#[ignore = "missing: known v3 parity gap"]
fn svelte_fragment_named_slot() {
    assert_compiler("svelte_fragment_named_slot");
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
#[ignore = "missing: known v3 parity gap"]
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
#[ignore = "missing: known v3 parity gap"]
fn html_tag_nested_svg() {
    assert_compiler("html_tag_nested_svg");
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
#[ignore = "missing: known v3 parity gap"]
fn debug_non_runes_untrack() {
    assert_compiler("debug_non_runes_untrack");
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
fn async_if_basic() {
    assert_compiler("async_if_basic");
}

#[test]
#[ignore = "missing: known v3 parity gap"]
fn async_if_else_if_condition() {
    assert_compiler("async_if_else_if_condition");
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
fn async_for_await_dev() {
    assert_compiler("async_for_await_dev");
}

#[rstest]
fn inline_await_basic() {
    assert_compiler("inline_await_basic");
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
#[ignore = "missing: snippet parameter object destructuring (codegen)"]
fn snippet_object_destructure() {
    assert_compiler("snippet_object_destructure");
}

#[rstest]
#[ignore = "missing: snippet parameter array destructuring (codegen)"]
fn snippet_array_destructure() {
    assert_compiler("snippet_array_destructure");
}

#[rstest]
#[ignore = "missing: snippet mixed parameter types (codegen)"]
fn snippet_mixed_params() {
    assert_compiler("snippet_mixed_params");
}

#[rstest]
fn tag_state_destructured_array() {
    assert_compiler("tag_state_destructured_array");
}

#[rstest]
#[ignore = "missing: $.safe_get for var-declared state (codegen)"]
fn state_var_safe_get() {
    assert_compiler("state_var_safe_get");
}

#[rstest]
#[ignore = "missing: dev-mode $.assign_* transforms for non-statement member assignment (codegen)"]
fn state_assign_dev() {
    assert_compiler("state_assign_dev");
}
