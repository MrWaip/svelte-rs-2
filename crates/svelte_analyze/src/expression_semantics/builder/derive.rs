use super::super::Evaluation;
use super::super::data::{LegacyWrap, SyntheticPropsCarrier, Volatility};
use super::collector::{ExprFacts, TopLevelForm};
use super::walker::Ctx;
use crate::reactivity_semantics::builder_v2::expression_root_reference_id;
use crate::reactivity_semantics::data::ContextualBindingSemantics;
use crate::reactivity_semantics::data::ReactivitySemantics;
use crate::scope::{ComponentScoping, SymbolId};
use crate::types::data::{BindingSemantics, BlockerData, PropBindingKind};
use crate::value_evaluation::symbol_read_is_static;
use oxc_ast::ast::Expression;
use smallvec::SmallVec;

pub(super) fn needs_context(facts: &ExprFacts, reactivity: &ReactivitySemantics) -> bool {
    if !matches!(
        facts.top_level_form,
        TopLevelForm::Member | TopLevelForm::Call
    ) {
        return false;
    }
    facts
        .references
        .iter()
        .any(|&sym| binding_reads_through_props_object(reactivity.binding_semantics(sym)))
}

fn binding_reads_through_props_object(semantics: BindingSemantics) -> bool {
    match semantics {
        BindingSemantics::MaybeReactive => true,
        BindingSemantics::Prop(prop) => match &prop.kind {
            PropBindingKind::Source { .. } | PropBindingKind::NonSource => true,
            PropBindingKind::Identifier | PropBindingKind::Rest => false,
        },
        BindingSemantics::State(_)
        | BindingSemantics::Derived(_)
        | BindingSemantics::OptimizedDerived(_)
        | BindingSemantics::OptimizedRune(_)
        | BindingSemantics::RuntimeRune { .. }
        | BindingSemantics::Store(_)
        | BindingSemantics::LegacyBindableProp(_)
        | BindingSemantics::LegacyState(_)
        | BindingSemantics::Const(_)
        | BindingSemantics::OptimizedConst(_)
        | BindingSemantics::DeclarationTag
        | BindingSemantics::OptimizedDeclarationTag
        | BindingSemantics::Contextual(_)
        | BindingSemantics::NonReactive
        | BindingSemantics::LegacyApiExport
        | BindingSemantics::Unresolved => false,
    }
}

pub(super) fn is_reactive_template(facts: &ExprFacts, ctx: &Ctx<'_, '_>) -> bool {
    if facts.has_await || facts.has_state_rune || needs_context(facts, ctx.reactivity) {
        return true;
    }
    if facts.has_store_ref || reads_legacy_props_object(facts) {
        return true;
    }

    match facts.top_level_form {
        TopLevelForm::Call => {
            if facts.has_runtime_root {
                return true;
            }
            facts
                .top_level_reads
                .iter()
                .any(|&sym| call_root_is_reactive(ctx, sym))
        }
        TopLevelForm::Member => facts.has_runtime_root || !facts.top_level_reads.is_empty(),
        TopLevelForm::Identifier
        | TopLevelForm::Assignment
        | TopLevelForm::Update
        | TopLevelForm::Other => facts
            .top_level_reads
            .iter()
            .any(|&sym| identifier_read_is_reactive(ctx, sym)),
    }
}

fn call_root_is_reactive(ctx: &Ctx<'_, '_>, sym: SymbolId) -> bool {
    if ctx.reactivity.binding_semantics(sym).is_maybe_reactive() {
        return true;
    }
    symbol_is_volatile(ctx, sym) || ctx.scoping.is_component_top_level_symbol(sym)
}

fn identifier_read_is_reactive(ctx: &Ctx<'_, '_>, sym: SymbolId) -> bool {
    if symbol_is_volatile(ctx, sym) {
        return true;
    }
    if ctx.reactivity.binding_semantics(sym).is_props() {
        return true;
    }
    ctx.has_class_state_fields
        && ctx.scoping.is_component_top_level_symbol(sym)
        && is_unified_plain_symbol(ctx.reactivity, sym)
}

