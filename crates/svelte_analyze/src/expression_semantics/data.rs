use compact_str::CompactString;
use smallvec::SmallVec;
use svelte_component_semantics::SymbolId;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum ExpressionSemantics {
    #[default]
    NonSpecial,
    Expression(ExpressionData),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExpressionData {
    pub kind: ExprKind,
    pub blockers: SmallVec<[u32; 2]>,
    pub legacy_wrap: LegacyWrap,
    pub memoization: Memoization,
    pub references: SmallVec<[SymbolId; 2]>,
}

impl ExpressionData {
    pub fn has_await(&self) -> bool {
        matches!(self.kind, ExprKind::Async { has_await: true, .. })
    }

    pub fn is_dynamic(&self) -> bool {
        matches!(self.kind, ExprKind::Dynamic | ExprKind::Async { .. })
    }

    pub fn needs_effect(&self) -> bool {
        self.is_dynamic() || !self.blockers.is_empty()
    }

    pub fn needs_node_memo(&self) -> bool {
        !matches!(self.memoization, Memoization::None)
            && (self.has_await() || !self.references.is_empty())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExprKind {
    Folded(CompactString),
    Static,
    Dynamic,
    Async { has_await: bool, is_pickled: bool },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LegacyWrap {
    None,
    CoarseWrap,
    SanitizedProps,
    CoarseAndSanitized,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memoization {
    None,
    SyncMemo,
    AsyncMemo,
}
