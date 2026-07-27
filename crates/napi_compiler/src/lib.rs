use std::any::Any;
use std::panic;

use napi::bindgen_prelude::{Env, External, Object};
use napi_derive::napi;
use svelte_ast::{Attribute, Script};
use svelte_compiler::{
    CompileOptions, CompileResult, CssMode, GenerateMode, ModuleCompileOptions, Namespace,
    SourceMap,
};
use svelte_diagnostics::{Diagnostic, LineIndex};
use svelte_span::Span;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

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
pub struct NativeJsOutput {
    pub code: String,
    pub map: Option<String>,
}

#[napi(object)]
pub struct NativeCssOutput {
    pub code: String,
    pub map: Option<String>,
    pub has_global: bool,
}

#[napi(object)]
pub struct NativeCompileResult {
    pub js: Option<NativeJsOutput>,
    pub css: Option<NativeCssOutput>,
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
    pub sourcemap: Option<String>,
    pub suppress: Option<Vec<String>>,
}

#[napi(object)]
#[derive(Default)]
pub struct NativeModuleCompileOptions {
    pub dev: Option<bool>,
    pub filename: Option<String>,
    pub root_dir: Option<String>,
    pub generate: Option<String>,
    pub sourcemap: Option<String>,
    pub suppress: Option<Vec<String>>,
}

#[napi]
pub fn compile(source: String, options: Option<NativeCompileOptions>) -> NativeCompileResult {
    let options = to_compile_options(options.unwrap_or_default());
    let strip_single_source = options.preprocessor_map.is_none();
    let result = catch_compile(|| svelte_compiler::compile(&source, &options));
    to_node_result(result, &source, strip_single_source)
}

#[napi(js_name = "compileModule")]
pub fn compile_module(
    source: String,
    options: Option<NativeModuleCompileOptions>,
) -> NativeCompileResult {
    let options = to_module_compile_options(options.unwrap_or_default());
    let strip_single_source = options.preprocessor_map.is_none();
    let result = catch_compile(|| svelte_compiler::compile_module(&source, &options));
    to_node_result(result, &source, strip_single_source)
}

fn catch_compile(f: impl FnOnce() -> CompileResult) -> CompileResult {
    match panic::catch_unwind(panic::AssertUnwindSafe(f)) {
        Ok(result) => result,
        Err(payload) => {
            let message = panic_message(&payload);
            CompileResult {
                js: None,
                css: None,
                diagnostics: vec![Diagnostic::internal_error(message)],
            }
        }
    }
}

