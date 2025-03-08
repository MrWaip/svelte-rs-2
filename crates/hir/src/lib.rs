mod attributes;
mod id;
mod node;
mod owner;
mod store;

use std::cell::RefCell;

pub use attributes::{
    Attribute, BindDirective, BooleanAttribute, ClassDirective, ConcatenationAttribute,
    ConcatenationAttributePart, ExpressionAttribute, StringAttribute,
};
pub use id::{AttributeId, ExpressionId, NodeId, OwnerId};
pub use node::*;
pub use owner::*;
use oxc_ast::ast::Language;
pub use store::HirStore;

#[derive(Debug)]
pub struct Template {
    pub node_ids: Vec<NodeId>,
    pub node_id: NodeId,
}

#[derive(Debug)]
pub struct Text<'hir> {
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
pub struct Element<'hir> {
    pub node_id: NodeId,
    pub owner_id: OwnerId,
    pub name: &'hir str,
    pub attributes: Vec<AttributeId>,
    pub node_ids: Vec<NodeId>,
    pub self_closing: bool,
}

#[derive(Debug)]
pub struct IfBlock {
    pub node_id: NodeId,
    pub owner_id: OwnerId,
    pub is_elseif: bool,
    pub test: ExpressionId,
    pub consequent: Vec<NodeId>,
    pub alternate: Option<Vec<NodeId>>,
}

#[derive(Debug)]
pub struct Program<'hir> {
    pub language: Language,
    pub program: RefCell<oxc_ast::ast::Program<'hir>>,
}
