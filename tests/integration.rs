use std::fs::read_to_string;

use glob::glob;
use oxc_allocator::Allocator;
use pretty_assertions::assert_eq;
use svelte_rs_2::compiler::Compiler;

#[test]
fn integration() {
    let files = glob("./tests/cases/**/*.svelte").expect("Не удалось считать компоненты");

    for entry in files {
        let entry = entry.unwrap();
        let allocator = Allocator::default();

        let file = read_to_string(&entry).unwrap();
        let compiler = Compiler::new();

        let actual = compiler.compile(&file, &allocator);
        let path = entry.parent().unwrap();

        let expected = read_to_string(path.join("case-svelte.js")).unwrap();

        assert_eq!(actual.js, expected);
    }
}
