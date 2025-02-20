use std::collections::HashMap;

use ancestry::VisitorAncestry;
use element_flags::ElementFlags;
use oxc_index::Idx;
use oxc_semantic::NodeId;

use crate::{ancestor::Ancestor, compute_optimization::OptimizationResult};

pub mod ancestry;
pub mod element_flags;

pub struct VisitorContext<'a> {
    ancestry: VisitorAncestry<'a>,
    current_node_id: usize,
    elements_metadata: HashMap<NodeId, ElementFlags>,
}

impl<'a> VisitorContext<'a> {
    pub fn new() -> Self {
        return Self {
            ancestry: VisitorAncestry::new(),
            current_node_id: 0,
            elements_metadata: HashMap::default(),
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

    pub(crate) fn add_default_element_flags(&mut self, node_id: NodeId) {
        self.elements_metadata
            .insert(node_id, ElementFlags::default());
    }

    pub fn parent_element_flags(&mut self) -> Option<&mut ElementFlags> {
        let node_id = self.parent().get_node_id();

        return self.elements_metadata.get_mut(&node_id);
    }

    pub fn resolve_element_flags(&mut self, node_id: NodeId) -> ElementFlags {
        return self.elements_metadata.remove(&node_id).unwrap();
    }

    pub fn mark_parent_element_as_dynamic(&mut self) {
        if let Some(flags) = self.parent_element_flags() {
            flags.dynamic = true;
        }
    }
}
