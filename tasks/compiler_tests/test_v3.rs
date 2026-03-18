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

    let opts = CompileOptions { name: Some("App".into()), ..Default::default() };
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
fn single_if_block() {
    assert_compiler("single_if_block");
}

#[rstest]
fn single_if_else_block() {
    assert_compiler("single_if_else_block");
}

#[rstest]
fn element_attributes() {
    assert_compiler("element_attributes");
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
fn component_element_children() {
    assert_compiler("component_element_children");
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
fn html_tag() {
    assert_compiler("html_tag");
}

#[rstest]
fn key_block() {
    assert_compiler("key_block");
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
fn component_bind_this_plain() {
    assert_compiler("component_bind_this_plain");
}

#[rstest]
fn component_bind_this_props() {
    assert_compiler("component_bind_this_props");
}

#[rstest]
fn component_bind_this_member() {
    assert_compiler("component_bind_this_member");
}

#[rstest]
fn component_bind_this_each() {
    assert_compiler("component_bind_this_each");
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
fn svelte_head_multiple() {
    assert_compiler("svelte_head_multiple");
}

#[rstest]
fn svelte_head_empty() {
    assert_compiler("svelte_head_empty");
}

#[rstest]
fn svelte_head_with_content() {
    assert_compiler("svelte_head_with_content");
}

// <title> in <svelte:head> tests
#[rstest]
fn title_static() {
    assert_compiler("title_static");
}

#[rstest]
fn title_dynamic() {
    assert_compiler("title_dynamic");
}

#[rstest]
fn title_mixed() {
    assert_compiler("title_mixed");
}

#[rstest]
fn title_empty() {
    assert_compiler("title_empty");
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
fn svelte_document_event_legacy() {
    assert_compiler("svelte_document_event_legacy");
}

#[rstest]
fn svelte_document_event_attr() {
    assert_compiler("svelte_document_event_attr");
}

#[rstest]
fn svelte_document_event_modifiers() {
    assert_compiler("svelte_document_event_modifiers");
}

#[rstest]
fn svelte_document_multiple_events() {
    assert_compiler("svelte_document_multiple_events");
}

#[rstest]
fn svelte_document_bubble() {
    assert_compiler("svelte_document_bubble");
}

#[rstest]
fn svelte_document_bind_active_element() {
    assert_compiler("svelte_document_bind_active_element");
}

#[rstest]
fn svelte_document_bind_fullscreen() {
    assert_compiler("svelte_document_bind_fullscreen");
}

#[rstest]
fn svelte_document_bind_visibility() {
    assert_compiler("svelte_document_bind_visibility");
}

#[rstest]
fn svelte_document_bind_pointer_lock() {
    assert_compiler("svelte_document_bind_pointer_lock");
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