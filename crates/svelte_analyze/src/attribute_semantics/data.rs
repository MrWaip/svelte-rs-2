use crate::expression_semantics::{LegacyWrap, Volatility};
use crate::scope::SymbolId;
use crate::types::data::{
    ClassDirectiveInfo, ContentEditableKind, DocumentBindKind, ElementSizeKind, EventModifier,
    ImageNaturalSizeKind, MediaBindKind, ResizeObserverKind, WindowBindKind,
};
use compact_str::CompactString;
use smallvec::SmallVec;
use svelte_ast::{Attribute, NodeId, OxcNodeId, Span, StyleDirective};

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
    ComponentCssProp(ComponentCssPropValue),
    SvelteComponentThis(SvelteComponentThisSemantics),
    ComponentSpread(ComponentSpreadSemantics),
    ComponentAttach(ComponentAttachSemantics),
    BoundaryProp(BoundaryPropSemantics),
    HtmlConcat(HtmlConcatSemantics),
    CannotBeStatic(DefaultAttrSemantics),
    StaticAttr,
    SpecialValueAttr(SpecialValueSemantics),
    Class(ClassSemantics),
    Style(StyleSemantics),
    Skip(SkipCause),
    Autofocus,
    RuntimeBehavior,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SkipCause {
    TagCarrier,
    Member,
    SlotName,
    SlotBindingLegacy,
}

impl AttributeSemantics {
    pub fn skips_member_write_instrumentation(&self) -> bool {
        match self {
            AttributeSemantics::ElementBind(_)
            | AttributeSemantics::WindowBind(_)
            | AttributeSemantics::DocumentBind(_)
            | AttributeSemantics::ComponentBind(_)
            | AttributeSemantics::Event(_) => true,
            AttributeSemantics::ComponentProp(prop) => match prop.carrier() {
                ComponentPropCarrier::Component => true,
                ComponentPropCarrier::SvelteSelf
                | ComponentPropCarrier::SvelteComponentLegacy
                | ComponentPropCarrier::SlotLegacy => false,
            },
            AttributeSemantics::NonSpecial
            | AttributeSemantics::StaticAttr
            | AttributeSemantics::Skip(_)
            | AttributeSemantics::RuntimeBehavior
            | AttributeSemantics::Class(_)
            | AttributeSemantics::Style(_)
            | AttributeSemantics::ComponentCssProp(_)
            | AttributeSemantics::SvelteComponentThis(_)
            | AttributeSemantics::ComponentSpread(_)
            | AttributeSemantics::ComponentAttach(_)
            | AttributeSemantics::BoundaryProp(_)
            | AttributeSemantics::HtmlConcat(_)
            | AttributeSemantics::CannotBeStatic(_)
            | AttributeSemantics::SpecialValueAttr(_)
            | AttributeSemantics::Autofocus => false,
        }
    }