pub(super) fn symbol_is_volatile(ctx: &Ctx<'_, '_>, sym: SymbolId) -> bool {
    if ctx.scoping.is_each_index_non_dynamic(sym) {
        return false;
    }
    match ctx.reactivity.binding_semantics(sym) {
        BindingSemantics::Contextual(ContextualBindingSemantics::LetDirectiveDirect) => false,
        BindingSemantics::MaybeReactive
        | BindingSemantics::State(_)
        | BindingSemantics::Prop(_)
        | BindingSemantics::LegacyBindableProp(_)
        | BindingSemantics::LegacyState(_)
        | BindingSemantics::Store(_)
        | BindingSemantics::Contextual(_)
        | BindingSemantics::RuntimeRune { .. } => true,
        BindingSemantics::Derived(_) => true,
        BindingSemantics::OptimizedDerived(_) => false,
        BindingSemantics::Const(_) | BindingSemantics::DeclarationTag => true,
        BindingSemantics::OptimizedConst(_) | BindingSemantics::OptimizedDeclarationTag => false,
        BindingSemantics::OptimizedRune(opt) if opt.proxy_init => true,
        BindingSemantics::NonReactive => {
            if !ctx.scoping.is_component_top_level_symbol(sym) {
                return true;
            }
            !symbol_read_is_static(ctx.value_evaluation, ctx.semantics, sym)
        }
        BindingSemantics::OptimizedRune(_) => {
            !matches!(ctx.value_evaluation.evaluation(sym), Evaluation::Known(_))
        }
        BindingSemantics::Unresolved => !ctx.scoping.is_component_top_level_symbol(sym),
        BindingSemantics::LegacyApiExport => false,
    }
}

fn reads_legacy_props_object(facts: &ExprFacts) -> bool {
    facts.reads_legacy_props || facts.reads_legacy_rest_props
}

fn is_unified_plain_symbol(reactivity: &ReactivitySemantics, sym_id: SymbolId) -> bool {
    match reactivity.binding_semantics(sym_id) {
        BindingSemantics::NonReactive
        | BindingSemantics::Const(_)
        | BindingSemantics::OptimizedConst(_)
        | BindingSemantics::DeclarationTag
        | BindingSemantics::OptimizedDeclarationTag => true,
        BindingSemantics::Prop(_)
        | BindingSemantics::State(_)
        | BindingSemantics::Derived(_)
        | BindingSemantics::OptimizedDerived(_)
        | BindingSemantics::OptimizedRune(_)
        | BindingSemantics::RuntimeRune { .. }
        | BindingSemantics::Store(_)
        | BindingSemantics::LegacyBindableProp(_)
        | BindingSemantics::LegacyState(_)
        | BindingSemantics::Contextual(_)
        | BindingSemantics::MaybeReactive
        | BindingSemantics::LegacyApiExport
        | BindingSemantics::Unresolved => false,
    }
}

fn call_has_dynamic_input(facts: &ExprFacts) -> bool {
    !facts.references.is_empty() || facts.has_impure_call || reads_legacy_props_object(facts)
}

pub(super) fn is_heavy(facts: &ExprFacts) -> bool {
    facts.has_call && call_has_dynamic_input(facts)
}

pub(super) fn volatility(reactive_gate: bool, facts: &ExprFacts) -> Volatility {
    if facts.has_await {
        Volatility::Asynchronous
    } else if is_heavy(facts) {
        Volatility::Heavy
    } else if reactive_gate {
        Volatility::Reactive
    } else {
        Volatility::Static
    }
}

pub(super) fn blockers(facts: &ExprFacts, blocker_data: &BlockerData) -> SmallVec<[u32; 2]> {
    let mut out: SmallVec<[u32; 2]> = SmallVec::new();
    for sym in &facts.references {
        if let Some(idx) = blocker_data.symbol_blocker(*sym)
            && !out.contains(&idx)
        {
            out.push(idx);
        }
    }
    out.sort_unstable();
    out
}

pub(super) fn volatile(
    facts: &ExprFacts,
    has_blockers: bool,
    is_reactive: bool,
    evaluation: &Evaluation,
    reactivity: &ReactivitySemantics,
) -> bool {
    if facts.has_await || has_blockers {
        return true;
    }
    if facts.has_call {
        return call_has_dynamic_input(facts) || facts.has_state_rune;
    }
    if matches!(evaluation, Evaluation::Known(_)) && !references_optimized_rune(facts, reactivity) {
        return false;
    }
    is_reactive
}

pub(super) fn volatile_element_attr(
    is_reactive: bool,
    references: &[SymbolId],
    ctx: &Ctx<'_, '_>,
) -> bool {
    if is_reactive && references.is_empty() {
        return true;
    }
    references
        .iter()
        .any(|&sym| attr_read_is_volatile(ctx, sym))
}

