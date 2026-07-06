pub mod attribute_semantics;
pub mod block_semantics;
pub(crate) mod css;
pub mod expression_semantics;
pub(crate) mod passes;
pub mod reactivity_semantics;
pub mod value_evaluation;

pub use attribute_semantics::{
    AttributeSemantics, AttributeSemanticsStore, BoundaryPropSemantics, ClassSemantics,
    ComponentAttachEmit, ComponentAttachSemantics, ComponentBindKind, ComponentBindSemantics,
    ComponentBindTarget, ComponentCssPropValue, ComponentPropConcatSemantics,
    ComponentPropExpressionSemantics, ComponentPropMemo, ComponentPropSemantics,
    ComponentSpreadEmit, ComponentSpreadSemantics, ConcatPartEmit, DefaultAttrKind,
    DefaultAttrSemantics, DocumentBindSemantics, ElementBindPropertyKind, ElementBindSemantics,
    EventEmit, EventSemantics, HandlerEmit, HtmlBindKind, HtmlConcatPart, HtmlConcatSemantics,
    SpecialValueKind, SpecialValueSemantics, StyleSemantics, SvelteComponentThisSemantics,
    TemplateEffect, WindowBindSemantics,
};
pub use expression_semantics::{
    Evaluation, ExpressionData, ExpressionSemantics, ExpressionSemanticsStore, KnownValue,
    LegacyWrap, SyntheticPropsCarrier, ValueClass, Volatility,
};

pub use passes::css_analyze::analyze_css_pass;
pub mod scope;
pub mod types;
pub(crate) mod utils;
mod validate;
pub(crate) mod walker;

pub use block_semantics::{
    AwaitBinding, AwaitBlockSemantics, AwaitBranch, AwaitDestructureKind, AwaitWrapper,
    BlockSemantics, ConstTagAsyncKind, ConstTagBlockSemantics, EachAsyncKind, EachBlockSemantics,
    EachCollection, EachCollectionSource, EachFlags, EachFlavor, EachIndexKind, EachItemKind,
    EachKeyKind, HtmlTagAsyncKind, IfAlternate, IfAsyncKind, IfBlockSemantics, IfBranch,
    IfConditionKind, KeyAsyncKind, KeyBlockSemantics, RenderArgKind, RenderAsyncKind,
    RenderCallKind, RenderTagBlockSemantics, SnippetBlockSemantics, SnippetParam, SnippetPlacement,
    SnippetSlotKey,
};
pub use scope::ComponentScoping;
pub use types::data::{
    AnalysisData, ApiExport, AsyncStmtMeta, AttrIndex, BindHostKind, BindPropertyKind, BindSource,
    BindTargetSemantics, BindingSemantics, BlockAnalysis, BlockerData, CarrierMemberReadSemantics,
    ClassDirectiveInfo, ClassFieldDerivedSemantics, ClassFieldSemantics, ClassFieldStateSemantics,
    CodegenView, ComponentBindMode, ComponentPropInfo, ComponentPropKind, ConstBindingSemantics,
    ContentEditableKind, ContextualBindingSemantics, ContextualReadKind, ContextualReadSemantics,
    CssAnalysis, DeclaratorGroup, DeclaratorSemantics, DerivedDeclarationSemantics, DerivedEmit,
    DerivedKind, DerivedSource, DocumentBindKind, EachIndexStrategy, EachItemStrategy,
    ElementAnalysis, ElementFacts, ElementFactsEntry, ElementFlags, ElementSizeKind,
    EventHandlerMode, EventModifier, FragmentFacts, FragmentFactsEntry, IgnoreData,
    ImageNaturalSizeKind, JsAst, LegacyBindablePropSemantics, LegacyDefaultSlot, LegacyDependency,
    LegacyInit, LegacySummary, MediaBindKind, NamespaceKind, OptimizedRuneSemantics, OutputData,
    ParentKind, ParentRef, PickledAwaits, PropBindingKind, PropBindingSemantics, PropDefaultKind,
    PropEmitMode, PropReferenceSemantics, PropsSummary, ReactivitySemantics, ReactivitySummary,
    ReferenceSemantics, ResizeObserverKind, RichContentFacts, RichContentFactsEntry,
    RichContentParentKind, RuntimeInfo, RuntimeRuneKind, ScriptAnalysis, SignalReferenceKind,
    SnippetData, SnippetParamStrategy, StateDeclarationSemantics, StateKind, StoreBindingSemantics,
    TemplateAnalysis, TemplateElementEntry, TemplateElementIndex, TemplateTopology, WindowBindKind,
};

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
    pub struct PropsFlags: u32 {
        const IMMUTABLE    = 1;
        const RUNES        = 1 << 1;
        const UPDATED      = 1 << 2;
        const BINDABLE     = 1 << 3;
        const LAZY_INITIAL = 1 << 4;
    }
}

