use smallvec::SmallVec;
use svelte_component_semantics::SymbolId;

pub use crate::value_evaluation::{Evaluation, KnownValue, ValueClass};

#[derive(Clone, Debug, PartialEq, Default)]
pub enum ExpressionSemantics {
    #[default]
    NonSpecial,
    Expression(ExpressionData),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExpressionData {
    pub kind: ExprKind,
    pub evaluation: Evaluation,
    pub blockers: SmallVec<[u32; 2]>,
    pub legacy_wrap: LegacyWrap,
    pub references: SmallVec<[SymbolId; 2]>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExprKind {
    KnownLiteral,
    SimpleRead { reactive: bool },
    Computed { reactive: bool },
    Call { dynamic: bool },
    Async { has_await: bool },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LegacyWrap {
    None,
    CoarseWrap,
    Synthetic(SyntheticPropsCarrier),
    CoarseAndSynthetic(SyntheticPropsCarrier),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SyntheticPropsCarrier {
    SanitizedProps,
    RestProps,
    Both,
}
