mod attributes;
mod id;

pub use attributes::{
    Attribute, BindDirective, BooleanAttribute, ClassDirective, ConcatenationAttribute,
    ConcatenationAttributePart, ExpressionAttribute, StringAttribute,
};
pub use id::{AttributeId, ExpressionId, NodeId, OwnerId};

pub struct Template {
    pub node_ids: Vec<NodeId>,
}

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

pub enum OwnerNode<'hir> {
    Element(&'hir Element<'hir>),
    Template(&'hir Template),
    IfBlock(&'hir IfBlock),
    EachBlock,
    Phantom,
}

pub struct Text<'hir> {
    pub owner_id: OwnerId,
    pub node_id: NodeId,
    pub value: &'hir str,
}

pub struct Interpolation {
    pub owner_id: OwnerId,
    pub node_id: NodeId,
    pub expression_id: ExpressionId,
}

pub struct Concatenation<'hir> {
    pub owner_id: OwnerId,
    pub node_id: NodeId,
    pub parts: Vec<ConcatenationPart<'hir>>,
}

pub enum ConcatenationPart<'hir> {
    Text(&'hir str),
    Expression(ExpressionId),
}

pub struct Element<'hir> {
    pub node_id: NodeId,
    pub owner_id: OwnerId,
    pub name: &'hir str,
    pub attributes: Vec<AttributeId>,
    pub node_ids: Vec<NodeId>,
}

pub struct IfBlock {
    pub node_id: NodeId,
    pub owner_id: OwnerId,
    pub is_elseif: bool,
    pub test: ExpressionId,
    pub consequent: Vec<NodeId>,
    pub alternate: Option<Vec<NodeId>>,
}
