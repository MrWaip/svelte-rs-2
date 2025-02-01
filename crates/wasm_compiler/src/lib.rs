use compiler::Compiler;
use oxc_allocator::Allocator;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmCompiler {
    allocator: Allocator,
}

#[wasm_bindgen]
impl WasmCompiler {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        return Self {
            allocator: Allocator::default(),
        };
    }

    #[wasm_bindgen(catch, method)]
    pub fn compile(&self, source: &str) -> Result<String, serde_wasm_bindgen::Error> {
        let compiler = Compiler::new();

        let result = compiler
            .compile(source, &self.allocator)
            .map_err(|diagnostic| serde_wasm_bindgen::Error::new(diagnostic))?;

        return Ok(result.js);
    }
}
