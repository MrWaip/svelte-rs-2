use crate::expression_semantics::Volatility;
use crate::scope::SymbolId;
use bitflags::bitflags;
use smallvec::SmallVec;
use svelte_ast::NodeId;
use svelte_component_semantics::OxcNodeId;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum BlockSemantics {
    #[default]
    NonSpecial,

    Each(EachBlockSemantics),

    Await(AwaitBlockSemantics),

    Snippet(SnippetBlockSemantics),

    ConstTag(ConstTagBlockSemantics),

    Render(RenderTagBlockSemantics),

    If(IfBlockSemantics),

    Key(KeyBlockSemantics),

    HtmlTag(HtmlTagSemantics),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HtmlTagSemantics {
    pub parent_strategy: HtmlTagNamespace,
    pub hydration_html_changed_ignored: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HtmlTagNamespace {
    Html,
    Svg,
    MathMl,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EachBlockSemantics {
    pub item: EachItemKind,

    pub index: EachIndexKind,

    pub key: EachKeyKind,

    pub flavor: EachFlavor,

    pub each_flags: EachFlags,

    pub shadows_outer: bool,

    pub async_kind: EachAsyncKind,

    pub collection: EachCollection,

    pub collection_store: Option<SymbolId>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EachCollection {
    pub source: EachCollectionSource,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EachCollectionSource {
    Local,

    Prop { sym: SymbolId },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EachAsyncKind {
    Sync,

    Awaited { blockers: SmallVec<[u32; 2]> },

    Deferred { blockers: SmallVec<[u32; 2]> },
}

bitflags! {



    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub struct EachFlags: u8 {
        const ITEM_REACTIVE  = 1;
        const INDEX_REACTIVE = 2;
        const ANIMATED       = 8;
        const ITEM_IMMUTABLE = 16;
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EachItemKind {
    NoBinding,

    Identifier(SymbolId),

    Pattern(OxcNodeId),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EachIndexKind {
    Absent,

    Declared {
        sym: SymbolId,

        used_in_body: bool,

        used_in_key: bool,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EachKeyKind {
    Unkeyed,

    KeyedByIndex,

    KeyedByItem,

    KeyedByExpr(OxcNodeId),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EachFlavor {
    Regular,

    BindGroup,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AwaitBlockSemantics {
    pub pending: AwaitBranch,

    pub then: AwaitBranch,

    pub catch: AwaitBranch,

    pub expression_volatility: Volatility,

    pub wrapper: AwaitWrapper,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AwaitBranch {
    Absent,

    Present { binding: AwaitBinding },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AwaitBinding {
    None,

    Identifier(SymbolId),

    Pattern {
        kind: AwaitDestructureKind,
        leaves: SmallVec<[SymbolId; 4]>,
        pattern_id: OxcNodeId,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AwaitDestructureKind {
    Object,

    Array,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AwaitWrapper {
    None,

    AsyncWrap { blockers: SmallVec<[u32; 2]> },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SnippetBlockSemantics {
    pub name: SymbolId,

    pub hoistable: bool,

    pub params: SmallVec<[SnippetParam; 4]>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SnippetParam {
    Identifier { sym: SymbolId },

    Pattern { pattern_id: OxcNodeId },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConstTagBlockSemantics {
    pub decl_node_id: OxcNodeId,

    pub async_kind: ConstTagAsyncKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConstTagAsyncKind {
    Sync,

    Awaited { blockers: SmallVec<[u32; 2]> },

    Deferred { blockers: SmallVec<[u32; 2]> },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RenderTagBlockSemantics {
    pub call_kind: RenderCallKind,

    pub callee_sym: Option<SymbolId>,

    pub args: SmallVec<[RenderArgKind; 4]>,

    pub async_kind: RenderAsyncKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderCallKind {
    Plain,

    OptionalChain,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderArgKind {
    PropPassthrough { sym: SymbolId },

    NeedsMemo,

    AwaitMemo { inner_node_id: Option<OxcNodeId> },

    InertThunk,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RenderAsyncKind {
    Sync,

    Async { blockers: SmallVec<[u32; 2]> },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IfBlockSemantics {
    pub branches: SmallVec<[IfBranch; 2]>,

    pub final_alternate: IfAlternate,

    pub is_elseif_root: bool,

    pub async_kind: IfAsyncKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IfBranch {
    pub block_id: NodeId,

    pub condition: IfConditionKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IfConditionKind {
    Raw,

    Memo,

    AsyncParam,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IfAsyncKind {
    Sync,

    Awaited { blockers: SmallVec<[u32; 2]> },

    Deferred { blockers: SmallVec<[u32; 2]> },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IfAlternate {
    None,

    Fragment { last_branch_block_id: NodeId },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KeyBlockSemantics {
    pub async_kind: KeyAsyncKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum KeyAsyncKind {
    Sync,

    Awaited { blockers: SmallVec<[u32; 2]> },

    Deferred { blockers: SmallVec<[u32; 2]> },
}
