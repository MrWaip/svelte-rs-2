use smallvec::SmallVec;
use svelte_span::{GetSpan, Span};

/// Inline capacity for selector lists (most rules have 1-2 selectors).
pub type SelectorVec<T> = SmallVec<[T; 2]>;

/// Inline capacity for simple selectors within a relative selector (usually 1-3).
pub type SimpleSelectorVec = SmallVec<[SimpleSelector; 4]>;

/// Inline capacity for relative selectors within a complex selector (usually 1-2).
pub type RelativeSelectorVec = SmallVec<[RelativeSelector; 2]>;

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
    pub span: Span,
    pub prelude: SelectorList,
    pub block: Block,
    pub metadata: RuleMetadata,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RuleMetadata {
    pub parent_rule: Option<usize>,
    pub has_local_selectors: bool,
    pub has_global_selectors: bool,
    pub is_global_block: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AtRule {
    pub span: Span,
    pub name: Span,
    pub prelude: Span,
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
    pub span: Span,
    pub children: RelativeSelectorVec,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RelativeSelector {
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
    Type(Span),
    /// `#id`
    Id(Span),
    /// `.class`
    Class(Span),
    /// `:pseudo-class` or `:pseudo-class(...)`
    PseudoClass(PseudoClassSelector),
    /// `::pseudo-element`
    PseudoElement(PseudoElementSelector),
    /// `[attr]`, `[attr=value]`, etc.
    Attribute(AttributeSelector),
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
    pub name: Span,
    /// Boxed to break the recursive type cycle (SelectorList → SimpleSelector → PseudoClassSelector).
    pub args: Option<Box<SelectorList>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PseudoElementSelector {
    pub span: Span,
    pub name: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AttributeSelector {
    pub span: Span,
    pub name: Span,
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
}

#[derive(Debug, Clone, PartialEq)]
pub struct Declaration {
    pub span: Span,
    pub property: Span,
    pub value: Span,
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
            Self::Type(s)
            | Self::Id(s)
            | Self::Class(s)
            | Self::Nesting(s)
            | Self::Nth(s)
            | Self::Percentage(s) => *s,
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
        }
    }
}

impl GetSpan for Declaration {
    fn span(&self) -> Span {
        self.span
    }
}
