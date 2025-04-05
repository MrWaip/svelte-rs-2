use std::{cell::RefMut, ops::Index, slice::Iter};

use metadata::{ElementMetadata, FragmentMetadata, InterpolationMetadata};
use oxc_allocator::Allocator;
use oxc_ast::ast::{Expression, Program};
use oxc_span::Language;
use oxc_syntax::node::NodeId;
use rccell::RcCell;
use span::{GetSpan, Span};

use diagnostics::Diagnostic;

mod attribute;
pub mod format;
pub mod metadata;
pub mod node_id;

pub use attribute::*;

pub struct Ast<'a> {
    pub template: RcCell<Template<'a>>,
    pub script: Option<ScriptTag<'a>>,
}

#[derive(Debug)]
pub struct Template<'a> {
    pub nodes: Fragment<'a>,
}

pub trait AsNode<'a> {
    fn as_node(self) -> Node<'a>;
}

#[derive(Debug, Clone)]
pub enum Node<'a> {
    Element(RcCell<Element<'a>>),
    Comment(RcCell<Comment<'a>>),
    Text(RcCell<Text<'a>>),
    Interpolation(RcCell<Interpolation<'a>>),
    IfBlock(RcCell<IfBlock<'a>>),
    /** Напоминание для себя. Сейчас во время трансформации шаблона последовательность Text + Interpolation схлопывается в эту Node */
    VirtualConcatenation(RcCell<VirtualConcatenation<'a>>),
    ScriptTag(RcCell<ScriptTag<'a>>),
}

impl<'a> Node<'a> {
    pub fn as_element(self) -> Option<RcCell<Element<'a>>> {
        if let Node::Element(element) = self {
            Some(element)
        } else {
            None
        }
    }

    pub fn as_text_mut(&mut self) -> Option<RefMut<'_, Text<'a>>> {
        if let Node::Text(it) = self {
            Some(it.borrow_mut())
        } else {
            None
        }
    }

    pub fn is_if_block(&self) -> bool {
        matches!(self, Node::IfBlock(_))
    }

    pub fn is_compressible(&self) -> bool {
        matches!(self, Node::Text(_) | Node::Interpolation(_))
    }

    pub fn is_text(&self) -> bool {
        matches!(self, Node::Text(_))
    }

    pub fn is_element(&self) -> bool {
        matches!(self, Node::Element(_))
    }

    pub fn is_interpolation(&self) -> bool {
        matches!(self, Node::Interpolation(_))
    }

    pub fn from_option_mut<'local, 'long>(
        option: Option<&'local mut Node<'long>>,
    ) -> Result<&'local mut Node<'long>, Diagnostic> {
        if let Some(node) = option {
            Ok(node)
        } else {
            unimplemented!()
        }
    }

    pub fn is_comment(&self) -> bool {
        matches!(self, Node::Comment(_))
    }
}

impl GetSpan for Node<'_> {
    fn span(&self) -> Span {
        match self {
            Node::Element(it) => it.borrow().span,
            Node::Text(it) => it.borrow().span,
            Node::Interpolation(it) => it.borrow().span,
            Node::IfBlock(it) => it.borrow().span,
            Node::VirtualConcatenation(it) => it.borrow().span,
            Node::ScriptTag(it) => it.borrow().span,
            Node::Comment(it) => it.borrow().span,
        }
    }
}

#[derive(Debug)]
pub struct Interpolation<'a> {
    pub expression: Expression<'a>,
    pub span: Span,
    pub metadata: Option<InterpolationMetadata>,
}

impl<'a> AsNode<'a> for Interpolation<'a> {
    fn as_node(self) -> Node<'a> {
        Node::Interpolation(RcCell::new(self))
    }
}

#[derive(Debug)]
pub struct VirtualConcatenation<'a> {
    pub parts: Vec<ConcatenationPart<'a>>,
    pub span: Span,
    pub metadata: InterpolationMetadata,
}

impl<'a> AsNode<'a> for VirtualConcatenation<'a> {
    fn as_node(self) -> Node<'a> {
        Node::VirtualConcatenation(RcCell::new(self))
    }
}

#[derive(Debug)]
pub struct IfBlock<'a> {
    pub span: Span,
    pub test: Expression<'a>,
    pub is_elseif: bool,
    pub consequent: Fragment<'a>,
    pub alternate: Option<Fragment<'a>>,
}
impl<'a> IfBlock<'a> {
    pub fn push(&mut self, node: Node<'a>) {
        if let Some(alternate) = self.alternate.as_mut() {
            alternate.push(node);
        } else {
            self.consequent.push(node);
        }
    }
}

impl<'a> AsNode<'a> for IfBlock<'a> {
    fn as_node(self) -> Node<'a> {
        Node::IfBlock(RcCell::new(self))
    }
}

