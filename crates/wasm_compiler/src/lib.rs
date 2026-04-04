use oxc_allocator::Allocator;
use oxc_codegen::Codegen;
use oxc_parser::Parser;
use oxc_span::SourceType;
use serde::Serialize;
use svelte_compiler::{CompileOptions, CompileResult, ModuleCompileOptions};
use svelte_diagnostics::LineIndex;
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
struct WasmDiagnostic {
    code: String,
    message: String,
    severity: String,
    start_line: usize,
    start_col: usize,
    end_line: usize,
    end_col: usize,
    frame: Option<String>,
}

#[derive(Serialize)]
struct WasmCompileResult {
    js: Option<String>,
    css: Option<String>,
    diagnostics: Vec<WasmDiagnostic>,
}

fn to_wasm_result(result: CompileResult, source: &str) -> WasmCompileResult {
    let line_index = LineIndex::new(source);

    let diagnostics: Vec<WasmDiagnostic> = result
        .diagnostics
        .iter()
        .map(|d| {
            let (start_line, start_col) = line_index.line_col(d.span.start as usize);
            let (end_line, end_col) = line_index.line_col(d.span.end as usize);
            let mut message = d.kind.message();
            if let Some(url) = d.kind.svelte_doc_url() {
                message.push('\n');
                message.push_str(&url);
            }
            WasmDiagnostic {
                code: d.kind.code().to_string(),
                message,
                severity: format!("{:?}", d.severity),
                start_line,
                start_col,
                end_line,
                end_col,
                frame: line_index.code_frame(source, d.span),
            }
        })
        .collect();

    WasmCompileResult {
        js: result.js,
        css: result.css,
        diagnostics,
    }
}

#[wasm_bindgen]
pub struct WasmCompiler {}

#[wasm_bindgen]
impl WasmCompiler {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {}
    }

    #[wasm_bindgen()]
    pub fn compile(
        &self,
        source: &str,
        options: JsValue,
    ) -> Result<JsValue, serde_wasm_bindgen::Error> {
        let opts: CompileOptions = if options.is_undefined() || options.is_null() {
            CompileOptions::default()
        } else {
            serde_wasm_bindgen::from_value(options)?
        };
        let result = svelte_compiler::compile(source, &opts);
        serde_wasm_bindgen::to_value(&to_wasm_result(result, source))
    }

    #[wasm_bindgen()]
    pub fn compile_module(
        &self,
        source: &str,
        options: JsValue,
    ) -> Result<JsValue, serde_wasm_bindgen::Error> {
        let opts: ModuleCompileOptions = if options.is_undefined() || options.is_null() {
            ModuleCompileOptions::default()
        } else {
            serde_wasm_bindgen::from_value(options)?
        };
        let result = svelte_compiler::compile_module(source, &opts);
        serde_wasm_bindgen::to_value(&to_wasm_result(result, source))
    }

    #[wasm_bindgen()]
    pub fn format(&self, source: &str) -> String {
        let allocator = Allocator::default();
        let source_type = SourceType::mjs();
        let parsed = Parser::new(&allocator, source, source_type).parse();
        Codegen::default().build(&parsed.program).code
    }

    #[wasm_bindgen()]
    pub fn format_css(&self, source: &str) -> String {
        use lightningcss::stylesheet::{ParserOptions, PrinterOptions, StyleSheet};
        StyleSheet::parse(source, ParserOptions::default())
            .ok()
            .and_then(|ss| ss.to_css(PrinterOptions::default()).ok())
            .map(|r| r.code)
            .unwrap_or_else(|| source.to_string())
    }
}
