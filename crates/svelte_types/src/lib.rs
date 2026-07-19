use oxc_allocator::Allocator;
use svelte_analyze::{AnalysisData, IdentGen, JsAst};
use svelte_ast::Component;
use svelte_sourcemap::SourcemapKind;
use svelte_span::LineIndex;

pub struct CompileContext<'a, 'ctx> {
    pub alloc: &'a Allocator,
    pub component: &'ctx Component,
    pub analysis: &'ctx AnalysisData<'a>,
    pub js_arena: &'ctx mut JsAst<'a>,
    pub ident_gen: &'ctx mut IdentGen,
    pub line_index: &'ctx LineIndex,
}

#[derive(Default)]
pub struct TransformOptions {
    pub dev: bool,
    pub filename: String,
}

#[derive(Default)]
pub struct CodegenOptions {
    pub dev: bool,
    pub hmr: bool,
    pub experimental_async: bool,
    pub disclose_version: bool,
    pub filename: String,
    pub sourcemap_kind: SourcemapKind,
}
