pub mod attribute_semantics;
pub mod await_semantics;
pub mod block_semantics;
pub(crate) mod css;
pub mod element_semantics;
pub mod expression_semantics;
pub mod fragment_semantics;
pub(crate) mod js_walker;
pub(crate) mod passes;
pub mod reactivity_semantics;
pub mod runtime_semantics;
pub mod value_evaluation;

pub use attribute_semantics::{
    AttributeSemantics, AttributeSemanticsStore, BoundaryPropSemantics, ClassSemantics,
    ComponentAttachEmit, ComponentAttachSemantics, ComponentBindKind, ComponentBindSemantics,
    ComponentBindTarget, ComponentCssPropValue, ComponentPropConcatSemantics,
    ComponentPropExpressionSemantics, ComponentPropMemo, ComponentPropSemantics,
    ComponentSpreadEmit, ComponentSpreadSemantics, ConcatPartEmit, DefaultAttrKind,
    DefaultAttrSemantics, DocumentBindSemantics, ElementBindPropertyKind, ElementBindSemantics,
    EventHandler, EventSemantics, GroupBindValue, GroupReflection, HandlerEffect, HtmlBindKind,
    HtmlConcatPart, HtmlConcatSemantics, SkipCause, SpecialValueKind, SpecialValueSemantics,
    StyleSemantics, SvelteComponentThisSemantics, TemplateEffect, WindowBindSemantics,
};
pub use expression_semantics::{
    Evaluation, ExpressionData, ExpressionSemantics, ExpressionSemanticsStore, KnownValue,
    LegacyWrap, Suspension, SyntheticPropsCarrier, ValueClass, Volatility,
};

pub use css::head_hash;
pub use passes::css_analyze::analyze_css_pass;
pub mod scope;
pub mod types;
pub(crate) mod utils;
pub(crate) mod validate;
pub(crate) mod walker;

