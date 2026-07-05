use rustc_hash::{FxHashMap, FxHashSet};
use smallvec::SmallVec;
use svelte_ast::{ConcatPart, NodeId, StyleDirective};
use svelte_span::Span;

use super::node_table::{NodeBitSet, NodeTable};
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
mod fragment_facts;
mod fragment_namespaces;
mod ignore;
mod pickled_awaits;
mod rich_content_facts;
mod runtime;
mod template_data;
mod template_element_index;
pub(crate) mod template_topology;

pub use crate::reactivity_semantics::data::{
    BindingSemantics, CarrierMemberReadSemantics, ClassFieldDerivedSemantics, ClassFieldSemantics,
    ClassFieldStateSemantics, ConstBindingSemantics, ContextualBindingSemantics,
    ContextualReadKind, ContextualReadSemantics, DeclaratorGroup, DeclaratorSemantics,
    DerivedDeclarationSemantics, DerivedEmit, DerivedKind, DerivedSource, EachIndexStrategy,
    EachItemStrategy, LegacyBindablePropSemantics, LegacyDependency, LegacySummary,
    OptimizedRuneSemantics, PropBindingKind, PropBindingSemantics, PropDefaultKind, PropEmitMode,
    PropReferenceSemantics, PropsSummary, ReactivitySemantics, ReactivitySummary,
    ReferenceSemantics, RuntimeRuneKind, SignalReferenceKind, SnippetParamStrategy,
    StateDeclarationSemantics, StateKind, StoreBindingSemantics,
};
pub use analysis::{
    AnalysisData, ApiExport, BlockAnalysis, ElementAnalysis, OutputData, ScriptAnalysis,
    TemplateAnalysis,
};
pub use async_data::{AsyncStmtMeta, BlockerData};
pub use attr_index::AttrIndex;
pub use codegen_view::CodegenView;
pub use css::CssAnalysis;
pub use directive_modifier_flags::EventModifier;
pub use element_facts::{ElementFacts, ElementFactsEntry, NamespaceKind};
pub use elements::{
    ClassDirectiveInfo, ComponentBindMode, ComponentCssProp, ComponentCssPropValue,
    ComponentPropInfo, ComponentPropKind, ElementFlags, EventHandlerMode, LegacyDefaultSlot,
};
pub use fragment_facts::{FragmentFacts, FragmentFactsEntry};
pub use fragment_namespaces::FragmentNamespaces;
pub use ignore::IgnoreData;
pub use pickled_awaits::PickledAwaits;
pub use rich_content_facts::{RichContentFacts, RichContentFactsEntry, RichContentParentKind};
pub use runtime::LegacyInit;
pub use runtime::RuntimeInfo;
pub use template_data::{
    BindHostKind, BindPropertyKind, BindSemanticsData, BindSource, BindTargetSemantics,
    ContentEditableKind, DocumentBindKind, ElementSizeKind, ImageNaturalSizeKind, MediaBindKind,
    ResizeObserverKind, SnippetData, TemplateSemanticsData, TitleElementData, WindowBindKind,
    binding_group_name,
};
pub use template_element_index::{TemplateElementEntry, TemplateElementIndex};
pub use template_topology::{ParentKind, ParentRef, TemplateTopology};
