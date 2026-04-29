use svelte_span::GetSpan;
pub use svelte_span::Span;

mod expr_ref;
pub use expr_ref::{ExprRef, OxcNodeId, StmtRef};

const VOID_ELEMENTS: &[&str] = &[
    "area", "base", "br", "col", "command", "embed", "hr", "img", "input", "keygen", "link",
    "meta", "param", "source", "track", "wbr",
];

pub fn is_void(name: &str) -> bool {
    VOID_ELEMENTS.contains(&name)
}

const SVG_ELEMENTS: &[&str] = &[
    "altGlyph",
    "altGlyphDef",
    "altGlyphItem",
    "animate",
    "animateColor",
    "animateMotion",
    "animateTransform",
    "circle",
    "clipPath",
    "color-profile",
    "cursor",
    "defs",
    "desc",
    "discard",
    "ellipse",
    "feBlend",
    "feColorMatrix",
    "feComponentTransfer",
    "feComposite",
    "feConvolveMatrix",
    "feDiffuseLighting",
    "feDisplacementMap",
    "feDistantLight",
    "feDropShadow",
    "feFlood",
    "feFuncA",
    "feFuncB",
    "feFuncG",
    "feFuncR",
    "feGaussianBlur",
    "feImage",
    "feMerge",
    "feMergeNode",
    "feMorphology",
    "feOffset",
    "fePointLight",
    "feSpecularLighting",
    "feSpotLight",
    "feTile",
    "feTurbulence",
    "filter",
    "font",
    "font-face",
    "font-face-format",
    "font-face-name",
    "font-face-src",
    "font-face-uri",
    "foreignObject",
    "g",
    "glyph",
    "glyphRef",
    "hatch",
    "hatchpath",
    "hkern",
    "image",
    "line",
    "linearGradient",
    "marker",
    "mask",
    "mesh",
    "meshgradient",
    "meshpatch",
    "meshrow",
    "metadata",
    "missing-glyph",
    "mpath",
    "path",
    "pattern",
    "polygon",
    "polyline",
    "radialGradient",
    "rect",
    "set",
    "solidcolor",
    "stop",
    "svg",
    "switch",
    "symbol",
    "text",
    "textPath",
    "tref",
    "tspan",
    "unknown",
    "use",
    "view",
    "vkern",
];

pub fn is_svg(name: &str) -> bool {
    SVG_ELEMENTS.contains(&name)
}

const MATHML_ELEMENTS: &[&str] = &[
    "annotation",
    "annotation-xml",
    "maction",
    "math",
    "merror",
    "mfrac",
    "mi",
    "mmultiscripts",
    "mn",
    "mo",
    "mover",
    "mpadded",
    "mphantom",
    "mprescripts",
    "mroot",
    "mrow",
    "ms",
    "mspace",
    "msqrt",
    "mstyle",
    "msub",
    "msubsup",
    "msup",
    "mtable",
    "mtd",
    "mtext",
    "mtr",
    "munder",
    "munderover",
    "semantics",
];

pub fn is_mathml(name: &str) -> bool {
    MATHML_ELEMENTS.contains(&name)
}

const WHITESPACE_REMOVABLE_ELEMENTS: &[&str] = &[
    "select", "tr", "table", "tbody", "thead", "tfoot", "colgroup", "datalist",
];

pub fn is_whitespace_removable_parent(name: &str) -> bool {
    WHITESPACE_REMOVABLE_ELEMENTS.contains(&name)
}

pub const SVELTE_COMPONENT: &str = "svelte:component";

pub const SVELTE_SELF: &str = "svelte:self";

pub const SVELTE_FRAGMENT: &str = "svelte:fragment";

pub const SVELTE_OPTIONS: &str = "svelte:options";

pub const SVELTE_HEAD: &str = "svelte:head";

pub const SVELTE_WINDOW: &str = "svelte:window";

pub const SVELTE_DOCUMENT: &str = "svelte:document";

pub const SVELTE_BODY: &str = "svelte:body";

pub const SVELTE_ELEMENT: &str = "svelte:element";

