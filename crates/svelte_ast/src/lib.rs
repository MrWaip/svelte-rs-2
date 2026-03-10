use svelte_span::Span;

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

pub enum Node {
    Text(Text),
    Element(Element),
    Comment(Comment),
    ExpressionTag(ExpressionTag),
    IfBlock(IfBlock),
    EachBlock(EachBlock),
}

impl Node {
    pub fn node_id(&self) -> NodeId {
        match self {
            Node::Text(n) => n.id,
            Node::Element(n) => n.id,
            Node::Comment(n) => n.id,
            Node::ExpressionTag(n) => n.id,
            Node::IfBlock(n) => n.id,
            Node::EachBlock(n) => n.id,
        }
    }

    pub fn span(&self) -> Span {
        match self {
            Node::Text(n) => n.span,
            Node::Element(n) => n.span,
            Node::Comment(n) => n.span,
            Node::ExpressionTag(n) => n.span,
            Node::IfBlock(n) => n.span,
            Node::EachBlock(n) => n.span,
        }
    }

    pub fn is_text(&self) -> bool {
        matches!(self, Node::Text(_))
    }

    pub fn is_element(&self) -> bool {
        matches!(self, Node::Element(_))
    }

    pub fn is_if_block(&self) -> bool {
        matches!(self, Node::IfBlock(_))
    }

    pub fn is_expression_tag(&self) -> bool {
        matches!(self, Node::ExpressionTag(_))
    }

    pub fn is_comment(&self) -> bool {
        matches!(self, Node::Comment(_))
    }
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
        let attrs = self.attributes.iter().map(|a| match a {
            Attribute::StringAttribute(x) => Attribute::StringAttribute(StringAttribute {
                name: x.name.clone(),
                value_span: x.value_span,
            }),
            Attribute::BooleanAttribute(x) => {
                Attribute::BooleanAttribute(BooleanAttribute { name: x.name.clone() })
            }
            Attribute::ExpressionAttribute(x) => Attribute::ExpressionAttribute(ExpressionAttribute {
                name: x.name.clone(),
                expression_span: x.expression_span,
                shorthand: x.shorthand,
            }),
            Attribute::ConcatenationAttribute(x) => {
                Attribute::ConcatenationAttribute(ConcatenationAttribute {
                    name: x.name.clone(),
                    parts: x.parts.iter().map(|p| match p {
                        ConcatPart::Static(s) => ConcatPart::Static(s.clone()),
                        ConcatPart::Dynamic(sp) => ConcatPart::Dynamic(*sp),
                    }).collect(),
                })
            }
            Attribute::ShorthandOrSpread(x) => Attribute::ShorthandOrSpread(ShorthandOrSpread {
                expression_span: x.expression_span,
                is_spread: x.is_spread,
            }),
            Attribute::ClassDirective(x) => Attribute::ClassDirective(ClassDirective {
                name: x.name.clone(),
                expression_span: x.expression_span,
                shorthand: x.shorthand,
            }),
            Attribute::BindDirective(x) => Attribute::BindDirective(BindDirective {
                name: x.name.clone(),
                expression_span: x.expression_span,
                shorthand: x.shorthand,
            }),
        }).collect();

        Element {
            id: self.id,
            span: self.span,
            name: self.name.clone(),
            self_closing: self.self_closing,
            attributes: attrs,
            fragment: Fragment::empty(),
        }
    }
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
// Attributes
// ---------------------------------------------------------------------------

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
    /// bind:name or bind:name={expr}
    BindDirective(BindDirective),
}

pub struct StringAttribute {
    pub name: String,
    pub value_span: Span,
}

pub struct ExpressionAttribute {
    pub name: String,
    /// Span of just the JS expression.
    pub expression_span: Span,
    pub shorthand: bool,
}

pub struct BooleanAttribute {
    pub name: String,
}

pub struct ConcatenationAttribute {
    pub name: String,
    pub parts: Vec<ConcatPart>,
}

pub enum ConcatPart {
    /// Static text portion.
    Static(String),
    /// Dynamic expression portion — span of the JS expression.
    Dynamic(Span),
}

pub struct ShorthandOrSpread {
    /// Span of the expression (includes braces in the outer span).
    pub expression_span: Span,
    pub is_spread: bool,
}

pub struct ClassDirective {
    pub name: String,
    /// Span of the JS expression. None means shorthand (class:name).
    pub expression_span: Option<Span>,
    pub shorthand: bool,
}

pub struct BindDirective {
    pub name: String,
    /// Span of the JS expression. None means shorthand (bind:name).
    pub expression_span: Option<Span>,
    pub shorthand: bool,
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
