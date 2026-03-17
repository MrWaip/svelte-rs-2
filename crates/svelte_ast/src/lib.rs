use svelte_span::Span;

const VOID_ELEMENTS: &[&str] = &[
    "area", "base", "br", "col", "command", "embed", "hr", "img", "input", "keygen", "link",
    "meta", "param", "source", "track", "wbr",
];

pub fn is_void(name: &str) -> bool {
    VOID_ELEMENTS.contains(&name)
}

/// Unique node identifier, assigned during parsing.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct NodeId(pub u32);

// ---------------------------------------------------------------------------
// Root
// ---------------------------------------------------------------------------

pub struct Component {
    pub fragment: Fragment,
    pub script: Option<Script>,
    pub css: Option<RawBlock>,
    /// Full source text of the .svelte file.
    pub source: String,
    next_node_id: u32,
}

impl Component {
    pub fn new(source: String, fragment: Fragment, script: Option<Script>, css: Option<RawBlock>) -> Self {
        // next_node_id is set after parsing; during parsing we use NodeIdAllocator
        Self {
            fragment,
            script,
            css,
            source,
            next_node_id: 0,
        }
    }

    pub fn set_next_node_id(&mut self, id: u32) {
        self.next_node_id = id;
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
    pub nodes: Vec<Node>,
}

impl Fragment {
    pub fn new(nodes: Vec<Node>) -> Self {
        Self { nodes }
    }

    pub fn empty() -> Self {
        Self { nodes: vec![] }
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn push(&mut self, node: Node) {
        self.nodes.push(node);
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
    KeyBlock(KeyBlock)           => is_key_block / as_key_block,
    Error(ErrorNode)             => is_error / as_error,
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
}

impl Text {
    /// Get the text value from source.
    pub fn value<'a>(&self, source: &'a str) -> &'a str {
        &source[self.span.start as usize..self.span.end as usize]
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
    /// Span of the iteration variable (e.g., `item` or `{ value, flag }`).
    pub context_span: Span,
    /// Span of the index variable, if any.
    pub index_span: Option<Span>,
    /// Span of the key expression, if any.
    pub key_span: Option<Span>,
    pub body: Fragment,
    pub fallback: Option<Fragment>,
}

// ---------------------------------------------------------------------------
// SnippetBlock — {#snippet name(params)}...{/snippet}
// ---------------------------------------------------------------------------

pub struct SnippetBlock {
    pub id: NodeId,
    pub span: Span,
    pub name: String,
    /// Span of the parameter list inside parentheses.
    /// None if no params.
    pub params_span: Option<Span>,
    pub body: Fragment,
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
    /// Span of the declaration text: `doubled = item * 2` in `{@const doubled = item * 2}`.
    pub declaration_span: Span,
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
// Attributes
// ---------------------------------------------------------------------------

#[derive(Clone)]
pub enum Attribute {
    /// name="string"
    StringAttribute(StringAttribute),
    /// name={expr}
    ExpressionAttribute(ExpressionAttribute),
    /// name (boolean, no value)
    BooleanAttribute(BooleanAttribute),
    /// name="text{expr}text"
    ConcatenationAttribute(ConcatenationAttribute),
    /// {expr} or {...expr}
    ShorthandOrSpread(ShorthandOrSpread),
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
}

#[derive(Clone)]
pub struct StringAttribute {
    pub name: String,
    pub value_span: Span,
}

#[derive(Clone)]
pub struct ExpressionAttribute {
    pub name: String,
    /// Span of just the JS expression.
    pub expression_span: Span,
    pub shorthand: bool,
}

#[derive(Clone)]
pub struct BooleanAttribute {
    pub name: String,
}

#[derive(Clone)]
pub struct ConcatenationAttribute {
    pub name: String,
    pub parts: Vec<ConcatPart>,
}

#[derive(Clone)]
pub enum ConcatPart {
    /// Static text portion.
    Static(String),
    /// Dynamic expression portion — span of the JS expression.
    Dynamic(Span),
}

#[derive(Clone)]
pub struct ShorthandOrSpread {
    /// Span of the expression (includes braces in the outer span).
    pub expression_span: Span,
    pub is_spread: bool,
}

#[derive(Clone)]
pub struct ClassDirective {
    pub name: String,
    /// Span of the JS expression. None means shorthand (class:name).
    pub expression_span: Option<Span>,
    pub shorthand: bool,
}

#[derive(Clone)]
pub struct StyleDirective {
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
    pub name: String,
    /// Span of the JS expression. None means shorthand (bind:name).
    pub expression_span: Option<Span>,
    pub shorthand: bool,
}

/// use:name or use:name={expr}
#[derive(Clone)]
pub struct UseDirective {
    /// Directive name (e.g., "tooltip" in `use:tooltip`, "a.b" in `use:a.b`).
    pub name: String,
    /// Span of the argument expression. None if no expression (`use:name`).
    pub expression_span: Option<Span>,
}

/// LEGACY(svelte4): on:directive syntax. Deprecated in Svelte 5, remove in Svelte 6.
#[derive(Clone)]
pub struct OnDirectiveLegacy {
    /// Event name (e.g., "click" in `on:click`).
    pub name: String,
    /// Span of the JS expression. None if no expression (bubble event).
    pub expression_span: Option<Span>,
    /// Modifiers like "preventDefault", "stopPropagation", "capture", etc.
    pub modifiers: Vec<String>,
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
    /// Transition function name (e.g., "fade", "fly", "custom.fn").
    pub name: String,
    /// Span of the argument expression. None if no expression.
    pub expression_span: Option<Span>,
    /// Modifiers like "local", "global".
    pub modifiers: Vec<String>,
    /// Whether this is `transition:`, `in:`, or `out:`.
    pub direction: TransitionDirection,
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
// NodeId allocator (used during parsing)
// ---------------------------------------------------------------------------

pub struct NodeIdAllocator {
    next: u32,
}

impl NodeIdAllocator {
    pub fn new() -> Self {
        Self { next: 0 }
    }

    pub fn next(&mut self) -> NodeId {
        let id = NodeId(self.next);
        self.next += 1;
        id
    }

    pub fn current(&self) -> u32 {
        self.next
    }
}

impl Default for NodeIdAllocator {
    fn default() -> Self {
        Self::new()
    }
}
