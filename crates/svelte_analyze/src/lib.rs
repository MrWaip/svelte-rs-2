pub(crate) mod css;
pub(crate) mod passes;

pub use passes::css_analyze::analyze_css_pass;
pub mod scope;
pub mod types;
pub(crate) mod utils;
mod validate;
pub(crate) mod walker;

pub use scope::ComponentScoping;
pub use types::data::{
    AnalysisData, AsyncStmtMeta, AwaitBindingData, AwaitBindingInfo, BlockerData,
    ClassDirectiveInfo, CodegenView, ComponentBindMode, ComponentPropInfo, ComponentPropKind,
    ConstTagData, ContentStrategy, CssAnalysis, DebugTagData, DestructureKind, EachContextIndex,
    ElementFacts, ElementFactsEntry, ElementFlags, EventHandlerMode, ExprDeps, ExprHandle,
    ExprSite, ExpressionInfo, ExpressionKind, FragmentData, FragmentFacts, FragmentFactsEntry,
    FragmentItem, FragmentKey, FragmentKeyExt, IgnoreData, LoweredFragment, LoweredTextPart,
    ParentKind, ParentRef, ParserResult, PropAnalysis, PropsAnalysis, RenderTagCalleeMode,
    RenderTagPlan, RichContentFacts, RichContentFactsEntry, RichContentParentKind, RuntimePlan,
    SnippetData, StmtHandle, TemplateElementEntry, TemplateElementIndex, TemplateTopology,
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

    let mut data = AnalysisData::new_empty(component.node_count());
    data.runes = options.runes;
    data.custom_element = options.custom_element;
    let execution_order = passes::resolve_default_execution_order()
        .unwrap_or_else(|err| panic!("invalid analyze pass configuration: {err:?}"));
    debug_assert_eq!(execution_order, passes::default_stage_execution_order());

    for &key in passes::PRE_TEMPLATE_SCRIPT_STAGE {
        passes::execute_pass(key, component, &mut parsed, &mut data, options, &mut diags);
    }
    for &key in passes::INDEX_BUILD_STAGE {
        passes::execute_pass(key, component, &mut parsed, &mut data, options, &mut diags);
    }
    for &key in passes::POST_TEMPLATE_ANALYSIS_STAGE {
        passes::execute_pass(key, component, &mut parsed, &mut data, options, &mut diags);
    }
    for &key in passes::TEMPLATE_EXECUTION_STAGE {
        passes::execute_pass(key, component, &mut parsed, &mut data, options, &mut diags);
    }
    for &key in passes::VALIDATION_STAGE {
        passes::execute_pass(key, component, &mut parsed, &mut data, options, &mut diags);
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
        Ok((program, _scoping)) => {
            let mut builder = svelte_component_semantics::ComponentSemanticsBuilder::new();
            builder.add_instance_program(&program);
            let mut scoping = scope::ComponentScoping::from_semantics(builder.finish());
            scoping.build_template_scope_set();

            let mut script_info = utils::script_info::extract_script_info(&program, 0, source);
            utils::script_info::enrich_from_component_scoping(&scoping, &mut script_info);

            data.scoping = scoping;
            data.script = Some(script_info);
            passes::mark_runes::mark_script_runes(&mut data);
            passes::mark_runes::mark_nested_runes(&program, &mut data.scoping);
            data.import_syms = data.scoping.collect_import_syms();
            validate::validate_program(&data, &program, 0, true, &mut diags);
        }
        Err(errs) => diags.extend(errs),
    }

    (data, diags)
}
fn build_runtime_plan(data: &AnalysisData, dev: bool) -> RuntimePlan {
    let has_exports = !data.exports.is_empty();
    let has_bindable = data.props.as_ref().is_some_and(|p| p.has_bindable);
    let has_stores = data.scoping.has_stores();
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