pub const PROPS_IS_IMMUTABLE: u32 = PropsFlags::IMMUTABLE.bits();
pub const PROPS_IS_RUNES: u32 = PropsFlags::RUNES.bits();
pub const PROPS_IS_UPDATED: u32 = PropsFlags::UPDATED.bits();
pub const PROPS_IS_BINDABLE: u32 = PropsFlags::BINDABLE.bits();
pub const PROPS_IS_LAZY_INITIAL: u32 = PropsFlags::LAZY_INITIAL.bits();
pub use utils::{IdentGen, IdentGenSnapshot};
pub use utils::{
    collapse_attribute_whitespace, concat_single_dynamic_expr, emit_html_attribute_name,
    event_attribute, expression_calls_or_awaits, is_capture_event, is_delegatable_event,
    is_dom_boolean_attribute, is_let_or_var, is_passive_event, is_regular_dom_property,
    is_simple_expression, is_simple_identifier, normalize_regular_attribute_name,
    property_key_static_name, strip_capture_event,
};

use svelte_ast::Component;
use svelte_diagnostics::{Diagnostic, Severity};

pub struct AnalyzeOptions {
    pub custom_element: bool,
    pub experimental_async: bool,
    pub runes: svelte_ast::RunesOption,
    pub inline_runes: Option<bool>,
    pub accessors: bool,
    pub immutable: bool,
    pub preserve_whitespace: bool,
    pub preserve_comments: bool,
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
            runes: svelte_ast::RunesOption::Runes,
            inline_runes: None,
            accessors: false,
            immutable: false,
            preserve_whitespace: false,
            preserve_comments: false,
            dev: false,
            component_name: "Self".to_string(),
            filename_basename: "Self.svelte".to_string(),
            warning_filter: None,
        }
    }
}

pub fn analyze<'a>(
    component: &Component,
    parsed: JsAst<'a>,
) -> (AnalysisData<'a>, JsAst<'a>, Vec<Diagnostic>) {
    analyze_with_options(component, parsed, &AnalyzeOptions::default())
}

pub fn analyze_with_options<'a>(
    component: &Component,
    mut parsed: JsAst<'a>,
    options: &AnalyzeOptions,
) -> (AnalysisData<'a>, JsAst<'a>, Vec<Diagnostic>) {
    let mut diags = Vec::new();

    let mut data = AnalysisData::new_empty(component.node_count());
    data.script.preserve_whitespace = options.preserve_whitespace;
    data.script.preserve_comments = options.preserve_comments;
    data.script.dev = options.dev;
    data.output.custom_element_compile_flag = options.custom_element;
    data.output.is_custom_element_target = options.custom_element
        || component
            .options
            .as_ref()
            .and_then(|opts| opts.custom_element.as_ref())
            .is_some();
    data.output.component_name = options.component_name.clone();
    data.script.experimental_async = options.experimental_async;
    debug_assert_eq!(
        passes::resolve_default_execution_order()
            .unwrap_or_else(|err| panic!("invalid analyze pass configuration: {err:?}")),
        passes::default_stage_execution_order()
    );

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

    if let Some(ref filter) = options.warning_filter {
        diags.retain(|d| d.severity != Severity::Warning || filter(d));
    }

    data.output.runtime_plan = build_runtime_info(component, &data, options.dev);

    (data, parsed, diags)
}

