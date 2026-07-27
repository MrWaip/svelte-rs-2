use svelte_ast::Component;
use svelte_diagnostics::Diagnostic;

use crate::reactivity_semantics::{
    ReactivityInputs, build_v2, finalize_component_prop_facts, finalize_reactivity,
};
use crate::types::markers::ScopingBuilt;
use crate::{
    AnalysisData, AnalyzeOptions, JsAst, await_semantics, validate, value_evaluation, walker,
};
use crate::{
    attribute_semantics, block_semantics, element_semantics, expression_semantics,
    fragment_semantics, runtime_semantics,
};

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
    let component_name = data.component_name.clone();
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
            js_analyze::analyze_script(data, parsed);
        }
        super::PassKey::BuildComponentSemantics => {
            super::build_component_semantics::build(component, parsed, data);
        }
        super::PassKey::FinalizeComponentName => {
            finalize_component_name::run(data);
        }
        super::PassKey::ScanIgnoreComments => {
            if options.dev {
                if let Some(program) = &parsed.program {
                    data.ignore
                        .scan_program_comments(program, &component.source, runes);
                }
                data.ignore
                    .scan_comments(parsed.template_comments(), &component.source, runes);
            }
        }
        super::PassKey::ExtractCeConfig => {
            data.script.ce_config =
                super::build_ce_config::build(component, parsed, data.scoping.semantics());
        }
        super::PassKey::TemplateSideTables => {
            super::template_side_tables::collect_fragment_facts(component, data);
            super::template_side_tables::collect_rich_content_facts(component, data);
            let mut bundle = bundles::TemplateSideTablesBundle::new(component, ScopingBuilt::new());
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
            super::template_side_tables::promote_anchor_namespaces(component, data);
            super::template_side_tables::collect_fragment_namespaces(component, data);

            data.template.expression_tags_by_fragment = bundle.take_expression_tag_buckets();
        }
        super::PassKey::BuildFragmentSemantics => {
            data.fragment_semantics = fragment_semantics::build(component, data);
        }
        super::PassKey::BuildRuntimeSemantics => {
            let semantics = runtime_semantics::build(
                &data.script,
                &data.reactivity,
                &data.elements,
                &data.expressions_v2,
                &data.api_exports,
                data.legacy_has_export_declaration,
                data.custom_element.is_target,
                source,
                options.dev,
            );
            data.runtime_semantics.record(semantics);
        }
        super::PassKey::JsAnalyzePostTemplate => {
            js_analyze::calculate_instance_blockers(parsed, data);
        }
        super::PassKey::BuildAwaitSemantics => {
            let semantics = await_semantics::build(component, parsed, &data.reactivity);
            data.await_semantics = semantics;
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
                component,
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
                data.script.dev,
            );
            finalize_component_prop_facts(&mut data.reactivity, &data.scoping);
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
                data.script.observes_context,
                &data.script.blocker_data,
                &data.await_semantics,
                data.script.runes_mode,
                component.node_count(),
                data.script.dev,
            );
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
                &data.ignore,
                &data.elements.facts,
                options.dev,
                component.node_count(),
            );
            data.attributes = attributes;
            data.template.bind_semantics.binding_group_id_by_attr = binding_groups.ids;
            data.template.bind_semantics.binding_group_count = binding_groups.count;
        }
        super::PassKey::BuildBlockSemantics => {
            let block_store = block_semantics::build(
                component,
                parsed,
                data.scoping.semantics(),
                &data.reactivity,
                &data.expressions_v2,
                &data.template.fragment_namespaces,
                &data.ignore,
                &data.script.blocker_data,
                data.script.dev,
                component.node_count(),
            );
            data.block_semantics_store = block_store;
        }
        super::PassKey::BuildElementSemantics => {
            data.element_semantics =
                element_semantics::build(component, parsed, data, source, component.node_count());
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
        super::PassKey::Validate => {
            let legacy_explicit = match options.runes {
                svelte_ast::RunesOption::Legacy => true,
                svelte_ast::RunesOption::Auto | svelte_ast::RunesOption::Runes => false,
            };
            validate::validate(component, data, parsed, runes, legacy_explicit, diags);
        }
    }
}
