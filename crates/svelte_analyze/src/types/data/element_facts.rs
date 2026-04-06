use super::attr_index::AttrIndex;
use super::*;
use svelte_ast::Attribute;

pub struct ElementFactsEntry {
    attr_index: AttrIndex,
    has_spread: bool,
    has_runtime_attrs: bool,
}

impl ElementFactsEntry {
    fn build(attrs: &[Attribute]) -> Self {
        let has_spread = attrs
            .iter()
            .any(|attr| matches!(attr, Attribute::SpreadAttribute(_)));
        let has_runtime_attrs = attrs.iter().any(|attr| {
            !matches!(
                attr,
                Attribute::StringAttribute(_) | Attribute::BooleanAttribute(_)
            )
        });

        Self {
            attr_index: AttrIndex::build(attrs),
            has_spread,
            has_runtime_attrs,
        }
    }

    pub(crate) fn attr_index(&self) -> &AttrIndex {
        &self.attr_index
    }

    pub fn has_spread(&self) -> bool {
        self.has_spread
    }

    pub fn has_runtime_attrs(&self) -> bool {
        self.has_runtime_attrs
    }
}

pub struct ElementFacts {
    entries: NodeTable<ElementFactsEntry>,
}

impl ElementFacts {
    pub fn new(node_count: u32) -> Self {
        Self {
            entries: NodeTable::new(node_count),
        }
    }

    pub(crate) fn record(&mut self, id: NodeId, attrs: &[Attribute]) {
        self.entries.insert(id, ElementFactsEntry::build(attrs));
    }

    pub fn entry(&self, id: NodeId) -> Option<&ElementFactsEntry> {
        self.entries.get(id)
    }

    pub(crate) fn attr_index(&self, id: NodeId) -> Option<&AttrIndex> {
        self.entry(id).map(ElementFactsEntry::attr_index)
    }

    pub fn has_spread(&self, id: NodeId) -> bool {
        self.entry(id).is_some_and(ElementFactsEntry::has_spread)
    }

    pub fn has_runtime_attrs(&self, id: NodeId) -> bool {
        self.entry(id)
            .is_some_and(ElementFactsEntry::has_runtime_attrs)
    }
}
