use ast_builder::Builder;
use diagnostics::Diagnostic;
use oxc_allocator::Allocator;
use oxc_ast::AstBuilder as OxcBuilder;

use analyzer::Analyzer;
use parser::Parser;
use transformer::transform_client;

pub struct Compiler {}

pub struct CompilerResult {
    pub js: String,
}

impl Compiler {
    pub fn new() -> Self {
        return Self {};
    }

    pub fn compile<'a>(
        &self,
        source: &'a str,
        allocator: &'a Allocator,
    ) -> Result<CompilerResult, Diagnostic> {
        let mut parser = Parser::new(source, allocator);
        let oxc_builder = OxcBuilder::new(allocator);
        let builder = Builder::new(oxc_builder);
        let analyzer = Analyzer::new(&builder);

        let mut ast = parser.parse()?;
        let analyze_result = analyzer.analyze(&mut ast);

        let code = transform_client(ast, &builder, analyze_result);

        return Ok(CompilerResult { js: code });
    }
}

#[cfg(test)]
mod tests {
    use oxc_allocator::Allocator;
    use pretty_assertions::assert_eq;

    use crate::Compiler;

    #[test]
    fn trim_whitespaces() {
        let allocator = Allocator::default();
        let compiler = Compiler::new();

        let result = compiler
            .compile("<script></script>    ", &allocator)
            .unwrap();

        assert_eq!(
            result.js,
            r#"import * as $ from "svelte/internal/client";
export default function App($$anchor) {}
"#
        );
    }
}
