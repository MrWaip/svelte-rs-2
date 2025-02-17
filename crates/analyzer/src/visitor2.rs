#![allow(
    unused_variables,
    clippy::extra_unused_type_parameters,
    clippy::explicit_iter_loop,
    clippy::self_named_module_files,
    clippy::semicolon_if_nothing_returned,
    clippy::match_wildcard_for_single_variants
)]

use oxc_ast::ast::Expression;

use ast::{
    Attribute, ClassDirective, Concatenation, ConcatenationPart, Element, ExpressionAttribute,
    ExpressionAttributeValue, Fragment, HTMLAttribute, IfBlock, Interpolation, Node, ScriptTag,
    Template, Text,
};

use crate::context::VisitorContext;

pub trait TemplateVisitor<'a>: Sized {
    fn enter_template(&mut self, it: &mut Template<'a>, ctx: &mut VisitorContext<'a>) {}

    fn exit_template(&mut self, it: &mut Template<'a>, ctx: &mut VisitorContext<'a>) {}

    fn enter_fragment(&mut self, it: &mut Fragment<'a>, ctx: &mut VisitorContext<'a>) {}

    fn exit_fragment(&mut self, it: &mut Fragment<'a>, ctx: &mut VisitorContext<'a>) {}

    fn enter_nodes(&mut self, it: &Vec<Node<'a>>, ctx: &mut VisitorContext<'a>) {}

    fn exit_nodes(&mut self, it: &Vec<Node<'a>>, ctx: &mut VisitorContext<'a>) {}

    fn enter_node(&mut self, it: &Node<'a>, ctx: &mut VisitorContext<'a>) {}

    fn exit_node(&mut self, it: &Node<'a>, ctx: &mut VisitorContext<'a>) {}

    fn enter_text(&mut self, it: &Text<'a>, ctx: &mut VisitorContext<'a>) {}

    fn exit_text(&mut self, it: &Text<'a>, ctx: &mut VisitorContext<'a>) {}

    fn enter_interpolation(&mut self, it: &mut Interpolation<'a>, ctx: &mut VisitorContext<'a>) {}

    fn exit_interpolation(&mut self, it: &mut Interpolation<'a>, ctx: &mut VisitorContext<'a>) {}

    fn enter_expression(&mut self, it: &Expression<'a>, ctx: &mut VisitorContext<'a>) {}

    fn exit_expression(&mut self, it: &Expression<'a>, ctx: &mut VisitorContext<'a>) {}

    fn enter_element(&mut self, it: &mut Element<'a>, ctx: &mut VisitorContext<'a>) {}

    fn exit_element(&mut self, it: &mut Element<'a>, ctx: &mut VisitorContext<'a>) {}

    fn enter_attributes(&mut self, it: &mut Vec<Attribute<'a>>, ctx: &mut VisitorContext<'a>) {}

    fn exit_attributes(&mut self, it: &mut Vec<Attribute<'a>>, ctx: &mut VisitorContext<'a>) {}

    fn enter_attribute(&mut self, it: &mut Attribute<'a>, ctx: &mut VisitorContext<'a>) {}

    fn exit_attribute(&mut self, it: &mut Attribute<'a>, ctx: &mut VisitorContext<'a>) {}

    fn enter_html_attribute(&mut self, it: &mut HTMLAttribute<'a>, ctx: &mut VisitorContext<'a>) {}

    fn exit_html_attribute(&mut self, it: &mut HTMLAttribute<'a>, ctx: &mut VisitorContext<'a>) {}

    fn enter_expression_attribute(
        &mut self,
        it: &mut ExpressionAttribute<'a>,
        ctx: &mut VisitorContext<'a>,
    ) {
    }

    fn exit_expression_attribute(
        &mut self,
        it: &mut ExpressionAttribute<'a>,
        ctx: &mut VisitorContext<'a>,
    ) {
    }

    fn enter_class_directive_attribute(
        &mut self,
        it: &mut ClassDirective<'a>,
        ctx: &mut VisitorContext<'a>,
    ) {
    }

    fn exit_class_directive_attribute(
        &mut self,
        it: &mut ClassDirective<'a>,
        ctx: &mut VisitorContext<'a>,
    ) {
    }

    fn enter_string_attribute_value(&mut self, it: &str, ctx: &mut VisitorContext<'a>) {}

    fn exit_string_attribute_value(&mut self, it: &str, ctx: &mut VisitorContext<'a>) {}

    fn enter_expression_attribute_value(
        &mut self,
        it: &mut ExpressionAttributeValue<'a>,
        ctx: &mut VisitorContext<'a>,
    ) {
    }

    fn exit_expression_attribute_value(
        &mut self,
        it: &mut ExpressionAttributeValue<'a>,
        ctx: &mut VisitorContext<'a>,
    ) {
    }

    fn enter_boolean_attribute_value(&mut self, ctx: &mut VisitorContext<'a>) {}

    fn exit_boolean_attribute_value(&mut self, ctx: &mut VisitorContext<'a>) {}

    fn enter_concatenation_attribute_value(
        &mut self,
        it: &mut Concatenation<'a>,
        ctx: &mut VisitorContext<'a>,
    ) {
    }

    fn exit_concatenation_attribute_value(
        &mut self,
        it: &mut Concatenation<'a>,
        ctx: &mut VisitorContext<'a>,
    ) {
    }

    fn enter_concatenation_part(
        &mut self,
        it: &ConcatenationPart<'a>,
        ctx: &mut VisitorContext<'a>,
    ) {
    }

    fn exit_concatenation_part(
        &mut self,
        it: &ConcatenationPart<'a>,
        ctx: &mut VisitorContext<'a>,
    ) {
    }

    fn enter_string_concatenation_part(&mut self, it: &str, ctx: &mut VisitorContext<'a>) {}

    fn exit_string_concatenation_part(&mut self, it: &str, ctx: &mut VisitorContext<'a>) {}

    fn enter_expression_concatenation_part(
        &mut self,
        it: &Expression<'a>,
        ctx: &mut VisitorContext<'a>,
    ) {
    }

    fn exit_expression_concatenation_part(
        &mut self,
        it: &Expression<'a>,
        ctx: &mut VisitorContext<'a>,
    ) {
    }

    fn enter_script_tag(&mut self, it: &ScriptTag<'a>, ctx: &mut VisitorContext<'a>) {}

    fn exit_script_tag(&mut self, it: &ScriptTag<'a>, ctx: &mut VisitorContext<'a>) {}

    fn enter_if_block(&mut self, it: &mut IfBlock<'a>, ctx: &mut VisitorContext<'a>) {}

    fn exit_if_block(&mut self, it: &mut IfBlock<'a>, ctx: &mut VisitorContext<'a>) {}
}

pub mod walk {

    use ast::Node;
    use rccell::RcCell;

    use crate::ancestor::Ancestor;

    use super::*;

    pub fn walk_template<'a, V: TemplateVisitor<'a>>(
        visitor: &mut V,
        it: &RcCell<Template<'a>>,
        ctx: &mut VisitorContext<'a>,
    ) {
        let template = &mut *it.borrow_mut();
        ctx.push_stack(Ancestor::Template(it.clone()));
        visitor.enter_template(template, ctx);

        walk_fragment(visitor, &mut template.nodes, ctx);

        visitor.exit_template(template, ctx);
        ctx.pop_stack();
    }

    pub fn walk_fragment<'a, V: TemplateVisitor<'a>>(
        visitor: &mut V,
        it: &mut Fragment<'a>,
        ctx: &mut VisitorContext<'a>,
    ) {
        visitor.enter_fragment(it, ctx);

        walk_nodes(visitor, &mut it.nodes, ctx);

        visitor.exit_fragment(it, ctx);
    }

    pub fn walk_nodes<'a, V: TemplateVisitor<'a>>(
        visitor: &mut V,
        it: &Vec<Node<'a>>,
        ctx: &mut VisitorContext<'a>,
    ) {
        visitor.enter_nodes(it, ctx);

        for node in it.iter() {
            walk_node(visitor, node, ctx);
        }

        visitor.exit_nodes(it, ctx);
    }

    pub fn walk_node<'a, V: TemplateVisitor<'a>>(
        visitor: &mut V,
        it: &Node<'a>,
        ctx: &mut VisitorContext<'a>,
    ) {
        visitor.enter_node(it, ctx);

        match it {
            Node::Text(it) => {
                let it = &*it.borrow_mut();
                visitor.enter_text(it, ctx);
                visitor.exit_text(it, ctx);
            }
            Node::ScriptTag(it) => {
                let it = &*it.borrow_mut();
                visitor.enter_script_tag(it, ctx);
                visitor.exit_script_tag(it, ctx);
            }
            Node::Element(it) => {
                ctx.push_stack(Ancestor::Element(it.clone()));
                let it = &mut *it.borrow_mut();
                walk_element(visitor, it, ctx);
                ctx.pop_stack();
            }
            Node::Interpolation(it) => walk_interpolation(visitor, &mut *it.borrow_mut(), ctx),
            Node::IfBlock(it) => {
                ctx.push_stack(Ancestor::IfBlock(it.clone()));
                let it = &mut *it.borrow_mut();
                walk_if_block(visitor, it, ctx);
                ctx.pop_stack();
            }
            Node::VirtualConcatenation(_) => unreachable!(),
        }

        visitor.exit_node(it, ctx);
    }

    pub fn walk_interpolation<'a, V: TemplateVisitor<'a>>(
        visitor: &mut V,
        it: &mut Interpolation<'a>,
        ctx: &mut VisitorContext<'a>,
    ) {
        visitor.enter_interpolation(it, ctx);

        walk_expression(visitor, &mut it.expression, ctx);

        visitor.exit_interpolation(it, ctx);
    }

    pub fn walk_expression<'a, V: TemplateVisitor<'a>>(
        visitor: &mut V,
        it: &mut Expression<'a>,
        ctx: &mut VisitorContext<'a>,
    ) {
        visitor.enter_expression(it, ctx);
        visitor.exit_expression(it, ctx);
    }

    pub fn walk_element<'a, V: TemplateVisitor<'a>>(
        visitor: &mut V,
        it: &mut Element<'a>,
        ctx: &mut VisitorContext<'a>,
    ) {
        visitor.enter_element(it, ctx);

        walk_attributes(visitor, &mut it.attributes, ctx);
        walk_nodes(visitor, &mut it.nodes, ctx);

        visitor.exit_element(it, ctx);
    }

    pub fn walk_attributes<'a, V: TemplateVisitor<'a>>(
        visitor: &mut V,
        it: &mut Vec<Attribute<'a>>,
        ctx: &mut VisitorContext<'a>,
    ) {
        visitor.enter_attributes(it, ctx);

        for attribute in it.iter_mut() {
            walk_attribute(visitor, attribute, ctx);
        }

        visitor.exit_attributes(it, ctx);
    }

    pub fn walk_attribute<'a, V: TemplateVisitor<'a>>(
        visitor: &mut V,
        it: &mut Attribute<'a>,
        ctx: &mut VisitorContext<'a>,
    ) {
        visitor.enter_attribute(it, ctx);

        match it {
            Attribute::HTMLAttribute(it) => walk_html_attribute(visitor, it, ctx),
            Attribute::Expression(it) => walk_expression_attribute(visitor, it, ctx),
            Attribute::ClassDirective(it) => walk_class_directive_attribute(visitor, it, ctx),
        }

        visitor.exit_attribute(it, ctx);
    }

    pub fn walk_expression_attribute<'a, V: TemplateVisitor<'a>>(
        visitor: &mut V,
        it: &mut ExpressionAttribute<'a>,
        ctx: &mut VisitorContext<'a>,
    ) {
        visitor.enter_expression_attribute(it, ctx);

        walk_expression(visitor, &mut it.expression, ctx);

        visitor.exit_expression_attribute(it, ctx);
    }

    pub fn walk_class_directive_attribute<'a, V: TemplateVisitor<'a>>(
        visitor: &mut V,
        it: &mut ClassDirective<'a>,
        ctx: &mut VisitorContext<'a>,
    ) {
        visitor.enter_class_directive_attribute(it, ctx);

        walk_expression(visitor, &mut it.expression, ctx);

        visitor.exit_class_directive_attribute(it, ctx);
    }

    pub fn walk_html_attribute<'a, V: TemplateVisitor<'a>>(
        visitor: &mut V,
        it: &mut HTMLAttribute<'a>,
        ctx: &mut VisitorContext<'a>,
    ) {
        visitor.enter_html_attribute(it, ctx);

        match &mut it.value {
            ast::AttributeValue::String(it) => {
                visitor.enter_string_attribute_value(*it, ctx);
                visitor.exit_string_attribute_value(*it, ctx);
            }
            ast::AttributeValue::Expression(it) => {
                walk_expression_attribute_value(visitor, it, ctx);
            }
            ast::AttributeValue::Boolean => {
                visitor.enter_boolean_attribute_value(ctx);
                visitor.exit_boolean_attribute_value(ctx);
            }
            ast::AttributeValue::Concatenation(it) => {
                walk_concatenation_attribute_value(visitor, it, ctx);
            }
        }

        visitor.exit_html_attribute(it, ctx);
    }

    pub fn walk_expression_attribute_value<'a, V: TemplateVisitor<'a>>(
        visitor: &mut V,
        it: &mut ExpressionAttributeValue<'a>,
        ctx: &mut VisitorContext<'a>,
    ) {
        visitor.enter_expression_attribute_value(it, ctx);
        walk_expression(visitor, &mut it.expression, ctx);
        visitor.exit_expression_attribute_value(it, ctx);
    }

    pub fn walk_concatenation_attribute_value<'a, V: TemplateVisitor<'a>>(
        visitor: &mut V,
        it: &mut Concatenation<'a>,
        ctx: &mut VisitorContext<'a>,
    ) {
        visitor.enter_concatenation_attribute_value(it, ctx);

        for part in it.parts.iter_mut() {
            walk_concatenation_part(visitor, part, ctx);
        }

        visitor.exit_concatenation_attribute_value(it, ctx);
    }

    pub fn walk_concatenation_part<'a, V: TemplateVisitor<'a>>(
        visitor: &mut V,
        it: &mut ConcatenationPart<'a>,
        ctx: &mut VisitorContext<'a>,
    ) {
        visitor.enter_concatenation_part(it, ctx);

        match it {
            ConcatenationPart::String(it) => {
                visitor.enter_string_concatenation_part(it, ctx);
                visitor.exit_string_concatenation_part(it, ctx);
            }
            ConcatenationPart::Expression(it) => {
                walk_expression_concatenation_part(visitor, it, ctx);
            }
        }

        visitor.exit_concatenation_part(it, ctx);
    }

    pub fn walk_expression_concatenation_part<'a, V: TemplateVisitor<'a>>(
        visitor: &mut V,
        it: &mut Expression<'a>,
        ctx: &mut VisitorContext<'a>,
    ) {
        visitor.enter_expression_concatenation_part(it, ctx);
        walk_expression(visitor, it, ctx);
        visitor.exit_expression_concatenation_part(it, ctx);
    }

    pub fn walk_if_block<'a, V: TemplateVisitor<'a>>(
        visitor: &mut V,
        it: &mut IfBlock<'a>,
        ctx: &mut VisitorContext<'a>,
    ) {
        visitor.enter_if_block(it, ctx);

        walk_expression(visitor, &mut it.test, ctx);

        walk_fragment(visitor, &mut it.consequent, ctx);

        if let Some(alternate) = &mut it.alternate {
            walk_fragment(visitor, alternate, ctx);
        }

        visitor.exit_if_block(it, ctx);
    }
}
