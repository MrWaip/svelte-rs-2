use super::*;

pub struct TemplateSemanticsData {
    pub(crate) node_expr_handles: NodeTable<ExprHandle>,
    pub(crate) attr_expr_handles: NodeTable<ExprHandle>,
    pub(crate) let_directive_stmt_handles: NodeTable<StmtHandle>,
    pub(crate) const_tag_stmt_handles: NodeTable<StmtHandle>,
    pub(crate) snippet_stmt_handles: NodeTable<StmtHandle>,
    pub(crate) await_value_stmt_handles: NodeTable<StmtHandle>,
    pub(crate) await_error_stmt_handles: NodeTable<StmtHandle>,
    pub(crate) node_ref_symbols: NodeTable<SmallVec<[SymbolId; 2]>>,
    pub(crate) stmt_ref_symbols: NodeTable<SmallVec<[SymbolId; 2]>>,
}

impl TemplateSemanticsData {
    pub fn new(node_count: u32) -> Self {
        Self {
            node_expr_handles: NodeTable::new(node_count),
            attr_expr_handles: NodeTable::new(node_count),
            let_directive_stmt_handles: NodeTable::new(node_count),
            const_tag_stmt_handles: NodeTable::new(node_count),
            snippet_stmt_handles: NodeTable::new(node_count),
            await_value_stmt_handles: NodeTable::new(node_count),
            await_error_stmt_handles: NodeTable::new(node_count),
            node_ref_symbols: NodeTable::new(node_count),
            stmt_ref_symbols: NodeTable::new(node_count),
        }
    }

    pub fn node_ref_symbols(&self, id: NodeId) -> &[SymbolId] {
        self.node_ref_symbols.get(id).map_or(&[], |v| v.as_slice())
    }

    pub fn stmt_ref_symbols(&self, id: NodeId) -> &[SymbolId] {
        self.stmt_ref_symbols.get(id).map_or(&[], |v| v.as_slice())
    }
}

pub struct SnippetData {
    pub(crate) hoistable: NodeBitSet,
    pub(crate) component_snippets: NodeTable<Vec<NodeId>>,
    /// Named slots for component children: maps component NodeId → vec of (slot_element_id, fragment_key).
    pub(crate) component_named_slots: NodeTable<Vec<(NodeId, FragmentKey)>>,
    pub(crate) local_snippets: Vec<NodeId>,
    pub(crate) snippet_name_symbols: FxHashMap<SymbolId, NodeId>,
}

impl SnippetData {
    pub fn new(node_count: u32) -> Self {
        Self {
            hoistable: NodeBitSet::new(node_count),
            component_snippets: NodeTable::new(node_count),
            component_named_slots: NodeTable::new(node_count),
            local_snippets: Vec::new(),
            snippet_name_symbols: FxHashMap::default(),
        }
    }

    pub fn is_hoistable(&self, id: NodeId) -> bool {
        self.hoistable.contains(&id)
    }
    pub fn component_snippets(&self, id: NodeId) -> &[NodeId] {
        self.component_snippets
            .get(id)
            .map_or(&[], |v| v.as_slice())
    }
    pub fn component_named_slots(&self, id: NodeId) -> &[(NodeId, FragmentKey)] {
        self.component_named_slots
            .get(id)
            .map_or(&[], |v| v.as_slice())
    }
    pub fn local_snippets(&self) -> &[NodeId] {
        self.local_snippets.as_slice()
    }
    pub fn snippet_by_symbol(&self, sym_id: SymbolId) -> Option<NodeId> {
        self.snippet_name_symbols.get(&sym_id).copied()
    }
}

pub struct ConstTagData {
    pub(crate) names: NodeTable<Vec<String>>,
    pub(crate) syms: NodeTable<Vec<SymbolId>>,
    pub(crate) by_fragment: FxHashMap<FragmentKey, Vec<NodeId>>,
}

impl ConstTagData {
    pub fn new(node_count: u32) -> Self {
        Self {
            names: NodeTable::new(node_count),
            syms: NodeTable::new(node_count),
            by_fragment: FxHashMap::default(),
        }
    }

    pub fn names(&self, id: NodeId) -> Option<&Vec<String>> {
        self.names.get(id)
    }
    pub fn syms(&self, id: NodeId) -> Option<&Vec<SymbolId>> {
        self.syms.get(id)
    }
    pub fn by_fragment(&self, key: &FragmentKey) -> Option<&Vec<NodeId>> {
        self.by_fragment.get(key)
    }
}

