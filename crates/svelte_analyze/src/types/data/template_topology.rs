use super::*;
use svelte_ast::Attribute;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ParentKind {
    Element,
    SlotElementLegacy,
    IfBlock,
    EachBlock,
    SnippetBlock,
    ComponentNode,
    KeyBlock,
    SvelteHead,
    SvelteFragmentLegacy,
    SvelteElement,
    SvelteWindow,
    SvelteDocument,
    SvelteBody,
    SvelteBoundary,
    AwaitBlock,
    ExpressionAttribute,
    ConcatenationAttribute,
    SpreadAttribute,
    ClassDirective,
    StyleDirective,
    BindDirective,
    LetDirectiveLegacy,
    UseDirective,
    OnDirectiveLegacy,
    TransitionDirective,
    AnimateDirective,
    AttachTag,
}

impl ParentKind {
    pub fn is_element(&self) -> bool {
        matches!(
            self,
            Self::Element
                | Self::SlotElementLegacy
                | Self::SvelteFragmentLegacy
                | Self::SvelteElement
        )
    }

    pub fn is_attr(&self) -> bool {
        matches!(
            self,
            Self::ExpressionAttribute
                | Self::ConcatenationAttribute
                | Self::SpreadAttribute
                | Self::ClassDirective
                | Self::StyleDirective
                | Self::BindDirective
                | Self::LetDirectiveLegacy
                | Self::UseDirective
                | Self::OnDirectiveLegacy
                | Self::TransitionDirective
                | Self::AnimateDirective
                | Self::AttachTag
        )
    }

    pub fn needs_element_ref(&self) -> bool {
        match self {
            Self::BindDirective
            | Self::LetDirectiveLegacy
            | Self::UseDirective
            | Self::TransitionDirective
            | Self::AnimateDirective
            | Self::AttachTag => true,
            Self::Element
            | Self::SlotElementLegacy
            | Self::IfBlock
            | Self::EachBlock
            | Self::SnippetBlock
            | Self::ComponentNode
            | Self::KeyBlock
            | Self::SvelteHead
            | Self::SvelteFragmentLegacy
            | Self::SvelteElement
            | Self::SvelteWindow
            | Self::SvelteDocument
            | Self::SvelteBody
            | Self::SvelteBoundary
            | Self::AwaitBlock
            | Self::ExpressionAttribute
            | Self::ConcatenationAttribute
            | Self::SpreadAttribute
            | Self::ClassDirective
            | Self::StyleDirective
            | Self::OnDirectiveLegacy => false,
        }
    }

    pub fn from_attr(attr: &Attribute) -> Option<Self> {
        match attr {
            Attribute::ExpressionAttribute(_) => Some(Self::ExpressionAttribute),
            Attribute::ConcatenationAttribute(_) => Some(Self::ConcatenationAttribute),
            Attribute::SpreadAttribute(_) => Some(Self::SpreadAttribute),
            Attribute::ClassDirective(_) => Some(Self::ClassDirective),
            Attribute::StyleDirective(_) => Some(Self::StyleDirective),
            Attribute::BindDirective(_) => Some(Self::BindDirective),
            Attribute::LetDirectiveLegacy(_) => Some(Self::LetDirectiveLegacy),
            Attribute::UseDirective(_) => Some(Self::UseDirective),
            Attribute::OnDirectiveLegacy(_) => Some(Self::OnDirectiveLegacy),
            Attribute::TransitionDirective(_) => Some(Self::TransitionDirective),
            Attribute::AnimateDirective(_) => Some(Self::AnimateDirective),
            Attribute::AttachTag(_) => Some(Self::AttachTag),
            Attribute::StringAttribute(_) | Attribute::BooleanAttribute(_) => None,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ParentRef {
    pub id: NodeId,
    pub kind: ParentKind,
}

pub struct TemplateTopology {
    node_parents: NodeTable<ParentRef>,
    expr_parents: NodeTable<ParentRef>,
    node_fragments: NodeTable<FragmentKey>,
}

impl TemplateTopology {
    pub fn new(node_count: u32) -> Self {
        Self {
            node_parents: NodeTable::new(node_count),
            expr_parents: NodeTable::new(node_count),
            node_fragments: NodeTable::new(node_count),
        }
    }

    pub fn record_node_parent(&mut self, id: NodeId, parent: Option<ParentRef>) {
        if let Some(parent) = parent {
            self.node_parents.insert(id, parent);
        }
    }

    pub fn record_expr_parent(&mut self, id: NodeId, parent: Option<ParentRef>) {
        if let Some(parent) = parent {
            self.expr_parents.insert(id, parent);
        }
    }

    pub fn parent(&self, id: NodeId) -> Option<ParentRef> {
        self.node_parents.get(id).copied()
    }

    pub fn record_node_fragment(&mut self, id: NodeId, key: FragmentKey) {
        self.node_fragments.insert(id, key);
    }

    pub fn node_fragment(&self, id: NodeId) -> Option<FragmentKey> {
        self.node_fragments.get(id).copied()
    }

    pub fn expr_parent(&self, id: NodeId) -> Option<ParentRef> {
        self.expr_parents.get(id).copied()
    }

    pub fn ancestors(&self, id: NodeId) -> Ancestors<'_> {
        Ancestors {
            topology: self,
            current: self.parent(id),
        }
    }

    pub fn expr_ancestors(&self, id: NodeId) -> Ancestors<'_> {
        Ancestors {
            topology: self,
            current: self.expr_parent(id),
        }
    }

    pub fn nearest_element(&self, id: NodeId) -> Option<NodeId> {
        self.ancestors(id)
            .find(|parent| parent.kind.is_element())
            .map(|parent| parent.id)
    }

    pub fn nearest_element_for_expr(&self, id: NodeId) -> Option<NodeId> {
        self.expr_ancestors(id)
            .find(|parent| parent.kind.is_element())
            .map(|parent| parent.id)
    }
}

pub struct Ancestors<'a> {
    topology: &'a TemplateTopology,
    current: Option<ParentRef>,
}

impl Iterator for Ancestors<'_> {
    type Item = ParentRef;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current?;
        self.current = self.topology.parent(current.id);
        Some(current)
    }
}
