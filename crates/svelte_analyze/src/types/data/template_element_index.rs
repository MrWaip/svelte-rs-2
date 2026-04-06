use compact_str::CompactString;
use rustc_hash::FxHashMap;
use smallvec::SmallVec;
use svelte_ast::{Attribute, NodeId};

use super::NodeTable;

static EMPTY_NODE_IDS: [NodeId; 0] = [];

pub struct TemplateElementEntry {
    tag_name: CompactString,
    parent_element: Option<NodeId>,
    static_id: Option<CompactString>,
    static_classes: SmallVec<[CompactString; 2]>,
    has_spread: bool,
    has_dynamic_id: bool,
    has_dynamic_class: bool,
}

impl TemplateElementEntry {
    fn build(
        tag_name: &str,
        attrs: &[Attribute],
        parent_element: Option<NodeId>,
        source: &str,
    ) -> Self {
        let mut static_id = None;
        let mut static_classes = SmallVec::new();
        let mut has_spread = false;
        let mut has_dynamic_id = false;
        let mut has_dynamic_class = false;

        for attr in attrs {
            match attr {
                Attribute::StringAttribute(attr) if attr.name == "id" => {
                    static_id = Some(CompactString::new(attr.value_span.source_text(source)));
                }
                Attribute::StringAttribute(attr) if attr.name == "class" => {
                    static_classes.extend(
                        attr.value_span
                            .source_text(source)
                            .split_whitespace()
                            .map(CompactString::new),
                    );
                }
                Attribute::ExpressionAttribute(attr) if attr.name == "id" => {
                    has_dynamic_id = true;
                }
                Attribute::ConcatenationAttribute(attr) if attr.name == "id" => {
                    has_dynamic_id = true;
                }
                Attribute::BindDirective(attr) if attr.name == "id" => {
                    has_dynamic_id = true;
                }
                Attribute::ExpressionAttribute(attr) if attr.name == "class" => {
                    has_dynamic_class = true;
                }
                Attribute::ConcatenationAttribute(attr) if attr.name == "class" => {
                    has_dynamic_class = true;
                }
                Attribute::ClassDirective(_) => {
                    has_dynamic_class = true;
                }
                Attribute::SpreadAttribute(_) => {
                    has_spread = true;
                }
                _ => {}
            }
        }

        Self {
            tag_name: CompactString::new(tag_name),
            parent_element,
            static_id,
            static_classes,
            has_spread,
            has_dynamic_id,
            has_dynamic_class,
        }
    }

    pub fn tag_name(&self) -> &str {
        self.tag_name.as_str()
    }

    pub fn parent_element(&self) -> Option<NodeId> {
        self.parent_element
    }

    pub fn static_id(&self) -> Option<&str> {
        self.static_id.as_deref()
    }

    pub fn static_classes(&self) -> &[CompactString] {
        &self.static_classes
    }

    pub fn has_spread(&self) -> bool {
        self.has_spread
    }

    pub fn has_dynamic_id(&self) -> bool {
        self.has_dynamic_id
    }

    pub fn has_dynamic_class(&self) -> bool {
        self.has_dynamic_class
    }
}

pub struct TemplateElementIndex {
    entries: NodeTable<TemplateElementEntry>,
    all_elements: Vec<NodeId>,
    by_tag: FxHashMap<CompactString, Vec<NodeId>>,
    by_static_class: FxHashMap<CompactString, Vec<NodeId>>,
    by_static_id: FxHashMap<CompactString, Vec<NodeId>>,
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
        attrs: &[Attribute],
        parent_element: Option<NodeId>,
        source: &str,
    ) {
        let entry = TemplateElementEntry::build(tag_name, attrs, parent_element, source);
        self.by_tag
            .entry(CompactString::new(tag_name))
            .or_default()
            .push(id);
        for class_name in entry.static_classes() {
            self.by_static_class
                .entry(class_name.clone())
                .or_default()
                .push(id);
        }
        if let Some(static_id) = entry.static_id() {
            self.by_static_id
                .entry(CompactString::new(static_id))
                .or_default()
                .push(id);
        }
        if entry.has_spread() || entry.has_dynamic_class() {
            self.maybe_matches_class.push(id);
        }
        if entry.has_spread() || entry.has_dynamic_id() {
            self.maybe_matches_id.push(id);
        }
        if let Some(prev_id) = self.last_child_by_parent.insert(parent_element, id) {
            self.previous_sibling.insert(id, prev_id);
            self.next_sibling.insert(prev_id, id);
        }
        self.all_elements.push(id);
        self.entries.insert(id, entry);
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
            .map(Vec::as_slice)
            .unwrap_or(&EMPTY_NODE_IDS)
    }

    pub fn elements_with_static_class(&self, class_name: &str) -> &[NodeId] {
        self.by_static_class
            .get(class_name)
            .map(Vec::as_slice)
            .unwrap_or(&EMPTY_NODE_IDS)
    }

    pub fn elements_with_static_id(&self, id_name: &str) -> &[NodeId] {
        self.by_static_id
            .get(id_name)
            .map(Vec::as_slice)
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

    pub fn static_id(&self, id: NodeId) -> Option<&str> {
        self.entry(id).and_then(TemplateElementEntry::static_id)
    }

    pub fn has_static_class(&self, id: NodeId, class_name: &str) -> bool {
        self.entry(id)
            .is_some_and(|entry| entry.static_classes().iter().any(|cls| cls == class_name))
    }

    pub fn may_match_class(&self, id: NodeId) -> bool {
        self.entry(id)
            .is_some_and(|entry| entry.has_spread() || entry.has_dynamic_class())
    }

    pub fn may_match_id(&self, id: NodeId) -> bool {
        self.entry(id)
            .is_some_and(|entry| entry.has_spread() || entry.has_dynamic_id())
    }

    pub fn class_candidates(&self, class_name: &str) -> SmallVec<[NodeId; 8]> {
        let mut candidates =
            SmallVec::<[NodeId; 8]>::from_slice(self.elements_with_static_class(class_name));
        candidates.extend(self.maybe_matches_class.iter().copied());
        candidates
    }

    pub fn id_candidates(&self, id_name: &str) -> SmallVec<[NodeId; 8]> {
        let mut candidates =
            SmallVec::<[NodeId; 8]>::from_slice(self.elements_with_static_id(id_name));
        candidates.extend(self.maybe_matches_id.iter().copied());
        candidates
    }

    pub fn previous_siblings(&self, id: NodeId) -> SmallVec<[NodeId; 8]> {
        let mut siblings = SmallVec::<[NodeId; 8]>::new();
        let mut current = self.previous_sibling(id);
        while let Some(prev_id) = current {
            siblings.push(prev_id);
            current = self.previous_sibling(prev_id);
        }
        siblings
    }

    pub fn is_empty(&self) -> bool {
        self.all_elements.is_empty()
    }
}