fn attr_read_is_volatile(ctx: &Ctx<'_, '_>, sym: SymbolId) -> bool {
    let non_source_prop = match ctx.reactivity.binding_semantics(sym) {
        BindingSemantics::Prop(prop) => match &prop.kind {
            PropBindingKind::NonSource => true,
            PropBindingKind::Identifier
            | PropBindingKind::Source { .. }
            | PropBindingKind::Rest => false,
        },
        BindingSemantics::State(_)
        | BindingSemantics::Derived(_)
        | BindingSemantics::OptimizedDerived(_)
        | BindingSemantics::OptimizedRune(_)
        | BindingSemantics::RuntimeRune { .. }
        | BindingSemantics::Store(_)
        | BindingSemantics::LegacyBindableProp(_)
        | BindingSemantics::LegacyState(_)
        | BindingSemantics::Const(_)
        | BindingSemantics::OptimizedConst(_)
        | BindingSemantics::DeclarationTag
        | BindingSemantics::OptimizedDeclarationTag
        | BindingSemantics::Contextual(_)
        | BindingSemantics::MaybeReactive
        | BindingSemantics::NonReactive
        | BindingSemantics::LegacyApiExport
        | BindingSemantics::Unresolved => false,
    };
    non_source_prop || symbol_is_volatile(ctx, sym)
}

pub(super) fn volatile_component_name(
    expr: &Expression<'_>,
    uses_runes: bool,
    scoping: &ComponentScoping,
    reactivity: &ReactivitySemantics,
) -> bool {
    if !uses_runes {
        return false;
    }
    if matches!(
        expr.get_inner_expression(),
        Expression::StaticMemberExpression(_)
    ) {
        return true;
    }
    let Some(ref_id) = expression_root_reference_id(expr) else {
        return false;
    };
    let Some(sym_id) = scoping.symbol_for_reference(ref_id) else {
        return false;
    };
    is_reactive_component_binding(reactivity, sym_id)
}

fn is_reactive_component_binding(reactivity: &ReactivitySemantics, sym: SymbolId) -> bool {
    match reactivity.binding_semantics(sym) {
        BindingSemantics::MaybeReactive
        | BindingSemantics::NonReactive
        | BindingSemantics::Unresolved => false,
        BindingSemantics::State(_)
        | BindingSemantics::Derived(_)
        | BindingSemantics::OptimizedDerived(_)
        | BindingSemantics::LegacyState(_)
        | BindingSemantics::Prop(_)
        | BindingSemantics::LegacyBindableProp(_)
        | BindingSemantics::Store(_)
        | BindingSemantics::Contextual(_)
        | BindingSemantics::OptimizedRune(_)
        | BindingSemantics::Const(_)
        | BindingSemantics::OptimizedConst(_)
        | BindingSemantics::DeclarationTag
        | BindingSemantics::OptimizedDeclarationTag
        | BindingSemantics::RuntimeRune { .. }
        | BindingSemantics::LegacyApiExport => true,
    }
}

fn references_optimized_rune(facts: &ExprFacts, reactivity: &ReactivitySemantics) -> bool {
    facts
        .references
        .iter()
        .any(|&sym| reactivity.binding_semantics(sym).is_optimized_rune())
}

pub(super) fn legacy_wrap(
    uses_legacy_coarse_wrap: bool,
    facts: &ExprFacts,
    has_context_member_root: bool,
) -> LegacyWrap {
    if !uses_legacy_coarse_wrap {
        return LegacyWrap::None;
    }
    let call_is_reactive =
        facts.has_impure_call || (facts.has_call && !facts.references.is_empty());
    let needs_coarse = call_is_reactive
        || facts.has_member
        || matches!(
            facts.top_level_form,
            TopLevelForm::Member | TopLevelForm::Assignment | TopLevelForm::Update
        )
        || has_context_member_root;
    if !needs_coarse {
        return LegacyWrap::None;
    }
    let carrier = synthetic_props_carrier(facts.reads_legacy_props, facts.reads_legacy_rest_props);
    match carrier {
        None => LegacyWrap::CoarseWrap,
        Some(c) => LegacyWrap::CoarseAndSynthetic(c),
    }
}

pub(super) fn synthetic_props_carrier(
    reads_props: bool,
    reads_rest_props: bool,
) -> Option<SyntheticPropsCarrier> {
    match (reads_props, reads_rest_props) {
        (false, false) => None,
        (true, false) => Some(SyntheticPropsCarrier::SanitizedProps),
        (false, true) => Some(SyntheticPropsCarrier::RestProps),
        (true, true) => Some(SyntheticPropsCarrier::Both),
    }
}
