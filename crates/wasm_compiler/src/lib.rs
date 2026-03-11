use oxc_allocator::Allocator;
use oxc_codegen::Codegen;
use oxc_parser::Parser;
use oxc_span::SourceType;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmCompiler {}

#[wasm_bindgen]
impl WasmCompiler {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {}
    }

    #[wasm_bindgen()]
    pub fn compile(&self, source: &str) -> Result<String, serde_wasm_bindgen::Error> {
        let result = svelte_compiler::compile(source)
            .map_err(|diagnostic| serde_wasm_bindgen::Error::new(diagnostic))?;

        Ok(result.js)
    }

    #[wasm_bindgen()]
    pub fn format(&self, source: &str) -> String {
        let allocator = Allocator::default();
        let source_type = SourceType::mjs();
        let parsed = Parser::new(&allocator, source, source_type).parse();
        Codegen::default().build(&parsed.program).code
    }
}
