use crate::{Comment, Concatenation, Element, IfBlock, Interpolation, Text};

#[derive(Debug)]
pub enum Node<'hir> {
    Text(&'hir Text<'hir>),
    Interpolation(&'hir Interpolation),
    Element(&'hir Element<'hir>),
    Comment(&'hir Comment<'hir>),
    IfBlock(&'hir IfBlock),
    EachBlock,
    Script,
    Concatenation(&'hir Concatenation<'hir>),
    Phantom,
}

impl<'hir> Node<'hir> {
    pub fn is_element(&self) -> bool {
        matches!(self, Node::Element(_))
    }

    pub fn as_element(&self) -> Option<&'hir Element<'hir>> {
        match self {
            Node::Element(element) => Some(element),
            _ => None,
        }
    }

    pub fn as_text(&self) -> Option<&'hir Text<'hir>> {
        match self {
            Node::Text(text) => Some(text),
            _ => None,
        }
    }

    pub fn is_interpolation_like(&self) -> bool {
        matches!(self, Node::Interpolation(_) | Node::Concatenation(_))
    }

    pub fn is_text(&self) -> bool {
        matches!(self, Node::Text(_))
    }

    pub fn is_text_like(&self) -> bool {
        matches!(
            self,
            Node::Text(_) | Node::Interpolation(_) | Node::Concatenation(_)
        )
    }

    pub fn is_elseif_block(&self) -> bool {
        matches!(self, Node::IfBlock(it) if it.is_elseif)
    }

    pub fn contains_expression(&self) -> bool {
        match self {
            Node::Element(_) => false,
            Node::Interpolation(_) => true,
            Node::Text(_) => false,
            Node::IfBlock(_) => true,
            Node::Script => true,
            Node::Concatenation(_) => true,
            Node::EachBlock => true,
            Node::Phantom => false,
            Node::Comment(_) => false,
        }
    }
}
