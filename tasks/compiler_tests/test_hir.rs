use rstest::*;
use std::{
    fs::{read_to_string, File},
    io::Write,
    path::Path,
};

use compiler::Compiler;
use oxc_allocator::Allocator;
use pretty_assertions::assert_eq;

fn asser_compiler(case: &str) {
    let path = Path::new("./cases2").join(case).join("case.svelte");
    let allocator = Allocator::default();
    let file = read_to_string(&path).unwrap();

    let compiler = Compiler::new();

    let actual = compiler.compile2(&file, &allocator).unwrap();
    let path = path.parent().unwrap();

    let expected = read_to_string(path.join("case-svelte.js")).unwrap();

    let mut file = File::create(path.join("case-rust.js")).unwrap();

    file.write_all(actual.js.as_bytes()).unwrap();

    assert_eq!(actual.js, expected);
}

#[rstest]
fn single_text_node() {
    asser_compiler("single_text_node");
}
