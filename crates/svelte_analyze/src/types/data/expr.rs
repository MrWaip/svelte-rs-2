use super::*;

#[derive(Debug, Clone)]
pub enum AwaitBindingInfo {
    Simple(String),
    Destructured {
        kind: DestructureKind,
        names: Vec<String>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DestructureKind {
    Object,
    Array,
}

impl AwaitBindingInfo {
    pub fn names(&self) -> &[String] {
        match self {
            Self::Simple(name) => std::slice::from_ref(name),
            Self::Destructured { names, .. } => names,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExpressionInfo {
    pub kind: ExpressionKind,
    pub ref_symbols: SmallVec<[SymbolId; 2]>,
    pub has_store_ref: bool,
    pub has_side_effects: bool,
    pub has_call: bool,
    pub has_await: bool,
    pub has_state_rune: bool,
    pub has_store_member_mutation: bool,
    pub needs_context: bool,
    pub is_dynamic: bool,
    pub has_state: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpressionKind {
    Identifier(CompactString),
    Literal,
    CallExpression { callee: CompactString },
    MemberExpression,
    ArrowFunction,
    Assignment,
    Other,
}

impl ExpressionKind {
    pub fn is_simple(&self) -> bool {
        matches!(self, Self::Identifier(_) | Self::MemberExpression)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExprSite {
    Node(NodeId),
    Attr(NodeId),
}

#[derive(Debug, Clone)]
pub struct ExprDeps<'a> {
    pub info: &'a ExpressionInfo,
    pub blockers: SmallVec<[u32; 2]>,
    pub needs_memo: bool,
}

impl ExprDeps<'_> {
    pub fn has_await(&self) -> bool {
        self.info.has_await
    }

    pub fn has_call(&self) -> bool {
        self.info.has_call
    }

    pub fn has_blockers(&self) -> bool {
        !self.blockers.is_empty()
    }
}
