use crate::expression_semantics::LegacyWrap;
use crate::scope::SymbolId;
use crate::types::data::{
    ContentEditableKind, DocumentBindKind, ElementSizeKind, EventModifier, ImageNaturalSizeKind,
    MediaBindKind, ResizeObserverKind, WindowBindKind,
};
use compact_str::CompactString;
use smallvec::SmallVec;
use svelte_ast::{NodeId, OxcNodeId};

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum AttributeSemantics {
    #[default]
    NonSpecial,

    ElementBind(ElementBindSemantics),
    WindowBind(WindowBindSemantics),
    DocumentBind(DocumentBindSemantics),
    ComponentBind(ComponentBindSemantics),

    Event(EventSemantics),
    ComponentProp(ComponentPropSemantics),
    SvelteComponentThis(SvelteComponentThisSemantics),
    ComponentSpread(ComponentSpreadSemantics),
    ComponentAttach(ComponentAttachSemantics),
    BoundaryProp(BoundaryPropSemantics),
    HtmlConcat(HtmlConcatSemantics),
    MustBeProperty(MustBePropertySemantics),
    SpecialValueAttr(SpecialValueSemantics),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SpecialValueSemantics {
    pub kind: SpecialValueKind,
    pub defined: bool,
    pub volatile: bool,
    pub concat: Option<HtmlConcatSemantics>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpecialValueKind {
    Select,
    Option,
    InputBindGroup,
    InputBindChecked,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MustBePropertySemantics {
    pub property: CompactString,
    pub value: MustBePropertyValue,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MustBePropertyValue {
    BoolTrue,
    Str(CompactString),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HtmlConcatSemantics {
    pub effect: TemplateEffect,
    pub parts: SmallVec<[HtmlConcatPart; 4]>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TemplateEffect {
    None,
    Sync,
    Async,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HtmlConcatPart {
    StaticText(CompactString),
    Inline {
        part_id: NodeId,
        defined: bool,
        wrap: LegacyWrap,
    },
    SyncMemoSlot {
        index: u8,
        part_id: NodeId,
        defined: bool,
        wrap: LegacyWrap,
    },
    AsyncMemoSlot {
        index: u8,
        part_id: NodeId,
        defined: bool,
        wrap: LegacyWrap,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ComponentSpreadSemantics {
    pub emit: ComponentSpreadEmit,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComponentSpreadEmit {
    Inline,
    Thunk,
    MemoThunk,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ComponentAttachSemantics {
    pub emit: ComponentAttachEmit,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComponentAttachEmit {
    Inline,
    Wrapped,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BoundaryPropSemantics {
    pub emit: BoundaryPropEmit,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoundaryPropEmit {
    KeyValue,
    Getter,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SvelteComponentThisSemantics {
    pub expr_id: OxcNodeId,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ComponentPropSemantics {
    Expression(ComponentPropExpressionSemantics),
    Concat(ComponentPropConcatSemantics),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ComponentPropExpressionSemantics {
    pub memo: ComponentPropMemo,
    pub shorthand: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ComponentPropConcatSemantics {
    pub memo: ComponentPropMemo,
    pub plan: SmallVec<[ConcatPartEmit; 4]>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConcatPartEmit {
    Static,
    Inline,
    HoistDerived,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComponentPropMemo {
    Inline,
    Getter,
    Derived,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EventSemantics {
    pub modifiers: EventModifier,
    pub emit: EventEmit,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EventEmit {
    HtmlDelegated {
        handler: HandlerEmit,
    },
    HtmlDirect {
        capture: bool,
        passive: Option<bool>,
        handler: HandlerEmit,
    },
    HtmlBubble,
    Component {
        handler: HandlerEmit,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HandlerEmit {
    Direct,
    WrappedInert,
    WrappedSideEffects,
    WrappedMemoized,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ComponentBindSemantics {
    pub kind: ComponentBindKind,
    pub each_context_vars: SmallVec<[SymbolId; 4]>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ComponentBindKind {
    Expression,
    FunctionPair,
    Identifier {
        symbol: SymbolId,
        target: ComponentBindTarget,
    },
    StoreSubscribed {
        base_symbol: SymbolId,
    },
    This {
        symbol: Option<SymbolId>,
        target: ComponentBindTarget,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComponentBindTarget {
    Plain,
    Rune { proxy: bool },
    RuneDerived,
    LegacyState,
    LegacyStateSubscribed,
    PropSource,
    PropSourceOwned,
    EachItemDestructureLegacy { symbol: SymbolId },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ElementBindSemantics {
    pub property: ElementBindPropertyKind,
    pub kind: HtmlBindKind,
    pub blockers: SmallVec<[u32; 2]>,
    pub parent_each_blocks: SmallVec<[NodeId; 4]>,
    pub each_context_vars: SmallVec<[SymbolId; 4]>,
    pub group_value_attr: Option<NodeId>,
    pub group_id: Option<u32>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ElementBindPropertyKind {
    Value,
    Checked,
    Group,
    Files,
    Indeterminate,
    Open,
    This,
    ContentEditable(ContentEditableKind),
    ElementSize(ElementSizeKind),
    ResizeObserver(ResizeObserverKind),
    Media(MediaBindKind),
    ImageNaturalSize(ImageNaturalSizeKind),
    Focused,
}

impl ElementBindPropertyKind {
    pub fn is_this(self) -> bool {
        matches!(self, ElementBindPropertyKind::This)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WindowBindSemantics {
    pub property: WindowBindKind,
    pub kind: HtmlBindKind,
    pub blockers: SmallVec<[u32; 2]>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DocumentBindSemantics {
    pub property: DocumentBindKind,
    pub kind: HtmlBindKind,
    pub blockers: SmallVec<[u32; 2]>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HtmlBindKind {
    Plain,
    Rune,
    LegacyState,
    BindableProp,
    StoreSubscribed { base_symbol: SymbolId },
    EachItemDestructureLegacy { symbol: SymbolId },
}