pub use await_semantics::{AwaitSemantics, AwaitSemanticsStore};
pub use block_semantics::{
    AwaitBinding, AwaitBlockSemantics, AwaitBranch, AwaitDestructureKind, AwaitWrapper,
    BlockSemantics, ConstTagBlockSemantics, DeclarationTagBlockSemantics, EachAsyncKind,
    EachBlockSemantics, EachCollection, EachCollectionSource, EachFlags, EachFlavor, EachIndexKind,
    EachItemKind, EachKeyKind, ExpressionBlocker, FragmentDeclarationAsyncKind, HtmlTagAsyncKind,
    IfAlternate, IfAsyncKind, IfBlockSemantics, IfBranch, IfConditionKind, KeyAsyncKind,
    KeyBlockSemantics, RenderArgKind, RenderAsyncKind, RenderCallKind, RenderTagBlockSemantics,
    SnippetBlockSemantics, SnippetParam, SnippetPlacement, SnippetSlotKey,
};
pub use element_semantics::{
    BoundaryBranch, BoundarySemantics, ComponentElementSemantics, ElementAsyncKind,
    ElementPropertyReset, ElementReplayEvent, ElementSemantics, ElementSemanticsStore,
    ElementValueRole, LegacyComponentSlotsSemantics, LegacyDefaultSlot, LegacySlotSemantics,
    RegularElementSemantics, SvelteElementSemantics, TextareaBody, TextareaSegment,
};
pub use fragment_semantics::{
    FragmentContent, FragmentSemantics, FragmentSemanticsStore, FragmentWhitespace,
};
pub use runtime_semantics::{
    ChildPropMode, ComponentBindOwnership, ComponentFrame, ContextScope, FunctionTracing,
    LegacyInit, LegacySlotSanitization, PropAccessors, PropsInput, RuntimeSemantics,
    RuntimeSemanticsStore, StoreBindings,
};
pub use scope::ComponentScoping;
pub use types::data::{
    AnalysisData, ApiExport, AsyncEntry, AsyncEntryLocation, AsyncEntryMemberKind, AttrIndex,
    BindHostKind, BindPropertyKind, BindSource, BindTargetSemantics, BindingSemantics,
    BlockAnalysis, BlockerData, BlockerSlot, CarrierMemberReadSemantics, ClassDirectiveInfo,
    ClassFieldDerivedSemantics, ClassFieldSemantics, ClassFieldStateSemantics, CodegenView,
    ComponentBindMode, ComponentPropInfo, ComponentPropKind, ConstTagSemantics,
    ContentEditableKind, ContextualBindingSemantics, ContextualReadKind, ContextualReadSemantics,
    CssAnalysis, DeclaratorGroup, DeclaratorSemantics, DerivedAsyncKind,
    DerivedDeclarationSemantics, DerivedKind, DerivedSource, DocumentBindKind, EachIndexStrategy,
    EachItemStrategy, ElementAnalysis, ElementFacts, ElementFactsEntry, ElementFlags,
    ElementSizeKind, EventHandlerMode, EventModifier, FragmentFacts, FragmentFactsEntry,
    IgnoreData, ImageNaturalSizeKind, JsAst, LegacyBindablePropSemantics, LegacyDependency,
    LegacySummary, MediaBindKind, NamespaceKind, OptimizedRuneSemantics, ParentKind, ParentRef,
    PropBindingKind, PropBindingSemantics, PropDefaultKind, PropEmitMode, PropReferenceSemantics,
    PropsSummary, ReactivitySemantics, ReactivitySummary, ReferenceSemantics, ResizeObserverKind,
    RichContentFacts, RichContentFactsEntry, RichContentParentKind, RuntimeRuneKind,
    ScriptAnalysis, SignalReadLocality, SignalReferenceKind, SnippetData, SnippetParamStrategy,
    StateDeclarationSemantics, StateKind, StoreBindingSemantics, SvelteElementTag,
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
pub use svelte_diagnostics::codes::WarningCode;
pub use utils::{IdentGen, IdentGenSnapshot};
pub use utils::{
    collapse_attribute_whitespace, concat_single_dynamic_expr, emit_html_attribute_name,
    event_attribute, expression_calls_or_awaits, is_dom_boolean_attribute, is_let_or_var,
    is_regular_dom_property, is_simple_expression, is_simple_identifier,
    normalize_regular_attribute_name, property_key_static_name,
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
    data.custom_element.compile_flag = options.custom_element;
    data.custom_element.is_target = options.custom_element
        || component
            .options
            .as_ref()
            .and_then(|opts| opts.custom_element.as_ref())
            .is_some();
    data.component_name = options.component_name.clone();
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
            let mut builder = svelte_component_semantics::ComponentSemanticsBuilder::with_capacity(
                source.len() / 6,
            );
            builder.add_instance_program(&program);
            let mut scoping = scope::ComponentScoping::from_semantics(builder.finish());
            scoping.build_template_scope_set();

            data.scoping = scoping;
            data.script.runes_mode = svelte_ast::RunesMode::Runes;
            data.script.is_standalone_module = true;

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
                dev,
            );

            if let Some(program) = parsed.program.as_ref() {
                validate::validate_standalone_module(&data, program, source, true, &mut diags);
            }

            data.script.dev = dev;
            if dev {
                if let Some(program) = parsed.program.as_ref() {
                    data.ignore.scan_program_comments(program, source, true);
                }
                let value_evaluation = value_evaluation::build(
                    &parsed,
                    &stub_component,
                    &data.scoping,
                    data.scoping.semantics(),
                    &data.template.snippets,
                    &data.reactivity,
                    dev,
                );
                data.value_evaluation = value_evaluation;
            }

            if let Some(program) = parsed.program.as_ref() {
                validate::validate_module_experimental_async(&data, program, &mut diags);
            }
        }
        Err(errs) => diags.extend(errs),
    }

    (data, parsed, diags)
}
#[cfg(test)]
pub(crate) mod tests;