fn panic_message(payload: &Box<dyn Any + Send>) -> String {
    if let Some(s) = payload.downcast_ref::<String>() {
        s.clone()
    } else if let Some(s) = payload.downcast_ref::<&str>() {
        s.to_string()
    } else {
        "unknown internal error".to_string()
    }
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
        options.runes = if value {
            svelte_compiler::RunesOption::Runes
        } else {
            svelte_compiler::RunesOption::Legacy
        };
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
    if let Some(value) = native.sourcemap {
        options.preprocessor_map = Some(value);
    }
    if let Some(value) = native.suppress {
        options.suppress = value;
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
    if let Some(value) = native.sourcemap {
        options.preprocessor_map = Some(value);
    }
    if let Some(value) = native.suppress {
        options.suppress = value;
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

fn to_node_result(
    result: CompileResult,
    source: &str,
    strip_single_source: bool,
) -> NativeCompileResult {
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
        js: result.js.map(|out| NativeJsOutput {
            code: out.code,
            map: out.map.map(|m| map_to_json(m, strip_single_source)),
        }),
        css: result.css.map(|out| NativeCssOutput {
            code: out.code,
            map: out.map.map(|m| map_to_json(m, strip_single_source)),
            has_global: out.has_global,
        }),
        diagnostics,
    }
}

fn map_to_json(mut map: SourceMap, strip_single_source: bool) -> String {
    if strip_single_source && map.get_sources().count() == 1 {
        map.set_source_contents(vec![None]);
    }
    map.to_json_string()
}

#[napi(object)]
pub struct NativeTagAttribute {
    pub name: String,
    pub value: Option<String>,
}

#[napi(object)]
pub struct NativeTagRegion {
    pub tag_start: u32,
    pub tag_end: u32,
    pub content_start: u32,
    pub content_end: u32,
    pub content: String,
    pub attributes: Vec<NativeTagAttribute>,
}

#[napi(object)]
pub struct NativeScriptRegion {
    pub region: NativeTagRegion,
    pub is_typescript: bool,
    pub is_module: bool,
}

#[napi(object)]
#[derive(Default)]
pub struct NativePreprocessorRegions {
    pub instance_script: Option<NativeScriptRegion>,
    pub module_script: Option<NativeScriptRegion>,
    pub style: Option<NativeTagRegion>,
}

#[napi(js_name = "findPreprocessorRegions")]
pub fn find_preprocessor_regions(source: String) -> NativePreprocessorRegions {
    let (component, _diagnostics) = svelte_parser::Parser::new(&source).parse();

    NativePreprocessorRegions {
        instance_script: component
            .instance_script
            .as_ref()
            .map(|script| to_native_script_region(&source, script)),
        module_script: component
            .module_script
            .as_ref()
            .map(|script| to_native_script_region(&source, script)),
        style: component
            .css
            .as_ref()
            .map(|css| to_native_region(&source, css.span, css.content_span, &css.attributes)),
    }
}

fn to_native_script_region(source: &str, script: &Script) -> NativeScriptRegion {
    NativeScriptRegion {
        region: to_native_region(source, script.span, script.content_span, &script.attributes),
        is_typescript: script.language == svelte_ast::ScriptLanguage::TypeScript,
        is_module: script.context == svelte_ast::ScriptContext::Module,
    }
}

fn to_native_region(
    source: &str,
    span: Span,
    content_span: Span,
    attributes: &[Attribute],
) -> NativeTagRegion {
    NativeTagRegion {
        tag_start: span.start,
        tag_end: span.end,
        content_start: content_span.start,
        content_end: content_span.end,
        content: content_span.source_text(source).to_string(),
        attributes: attributes
            .iter()
            .filter_map(|attr| to_native_attribute(source, attr))
            .collect(),
    }
}

#[napi(js_name = "spliceRegion")]
pub fn splice_region<'env>(
    env: Env,
    document: String,
    document_map: Option<&'env External<SourceMap<'static>>>,
    region_start: u32,
    region_end: u32,
    new_content: String,
    new_content_map: Option<String>,
    filename: String,
) -> napi::Result<Object<'env>> {
    let new_content_map = new_content_map
        .as_deref()
        .and_then(svelte_sourcemap::parse_input_map);

    let (code, map) = svelte_sourcemap::splice_region(
        &document,
        document_map.map(|external| &**external),
        Span::new(region_start, region_end),
        &new_content,
        new_content_map.as_ref(),
        &filename,
    );

    let mut result = Object::new(&env)?;
    result.set("code", code)?;
    result.set("map", External::new(map))?;
    Ok(result)
}

#[napi(js_name = "sourceMapToJson")]
pub fn source_map_to_json(map: &External<SourceMap<'static>>) -> String {
    map.to_json_string()
}

fn to_native_attribute(source: &str, attr: &Attribute) -> Option<NativeTagAttribute> {
    match attr {
        Attribute::StringAttribute(a) => Some(NativeTagAttribute {
            name: a.name.clone(),
            value: Some(a.value(source).to_string()),
        }),
        Attribute::BooleanAttribute(a) => Some(NativeTagAttribute {
            name: a.name.clone(),
            value: None,
        }),
        Attribute::ExpressionAttribute(a) => Some(NativeTagAttribute {
            name: a.name.clone(),
            value: Some(a.expression.span.source_text(source).trim().to_string()),
        }),
        Attribute::ConcatenationAttribute(_)
        | Attribute::SpreadAttribute(_)
        | Attribute::ClassDirective(_)
        | Attribute::StyleDirective(_)
        | Attribute::BindDirective(_)
        | Attribute::LetDirectiveLegacy(_)
        | Attribute::UseDirective(_)
        | Attribute::OnDirectiveLegacy(_)
        | Attribute::TransitionDirective(_)
        | Attribute::AnimateDirective(_)
        | Attribute::AttachTag(_) => None,
    }
}
