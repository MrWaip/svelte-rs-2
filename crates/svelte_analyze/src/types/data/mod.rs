use compact_str::CompactString;
use rustc_hash::{FxHashMap, FxHashSet};
use smallvec::SmallVec;
use svelte_ast::{ConcatPart, NodeId, StyleDirective};
use svelte_span::Span;

use super::node_table::{NodeBitSet, NodeTable};
use super::script::{ExportInfo, ScriptInfo};
use crate::scope::{ComponentScoping, SymbolId};

pub use svelte_parser::{ExprHandle, ParserResult, StmtHandle};

mod analysis;
mod async_data;
pub(crate) mod attr_index;
mod codegen_view;
mod css;
mod directive_modifier_flags;
mod each_context_index;
mod element_facts;
mod elements;
mod expr;
mod fragment_facts;
mod fragments;
mod ignore;
mod pickled_await_offsets;
mod props;
mod proxy_state_inits;
mod render;
mod rich_content_facts;
mod runtime;
mod script_rune_calls;
mod template_data;
mod template_element_index;
pub(crate) mod template_topology;

pub use analysis::{
    AnalysisData, BlockAnalysis, ElementAnalysis, OutputPlanData, ScriptAnalysis, TemplateAnalysis,
};
pub use async_data::{AsyncStmtMeta, BlockerData};
pub use attr_index::AttrIndex;
pub use codegen_view::CodegenView;
pub use css::CssAnalysis;
pub use directive_modifier_flags::{DirectiveModifierFlags, EventModifier};
pub use each_context_index::EachContextIndex;
pub use element_facts::{ElementFacts, ElementFactsEntry, NamespaceKind};
pub use elements::{
    ClassDirectiveInfo, ComponentBindMode, ComponentPropInfo, ComponentPropKind, ElementFlags,
    EventHandlerMode,
};
pub use expr::{
    AwaitBindingInfo, DestructureKind, ExprDeps, ExprSite, ExpressionInfo, ExpressionKind,
};
pub use fragment_facts::{FragmentFacts, FragmentFactsEntry};
pub use fragments::{
    ContentStrategy, FragmentData, FragmentItem, FragmentKey, FragmentKeyExt, LoweredFragment,
    LoweredTextPart,
};
pub use ignore::IgnoreData;
pub use pickled_await_offsets::PickledAwaitOffsets;
pub use props::{PropAnalysis, PropsAnalysis};
pub use proxy_state_inits::ProxyStateInits;
pub use render::{RenderTagArgPlan, RenderTagCalleeMode, RenderTagPlan};
pub use rich_content_facts::{RichContentFacts, RichContentFactsEntry, RichContentParentKind};
pub use runtime::RuntimePlan;
pub use script_rune_calls::ScriptRuneCalls;
pub use template_data::{
    AwaitBindingData, BindSemanticsData, ConstTagData, DebugTagData, SnippetData,
    TemplateSemanticsData, TitleElementData,
};
pub use template_element_index::{TemplateElementEntry, TemplateElementIndex};
pub use template_topology::{ParentKind, ParentRef, TemplateTopology};