pub const SVELTE_BOUNDARY: &str = "svelte:boundary";

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct NodeId(pub u32);

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct FragmentId(pub u32);

pub struct Component {
    pub root: FragmentId,
    pub store: AstStore,

    pub instance_script: Option<Script>,

    pub module_script: Option<Script>,
    pub css: Option<RawBlock>,
    pub options: Option<SvelteOptions>,

    pub source: String,
}

impl Component {
    pub fn new(
        source: String,
        root: FragmentId,
        store: AstStore,
        instance_script: Option<Script>,
        module_script: Option<Script>,
        css: Option<RawBlock>,
    ) -> Self {
        Self {
            root,
            store,
            instance_script,
            module_script,
            css,
            options: None,
            source,
        }
    }

    pub fn dummy_for_standalone_module(source: String) -> Self {
        let mut store = AstStore::default();
        let root = store.push_fragment(FragmentRole::Root, vec![]);
        Self {
            root,
            store,
            instance_script: None,
            module_script: None,
            css: None,
            options: None,
            source,
        }
    }

    pub fn root_fragment(&self) -> &Fragment {
        self.store.fragment(self.root)
    }

    pub fn fragment_nodes(&self, id: FragmentId) -> &[NodeId] {
        self.store.fragment_nodes(id)
    }

    pub fn node_count(&self) -> u32 {
        self.store.len()
    }

    pub fn fragment_count(&self) -> u32 {
        self.store.fragments_len()
    }

