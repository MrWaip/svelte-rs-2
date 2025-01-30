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

    pub fn compile(&self, source: &str) -> String {
        let compiler = Compiler::new();

        let result = compiler.compile(source, &self.allocator);

        return result.js;
    }
}
