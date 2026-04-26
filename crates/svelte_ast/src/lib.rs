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

/// Elements that belong to the SVG namespace.
/// Matches the reference Svelte compiler's `SVG_ELEMENTS` list.
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

/// Elements that belong to the MathML namespace.
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

/// HTML elements where inter-element whitespace should be removed entirely.
const WHITESPACE_REMOVABLE_ELEMENTS: &[&str] = &[
    "select", "tr", "table", "tbody", "thead", "tfoot", "colgroup", "datalist",
];

pub fn is_whitespace_removable_parent(name: &str) -> bool {
    WHITESPACE_REMOVABLE_ELEMENTS.contains(&name)
}

/// Special Svelte pseudo-element names that are handled as components.
pub const SVELTE_COMPONENT: &str = "svelte:component";
/// Special Svelte pseudo-element for recursive self-reference.
pub const SVELTE_SELF: &str = "svelte:self";
/// Transparent wrapper for named slot content (`<svelte:fragment slot="name">`).
pub const SVELTE_FRAGMENT: &str = "svelte:fragment";
/// Component options tag.
pub const SVELTE_OPTIONS: &str = "svelte:options";
/// Renders into `<head>` at SSR time; no-op on the client.
pub const SVELTE_HEAD: &str = "svelte:head";
/// Binds to `window` event listeners and properties.
pub const SVELTE_WINDOW: &str = "svelte:window";
/// Binds to `document` event listeners and properties.
pub const SVELTE_DOCUMENT: &str = "svelte:document";
/// Binds to `document.body` event listeners.
pub const SVELTE_BODY: &str = "svelte:body";
/// Dynamic element tag (`<svelte:element this={tag}>`).
pub const SVELTE_ELEMENT: &str = "svelte:element";
/// Error boundary (`<svelte:boundary>`).
pub const SVELTE_BOUNDARY: &str = "svelte:boundary";

/// Unique node identifier, assigned during parsing.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct NodeId(pub u32);

/// Unique fragment identifier, assigned during parsing.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct FragmentId(pub u32);

// ---------------------------------------------------------------------------
// Root
// ---------------------------------------------------------------------------

pub struct Component {
    pub root: FragmentId,
    pub store: AstStore,
    /// Instance-level `<script>` block (runs once per component instance).
    pub instance_script: Option<Script>,
    /// Module-level `<script module>` block (runs once when the module loads).
    pub module_script: Option<Script>,
    pub css: Option<RawBlock>,
    pub options: Option<SvelteOptions>,
    /// Full source text of the .svelte file.
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

    /// Dummy `Component` for standalone `.svelte.js` modules (no template).
    /// Used only to satisfy analysis APIs that expect `&Component`.
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

    /// Total number of NodeId slots allocated during parsing (nodes + attrs + misc).
    pub fn node_count(&self) -> u32 {
        self.store.len()
    }

    pub fn fragment_count(&self) -> u32 {
        self.store.fragments_len()
    }

    /// Get source text for a span.
    pub fn source_text(&self, span: Span) -> &str {
        &self.source[span.start as usize..span.end as usize]
    }
}

// ---------------------------------------------------------------------------
// Fragment
// ---------------------------------------------------------------------------

/// Role of a fragment in the template — what parent construct owns it.
///
/// A single AST node (e.g. `IfBlock`) may produce multiple fragments
/// (consequent + alternate), so the role disambiguates them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FragmentRole {
    Root,
    Element,
    ComponentChildren,
    /// LEGACY(svelte4): named slot body within a component.
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
    /// NodeId владельца fragment (Element/IfBlock/...). `None` для корня.
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

// ---------------------------------------------------------------------------
// Nodes
// ---------------------------------------------------------------------------

/// Generates the `Node` enum plus `node_id()`, `span()`, `is_*()`, and `as_*()` helpers.
/// Every variant's inner type must have `pub id: NodeId` and `pub span: Span`.
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

            /// Set the NodeId on the inner type (used by AstStore::push).
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

// ---------------------------------------------------------------------------
// Text
// ---------------------------------------------------------------------------

pub struct Text {
    pub id: NodeId,
    pub span: Span,
    pub decoded: Option<String>,
}

impl Text {
    /// Returns the original source slice for diagnostics and span-based lookups.
    pub fn raw_value<'a>(&self, source: &'a str) -> &'a str {
        &source[self.span.start as usize..self.span.end as usize]
    }

    /// Returns decoded text content when entities were normalized during parsing.
    pub fn value<'a>(&'a self, source: &'a str) -> &'a str {
        self.decoded
            .as_deref()
            .unwrap_or_else(|| self.raw_value(source))
    }
}

