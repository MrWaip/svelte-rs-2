pub(crate) mod passes;
pub mod scope;
pub mod types;
pub(crate) mod utils;
mod validate;
pub(crate) mod walker;

pub use types::data::{
    AnalysisData, AwaitBindingData, AwaitBindingInfo, ClassDirectiveInfo, ComponentBindMode,
    ComponentPropInfo, ComponentPropKind, ConstTagData, ContentStrategy, DebugTagData,
    DestructureKind, ElementFlags, EventHandlerMode, ExpressionInfo, ExpressionKind, FragmentData,
    FragmentItem, FragmentKey, LoweredFragment, LoweredTextPart, ParserResult, PropAnalysis,
    PropsAnalysis, RenderTagCalleeMode, SnippetData,
};
pub use utils::IdentGen;
pub use scope::ComponentScoping;
pub use types::script::{
    DeclarationInfo, DeclarationKind, ExportInfo, PropInfo, PropsDeclaration, RuneKind, ScriptInfo,
};
pub use utils::{
    is_capture_event, is_delegatable_event, is_passive_event, is_simple_identifier,
    strip_capture_event,
};

use svelte_ast::Component;
use svelte_diagnostics::Diagnostic;
/// Run all analysis passes over a parsed component.
pub fn analyze<'a>(
    component: &Component,
    parsed: ParserResult<'a>,
) -> (AnalysisData, ParserResult<'a>, Vec<Diagnostic>) {
    analyze_with_options(component, parsed, false)
}

/// Analyze with compile options that affect analysis behavior.
pub fn analyze_with_options<'a>(
    component: &Component,
    mut parsed: ParserResult<'a>,
    custom_element: bool,
) -> (AnalysisData, ParserResult<'a>, Vec<Diagnostic>) {
    let mut diags = Vec::new();

    let mut data = AnalysisData::new_empty(component.node_count);
    data.custom_element = custom_element;

    // Classify render tags: unwrap ChainExpression → CallExpression, extract callee name
    passes::js_analyze::classify_render_tags(&mut parsed, component, &mut data);

    // Extract AwaitBlock binding metadata from parsed expressions
    passes::js_analyze::prepare_template_bindings(&mut parsed, component, &mut data);

    // Extract script info from pre-parsed Program AST
    let script_info = parsed.program.as_ref().and_then(|program| {
        let span = parsed.script_content_span?;
        let source = component.source_text(span);
        Some(utils::script_info::extract_script_info(
            program, span.start, source,
        ))
    });

    // JS analysis: enrich script info and extract OXC Scoping
    let script_scoping =
        script_info.and_then(|si| passes::js_analyze::analyze_script(&parsed, &mut data, si));
    data.scoping = ComponentScoping::new(script_scoping);

    // Extract expression info + classify shorthand/clsx/snippets/render_tag_args/CE config
    passes::js_analyze::extract_all_expressions(&parsed, component, &mut data);

    // Classify per-expression needs_context (import/prop member access, calls)
    // then aggregate into module-level flag for $.push/$.pop
    passes::js_analyze::classify_expression_needs_context(&mut data);
    if !data.needs_context {
        data.needs_context = data
            .expressions
            .values()
            .chain(data.attr_expressions.values())
            .any(|info| info.needs_context);
    }

    let scoping_built = scope::build_scoping(component, &mut data, &parsed.stmts);
    if let Some(ref program) = parsed.program {
        scope::mark_nested_runes(program, &mut data.scoping);
    }
    data.import_syms = data.scoping.collect_import_syms();
    // Combined walk: arrow scope registration + each-block index usage + reference resolution
    {
        let root = data.scoping.root_scope_id();
        let mut v1 = passes::js_metadata::JsMetadataVisitor {
            component,
            parsed: &parsed,
        };
        let mut v2 = passes::resolve_references::make_visitor(component, scoping_built);
        let mut ctx = walker::VisitContext::new(root, &mut data);
        walker::walk_template(
            &component.fragment,
            &mut ctx,
            &mut [&mut v1, &mut v2],
        );
    }
    passes::resolve_references::resolve_script_stores(&mut data);
    passes::post_resolve::run_post_resolve_passes(component, &mut data);
    resolve_render_tag_prop_sources(&mut data);
    resolve_render_tag_dynamic(&mut data);
    data.scoping.precompute_dynamic_cache();
    passes::js_analyze::classify_expression_dynamicity(&mut data);
    passes::lower::lower(component, &mut data);

    // Single composite walk: reactivity + element flags + hoistable snippets +
    // bind semantics + content classification + needs_var (bottom-up via leave_element)
    {
        let root = data.scoping.root_scope_id();
        let script_syms: rustc_hash::FxHashSet<crate::scope::SymbolId> = data
            .script
            .as_ref()
            .map(|s| {
                s.declarations
                    .iter()
                    .filter_map(|d| data.scoping.find_binding(root, &d.name))
                    .collect()
            })
            .unwrap_or_default();
        let top_level_snippet_ids: rustc_hash::FxHashSet<svelte_ast::NodeId> = component
            .fragment
            .nodes
            .iter()
            .filter_map(|n| {
                if let svelte_ast::Node::SnippetBlock(b) = n {
                    Some(b.id)
                } else {
                    None
                }
            })
            .collect();
        let mut v1 = passes::reactivity::ReactivityVisitor::new();
        let mut v2 = passes::element_flags::ElementFlagsVisitor::new(&component.source);
        let mut v3 = passes::hoistable::HoistableSnippetsVisitor::new(script_syms, top_level_snippet_ids);
        let mut v4 = passes::bind_semantics::BindSemanticsVisitor::new(&component.source);
        let mut v5 = passes::content_types::ContentAndVarVisitor {
            source: &component.source,
        };
        let mut ctx = walker::VisitContext::new(root, &mut data);
        walker::walk_template(
            &component.fragment,
            &mut ctx,
            &mut [&mut v1, &mut v2, &mut v3, &mut v4, &mut v5],
        );
    }

    // Classify non-element fragments (Root, IfConsequent, EachBody, etc.)
    // Element fragments already classified by ContentAndVarVisitor::leave_element
    passes::content_types::classify_remaining_fragments(&mut data, &component.source);
    validate::validate(component, &data, &mut diags);

    (data, parsed, diags)
}

