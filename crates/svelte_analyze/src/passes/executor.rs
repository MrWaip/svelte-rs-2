use svelte_ast::Component;
use svelte_diagnostics::Diagnostic;

use crate::reactivity_semantics::{ReactivityInputs, build_v2, finalize_reactivity};
use crate::types::markers::ScopingBuilt;
use crate::utils::ce_config;
use crate::{AnalysisData, AnalyzeOptions, JsAst, validate, value_evaluation, walker};
use crate::{attribute_semantics, block_semantics, expression_semantics};

use super::{bundles, finalize_component_name, fragment_topology, js_analyze};

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
            if let Some(program) = parsed.program.as_ref()
                && parsed.script_content_span.is_some()
            {
                js_analyze::analyze_script(data, program);
            }
            if let Some(module_program) = parsed.module_program.as_ref()
                && parsed.module_script_content_span.is_some()
            {
                data.output.needs_context |= js_analyze::needs_context_for_program(
                    module_program,
                    &data.scoping,
                    &data.reactivity,
                );
            }
        }
        super::PassKey::BuildComponentSemantics => {
            super::build_component_semantics::build(component, parsed, data);
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
                let config = ce_config::extract_ce_config_from_expr(expr, span.start);
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
            let mut bundle = bundles::SymbolCollectionBundle::new(ScopingBuilt::new());
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
        super::PassKey::BuildReactivitySemantics => {
            build_v2(
                component,
                parsed,
                data,
                ReactivityInputs {
                    inline_runes: options.inline_runes,
                    compile_runes: options.runes,
                    immutable: options.immutable,
                    accessors: options.accessors,
                },
            );
        }
        super::PassKey::BuildValueEvaluation => {
            data.value_evaluation = value_evaluation::build(
                parsed,
                &data.scoping,
                data.scoping.semantics(),
                &data.template.snippets,
                &data.reactivity,
                data.script.dev,
            );
        }
        super::PassKey::FinalizeReactivity => {
            finalize_reactivity(
                parsed,
                &mut data.reactivity,
                &data.value_evaluation,
                data.scoping.semantics(),
            );
        }
        super::PassKey::BuildExpressionSemantics => {
            let expressions_v2 = expression_semantics::build(
                component,
                parsed,
                data.scoping.semantics(),
                &data.reactivity,
                &data.scoping,
                &data.template.snippets,
                &data.value_evaluation,
                data.script.has_class_state_fields,
                &data.script.blocker_data,
                data.script.runes_mode,
                component.node_count(),
                data.script.dev,
            );
            if !data.output.needs_context && expressions_v2.is_context_required() {
                data.output.needs_context = true;
            }
            data.expressions_v2 = expressions_v2;
        }
        super::PassKey::BuildAttributeSemantics => {
            let (attributes, binding_groups) = attribute_semantics::build(
                component,
                parsed,
                &data.scoping,
                data.scoping.semantics(),
                &data.reactivity,
                &data.expressions_v2,
                &data.template.snippets,
                &data.value_evaluation,
                &data.script.blocker_data,
                &data.output.ignore_data,
                options.dev,
                component.node_count(),
            );
            data.attributes = attributes;
            data.template.bind_semantics.binding_group_id_by_attr = binding_groups.ids;
            data.template.bind_semantics.binding_group_count = binding_groups.count;
        }
        super::PassKey::BuildBlockSemantics => {
            data.block_semantics_store = block_semantics::build(
                component,
                parsed,
                data.scoping.semantics(),
                &data.reactivity,
                &data.expressions_v2,
                &data.template.fragment_namespaces,
                &data.output.ignore_data,
                data.script.dev,
                component.node_count(),
            );
        }
        super::PassKey::BuildFragmentTopology => {
            fragment_topology::build(component, data);
        }
        super::PassKey::TemplateClassificationWalk => {
            let mut bundle = bundles::TemplateClassificationBundle::new(component, data, source);
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
            let legacy_explicit = match options.runes {
                svelte_ast::RunesOption::Legacy => true,
                svelte_ast::RunesOption::Auto | svelte_ast::RunesOption::Runes => false,
            };
            validate::validate(component, data, parsed, runes, legacy_explicit, diags);
        }
    }
}