#[derive(Debug)]
pub struct Element<'a> {
    pub name: String,
    pub span: Span,
    pub self_closing: bool,
    pub nodes: Vec<Node<'a>>,
    pub attributes: Vec<Attribute<'a>>,
    pub metadata: Option<ElementMetadata>,
    pub node_id: Option<NodeId>,
    pub kind: ElementKind,
}

#[derive(Debug)]
pub enum ElementKind {
    Unknown,
    Input,
    // ... other
}

impl ElementKind {
    pub fn from_str(value: &str) -> Self {
        match value {
            "input" => ElementKind::Input,
            _ => ElementKind::Unknown,
        }
    }

    pub fn is_input(&self) -> bool {
        matches!(self, ElementKind::Input)
    }
}

impl<'a> AsNode<'a> for Element<'a> {
    fn as_node(self) -> Node<'a> {
        Node::Element(RcCell::new(self))
    }
}

impl<'a> Element<'a> {
    pub fn push(&mut self, node: Node<'a>) {
        self.nodes.push(node);
    }
}

#[derive(Debug)]
pub struct Comment<'a> {
    pub value: &'a str,
    pub span: Span,
}

impl<'a> AsNode<'a> for Comment<'a> {
    fn as_node(self) -> Node<'a> {
        Node::Comment(RcCell::new(self))
    }
}

#[derive(Debug)]
pub struct Text<'a> {
    pub value: &'a str,
    pub span: Span,
}

impl<'a> Text<'a> {
    pub fn is_removable(&self) -> bool {
        self.value.chars().all(|char| char.is_whitespace())
    }

    pub fn trim_start(&mut self) -> bool {
        let new = self.value.trim_ascii_start();
        let trimmed = new.len() != self.value.len();
        self.value = new;
        trimmed
    }

    pub fn trim_end(&mut self) -> bool {
        let new = self.value.trim_ascii_end();
        let trimmed = new.len() != self.value.len();
        self.value = new;
        trimmed
    }

    pub fn trim(&mut self) {
        self.value = self.value.trim_ascii();
    }

    pub fn trim_one_whitespace(&mut self, allocator: &'a Allocator) {
        self.trim_start_one_whitespace(allocator);
        self.trim_end_one_whitespace(allocator);
    }

    pub fn trim_start_one_whitespace(&mut self, allocator: &'a Allocator) {
        if !self.trim_start() {
            return;
        }

        let mut new = String::from(" ");
        new.push_str(self.value);

        self.value = allocator.alloc_str(new.as_str());
    }

    pub fn trim_end_one_whitespace(&mut self, allocator: &'a Allocator) {
        if !self.trim_end() {
            return;
        }

        let mut new = String::from(self.value);
        new.push(' ');

        self.value = allocator.alloc_str(new.as_str());
    }
}

impl<'a> AsNode<'a> for Text<'a> {
    fn as_node(self) -> Node<'a> {
        Node::Text(RcCell::new(self))
    }
}

#[derive(Debug)]
pub struct ScriptTag<'a> {
    pub program: Program<'a>,
    pub span: Span,
    pub language: Language,
}

impl ScriptTag<'_> {
    pub fn is_typescript(&self) -> bool {
        self.language == Language::TypeScript
    }
}

impl<'a> From<Node<'a>> for ScriptTag<'a> {
    fn from(val: Node<'a>) -> Self {
        match val {
            Node::ScriptTag(script_tag) => script_tag.unwrap(),
            _ => panic!("node is not ScriptTag"),
        }
    }
}

impl<'a> AsNode<'a> for ScriptTag<'a> {
    fn as_node(self) -> Node<'a> {
        Node::ScriptTag(RcCell::new(self))
    }
}

#[derive(Debug)]
pub struct Fragment<'a> {
    pub nodes: Vec<Node<'a>>,
    pub metadata: Option<FragmentMetadata>,
    pub node_id: Option<NodeId>,
}

impl<'a> Fragment<'a> {
    pub fn push(&mut self, node: Node<'a>) {
        self.nodes.push(node);
    }

    pub fn iter(&self) -> Iter<Node<'a>> {
        self.nodes.iter()
    }

    pub fn first(&self) -> Option<&Node<'a>> {
        self.nodes.first()
    }

    pub fn from(nodes: Vec<Node<'a>>) -> Self {
        Self {
            metadata: None,
            nodes,
            node_id: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn empty() -> Self {
        Self {
            metadata: None,
            nodes: vec![],
            node_id: None,
        }
    }
}

impl<'a> Index<usize> for Fragment<'a> {
    type Output = Node<'a>;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.nodes[idx]
    }
}
