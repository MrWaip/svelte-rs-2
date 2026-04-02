pub(crate) mod passes;
pub mod scope;
pub mod types;
pub(crate) mod utils;
mod validate;
pub(crate) mod walker;

pub use scope::ComponentScoping;
pub use types::data::{
    AnalysisData, AsyncStmtMeta, AwaitBindingData, AwaitBindingInfo, BlockerData,
    ClassDirectiveInfo, CodegenView, ComponentBindMode, ComponentPropInfo, ComponentPropKind,
    ConstTagData, ContentStrategy, DebugTagData, DestructureKind, ElementFlags, EventHandlerMode,
    ExprDeps, ExprHandle, ExprSite, ExpressionInfo, ExpressionKind, FragmentData, FragmentItem,
    FragmentKey, IgnoreData, LoweredFragment, LoweredTextPart, ParserResult, PropAnalysis,
    PropsAnalysis, RenderTagCalleeMode, RenderTagPlan, RuntimePlan, SnippetData, StmtHandle,
};
pub use types::script::{
    DeclarationInfo, DeclarationKind, ExportInfo, PropInfo, PropsDeclaration, RuneKind, ScriptInfo,
};
pub use utils::IdentGen;
pub use utils::{
    is_capture_event, is_delegatable_event, is_passive_event, is_simple_identifier,
    strip_capture_event,
};

use svelte_ast::Component;
use svelte_diagnostics::{Diagnostic, Severity};

fn run_template_bundle<'a, const N: usize>(
    component: &Component,
    data: &'a mut AnalysisData,
    source: &'a str,
    runes: bool,
    diags: &mut Vec<Diagnostic>,
    visitors: &mut [&mut dyn walker::TemplateVisitor; N],
) {
    let root = data.scoping.root_scope_id();
    let mut ctx = walker::VisitContext::new(root, data, &component.store, source, runes);
    walker::walk_template(&component.fragment, &mut ctx, visitors);
    diags.extend(ctx.take_warnings());
}

