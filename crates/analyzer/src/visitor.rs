#![allow(
    unused_variables,
    clippy::extra_unused_type_parameters,
    clippy::explicit_iter_loop,
    clippy::self_named_module_files,
    clippy::semicolon_if_nothing_returned,
    clippy::match_wildcard_for_single_variants
)]

use oxc_ast::ast::Expression;
use rccell::RcCell;
use walk::*;

use ast::{
    Attribute, ClassDirective, Concatenation, ConcatenationPart, Element, ExpressionAttributeValue,
    HTMLAttribute, IfBlock, Interpolation, Node, ScriptTag, Text,
};

pub trait TemplateVisitor<'a>: Sized {
    fn visit_template(&mut self, it: &Vec<RcCell<Node<'a>>>) {
        walk_template(self, it);
    }

    fn visit_nodes(&mut self, it: &Vec<RcCell<Node<'a>>>) {
        walk_nodes(self, it);
    }

    fn visit_node(&mut self, it: &mut Node<'a>) {
        walk_node(self, it);
    }

    fn visit_text(&mut self, it: &Text<'a>) {}

    fn visit_interpolation(&mut self, it: &mut Interpolation<'a>) {
        walk_interpolation(self, it);
    }

    fn visit_expression(&mut self, it: &Expression<'a>) {}

    fn visit_element(&mut self, it: &mut Element<'a>) {
        walk_element(self, it);
    }

    fn visit_attributes(&mut self, it: &mut Vec<Attribute<'a>>) {
        walk_attributes(self, it);
    }

    fn visit_attribute(&mut self, it: &mut Attribute<'a>) {
        walk_attribute(self, it);
    }

    fn visit_html_attribute(&mut self, it: &mut HTMLAttribute<'a>) {
        walk_html_attribute(self, it);
    }

    fn visit_expression_attribute(&mut self, it: &Expression<'a>) {
        walk_expression_attribute(self, it);
    }

    fn visit_class_directive_attribute(&mut self, it: &mut ClassDirective<'a>) {
        walk_class_directive_attribute(self, it);
    }

    fn visit_string_attribute_value(&mut self, it: &str) {}

    fn visit_expression_attribute_value(&mut self, it: &mut ExpressionAttributeValue<'a>) {
        walk_expression_attribute_value(self, it);
    }

    fn visit_boolean_attribute_value(&mut self) {}

    fn visit_concatenation_attribute_value(&mut self, it: &mut Concatenation<'a>) {
        walk_concatenation_attribute_value(self, it);
    }

    fn visit_concatenation_part(&mut self, it: &ConcatenationPart<'a>) {
        walk_concatenation_part(self, it);
    }

    fn visit_string_concatenation_part(&mut self, it: &str) {}

    fn visit_expression_concatenation_part(&mut self, it: &Expression<'a>) {
        walk_expression_concatenation_part(self, it);
    }

    fn visit_script_tag(&mut self, it: &ScriptTag<'a>) {}

    fn visit_if_block(&mut self, it: &mut IfBlock<'a>) {
        walk_if_block(self, it);
    }
}

pub mod walk {
    use rccell::RcCell;

    use ast::Node;

    use super::*;

    pub fn walk_template<'a, V: TemplateVisitor<'a>>(visitor: &mut V, it: &Vec<RcCell<Node<'a>>>) {
        visitor.visit_nodes(it);
    }

    pub fn walk_nodes<'a, V: TemplateVisitor<'a>>(visitor: &mut V, it: &Vec<RcCell<Node<'a>>>) {
        for cell in it.iter() {
            let mut node = cell.borrow_mut();

            visitor.visit_node(&mut *node);
        }
    }

    pub fn walk_node<'a, V: TemplateVisitor<'a>>(visitor: &mut V, it: &mut Node<'a>) {
        match it {
            Node::Element(it) => visitor.visit_element(it),
            Node::Text(it) => visitor.visit_text(it),
            Node::Interpolation(it) => visitor.visit_interpolation(it),
            Node::IfBlock(it) => visitor.visit_if_block(it),
            Node::VirtualConcatenation(_) => unreachable!(),
            Node::ScriptTag(it) => visitor.visit_script_tag(it),
        }
    }

    pub fn walk_interpolation<'a, V: TemplateVisitor<'a>>(
        visitor: &mut V,
        it: &mut Interpolation<'a>,
    ) {
        visitor.visit_expression(&it.expression);
    }

    pub fn walk_element<'a, V: TemplateVisitor<'a>>(visitor: &mut V, it: &mut Element<'a>) {
        visitor.visit_attributes(&mut it.attributes);
        visitor.visit_nodes(&mut it.nodes);
    }

    pub fn walk_attributes<'a, V: TemplateVisitor<'a>>(
        visitor: &mut V,
        it: &mut Vec<Attribute<'a>>,
    ) {
        for attribute in it.iter_mut() {
            visitor.visit_attribute(attribute);
        }
    }

    pub fn walk_attribute<'a, V: TemplateVisitor<'a>>(visitor: &mut V, it: &mut Attribute<'a>) {
        match it {
            Attribute::HTMLAttribute(it) => visitor.visit_html_attribute(it),
            Attribute::Expression(it) => visitor.visit_expression_attribute(it),
            Attribute::ClassDirective(it) => visitor.visit_class_directive_attribute(it),
        }
    }

    pub fn walk_expression_attribute<'a, V: TemplateVisitor<'a>>(
        visitor: &mut V,
        it: &Expression<'a>,
    ) {
        visitor.visit_expression(it);
    }

    pub fn walk_class_directive_attribute<'a, V: TemplateVisitor<'a>>(
        visitor: &mut V,
        it: &ClassDirective<'a>,
    ) {
        visitor.visit_expression(&it.expression);
    }

    pub fn walk_html_attribute<'a, V: TemplateVisitor<'a>>(
        visitor: &mut V,
        it: &mut HTMLAttribute<'a>,
    ) {
        match &mut it.value {
            ast::AttributeValue::String(it) => visitor.visit_string_attribute_value(*it),
            ast::AttributeValue::Expression(it) => visitor.visit_expression_attribute_value(it),
            ast::AttributeValue::Boolean => visitor.visit_boolean_attribute_value(),
            ast::AttributeValue::Concatenation(it) => {
                visitor.visit_concatenation_attribute_value(it)
            }
        }
    }

    pub fn walk_expression_attribute_value<'a, V: TemplateVisitor<'a>>(
        visitor: &mut V,
        it: &ExpressionAttributeValue<'a>,
    ) {
        visitor.visit_expression(&it.expression);
    }

    pub fn walk_concatenation_attribute_value<'a, V: TemplateVisitor<'a>>(
        visitor: &mut V,
        it: &Concatenation<'a>,
    ) {
        for part in it.parts.iter() {
            visitor.visit_concatenation_part(part);
        }
    }

    pub fn walk_concatenation_part<'a, V: TemplateVisitor<'a>>(
        visitor: &mut V,
        it: &ConcatenationPart<'a>,
    ) {
        match it {
            ConcatenationPart::String(it) => visitor.visit_string_concatenation_part(*it),
            ConcatenationPart::Expression(it) => visitor.visit_expression_concatenation_part(it),
        }
    }

    pub fn walk_expression_concatenation_part<'a, V: TemplateVisitor<'a>>(
        visitor: &mut V,
        it: &Expression<'a>,
    ) {
        visitor.visit_expression(it);
    }

    pub fn walk_if_block<'a, V: TemplateVisitor<'a>>(visitor: &mut V, it: &mut IfBlock<'a>) {
        visitor.visit_expression(&it.test);

        visitor.visit_nodes(&it.consequent);

        if let Some(alternate) = &it.alternate {
            visitor.visit_nodes(alternate);
        }
    }
}
