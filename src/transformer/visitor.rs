use std::borrow::Borrow;

use oxc_ast::ast::Expression;
use rccell::RcCell;

use crate::ast::{Ast, Attribute, Element, HTMLAttribute, IfBlock, Interpolation, Node, Text};

pub struct Visitor<'a> {
    ast: Ast<'a>,
}

impl<'a> Visitor<'a> {
    pub fn new(ast: Ast<'a>) -> Visitor<'a> {
        return Visitor { ast };
    }

    pub fn walk(&self) {
        self.visit_nodes(&self.ast.template);
    }

    fn visit_nodes(&self, nodes: &Vec<RcCell<Node<'a>>>) {
        for node in nodes.iter() {
            match &*node.borrow() {
                Node::Element(element) => self.visit_element(element),
                Node::Text(text) => self.visit_text(text),
                Node::Interpolation(interpolation) => self.visit_interpolation(interpolation),
                Node::IfBlock(if_block) => self.visit_if_block(if_block),
            }
        }
    }

    fn visit_text(&self, text: &Text) {
        dbg!(text);
    }

    fn visit_interpolation(&self, interpolation: &Interpolation<'a>) {
        dbg!(&interpolation.expression);
    }

    fn visit_element(&self, element: &Element<'a>) {
        self.visit_attributes(&element.attributes);
        self.visit_nodes(&element.nodes);
    }

    fn visit_attributes(&self, attributes: &Vec<Attribute<'a>>) {
        for attr in attributes.iter() {
            match attr {
                Attribute::HTMLAttribute(attribute) => self.visit_html_attribute(attribute),
                Attribute::Expression(expression) => self.visit_expression_attribute(expression),
            }
        }
    }

    fn visit_html_attribute(&self, attribute: &HTMLAttribute<'a>) {
        dbg!(attribute);
    }

    fn visit_expression_attribute(&self, expr: &Expression<'a>) {
        dbg!(expr);
    }

    fn visit_if_block(&self, if_block: &IfBlock<'a>) {
        dbg!(&if_block.test);

        self.visit_nodes(&if_block.consequent);

        if let Some(nodes) = &if_block.alternate {
            self.visit_nodes(nodes);
        }
    }
}

#[cfg(test)]
mod tests {
    use oxc_allocator::Allocator;

    use crate::parser::Parser;

    use super::*;

    #[test]
    fn smoke() {
        let allocator = Allocator::default();
        let mut parser = Parser::new("prefix <div>text</div>", &allocator);
        let ast = parser.parse().unwrap();
        let visitor = Visitor::new(ast);

        visitor.walk();
    }
}