    pub fn source_text(&self, span: Span) -> &str {
        &self.source[span.start as usize..span.end as usize]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FragmentRole {
    Root,
    Element,
    ComponentChildren,

    NamedSlot,
    IfConsequent,
    IfAlternate,
    EachBody,
    EachFallback,
    SnippetBody,
    KeyBlockBody,
    SvelteHeadBody,
    SvelteElementBody,
    SvelteBoundaryBody,
    AwaitPending,
    AwaitThen,
    AwaitCatch,
}

pub struct Fragment {
    pub id: FragmentId,
    pub role: FragmentRole,
    pub nodes: Vec<NodeId>,

    pub owner: Option<NodeId>,
}

impl Fragment {
    pub fn new(id: FragmentId, role: FragmentRole, nodes: Vec<NodeId>) -> Self {
        Self {
            id,
            role,
            nodes,
            owner: None,
        }
    }

    pub fn empty(id: FragmentId, role: FragmentRole) -> Self {
        Self {
            id,
            role,
            nodes: vec![],
            owner: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn push(&mut self, id: NodeId) {
        self.nodes.push(id);
    }
}

macro_rules! impl_node_enum {
    ( $( $Variant:ident($Type:ident) => $is:ident / $as:ident ),+ $(,)? ) => {
        pub enum Node {
            $( $Variant($Type), )+
        }

        impl Node {
            pub fn node_id(&self) -> NodeId {
                match self { $( Node::$Variant(n) => n.id, )+ }
            }

            pub fn span(&self) -> Span {
                match self { $( Node::$Variant(n) => n.span, )+ }
            }


            fn set_id(&mut self, id: NodeId) {
                match self { $( Node::$Variant(n) => n.id = id, )+ }
            }

            $(
                pub fn $is(&self) -> bool {
                    matches!(self, Node::$Variant(_))
                }

                pub fn $as(&self) -> Option<&$Type> {
                    match self { Node::$Variant(n) => Some(n), _ => None }
                }
            )+
        }
    };
}

impl_node_enum! {
    Text(Text)                   => is_text / as_text,
    Element(Element)             => is_element / as_element,
    SlotElementLegacy(SlotElementLegacy) => is_slot_element_legacy / as_slot_element_legacy,
    ComponentNode(ComponentNode) => is_component_node / as_component_node,
    Comment(Comment)             => is_comment / as_comment,
    ExpressionTag(ExpressionTag) => is_expression_tag / as_expression_tag,
    IfBlock(IfBlock)             => is_if_block / as_if_block,
    EachBlock(EachBlock)         => is_each_block / as_each_block,
    SnippetBlock(SnippetBlock)   => is_snippet_block / as_snippet_block,
    RenderTag(RenderTag)         => is_render_tag / as_render_tag,
    HtmlTag(HtmlTag)             => is_html_tag / as_html_tag,
    ConstTag(ConstTag)           => is_const_tag / as_const_tag,
    DebugTag(DebugTag)           => is_debug_tag / as_debug_tag,
    KeyBlock(KeyBlock)           => is_key_block / as_key_block,
    SvelteHead(SvelteHead)       => is_svelte_head / as_svelte_head,
    SvelteFragmentLegacy(SvelteFragmentLegacy) => is_svelte_fragment_legacy / as_svelte_fragment_legacy,
    SvelteElement(SvelteElement) => is_svelte_element / as_svelte_element,
    SvelteWindow(SvelteWindow)       => is_svelte_window / as_svelte_window,
    SvelteDocument(SvelteDocument)   => is_svelte_document / as_svelte_document,
    SvelteBody(SvelteBody)           => is_svelte_body / as_svelte_body,
    SvelteBoundary(SvelteBoundary)   => is_svelte_boundary / as_svelte_boundary,
    AwaitBlock(AwaitBlock)           => is_await_block / as_await_block,
    Error(ErrorNode)                 => is_error / as_error,
}

pub struct ErrorNode {
    pub id: NodeId,
    pub span: Span,
}

pub struct Text {
    pub id: NodeId,
    pub span: Span,
    pub decoded: Option<String>,
}

impl Text {
    pub fn raw_value<'a>(&self, source: &'a str) -> &'a str {
        &source[self.span.start as usize..self.span.end as usize]
    }

    pub fn value<'a>(&'a self, source: &'a str) -> &'a str {
        self.decoded
            .as_deref()
            .unwrap_or_else(|| self.raw_value(source))
    }
}

pub struct Element {
    pub id: NodeId,
    pub span: Span,
    pub name: String,
    pub self_closing: bool,
    pub attributes: Vec<Attribute>,
    pub fragment: FragmentId,
}

pub struct SlotElementLegacy {
    pub id: NodeId,
    pub span: Span,
    pub attributes: Vec<Attribute>,
    pub fragment: FragmentId,
}

pub struct ComponentNode {
    pub id: NodeId,
    pub span: Span,
    pub name: String,
    pub self_closing: bool,
    pub attributes: Vec<Attribute>,
    pub fragment: FragmentId,
    pub legacy_slots: Vec<LegacySlot>,
}

pub struct LegacySlot {
    pub name: String,
    pub fragment: FragmentId,
}

pub struct Comment {
    pub id: NodeId,
    pub span: Span,
}

impl Comment {
    pub fn value<'a>(&self, source: &'a str) -> &'a str {
        &source[self.span.start as usize..self.span.end as usize]
    }
}

pub struct ExpressionTag {
    pub id: NodeId,

    pub span: Span,
    pub expression: ExprRef,
}

pub struct IfBlock {
    pub id: NodeId,
    pub span: Span,
    pub test: ExprRef,
    pub elseif: bool,
    pub consequent: FragmentId,
    pub alternate: Option<FragmentId>,
}

pub struct EachBlock {
    pub id: NodeId,
    pub span: Span,
    pub expression: ExprRef,
    pub context: Option<StmtRef>,
    pub index: Option<StmtRef>,
    pub key: Option<ExprRef>,

    pub key_id: Option<NodeId>,
    pub body: FragmentId,
    pub fallback: Option<FragmentId>,
}

pub struct SnippetBlock {
    pub id: NodeId,
    pub span: Span,
    pub decl: StmtRef,
    pub body: FragmentId,
}

impl SnippetBlock {
    pub fn name<'a>(&self, source: &'a str) -> &'a str {
        let expr = &source[self.decl.span.start as usize..self.decl.span.end as usize];
        match expr.find('(') {
            Some(pos) => &expr[..pos],
            None => expr,
        }
    }
}

