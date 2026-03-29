pub(crate) mod passes;
pub mod scope;
pub mod types;
pub(crate) mod utils;
mod validate;
pub(crate) mod walker;

pub use types::data::{
    AnalysisData, AsyncStmtMeta, AwaitBindingData, AwaitBindingInfo, BlockerData, IgnoreData,
    ClassDirectiveInfo, ComponentBindMode, ComponentPropInfo, ComponentPropKind, ConstTagData,
    ContentStrategy, DebugTagData, DestructureKind, ElementFlags, EventHandlerMode, ExpressionInfo,
    ExpressionKind, FragmentData, FragmentItem, FragmentKey, LoweredFragment, LoweredTextPart,
    ParserResult, PropAnalysis, PropsAnalysis, RenderTagCalleeMode, SnippetData,
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
use svelte_diagnostics::{Diagnostic, Severity};

/// Options controlling analysis behavior.
pub struct AnalyzeOptions {
    pub custom_element: bool,
    pub runes: bool,
    pub dev: bool,
    pub warning_filter: Option<Box<dyn Fn(&Diagnostic) -> bool>>,
}

impl Default for AnalyzeOptions {
    fn default() -> Self {
        Self {
            custom_element: false,
            runes: true,
            dev: false,
            warning_filter: None,
        }
    }
}

/// Run all analysis passes over a parsed component (default options).
pub fn analyze<'a>(
    component: &Component,
    parsed: ParserResult<'a>,
) -> (AnalysisData, ParserResult<'a>, Vec<Diagnostic>) {
    analyze_with_options(component, parsed, &AnalyzeOptions::default())
}

/// Analyze with compile options that affect analysis behavior.
pub fn analyze_with_options<'a>(
    component: &Component,
    mut parsed: ParserResult<'a>,
    options: &AnalyzeOptions,
) -> (AnalysisData, ParserResult<'a>, Vec<Diagnostic>) {
    let mut diags = Vec::new();
    let runes = options.runes;
    let source = &component.source;

    let mut data = AnalysisData::new_empty(component.node_count());
    data.custom_element = options.custom_element;

    // Classify render tags: unwrap ChainExpression → CallExpression, extract callee name
    passes::js_analyze::classify_render_tags(&mut parsed, component, &mut data, source, runes);

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

    // Mark runes declared at root scope (from ScriptInfo declarations)
    passes::mark_runes::mark_script_runes(&mut data);
    // Mark runes in nested function scopes ($derived/$state inside closures etc.)
    if let Some(program) = &parsed.program {
        passes::mark_runes::mark_nested_runes(program, &mut data.scoping);
    }

    // Await binding metadata (independent of expression analysis)
    {
        let root = data.scoping.root_scope_id();
        let mut v1 = passes::js_analyze::BindingPreparer;
        let mut ctx = walker::VisitContext::with_parsed(root, &mut data, &component.store, &parsed, source, runes);
        walker::walk_template(&component.fragment, &mut ctx, &mut [&mut v1]);
        diags.extend(ctx.take_warnings());
    }
    // CE config (not template-related, extracted separately)
    if let Some(svelte_ast::CustomElementConfig::Expression(span)) = component
        .options
        .as_ref()
        .and_then(|o| o.custom_element.as_ref())
    {
        if let Some(expr) = parsed.exprs.get(&span.start) {
            let config = utils::ce_config::extract_ce_config_from_expr(expr, span.start);
            data.ce_config = Some(config);
        }
    }

    passes::template_scoping::create_template_scopes(component, &mut data.scoping, &parsed);
    // TODO: build_scoping pass removed — reimplement template bindings, side tables
    let scoping_built = crate::types::markers::ScopingBuilt::new();
    data.import_syms = data.scoping.collect_import_syms();

    // Mini-SemanticBuilder + side tables: scopes, bindings, OXC references,
    // each/snippet/const marks and metadata — all in one walk.
    {
        let root = data.scoping.root_scope_id();
        let mut v1 = passes::template_semantic::TemplateSemanticVisitor;
        let mut v2 = passes::template_side_tables::TemplateSideTablesVisitor { component };
        let mut ctx = walker::VisitContext::with_parsed(root, &mut data, &component.store, &parsed, source, runes);
        walker::walk_template(
            &component.fragment,
            &mut ctx,
            &mut [&mut v1, &mut v2],
        );
        diags.extend(ctx.take_warnings());
    }
    data.scoping.build_template_scope_set();

    // Collect ref_symbols from OXC references + store detection + index usage detection
    data.each_blocks.build_index_lookup();
    {
        let root = data.scoping.root_scope_id();
        let mut v2 = passes::collect_symbols::make_visitor(scoping_built);
        let mut ctx = walker::VisitContext::with_parsed(root, &mut data, &component.store, &parsed, source, runes);
        walker::walk_template(
            &component.fragment,
            &mut ctx,
            &mut [&mut v2],
        );
        diags.extend(ctx.take_warnings());
    }
    passes::collect_symbols::resolve_script_stores(&mut data);

    // Instance body blocker analysis (experimental.async)
    passes::js_analyze::calculate_instance_blockers(&parsed, &mut data);

    // needs_context requires ref_symbols — must run after collect_symbols
    passes::js_analyze::classify_expression_needs_context(&mut data);
    if !data.needs_context {
        data.needs_context = data
            .expressions
            .values()
            .chain(data.attr_expressions.values())
            .any(|info| info.needs_context);
    }

    passes::post_resolve::run_post_resolve_passes(&mut data);
    resolve_render_tag_prop_sources(&mut data, &parsed);
    resolve_render_tag_dynamic(&mut data);
    data.scoping.precompute_dynamic_cache();
    passes::js_analyze::classify_expression_dynamicity(&mut data);

    // Mark expressions referencing blocked symbols as dynamic (after classify_expression_dynamicity)
    if data.blocker_data.has_async {
        for info in data.expressions.values_mut() {
            if !info.is_dynamic && info.ref_symbols.iter().any(|sym| data.blocker_data.symbol_blockers.contains_key(sym)) {
                info.is_dynamic = true;
            }
        }
    }

    passes::lower::lower(component, &mut data);
    // by_fragment populated by lower — now we can mark const_alias
    mark_const_tag_bindings(&mut data);

    // Walk 1: Reactivity — produces dynamic_nodes, dynamic_attrs, needs_ref.
    // Must complete before Walk 2 because ElementFlagsVisitor and
    // ContentAndVarVisitor read these outputs.
    {
        let root = data.scoping.root_scope_id();
        let mut v1 = passes::reactivity::ReactivityVisitor::new();
        let mut ctx = walker::VisitContext::new(root, &mut data, &component.store, source, runes);
        walker::walk_template(
            &component.fragment,
            &mut ctx,
            &mut [&mut v1],
        );
        diags.extend(ctx.take_warnings());
    }

    // Walk 2: Element flags + hoistable snippets + bind semantics +
    // content classification + needs_var (bottom-up via leave_element).
    // Depends on dynamic_attrs/dynamic_nodes/needs_ref from Walk 1.
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
            .filter_map(|&id| {
                if let svelte_ast::Node::SnippetBlock(b) = component.store.get(id) {
                    Some(b.id)
                } else {
                    None
                }
            })
            .collect();
        let mut v2 = passes::element_flags::ElementFlagsVisitor::new(&component.source);
        let mut v3 = passes::hoistable::HoistableSnippetsVisitor::new(script_syms, top_level_snippet_ids);
        let mut v4 = passes::bind_semantics::BindSemanticsVisitor::new(&component.source);
        let mut v5 = passes::content_types::ContentAndVarVisitor {
            source: &component.source,
        };
        let mut ctx = walker::VisitContext::new(root, &mut data, &component.store, source, runes);
        walker::walk_template(
            &component.fragment,
            &mut ctx,
            &mut [&mut v2, &mut v3, &mut v4, &mut v5],
        );
        diags.extend(ctx.take_warnings());
        v3.finish(&mut data);
    }

    // Classify non-element fragments (Root, IfConsequent, EachBody, etc.)
    // Element fragments already classified by ContentAndVarVisitor::leave_element
    passes::content_types::classify_remaining_fragments(&mut data, &component.source);
    validate::validate(&parsed, &mut diags);

    // Apply warning filter if provided
    if let Some(ref filter) = options.warning_filter {
        diags.retain(|d| d.severity != Severity::Warning || filter(d));
    }

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
            data.script = Some(script_info);
            passes::mark_runes::mark_script_runes(&mut data);
        }
        Err(errs) => diags.extend(errs),
    }

    (data, diags)
}

