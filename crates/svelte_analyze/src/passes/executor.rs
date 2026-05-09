use svelte_ast::Component;
use svelte_diagnostics::Diagnostic;

use crate::{AnalysisData, AnalyzeOptions, JsAst, validate, walker};

use super::{bundles, finalize_component_name, fragment_topology, js_analyze, post_resolve};

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
    walker::walk_template(component.root, &mut ctx, visitors);
    diags.extend(ctx.take_warnings());
}

fn run_parsed_template_bundle<'d, 'a, const N: usize>(
    component: &'d Component,
    data: &'d mut AnalysisData<'a>,
    parsed: &'d JsAst<'a>,
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
    walker::walk_template(component.root, &mut ctx, visitors);
    diags.extend(ctx.take_warnings());
}

pub(crate) fn execute_pass<'a>(
    key: super::PassKey,
    component: &Component,
    parsed: &mut JsAst<'a>,
    data: &mut AnalysisData<'a>,
    options: &AnalyzeOptions,
    diags: &mut Vec<Diagnostic>,
) {
    let runes = data.script.runes();
    let source = &component.source;

    match key {
        super::PassKey::AnalyzeScript => {
            let script_info = parsed.program.as_ref().and_then(|program| {
                parsed.script_content_span?;
                Some(crate::utils::script_info::extract_script_info(
                    program,
                    &component.source,
                    runes,
                ))
            });
            if let (Some(program), Some(script_info)) = (parsed.program.as_ref(), script_info) {
                js_analyze::analyze_script(data, script_info, program);
            }
        }
        super::PassKey::BuildComponentSemantics => {
            super::build_component_semantics::build(component, parsed, data);
        }
        super::PassKey::EnrichScriptInfo => {
            super::enrich_script_info::run(component, parsed, data);
        }
        super::PassKey::FinalizeComponentName => {
            finalize_component_name::run(data);
        }
        super::PassKey::ScanIgnoreComments => {
            if let Some(program) = &parsed.program
                && options.dev
            {
                data.output
                    .ignore_data
                    .scan_program_comments(program, &component.source, runes);
            }
        }
        super::PassKey::ExtractCeConfig => {
            if let Some(svelte_ast::CustomElementConfig::Expression(span)) = component
                .options
                .as_ref()
                .and_then(|o| o.custom_element.as_ref())
                && let Some(expr) = parsed.pending_expr(span.start)
            {
                let config = crate::utils::ce_config::extract_ce_config_from_expr(expr, span.start);
                data.script.ce_config = Some(config);
            }
        }
        super::PassKey::TemplateSideTables => {
            super::template_side_tables::collect_fragment_facts(component, data);
            super::template_side_tables::collect_rich_content_facts(component, data);
            let mut bundle = bundles::TemplateSideTablesBundle::new(component);
            {
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
            data.template.template_elements.finalize();
            super::template_side_tables::collect_fragment_namespaces(component, data);

            for (idx, slot) in bundle.take_title_buckets().into_iter().enumerate() {
                if let Some(ids) = slot {
                    data.template
                        .title_elements
                        .by_fragment
                        .insert(svelte_ast::FragmentId(idx as u32), ids);
                }
            }
            data.template.expression_tags_by_fragment = bundle.take_expression_tag_buckets();
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
            js_analyze::classify_pickled_awaits(parsed, data);
        }
        super::PassKey::ClassifyNeedsContext => {
            let _ = data;
        }
        super::PassKey::PostResolve => {
            post_resolve::run_post_resolve_passes(data);
        }
        super::PassKey::BuildReactivitySemantics => {
            crate::reactivity_semantics::build_v2(
                component,
                parsed,
                data,
                crate::reactivity_semantics::ReactivityInputs {
                    inline_runes: options.inline_runes,
                    compile_runes: options.runes,
                    immutable: options.immutable,
                    accessors: options.accessors,
                },
            );
        }
        super::PassKey::BuildBlockSemantics => {
            data.block_semantics_store = crate::block_semantics::build(
                component,
                parsed,
                data.scoping.semantics(),
                &data.reactivity,
                &data.script.blocker_data,
                &data.template.fragment_namespaces,
                &data.output.ignore_data,
                data.script.dev,
                component.node_count(),
            );
        }
        super::PassKey::BuildFragmentTopology => {
            fragment_topology::build(component, data);
        }
        super::PassKey::ReactivityWalk => {
            let mut bundle = bundles::ReactivityBundle::new();
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
