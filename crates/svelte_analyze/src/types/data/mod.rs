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
mod codegen_view;
mod elements;
mod expr;
mod fragments;
mod ignore;
mod props;
mod render;
mod runtime;
mod template_data;

pub use analysis::AnalysisData;
pub use async_data::{AsyncStmtMeta, BlockerData};
pub use codegen_view::CodegenView;
pub use elements::{
    ClassDirectiveInfo, ComponentBindMode, ComponentPropInfo, ComponentPropKind, ElementFlags,
    EventHandlerMode,
};
pub use expr::{
    AwaitBindingInfo, DestructureKind, ExprDeps, ExprSite, ExpressionInfo, ExpressionKind,
};
pub use fragments::{
    ContentStrategy, FragmentData, FragmentItem, FragmentKey, FragmentKeyExt, LoweredFragment,
    LoweredTextPart,
};
pub use ignore::IgnoreData;
pub use props::{PropAnalysis, PropsAnalysis};
pub use render::{RenderTagArgPlan, RenderTagCalleeMode, RenderTagPlan};
pub use runtime::RuntimePlan;
pub use template_data::{
    AwaitBindingData, BindSemanticsData, ConstTagData, DebugTagData, EachBlockData, SnippetData,
    TemplateSemanticsData, TitleElementData,
};
