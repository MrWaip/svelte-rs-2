mod attributes;
mod id;

pub use attributes::{
    Attribute, BindDirective, BooleanAttribute, ClassDirective, ConcatenationAttribute,
    ConcatenationAttributePart, ExpressionAttribute, StringAttribute,
};
pub use id::{AttributeId, ExpressionId, NodeId, OwnerId};

#[derive(Debug)]
pub struct Template {
    pub node_ids: Vec<NodeId>,
}

#[derive(Debug)]
pub enum Node<'hir> {
    Text(&'hir Text<'hir>),
    Interpolation(&'hir Interpolation),
    Element(&'hir Element<'hir>),
    Comment,
    IfBlock(&'hir IfBlock),
    EachBlock,
    Script,
    Concatenation(&'hir Concatenation<'hir>),
    Phantom,
}

impl<'hir> Node<'hir> {
    pub fn is_element(&self) -> bool {
        matches!(self, Node::Element(_))
    }

    pub fn as_element(&self) -> Option<&'hir Element<'hir>> {
        match self {
            Node::Element(element) => Some(element),
            _ => None,
        }
    }

    pub fn as_text(&self) -> Option<&'hir Text<'hir>> {
        match self {
            Node::Text(text) => Some(text),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum OwnerNode<'hir> {
    Element(&'hir Element<'hir>),
    Template(&'hir Template),
    IfBlock(&'hir IfBlock),
    EachBlock,
    Phantom,
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