pub fn analyze_module<'a>(
    alloc: &'a oxc_allocator::Allocator,
    source: &'a str,
    is_ts: bool,
    dev: bool,
) -> (AnalysisData<'a>, JsAst<'a>, Vec<Diagnostic>) {
    let mut diags = Vec::new();
    let mut data = AnalysisData::new_empty(0);
    let mut parsed = JsAst::new();

    match svelte_parser::parse_module(alloc, source, is_ts) {
        Ok(program) => {
            let mut builder = svelte_component_semantics::ComponentSemanticsBuilder::new();
            builder.add_instance_program(&program);
            let mut scoping = scope::ComponentScoping::from_semantics(builder.finish());
            scoping.build_template_scope_set();

            data.scoping = scoping;
            data.script.runes_mode = svelte_ast::RunesMode::Runes;

            validate::validate_standalone_module(&data, &program, true, &mut diags);

            parsed.program = Some(program);
            let stub_component =
                svelte_ast::Component::dummy_for_standalone_module(source.to_string());
            reactivity_semantics::build_v2(
                &stub_component,
                &parsed,
                &mut data,
                reactivity_semantics::ReactivityInputs {
                    inline_runes: None,
                    compile_runes: svelte_ast::RunesOption::Runes,
                    immutable: false,
                    accessors: false,
                },
            );
            reactivity_semantics::finalize_proxy(
                &parsed,
                &mut data.reactivity,
                data.scoping.semantics(),
            );

            data.script.dev = dev;
            if dev {
                data.value_evaluation = value_evaluation::build_module_console_calls(
                    &parsed,
                    &data.scoping,
                    data.scoping.semantics(),
                );
            }

            if let Some(program) = parsed.program.as_ref() {
                validate::validate_module_experimental_async(&data, program, &mut diags);
            }
        }
        Err(errs) => diags.extend(errs),
    }

    (data, parsed, diags)
}
fn build_runtime_info(
    component: &svelte_ast::Component,
    data: &AnalysisData<'_>,
    dev: bool,
) -> RuntimeInfo {
    let summary = data.reactivity.summary();
    let has_exports = !data.output.api_exports.is_empty();
    let needs_push = needs_push(data, summary, has_exports, dev);
    let has_legacy_accessor_props = has_legacy_accessor_props(data);
    let has_component_exports =
        has_exports || summary.props.has_custom_element || has_legacy_accessor_props || dev;
    let needs_props_param = needs_props_param(component, data, summary, needs_push);

    RuntimeInfo {
        needs_push,
        has_component_exports,
        has_exports,
        has_bindable: summary.props.has_bindable,
        has_stores: summary.has_store_bindings,
        has_ce_props: summary.props.has_custom_element,
        has_legacy_accessor_props,
        needs_props_param,
        needs_pop_with_return: needs_push && has_component_exports,
        legacy_init: legacy_init(data, summary.legacy),
    }
}

fn needs_push(
    data: &AnalysisData<'_>,
    summary: ReactivitySummary,
    has_exports: bool,
    dev: bool,
) -> bool {
    if summary.props.has_bindable || summary.props.has_custom_element {
        return true;
    }
    if has_exports {
        return true;
    }
    if data.output.needs_context {
        return true;
    }
    if dev {
        return true;
    }
    if data.uses_runes() {
        return false;
    }
    if has_legacy_accessor_props(data) {
        return true;
    }
    if summary.legacy.has_member_mutated {
        return true;
    }
    has_legacy_reactive_statements(data)
}

fn has_legacy_reactive_statements(data: &AnalysisData<'_>) -> bool {
    data.reactivity
        .legacy_reactive()
        .iter_statements_topo()
        .next()
        .is_some()
}

fn has_legacy_accessor_props(data: &AnalysisData<'_>) -> bool {
    if data.uses_runes() {
        return false;
    }
    if !data.script.accessors {
        return false;
    }
    data.reactivity
        .legacy_bindable_prop_symbols()
        .iter()
        .any(|&sym| !legacy_bindable_prop_key(data, sym).starts_with("$$"))
}

fn legacy_bindable_prop_key<'d>(
    data: &'d AnalysisData<'_>,
    sym: svelte_component_semantics::SymbolId,
) -> &'d str {
    match data.reactivity.legacy_bindable_prop_alias(sym) {
        Some(alias) => alias,
        None => data.scoping.symbol_name(sym),
    }
}

fn needs_props_param(
    component: &svelte_ast::Component,
    data: &AnalysisData<'_>,
    summary: ReactivitySummary,
    needs_push: bool,
) -> bool {
    if needs_push || summary.props.has_props {
        return true;
    }
    if summary.legacy.has_bindable_prop {
        return true;
    }
    if summary.legacy.reads_props_object || summary.legacy.reads_rest_props_object {
        return true;
    }
    if data.output.legacy_has_export_declaration {
        return true;
    }
    has_legacy_event_forward(component)
}

fn has_legacy_event_forward(component: &svelte_ast::Component) -> bool {
    for raw in 0..component.node_count() {
        let id = svelte_ast::NodeId(raw);
        let node = component.store.get(id);
        let forwards = node.attributes().iter().any(|a| {
            matches!(
                a,
                svelte_ast::Attribute::OnDirectiveLegacy(od) if od.expression.is_none()
            )
        });
        if forwards {
            return true;
        }
    }
    false
}

fn legacy_init(data: &AnalysisData<'_>, legacy: LegacySummary) -> LegacyInit {
    if data.uses_runes() {
        return LegacyInit::None;
    }
    let needs_init = legacy.has_member_mutated || data.output.needs_context;
    if !needs_init {
        return LegacyInit::None;
    }
    if data.script.immutable {
        return LegacyInit::Immutable;
    }
    LegacyInit::Plain
}

#[cfg(test)]
pub(crate) mod tests;
