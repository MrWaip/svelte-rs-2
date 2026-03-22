mod bind_semantics;
mod ce_config;
mod content_types;
mod markers;
mod data;
mod element_flags;
mod elseif;
mod hoistable;
pub mod ident_gen;
pub(crate) mod js_analyze;
mod lower;
mod needs_var;
mod analyze_semantic;
mod post_resolve;
mod reactivity;
mod resolve_references;
pub mod scope;
pub mod script_types;
pub(crate) mod script_info;
mod store_subscriptions;
pub mod utils;
mod validate;
pub(crate) mod walker;

pub use data::{
    AnalysisData, AwaitBindingData, ClassDirectiveInfo, ComponentPropInfo, ComponentPropKind,
    EventHandlerMode, ExpressionInfo, ExpressionKind, LoweredTextPart, ConstTagData, ContentStrategy,
    DebugTagData, ElementFlags, FragmentData, FragmentItem, FragmentKey, LoweredFragment, ParsedExprs,
    PropAnalysis, PropsAnalysis, RenderTagCalleeMode, SnippetData,
};
pub use ident_gen::IdentGen;
pub use scope::ComponentScoping;
pub use script_types::{ScriptInfo, DeclarationInfo, DeclarationKind, RuneKind, PropInfo, PropsDeclaration, ExportInfo};
pub use utils::{is_delegatable_event, is_capture_event, strip_capture_event, is_passive_event, is_simple_identifier};

use svelte_ast::Component;
use svelte_diagnostics::Diagnostic;
use svelte_parser::JsParseResult;

/// Run all analysis passes over a parsed component.
pub fn analyze<'a>(
    component: &Component,
    js_result: JsParseResult<'a>,
) -> (AnalysisData, ParsedExprs<'a>, Vec<Diagnostic>) {
    analyze_with_options(component, js_result, false)
}

/// Analyze with compile options that affect analysis behavior.
pub fn analyze_with_options<'a>(
    component: &Component,
    js_result: JsParseResult<'a>,
    custom_element: bool,
) -> (AnalysisData, ParsedExprs<'a>, Vec<Diagnostic>) {
    let mut diags = Vec::new();

    let mut data = AnalysisData::new_empty();
    data.custom_element = custom_element;

    let script_content_span = js_result.script_content_span;
    let typescript = js_result.typescript;
    let mut parsed = js_result.parsed;

    // Classify render tags: unwrap ChainExpression → CallExpression, extract callee name
    js_analyze::classify_render_tags(&mut parsed, component, &mut data);

    // Extract script info from pre-parsed Program AST
    let script_info = parsed.program.as_ref().and_then(|program| {
        let span = script_content_span?;
        let source = component.source_text(span);
        Some(script_info::extract_script_info(program, span.start, source))
    });

    // JS analysis: enrich script info and extract OXC Scoping
    let script_scoping = script_info
        .and_then(|si| js_analyze::analyze_script(&parsed, &mut data, si));
    data.scoping = ComponentScoping::new(script_scoping);

    // Extract expression info + classify shorthand/clsx/snippets/const_tags/await_bindings/render_tag_args/CE config
    js_analyze::extract_all_expressions(&parsed, component, &mut data, typescript);

    let scoping_built = scope::build_scoping(component, &mut data);
    analyze_semantic::compute_js_metadata(component, &mut data, &parsed);
    data.import_syms = data.scoping.collect_import_syms();
    resolve_references::resolve_references(component, &mut data, scoping_built);
    post_resolve::run_post_resolve_passes(component, &mut data);
    resolve_render_tag_prop_sources(&mut data);
    resolve_render_tag_dynamic(&mut data);
    data.scoping.precompute_dynamic_cache();
    lower::lower(component, &mut data);

    // Single composite walk: reactivity + elseif + element flags + hoistable snippets
    {
        let root = data.scoping.root_scope_id();
        let script_syms: rustc_hash::FxHashSet<crate::scope::SymbolId> = data
            .script
            .as_ref()
            .map(|s| {
                s.declarations.iter()
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
        let mut visitor = (
            reactivity::ReactivityVisitor,
            elseif::ElseifVisitor,
            element_flags::ElementFlagsVisitor::new(&component.source),
            hoistable::HoistableSnippetsVisitor::new(script_syms, top_level_snippet_ids),
            bind_semantics::BindSemanticsVisitor::new(&component.source),
        );
        walker::walk_template(&component.fragment, &mut data, root, &mut visitor);
    }

    // Classify fragments + mark which have dynamic children (single pass over lowered_fragments)
    debug_assert!(
        !data.fragments.lowered.is_empty() || component.fragment.nodes.is_empty(),
        "classify_and_mark_dynamic requires lowered_fragments (from lower pass)"
    );
    content_types::classify_and_mark_dynamic(&mut data);
    // Separate walker pass: needs_var depends on content_types (computed above)
    debug_assert!(
        !data.fragments.content_types.is_empty() || component.fragment.nodes.is_empty(),
        "needs_var requires content_types (from classify_and_mark_dynamic)"
    );
    {
        let root = data.scoping.root_scope_id();
        let mut visitor = needs_var::NeedsVarVisitor;
        walker::walk_template(&component.fragment, &mut data, root, &mut visitor);
    }
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

    let mut data = AnalysisData::new_empty();
    match svelte_parser::parse_module(alloc, source, is_ts) {
        Ok((program, scoping)) => {
            data.scoping = scope::ComponentScoping::new(Some(scoping));
            let script_info = script_info::extract_script_info(&program, 0, source);
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
        let resolved = idents.into_iter().map(|opt_name| {
            opt_name.and_then(|name| {
                let sym = data.scoping.find_binding(root, &name)?;
                data.scoping.is_prop_source(sym).then_some(sym)
            })
        }).collect();
        data.render_tag_prop_sources.insert(node_id, resolved);
    }
}

/// Compute `RenderTagCalleeMode` for each render tag.
/// Must run after `resolve_render_tag_prop_sources` (which runs after `props`).
fn resolve_render_tag_dynamic(data: &mut AnalysisData) {
    use crate::data::RenderTagCalleeMode;

    let all_ids: Vec<svelte_ast::NodeId> = data.render_tag_arg_has_call.keys().copied().collect();

    for node_id in all_ids {
        let is_dynamic = match data.render_tag_callee_sym.get(&node_id) {
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
