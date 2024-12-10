use std::fmt::Debug;

use oxc_ast::ast::Expression;
use rccell::RcCell;

use crate::parser::span::{GetSpan, Span};

pub struct Ast<'a> {
    pub template: Vec<RcCell<Node<'a>>>,
}

pub trait FormatNode {
    fn format_node(&self) -> String;
}

pub trait AsNode<'a> {
    fn as_node(self) -> Node<'a>;
}

#[derive(Debug)]
pub enum Node<'a> {
    Element(Element<'a>),
    Text(Text),
    Interpolation(Interpolation<'a>),
}

impl<'a> Node<'a> {
    pub fn as_rc_cell(self) -> RcCell<Node<'a>> {
        return RcCell::new(self);
    }
}

impl<'a> FormatNode for Node<'a> {
    fn format_node(&self) -> String {
        return match self {
            Node::Element(element) => element.format_node(),
            Node::Text(text) => text.format_node(),
            Node::Interpolation(interpolation) => interpolation.format_node(),
        };
    }
}

impl<'a> GetSpan for Node<'a> {
    fn span(&self) -> Span {
        match self {
            Node::Element(element) => element.span,
            Node::Text(text) => text.span,
            Node::Interpolation(interpolation) => interpolation.span,
        }
    }
}

#[derive(Debug)]
pub struct Interpolation<'a> {
    pub expression: Expression<'a>,
    pub span: Span,
}

impl<'a> FormatNode for Interpolation<'a> {
    fn format_node(&self) -> String {
        let mut result = String::new();

        result.push_str("{ ");

        let mut codegen = oxc_codegen::Codegen::default();
        codegen.print_expression(&self.expression);
        result.push_str(codegen.into_source_text().as_str());

        result.push_str(" }");

        return result;
    }
}

impl<'a> AsNode<'a> for Interpolation<'a> {
    fn as_node(self) -> Node<'a> {
        return Node::Interpolation(self);
    }
}

#[derive(Debug)]
pub struct Element<'a> {
    pub name: String,
    pub span: Span,
    pub self_closing: bool,
    pub nodes: Vec<RcCell<Node<'a>>>,
}

impl<'a> FormatNode for Element<'a> {
    fn format_node(&self) -> String {
        let mut result = String::new();

        result.push_str("<");
        result.push_str(&self.name);

        if self.self_closing {
            result.push_str(" />");
            return result;
        } else {
            result.push_str(">");
        }

        for node in self.nodes.iter() {
            let formatted = node.borrow().format_node();
            result.push_str(&formatted);
        }

        result.push_str("</");
        result.push_str(&self.name);
        result.push_str(">");

        return result;
    }
}

impl<'a> AsNode<'a> for Element<'a> {
    fn as_node(self) -> Node<'a> {
        return Node::Element(self);
    }
}

#[derive(Debug)]
pub struct Text {
    pub value: String,
    pub span: Span,
}

impl FormatNode for Text {
    fn format_node(&self) -> String {
        return self.value.clone();
    }
}

impl<'a> AsNode<'a> for Text {
    fn as_node(self) -> Node<'a> {
        return Node::Text(self);
    }
}
