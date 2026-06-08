use super::super::Evaluation;
use super::super::data::{LegacyWrap, SyntheticPropsCarrier, Volatility};
use super::collector::{ExprFacts, TopLevelForm};
use crate::reactivity_semantics::builder_v2::expression_root_reference_id;
use crate::reactivity_semantics::data::ReactivitySemantics;
use crate::scope::{ComponentScoping, SymbolId};
use crate::types::data::{BindingSemantics, BlockerData, PropBindingKind, PropBindingSemantics};
use oxc_ast::ast::Expression;
use smallvec::SmallVec;

pub(super) fn needs_context(facts: &ExprFacts, reactivity: &ReactivitySemantics) -> bool {
    if !matches!(
        facts.top_level_form,
        TopLevelForm::Member | TopLevelForm::Call
    ) {
        return false;
    }
    facts.references.iter().any(|&sym| {
        matches!(
            reactivity.binding_semantics(sym),
            BindingSemantics::MaybeReactive
                | BindingSemantics::Prop(PropBindingSemantics {
                    kind: PropBindingKind::Source { .. } | PropBindingKind::NonSource,
                    ..
                })
        )
    })
}

pub(super) fn is_reactive_template(
    facts: &ExprFacts,
    scoping: &ComponentScoping,
    reactivity: &ReactivitySemantics,
    has_class_state_fields: bool,
) -> bool {
    if facts.has_await || facts.has_state_rune || needs_context(facts, reactivity) {
        return true;
    }

    if matches!(facts.top_level_form, TopLevelForm::Call) {
        return facts.has_runtime_root
            || facts.has_store_ref
            || facts.references.iter().any(|&sym| {
                let semantics = reactivity.binding_semantics(sym);
                if matches!(semantics, BindingSemantics::MaybeReactive) {
                    return true;
                }
                reactivity.needs_effect(scoping, sym) || scoping.is_component_top_level_symbol(sym)
            });
    }

    if matches!(facts.top_level_form, TopLevelForm::Member) {
        return facts.has_runtime_root || facts.has_store_ref || !facts.references.is_empty();
    }

    if facts.has_store_ref {
        return true;
    }
    facts.references.iter().any(|&sym| {
        if reactivity.needs_effect(scoping, sym) {
            return true;
        }
        if is_unified_prop_source(reactivity, sym) {
            return true;
        }
        if has_class_state_fields
            && scoping.is_component_top_level_symbol(sym)
            && is_unified_plain_symbol(reactivity, sym)
        {
            return true;
        }
        false
    })
}

fn is_unified_prop_source(reactivity: &ReactivitySemantics, sym_id: SymbolId) -> bool {
    matches!(
        reactivity.binding_semantics(sym_id),
        BindingSemantics::Prop(PropBindingSemantics {
            kind: PropBindingKind::Source { .. },
            ..
        })
    )
}

fn is_unified_plain_symbol(reactivity: &ReactivitySemantics, sym_id: SymbolId) -> bool {
    matches!(
        reactivity.binding_semantics(sym_id),
        BindingSemantics::NonReactive | BindingSemantics::Const(_)
    )
}

pub(super) fn is_heavy(facts: &ExprFacts) -> bool {
    facts.has_call && (!facts.references.is_empty() || facts.has_impure_call)
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
        let dynamic = !facts.references.is_empty() || facts.has_impure_call;
        return dynamic || facts.has_state_rune;
    }
    if matches!(evaluation, Evaluation::Known(_)) && !references_optimized_rune(facts, reactivity) {
        return false;
    }
    is_reactive
}

pub(super) fn volatile_element_attr(
    is_reactive: bool,
    references: &[SymbolId],
    scoping: &ComponentScoping,
    reactivity: &ReactivitySemantics,
) -> bool {
    if is_reactive && references.is_empty() {
        return true;
    }
    references.iter().any(|&sym| {
        matches!(
            reactivity.binding_semantics(sym),
            BindingSemantics::Prop(PropBindingSemantics {
                kind: PropBindingKind::NonSource,
                ..
            })
        ) || reactivity.needs_effect(scoping, sym)
    })
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
        | BindingSemantics::RuntimeRune { .. }
        | BindingSemantics::LegacyApiExport => true,
    }
}

fn references_optimized_rune(facts: &ExprFacts, reactivity: &ReactivitySemantics) -> bool {
    facts.references.iter().any(|&sym| {
        matches!(
            reactivity.binding_semantics(sym),
            BindingSemantics::OptimizedRune(_),
        )
    })
}

pub(super) fn legacy_wrap(
    uses_legacy_coarse_wrap: bool,
    facts: &ExprFacts,
    has_context_member_root: bool,
) -> LegacyWrap {
    if !uses_legacy_coarse_wrap {
        return LegacyWrap::None;
    }
    let needs_coarse = facts.has_call
        || facts.has_member
        || matches!(
            facts.top_level_form,
            TopLevelForm::Member | TopLevelForm::Assignment | TopLevelForm::Update
        )
        || has_context_member_root;
    let carrier = synthetic_props_carrier(facts.reads_legacy_props, facts.reads_legacy_rest_props);
    match (needs_coarse, carrier) {
        (false, None) => LegacyWrap::None,
        (true, None) => LegacyWrap::CoarseWrap,
        (false, Some(c)) => LegacyWrap::Synthetic(c),
        (true, Some(c)) => LegacyWrap::CoarseAndSynthetic(c),
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
