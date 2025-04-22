mod attributes;
mod attributes_store;
mod element;
mod id;
mod node;
mod owner;
mod store;

use std::cell::{Cell, RefCell};

pub use attributes::*;
pub use attributes_store::*;
pub use element::*;
pub use id::*;
pub use node::*;
pub use owner::*;

use oxc_ast::ast::Language;
use oxc_syntax::scope::ScopeId;
pub use store::HirStore;

#[derive(Debug)]
pub struct Template {
    pub node_ids: Vec<NodeId>,
    pub node_id: NodeId,
    pub scope_id: Cell<Option<ScopeId>>,
}

#[derive(Debug)]
pub struct Text<'hir> {
    pub owner_id: OwnerId,
    pub node_id: NodeId,
    pub value: &'hir str,
}

#[derive(Debug)]
pub struct Comment<'hir> {
    pub owner_id: OwnerId,
    pub node_id: NodeId,
    pub value: &'hir str,
}

#[derive(Debug)]
pub struct Interpolation {
    pub owner_id: OwnerId,
    pub node_id: NodeId,
    pub expression_id: ExpressionId,
}

#[derive(Debug)]
pub struct Concatenation<'hir> {
    pub owner_id: OwnerId,
    pub node_id: NodeId,
    pub parts: Vec<ConcatenationPart<'hir>>,
}

#[derive(Debug)]
pub enum ConcatenationPart<'hir> {
    Text(&'hir str),
    Expression(ExpressionId),
}

#[derive(Debug)]
pub struct IfBlock {
    pub node_id: NodeId,
    pub owner_id: OwnerId,
    pub is_elseif: bool,
    pub test: ExpressionId,
    pub consequent: Vec<NodeId>,
    pub alternate: Option<Vec<NodeId>>,
    pub scope_id: Cell<Option<ScopeId>>,
}

#[derive(Debug)]
pub struct EachBlock {
    pub node_id: NodeId,
    pub owner_id: OwnerId,

    pub node_ids: Vec<NodeId>,
    pub collection: ExpressionId,
    pub item: ExpressionId,
    pub index: Option<ExpressionId>,
    pub key: Option<ExpressionId>,
    pub scope_id: Cell<Option<ScopeId>>,
}

#[derive(Debug)]
pub struct Program<'hir> {
    pub language: Language,
    pub program: RefCell<oxc_ast::ast::Program<'hir>>,
}
