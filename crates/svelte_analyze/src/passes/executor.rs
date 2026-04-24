use svelte_ast::Component;
use svelte_diagnostics::Diagnostic;

use crate::{validate, walker, AnalysisData, AnalyzeOptions, ParserResult};

use super::{
    bundles, const_tag_fragments, finalize_component_name, fragment_topology, html_tag_ns_flags,
    js_analyze, post_resolve,
};

fn run_template_bundle<'d, 'a, const N: usize>(
    component: &'d Component,
    data: &'d mut AnalysisData<'a>,
    source: &'d str,
    runes: bool,
    options: &AnalyzeOptions,
    diags: &mut Vec<Diagnostic>,
    visitors: &mut [&mut dyn walker::TemplateVisitor; N],
) {
    let root = data.scoping.root_scope_id();
    let component_name = data.output.component_name.clone();
    let mut ctx = walker::VisitContext::new(
        root,
        data,
        &component.store,
        source,
        runes,
        &component_name,
        &options.filename_basename,
    );
    walker::walk_template(&component.fragment, &mut ctx, visitors);
    diags.extend(ctx.take_warnings());
}

fn run_parsed_template_bundle<'d, 'a, const N: usize>(
    component: &'d Component,
    data: &'d mut AnalysisData<'a>,
    parsed: &'d ParserResult<'a>,
    source: &'d str,
    runes: bool,
    options: &AnalyzeOptions,
    diags: &mut Vec<Diagnostic>,
    visitors: &mut [&mut dyn walker::TemplateVisitor; N],
) {
    let root = data.scoping.root_scope_id();
    let component_name = data.output.component_name.clone();
    let mut ctx = walker::VisitContext::with_parsed(
        root,
        data,
        &component.store,
        parsed,
        source,
        runes,
        &component_name,
        &options.filename_basename,
    );
    walker::walk_template(&component.fragment, &mut ctx, visitors);
    diags.extend(ctx.take_warnings());
}

pub(crate) fn execute_pass<'a>(
    key: super::PassKey,
    component: &Component,
    parsed: &mut ParserResult<'a>,
    data: &mut AnalysisData<'a>,
    options: &AnalyzeOptions,
    diags: &mut Vec<Diagnostic>,
) {
    let runes = options.runes;
    let source = &component.source;

    match key {
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
        super::PassKey::FinalizeComponentName => {
            finalize_component_name::run(data);
        }
        super::PassKey::ScanIgnoreComments => {
            if let Some(program) = &parsed.program {
                if options.dev {
                    data.output
                        .ignore_data
                        .scan_program_comments(program, runes);
                }
            }
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
                    .any(|info| info.is_dynamic_with_context_role());
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
                        info.has_context_sensitive_shape()
                            && info
                                .ref_symbols()
                                .iter()
                                // NOTE: runs inside the `PostResolve` pass, BEFORE
                                // `BuildReactivitySemantics` populates v2 prop facts.
                                // Uses `ComponentScoping.is_rest_prop` because
                                // `post_resolve::handle_props_declaration` (same pass)
                                // already marked the rest-prop symbol.
                                .any(|&sym| data.scoping.is_rest_prop(sym))
                    });
            }
        }
        super::PassKey::CollectConstTagFragments => {
            const_tag_fragments::collect(component, data);
        }
        super::PassKey::BuildReactivitySemantics => {
            crate::reactivity_semantics::build_v2(component, parsed, data);
        }
        super::PassKey::BuildBlockSemantics => {
            data.block_semantics_store = crate::block_semantics::build(
                component,
                parsed,
                data.scoping.semantics(),
                &data.reactivity,
                &data.script.blocker_data,
                component.node_count(),
            );
        }
        super::PassKey::BuildFragmentTopology => {
            fragment_topology::build(component, data);
        }
        super::PassKey::CollectHtmlTagNsFlags => {
            html_tag_ns_flags::collect(component, data);
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
            super::dynamism::populate_expr_roles(data);
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
