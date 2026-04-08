use super::attr_index::AttrIndex;
use super::*;
use compact_str::CompactString;
use smallvec::SmallVec;
use svelte_ast::{Attribute, Namespace};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum NamespaceKind {
    Html,
    Svg,
    MathMl,
    ForeignObject,
    AnnotationXml,
}

impl NamespaceKind {
    pub fn as_namespace(self) -> Namespace {
        match self {
            Self::Html | Self::ForeignObject | Self::AnnotationXml => Namespace::Html,
            Self::Svg => Namespace::Svg,
            Self::MathMl => Namespace::Mathml,
        }
    }

    pub fn creation_namespace(self) -> Namespace {
        match self {
            Self::ForeignObject => Namespace::Svg,
            Self::AnnotationXml => Namespace::Mathml,
            _ => self.as_namespace(),
        }
    }
}

pub struct ElementFactsEntry {
    attr_index: AttrIndex,
    has_spread: bool,
    has_runtime_attrs: bool,
    namespace: NamespaceKind,
    creation_namespace: Namespace,
    is_void: bool,
    is_custom_element: bool,
    static_id: Option<CompactString>,
    static_classes: SmallVec<[CompactString; 2]>,
    has_dynamic_id: bool,
    has_dynamic_class: bool,
}

impl ElementFactsEntry {
    pub(crate) fn build(
        attrs: &[Attribute],
        source: &str,
        namespace: NamespaceKind,
        creation_namespace: Namespace,
        is_void: bool,
        is_custom_element: bool,
    ) -> Self {
        let mut has_spread = false;
        let mut has_runtime_attrs = false;
        let mut static_id = None;
        let mut static_classes = SmallVec::new();
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
                    has_runtime_attrs = true;
                }
                Attribute::ConcatenationAttribute(attr) if attr.name == "id" => {
                    has_dynamic_id = true;
                    has_runtime_attrs = true;
                }
                Attribute::BindDirective(attr) if attr.name == "id" => {
                    has_dynamic_id = true;
                    has_runtime_attrs = true;
                }
                Attribute::ExpressionAttribute(attr) if attr.name == "class" => {
                    has_dynamic_class = true;
                    has_runtime_attrs = true;
                }
                Attribute::ConcatenationAttribute(attr) if attr.name == "class" => {
                    has_dynamic_class = true;
                    has_runtime_attrs = true;
                }
                Attribute::ClassDirective(_) => {
                    has_dynamic_class = true;
                    has_runtime_attrs = true;
                }
                Attribute::SpreadAttribute(_) => {
                    has_spread = true;
                    has_runtime_attrs = true;
                }
                Attribute::StringAttribute(_) | Attribute::BooleanAttribute(_) => {}
                _ => {
                    has_runtime_attrs = true;
                }
            }
        }

        Self {
            attr_index: AttrIndex::build(attrs),
            has_spread,
            has_runtime_attrs,
            namespace,
            creation_namespace,
            is_void,
            is_custom_element,
            static_id,
            static_classes,
            has_dynamic_id,
            has_dynamic_class,
        }
    }

    pub fn attr_index(&self) -> &AttrIndex {
        &self.attr_index
    }

    pub fn has_spread(&self) -> bool {
        self.has_spread
    }

    pub fn has_runtime_attrs(&self) -> bool {
        self.has_runtime_attrs
    }

    pub fn namespace(&self) -> NamespaceKind {
        self.namespace
    }

    pub fn creation_namespace(&self) -> Namespace {
        self.creation_namespace
    }

    pub fn is_void(&self) -> bool {
        self.is_void
    }

    pub fn is_custom_element(&self) -> bool {
        self.is_custom_element
    }

    pub fn static_id(&self) -> Option<&str> {
        self.static_id.as_deref()
    }

    pub fn static_classes(&self) -> &[CompactString] {
        &self.static_classes
    }

    pub fn has_static_class(&self, class_name: &str) -> bool {
        self.static_classes.iter().any(|cls| cls == class_name)
    }

    pub fn has_dynamic_id(&self) -> bool {
        self.has_dynamic_id
    }

    pub fn has_dynamic_class(&self) -> bool {
        self.has_dynamic_class
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

    pub(crate) fn record_entry(&mut self, id: NodeId, entry: ElementFactsEntry) {
        self.entries.insert(id, entry);
    }

    pub fn entry(&self, id: NodeId) -> Option<&ElementFactsEntry> {
        self.entries.get(id)
    }

    pub fn attr_index(&self, id: NodeId) -> Option<&AttrIndex> {
        self.entry(id).map(ElementFactsEntry::attr_index)
    }

    pub fn has_spread(&self, id: NodeId) -> bool {
        self.entry(id).is_some_and(ElementFactsEntry::has_spread)
    }

    pub fn has_runtime_attrs(&self, id: NodeId) -> bool {
        self.entry(id)
            .is_some_and(ElementFactsEntry::has_runtime_attrs)
    }

    pub fn namespace(&self, id: NodeId) -> Option<NamespaceKind> {
        self.entry(id).map(ElementFactsEntry::namespace)
    }

    pub fn creation_namespace(&self, id: NodeId) -> Option<Namespace> {
        self.entry(id).map(ElementFactsEntry::creation_namespace)
    }

    pub fn is_void(&self, id: NodeId) -> bool {
        self.entry(id).is_some_and(ElementFactsEntry::is_void)
    }

    pub fn is_custom_element(&self, id: NodeId) -> bool {
        self.entry(id)
            .is_some_and(ElementFactsEntry::is_custom_element)
    }

    pub fn static_id(&self, id: NodeId) -> Option<&str> {
        self.entry(id).and_then(ElementFactsEntry::static_id)
    }

    pub fn has_static_class(&self, id: NodeId, class_name: &str) -> bool {
        self.entry(id)
            .is_some_and(|entry| entry.has_static_class(class_name))
    }

    pub fn may_match_class(&self, id: NodeId) -> bool {
        self.entry(id)
            .is_some_and(|entry| entry.has_spread() || entry.has_dynamic_class())
    }

    pub fn may_match_id(&self, id: NodeId) -> bool {
        self.entry(id)
            .is_some_and(|entry| entry.has_spread() || entry.has_dynamic_id())
    }
}
