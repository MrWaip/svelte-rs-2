mod attributes;
mod id;
mod node;
mod store;

use std::cell::RefCell;

pub use attributes::{
    Attribute, BindDirective, BooleanAttribute, ClassDirective, ConcatenationAttribute,
    ConcatenationAttributePart, ExpressionAttribute, StringAttribute,
};
pub use id::{AttributeId, ExpressionId, NodeId, OwnerId};
pub use node::*;
use oxc_ast::ast::Language;
pub use store::HirStore;

#[derive(Debug)]
pub struct Template {
    pub node_ids: Vec<NodeId>,
}

#[derive(Debug)]
pub enum OwnerNode<'hir> {
    Element(&'hir Element<'hir>),
    Template(&'hir Template),
    IfBlock(&'hir IfBlock),
    EachBlock,
    Phantom,
}

impl<'hir> OwnerNode<'hir> {
    pub fn is_if_block(&self) -> bool {
        return matches!(self, OwnerNode::IfBlock(_));
    }

    pub fn as_element(&self) -> Option<&Element> {
        match self {
            OwnerNode::Element(element) => Some(element),
            _ => None,
        }
    }

    pub fn as_template(&self) -> Option<&Template> {
        match self {
            OwnerNode::Template(template) => Some(template),
            _ => None,
        }
    }

    pub fn as_if_block(&self) -> Option<&IfBlock> {
        match self {
            OwnerNode::IfBlock(if_block) => Some(if_block),
            _ => None,
        }
    }

    pub fn first(&self) -> Option<&NodeId> {
        return match self {
            OwnerNode::Element(it) => it.node_ids.first(),
            OwnerNode::Template(it) => it.node_ids.first(),
            OwnerNode::IfBlock(it) => it.consequent.first(),
            OwnerNode::EachBlock => todo!(),
            OwnerNode::Phantom => todo!(),
        };
    }
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
