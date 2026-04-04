use svelte_span::Span;

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

/// Unique node identifier, assigned during parsing.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct NodeId(pub u32);

// ---------------------------------------------------------------------------
// Root
// ---------------------------------------------------------------------------

pub struct Component {
    pub fragment: Fragment,
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
        fragment: Fragment,
        store: AstStore,
        instance_script: Option<Script>,
        module_script: Option<Script>,
        css: Option<RawBlock>,
    ) -> Self {
        Self {
            fragment,
            store,
            instance_script,
            module_script,
            css,
            options: None,
            source,
        }
    }

    /// Total number of NodeId slots allocated during parsing (nodes + attrs + misc).
    pub fn node_count(&self) -> u32 {
        self.store.len()
    }

    /// Get source text for a span.
    pub fn source_text(&self, span: Span) -> &str {
        &self.source[span.start as usize..span.end as usize]
    }
}

// ---------------------------------------------------------------------------
// Fragment
// ---------------------------------------------------------------------------

pub struct Fragment {
    pub nodes: Vec<NodeId>,
}

impl Fragment {
    pub fn new(nodes: Vec<NodeId>) -> Self {
        Self { nodes }
    }

    pub fn empty() -> Self {
        Self { nodes: vec![] }
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
    pub fragment: Fragment,
}

impl Element {
    /// Clone element metadata without children (fragment set to empty).
    /// Used when we need owned attribute data while borrowing the AST elsewhere.
    pub fn clone_without_fragment(&self) -> Element {
        Element {
            id: self.id,
            span: self.span,
            name: self.name.clone(),
            self_closing: self.self_closing,
            attributes: self.attributes.clone(),
            fragment: Fragment::empty(),
        }
    }
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
    pub fragment: Fragment,
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
    /// Span of just the JS expression inside the braces.
    pub expression_span: Span,
}

// ---------------------------------------------------------------------------
// IfBlock
// ---------------------------------------------------------------------------

pub struct IfBlock {
    pub id: NodeId,
    pub span: Span,
    /// Span of the JS test expression.
    pub test_span: Span,
    pub elseif: bool,
    pub consequent: Fragment,
    pub alternate: Option<Fragment>,
}

// ---------------------------------------------------------------------------
// EachBlock
// ---------------------------------------------------------------------------

pub struct EachBlock {
    pub id: NodeId,
    pub span: Span,
    /// Span of the collection expression.
    pub expression_span: Span,
    /// Span of the iteration variable (e.g., `item` or `{ value, flag }`). None for `{#each items}` without `as`.
    pub context_span: Option<Span>,
    /// Span of the index variable, if any.
    pub index_span: Option<Span>,
    /// Span of the key expression, if any.
    pub key_span: Option<Span>,
    /// Unique NodeId for the key expression (separate from block id to avoid NodeId collision).
    pub key_id: Option<NodeId>,
    pub body: Fragment,
    pub fallback: Option<Fragment>,
}

// ---------------------------------------------------------------------------
// SnippetBlock — {#snippet name(params)}...{/snippet}
// ---------------------------------------------------------------------------

pub struct SnippetBlock {
    pub id: NodeId,
    pub span: Span,
    /// Span covering `name(params)` or just `name` — the full snippet declaration expression.
    pub expression_span: Span,
    pub body: Fragment,
}

impl SnippetBlock {
    /// Get the snippet name from source text (everything before '(' or the whole expression).
    pub fn name<'a>(&self, source: &'a str) -> &'a str {
        let expr = &source[self.expression_span.start as usize..self.expression_span.end as usize];
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
    /// Span of the full call expression: "greeting(message)" in `{@render greeting(message)}`.
    pub expression_span: Span,
}

// ---------------------------------------------------------------------------
// HtmlTag — {@html expr}
// ---------------------------------------------------------------------------

pub struct HtmlTag {
    pub id: NodeId,
    pub span: Span,
    /// Span of the JS expression: "content" in `{@html content}`.
    pub expression_span: Span,
}

// ---------------------------------------------------------------------------
// ConstTag — {@const decl}
// ---------------------------------------------------------------------------

pub struct ConstTag {
    pub id: NodeId,
    pub span: Span,
    /// Span of the full declaration: `const doubled = item * 2` in `{@const doubled = item * 2}`.
    pub expression_span: Span,
}

// ---------------------------------------------------------------------------
// DebugTag — {@debug vars}
// ---------------------------------------------------------------------------

