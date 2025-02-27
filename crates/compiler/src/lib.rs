use analyze_hir::AnalyzeHir;
use ast_builder::Builder;
use ast_to_hir::AstToHir;
use diagnostics::Diagnostic;
use oxc_allocator::Allocator;
use oxc_ast::AstBuilder as OxcBuilder;

use analyzer::Analyzer;
use oxc_codegen::Codegen;
use parser::Parser;
use transform_hir::transform_hir;
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

    pub fn compile2<'a>(
        &self,
        source: &'a str,
        allocator: &'a Allocator,
    ) -> Result<CompilerResult, Diagnostic> {
        let mut parser = Parser::new(source, allocator);
        let codegen = Codegen::default();
        let analyze_hir = AnalyzeHir::new();

        let mut lowerer = AstToHir::new();

        let ast = parser.parse()?;

        let _hir = lowerer.traverse(ast, &allocator);
        analyze_hir.analyze();
        let program = transform_hir(allocator);

        return Ok(CompilerResult {
            js: codegen.build(&program).code,
        });
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

    #[test]
    fn smoke_compile2() {
        let allocator = Allocator::default();
        let compiler = Compiler::new();

        let result = compiler.compile2("text", &allocator).unwrap();

        assert_eq!(result.js, r#""#);
    }
}
