use std::{
    fs::{read_to_string, File},
    io::Write,
    path::Path,
};

use pretty_assertions::assert_eq;
use rstest::rstest;
use svelte_compiler::{compile, compile_module};

fn assert_compiler(case: &str) {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("cases2")
        .join(case)
        .join("case.svelte");
    let input = read_to_string(&path).unwrap();

    let result = compile(&input);
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

// ---------------------------------------------------------------------------
// Module compilation tests
// ---------------------------------------------------------------------------

fn assert_compiler_module(case: &str) {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("cases2")
        .join(case)
        .join("case.svelte.js");
    let input = read_to_string(&path).unwrap();

    let result = compile_module(&input);
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