pub struct DebugTag {
    pub id: NodeId,
    pub span: Span,
    /// Spans of the identifier names. Empty vec means `{@debug}` (debug all).
    pub identifiers: Vec<Span>,
}

// ---------------------------------------------------------------------------
// KeyBlock — {#key expr}...{/key}
// ---------------------------------------------------------------------------

pub struct KeyBlock {
    pub id: NodeId,
    pub span: Span,
    /// Span of the JS expression: "count" in `{#key count}`.
    pub expression_span: Span,
    pub fragment: Fragment,
}

// ---------------------------------------------------------------------------
// SvelteHead — <svelte:head>...</svelte:head>
// ---------------------------------------------------------------------------

pub struct SvelteHead {
    pub id: NodeId,
    pub span: Span,
    pub fragment: Fragment,
}

// ---------------------------------------------------------------------------
// SvelteElement — <svelte:element this={tag}>...</svelte:element>
// ---------------------------------------------------------------------------

pub struct SvelteElement {
    pub id: NodeId,
    pub span: Span,
    /// Span of the `this` expression (the dynamic tag).
    pub tag_span: Span,
    /// True when `this="literal"` (StringAttribute) — tag is a static string, not a JS expression.
    pub static_tag: bool,
    pub attributes: Vec<Attribute>,
    pub fragment: Fragment,
}

// ---------------------------------------------------------------------------
// SvelteWindow — <svelte:window on:event={handler} bind:scrollY />
// ---------------------------------------------------------------------------

pub struct SvelteWindow {
    pub id: NodeId,
    pub span: Span,
    pub attributes: Vec<Attribute>,
    pub fragment: Fragment,
}

// ---------------------------------------------------------------------------
// SvelteDocument — <svelte:document on:event={handler} bind:activeElement />
// ---------------------------------------------------------------------------

pub struct SvelteDocument {
    pub id: NodeId,
    pub span: Span,
    pub attributes: Vec<Attribute>,
    pub fragment: Fragment,
}

// ---------------------------------------------------------------------------
// SvelteBody — <svelte:body onclick={handler} use:action />
// ---------------------------------------------------------------------------

pub struct SvelteBody {
    pub id: NodeId,
    pub span: Span,
    pub attributes: Vec<Attribute>,
    pub fragment: Fragment,
}

// ---------------------------------------------------------------------------
// SvelteBoundary — <svelte:boundary onerror={fn} failed={snippet}>...</svelte:boundary>
// ---------------------------------------------------------------------------

pub struct SvelteBoundary {
    pub id: NodeId,
    pub span: Span,
    pub attributes: Vec<Attribute>,
    pub fragment: Fragment,
}

// ---------------------------------------------------------------------------
// AwaitBlock — {#await expr}...{:then val}...{:catch err}...{/await}
// ---------------------------------------------------------------------------

pub struct AwaitBlock {
    pub id: NodeId,
    pub span: Span,
    /// Span of the promise expression.
    pub expression_span: Span,
    /// Span of the then binding pattern (e.g., `value` or `{name, age}`). None if no binding.
    pub value_span: Option<Span>,
    /// Span of the catch binding pattern (e.g., `error`). None if no binding.
    pub error_span: Option<Span>,
    /// Content shown while promise is pending. None if short form.
    pub pending: Option<Fragment>,
    /// Content shown when promise resolves.
    pub then: Option<Fragment>,
    /// Content shown when promise rejects.
    pub catch: Option<Fragment>,
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
        }
    };
}

