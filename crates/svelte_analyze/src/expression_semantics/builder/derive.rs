use super::collector::{ExprFacts, TopLevelShape};
use super::super::data::{ExprKind, LegacyWrap, Memoization};
use crate::types::data::BlockerData;
use smallvec::SmallVec;

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

pub(super) fn kind(facts: &ExprFacts, has_blockers: bool, is_dynamic: bool) -> ExprKind {
    if facts.has_await || has_blockers {
        ExprKind::Async {
            has_await: facts.has_await,
            is_pickled: facts.any_pickled_await,
        }
    } else if is_dynamic {
        ExprKind::Dynamic
    } else {
        ExprKind::Static
    }
}

pub(super) fn memoization(facts: &ExprFacts) -> Memoization {
    if facts.has_await {
        Memoization::AsyncMemo
    } else if facts.has_call && !facts.references.is_empty() {
        Memoization::SyncMemo
    } else {
        Memoization::None
    }
}

pub(super) fn legacy_wrap(runes: bool, facts: &ExprFacts) -> LegacyWrap {
    if runes {
        return LegacyWrap::None;
    }
    let needs_coarse = facts.has_call
        || matches!(
            facts.top_level_shape,
            TopLevelShape::Member | TopLevelShape::Assignment | TopLevelShape::Update
        );
    let uses_sanitized = facts.uses_legacy_sanitized_props;
    match (needs_coarse, uses_sanitized) {
        (false, false) => LegacyWrap::None,
        (true, false) => LegacyWrap::CoarseWrap,
        (false, true) => LegacyWrap::SanitizedProps,
        (true, true) => LegacyWrap::CoarseAndSanitized,
    }
}
