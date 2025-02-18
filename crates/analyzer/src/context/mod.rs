use ancestry::VisitorAncestry;
use oxc_index::Idx;
use oxc_semantic::NodeId;

use crate::ancestor::Ancestor;

pub mod ancestry;

pub struct VisitorContext<'a> {
    ancestry: VisitorAncestry<'a>,
    current_node_id: usize,
}

impl<'a> VisitorContext<'a> {
    pub fn new() -> Self {
        return Self {
            ancestry: VisitorAncestry::new(),
            current_node_id: 0,
        };
    }

    pub fn parent(&self) -> &Ancestor<'a> {
        return self.ancestry.parent();
    }

    pub(crate) fn push_stack(&mut self, ancestor: Ancestor<'a>) {
        self.ancestry.push_stack(ancestor);
    }

    pub(crate) fn pop_stack(&mut self) {
        self.ancestry.pop_stack();
    }

    pub fn next_node_id(&mut self) -> NodeId {
        let res = NodeId::from_usize(self.current_node_id);
        
        self.current_node_id += 1;

        return res;
    }

    pub fn node_id(&self) -> NodeId {
        return NodeId::from_usize(self.current_node_id);
    }
}
