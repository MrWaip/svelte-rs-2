use std::mem;

use hir::{Attribute, AttributeId, ExpressionId, Node, NodeId, OwnerId, OwnerNode};
use oxc_allocator::Allocator;
use oxc_ast::ast::Expression;
use oxc_index::IndexVec;

pub struct ToHirContext<'hir> {
    pub(crate) allocator: &'hir Allocator,
    pub(crate) owners: IndexVec<OwnerId, OwnerNode<'hir>>,
    pub(crate) expressions: IndexVec<ExpressionId, Expression<'hir>>,
    pub(crate) attributes: IndexVec<AttributeId, Attribute<'hir>>,
    pub(crate) nodes: IndexVec<NodeId, Node<'hir>>,
    current_owner_id: OwnerId,
}

impl<'hir> ToHirContext<'hir> {
    pub fn new(allocator: &'hir Allocator) -> Self {
        Self {
            owners: IndexVec::new(),
            expressions: IndexVec::new(),
            attributes: IndexVec::new(),
            nodes: IndexVec::new(),
            allocator,
            current_owner_id: OwnerId::new(0),
        }
    }

    pub fn push_node(
        &mut self,
        f: impl FnOnce(&mut ToHirContext<'hir>, NodeId, OwnerId) -> hir::Node<'hir>,
    ) -> NodeId {
        let node_id = self.nodes.push(Node::Phantom);
        self.nodes[node_id] = f(self, node_id, self.current_owner_id);

        return node_id;
    }

    pub fn push_owner_node(
        &mut self,
        f: impl FnOnce(
            &mut ToHirContext<'hir>,
            NodeId,
            OwnerId,
        ) -> (hir::Node<'hir>, hir::OwnerNode<'hir>),
    ) -> NodeId {
        let owner_id = self.owners.push(OwnerNode::Phantom);
        let node_id = self.nodes.push(Node::Phantom);
        let prev_owner_id = mem::replace(&mut self.current_owner_id, owner_id);

        let (node, owner) = f(self, node_id, prev_owner_id);
        self.nodes[node_id] = node;
        self.owners[owner_id] = owner;
        self.current_owner_id = prev_owner_id;

        return node_id;
    }

    pub fn push_expression(&mut self, expression: Expression<'hir>) -> ExpressionId {
        let expression_id = self.expressions.push(expression);
        return expression_id;
    }

    pub fn push_attribute(&mut self, attribute: hir::Attribute<'hir>) -> AttributeId {
        let attribute_id = self.attributes.push(attribute);
        return attribute_id;
    }

    pub fn push_root_owner(
        &mut self,
        f: impl FnOnce(&mut ToHirContext<'hir>) -> hir::OwnerNode<'hir>,
    ) -> OwnerId {
        let owner_id = self.owners.push(OwnerNode::Phantom);
        let prev_owner_id = mem::replace(&mut self.current_owner_id, owner_id);
        self.owners[owner_id] = f(self);
        self.current_owner_id = prev_owner_id;
        return owner_id;
    }

    pub fn alloc<T>(&self, value: T) -> &'hir mut T {
        return self.allocator.alloc(value);
    }
}
