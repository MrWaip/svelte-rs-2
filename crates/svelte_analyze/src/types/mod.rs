pub mod data;
pub(crate) mod markers;
pub mod node_table;
pub mod script;

pub use data::{
    AnalysisData, AsyncStmtMeta, AwaitBindingData, AwaitBindingInfo, BindSemanticsData,
    BlockAnalysis, BlockerData, ClassDirectiveInfo, CodegenView, ComponentBindMode,
    ComponentPropInfo, ComponentPropKind, ConstTagData, ContentStrategy, DebugTagData,
    DestructureKind, EachContextIndex, ElementAnalysis, ElementFlags, EventHandlerMode, ExprDeps,
    ExprRole, ExprSite, ExpressionInfo, ExpressionKind, FragmentData, FragmentFacts,
    FragmentFactsEntry, FragmentItem, FragmentKey, IgnoreData, LoweredFragment, LoweredTextPart,
    OutputPlanData, PropAnalysis, PropsAnalysis, RenderTagArgPlan, RenderTagCalleeMode,
    RenderTagPlan, RuntimePlan, ScriptAnalysis, SnippetData, TemplateAnalysis,
    TemplateSemanticsData, TitleElementData,
};
pub use node_table::{NodeBitSet, NodeTable};
pub use script::{
    DeclarationInfo, DeclarationKind, ExportInfo, PropInfo, PropsDeclaration, RuneKind, ScriptInfo,
};
