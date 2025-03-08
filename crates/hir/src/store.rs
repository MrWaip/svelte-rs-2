use std::{
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
};

use oxc_ast::ast::Expression;
use oxc_index::IndexVec;

use crate::{
    Attribute, AttributeId, ExpressionId, Node, NodeId, OwnerId, OwnerNode, Program, Template,
};

#[derive(Debug)]
pub struct HirStore<'hir> {
    pub owners: IndexVec<OwnerId, OwnerNode<'hir>>,
    pub expressions: IndexVec<ExpressionId, RefCell<Expression<'hir>>>,
    pub attributes: IndexVec<AttributeId, Attribute<'hir>>,
    pub node_to_owner: HashMap<NodeId, OwnerId>,
    pub nodes: IndexVec<NodeId, Node<'hir>>,
    pub program: Program<'hir>,
}

impl<'hir> HirStore<'hir> {
    pub const TEMPLATE_OWNER_ID: OwnerId = OwnerId::new(0);
    pub const TEMPLATE_NODE_ID: NodeId = NodeId::new(0);

    pub fn new(program: Program<'hir>) -> Self {
        HirStore {
            owners: IndexVec::new(),
            expressions: IndexVec::new(),
            attributes: IndexVec::new(),
            node_to_owner: HashMap::new(),
            nodes: IndexVec::new(),
            program,
        }
    }

    pub fn get_owner(&self, owner_id: OwnerId) -> &OwnerNode<'hir> {
        self.owners.get(owner_id).unwrap()
    }

    pub fn get_nth_owner(&self, idx: usize) -> &OwnerNode<'hir> {
        self.owners.get(OwnerId::new(idx)).unwrap()
    }

    pub fn get_expression(&self, expression_id: ExpressionId) -> Ref<Expression<'hir>> {
        return self.expressions.get(expression_id).unwrap().borrow();
    }

    pub fn get_expression_mut(&self, expression_id: ExpressionId) -> RefMut<Expression<'hir>> {
        self.expressions.get(expression_id).unwrap().borrow_mut()
    }

    pub fn get_attribute(&self, attribute_id: AttributeId) -> &Attribute<'hir> {
        self.attributes.get(attribute_id).unwrap()
    }

    pub fn get_node(&self, node_id: NodeId) -> &Node<'hir> {
        self.nodes.get(node_id).unwrap()
    }

    pub fn get_template(&self) -> &Template {
        self.get_owner(Self::TEMPLATE_OWNER_ID)
            .as_template()
            .unwrap()
    }

    pub fn first_of(&self, owner_id: OwnerId) -> Option<&Node<'hir>> {
        let owner = self.owners.get(owner_id);

        if let Some(owner) = owner {
            let option = owner.first();

            if option.is_none() {
                return None;
            }

            return Some(self.get_node(*option.unwrap()));
        } else {
            return None;
        }
    }

    pub fn node_to_owner(&self, node_id: &NodeId) -> OwnerId {
        *self.node_to_owner.get(node_id).unwrap()
    }

    pub fn is_first_of(&self, owner_id: OwnerId, f: impl FnOnce(&Node<'hir>) -> bool) -> bool {
        let owner = self.owners.get(owner_id);

        if let Some(owner) = owner {
            let option = owner.first();

            if option.is_none() {
                return false;
            }

            return f(self.get_node(*option.unwrap()));
        } else {
            return false;
        }
    }
}
