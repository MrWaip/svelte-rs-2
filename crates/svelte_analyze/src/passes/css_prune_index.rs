use svelte_ast::NodeId;

use crate::types::node_table::NodeTable;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum NodeExistsValue {
    Probably,
    Definitely,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum CandidateKind {
    Element,
    SvelteElement,
    RenderTag,
    ComponentNode,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct CandidateNode {
    pub(crate) id: NodeId,
    pub(crate) kind: CandidateKind,
}

#[derive(Clone, Copy)]
pub(crate) struct SiblingCandidate {
    pub(crate) node: CandidateNode,
    pub(crate) existence: NodeExistsValue,
}

pub(crate) struct CssPruneIndex {
    render_possible_snippets: NodeTable<Vec<NodeId>>,
    component_possible_snippets: NodeTable<Vec<NodeId>>,
    snippet_sites: NodeTable<Vec<NodeId>>,
    ancestor_adjacent_cache: NodeTable<Vec<NodeId>>,
    ancestor_all_cache: NodeTable<Vec<NodeId>>,
    descendant_adjacent_cache: NodeTable<Vec<NodeId>>,
    descendant_all_cache: NodeTable<Vec<NodeId>>,
    sibling_forward_adjacent_cache: NodeTable<Vec<SiblingCandidate>>,
    sibling_forward_all_cache: NodeTable<Vec<SiblingCandidate>>,
    sibling_backward_adjacent_cache: NodeTable<Vec<SiblingCandidate>>,
    sibling_backward_all_cache: NodeTable<Vec<SiblingCandidate>>,
}

impl CssPruneIndex {
    pub(crate) fn new(node_count: u32) -> Self {
        Self {
            render_possible_snippets: NodeTable::new(node_count),
            component_possible_snippets: NodeTable::new(node_count),
            snippet_sites: NodeTable::new(node_count),
            ancestor_adjacent_cache: NodeTable::new(node_count),
            ancestor_all_cache: NodeTable::new(node_count),
            descendant_adjacent_cache: NodeTable::new(node_count),
            descendant_all_cache: NodeTable::new(node_count),
            sibling_forward_adjacent_cache: NodeTable::new(node_count),
            sibling_forward_all_cache: NodeTable::new(node_count),
            sibling_backward_adjacent_cache: NodeTable::new(node_count),
            sibling_backward_all_cache: NodeTable::new(node_count),
        }
    }

    pub(crate) fn render_possible_snippets(&self, id: NodeId) -> &[NodeId] {
        self.render_possible_snippets
            .get(id)
            .map_or(&[], |v| v.as_slice())
    }

    pub(crate) fn insert_render_possible_snippets(&mut self, id: NodeId, snippets: Vec<NodeId>) {
        self.render_possible_snippets.insert(id, snippets);
    }

    pub(crate) fn component_possible_snippets(&self, id: NodeId) -> &[NodeId] {
        self.component_possible_snippets
            .get(id)
            .map_or(&[], |v| v.as_slice())
    }

    pub(crate) fn insert_component_possible_snippets(&mut self, id: NodeId, snippets: Vec<NodeId>) {
        self.component_possible_snippets.insert(id, snippets);
    }

    pub(crate) fn snippet_sites(&self, id: NodeId) -> &[NodeId] {
        self.snippet_sites.get(id).map_or(&[], |v| v.as_slice())
    }

    pub(crate) fn snippet_sites_mut(&mut self, id: NodeId) -> &mut Vec<NodeId> {
        self.snippet_sites.get_or_default(id)
    }

    pub(crate) fn cached_ancestors(&self, id: NodeId, adjacent_only: bool) -> Option<&[NodeId]> {
        let cache = if adjacent_only {
            &self.ancestor_adjacent_cache
        } else {
            &self.ancestor_all_cache
        };
        cache.get(id).map(Vec::as_slice)
    }

    pub(crate) fn store_ancestors(&mut self, id: NodeId, adjacent_only: bool, nodes: Vec<NodeId>) {
        let cache = if adjacent_only {
            &mut self.ancestor_adjacent_cache
        } else {
            &mut self.ancestor_all_cache
        };
        cache.insert(id, nodes);
    }

    pub(crate) fn cached_descendants(&self, id: NodeId, adjacent_only: bool) -> Option<&[NodeId]> {
        let cache = if adjacent_only {
            &self.descendant_adjacent_cache
        } else {
            &self.descendant_all_cache
        };
        cache.get(id).map(Vec::as_slice)
    }

    pub(crate) fn store_descendants(
        &mut self,
        id: NodeId,
        adjacent_only: bool,
        nodes: Vec<NodeId>,
    ) {
        let cache = if adjacent_only {
            &mut self.descendant_adjacent_cache
        } else {
            &mut self.descendant_all_cache
        };
        cache.insert(id, nodes);
    }

    pub(crate) fn cached_siblings(
        &self,
        id: NodeId,
        forward: bool,
        adjacent_only: bool,
    ) -> Option<&[SiblingCandidate]> {
        self.sibling_cache_table(forward, adjacent_only)
            .get(id)
            .map(Vec::as_slice)
    }

    pub(crate) fn store_siblings(
        &mut self,
        id: NodeId,
        forward: bool,
        adjacent_only: bool,
        siblings: Vec<SiblingCandidate>,
    ) {
        self.sibling_cache_table_mut(forward, adjacent_only)
            .insert(id, siblings);
    }

    fn sibling_cache_table(
        &self,
        forward: bool,
        adjacent_only: bool,
    ) -> &NodeTable<Vec<SiblingCandidate>> {
        match (forward, adjacent_only) {
            (true, true) => &self.sibling_forward_adjacent_cache,
            (true, false) => &self.sibling_forward_all_cache,
            (false, true) => &self.sibling_backward_adjacent_cache,
            (false, false) => &self.sibling_backward_all_cache,
        }
    }

    fn sibling_cache_table_mut(
        &mut self,
        forward: bool,
        adjacent_only: bool,
    ) -> &mut NodeTable<Vec<SiblingCandidate>> {
        match (forward, adjacent_only) {
            (true, true) => &mut self.sibling_forward_adjacent_cache,
            (true, false) => &mut self.sibling_forward_all_cache,
            (false, true) => &mut self.sibling_backward_adjacent_cache,
            (false, false) => &mut self.sibling_backward_all_cache,
        }
    }
}
