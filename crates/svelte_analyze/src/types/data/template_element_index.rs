use compact_str::CompactString;
use rustc_hash::FxHashMap;
use smallvec::SmallVec;
use std::mem;
use svelte_ast::NodeId;

use super::{ElementFactsEntry, NodeTable};

static EMPTY_NODE_IDS: [NodeId; 0] = [];

pub struct TemplateElementEntry {
    tag_name: CompactString,
    parent_element: Option<NodeId>,
}

impl TemplateElementEntry {
    fn build(tag_name: &str, parent_element: Option<NodeId>) -> Self {
        Self {
            tag_name: CompactString::new(tag_name),
            parent_element,
        }
    }

    pub fn tag_name(&self) -> &str {
        self.tag_name.as_str()
    }

    pub fn parent_element(&self) -> Option<NodeId> {
        self.parent_element
    }
}

pub struct TemplateElementIndex {
    entries: NodeTable<TemplateElementEntry>,
    all_elements: Vec<NodeId>,
    by_tag: FxHashMap<CompactString, SmallVec<[NodeId; 4]>>,
    by_static_class: FxHashMap<CompactString, SmallVec<[NodeId; 4]>>,
    by_static_id: FxHashMap<CompactString, SmallVec<[NodeId; 4]>>,
    maybe_matches_class: Vec<NodeId>,
    maybe_matches_id: Vec<NodeId>,
    previous_sibling: NodeTable<NodeId>,
    next_sibling: NodeTable<NodeId>,
    last_child_by_parent: FxHashMap<Option<NodeId>, NodeId>,
}

impl TemplateElementIndex {
    pub fn new(node_count: u32) -> Self {
        Self {
            entries: NodeTable::new(node_count),
            all_elements: Vec::new(),
            by_tag: FxHashMap::default(),
            by_static_class: FxHashMap::default(),
            by_static_id: FxHashMap::default(),
            maybe_matches_class: Vec::new(),
            maybe_matches_id: Vec::new(),
            previous_sibling: NodeTable::new(node_count),
            next_sibling: NodeTable::new(node_count),
            last_child_by_parent: FxHashMap::default(),
        }
    }

    pub fn record(
        &mut self,
        id: NodeId,
        tag_name: &str,
        facts: &ElementFactsEntry,
        parent_element: Option<NodeId>,
    ) {
        let entry = TemplateElementEntry::build(tag_name, parent_element);
        self.by_tag
            .entry(CompactString::new(tag_name))
            .or_default()
            .push(id);
        for class_name in facts.static_classes() {
            self.by_static_class
                .entry(class_name.clone())
                .or_default()
                .push(id);
        }
        if let Some(static_id) = facts.static_id() {
            self.by_static_id
                .entry(CompactString::new(static_id))
                .or_default()
                .push(id);
        }
        if facts.has_spread() || facts.has_dynamic_class() {
            self.maybe_matches_class.push(id);
        }
        if facts.has_spread() || facts.has_dynamic_id() {
            self.maybe_matches_id.push(id);
        }
        if let Some(prev_id) = self.last_child_by_parent.insert(parent_element, id) {
            self.previous_sibling.insert(id, prev_id);
            self.next_sibling.insert(prev_id, id);
        }
        self.all_elements.push(id);
        self.entries.insert(id, entry);
    }

    pub fn finalize(&mut self) {
        let _ = mem::take(&mut self.last_child_by_parent);
    }

    pub fn entry(&self, id: NodeId) -> Option<&TemplateElementEntry> {
        self.entries.get(id)
    }

    pub fn all_elements(&self) -> &[NodeId] {
        &self.all_elements
    }

    pub fn elements_with_tag(&self, tag_name: &str) -> &[NodeId] {
        self.by_tag
            .get(tag_name)
            .map(SmallVec::as_slice)
            .unwrap_or(&EMPTY_NODE_IDS)
    }

    pub fn elements_with_static_class(&self, class_name: &str) -> &[NodeId] {
        self.by_static_class
            .get(class_name)
            .map(SmallVec::as_slice)
            .unwrap_or(&EMPTY_NODE_IDS)
    }

    pub fn elements_with_static_id(&self, id_name: &str) -> &[NodeId] {
        self.by_static_id
            .get(id_name)
            .map(SmallVec::as_slice)
            .unwrap_or(&EMPTY_NODE_IDS)
    }

    pub fn parent_element(&self, id: NodeId) -> Option<NodeId> {
        self.entry(id)
            .and_then(TemplateElementEntry::parent_element)
    }

    pub fn previous_sibling(&self, id: NodeId) -> Option<NodeId> {
        self.previous_sibling.get(id).copied()
    }

    pub fn next_sibling(&self, id: NodeId) -> Option<NodeId> {
        self.next_sibling.get(id).copied()
    }

    pub fn tag_name(&self, id: NodeId) -> Option<&str> {
        self.entry(id).map(TemplateElementEntry::tag_name)
    }

    pub fn class_candidates(&self, class_name: &str) -> impl Iterator<Item = NodeId> + '_ {
        self.elements_with_static_class(class_name)
            .iter()
            .copied()
            .chain(self.maybe_matches_class.iter().copied())
    }

    pub fn id_candidates(&self, id_name: &str) -> impl Iterator<Item = NodeId> + '_ {
        self.elements_with_static_id(id_name)
            .iter()
            .copied()
            .chain(self.maybe_matches_id.iter().copied())
    }

    pub fn previous_siblings(&self, id: NodeId) -> impl Iterator<Item = NodeId> + '_ {
        std::iter::successors(self.previous_sibling(id), move |&prev_id| {
            self.previous_sibling(prev_id)
        })
    }

    pub fn is_empty(&self) -> bool {
        self.all_elements.is_empty()
    }
}