fn run_parsed_template_bundle<'a, const N: usize>(
    component: &Component,
    data: &'a mut AnalysisData,
    parsed: &'a ParserResult<'a>,
    source: &'a str,
    runes: bool,
    diags: &mut Vec<Diagnostic>,
    visitors: &mut [&mut dyn walker::TemplateVisitor; N],
) {
    let root = data.scoping.root_scope_id();
    let mut ctx =
        walker::VisitContext::with_parsed(root, data, &component.store, parsed, source, runes);
    walker::walk_template(&component.fragment, &mut ctx, visitors);
    diags.extend(ctx.take_warnings());
}

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
    let execution_order = passes::resolve_default_execution_order()
        .unwrap_or_else(|err| panic!("invalid analyze pass configuration: {err:?}"));
    for key in execution_order {
        match key {
            passes::PassKey::ClassifyRenderTags => {
                passes::js_analyze::classify_render_tags(
                    &mut parsed,
                    component,
                    &mut data,
                    source,
                    runes,
                );
            }
            passes::PassKey::AnalyzeScript => {
                let script_info = parsed.program.as_ref().and_then(|program| {
                    let span = parsed.script_content_span?;
                    let source = component.source_text(span);
                    Some(utils::script_info::extract_script_info(
                        program, span.start, source,
                    ))
                });
                let script_scoping = script_info
                    .and_then(|si| passes::js_analyze::analyze_script(&parsed, &mut data, si));
                data.scoping = ComponentScoping::new(script_scoping);
            }
            passes::PassKey::MarkRunes => {
                passes::mark_runes::mark_script_runes(&mut data);
                if let Some(program) = &parsed.program {
                    passes::mark_runes::mark_nested_runes(program, &mut data.scoping);
                    if options.dev {
                        data.ignore_data.scan_program_comments(program, runes);
                    }
                }
            }
            passes::PassKey::PrepareAwaitBindings => {
                let mut bundle = passes::bundles::AwaitBindingBundle::new();
                let mut visitors = bundle.visitors();
                run_parsed_template_bundle(
                    component,
                    &mut data,
                    &parsed,
                    source,
                    runes,
                    &mut diags,
                    &mut visitors,
                );
            }
            passes::PassKey::ExtractCeConfig => {
                if let Some(svelte_ast::CustomElementConfig::Expression(span)) = component
                    .options
                    .as_ref()
                    .and_then(|o| o.custom_element.as_ref())
                {
                    if let Some(expr) = parsed
                        .expr_handle(span.start)
                        .and_then(|handle| parsed.expr(handle))
                    {
                        let config =
                            utils::ce_config::extract_ce_config_from_expr(expr, span.start);
                        data.ce_config = Some(config);
                    }
                }
            }
            passes::PassKey::TemplateScoping => {
                passes::template_scoping::create_template_scopes(
                    component,
                    &mut data.scoping,
                    &parsed,
                );
                data.import_syms = data.scoping.collect_import_syms();
            }
            passes::PassKey::TemplateSemanticAndSideTables => {
                let mut bundle = passes::bundles::TemplateSemanticBundle::new(component);
                let mut visitors = bundle.visitors();
                run_parsed_template_bundle(
                    component,
                    &mut data,
                    &parsed,
                    source,
                    runes,
                    &mut diags,
                    &mut visitors,
                );
                data.scoping.build_template_scope_set();
            }
            passes::PassKey::CollectSymbols => {
                data.each_blocks.build_index_lookup();
                let mut bundle = passes::bundles::SymbolCollectionBundle::new(
                    crate::types::markers::ScopingBuilt::new(),
                );
                let mut visitors = bundle.visitors();
                run_parsed_template_bundle(
                    component,
                    &mut data,
                    &parsed,
                    source,
                    runes,
                    &mut diags,
                    &mut visitors,
                );
            }
            passes::PassKey::ResolveScriptStores => {
                passes::collect_symbols::resolve_script_stores(&mut data);
            }
            passes::PassKey::JsAnalyzePostTemplate => {
                passes::js_analyze::calculate_instance_blockers(&parsed, &mut data);
                passes::js_analyze::collect_script_rune_call_kinds(&parsed, &mut data);
                passes::js_analyze::classify_pickled_awaits(&parsed, &mut data);
            }
            passes::PassKey::ClassifyNeedsContext => {
                passes::js_analyze::classify_expression_needs_context(&mut data);
                if !data.needs_context {
                    data.needs_context = data
                        .expressions
                        .values()
                        .chain(data.attr_expressions.values())
                        .any(|info| info.needs_context);
                }
            }
            passes::PassKey::PostResolve => {
                passes::post_resolve::run_post_resolve_passes(&mut data);
                if !data.needs_context {
                    data.needs_context = data
                        .expressions
                        .values()
                        .chain(data.attr_expressions.values())
                        .any(|info| {
                            matches!(
                                info.kind,
                                crate::types::data::ExpressionKind::MemberExpression
                                    | crate::types::data::ExpressionKind::CallExpression { .. }
                            ) && info
                                .ref_symbols
                                .iter()
                                .any(|&sym| data.scoping.is_rest_prop(sym))
                        });
                }
            }
            passes::PassKey::ResolveRenderTagMeta => {
                resolve_render_tag_prop_sources(&mut data, &parsed);
                resolve_render_tag_dynamic(&mut data);
            }
            passes::PassKey::CollectConstTagFragments => {
                passes::lower::collect_const_tag_fragments(component, &mut data);
            }
            passes::PassKey::MarkConstTagBindings => {
                mark_const_tag_bindings(&mut data);
            }
            passes::PassKey::PrecomputeDynamicCache => {
                data.scoping.precompute_dynamic_cache();
            }
            passes::PassKey::MarkBlockedSymbolsDynamic => {
                if data.blocker_data.has_async {
                    data.scoping
                        .mark_blocked_symbols_dynamic(&data.blocker_data.symbol_blockers);
                }
            }
            passes::PassKey::ClassifyExpressionDynamicity => {
                passes::js_analyze::classify_expression_dynamicity(&mut data);
            }
            passes::PassKey::MarkBlockedExpressionsDynamic => {
                if data.blocker_data.has_async {
                    for info in data.expressions.values_mut() {
                        if !info.is_dynamic
                            && info
                                .ref_symbols
                                .iter()
                                .any(|sym| data.blocker_data.symbol_blockers.contains_key(sym))
                        {
                            info.is_dynamic = true;
                        }
                    }
                }
            }
            passes::PassKey::LowerTemplate => {
                passes::lower::lower(component, &mut data);
            }
            passes::PassKey::ReactivityWalk => {
                let mut bundle = passes::bundles::ReactivityBundle::new();
                let mut visitors = bundle.visitors();
                run_template_bundle(
                    component,
                    &mut data,
                    source,
                    runes,
                    &mut diags,
                    &mut visitors,
                );
            }
            passes::PassKey::TemplateClassificationWalk => {
                let mut bundle = passes::bundles::TemplateClassificationBundle::new(
                    component,
                    &data,
                    &component.source,
                );
                let mut visitors = bundle.visitors();
                run_template_bundle(
                    component,
                    &mut data,
                    source,
                    runes,
                    &mut diags,
                    &mut visitors,
                );
                bundle.finish(&mut data);
            }
            passes::PassKey::ClassifyRemainingFragments => {
                passes::content_types::classify_remaining_fragments(&mut data, &component.source);
            }
            passes::PassKey::ValidateTemplate => {
                let mut bundle = passes::bundles::TemplateValidationBundle::new();
                let mut visitors = bundle.visitors();
                run_parsed_template_bundle(
                    component,
                    &mut data,
                    &parsed,
                    &component.source,
                    runes,
                    &mut diags,
                    &mut visitors,
                );
            }
            passes::PassKey::Validate => {
                validate::validate(&data, &parsed, &mut diags);
            }
        }
    }

    // Apply warning filter if provided
    if let Some(ref filter) = options.warning_filter {
        diags.retain(|d| d.severity != Severity::Warning || filter(d));
    }

    data.runtime_plan = build_runtime_plan(&data, options.dev);

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
            validate::validate_program(&data, &program, 0, &mut diags);
        }
        Err(errs) => diags.extend(errs),
    }

    (data, diags)
}