    pub fn forces_runtime_reference(&self) -> bool {
        match self {
            AttributeSemantics::NonSpecial
            | AttributeSemantics::StaticAttr
            | AttributeSemantics::Skip(_)
            | AttributeSemantics::RuntimeBehavior => false,
            AttributeSemantics::Class(class) => {
                class.attr.is_some() || !class.directives.is_empty()
            }
            AttributeSemantics::Style(style) => {
                style.attr.is_some() || !style.directives.is_empty()
            }
            AttributeSemantics::ElementBind(_)
            | AttributeSemantics::WindowBind(_)
            | AttributeSemantics::DocumentBind(_)
            | AttributeSemantics::ComponentBind(_)
            | AttributeSemantics::Event(_)
            | AttributeSemantics::ComponentProp(_)
            | AttributeSemantics::ComponentCssProp(_)
            | AttributeSemantics::SvelteComponentThis(_)
            | AttributeSemantics::ComponentSpread(_)
            | AttributeSemantics::ComponentAttach(_)
            | AttributeSemantics::BoundaryProp(_)
            | AttributeSemantics::HtmlConcat(_)
            | AttributeSemantics::CannotBeStatic(_)
            | AttributeSemantics::SpecialValueAttr(_)
            | AttributeSemantics::Autofocus => true,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ComponentCssPropValue {
    Expression(OxcNodeId),
    StaticString(Span),
    Concatenation(SmallVec<[ConcatPartEmit; 4]>),
    Boolean,
}

pub fn is_component_css_property(attribute: &Attribute) -> bool {
    match attribute {
        Attribute::ExpressionAttribute(expression) => expression.name.starts_with("--"),
        Attribute::StringAttribute(string) => string.name.starts_with("--"),
        Attribute::ConcatenationAttribute(concatenation) => concatenation.name.starts_with("--"),
        Attribute::BooleanAttribute(boolean) => boolean.name.starts_with("--"),
        Attribute::SpreadAttribute(_)
        | Attribute::ClassDirective(_)
        | Attribute::StyleDirective(_)
        | Attribute::BindDirective(_)
        | Attribute::LetDirectiveLegacy(_)
        | Attribute::UseDirective(_)
        | Attribute::OnDirectiveLegacy(_)
        | Attribute::TransitionDirective(_)
        | Attribute::AnimateDirective(_)
        | Attribute::AttachTag(_) => false,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClassSemantics {
    pub attr: Option<NodeId>,
    pub static_attr: Option<NodeId>,
    pub attr_concat: Option<HtmlConcatSemantics>,
    pub static_base: Option<CompactString>,
    pub needs_clsx: bool,
    pub needs_base: bool,
    pub directives: Vec<ClassDirectiveInfo>,
    pub directives_volatility: Volatility,
    pub state_volatility: Volatility,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StyleSemantics {
    pub attr: Option<NodeId>,
    pub static_attr: Option<NodeId>,
    pub attr_concat: Option<HtmlConcatSemantics>,
    pub static_base: Option<CompactString>,
    pub needs_base: bool,
    pub directives: Vec<StyleDirective>,
    pub directives_volatility: Volatility,
    pub state_volatility: Volatility,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DefaultAttrSemantics {
    pub kind: DefaultAttrKind,
    pub reflects_in_html: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DefaultAttrKind {
    PlainProperty,
    ReconcileValue,
    ReconcileChecked,
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
    AwaitedThunk,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ComponentAttachSemantics {
    pub emit: ComponentAttachEmit,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComponentAttachEmit {
    Inline,
    Wrapped,
    WrappedFallback,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BoundaryPropSemantics {
    pub volatility: Volatility,
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

impl ComponentPropSemantics {
    pub fn carrier(&self) -> ComponentPropCarrier {
        match self {
            ComponentPropSemantics::Expression(expression) => expression.carrier,
            ComponentPropSemantics::Concat(concat) => concat.carrier,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComponentPropCarrier {
    Component,
    SvelteSelf,
    SvelteComponentLegacy,
    SlotLegacy,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ComponentPropExpressionSemantics {
    pub memo: ComponentPropMemo,
    pub shorthand: bool,
    pub carrier: ComponentPropCarrier,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ComponentPropConcatSemantics {
    pub memo: ComponentPropMemo,
    pub plan: SmallVec<[ConcatPartEmit; 4]>,
    pub carrier: ComponentPropCarrier,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConcatPartEmit {
    Static,
    Inline,
    HoistDerived,
    Awaited,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComponentPropMemo {
    Inline,
    Getter,
    Derived,
    Awaited,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EventSemantics {
    pub name: String,
    pub modifiers: EventModifier,
    pub delegatable: bool,
    pub capture: bool,
    pub passive: bool,
    pub handler: EventHandler,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventHandler {
    Forwarded,
    FunctionValue,
    LooseReference,
    Expression(HandlerEffect),
}

impl EventHandler {
    pub fn is_user(&self) -> bool {
        match self {
            EventHandler::FunctionValue
            | EventHandler::LooseReference
            | EventHandler::Expression(_) => true,
            EventHandler::Forwarded => false,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HandlerEffect {
    Pure,
    Mutation,
    Call {
        top_level_side_effect: bool,
        bare_named_call: bool,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ComponentBindSemantics {
    pub kind: ComponentBindKind,
    pub each_context_vars: SmallVec<[SymbolId; 4]>,
    pub ownership_root: Option<SymbolId>,
    pub each_item_store_backed: bool,
    pub needs_binding_validation: bool,
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
    StoreMemberMutation {
        store_symbol: SymbolId,
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
    EachItemWriteLegacy { symbol: SymbolId },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GroupBindValue {
    Expression { expression: OxcNodeId, data: NodeId },
    Static { node: NodeId },
    Boolean,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GroupReflection {
    Equality,
    Includes,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ElementBindSemantics {
    pub property: ElementBindPropertyKind,
    pub kind: HtmlBindKind,
    pub blockers: SmallVec<[u32; 2]>,
    pub parent_each_blocks: SmallVec<[NodeId; 4]>,
    pub each_context_vars: SmallVec<[SymbolId; 4]>,
    pub group_value: Option<GroupBindValue>,
    pub group_reflection: Option<GroupReflection>,
    pub group_id: Option<u32>,
    pub needs_binding_validation: bool,
    pub each_item_store_backed: bool,
    pub reflects_as_attribute: bool,
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

    pub fn reflects_in_html(self) -> bool {
        match self {
            ElementBindPropertyKind::Value
            | ElementBindPropertyKind::Checked
            | ElementBindPropertyKind::Group
            | ElementBindPropertyKind::Open
            | ElementBindPropertyKind::Focused
            | ElementBindPropertyKind::ContentEditable(_) => true,
            ElementBindPropertyKind::Files
            | ElementBindPropertyKind::Indeterminate
            | ElementBindPropertyKind::This
            | ElementBindPropertyKind::ElementSize(_)
            | ElementBindPropertyKind::ResizeObserver(_)
            | ElementBindPropertyKind::Media(_)
            | ElementBindPropertyKind::ImageNaturalSize(_) => false,
        }
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
    EachItemWriteLegacy { symbol: SymbolId },
}
