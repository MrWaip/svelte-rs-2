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
}
