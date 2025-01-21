use oxc_allocator::Allocator;
use oxc_ast::AstBuilder;

use crate::{
    analyze::Analyzer,
    parser::Parser,
    transformer::{builder::Builder, transform_client},
};

pub struct Compiler {}

pub struct CompilerResult {
    pub js: String,
}

impl Compiler {
    pub fn new() -> Self {
        return Self {};
    }

    pub fn compile<'a>(&self, source: &'a str, allocator: &'a Allocator) -> CompilerResult {
        let mut parser = Parser::new(source, allocator);
        let analyzer = Analyzer::new();
        let oxc_builder = AstBuilder::new(allocator);
        let builder = Builder::new(oxc_builder);

        let ast = parser.parse().unwrap();
        let analyze_result = analyzer.analyze(&ast);

        let code = transform_client(ast, &builder, analyze_result);

        return CompilerResult { js: code };
    }
}
