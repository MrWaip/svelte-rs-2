pub mod block_semantics;
pub(crate) mod css;
pub(crate) mod passes;
pub mod reactivity_semantics;

pub use passes::css_analyze::analyze_css_pass;
pub mod scope;
pub mod types;
pub(crate) mod utils;
mod validate;
pub(crate) mod walker;

pub use block_semantics::{
    AwaitBinding, AwaitBlockSemantics, AwaitBranch, AwaitDestructureKind, AwaitWrapper,
    BlockSemantics, ConstTagAsyncKind, ConstTagBlockSemantics, EachAsyncKind, EachBlockSemantics,
    EachCollectionKind, EachFlags, EachFlavor, EachIndexKind, EachItemKind, EachKeyKind,
    IfAlternate, IfAsyncKind, IfBlockSemantics, IfBranch, IfConditionKind, KeyAsyncKind,
    KeyBlockSemantics, RenderArgLowering, RenderAsyncKind, RenderCalleeShape,
    RenderTagBlockSemantics, SnippetBlockSemantics, SnippetParam,
};
pub use scope::ComponentScoping;
pub use types::data::{
    AnalysisData, AsyncStmtMeta, AttrIndex, BindHostKind, BindPropertyKind, BindTargetSemantics,
    BindingSemantics, BlockAnalysis, BlockerData, CarrierMemberReadSemantics, ClassDirectiveInfo,
    CodegenView, ComponentBindMode, ComponentPropInfo, ComponentPropKind, ConstBindingSemantics,
    ConstTagData, ContentEditableKind, ContextualBindingSemantics, ContextualReadKind,
    ContextualReadSemantics, CssAnalysis, DebugTagData, DeclaratorSemantics,
    DerivedDeclarationSemantics, DerivedKind, DerivedLowering, DirectiveModifierFlags,
    DocumentBindKind, EachIndexStrategy, EachItemStrategy, ElementAnalysis, ElementFacts,
    ElementFactsEntry, ElementFlags, ElementSizeKind, EventHandlerMode, EventModifier, ExprDeps,
    ExprRole, ExprSite, ExpressionInfo, ExpressionKind, FragmentFacts, FragmentFactsEntry,
    IgnoreData, ImageNaturalSizeKind, JsAst, LegacyBindablePropSemantics, LegacyInit,
    MediaBindKind, NamespaceKind, OptimizedRuneSemantics, OutputPlanData, ParentKind, ParentRef,
    PickledAwaitOffsets, PropBindingKind, PropBindingSemantics, PropDefaultLowering,
    PropLoweringMode, PropReferenceSemantics, ProxyStateInits, ReactivitySemantics,
    ReferenceSemantics, ResizeObserverKind, RichContentFacts, RichContentFactsEntry,
    RichContentParentKind, RuntimePlan, RuntimeRuneKind, ScriptAnalysis, ScriptRuneCalls,
    SignalReferenceKind, SnippetData, SnippetParamStrategy, StateBindingSemantics,
    StateDeclarationSemantics, StateKind, StoreBindingSemantics, TemplateAnalysis,
    TemplateElementEntry, TemplateElementIndex, TemplateTopology, WindowBindKind,
};
pub use types::script::{
    DeclarationInfo, DeclarationKind, ExportInfo, PropInfo, PropsDeclaration, RuneKind, ScriptInfo,
};
pub use utils::script_info::BINDABLE_RUNE_NAME;

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
    is_capture_event, is_delegatable_event, is_let_or_var, is_passive_event,
    is_regular_dom_property, is_simple_expression, is_simple_identifier,
    normalize_regular_attribute_name, property_key_static_name, strip_capture_event,
};

use svelte_ast::Component;
use svelte_diagnostics::{Diagnostic, Severity};

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
    data.script.runes = options.runes;
    data.script.accessors = options.accessors;
    data.script.immutable = options.immutable;
    data.script.preserve_whitespace = options.preserve_whitespace;
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

    if let Some(ref filter) = options.warning_filter {
        diags.retain(|d| d.severity != Severity::Warning || filter(d));
    }

    if data
        .scoping
        .root_unresolved_references()
        .contains_key("$$slots")
    {
        data.output.needs_sanitized_legacy_slots = true;
    }

    data.output.runtime_plan = build_runtime_plan(&data, options.dev);

    (data, parsed, diags)
}

pub fn analyze_module<'a>(
    alloc: &'a oxc_allocator::Allocator,
    source: &'a str,
    is_ts: bool,
    dev: bool,
) -> (AnalysisData<'a>, JsAst<'a>, Vec<Diagnostic>) {
    let _ = dev;
    let mut diags = Vec::new();
    let mut data = AnalysisData::new_empty(0);
    let mut parsed = JsAst::new();

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
            data.script.runes = true;

            validate::validate_standalone_module(&data, &program, 0, true, &mut diags);

            parsed.program = Some(program);
            let stub_component =
                svelte_ast::Component::dummy_for_standalone_module(source.to_string());
            reactivity_semantics::build_v2(&stub_component, &parsed, &mut data);
        }
        Err(errs) => diags.extend(errs),
    }

    (data, parsed, diags)
}
fn build_runtime_plan(data: &AnalysisData<'_>, dev: bool) -> RuntimePlan {
    let legacy_symbols = data.reactivity.legacy_bindable_prop_symbols();
    let has_legacy_bindable_prop = !legacy_symbols.is_empty();
    let has_legacy_member_mutated = data.reactivity.legacy_has_member_mutated();
    let has_legacy_props_read = data.reactivity.legacy_uses_props();
    let has_legacy_reactive_statements = data
        .reactivity
        .legacy_reactive()
        .iter_statements_topo()
        .next()
        .is_some();
    let has_exports = data.script.exports.iter().any(|exp| {
        let Some(instance_scope) = data.scoping.instance_scope_id() else {
            return true;
        };
        let Some(sym) = data.scoping.find_binding(instance_scope, exp.name.as_str()) else {
            return true;
        };
        !legacy_symbols.contains(&sym)
    });
    let has_bindable = data
        .script
        .props_declaration()
        .is_some_and(|d| d.has_bindable());
    let has_stores = data.reactivity.has_store_bindings();
    let has_ce_props = data.output.is_custom_element_target
        && data
            .script
            .props_declaration()
            .is_some_and(|d| !d.props.is_empty());
    let needs_push = has_bindable
        || has_exports
        || has_ce_props
        || data.output.needs_context
        || data.script.accessors
        || (!data.uses_runes() && data.script.immutable)
        || dev
        || (!data.uses_runes()
            && (has_legacy_member_mutated
                || has_legacy_props_read
                || has_legacy_reactive_statements));
    let has_component_exports = has_exports || has_ce_props || data.script.accessors || dev;
    let needs_props_param =
        data.script.props_declaration().is_some() || needs_push || has_legacy_bindable_prop;

    let legacy_init = if data.uses_runes() {
        crate::types::data::LegacyInit::None
    } else if data.script.immutable {
        crate::types::data::LegacyInit::Immutable
    } else if has_legacy_member_mutated || has_legacy_props_read || data.output.needs_context {
        crate::types::data::LegacyInit::Plain
    } else {
        crate::types::data::LegacyInit::None
    };

    RuntimePlan {
        needs_push,
        has_component_exports,
        has_exports,
        has_bindable,
        has_stores,
        has_ce_props,
        needs_props_param,
        needs_pop_with_return: needs_push && has_component_exports,
        legacy_init,
    }
}

#[cfg(test)]
pub(crate) mod tests;
