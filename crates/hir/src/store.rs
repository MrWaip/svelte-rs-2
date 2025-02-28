use oxc_ast::ast::Expression;
use oxc_index::IndexVec;

use crate::{Attribute, AttributeId, ExpressionId, Node, NodeId, OwnerId, OwnerNode, Program, Template};

#[derive(Debug)]
pub struct HirStore<'hir> {
    pub owners: IndexVec<OwnerId, OwnerNode<'hir>>,
    pub expressions: IndexVec<ExpressionId, Expression<'hir>>,
    pub attributes: IndexVec<AttributeId, Attribute<'hir>>,
    pub nodes: IndexVec<NodeId, Node<'hir>>,
    pub program: Program<'hir>,
}

impl<'hir> HirStore<'hir> {
    pub const TEMPLATE_OWNER_ID: OwnerId = OwnerId::new(0);

    pub fn new(program: Program<'hir>) -> Self {
        HirStore {
            owners: IndexVec::new(),
            expressions: IndexVec::new(),
            attributes: IndexVec::new(),
            nodes: IndexVec::new(),
            program,
        }
    }

    pub fn get_owner(&self, owner_id: OwnerId) -> &OwnerNode<'hir> {
        self.owners.get(owner_id).unwrap()
    }

    pub fn get_expression(&self, expression_id: ExpressionId) -> &Expression<'hir> {
        self.expressions.get(expression_id).unwrap()
    }

    pub fn get_attribute(&self, attribute_id: AttributeId) -> &Attribute<'hir> {
        self.attributes.get(attribute_id).unwrap()
    }

    pub fn get_node(&self, node_id: NodeId) -> &Node<'hir> {
        self.nodes.get(node_id).unwrap()
    }

    pub fn get_template(&self) -> &Template {
        self.get_owner(Self::TEMPLATE_OWNER_ID).as_template().unwrap()
    }
}