/// Mark const tag bindings with RuneKind::Derived and const_alias.
/// Also populates `derived_deps` from the @const expression's `ref_symbols`
/// so that `is_dynamic_by_id` can determine dynamicity by following deps.
/// Scope is derived from const_tags.by_fragment + fragment_scopes.
fn mark_const_tag_bindings(data: &mut AnalysisData) {
    use types::script::RuneKind;
    let pairs: Vec<_> = data
        .const_tags
        .by_fragment
        .iter()
        .filter_map(|(frag_key, tag_ids)| {
            let scope = data.scoping.fragment_scope(frag_key)?;
            Some((scope, tag_ids.clone()))
        })
        .collect();
    for (scope, tag_ids) in pairs {
        for tag_id in tag_ids {
            let Some(names) = data.const_tags.names(tag_id).cloned() else {
                continue;
            };
            let is_destructured = names.len() > 1;
            let mut syms = Vec::new();
            // Get deps from the @const expression's ref_symbols
            let deps: Vec<_> = data
                .expressions
                .get(tag_id)
                .map(|info| info.ref_symbols.to_vec())
                .unwrap_or_default();
            for name in &names {
                if let Some(sym_id) = data.scoping.find_binding(scope, name) {
                    syms.push(sym_id);
                    data.scoping.mark_rune(sym_id, RuneKind::Derived);
                    data.scoping.set_derived_deps(sym_id, deps.clone());
                    if is_destructured {
                        data.scoping.mark_const_alias(sym_id, tag_id);
                    }
                }
            }
            if !syms.is_empty() {
                data.const_tags.syms.insert(tag_id, syms);
            }
        }
    }
}

/// Resolve render tag argument prop sources via reference_id from parsed expressions.
fn resolve_render_tag_prop_sources(data: &mut AnalysisData, parsed: &ParserResult<'_>) {
    use oxc_ast::ast::Expression;
    let tag_ids: Vec<svelte_ast::NodeId> = data.render_tag_plans.keys().collect();
    for tag_id in tag_ids {
        let handle = match data.template_semantics.node_expr_handles.get(tag_id) {
            Some(&handle) => handle,
            None => continue,
        };
        let resolved: Vec<Option<crate::scope::SymbolId>> = match parsed.expr(handle) {
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
        let Some(plan) = data.render_tag_plans.get_mut(tag_id) else {
            continue;
        };
        for (arg_plan, prop_source) in plan.arg_plans.iter_mut().zip(resolved) {
            arg_plan.prop_source = prop_source;
        }
    }
}

/// Compute `RenderTagCalleeMode` for each render tag.
/// Must run after `resolve_render_tag_prop_sources` (which runs after `props`).
fn resolve_render_tag_dynamic(data: &mut AnalysisData) {
    use crate::types::data::RenderTagCalleeMode;

    let all_ids: Vec<svelte_ast::NodeId> = data.render_tag_plans.keys().collect();

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
        if let Some(plan) = data.render_tag_plans.get_mut(node_id) {
            plan.callee_mode = mode;
        }
    }
}

fn build_runtime_plan(data: &AnalysisData, dev: bool) -> RuntimePlan {
    let has_exports = !data.exports.is_empty();
    let has_bindable = data.props.as_ref().is_some_and(|p| p.has_bindable);
    let has_stores = !data.scoping.store_symbol_ids().is_empty();
    let has_ce_props =
        data.custom_element && data.props.as_ref().is_some_and(|p| !p.props.is_empty());
    let needs_push = has_bindable || has_exports || has_ce_props || data.needs_context || dev;
    let has_component_exports = has_exports || has_ce_props || dev;
    let needs_props_param = data.props.is_some() || needs_push;

    RuntimePlan {
        needs_push,
        has_component_exports,
        has_exports,
        has_bindable,
        has_stores,
        has_ce_props,
        needs_props_param,
        needs_pop_with_return: needs_push && has_component_exports,
    }
}

#[cfg(test)]
mod tests;
