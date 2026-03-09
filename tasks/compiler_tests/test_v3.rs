use rstest::*;
use std::{
    fs::{read_to_string, File},
    io::Write,
    path::PathBuf,
};

use pretty_assertions::assert_eq;
use svelte_compiler::compile;

#[rstest]
fn integration(#[files("./cases2/**/*.svelte")] path: PathBuf) {
    let file = read_to_string(&path).unwrap();

    let actual = compile(&file).unwrap();
    let path = path.parent().unwrap();

    let expected = read_to_string(path.join("case-svelte.js")).unwrap();

    let mut out = File::create(path.join("case-rust.js")).unwrap();

    out.write_all(actual.js.as_bytes()).unwrap();

    assert_eq!(actual.js, expected);
}