/// Simplified analysis for standalone `.svelte.js`/`.svelte.ts` modules.
///
/// Only parses JS, builds scopes, and detects runes. No template, no props,
/// no fragment classification — modules are pure JS with rune transforms.
pub fn analyze_module(
    alloc: &oxc_allocator::Allocator,
    source: &str,
    is_ts: bool,
    dev: bool,
) -> (AnalysisData, Vec<Diagnostic>) {
    let _ = dev;
    let mut diags = Vec::new();

    let mut data = AnalysisData::new_empty(0);
    match svelte_parser::parse_module(alloc, source, is_ts) {
        Ok((program, scoping)) => {
            data.scoping = scope::ComponentScoping::new(Some(scoping));
            let script_info = utils::script_info::extract_script_info(&program, 0, source);
            for decl in &script_info.declarations {
                if let Some(rune_kind) = decl.is_rune {
                    let root = data.scoping.root_scope_id();
                    if let Some(sym_id) = data.scoping.find_binding(root, &decl.name) {
                        data.scoping.mark_rune(sym_id, rune_kind);
                    }
                }
            }
        }
        Err(errs) => diags.extend(errs),
    }

    (data, diags)
}

/// Resolve render tag argument identifiers to prop-source getter names.
/// Consumes `render_tag_arg_idents` (intermediate) and populates `render_tag_prop_sources`.
fn resolve_render_tag_prop_sources(data: &mut AnalysisData) {
    let root = data.scoping.root_scope_id();
    for (node_id, idents) in data.render_tag_arg_idents.drain() {
        let resolved = idents
            .into_iter()
            .map(|opt_name| {
                opt_name.and_then(|name| {
                    let sym = data.scoping.find_binding(root, &name)?;
                    data.scoping.is_prop_source(sym).then_some(sym)
                })
            })
            .collect();
        data.render_tag_prop_sources.insert(node_id, resolved);
    }
}

/// Compute `RenderTagCalleeMode` for each render tag.
/// Must run after `resolve_render_tag_prop_sources` (which runs after `props`).
fn resolve_render_tag_dynamic(data: &mut AnalysisData) {
    use crate::types::data::RenderTagCalleeMode;

    let all_ids: Vec<svelte_ast::NodeId> = data.render_tag_arg_has_call.keys().collect();

    for node_id in all_ids {
        let is_dynamic = match data.render_tag_callee_sym.get(node_id) {
            Some(&sym_id) => !data.scoping.is_normal_binding(sym_id),
            // No sym: non-Identifier callee or unresolved binding — always dynamic.
            None => true,
        };
        let is_chain = data.render_tag_is_chain.contains(&node_id);

        let mode = match (is_dynamic, is_chain) {
            (true, true) => RenderTagCalleeMode::DynamicChain,
            (true, false) => RenderTagCalleeMode::DynamicRegular,
            (false, true) => RenderTagCalleeMode::Chain,
            (false, false) => RenderTagCalleeMode::Direct,
        };
        data.render_tag_callee_mode.insert(node_id, mode);
    }
}

#[cfg(test)]
mod tests;