pub struct RenderTag {
    pub id: NodeId,
    pub span: Span,
    pub expression: ExprRef,
}

pub struct HtmlTag {
    pub id: NodeId,
    pub span: Span,
    pub expression: ExprRef,
}

pub struct ConstTag {
    pub id: NodeId,
    pub span: Span,
    pub decl: StmtRef,
}

pub struct DebugTag {
    pub id: NodeId,
    pub span: Span,
    pub identifier_refs: Vec<ExprRef>,
}

pub struct KeyBlock {
    pub id: NodeId,
    pub span: Span,
    pub expression: ExprRef,
    pub fragment: FragmentId,
}

pub struct SvelteHead {
    pub id: NodeId,
    pub span: Span,
    pub fragment: FragmentId,
}

pub struct SvelteFragmentLegacy {
    pub id: NodeId,
    pub span: Span,
    pub attributes: Vec<Attribute>,
    pub fragment: FragmentId,
}

pub struct SvelteElement {
    pub id: NodeId,
    pub span: Span,

    pub tag_span: Span,

    pub tag: Option<ExprRef>,

    pub static_tag: bool,
    pub attributes: Vec<Attribute>,
    pub fragment: FragmentId,
}

pub struct SvelteWindow {
    pub id: NodeId,
    pub span: Span,
    pub attributes: Vec<Attribute>,
    pub fragment: FragmentId,
}

pub struct SvelteDocument {
    pub id: NodeId,
    pub span: Span,
    pub attributes: Vec<Attribute>,
    pub fragment: FragmentId,
}

pub struct SvelteBody {
    pub id: NodeId,
    pub span: Span,
    pub attributes: Vec<Attribute>,
    pub fragment: FragmentId,
}

pub struct SvelteBoundary {
    pub id: NodeId,
    pub span: Span,
    pub attributes: Vec<Attribute>,
    pub fragment: FragmentId,
}

pub struct AwaitBlock {
    pub id: NodeId,
    pub span: Span,
    pub expression: ExprRef,
    pub value: Option<StmtRef>,
    pub error: Option<StmtRef>,
    pub pending: Option<FragmentId>,
    pub then: Option<FragmentId>,
    pub catch: Option<FragmentId>,
}

