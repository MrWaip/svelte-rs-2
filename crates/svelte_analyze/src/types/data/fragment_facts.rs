use svelte_ast::{AstStore, Attribute, Fragment, FragmentId, Node, NodeId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FragmentFactsEntry {
    child_count: u32,
    single_child: Option<NodeId>,
    non_trivial_child_count: u32,
    single_non_trivial_child: Option<NodeId>,
    has_expression_child: bool,
    single_expression_child: Option<NodeId>,
    has_direct_animate_child: bool,
}

impl FragmentFactsEntry {
    pub(crate) fn from_fragment(fragment: &Fragment, store: &AstStore, source: &str) -> Self {
        let child_count = fragment.nodes.len() as u32;
        let single_child = if child_count == 1 {
            Some(fragment.nodes[0])
        } else {
            None
        };
        let has_expression_child = fragment
            .nodes
            .iter()
            .any(|&id| matches!(store.get(id), Node::ExpressionTag(_)));
        let single_expression_child =
            single_child.filter(|&id| matches!(store.get(id), Node::ExpressionTag(_)));
        let has_direct_animate_child = fragment.nodes.iter().any(|&id| match store.get(id) {
            Node::Element(el) => el
                .attributes
                .iter()
                .any(|attr| matches!(attr, Attribute::AnimateDirective(_))),
            Node::SvelteElement(el) => el
                .attributes
                .iter()
                .any(|attr| matches!(attr, Attribute::AnimateDirective(_))),
            _ => false,
        });
        let mut non_trivial_children = fragment
            .nodes
            .iter()
            .copied()
            .filter(|&id| !is_trivial_node(store.get(id), source));
        let first_non_trivial_child = non_trivial_children.next();
        let second_non_trivial_child = non_trivial_children.next();
        let single_non_trivial_child = match (first_non_trivial_child, second_non_trivial_child) {
            (Some(id), None) => Some(id),
            _ => None,
        };
        let non_trivial_child_count = match (first_non_trivial_child, second_non_trivial_child) {
            (None, _) => 0,
            (Some(_), None) => 1,
            (Some(_), Some(_)) => 2,
        };

        Self {
            child_count,
            single_child,
            non_trivial_child_count,
            single_non_trivial_child,
            has_expression_child,
            single_expression_child,
            has_direct_animate_child,
        }
    }

    pub fn child_count(&self) -> u32 {
        self.child_count
    }

    pub fn has_children(&self) -> bool {
        self.child_count != 0
    }

    pub fn single_child(&self) -> Option<NodeId> {
        self.single_child
    }

    pub fn non_trivial_child_count(&self) -> u32 {
        self.non_trivial_child_count
    }

    pub fn has_non_trivial_children(&self) -> bool {
        self.non_trivial_child_count != 0
    }

    pub fn has_expression_child(&self) -> bool {
        self.has_expression_child
    }

    pub fn single_expression_child(&self) -> Option<NodeId> {
        self.single_expression_child
    }

    pub fn single_non_trivial_child(&self) -> Option<NodeId> {
        self.single_non_trivial_child
    }

    pub fn has_direct_animate_child(&self) -> bool {
        self.has_direct_animate_child
    }
}

pub struct FragmentFacts {
    entries: Vec<Option<FragmentFactsEntry>>,
}

impl Default for FragmentFacts {
    fn default() -> Self {
        Self::new()
    }
}

impl FragmentFacts {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub(crate) fn record(&mut self, id: FragmentId, entry: FragmentFactsEntry) {
        let idx = id.0 as usize;
        if self.entries.len() <= idx {
            self.entries.resize(idx + 1, None);
        }
        self.entries[idx] = Some(entry);
    }

    pub fn lookup_by_id(&self, id: FragmentId) -> Option<&FragmentFactsEntry> {
        self.entries.get(id.0 as usize)?.as_ref()
    }
}

fn is_trivial_node(node: &Node, source: &str) -> bool {
    match node {
        Node::Comment(_) | Node::ConstTag(_) => true,
        Node::Text(text) => text.value(source).trim().is_empty(),
        _ => false,
    }
}
