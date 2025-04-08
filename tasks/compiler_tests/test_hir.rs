use rstest::*;
use std::{
    fs::{read_to_string, File},
    io::Write,
    path::Path,
};

use compiler::Compiler;
use oxc_allocator::Allocator;
use pretty_assertions::assert_eq;

fn assert_compiler(case: &str) {
    let path = Path::new("./cases2").join(case).join("case.svelte");
    let allocator = Allocator::default();
    let file = read_to_string(&path).unwrap();

    let compiler = Compiler::new();

    let actual = compiler.compile2(&file, &allocator).unwrap();
    let path = path.parent().unwrap();

    let expected = read_to_string(path.join("case-svelte.js")).unwrap();

    let output_path = path.join("case-rust.js");
    let mut file = File::create(&output_path).unwrap();

    println!(
        "Output path: {}",
        &output_path.into_os_string().into_string().unwrap()
    );

    file.write_all(actual.js.as_bytes()).unwrap();

    assert_eq!(actual.js, expected);
}

#[rstest]
fn empty() {
    assert_compiler("single_text_node");
}

#[rstest]
fn single_text_node() {
    assert_compiler("empty");
}

#[rstest]
fn single_concatenation() {
    assert_compiler("single_concatenation");
}

#[rstest]
fn single_interpolation() {
    assert_compiler("single_interpolation");
}

#[rstest]
fn single_element() {
    assert_compiler("single_element");
}

#[rstest]
fn generic_root_sequence() {
    assert_compiler("generic_root_sequence");
}

#[rstest]
fn elements_childs() {
    assert_compiler("elements_childs");
}

#[rstest]
fn element_attributes() {
    assert_compiler("element_attributes");
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
fn utf8() {
    assert_compiler("utf8");
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
fn smoke() {
    assert_compiler("smoke");
}

#[rstest]
fn spread_attribute() {
    assert_compiler("spread_attribute");
}

#[rstest]
fn state_runes() {
    assert_compiler("state_runes");
}

#[rstest]
fn bind_directives() {
    assert_compiler("bind_directives");
}

#[rstest]
fn each_block() {
    assert_compiler("each_block");
}
