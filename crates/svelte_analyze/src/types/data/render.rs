use super::*;

#[deprecated(note = "use BlockSemantics::Render / block_semantics(id) instead")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderTagCalleeMode {
    Direct,
    Chain,
    DynamicRegular,
    DynamicChain,
}

#[allow(deprecated)]
impl RenderTagCalleeMode {
    pub fn is_dynamic(self) -> bool {
        matches!(self, Self::DynamicRegular | Self::DynamicChain)
    }
    pub fn is_chain(self) -> bool {
        matches!(self, Self::Chain | Self::DynamicChain)
    }
}

#[deprecated(note = "use BlockSemantics::Render / block_semantics(id) instead")]
#[derive(Debug, Clone)]
#[allow(deprecated)]
pub struct RenderTagPlan {
    pub callee_mode: RenderTagCalleeMode,
    pub arg_plans: Vec<RenderTagArgPlan>,
}

#[deprecated(note = "use BlockSemantics::Render / block_semantics(id) instead")]
#[derive(Debug, Clone)]
pub struct RenderTagArgPlan {
    pub info: ExpressionInfo,
    pub prop_source: Option<SymbolId>,
}