// ---------------------------------------------------------------------------
// Element
// ---------------------------------------------------------------------------

pub struct Element {
    pub id: NodeId,
    pub span: Span,
    pub name: String,
    pub self_closing: bool,
    pub attributes: Vec<Attribute>,
    pub fragment: FragmentId,
}

/// LEGACY(svelte4): legacy `<slot>` pseudo-element. Deprecated in Svelte 5, remove in Svelte 6.
pub struct SlotElementLegacy {
    pub id: NodeId,
    pub span: Span,
    pub attributes: Vec<Attribute>,
    pub fragment: FragmentId,
}

// ---------------------------------------------------------------------------
// ComponentNode
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// Comment
// ---------------------------------------------------------------------------

pub struct Comment {
    pub id: NodeId,
    pub span: Span,
}

impl Comment {
    pub fn value<'a>(&self, source: &'a str) -> &'a str {
        &source[self.span.start as usize..self.span.end as usize]
    }
}

// ---------------------------------------------------------------------------
// ExpressionTag — {expr}
// ---------------------------------------------------------------------------

pub struct ExpressionTag {
    pub id: NodeId,
    /// Span of the whole `{expr}` including braces.
    pub span: Span,
    pub expression: ExprRef,
}

// ---------------------------------------------------------------------------
// IfBlock
// ---------------------------------------------------------------------------

pub struct IfBlock {
    pub id: NodeId,
    pub span: Span,
    pub test: ExprRef,
    pub elseif: bool,
    pub consequent: FragmentId,
    pub alternate: Option<FragmentId>,
}

// ---------------------------------------------------------------------------
// EachBlock
// ---------------------------------------------------------------------------

pub struct EachBlock {
    pub id: NodeId,
    pub span: Span,
    pub expression: ExprRef,
    pub context: Option<StmtRef>,
    pub index: Option<StmtRef>,
    pub key: Option<ExprRef>,
    /// Unique NodeId for the key expression (separate from block id to avoid NodeId collision).
    pub key_id: Option<NodeId>,
    pub body: FragmentId,
    pub fallback: Option<FragmentId>,
}

// ---------------------------------------------------------------------------
// SnippetBlock — {#snippet name(params)}...{/snippet}
// ---------------------------------------------------------------------------

pub struct SnippetBlock {
    pub id: NodeId,
    pub span: Span,
    pub decl: StmtRef,
    pub body: FragmentId,
}

impl SnippetBlock {
    /// Get the snippet name from source text (everything before '(' or the whole expression).
    pub fn name<'a>(&self, source: &'a str) -> &'a str {
        let expr = &source[self.decl.span.start as usize..self.decl.span.end as usize];
        match expr.find('(') {
            Some(pos) => &expr[..pos],
            None => expr,
        }
    }
}

// ---------------------------------------------------------------------------
// RenderTag — {@render snippet(args)}
// ---------------------------------------------------------------------------

pub struct RenderTag {
    pub id: NodeId,
    pub span: Span,
    pub expression: ExprRef,
}

// ---------------------------------------------------------------------------
// HtmlTag — {@html expr}
// ---------------------------------------------------------------------------

pub struct HtmlTag {
    pub id: NodeId,
    pub span: Span,
    pub expression: ExprRef,
}

// ---------------------------------------------------------------------------
// ConstTag — {@const decl}
// ---------------------------------------------------------------------------

pub struct ConstTag {
    pub id: NodeId,
    pub span: Span,
    pub decl: StmtRef,
}

// ---------------------------------------------------------------------------
// DebugTag — {@debug vars}
// ---------------------------------------------------------------------------

pub struct DebugTag {
    pub id: NodeId,
    pub span: Span,
    pub identifier_refs: Vec<ExprRef>,
}

// ---------------------------------------------------------------------------
// KeyBlock — {#key expr}...{/key}
// ---------------------------------------------------------------------------

pub struct KeyBlock {
    pub id: NodeId,
    pub span: Span,
    pub expression: ExprRef,
    pub fragment: FragmentId,
}

// ---------------------------------------------------------------------------
// SvelteHead — <svelte:head>...</svelte:head>
// ---------------------------------------------------------------------------

pub struct SvelteHead {
    pub id: NodeId,
    pub span: Span,
    pub fragment: FragmentId,
}

