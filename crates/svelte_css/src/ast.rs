use compact_str::CompactString;
use smallvec::SmallVec;
use svelte_span::{GetSpan, Span};

pub type SelectorVec<T> = SmallVec<[T; 2]>;

pub type SimpleSelectorVec = SmallVec<[SimpleSelector; 4]>;

pub type RelativeSelectorVec = SmallVec<[RelativeSelector; 2]>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CssNodeId(pub u32);

impl CssNodeId {
    pub const DUMMY: Self = Self(u32::MAX);
}

#[derive(Debug, Clone, PartialEq)]
pub struct StyleSheet {
    pub span: Span,
    pub children: Vec<StyleSheetChild>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StyleSheetChild {
    Rule(Rule),
    Comment(Comment),

    Error(Span),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Comment {
    pub span: Span,
}

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

    pub prelude_override: Option<CompactString>,
    pub block: Option<Block>,
}

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
    Descendant,

    Child,

    NextSibling,

    SubsequentSibling,

    Column,
}

impl CombinatorKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Descendant => " ",
            Self::Child => ">",
            Self::NextSibling => "+",
            Self::SubsequentSibling => "~",
            Self::Column => "||",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SimpleSelector {
    Type {
        span: Span,
        name: CompactString,
    },

    Id {
        span: Span,
        name: CompactString,
    },

    Class {
        span: Span,
        name: CompactString,
    },

    PseudoClass(PseudoClassSelector),

    PseudoElement(PseudoElementSelector),

    Attribute(AttributeSelector),

    Global {
        span: Span,
        args: Option<Box<SelectorList>>,
    },

    Nesting(Span),

    Nth(Span),

    Percentage(Span),
}

#[derive(Debug, Clone, PartialEq)]
pub struct PseudoClassSelector {
    pub span: Span,
    pub name: CompactString,

    pub args: Option<Box<SelectorList>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PseudoElementSelector {
    pub span: Span,
    pub name: CompactString,

    pub args: Option<Box<SelectorList>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AttributeSelector {
    pub span: Span,
    pub name: CompactString,

    pub matcher: Option<Span>,

    pub value: Option<Span>,

    pub flags: Option<Span>,
}

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

    Error(Span),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Declaration {
    pub span: Span,
    pub property: Span,
    pub value: Span,

    pub value_override: Option<String>,
}

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
