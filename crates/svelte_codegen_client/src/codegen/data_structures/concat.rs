use svelte_ast::{NodeId, Span};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ConcatPart {
    Static(Span),
    StaticOwned(String),
    StaticEntities { html: String, text: String },
    Expr(NodeId),
}
