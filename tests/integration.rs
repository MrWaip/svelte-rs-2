use std::fs::read_to_string;

use glob::glob;
use oxc_allocator::Allocator;
use oxc_ast::AstBuilder;
use pretty_assertions::assert_eq;
use svelte_rs_2::{
    parser::Parser,
    transformer::{builder::Builder, transform_client},
};

#[test]
fn integration() {
    let files = glob("./tests/cases/**/*.svelte").expect("Не удалось считать компоненты");

    for entry in files {
        let entry = entry.unwrap();
        let allocator = Allocator::default();

        let file = read_to_string(&entry).unwrap();
        let mut parser = Parser::new(&file, &allocator);
        let ast = parser.parse().unwrap();
        let ast_builder = AstBuilder::new(&allocator);
        let builder = Builder::new(ast_builder);

        let actual = transform_client(ast, &builder);
        let path = entry.parent().unwrap();

        let expected = read_to_string(path.join("case-svelte.js")).unwrap();

        assert_eq!(actual, expected);
    }
}