/// LEGACY(svelte4): transparent named-slot wrapper `<svelte:fragment>`. Remove in Svelte 6.
pub struct SvelteFragmentLegacy {
    pub id: NodeId,
    pub span: Span,
    pub attributes: Vec<Attribute>,
    pub fragment: FragmentId,
}

// ---------------------------------------------------------------------------
// SvelteElement — <svelte:element this={tag}>...</svelte:element>
// ---------------------------------------------------------------------------

pub struct SvelteElement {
    pub id: NodeId,
    pub span: Span,
    /// Span of the `this` expression (the dynamic tag).
    pub tag_span: Span,
    /// JS expression (None when `static_tag`).
    pub tag: Option<ExprRef>,
    /// True when `this="literal"` (StringAttribute) — tag is a static string, not a JS expression.
    pub static_tag: bool,
    pub attributes: Vec<Attribute>,
    pub fragment: FragmentId,
}

// ---------------------------------------------------------------------------
// SvelteWindow — <svelte:window on:event={handler} bind:scrollY />
// ---------------------------------------------------------------------------

pub struct SvelteWindow {
    pub id: NodeId,
    pub span: Span,
    pub attributes: Vec<Attribute>,
    pub fragment: FragmentId,
}

// ---------------------------------------------------------------------------
// SvelteDocument — <svelte:document on:event={handler} bind:activeElement />
// ---------------------------------------------------------------------------

pub struct SvelteDocument {
    pub id: NodeId,
    pub span: Span,
    pub attributes: Vec<Attribute>,
    pub fragment: FragmentId,
}

// ---------------------------------------------------------------------------
// SvelteBody — <svelte:body onclick={handler} use:action />
// ---------------------------------------------------------------------------

pub struct SvelteBody {
    pub id: NodeId,
    pub span: Span,
    pub attributes: Vec<Attribute>,
    pub fragment: FragmentId,
}

// ---------------------------------------------------------------------------
// SvelteBoundary — <svelte:boundary onerror={fn} failed={snippet}>...</svelte:boundary>
// ---------------------------------------------------------------------------

pub struct SvelteBoundary {
    pub id: NodeId,
    pub span: Span,
    pub attributes: Vec<Attribute>,
    pub fragment: FragmentId,
}

// ---------------------------------------------------------------------------
// AwaitBlock — {#await expr}...{:then val}...{:catch err}...{/await}
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// Attributes
// ---------------------------------------------------------------------------

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
    /// name="string"
    StringAttribute(StringAttribute),
    /// name={expr} or shorthand `{name}` (ExpressionAttribute with `shorthand: true`
    /// and `name == expression text`). Shorthand is represented by this variant
    /// rather than a separate one so downstream passes have one expression path.
    ExpressionAttribute(ExpressionAttribute),
    /// name (boolean, no value)
    BooleanAttribute(BooleanAttribute),
    /// name="text{expr}text"
    ConcatenationAttribute(ConcatenationAttribute),
    /// {...expr} — spread attribute
    SpreadAttribute(SpreadAttribute),
    /// class:name or class:name={expr}
    ClassDirective(ClassDirective),
    /// style:name or style:name={expr} or style:name|important
    StyleDirective(StyleDirective),
    /// bind:name or bind:name={expr}
    BindDirective(BindDirective),
    /// LEGACY(svelte4): let:name or let:name={expr} for slot props. Deprecated in Svelte 5, remove in Svelte 6.
    LetDirectiveLegacy(LetDirectiveLegacy),
    /// use:name or use:name={expr}
    UseDirective(UseDirective),
    /// LEGACY(svelte4): on:event or on:event={handler} or on:event|modifier={handler}
    /// Deprecated in Svelte 5, remove in Svelte 6.
    OnDirectiveLegacy(OnDirectiveLegacy),
    /// transition:name, in:name, or out:name
    TransitionDirective(TransitionDirective),
    /// animate:name or animate:name={expr}
    AnimateDirective(AnimateDirective),
    /// {@attach expr} — element attachment (Svelte 5.29+)
    AttachTag(AttachTag),
}

impl Attribute {
    /// Returns the attribute name if it is stored as a `String` field.
    /// Variants with `Span`-based names (`UseDirective`, `TransitionDirective`,
    /// `AnimateDirective`) and nameless variants return `None`.
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

    /// Name of this attribute if it is a plain HTML attribute (not a Svelte directive).
    /// Returns `""` for directive and nameless variants — use this when checking raw HTML
    /// attribute names (e.g. `is`, colon presence, React-style names). The empty string
    /// never matches any valid attribute name check.
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
    /// Pre-computed event name (after "on" prefix), if this is an event attribute.
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
    /// `true` when the source used quotes (`foo="a{b}"`), `false` for unquoted forms (`foo=a{b}`).
    pub quoted: bool,
    pub parts: Vec<ConcatPart>,
}

