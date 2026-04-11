pub(crate) mod css;
pub(crate) mod passes;

pub use passes::css_analyze::analyze_css_pass;
pub mod scope;
pub mod types;
pub(crate) mod utils;
mod validate;
pub(crate) mod walker;

pub use scope::{ComponentScoping, TemplateBindingReadKind};
pub use types::data::{
    AnalysisData, AsyncStmtMeta, AttrIndex, AwaitBindingData, AwaitBindingInfo, BlockAnalysis,
    BlockerData, ClassDirectiveInfo, CodegenView, ComponentBindMode, ComponentPropInfo,
    ComponentPropKind, ConstTagData, ContentStrategy, CssAnalysis, DebugTagData, DestructureKind,
    DirectiveModifierFlags, EachContextIndex, ElementAnalysis, ElementFacts, ElementFactsEntry,
    ElementFlags, EventHandlerMode, EventModifier, ExprDeps, ExprHandle, ExprSite, ExpressionInfo,
    ExpressionKind, FragmentData, FragmentFacts, FragmentFactsEntry, FragmentItem, FragmentKey,
    FragmentKeyExt, IgnoreData, LoweredFragment, LoweredTextPart, NamespaceKind, OutputPlanData,
    ParentKind, ParentRef, ParserResult, PickledAwaitOffsets, PropAnalysis, PropsAnalysis,
    ProxyStateInits, RenderTagCalleeMode, RenderTagPlan, RichContentFacts, RichContentFactsEntry,
    RichContentParentKind, RuntimePlan, ScriptAnalysis, ScriptRuneCalls, SnippetData, StmtHandle,
    TemplateAnalysis, TemplateElementEntry, TemplateElementIndex, TemplateTopology,
};
pub use types::script::{
    DeclarationInfo, DeclarationKind, ExportInfo, PropInfo, PropsDeclaration, RuneKind, ScriptInfo,
};
pub use utils::IdentGen;
pub use utils::{
    is_capture_event, is_delegatable_event, is_passive_event, is_regular_dom_property,
    is_simple_expression, is_simple_identifier, normalize_regular_attribute_name,
    strip_capture_event,
};

use svelte_ast::Component;
use svelte_diagnostics::{Diagnostic, Severity};

/// Options controlling analysis behavior.
pub struct AnalyzeOptions {
    pub custom_element: bool,
    pub experimental_async: bool,
    pub runes: bool,
    pub accessors: bool,
    pub immutable: bool,
    pub preserve_whitespace: bool,
    pub dev: bool,
    pub component_name: String,
    pub filename_basename: String,
    pub warning_filter: Option<Box<dyn Fn(&Diagnostic) -> bool>>,
}

impl Default for AnalyzeOptions {
    fn default() -> Self {
        Self {
            custom_element: false,
            experimental_async: false,
            runes: true,
            accessors: false,
            immutable: false,
            preserve_whitespace: false,
            dev: false,
            component_name: "Self".to_string(),
            filename_basename: "Self.svelte".to_string(),
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
    data.script.runes = options.runes;
    data.script.accessors = options.accessors;
    data.script.immutable = options.immutable;
    data.script.preserve_whitespace = options.preserve_whitespace;
    data.output.custom_element = options.custom_element;
    data.output.component_name = options.component_name.clone();
    data.script.experimental_async = options.experimental_async;
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

    data.output.runtime_plan = build_runtime_plan(&data, options.dev);

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

            let mut script_info =
                utils::script_info::extract_script_info(&program, 0, source, true);
            utils::script_info::enrich_from_component_scoping(&scoping, &mut script_info);

            data.scoping = scoping;
            data.script.info = Some(script_info);
            passes::mark_runes::mark_script_runes(&mut data);
            passes::mark_runes::mark_nested_runes(&program, &mut data.scoping);
            validate::validate_program(&data, &program, 0, true, &mut diags);
        }
        Err(errs) => diags.extend(errs),
    }

    (data, diags)
}
fn build_runtime_plan(data: &AnalysisData, dev: bool) -> RuntimePlan {
    let has_exports = !data.script.exports.is_empty();
    let has_bindable = data.script.props.as_ref().is_some_and(|p| p.has_bindable);
    let has_stores = data.scoping.has_stores();
    let has_ce_props = data.output.custom_element
        && data
            .script
            .props
            .as_ref()
            .is_some_and(|p| !p.props.is_empty());
    let needs_push = has_bindable
        || has_exports
        || has_ce_props
        || data.output.needs_context
        || data.script.accessors
        || (!data.script.runes && data.script.immutable)
        || dev;
    let has_component_exports = has_exports || has_ce_props || data.script.accessors || dev;
    let needs_props_param = data.script.props.is_some() || needs_push;

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