macro_rules! impl_attr_enum {
    ( $( $(#[doc = $doc:expr])* $Variant:ident($Type:ident) ),+ $(,)? ) => {
        pub enum Attribute {
            $( $(#[doc = $doc])* $Variant($Type), )+
        }

        impl Clone for Attribute {
            fn clone(&self) -> Self {
                match self { $( Attribute::$Variant(a) => Attribute::$Variant(a.clone()), )+ }
            }
        }

        impl Attribute {
            pub fn id(&self) -> NodeId {
                match self { $( Attribute::$Variant(a) => a.id, )+ }
            }

            pub fn span(&self) -> Span {
                match self { $( Attribute::$Variant(a) => a.span, )+ }
            }
        }
    };
}

impl_attr_enum! {

    StringAttribute(StringAttribute),



    ExpressionAttribute(ExpressionAttribute),

    BooleanAttribute(BooleanAttribute),

    ConcatenationAttribute(ConcatenationAttribute),

    SpreadAttribute(SpreadAttribute),

    ClassDirective(ClassDirective),

    StyleDirective(StyleDirective),

    BindDirective(BindDirective),

    LetDirectiveLegacy(LetDirectiveLegacy),

    UseDirective(UseDirective),


    OnDirectiveLegacy(OnDirectiveLegacy),

    TransitionDirective(TransitionDirective),

    AnimateDirective(AnimateDirective),

    AttachTag(AttachTag),
}

impl Attribute {
    pub fn name(&self) -> Option<&str> {
        match self {
            Attribute::StringAttribute(a) => Some(&a.name),
            Attribute::ExpressionAttribute(a) => Some(&a.name),
            Attribute::BooleanAttribute(a) => Some(&a.name),
            Attribute::ConcatenationAttribute(a) => Some(&a.name),
            Attribute::ClassDirective(a) => Some(&a.name),
            Attribute::StyleDirective(a) => Some(&a.name),
            Attribute::BindDirective(a) => Some(&a.name),
            Attribute::LetDirectiveLegacy(a) => Some(&a.name),
            Attribute::OnDirectiveLegacy(a) => Some(&a.name),
            Attribute::UseDirective(_)
            | Attribute::TransitionDirective(_)
            | Attribute::AnimateDirective(_)
            | Attribute::SpreadAttribute(_)
            | Attribute::AttachTag(_) => None,
        }
    }

    pub fn html_name(&self) -> &str {
        match self {
            Attribute::StringAttribute(a) => &a.name,
            Attribute::ExpressionAttribute(a) => &a.name,
            Attribute::BooleanAttribute(a) => &a.name,
            Attribute::ConcatenationAttribute(a) => &a.name,
            _ => "",
        }
    }
}

impl GetSpan for Attribute {
    fn span(&self) -> Span {
        self.span()
    }
}

#[derive(Clone)]
pub struct StringAttribute {
    pub id: NodeId,
    pub span: Span,
    pub name: String,
    pub value_span: Span,
}

#[derive(Clone)]
pub struct ExpressionAttribute {
    pub id: NodeId,
    pub span: Span,
    pub name: String,
    pub expression: ExprRef,
    pub shorthand: bool,

    pub event_name: Option<String>,
}

#[derive(Clone)]
pub struct BooleanAttribute {
    pub id: NodeId,
    pub span: Span,
    pub name: String,
}

#[derive(Clone)]
pub struct ConcatenationAttribute {
    pub id: NodeId,
    pub span: Span,
    pub name: String,

    pub quoted: bool,
    pub parts: Vec<ConcatPart>,
}

#[derive(Clone)]
pub enum ConcatPart {
    Static(String),

    Dynamic { id: NodeId, expr: ExprRef },
}

#[derive(Clone)]
pub struct SpreadAttribute {
    pub id: NodeId,
    pub span: Span,
    pub expression: ExprRef,
}

#[derive(Clone)]
pub struct ClassDirective {
    pub id: NodeId,
    pub span: Span,
    pub name: String,
    pub expression: ExprRef,
    pub shorthand: bool,
}

#[derive(Clone)]
pub struct StyleDirective {
    pub id: NodeId,
    pub span: Span,
    pub name: String,
    pub expression: ExprRef,

    pub shorthand: bool,
    pub value: StyleDirectiveValue,
    pub important: bool,
}

#[derive(Clone)]
pub enum StyleDirectiveValue {
    Expression,

    String(String),

    Concatenation(Vec<ConcatPart>),
}

#[derive(Clone)]
pub struct BindDirective {
    pub id: NodeId,
    pub span: Span,
    pub name: String,
    pub expression: ExprRef,
    pub shorthand: bool,
}

#[derive(Clone)]
pub struct LetDirectiveLegacy {
    pub id: NodeId,
    pub span: Span,

    pub name: String,

    pub name_span: Span,
    pub binding: Option<StmtRef>,
}

#[derive(Clone)]
pub struct UseDirective {
    pub id: NodeId,
    pub span: Span,
    pub name_ref: ExprRef,
    pub expression: Option<ExprRef>,
}

#[derive(Clone)]
pub struct OnDirectiveLegacy {
    pub id: NodeId,
    pub span: Span,

    pub name: String,

    pub name_span: Span,
    pub expression: Option<ExprRef>,

    pub modifiers: Vec<String>,
}

impl OnDirectiveLegacy {
    pub fn parsed_modifiers(&self) -> OnDirectiveModifiers {
        OnDirectiveModifiers::from_modifiers(&self.modifiers)
    }
}

pub struct OnDirectiveModifiers {
    pub stop_propagation: bool,
    pub stop_immediate_propagation: bool,
    pub prevent_default: bool,
    pub self_: bool,
    pub trusted: bool,
    pub once: bool,
    pub capture: bool,

    pub passive: Option<bool>,
}

impl OnDirectiveModifiers {
    pub fn from_modifiers(modifiers: &[String]) -> Self {
        let mut result = Self {
            stop_propagation: false,
            stop_immediate_propagation: false,
            prevent_default: false,
            self_: false,
            trusted: false,
            once: false,
            capture: false,
            passive: None,
        };
        for m in modifiers {
            match m.as_str() {
                "stopPropagation" => result.stop_propagation = true,
                "stopImmediatePropagation" => result.stop_immediate_propagation = true,
                "preventDefault" => result.prevent_default = true,
                "self" => result.self_ = true,
                "trusted" => result.trusted = true,
                "once" => result.once = true,
                "capture" => result.capture = true,
                "passive" => result.passive = Some(true),
                "nonpassive" => result.passive = Some(false),
                _ => {}
            }
        }
        result
    }

    pub fn handler_wrappers(&self) -> impl Iterator<Item = &'static str> {
        [
            (self.stop_propagation, "stopPropagation"),
            (self.stop_immediate_propagation, "stopImmediatePropagation"),
            (self.prevent_default, "preventDefault"),
            (self.self_, "self"),
            (self.trusted, "trusted"),
            (self.once, "once"),
        ]
        .into_iter()
        .filter_map(|(active, name)| active.then_some(name))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TransitionDirection {
    Both,

    In,

    Out,
}

#[derive(Clone)]
pub struct TransitionDirective {
    pub id: NodeId,
    pub span: Span,
    pub name_ref: ExprRef,
    pub expression: Option<ExprRef>,

    pub modifiers: Vec<String>,

    pub direction: TransitionDirection,
}

#[derive(Clone)]
pub struct AnimateDirective {
    pub id: NodeId,
    pub span: Span,
    pub name_ref: ExprRef,
    pub expression: Option<ExprRef>,
}

#[derive(Clone)]
pub struct AttachTag {
    pub id: NodeId,
    pub span: Span,
    pub expression: ExprRef,
}

pub struct Script {
    pub id: NodeId,
    pub span: Span,

    pub content_span: Span,
    pub context: ScriptContext,
    pub language: ScriptLanguage,

    pub context_deprecated: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScriptContext {
    Default,
    Module,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScriptLanguage {
    JavaScript,
    TypeScript,
}

pub struct RawBlock {
    pub span: Span,
    pub content_span: Span,
}

pub struct SvelteOptions {
    pub span: Span,

    pub runes: Option<bool>,

    pub namespace: Option<Namespace>,

    pub css: Option<CssMode>,

    pub custom_element: Option<CustomElementConfig>,

    pub immutable: Option<bool>,

    pub accessors: Option<bool>,

    pub preserve_whitespace: Option<bool>,

    pub attributes: Vec<Attribute>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Namespace {
    Html,
    Svg,
    Mathml,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CssMode {
    Injected,
}

pub enum CustomElementConfig {
    Tag(String),

    Expression(Span),
}

pub struct AstStore {
    nodes: Vec<Node>,
    fragments: Vec<Fragment>,

    node_to_fragment: Vec<Option<FragmentId>>,
}

impl AstStore {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            fragments: Vec::new(),
            node_to_fragment: Vec::new(),
        }
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self {
            nodes: Vec::with_capacity(cap),
            fragments: Vec::new(),
            node_to_fragment: Vec::new(),
        }
    }

    pub fn freeze_node_fragments(&mut self) {
        self.node_to_fragment = vec![None; self.nodes.len()];
        for fragment in &self.fragments {
            for &nid in &fragment.nodes {
                self.node_to_fragment[nid.0 as usize] = Some(fragment.id);
            }
        }
    }

    pub fn node_fragment(&self, node_id: NodeId) -> Option<FragmentId> {
        self.node_to_fragment
            .get(node_id.0 as usize)
            .copied()
            .flatten()
    }

    pub fn push_fragment(&mut self, role: FragmentRole, nodes: Vec<NodeId>) -> FragmentId {
        let id = FragmentId(self.fragments.len() as u32);
        self.fragments.push(Fragment {
            id,
            role,
            nodes,
            owner: None,
        });
        id
    }

    pub fn reserve_fragment(&mut self, role: FragmentRole) -> FragmentId {
        let id = FragmentId(self.fragments.len() as u32);
        self.fragments.push(Fragment {
            id,
            role,
            nodes: Vec::new(),
            owner: None,
        });
        id
    }

    pub fn set_fragment_owner(&mut self, id: FragmentId, owner: NodeId) {
        self.fragments[id.0 as usize].owner = Some(owner);
    }

    pub fn fragment(&self, id: FragmentId) -> &Fragment {
        &self.fragments[id.0 as usize]
    }

    pub fn fragment_mut(&mut self, id: FragmentId) -> &mut Fragment {
        &mut self.fragments[id.0 as usize]
    }

    pub fn fragment_nodes(&self, id: FragmentId) -> &[NodeId] {
        &self.fragments[id.0 as usize].nodes
    }

    pub fn fragments_len(&self) -> u32 {
        self.fragments.len() as u32
    }

    pub fn iter_fragments(&self) -> impl Iterator<Item = &Fragment> {
        self.fragments.iter()
    }

    pub fn push(&mut self, mut node: Node) -> NodeId {
        let id = NodeId(self.nodes.len() as u32);
        node.set_id(id);
        self.nodes.push(node);
        id
    }

    pub fn reserve(&mut self) -> NodeId {
        let id = NodeId(self.nodes.len() as u32);
        self.nodes.push(Node::Error(ErrorNode {
            id,
            span: Span::new(0, 0),
        }));
        id
    }

    pub fn get(&self, id: NodeId) -> &Node {
        &self.nodes[id.0 as usize]
    }

    pub fn get_mut(&mut self, id: NodeId) -> &mut Node {
        &mut self.nodes[id.0 as usize]
    }

    pub fn take(&mut self, id: NodeId) -> Node {
        std::mem::replace(
            &mut self.nodes[id.0 as usize],
            Node::Error(ErrorNode {
                id,
                span: Span::new(0, 0),
            }),
        )
    }

    pub fn replace(&mut self, id: NodeId, mut node: Node) {
        node.set_id(id);
        self.nodes[id.0 as usize] = node;
    }

    pub fn len(&self) -> u32 {
        self.nodes.len() as u32
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn iter_nodes(&self) -> impl Iterator<Item = &Node> {
        self.nodes.iter()
    }
}

impl Default for AstStore {
    fn default() -> Self {
        Self::new()
    }
}

macro_rules! impl_store_accessors {
    ( $( $method:ident -> $Type:ident / $as_method:ident ),+ $(,)? ) => {
        impl AstStore {
            $(
                pub fn $method(&self, id: NodeId) -> &$Type {
                    self.get(id).$as_method()
                        .unwrap_or_else(|| panic!("{:?} is not a {}", id, stringify!($Type)))
                }
            )+
        }
    };
}

impl_store_accessors! {
    text -> Text / as_text,
    element -> Element / as_element,
    component_node -> ComponentNode / as_component_node,
    comment -> Comment / as_comment,
    expression_tag -> ExpressionTag / as_expression_tag,
    if_block -> IfBlock / as_if_block,
    each_block -> EachBlock / as_each_block,
    snippet_block -> SnippetBlock / as_snippet_block,
    render_tag -> RenderTag / as_render_tag,
    html_tag -> HtmlTag / as_html_tag,
    const_tag -> ConstTag / as_const_tag,
    debug_tag -> DebugTag / as_debug_tag,
    key_block -> KeyBlock / as_key_block,
    svelte_head -> SvelteHead / as_svelte_head,
    svelte_element -> SvelteElement / as_svelte_element,
    svelte_window -> SvelteWindow / as_svelte_window,
    svelte_document -> SvelteDocument / as_svelte_document,
    svelte_body -> SvelteBody / as_svelte_body,
    svelte_boundary -> SvelteBoundary / as_svelte_boundary,
    await_block -> AwaitBlock / as_await_block,
    error_node -> ErrorNode / as_error,
}
