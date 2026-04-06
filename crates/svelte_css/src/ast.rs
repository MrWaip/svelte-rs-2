use compact_str::CompactString;
use smallvec::SmallVec;
use svelte_span::{GetSpan, Span};

/// Inline capacity for selector lists (most rules have 1-2 selectors).
pub type SelectorVec<T> = SmallVec<[T; 2]>;

/// Inline capacity for simple selectors within a relative selector (usually 1-3).
pub type SimpleSelectorVec = SmallVec<[SimpleSelector; 4]>;

/// Inline capacity for relative selectors within a complex selector (usually 1-2).
pub type RelativeSelectorVec = SmallVec<[RelativeSelector; 2]>;

// ---------------------------------------------------------------------------
// CssNodeId — stable identity for side-table keying
// ---------------------------------------------------------------------------

/// Unique identifier for CSS AST nodes that receive per-node metadata
/// in the analyze phase. Assigned sequentially by the parser.
///
/// Used to key into side tables (e.g. `IndexVec<CssNodeId, RuleMeta>`)
/// in `svelte_analyze`, keeping derived data out of the immutable AST.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CssNodeId(pub u32);

impl CssNodeId {
    pub const DUMMY: Self = Self(u32::MAX);
}

// ---------------------------------------------------------------------------
// Top-level
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub struct StyleSheet {
    pub span: Span,
    pub children: Vec<StyleSheetChild>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StyleSheetChild {
    Rule(Rule),
    Comment(Comment),
    /// Skipped invalid CSS (parse error recovery).
    Error(Span),
}

// ---------------------------------------------------------------------------
// Comments
// ---------------------------------------------------------------------------

/// A CSS comment including its `/* ... */` delimiters.
#[derive(Debug, Clone, PartialEq)]
pub struct Comment {
    pub span: Span,
}

// ---------------------------------------------------------------------------
// Rules
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub enum Rule {
    Style(Box<StyleRule>),
    AtRule(AtRule),
}

#[derive(Debug, Clone, PartialEq)]
pub struct StyleRule {
    pub id: CssNodeId,
    pub span: Span,
    pub prelude: SelectorList,
    pub block: Block,
}

impl StyleRule {
    /// Returns true if this rule's prelude is a lone `:global` (block form).
    ///
    /// Matches exactly: one `ComplexSelector` → one `RelativeSelector` → one
    /// `SimpleSelector::Global { args: None }`.  This is distinct from the
    /// compound form (`:global .foo { … }`) which has additional selectors.
    pub fn is_lone_global_block(&self) -> bool {
        self.prelude.children.len() == 1
            && self.prelude.children[0].children.len() == 1
            && self.prelude.children[0].children[0].selectors.len() == 1
            && matches!(
                &self.prelude.children[0].children[0].selectors[0],
                SimpleSelector::Global { args: None, .. }
            )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AtRule {
    pub span: Span,
    pub name: CompactString,
    pub prelude: Span,
    /// When set by the transform, the printer uses this instead of `prelude`.
    pub prelude_override: Option<CompactString>,
    pub block: Option<Block>,
}

// ---------------------------------------------------------------------------
// Selectors
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub struct SelectorList {
    pub span: Span,
    pub children: SelectorVec<ComplexSelector>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ComplexSelector {
    pub id: CssNodeId,
    pub span: Span,
    pub children: RelativeSelectorVec,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RelativeSelector {
    pub id: CssNodeId,
    pub span: Span,
    pub combinator: Option<Combinator>,
    pub selectors: SimpleSelectorVec,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Combinator {
    pub span: Span,
    pub kind: CombinatorKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CombinatorKind {
    /// Whitespace (descendant)
    Descendant,
    /// `>`
    Child,
    /// `+`
    NextSibling,
    /// `~`
    SubsequentSibling,
    /// `||`
    Column,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SimpleSelector {
    /// Element / type selector (e.g. `div`, `*`, `ns|div`)
    Type { span: Span, name: CompactString },
    /// `#id`
    Id { span: Span, name: CompactString },
    /// `.class`
    Class { span: Span, name: CompactString },
    /// `:pseudo-class` or `:pseudo-class(...)`
    PseudoClass(PseudoClassSelector),
    /// `::pseudo-element`
    PseudoElement(PseudoElementSelector),
    /// `[attr]`, `[attr=value]`, etc.
    Attribute(AttributeSelector),
    /// `:global(...)` or `:global` — Svelte-specific scoping escape.
    /// Separated from `PseudoClass` because it controls scoping semantics,
    /// not CSS matching. Exhaustive match forces every consumer to handle it.
    Global {
        span: Span,
        args: Option<Box<SelectorList>>,
    },
    /// `&` nesting selector
    Nesting(Span),
    /// An+B notation inside `:nth-child()` etc.
    Nth(Span),
    /// Percentage (e.g. in `@keyframes`)
    Percentage(Span),
}

#[derive(Debug, Clone, PartialEq)]
pub struct PseudoClassSelector {
    pub span: Span,
    pub name: CompactString,
    /// Boxed to break the recursive type cycle (SelectorList → SimpleSelector → PseudoClassSelector).
    pub args: Option<Box<SelectorList>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PseudoElementSelector {
    pub span: Span,
    pub name: CompactString,
    /// Arguments for functional pseudo-elements like `::part(foo)`, `::slotted(.foo)`.
    pub args: Option<Box<SelectorList>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AttributeSelector {
    pub span: Span,
    pub name: CompactString,
    /// The matcher operator span (e.g. `~=`, `^=`, `=`)
    pub matcher: Option<Span>,
    /// The attribute value (quoted or unquoted content, without quotes)
    pub value: Option<Span>,
    /// Case-sensitivity flags (e.g. `i`, `s`)
    pub flags: Option<Span>,
}

// ---------------------------------------------------------------------------
// Blocks & Declarations
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub span: Span,
    pub children: Vec<BlockChild>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BlockChild {
    Declaration(Declaration),
    Rule(Rule),
    Comment(Comment),
    /// Skipped invalid CSS (parse error recovery).
    Error(Span),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Declaration {
    pub span: Span,
    pub property: Span,
    pub value: Span,
    /// When set by the transform, the printer uses this instead of `value`.
    pub value_override: Option<String>,
}

// ---------------------------------------------------------------------------
// GetSpan implementations
// ---------------------------------------------------------------------------

impl GetSpan for StyleSheet {
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for StyleSheetChild {
    fn span(&self) -> Span {
        match self {
            Self::Rule(r) => r.span(),
            Self::Comment(c) => c.span,
            Self::Error(s) => *s,
        }
    }
}

impl GetSpan for Comment {
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for Rule {
    fn span(&self) -> Span {
        match self {
            Self::Style(r) => r.as_ref().span,
            Self::AtRule(r) => r.span,
        }
    }
}

impl GetSpan for StyleRule {
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for AtRule {
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for SelectorList {
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for ComplexSelector {
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for RelativeSelector {
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for Combinator {
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for SimpleSelector {
    fn span(&self) -> Span {
        match self {
            Self::Type { span, .. }
            | Self::Id { span, .. }
            | Self::Class { span, .. }
            | Self::Global { span, .. }
            | Self::Nesting(span)
            | Self::Nth(span)
            | Self::Percentage(span) => *span,
            Self::PseudoClass(p) => p.span,
            Self::PseudoElement(p) => p.span,
            Self::Attribute(a) => a.span,
        }
    }
}

impl GetSpan for PseudoClassSelector {
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for PseudoElementSelector {
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for AttributeSelector {
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for Block {
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for BlockChild {
    fn span(&self) -> Span {
        match self {
            Self::Declaration(d) => d.span,
            Self::Rule(r) => r.span(),
            Self::Comment(c) => c.span,
            Self::Error(s) => *s,
        }
    }
}

impl GetSpan for Declaration {
    fn span(&self) -> Span {
        self.span
    }
}