/// Mark const tag bindings with RuneKind::Derived and const_alias after
/// JsMetadataVisitor has created the actual SymbolIds.
/// Scope is derived from const_tags.by_fragment + fragment_scopes.
fn mark_const_tag_bindings(data: &mut AnalysisData) {
    use types::script::RuneKind;
    let pairs: Vec<_> = data.const_tags.by_fragment.iter()
        .filter_map(|(frag_key, tag_ids)| {
            let scope = data.scoping.fragment_scope(frag_key)?;
            Some((scope, tag_ids.clone()))
        })
        .collect();
    for (scope, tag_ids) in pairs {
        for tag_id in tag_ids {
            let Some(names) = data.const_tags.names(tag_id).cloned() else { continue };
            let is_destructured = names.len() > 1;
            for name in &names {
                if let Some(sym_id) = data.scoping.find_binding(scope, name) {
                    data.scoping.mark_rune(sym_id, RuneKind::Derived);
                    if is_destructured {
                        data.scoping.mark_const_alias(sym_id, tag_id);
                    }
                }
            }
        }
    }
}

/// Resolve render tag argument prop sources via reference_id from parsed expressions.
fn resolve_render_tag_prop_sources(data: &mut AnalysisData, parsed: &ParserResult<'_>) {
    use oxc_ast::ast::Expression;
    let tag_ids: Vec<svelte_ast::NodeId> = data.render_tag_arg_has_call.keys().collect();
    for tag_id in tag_ids {
        let offset = match data.node_expr_offsets.get(tag_id) {
            Some(&o) => o,
            None => continue,
        };
        let resolved = match parsed.exprs.get(&offset) {
            Some(Expression::CallExpression(call)) => call
                .arguments
                .iter()
                .map(|arg| {
                    if let Expression::Identifier(ident) = arg.to_expression() {
                        ident
                            .reference_id
                            .get()
                            .and_then(|ref_id| data.scoping.get_reference(ref_id).symbol_id())
                            .filter(|&sym| data.scoping.is_prop_source(sym))
                    } else {
                        None
                    }
                })
                .collect(),
            _ => continue,
        };
        data.render_tag_prop_sources.insert(tag_id, resolved);
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