impl_attr_enum! {
    /// name="string"
    StringAttribute(StringAttribute),
    /// name={expr}
    ExpressionAttribute(ExpressionAttribute),
    /// name (boolean, no value)
    BooleanAttribute(BooleanAttribute),
    /// name="text{expr}text"
    ConcatenationAttribute(ConcatenationAttribute),
    /// {name} — shorthand attribute
    Shorthand(Shorthand),
    /// {...expr} — spread attribute
    SpreadAttribute(SpreadAttribute),
    /// class:name or class:name={expr}
    ClassDirective(ClassDirective),
    /// style:name or style:name={expr} or style:name|important
    StyleDirective(StyleDirective),
    /// bind:name or bind:name={expr}
    BindDirective(BindDirective),
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

#[derive(Clone)]
pub struct StringAttribute {
    pub id: NodeId,
    pub name: String,
    pub value_span: Span,
}

#[derive(Clone)]
pub struct ExpressionAttribute {
    pub id: NodeId,
    pub name: String,
    /// Span of just the JS expression.
    pub expression_span: Span,
    pub shorthand: bool,
    /// Pre-computed event name (after "on" prefix), if this is an event attribute.
    pub event_name: Option<String>,
}

#[derive(Clone)]
pub struct BooleanAttribute {
    pub id: NodeId,
    pub name: String,
}

#[derive(Clone)]
pub struct ConcatenationAttribute {
    pub id: NodeId,
    pub name: String,
    pub parts: Vec<ConcatPart>,
}

#[derive(Clone)]
pub enum ConcatPart {
    /// Static text portion.
    Static(String),
    /// Dynamic expression portion with its own NodeId to avoid sharing the parent attribute's id.
    Dynamic { id: NodeId, span: Span },
}

/// {name} — shorthand attribute.
#[derive(Clone)]
pub struct Shorthand {
    pub id: NodeId,
    pub expression_span: Span,
}

/// {...expr} — spread attribute.
#[derive(Clone)]
pub struct SpreadAttribute {
    pub id: NodeId,
    pub expression_span: Span,
}

#[derive(Clone)]
pub struct ClassDirective {
    pub id: NodeId,
    pub name: String,
    /// Span of the JS expression. None means shorthand (class:name).
    pub expression_span: Option<Span>,
    pub shorthand: bool,
}

#[derive(Clone)]
pub struct StyleDirective {
    pub id: NodeId,
    pub name: String,
    pub value: StyleDirectiveValue,
    pub important: bool,
}

#[derive(Clone)]
pub enum StyleDirectiveValue {
    /// style:name — shorthand, no explicit value
    Shorthand,
    /// style:name={expr} — expression in braces
    Expression(Span),
    /// style:name="string" — static string value
    String(String),
    /// style:name="text{expr}text" — concatenation of static and dynamic parts
    Concatenation(Vec<ConcatPart>),
}

#[derive(Clone)]
pub struct BindDirective {
    pub id: NodeId,
    pub name: String,
    /// Span of the JS expression. None means shorthand (bind:name).
    pub expression_span: Option<Span>,
    pub shorthand: bool,
}

/// use:name or use:name={expr}
#[derive(Clone)]
pub struct UseDirective {
    pub id: NodeId,
    /// Directive name span (e.g., "tooltip" in `use:tooltip`, "a.b" in `use:a.b`).
    pub name: Span,
    /// Span of the argument expression. None if no expression (`use:name`).
    pub expression_span: Option<Span>,
}

/// LEGACY(svelte4): on:directive syntax. Deprecated in Svelte 5, remove in Svelte 6.
#[derive(Clone)]
pub struct OnDirectiveLegacy {
    pub id: NodeId,
    /// Event name (e.g., "click" in `on:click`).
    pub name: String,
    /// Span of the event name in source (e.g., "click" span in `on:click`).
    pub name_span: Span,
    /// Span of the JS expression. None if no expression (bubble event).
    pub expression_span: Option<Span>,
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
    /// Transition function name span (e.g., "fade", "fly", "custom.fn").
    pub name: Span,
    /// Span of the argument expression. None if no expression.
    pub expression_span: Option<Span>,
    /// Modifiers like "local", "global".
    pub modifiers: Vec<String>,
    /// Whether this is `transition:`, `in:`, or `out:`.
    pub direction: TransitionDirection,
}

/// animate:name or animate:name={expr}
#[derive(Clone)]
pub struct AnimateDirective {
    pub id: NodeId,
    /// Directive name span (e.g., "flip" in `animate:flip`, "custom.fn" in `animate:custom.fn`).
    pub name: Span,
    /// Span of the argument expression. None if no expression (`animate:name`).
    pub expression_span: Option<Span>,
}

/// {@attach expr} — element attachment (Svelte 5.29+).
/// Modern alternative to `use:action`. Re-runs on reactive dependency changes.
#[derive(Clone)]
pub struct AttachTag {
    pub id: NodeId,
    /// Span of the JS expression inside `{@attach expr}`.
    pub expression_span: Span,
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

/// Flat arena storing all template nodes. NodeId is an index into this store.
/// Reserved slots (for attributes, key_id, script) hold placeholder Error nodes.
pub struct AstStore {
    nodes: Vec<Node>,
}

impl AstStore {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    /// Create a store pre-allocated for the expected number of nodes.
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            nodes: Vec::with_capacity(cap),
        }
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
