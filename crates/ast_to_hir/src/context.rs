use std::{cell::RefCell, mem};

use hir::{ExpressionId, HirStore, Node, NodeId, OwnerId, OwnerNode};
use oxc_allocator::Allocator;
use oxc_ast::ast::Expression;

pub struct ToHirContext<'hir> {
    pub(crate) allocator: &'hir Allocator,
    pub(crate) store: HirStore<'hir>,

    current_owner_id: OwnerId,
}

impl<'hir> ToHirContext<'hir> {
    pub fn new(allocator: &'hir Allocator, program: hir::Program<'hir>) -> Self {
        Self {
            store: HirStore::new(program),
            allocator,
            current_owner_id: HirStore::TEMPLATE_OWNER_ID,
        }
    }

    pub fn push_node(
        &mut self,
        f: impl FnOnce(&mut ToHirContext<'hir>, NodeId, OwnerId) -> hir::Node<'hir>,
    ) -> NodeId {
        let node_id = self.store.nodes.push(Node::Phantom);
        self.store.nodes[node_id] = f(self, node_id, self.current_owner_id);

        node_id
    }

    pub fn push_owner_node(
        &mut self,
        f: impl FnOnce(
            &mut ToHirContext<'hir>,
            NodeId,
            OwnerId,
        ) -> (hir::Node<'hir>, hir::OwnerNode<'hir>),
    ) -> NodeId {
        let owner_id = self.store.owners.push(OwnerNode::Phantom);
        let node_id = self.store.nodes.push(Node::Phantom);
        let prev_owner_id = mem::replace(&mut self.current_owner_id, owner_id);

        self.store.node_to_owner.insert(node_id, owner_id);

        let (node, owner) = f(self, node_id, prev_owner_id);
        self.store.nodes[node_id] = node;
        self.store.owners[owner_id] = owner;
        self.current_owner_id = prev_owner_id;

        node_id
    }

    pub fn push_expression(&mut self, expression: Expression<'hir>) -> ExpressionId {
        
        self.store.expressions.push(RefCell::new(expression))
    }

    pub fn alloc<T>(&self, value: T) -> &'hir mut T {
        self.allocator.alloc(value)
    }
}