pub struct DebugTagData {
    pub(crate) by_fragment: FxHashMap<FragmentKey, Vec<NodeId>>,
}

impl Default for DebugTagData {
    fn default() -> Self {
        Self::new()
    }
}

impl DebugTagData {
    pub fn new() -> Self {
        Self {
            by_fragment: FxHashMap::default(),
        }
    }

    pub fn by_fragment(&self, key: &FragmentKey) -> Option<&Vec<NodeId>> {
        self.by_fragment.get(key)
    }
}

pub struct TitleElementData {
    pub(crate) by_fragment: FxHashMap<FragmentKey, Vec<NodeId>>,
}

impl Default for TitleElementData {
    fn default() -> Self {
        Self::new()
    }
}

impl TitleElementData {
    pub fn new() -> Self {
        Self {
            by_fragment: FxHashMap::default(),
        }
    }

    pub fn by_fragment(&self, key: &FragmentKey) -> Option<&Vec<NodeId>> {
        self.by_fragment.get(key)
    }
}

pub struct AwaitBindingData {
    pub(crate) values: NodeTable<AwaitBindingInfo>,
    pub(crate) errors: NodeTable<AwaitBindingInfo>,
}

impl AwaitBindingData {
    pub fn new(node_count: u32) -> Self {
        Self {
            values: NodeTable::new(node_count),
            errors: NodeTable::new(node_count),
        }
    }

