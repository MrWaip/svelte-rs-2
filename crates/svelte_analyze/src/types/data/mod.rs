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
mod each_context_index;
mod element_facts;
mod elements;
mod expr;
mod fragment_facts;
mod fragments;
mod ignore;
mod props;
mod render;
mod rich_content_facts;
mod runtime;
mod template_data;
mod template_element_index;
pub(crate) mod template_topology;

pub use analysis::AnalysisData;
pub use async_data::{AsyncStmtMeta, BlockerData};
pub use attr_index::AttrIndex;
pub use codegen_view::CodegenView;
pub use css::CssAnalysis;
pub use each_context_index::EachContextIndex;
pub use element_facts::{ElementFacts, ElementFactsEntry};
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
pub use props::{PropAnalysis, PropsAnalysis};
pub use render::{RenderTagArgPlan, RenderTagCalleeMode, RenderTagPlan};
pub use rich_content_facts::{RichContentFacts, RichContentFactsEntry, RichContentParentKind};
pub use runtime::RuntimePlan;
pub use template_data::{
    AwaitBindingData, BindSemanticsData, ConstTagData, DebugTagData, SnippetData,
    TemplateSemanticsData, TitleElementData,
};
pub use template_element_index::{TemplateElementEntry, TemplateElementIndex};
pub use template_topology::{ParentKind, ParentRef, TemplateTopology};
