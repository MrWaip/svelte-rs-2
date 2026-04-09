use svelte_ast::Component;
use svelte_diagnostics::Diagnostic;

use crate::{validate, walker, AnalysisData, AnalyzeOptions, ParserResult};

use super::{bundles, collect_symbols, content_types, js_analyze, lower, mark_runes, post_resolve};

fn run_template_bundle<'a, const N: usize>(
    component: &Component,
    data: &'a mut AnalysisData,
    source: &'a str,
    runes: bool,
    options: &AnalyzeOptions,
    diags: &mut Vec<Diagnostic>,
    visitors: &mut [&mut dyn walker::TemplateVisitor; N],
) {
    let root = data.scoping.root_scope_id();
    let mut ctx = walker::VisitContext::new(
        root,
        data,
        &component.store,
        source,
        runes,
        &options.component_name,
        &options.filename_basename,
    );
    walker::walk_template(&component.fragment, &mut ctx, visitors);
    diags.extend(ctx.take_warnings());
}

fn run_parsed_template_bundle<'a, const N: usize>(
    component: &Component,
    data: &'a mut AnalysisData,
    parsed: &'a ParserResult<'a>,
    source: &'a str,
    runes: bool,
    options: &AnalyzeOptions,
    diags: &mut Vec<Diagnostic>,
    visitors: &mut [&mut dyn walker::TemplateVisitor; N],
) {
    let root = data.scoping.root_scope_id();
    let mut ctx = walker::VisitContext::with_parsed(
        root,
        data,
        &component.store,
        parsed,
        source,
        runes,
        &options.component_name,
        &options.filename_basename,
    );
    walker::walk_template(&component.fragment, &mut ctx, visitors);
    diags.extend(ctx.take_warnings());
}

pub(crate) fn execute_pass<'a>(
    key: super::PassKey,
    component: &Component,
    parsed: &mut ParserResult<'a>,
    data: &mut AnalysisData,
    options: &AnalyzeOptions,
    diags: &mut Vec<Diagnostic>,
) {
    let runes = options.runes;
    let source = &component.source;

    match key {
        super::PassKey::ClassifyRenderTags => {
            js_analyze::classify_render_tags(parsed, component, data, source, runes);
        }
        super::PassKey::AnalyzeScript => {
            let script_info = parsed.program.as_ref().and_then(|program| {
                let span = parsed.script_content_span?;
                let source = component.source_text(span);
                Some(crate::utils::script_info::extract_script_info(
                    program,
                    span.start,
                    source,
                    options.runes,
                ))
            });
            if let (Some(program), Some(script_info)) = (parsed.program.as_ref(), script_info) {
                js_analyze::analyze_script(data, script_info, program);
            }
        }
        super::PassKey::BuildComponentSemantics => {
            super::build_component_semantics::build(component, parsed, data);
        }
        super::PassKey::MarkRunes => {
            if runes {
                mark_runes::mark_script_runes(data);
                if let Some(module_program) = &parsed.module_program {
                    if let Some(span) = parsed.module_script_content_span {
                        let module_source = component.source_text(span);
                        let module_info = crate::utils::script_info::extract_script_info(
                            module_program,
                            span.start,
                            module_source,
                            true,
                        );
                        let module_scope = data
                            .scoping
                            .module_scope_id()
                            .unwrap_or_else(|| data.scoping.root_scope_id());
                        mark_runes::mark_root_script_runes_in_scope(
                            &mut data.scoping,
                            module_scope,
                            &module_info.declarations,
                            &crate::types::data::ProxyStateInits::new(),
                        );
                    }
                }
                if let Some(program) = &parsed.program {
                    mark_runes::mark_nested_runes(program, &mut data.scoping);
                }
                if let Some(module_program) = &parsed.module_program {
                    mark_runes::mark_nested_runes(module_program, &mut data.scoping);
                }
            }
            if let Some(program) = &parsed.program {
                if options.dev {
                    data.output.ignore_data.scan_program_comments(program, runes);
                }
            }
        }
        super::PassKey::PrepareAwaitBindings => {
            let mut bundle = bundles::AwaitBindingBundle::new();
            let mut visitors = bundle.visitors();
            run_parsed_template_bundle(
                component,
                data,
                parsed,
                source,
                runes,
                options,
                diags,
                &mut visitors,
            );
        }
        super::PassKey::ExtractCeConfig => {
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
                        crate::utils::ce_config::extract_ce_config_from_expr(expr, span.start);
                    data.script.ce_config = Some(config);
                }
            }
        }
        super::PassKey::TemplateSideTables => {
            super::template_side_tables::collect_fragment_facts(component, data);
            super::template_side_tables::collect_rich_content_facts(component, data);
            let mut bundle = bundles::TemplateSideTablesBundle::new(component);
            let mut visitors = bundle.visitors();
            run_parsed_template_bundle(
                component,
                data,
                parsed,
                source,
                runes,
                options,
                diags,
                &mut visitors,
            );
            data.template.template_elements.finalize();
        }
        super::PassKey::CollectSymbols => {
            let mut bundle =
                bundles::SymbolCollectionBundle::new(crate::types::markers::ScopingBuilt::new());
            let mut visitors = bundle.visitors();
            run_parsed_template_bundle(
                component,
                data,
                parsed,
                source,
                runes,
                options,
                diags,
                &mut visitors,
            );
        }
        super::PassKey::ResolveScriptStores => {
            collect_symbols::resolve_script_stores(data);
        }
        super::PassKey::JsAnalyzePostTemplate => {
            js_analyze::calculate_instance_blockers(parsed, data);
            if runes {
                js_analyze::collect_script_rune_call_kinds(parsed, data);
            }
            js_analyze::classify_pickled_awaits(parsed, data);
        }
        super::PassKey::ClassifyNeedsContext => {
            js_analyze::classify_expression_needs_context(data);
            if !data.output.needs_context {
                data.output.needs_context = data
                    .expressions
                    .values()
                    .chain(data.attr_expressions.values())
                    .any(|info| info.needs_context);
            }
        }
        super::PassKey::PostResolve => {
            post_resolve::run_post_resolve_passes(data);
            if !data.output.needs_context {
                data.output.needs_context = data
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
        super::PassKey::ResolveRenderTagMeta => {
            resolve_render_tag_prop_sources(data, parsed);
            resolve_render_tag_dynamic(data);
        }
        super::PassKey::CollectConstTagFragments => {
            lower::collect_const_tag_fragments(component, data);
        }
        super::PassKey::MarkConstTagBindings => {
            mark_const_tag_bindings(data);
        }
        super::PassKey::PrecomputeDynamicCache => {
            data.scoping.precompute_dynamic_cache();
        }
        super::PassKey::MarkBlockedSymbolsDynamic => {
            if data.script.blocker_data.has_async {
                data.scoping
                    .mark_blocked_symbols_dynamic(&data.script.blocker_data.symbol_blockers);
            }
        }
        super::PassKey::ClassifyExpressionDynamicity => {
            js_analyze::classify_expression_dynamicity(data);
        }
        super::PassKey::MarkBlockedExpressionsDynamic => {
            if data.script.blocker_data.has_async {
                for info in data.expressions.values_mut() {
                    if !info.is_dynamic
                        && info
                            .ref_symbols
                            .iter()
                            .any(|sym| data.script.blocker_data.symbol_blockers.contains_key(sym))
                    {
                        info.is_dynamic = true;
                    }
                }
            }
        }
        super::PassKey::LowerTemplate => {
            lower::lower(component, data);
        }
        super::PassKey::ReactivityWalk => {
            let mut bundle = bundles::ReactivityBundle::new();
            let mut visitors = bundle.visitors();
            run_template_bundle(
                component,
                data,
                source,
                runes,
                options,
                diags,
                &mut visitors,
            );
        }
        super::PassKey::TemplateClassificationWalk => {
            let mut bundle = bundles::TemplateClassificationBundle::new(component, data, source);
            let mut visitors = bundle.visitors();
            run_template_bundle(
                component,
                data,
                source,
                runes,
                options,
                diags,
                &mut visitors,
            );
            bundle.finish(data);
        }
        super::PassKey::ClassifyRemainingFragments => {
            content_types::classify_remaining_fragments(data, source);
        }
        super::PassKey::ValidateTemplate => {
            let mut bundle = bundles::TemplateValidationBundle::new();
            let mut visitors = bundle.visitors();
            run_parsed_template_bundle(
                component,
                data,
                parsed,
                source,
                runes,
                options,
                diags,
                &mut visitors,
            );
        }
        super::PassKey::Validate => {
            validate::validate(component, data, parsed, runes, diags);
        }
    }
}

