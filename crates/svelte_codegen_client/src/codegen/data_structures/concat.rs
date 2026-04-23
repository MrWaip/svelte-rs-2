use svelte_ast::{NodeId, Span};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ConcatPart {
    Static(Span),
    StaticOwned(String),
    Expr(NodeId),
}
