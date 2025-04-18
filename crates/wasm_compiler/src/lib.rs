#![allow(
    unused_variables,
    // clippy::extra_unused_type_parameters,
    // clippy::explicit_iter_loop,
    // clippy::self_named_module_files,
    // clippy::semicolon_if_nothing_returned,
    // clippy::match_wildcard_for_single_variants
)]

use compiler::Compiler;
use oxc_allocator::Allocator;
use oxc_codegen::Codegen;
use oxc_span::SourceType;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmCompiler {}

#[wasm_bindgen]
impl WasmCompiler {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        return Self {};
    }

    #[wasm_bindgen()]
    pub fn compile(&self, source: &str) -> Result<String, serde_wasm_bindgen::Error> {
        let allocator = Allocator::default();

        let compiler = Compiler::new();

        let result = compiler
            .compile2(source, &allocator)
            .map_err(|diagnostic| serde_wasm_bindgen::Error::new(diagnostic))?;

        return Ok(result.js);
    }

    #[wasm_bindgen()]
    pub fn format(&self, source: &str) -> String {
        let allocator = Allocator::default();
        let parser = oxc_parser::Parser::new(&allocator, source, SourceType::default());
        let codegen = Codegen::default();

        let ast = parser.parse();

        return codegen.build(&ast.program).code;
    }
}
