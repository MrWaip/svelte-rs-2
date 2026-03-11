use std::{
    fs::{read_to_string, File},
    io::Write,
    path::Path,
};

use pretty_assertions::assert_eq;
use rstest::rstest;
use svelte_compiler::compile;

fn assert_compiler(case: &str) {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("cases2")
        .join(case)
        .join("case.svelte");
    let input = read_to_string(&path).unwrap();

    let actual = compile(&input).unwrap_or_else(|e| panic!("[{case}] compile error: {e:?}"));

    let dir = path.parent().unwrap();
    let expected = read_to_string(dir.join("case-svelte.js")).unwrap();

    File::create(dir.join("case-rust.js"))
        .unwrap()
        .write_all(actual.js.as_bytes())
        .unwrap();

    assert_eq!(actual.js, expected);
}

#[rstest]
fn empty() {
    assert_compiler("empty");
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