fn mark_const_tag_bindings(data: &mut AnalysisData) {
    use crate::types::script::RuneKind;
    let pairs: Vec<_> = data
        .template
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
            let Some(names) = data.template.const_tags.names(tag_id).cloned() else {
                continue;
            };
            let is_destructured = names.len() > 1;
            let mut syms = Vec::new();
            let deps: Vec<_> = data
                .expressions
                .get(tag_id)
                .map(|info| info.ref_symbols.to_vec())
                .unwrap_or_default();
            for name in &names {
                if let Some(sym_id) = data.scoping.find_binding(scope, name) {
                    syms.push(sym_id);
                    data.scoping.mark_template_declaration(sym_id);
                    data.scoping.mark_rune(sym_id, RuneKind::Derived);
                    data.scoping.set_derived_deps(sym_id, deps.clone());
                    if is_destructured {
                        data.scoping.mark_const_alias(sym_id, tag_id);
                    }
                }
            }
            if !syms.is_empty() {
                data.template.const_tags.syms.insert(tag_id, syms);
            }
        }
    }
}

fn resolve_render_tag_prop_sources(data: &mut AnalysisData, parsed: &ParserResult<'_>) {
    use oxc_ast::ast::Expression;
    let tag_ids: Vec<svelte_ast::NodeId> = data.blocks.render_tag_plans.keys().collect();
    for tag_id in tag_ids {
        let handle = match data.template.template_semantics.node_expr_handles.get(tag_id) {
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
        let Some(plan) = data.blocks.render_tag_plans.get_mut(tag_id) else {
            continue;
        };
        for (arg_plan, prop_source) in plan.arg_plans.iter_mut().zip(resolved) {
            arg_plan.prop_source = prop_source;
        }
    }
}

fn resolve_render_tag_dynamic(data: &mut AnalysisData) {
    use crate::types::data::RenderTagCalleeMode;

    let all_ids: Vec<svelte_ast::NodeId> = data.blocks.render_tag_plans.keys().collect();

    for node_id in all_ids {
        let is_dynamic = match data.blocks.render_tag_callee_sym.get(node_id) {
            Some(&sym_id) => !data.scoping.is_normal_binding(sym_id),
            None => true,
        };
        let is_chain = data.blocks.render_tag_is_chain.contains(&node_id);

        let mode = match (is_dynamic, is_chain) {
            (true, true) => RenderTagCalleeMode::DynamicChain,
            (true, false) => RenderTagCalleeMode::DynamicRegular,
            (false, true) => RenderTagCalleeMode::Chain,
            (false, false) => RenderTagCalleeMode::Direct,
        };
        if let Some(plan) = data.blocks.render_tag_plans.get_mut(node_id) {
            plan.callee_mode = mode;
        }
    }
}
