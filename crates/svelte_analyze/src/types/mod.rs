pub mod data;
pub(crate) mod markers;
pub mod node_table;
pub mod script;

pub use data::{
    AnalysisData, AsyncStmtMeta, BindHostKind, BindPropertyKind, BindSemanticsData,
    BindTargetSemantics, BlockAnalysis, BlockerData, ClassDirectiveInfo, CodegenView,
    ComponentBindMode, ComponentPropInfo, ComponentPropKind, ConstTagData, ContentEditableKind,
    DebugTagData, DocumentBindKind, ElementAnalysis, ElementFlags, ElementSizeKind,
    EventHandlerMode, ExprDeps, ExprRole, ExprSite, ExpressionInfo, ExpressionKind, FragmentFacts,
    FragmentFactsEntry, IgnoreData, ImageNaturalSizeKind, MediaBindKind, OutputPlanData,
    ResizeObserverKind, RuntimePlan, ScriptAnalysis, SnippetData, TemplateAnalysis,
    TemplateSemanticsData, TitleElementData, WindowBindKind,
};
pub use node_table::{NodeBitSet, NodeTable};
pub use script::{
    DeclarationInfo, DeclarationKind, ExportInfo, PropInfo, PropsDeclaration, RuneKind, ScriptInfo,
};