#[derive(Clone)]
pub enum ConcatPart {
    /// Static text portion.
    Static(String),
    /// Dynamic expression portion with its own NodeId to avoid sharing the parent attribute's id.
    Dynamic { id: NodeId, expr: ExprRef },
}

/// {...expr} — spread attribute.
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
    /// `true` iff the directive has no explicit value (`style:name`).
    pub shorthand: bool,
    pub value: StyleDirectiveValue,
    pub important: bool,
}

#[derive(Clone)]
pub enum StyleDirectiveValue {
    /// style:name={expr} — expression in braces, or shorthand `style:name`
    /// (where the span of `name` acts as the expression). Use the
    /// `shorthand: bool` on `StyleDirective` to distinguish emission.
    Expression,
    /// style:name="string" — static string value
    String(String),
    /// style:name="text{expr}text" — concatenation of static and dynamic parts
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

/// LEGACY(svelte4): `let:` slot-prop binding syntax. Deprecated in Svelte 5, remove in Svelte 6.
#[derive(Clone)]
pub struct LetDirectiveLegacy {
    pub id: NodeId,
    pub span: Span,
    /// Slot-prop key (e.g. `item` in `let:item` or `let:item={value}`).
    pub name: String,
    /// Span of the slot-prop key in source.
    pub name_span: Span,
    pub binding: Option<StmtRef>,
}

/// use:name or use:name={expr}
#[derive(Clone)]
pub struct UseDirective {
    pub id: NodeId,
    pub span: Span,
    pub name_ref: ExprRef,
    pub expression: Option<ExprRef>,
}

/// LEGACY(svelte4): on:directive syntax. Deprecated in Svelte 5, remove in Svelte 6.
#[derive(Clone)]
pub struct OnDirectiveLegacy {
    pub id: NodeId,
    pub span: Span,
    /// Event name (e.g., "click" in `on:click`).
    pub name: String,
    /// Span of the event name in source (e.g., "click" span in `on:click`).
    pub name_span: Span,
    pub expression: Option<ExprRef>,
    /// Modifiers like "preventDefault", "stopPropagation", "capture", etc.
    pub modifiers: Vec<String>,
}

impl OnDirectiveLegacy {
    pub fn parsed_modifiers(&self) -> OnDirectiveModifiers {
        OnDirectiveModifiers::from_modifiers(&self.modifiers)
    }
}

/// LEGACY(svelte4): Pre-classified modifier flags for on:directive.
pub struct OnDirectiveModifiers {
    pub stop_propagation: bool,
    pub stop_immediate_propagation: bool,
    pub prevent_default: bool,
    pub self_: bool,
    pub trusted: bool,
    pub once: bool,
    pub capture: bool,
    /// `Some(true)` = passive, `Some(false)` = nonpassive, `None` = unset.
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

