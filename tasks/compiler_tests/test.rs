use rstest::*;
use std::{
    fs::{read_to_string, File},
    io::Write,
    path::PathBuf,
    
};

use oxc_allocator::Allocator;
use pretty_assertions::assert_eq;
use compiler::Compiler;

#[rstest]
fn integration(#[files("./cases/**/*.svelte")] path: PathBuf) {
    let allocator = Allocator::default();

    let file = read_to_string(&path).unwrap();
    let compiler = Compiler::new();

    let actual = compiler.compile(&file, &allocator);
    let path = path.parent().unwrap();

    let expected = read_to_string(path.join("case-svelte.js")).unwrap();

    let mut file = File::create(path.join("case-rust.js")).unwrap();

    file.write_all(actual.js.as_bytes()).unwrap();

    assert_eq!(actual.js, expected);
}
