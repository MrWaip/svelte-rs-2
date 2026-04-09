use napi_derive::napi;
use svelte_compiler::{
    CompileOptions, CompileResult, CssMode, GenerateMode, ModuleCompileOptions, Namespace,
};
use svelte_diagnostics::LineIndex;

#[napi(object)]
pub struct NativeDiagnostic {
    pub code: String,
    pub message: String,
    pub severity: String,
    pub start_line: u32,
    pub start_col: u32,
    pub end_line: u32,
    pub end_col: u32,
    pub frame: Option<String>,
}

#[napi(object)]
pub struct NativeCompileResult {
    pub js: Option<String>,
    pub css: Option<String>,
    pub diagnostics: Vec<NativeDiagnostic>,
}

#[napi(object)]
#[derive(Default)]
pub struct NativeCompileOptions {
    pub dev: Option<bool>,
    pub filename: Option<String>,
    pub root_dir: Option<String>,
    pub name: Option<String>,
    pub custom_element: Option<bool>,
    pub namespace: Option<String>,
    pub css: Option<String>,
    pub runes: Option<bool>,
    pub preserve_comments: Option<bool>,
    pub preserve_whitespace: Option<bool>,
    pub disclose_version: Option<bool>,
    pub hmr: Option<bool>,
    pub accessors: Option<bool>,
    pub immutable: Option<bool>,
    pub compatibility_component_api: Option<u8>,
    pub experimental_async: Option<bool>,
    pub generate: Option<String>,
}

#[napi(object)]
#[derive(Default)]
pub struct NativeModuleCompileOptions {
    pub dev: Option<bool>,
    pub filename: Option<String>,
    pub root_dir: Option<String>,
    pub generate: Option<String>,
}

#[napi]
pub fn compile(source: String, options: Option<NativeCompileOptions>) -> NativeCompileResult {
    let options = to_compile_options(options.unwrap_or_default());
    let result = svelte_compiler::compile(&source, &options);
    to_node_result(result, &source)
}

#[napi(js_name = "compileModule")]
pub fn compile_module(
    source: String,
    options: Option<NativeModuleCompileOptions>,
) -> NativeCompileResult {
    let options = to_module_compile_options(options.unwrap_or_default());
    let result = svelte_compiler::compile_module(&source, &options);
    to_node_result(result, &source)
}

fn to_compile_options(native: NativeCompileOptions) -> CompileOptions {
    let mut options = CompileOptions::default();
    if let Some(value) = native.dev {
        options.dev = value;
    }
    if let Some(value) = native.generate {
        options.generate = parse_generate_mode(&value);
    }
    if let Some(value) = native.filename {
        options.filename = value;
    }
    if let Some(value) = native.root_dir {
        options.root_dir = Some(value);
    }
    if let Some(value) = native.name {
        options.name = Some(value);
    }
    if let Some(value) = native.custom_element {
        options.custom_element = value;
    }
    if let Some(value) = native.namespace {
        options.namespace = parse_namespace(&value);
    }
    if let Some(value) = native.css {
        options.css = parse_css_mode(&value);
    }
    if let Some(value) = native.runes {
        options.runes = Some(value);
    }
    if let Some(value) = native.preserve_comments {
        options.preserve_comments = value;
    }
    if let Some(value) = native.preserve_whitespace {
        options.preserve_whitespace = value;
    }
    if let Some(value) = native.disclose_version {
        options.disclose_version = value;
    }
    if let Some(value) = native.hmr {
        options.hmr = value;
    }
    if let Some(value) = native.accessors {
        options.accessors = value;
    }
    if let Some(value) = native.immutable {
        options.immutable = value;
    }
    if let Some(value) = native.compatibility_component_api {
        options.compatibility_component_api = value;
    }
    if let Some(value) = native.experimental_async {
        options.experimental.async_ = value;
    }
    options
}

fn to_module_compile_options(native: NativeModuleCompileOptions) -> ModuleCompileOptions {
    let mut options = ModuleCompileOptions::default();
    if let Some(value) = native.dev {
        options.dev = value;
    }
    if let Some(value) = native.generate {
        options.generate = parse_generate_mode(&value);
    }
    if let Some(value) = native.filename {
        options.filename = value;
    }
    if let Some(value) = native.root_dir {
        options.root_dir = Some(value);
    }
    options
}

fn parse_generate_mode(raw: &str) -> GenerateMode {
    match raw {
        "server" => GenerateMode::Server,
        "false" => GenerateMode::False,
        _ => GenerateMode::Client,
    }
}

fn parse_namespace(raw: &str) -> Namespace {
    match raw {
        "svg" => Namespace::Svg,
        "mathml" => Namespace::MathMl,
        _ => Namespace::Html,
    }
}

fn parse_css_mode(raw: &str) -> CssMode {
    match raw {
        "injected" => CssMode::Injected,
        _ => CssMode::External,
    }
}

fn to_node_result(result: CompileResult, source: &str) -> NativeCompileResult {
    let line_index = LineIndex::new(source);

    let diagnostics = result
        .diagnostics
        .iter()
        .map(|diagnostic| {
            let (start_line, start_col) = line_index.line_col(diagnostic.span.start as usize);
            let (end_line, end_col) = line_index.line_col(diagnostic.span.end as usize);
            let mut message = diagnostic.kind.message();
            if let Some(url) = diagnostic.kind.svelte_doc_url() {
                message.push('\n');
                message.push_str(&url);
            }

            NativeDiagnostic {
                code: diagnostic.kind.code().to_string(),
                message,
                severity: format!("{:?}", diagnostic.severity),
                start_line: start_line as u32,
                start_col: start_col as u32,
                end_line: end_line as u32,
                end_col: end_col as u32,
                frame: line_index.code_frame(source, diagnostic.span),
            }
        })
        .collect();

    NativeCompileResult {
        js: result.js,
        css: result.css,
        diagnostics,
    }
}