    pub fn value(&self, id: NodeId) -> Option<&AwaitBindingInfo> {
        self.values.get(id)
    }
    pub fn error(&self, id: NodeId) -> Option<&AwaitBindingInfo> {
        self.errors.get(id)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BindHostKind {
    Element,
    Component,
    Window,
    Document,
    Body,
}

impl BindHostKind {
    pub fn from_parent_kind(kind: ParentKind) -> Option<Self> {
        match kind {
            ParentKind::Element | ParentKind::SlotElementLegacy | ParentKind::SvelteElement => {
                Some(Self::Element)
            }
            ParentKind::ComponentNode => Some(Self::Component),
            ParentKind::SvelteWindow => Some(Self::Window),
            ParentKind::SvelteDocument => Some(Self::Document),
            ParentKind::SvelteBody => Some(Self::Body),
            _ => None,
        }
    }

    pub fn is_component(self) -> bool {
        self == Self::Component
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BindPropertyKind {
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
    Window(WindowBindKind),
    Document(DocumentBindKind),
    ComponentProp,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContentEditableKind {
    InnerHtml,
    InnerText,
    TextContent,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ElementSizeKind {
    ClientWidth,
    ClientHeight,
    OffsetWidth,
    OffsetHeight,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResizeObserverKind {
    ContentRect,
    ContentBoxSize,
    BorderBoxSize,
    DevicePixelContentBoxSize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MediaBindKind {
    CurrentTime,
    PlaybackRate,
    Paused,
    Volume,
    Muted,
    Buffered,
    Seekable,
    Seeking,
    Ended,
    ReadyState,
    Played,
    Duration,
    VideoWidth,
    VideoHeight,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImageNaturalSizeKind {
    NaturalWidth,
    NaturalHeight,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WindowBindKind {
    ScrollX,
    ScrollY,
    InnerWidth,
    InnerHeight,
    OuterWidth,
    OuterHeight,
    Online,
    DevicePixelRatio,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DocumentBindKind {
    ActiveElement,
    FullscreenElement,
    PointerLockElement,
    VisibilityState,
}

#[derive(Clone, Copy)]
pub(crate) struct BindValidationSpec {
    valid_elements: &'static [&'static str],
    invalid_elements: &'static [&'static str],
}

impl BindValidationSpec {
    pub(crate) fn allows(self, element: &str) -> bool {
        if !self.valid_elements.is_empty() {
            self.valid_elements.contains(&element)
        } else {
            !self.invalid_elements.contains(&element)
        }
    }

    pub(crate) fn valid_elements(self) -> &'static [&'static str] {
        self.valid_elements
    }

    pub(crate) fn invalid_elements(self) -> &'static [&'static str] {
        self.invalid_elements
    }
}

impl BindPropertyKind {
    pub const KNOWN_NAMES: &[&str] = &[
        "currentTime",
        "duration",
        "focused",
        "paused",
        "buffered",
        "seekable",
        "played",
        "volume",
        "muted",
        "playbackRate",
        "seeking",
        "ended",
        "readyState",
        "videoHeight",
        "videoWidth",
        "naturalWidth",
        "naturalHeight",
        "activeElement",
        "fullscreenElement",
        "pointerLockElement",
        "visibilityState",
        "innerWidth",
        "innerHeight",
        "outerWidth",
        "outerHeight",
        "scrollX",
        "scrollY",
        "online",
        "devicePixelRatio",
        "clientWidth",
        "clientHeight",
        "offsetWidth",
        "offsetHeight",
        "contentRect",
        "contentBoxSize",
        "borderBoxSize",
        "devicePixelContentBoxSize",
        "indeterminate",
        "checked",
        "group",
        "this",
        "innerText",
        "innerHTML",
        "textContent",
        "open",
        "value",
        "files",
    ];

    pub fn from_host_and_name(host: BindHostKind, name: &str) -> Option<Self> {
        if host.is_component() {
            return Some(if name == "this" {
                Self::This
            } else {
                Self::ComponentProp
            });
        }

        match name {
            "value" => Some(Self::Value),
            "checked" => Some(Self::Checked),
            "group" => Some(Self::Group),
            "files" => Some(Self::Files),
            "indeterminate" => Some(Self::Indeterminate),
            "open" => Some(Self::Open),
            "this" => Some(Self::This),
            "innerHTML" => Some(Self::ContentEditable(ContentEditableKind::InnerHtml)),
            "innerText" => Some(Self::ContentEditable(ContentEditableKind::InnerText)),
            "textContent" => Some(Self::ContentEditable(ContentEditableKind::TextContent)),
            "clientWidth" => Some(Self::ElementSize(ElementSizeKind::ClientWidth)),
            "clientHeight" => Some(Self::ElementSize(ElementSizeKind::ClientHeight)),
            "offsetWidth" => Some(Self::ElementSize(ElementSizeKind::OffsetWidth)),
            "offsetHeight" => Some(Self::ElementSize(ElementSizeKind::OffsetHeight)),
            "contentRect" => Some(Self::ResizeObserver(ResizeObserverKind::ContentRect)),
            "contentBoxSize" => Some(Self::ResizeObserver(ResizeObserverKind::ContentBoxSize)),
            "borderBoxSize" => Some(Self::ResizeObserver(ResizeObserverKind::BorderBoxSize)),
            "devicePixelContentBoxSize" => Some(Self::ResizeObserver(
                ResizeObserverKind::DevicePixelContentBoxSize,
            )),
            "currentTime" => Some(Self::Media(MediaBindKind::CurrentTime)),
            "playbackRate" => Some(Self::Media(MediaBindKind::PlaybackRate)),
            "paused" => Some(Self::Media(MediaBindKind::Paused)),
            "volume" => Some(Self::Media(MediaBindKind::Volume)),
            "muted" => Some(Self::Media(MediaBindKind::Muted)),
            "buffered" => Some(Self::Media(MediaBindKind::Buffered)),
            "seekable" => Some(Self::Media(MediaBindKind::Seekable)),
            "seeking" => Some(Self::Media(MediaBindKind::Seeking)),
            "ended" => Some(Self::Media(MediaBindKind::Ended)),
            "readyState" => Some(Self::Media(MediaBindKind::ReadyState)),
            "played" => Some(Self::Media(MediaBindKind::Played)),
            "duration" => Some(Self::Media(MediaBindKind::Duration)),
            "videoWidth" => Some(Self::Media(MediaBindKind::VideoWidth)),
            "videoHeight" => Some(Self::Media(MediaBindKind::VideoHeight)),
            "naturalWidth" => Some(Self::ImageNaturalSize(ImageNaturalSizeKind::NaturalWidth)),
            "naturalHeight" => Some(Self::ImageNaturalSize(ImageNaturalSizeKind::NaturalHeight)),
            "focused" => Some(Self::Focused),
            "scrollX" => Some(Self::Window(WindowBindKind::ScrollX)),
            "scrollY" => Some(Self::Window(WindowBindKind::ScrollY)),
            "innerWidth" => Some(Self::Window(WindowBindKind::InnerWidth)),
            "innerHeight" => Some(Self::Window(WindowBindKind::InnerHeight)),
            "outerWidth" => Some(Self::Window(WindowBindKind::OuterWidth)),
            "outerHeight" => Some(Self::Window(WindowBindKind::OuterHeight)),
            "online" => Some(Self::Window(WindowBindKind::Online)),
            "devicePixelRatio" => Some(Self::Window(WindowBindKind::DevicePixelRatio)),
            "activeElement" => Some(Self::Document(DocumentBindKind::ActiveElement)),
            "fullscreenElement" => Some(Self::Document(DocumentBindKind::FullscreenElement)),
            "pointerLockElement" => Some(Self::Document(DocumentBindKind::PointerLockElement)),
            "visibilityState" => Some(Self::Document(DocumentBindKind::VisibilityState)),
            _ => None,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Value => "value",
            Self::Checked => "checked",
            Self::Group => "group",
            Self::Files => "files",
            Self::Indeterminate => "indeterminate",
            Self::Open => "open",
            Self::This => "this",
            Self::ContentEditable(kind) => kind.name(),
            Self::ElementSize(kind) => kind.name(),
            Self::ResizeObserver(kind) => kind.name(),
            Self::Media(kind) => kind.name(),
            Self::ImageNaturalSize(kind) => kind.name(),
            Self::Focused => "focused",
            Self::Window(kind) => kind.name(),
            Self::Document(kind) => kind.name(),
            Self::ComponentProp => "component-prop",
        }
    }

    pub(crate) fn validation_spec(self) -> BindValidationSpec {
        match self {
            Self::Value => BindValidationSpec {
                valid_elements: &["input", "textarea", "select"],
                invalid_elements: &[],
            },
            Self::Checked | Self::Group | Self::Files => BindValidationSpec {
                valid_elements: &["input"],
                invalid_elements: &[],
            },
            Self::Indeterminate => BindValidationSpec {
                valid_elements: &["input"],
                invalid_elements: &[],
            },
            Self::Open => BindValidationSpec {
                valid_elements: &["details"],
                invalid_elements: &[],
            },
            Self::This | Self::Focused => BindValidationSpec {
                valid_elements: &[],
                invalid_elements: &[],
            },
            Self::ContentEditable(_) | Self::ElementSize(_) | Self::ResizeObserver(_) => {
                BindValidationSpec {
                    valid_elements: &[],
                    invalid_elements: &["svelte:window", "svelte:document"],
                }
            }
            Self::Media(kind) => kind.validation_spec(),
            Self::ImageNaturalSize(_) => BindValidationSpec {
                valid_elements: &["img"],
                invalid_elements: &[],
            },
            Self::Window(_) => BindValidationSpec {
                valid_elements: &["svelte:window"],
                invalid_elements: &[],
            },
            Self::Document(_) => BindValidationSpec {
                valid_elements: &["svelte:document"],
                invalid_elements: &[],
            },
            Self::ComponentProp => BindValidationSpec {
                valid_elements: &[],
                invalid_elements: &[],
            },
        }
    }

    pub fn is_group(self) -> bool {
        self == Self::Group
    }

    pub fn is_this(self) -> bool {
        self == Self::This
    }

    pub fn is_contenteditable(self) -> bool {
        matches!(self, Self::ContentEditable(_))
    }

    pub fn marks_input_defaults(self) -> bool {
        matches!(self, Self::Value | Self::Checked | Self::Group)
    }

    pub fn requires_mutable_target(self) -> bool {
        !self.is_this()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BindTargetSemantics {
    host: BindHostKind,
    property: BindPropertyKind,
    requires_mutable_target: bool,
}

impl BindTargetSemantics {
    pub fn from_parent_kind_and_name(kind: ParentKind, name: &str) -> Option<Self> {
        let host = BindHostKind::from_parent_kind(kind)?;
        let property = BindPropertyKind::from_host_and_name(host, name)?;

        Some(Self::new(host, property))
    }

    pub fn new(host: BindHostKind, property: BindPropertyKind) -> Self {
        Self {
            host,
            property,
            requires_mutable_target: property.requires_mutable_target(),
        }
    }

    pub fn host(self) -> BindHostKind {
        self.host
    }

    pub fn property(self) -> BindPropertyKind {
        self.property
    }

    pub fn requires_mutable_target(self) -> bool {
        self.requires_mutable_target
    }

    pub fn is_this(&self) -> bool {
        self.property.is_this()
    }

    pub fn is_group(&self) -> bool {
        self.property.is_group()
    }

    pub fn is_contenteditable(&self) -> bool {
        self.property.is_contenteditable()
    }

    pub(crate) fn validation_spec(self) -> BindValidationSpec {
        self.property.validation_spec()
    }
}

impl ContentEditableKind {
    pub fn name(self) -> &'static str {
        match self {
            Self::InnerHtml => "innerHTML",
            Self::InnerText => "innerText",
            Self::TextContent => "textContent",
        }
    }
}

impl ElementSizeKind {
    pub fn name(self) -> &'static str {
        match self {
            Self::ClientWidth => "clientWidth",
            Self::ClientHeight => "clientHeight",
            Self::OffsetWidth => "offsetWidth",
            Self::OffsetHeight => "offsetHeight",
        }
    }
}

impl ResizeObserverKind {
    pub fn name(self) -> &'static str {
        match self {
            Self::ContentRect => "contentRect",
            Self::ContentBoxSize => "contentBoxSize",
            Self::BorderBoxSize => "borderBoxSize",
            Self::DevicePixelContentBoxSize => "devicePixelContentBoxSize",
        }
    }
}

impl MediaBindKind {
    pub fn name(self) -> &'static str {
        match self {
            Self::CurrentTime => "currentTime",
            Self::PlaybackRate => "playbackRate",
            Self::Paused => "paused",
            Self::Volume => "volume",
            Self::Muted => "muted",
            Self::Buffered => "buffered",
            Self::Seekable => "seekable",
            Self::Seeking => "seeking",
            Self::Ended => "ended",
            Self::ReadyState => "readyState",
            Self::Played => "played",
            Self::Duration => "duration",
            Self::VideoWidth => "videoWidth",
            Self::VideoHeight => "videoHeight",
        }
    }

    pub(crate) fn validation_spec(self) -> BindValidationSpec {
        match self {
            Self::VideoWidth | Self::VideoHeight => BindValidationSpec {
                valid_elements: &["video"],
                invalid_elements: &[],
            },
            Self::Duration
            | Self::CurrentTime
            | Self::PlaybackRate
            | Self::Paused
            | Self::Volume
            | Self::Muted
            | Self::Buffered
            | Self::Seekable
            | Self::Seeking
            | Self::Ended
            | Self::ReadyState
            | Self::Played => BindValidationSpec {
                valid_elements: &["audio", "video"],
                invalid_elements: &[],
            },
        }
    }
}

impl ImageNaturalSizeKind {
    pub fn name(self) -> &'static str {
        match self {
            Self::NaturalWidth => "naturalWidth",
            Self::NaturalHeight => "naturalHeight",
        }
    }
}

impl WindowBindKind {
    pub fn name(self) -> &'static str {
        match self {
            Self::ScrollX => "scrollX",
            Self::ScrollY => "scrollY",
            Self::InnerWidth => "innerWidth",
            Self::InnerHeight => "innerHeight",
            Self::OuterWidth => "outerWidth",
            Self::OuterHeight => "outerHeight",
            Self::Online => "online",
            Self::DevicePixelRatio => "devicePixelRatio",
        }
    }
}

impl DocumentBindKind {
    pub fn name(self) -> &'static str {
        match self {
            Self::ActiveElement => "activeElement",
            Self::FullscreenElement => "fullscreenElement",
            Self::PointerLockElement => "pointerLockElement",
            Self::VisibilityState => "visibilityState",
        }
    }
}

pub struct BindSemanticsData {
    pub(crate) target_semantics: NodeTable<BindTargetSemantics>,
    pub(crate) has_bind_group: NodeBitSet,
    pub(crate) bind_group_value_attr: NodeTable<NodeId>,
    pub(crate) bind_blockers: NodeTable<SmallVec<[u32; 2]>>,
    pub(crate) bind_this_each_context: NodeTable<SmallVec<[SymbolId; 4]>>,
}

impl BindSemanticsData {
    pub fn new(node_count: u32) -> Self {
        Self {
            target_semantics: NodeTable::new(node_count),
            has_bind_group: NodeBitSet::new(node_count),
            bind_group_value_attr: NodeTable::new(node_count),
            bind_blockers: NodeTable::new(node_count),
            bind_this_each_context: NodeTable::new(node_count),
        }
    }

    pub fn bind_target_semantics(&self, id: NodeId) -> Option<&BindTargetSemantics> {
        self.target_semantics.get(id)
    }
    pub fn has_bind_group(&self, id: NodeId) -> bool {
        self.has_bind_group.contains(&id)
    }
    pub fn bind_group_value_attr(&self, id: NodeId) -> Option<NodeId> {
        self.bind_group_value_attr.get(id).copied()
    }
    pub fn bind_blockers(&self, id: NodeId) -> &[u32] {
        self.bind_blockers.get(id).map_or(&[], |v| v.as_slice())
    }
    pub fn bind_this_each_context(&self, id: NodeId) -> Option<&[SymbolId]> {
        self.bind_this_each_context
            .get(id)
            .map(|syms| syms.as_slice())
    }
}
