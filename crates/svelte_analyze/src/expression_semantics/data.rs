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
    pub volatility: Volatility,
    pub suspension: Suspension,
    pub evaluation: Evaluation,
    pub declared_evaluation: Evaluation,
    pub blockers: SmallVec<[u32; 2]>,
    pub legacy_wrap: LegacyWrap,
    pub references: SmallVec<[SymbolId; 2]>,
    pub evaluated_reads: SmallVec<[SymbolId; 2]>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Suspension {
    #[default]
    None,
    Outermost,
    Interleaved,
}

impl Suspension {
    pub fn is_outermost(&self) -> bool {
        match self {
            Suspension::Outermost => true,
            Suspension::None | Suspension::Interleaved => false,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum Volatility {
    #[default]
    Static,
    Reactive,
    Heavy,
    Asynchronous,
}

impl Volatility {
    pub fn is_volatile(&self) -> bool {
        match self {
            Volatility::Static => false,
            Volatility::Reactive | Volatility::Heavy | Volatility::Asynchronous => true,
        }
    }

    pub fn is_asynchronous(&self) -> bool {
        match self {
            Volatility::Static | Volatility::Reactive | Volatility::Heavy => false,
            Volatility::Asynchronous => true,
        }
    }
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
