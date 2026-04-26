use compact_str::CompactString;
use rustc_hash::{FxHashMap, FxHashSet};
use smallvec::SmallVec;
use svelte_ast::{ConcatPart, NodeId, StyleDirective};
use svelte_span::Span;

use super::node_table::{NodeBitSet, NodeTable};
use super::script::{ExportInfo, ScriptInfo};
use crate::scope::{ComponentScoping, SymbolId};

pub use svelte_parser::JsAst;

mod analysis;
mod async_data;
pub(crate) mod attr_index;
mod codegen_view;
mod css;
mod directive_modifier_flags;
mod element_facts;
mod elements;
mod expr;
mod fragment_facts;
mod fragment_namespaces;
mod ignore;
mod pickled_await_offsets;
mod proxy_state_inits;
mod rich_content_facts;
mod runtime;
mod script_rune_calls;
mod template_data;
mod template_element_index;
pub(crate) mod template_topology;

pub use crate::reactivity_semantics::data::{
    CarrierMemberReadSemantics, ConstDeclarationSemantics, ContextualDeclarationSemantics,
    ContextualReadKind, ContextualReadSemantics, DeclarationSemantics, DerivedDeclarationSemantics,
    DerivedKind, DerivedLowering, EachIndexStrategy, EachItemStrategy, OptimizedRuneSemantics,
    PropDeclarationKind, PropDeclarationSemantics, PropDefaultLowering, PropLoweringMode,
    PropReferenceSemantics, PropsObjectPropertySemantics, ReactivitySemantics, ReferenceSemantics,
    RuntimeRuneKind, SignalReferenceKind, SnippetParamStrategy, StateBindingSemantics,
    StateDeclarationSemantics, StateKind, StoreDeclarationSemantics,
};
pub use analysis::{
    AnalysisData, BlockAnalysis, ElementAnalysis, OutputPlanData, ScriptAnalysis, TemplateAnalysis,
};
pub use async_data::{AsyncStmtMeta, BlockerData};
pub use attr_index::AttrIndex;
pub use codegen_view::CodegenView;
pub use css::CssAnalysis;
pub use directive_modifier_flags::{DirectiveModifierFlags, EventModifier};
pub use element_facts::{ElementFacts, ElementFactsEntry, NamespaceKind};
pub use elements::{
    ClassDirectiveInfo, ComponentBindMode, ComponentPropInfo, ComponentPropKind, ElementFlags,
    EventHandlerMode,
};
pub use expr::{ExprDeps, ExprRole, ExprSite, ExpressionInfo, ExpressionKind};
pub use fragment_facts::{FragmentFacts, FragmentFactsEntry};
pub use fragment_namespaces::FragmentNamespaces;
pub use ignore::IgnoreData;
pub use pickled_await_offsets::PickledAwaitOffsets;
pub use proxy_state_inits::ProxyStateInits;
pub use rich_content_facts::{RichContentFacts, RichContentFactsEntry, RichContentParentKind};
pub use runtime::RuntimePlan;
pub use script_rune_calls::ScriptRuneCalls;
pub use template_data::{
    BindHostKind, BindPropertyKind, BindSemanticsData, BindTargetSemantics, ConstTagData,
    ContentEditableKind, DebugTagData, DocumentBindKind, ElementSizeKind, ImageNaturalSizeKind,
    MediaBindKind, ResizeObserverKind, SnippetData, TemplateSemanticsData, TitleElementData,
    WindowBindKind,
};
pub use template_element_index::{TemplateElementEntry, TemplateElementIndex};
pub use template_topology::{ParentKind, ParentRef, TemplateTopology};
