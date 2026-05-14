use super::collector::{ExprFacts, TopLevelForm};
use super::super::data::{ExprKind, LegacyWrap};
use super::super::Evaluation;
use crate::reactivity_semantics::data::ReactivitySemantics;
use crate::scope::{ComponentScoping, SymbolId};
use crate::types::data::{
    BindingSemantics, BlockerData, ConstBindingSemantics, PropBindingKind, PropBindingSemantics,
};
use smallvec::SmallVec;

pub(super) fn needs_context(
    facts: &ExprFacts,
    reactivity: &ReactivitySemantics,
) -> bool {
    if !matches!(facts.top_level_form, TopLevelForm::Member | TopLevelForm::Call) {
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

pub(super) fn is_dynamic_template(
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
                is_symbol_dynamic(scoping, reactivity, sym)
                    || scoping.is_component_top_level_symbol(sym)
            });
    }

    if matches!(facts.top_level_form, TopLevelForm::Member) {
        return facts.has_runtime_root
            || facts.has_store_ref
            || !facts.references.is_empty();
    }

    if facts.has_store_ref {
        return true;
    }
    facts.references.iter().any(|&sym| {
        if is_symbol_dynamic(scoping, reactivity, sym) {
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

fn is_symbol_dynamic(
    scoping: &ComponentScoping,
    reactivity: &ReactivitySemantics,
    sym_id: SymbolId,
) -> bool {
    if scoping.is_each_index_non_dynamic(sym_id) {
        return false;
    }
    match reactivity.binding_semantics(sym_id) {
        BindingSemantics::MaybeReactive
        | BindingSemantics::State(_)
        | BindingSemantics::Prop(_)
        | BindingSemantics::LegacyBindableProp(_)
        | BindingSemantics::LegacyState(_)
        | BindingSemantics::Store(_)
        | BindingSemantics::Contextual(_)
        | BindingSemantics::RuntimeRune { .. } => true,
        BindingSemantics::Derived(d) => d.reactive,
        BindingSemantics::Const(ConstBindingSemantics::ConstTag { reactive, .. }) => reactive,
        BindingSemantics::OptimizedRune(opt) if opt.proxy_init => true,
        BindingSemantics::NonReactive => {
            if !scoping.is_component_top_level_symbol(sym_id) {
                return true;
            }
            !scoping.is_init_known(sym_id)
        }
        BindingSemantics::Unresolved | BindingSemantics::OptimizedRune(_) => {
            !scoping.is_component_top_level_symbol(sym_id)
        }
    }
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

pub(super) fn kind(
    facts: &ExprFacts,
    has_blockers: bool,
    is_dynamic: bool,
    evaluation: &Evaluation,
) -> ExprKind {
    if facts.has_await || has_blockers {
        ExprKind::Async {
            has_await: facts.has_await,
        }
    } else if matches!(evaluation, Evaluation::Known(_)) {
        ExprKind::KnownLiteral
    } else if facts.has_call {
        ExprKind::Call
    } else if matches!(
        facts.top_level_form,
        TopLevelForm::Identifier | TopLevelForm::Member,
    ) {
        ExprKind::SimpleRead { reactive: is_dynamic }
    } else {
        ExprKind::Computed { reactive: is_dynamic }
    }
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
    let uses_sanitized = facts.uses_legacy_sanitized_props;
    match (needs_coarse, uses_sanitized) {
        (false, false) => LegacyWrap::None,
        (true, false) => LegacyWrap::CoarseWrap,
        (false, true) => LegacyWrap::SanitizedProps,
        (true, true) => LegacyWrap::CoarseAndSanitized,
    }
}

