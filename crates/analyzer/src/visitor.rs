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
    Attribute, ClassDirective, Concatenation, ConcatenationPart, Element, ExpressionAttribute,
    ExpressionAttributeValue, Fragment, HTMLAttribute, IfBlock, Interpolation, Node, ScriptTag,
    Template, Text,
};

use crate::context::VisitorContext;

pub trait TemplateVisitor<'a>: Sized {
    fn visit_template(&mut self, it: &mut Template<'a>, ctx: &mut VisitorContext) {
        walk_template(self, it, ctx);
    }

    fn visit_fragment(&mut self, it: &mut Fragment<'a>, ctx: &mut VisitorContext) {
        walk_fragment(self, it, ctx);
    }

    fn visit_nodes(&mut self, it: &Vec<RcCell<Node<'a>>>, ctx: &mut VisitorContext) {
        walk_nodes(self, it, ctx);
    }

    fn visit_node(&mut self, it: &mut Node<'a>, ctx: &mut VisitorContext) {
        walk_node(self, it, ctx);
    }

    fn visit_text(&mut self, it: &Text<'a>, ctx: &mut VisitorContext) {}

    fn visit_interpolation(&mut self, it: &mut Interpolation<'a>, ctx: &mut VisitorContext) {
        walk_interpolation(self, it, ctx);
    }

    fn visit_expression(&mut self, it: &Expression<'a>, ctx: &mut VisitorContext) {}

    fn visit_element(&mut self, it: &mut Element<'a>, ctx: &mut VisitorContext) {
        walk_element(self, it, ctx);
    }

    fn visit_attributes(&mut self, it: &mut Vec<Attribute<'a>>, ctx: &mut VisitorContext) {
        walk_attributes(self, it, ctx);
    }

    fn visit_attribute(&mut self, it: &mut Attribute<'a>, ctx: &mut VisitorContext) {
        walk_attribute(self, it, ctx);
    }

    fn visit_html_attribute(&mut self, it: &mut HTMLAttribute<'a>, ctx: &mut VisitorContext) {
        walk_html_attribute(self, it, ctx);
    }

    fn visit_expression_attribute(
        &mut self,
        it: &mut ExpressionAttribute<'a>,
        ctx: &mut VisitorContext,
    ) {
        walk_expression_attribute(self, it, ctx);
    }

    fn visit_class_directive_attribute(
        &mut self,
        it: &mut ClassDirective<'a>,
        ctx: &mut VisitorContext,
    ) {
        walk_class_directive_attribute(self, it, ctx);
    }

    fn visit_string_attribute_value(&mut self, it: &str, ctx: &mut VisitorContext) {}

    fn visit_expression_attribute_value(
        &mut self,
        it: &mut ExpressionAttributeValue<'a>,
        ctx: &mut VisitorContext,
    ) {
        walk_expression_attribute_value(self, it, ctx);
    }

    fn visit_boolean_attribute_value(&mut self, ctx: &mut VisitorContext) {}

    fn visit_concatenation_attribute_value(
        &mut self,
        it: &mut Concatenation<'a>,
        ctx: &mut VisitorContext,
    ) {
        walk_concatenation_attribute_value(self, it, ctx);
    }

    fn visit_concatenation_part(&mut self, it: &ConcatenationPart<'a>, ctx: &mut VisitorContext) {
        walk_concatenation_part(self, it, ctx);
    }

    fn visit_string_concatenation_part(&mut self, it: &str, ctx: &mut VisitorContext) {}

    fn visit_expression_concatenation_part(
        &mut self,
        it: &Expression<'a>,
        ctx: &mut VisitorContext,
    ) {
        walk_expression_concatenation_part(self, it, ctx);
    }

    fn visit_script_tag(&mut self, it: &ScriptTag<'a>, ctx: &mut VisitorContext) {}

    fn visit_if_block(&mut self, it: &mut IfBlock<'a>, ctx: &mut VisitorContext) {
        walk_if_block(self, it, ctx);
    }
}

pub mod walk {
    use rccell::RcCell;

    use ast::Node;

    use super::*;

    pub fn walk_template<'a, V: TemplateVisitor<'a>>(
        visitor: &mut V,
        it: &mut Template<'a>,
        ctx: &mut VisitorContext,
    ) {
        visitor.visit_fragment(&mut it.nodes, ctx);
    }

    pub fn walk_fragment<'a, V: TemplateVisitor<'a>>(
        visitor: &mut V,
        it: &mut Fragment<'a>,
        ctx: &mut VisitorContext,
    ) {
        visitor.visit_nodes(&it.nodes, ctx);
    }

    pub fn walk_nodes<'a, V: TemplateVisitor<'a>>(
        visitor: &mut V,
        it: &Vec<RcCell<Node<'a>>>,
        ctx: &mut VisitorContext,
    ) {
        for cell in it.iter() {
            let mut node = cell.borrow_mut();

            visitor.visit_node(&mut *node, ctx);
        }
    }

    pub fn walk_node<'a, V: TemplateVisitor<'a>>(
        visitor: &mut V,
        it: &mut Node<'a>,
        ctx: &mut VisitorContext,
    ) {
        match it {
            Node::Element(it) => visitor.visit_element(it, ctx),
            Node::Text(it) => visitor.visit_text(it, ctx),
            Node::Interpolation(it) => visitor.visit_interpolation(it, ctx),
            Node::IfBlock(it) => visitor.visit_if_block(it, ctx),
            Node::VirtualConcatenation(_) => unreachable!(),
            Node::ScriptTag(it) => visitor.visit_script_tag(it, ctx),
        }
    }

    pub fn walk_interpolation<'a, V: TemplateVisitor<'a>>(
        visitor: &mut V,
        it: &mut Interpolation<'a>,
        ctx: &mut VisitorContext,
    ) {
        visitor.visit_expression(&it.expression, ctx);
    }

    pub fn walk_element<'a, V: TemplateVisitor<'a>>(
        visitor: &mut V,
        it: &mut Element<'a>,
        ctx: &mut VisitorContext,
    ) {
        visitor.visit_attributes(&mut it.attributes, ctx);
        visitor.visit_nodes(&mut it.nodes, ctx);
    }

    pub fn walk_attributes<'a, V: TemplateVisitor<'a>>(
        visitor: &mut V,
        it: &mut Vec<Attribute<'a>>,
        ctx: &mut VisitorContext,
    ) {
        for attribute in it.iter_mut() {
            visitor.visit_attribute(attribute, ctx);
        }
    }

    pub fn walk_attribute<'a, V: TemplateVisitor<'a>>(
        visitor: &mut V,
        it: &mut Attribute<'a>,
        ctx: &mut VisitorContext,
    ) {
        match it {
            Attribute::HTMLAttribute(it) => visitor.visit_html_attribute(it, ctx),
            Attribute::Expression(it) => visitor.visit_expression_attribute(it, ctx),
            Attribute::ClassDirective(it) => visitor.visit_class_directive_attribute(it, ctx),
        }
    }

    pub fn walk_expression_attribute<'a, V: TemplateVisitor<'a>>(
        visitor: &mut V,
        it: &ExpressionAttribute<'a>,
        ctx: &mut VisitorContext,
    ) {
        visitor.visit_expression(&it.expression, ctx);
    }

    pub fn walk_class_directive_attribute<'a, V: TemplateVisitor<'a>>(
        visitor: &mut V,
        it: &ClassDirective<'a>,
        ctx: &mut VisitorContext,
    ) {
        visitor.visit_expression(&it.expression, ctx);
    }

    pub fn walk_html_attribute<'a, V: TemplateVisitor<'a>>(
        visitor: &mut V,
        it: &mut HTMLAttribute<'a>,
        ctx: &mut VisitorContext,
    ) {
        match &mut it.value {
            ast::AttributeValue::String(it) => visitor.visit_string_attribute_value(*it, ctx),
            ast::AttributeValue::Expression(it) => {
                visitor.visit_expression_attribute_value(it, ctx)
            }
            ast::AttributeValue::Boolean => visitor.visit_boolean_attribute_value(ctx),
            ast::AttributeValue::Concatenation(it) => {
                visitor.visit_concatenation_attribute_value(it, ctx)
            }
        }
    }

    pub fn walk_expression_attribute_value<'a, V: TemplateVisitor<'a>>(
        visitor: &mut V,
        it: &ExpressionAttributeValue<'a>,
        ctx: &mut VisitorContext,
    ) {
        visitor.visit_expression(&it.expression, ctx);
    }

    pub fn walk_concatenation_attribute_value<'a, V: TemplateVisitor<'a>>(
        visitor: &mut V,
        it: &Concatenation<'a>,
        ctx: &mut VisitorContext,
    ) {
        for part in it.parts.iter() {
            visitor.visit_concatenation_part(part, ctx);
        }
    }

    pub fn walk_concatenation_part<'a, V: TemplateVisitor<'a>>(
        visitor: &mut V,
        it: &ConcatenationPart<'a>,
        ctx: &mut VisitorContext,
    ) {
        match it {
            ConcatenationPart::String(it) => visitor.visit_string_concatenation_part(*it, ctx),
            ConcatenationPart::Expression(it) => {
                visitor.visit_expression_concatenation_part(it, ctx)
            }
        }
    }

    pub fn walk_expression_concatenation_part<'a, V: TemplateVisitor<'a>>(
        visitor: &mut V,
        it: &Expression<'a>,
        ctx: &mut VisitorContext,
    ) {
        visitor.visit_expression(it, ctx);
    }

    pub fn walk_if_block<'a, V: TemplateVisitor<'a>>(
        visitor: &mut V,
        it: &mut IfBlock<'a>,
        ctx: &mut VisitorContext,
    ) {
        visitor.visit_expression(&it.test, ctx);

        visitor.visit_fragment(&mut it.consequent, ctx);

        if let Some(alternate) = &mut it.alternate {
            visitor.visit_fragment(alternate, ctx);
        }
    }
}