    /// Ordered handler wrapper modifiers that are active.
    /// Order matches Svelte reference (stopPropagation → once).
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

/// Direction of a transition directive.
#[derive(Clone, Debug, PartialEq)]
pub enum TransitionDirection {
    /// `transition:` — plays on both intro and outro
    Both,
    /// `in:` — plays only on intro
    In,
    /// `out:` — plays only on outro
    Out,
}

/// transition:name, in:name, or out:name directive.
#[derive(Clone)]
pub struct TransitionDirective {
    pub id: NodeId,
    pub span: Span,
    pub name_ref: ExprRef,
    pub expression: Option<ExprRef>,
    /// Modifiers like "local", "global".
    pub modifiers: Vec<String>,
    /// Whether this is `transition:`, `in:`, or `out:`.
    pub direction: TransitionDirection,
}

/// animate:name or animate:name={expr}
#[derive(Clone)]
pub struct AnimateDirective {
    pub id: NodeId,
    pub span: Span,
    pub name_ref: ExprRef,
    pub expression: Option<ExprRef>,
}

/// {@attach expr} — element attachment (Svelte 5.29+).
/// Modern alternative to `use:action`. Re-runs on reactive dependency changes.
#[derive(Clone)]
pub struct AttachTag {
    pub id: NodeId,
    pub span: Span,
    pub expression: ExprRef,
}

// ---------------------------------------------------------------------------
// Script
// ---------------------------------------------------------------------------

pub struct Script {
    pub id: NodeId,
    pub span: Span,
    /// Span of the content between <script> and </script>.
    pub content_span: Span,
    pub context: ScriptContext,
    pub language: ScriptLanguage,
    /// `true` when the legacy `context="module"` attribute was used instead of `module`.
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

// ---------------------------------------------------------------------------
// RawBlock (CSS)
// ---------------------------------------------------------------------------

pub struct RawBlock {
    pub span: Span,
    pub content_span: Span,
}

// ---------------------------------------------------------------------------
// SvelteOptions — parsed from <svelte:options> tag
// ---------------------------------------------------------------------------

/// Component-level options parsed from `<svelte:options .../>`.
/// Stored on `Component.options` after extraction from the fragment.
pub struct SvelteOptions {
    pub span: Span,
    /// Whether this component uses runes mode. `None` = inherit from compiler options.
    pub runes: Option<bool>,
    /// Component namespace: "html" (default), "svg", or "mathml".
    pub namespace: Option<Namespace>,
    /// CSS injection mode. Currently only "injected" is valid.
    pub css: Option<CssMode>,
    /// Custom element tag name (simple string form).
    pub custom_element: Option<CustomElementConfig>,
    /// LEGACY(svelte4): immutable mode. Deprecated in Svelte 5.
    pub immutable: Option<bool>,
    /// LEGACY(svelte4): accessors mode. Deprecated in Svelte 5.
    pub accessors: Option<bool>,
    /// Preserve whitespace in template.
    pub preserve_whitespace: Option<bool>,
    /// Raw attributes from the `<svelte:options>` tag, preserved for tooling.
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

/// Custom element configuration from `<svelte:options customElement=...>`.
pub enum CustomElementConfig {
    /// Simple string form: `customElement="my-tag"`
    Tag(String),
    /// Object form: `customElement={{ tag: "...", ... }}`
    /// Stores the expression span for deferred parsing in analysis.
    Expression(Span),
}

// ---------------------------------------------------------------------------
// AstStore — flat arena for all template nodes
// ---------------------------------------------------------------------------

/// Flat arena storing all template nodes and fragments.
/// `NodeId` and `FragmentId` are indices into the respective vectors.
/// Reserved slots (for attributes, key_id, script) hold placeholder Error nodes.
pub struct AstStore {
    nodes: Vec<Node>,
    fragments: Vec<Fragment>,
    /// Reverse index: NodeId → FragmentId of its containing fragment.
    /// Empty until `freeze_node_fragments` is called by the parser at the
    /// end of construction.
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

    /// Create a store pre-allocated for the expected number of nodes.
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

    /// Push a template node into the store. Sets the node's id to match its index.
    pub fn push(&mut self, mut node: Node) -> NodeId {
        let id = NodeId(self.nodes.len() as u32);
        node.set_id(id);
        self.nodes.push(node);
        id
    }

    /// Reserve a slot for a non-node id (attributes, key expressions, script, etc.).
    /// Fills with a placeholder — must never be accessed via `get()`.
    pub fn reserve(&mut self) -> NodeId {
        let id = NodeId(self.nodes.len() as u32);
        self.nodes.push(Node::Error(ErrorNode {
            id,
            span: Span::new(0, 0),
        }));
        id
    }

    /// Get a node by id.
    pub fn get(&self, id: NodeId) -> &Node {
        &self.nodes[id.0 as usize]
    }

    /// Get a mutable node by id.
    pub fn get_mut(&mut self, id: NodeId) -> &mut Node {
        &mut self.nodes[id.0 as usize]
    }

    /// Take a node out of the store, leaving a placeholder in its slot.
    pub fn take(&mut self, id: NodeId) -> Node {
        std::mem::replace(
            &mut self.nodes[id.0 as usize],
            Node::Error(ErrorNode {
                id,
                span: Span::new(0, 0),
            }),
        )
    }

    /// Replace a node in-place (used by svelte:element/boundary conversion).
    pub fn replace(&mut self, id: NodeId, mut node: Node) {
        node.set_id(id);
        self.nodes[id.0 as usize] = node;
    }

    /// Total number of allocated slots (nodes + reserved).
    pub fn len(&self) -> u32 {
        self.nodes.len() as u32
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Iterate every node in store insertion order.
    pub fn iter_nodes(&self) -> impl Iterator<Item = &Node> {
        self.nodes.iter()
    }
}

impl Default for AstStore {
    fn default() -> Self {
        Self::new()
    }
}

/// Typed accessor macro for AstStore — generates methods like `element(id) -> &Element`.
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
