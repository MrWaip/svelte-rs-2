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
    Attribute, BindDirective, BooleanAttribute, ClassDirective, ConcatenationAttribute,
    ConcatenationPart, Element, ExpressionAttribute, Fragment, IfBlock, Interpolation, Node,
    ScriptTag, StringAttribute, Template, Text,
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

    fn enter_bind_directive_attribute(
        &mut self,
        it: &mut BindDirective<'a>,
        ctx: &mut VisitorContext<'a>,
    ) {
    }

    fn exit_bind_directive_attribute(
        &mut self,
        it: &mut BindDirective<'a>,
        ctx: &mut VisitorContext<'a>,
    ) {
    }

    fn enter_string_attribute(
        &mut self,
        it: &mut StringAttribute<'a>,
        ctx: &mut VisitorContext<'a>,
    ) {
    }

    fn exit_string_attribute(
        &mut self,
        it: &mut StringAttribute<'a>,
        ctx: &mut VisitorContext<'a>,
    ) {
    }

    fn enter_boolean_attribute(
        &mut self,
        it: &mut BooleanAttribute<'a>,
        ctx: &mut VisitorContext<'a>,
    ) {
    }

    fn exit_boolean_attribute(
        &mut self,
        it: &mut BooleanAttribute<'a>,
        ctx: &mut VisitorContext<'a>,
    ) {
    }

    fn enter_concatenation_attribute(
        &mut self,
        it: &mut ConcatenationAttribute<'a>,
        ctx: &mut VisitorContext<'a>,
    ) {
    }

    fn exit_concatenation_attribute(
        &mut self,
        it: &mut ConcatenationAttribute<'a>,
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

    use ast::{BindDirective, Node};
    
    use rccell::RcCell;

    use crate::ancestor::Ancestor;

    use super::*;

    pub fn walk_template<'a, V: TemplateVisitor<'a>>(
        visitor: &mut V,
        it: &RcCell<Template<'a>>,
        ctx: &mut VisitorContext<'a>,
    ) {
        let template = &mut *it.borrow_mut();
        let node_id = ctx.next_node_id();

        template.nodes.set_node_id(node_id);

        visitor.enter_template(template, ctx);

        ctx.push_stack(Ancestor::Template(node_id));
        walk_fragment(visitor, &mut template.nodes, ctx);
        ctx.pop_stack();

        visitor.exit_template(template, ctx);
    }

    pub fn walk_fragment<'a, V: TemplateVisitor<'a>>(
        visitor: &mut V,
        it: &mut Fragment<'a>,
        ctx: &mut VisitorContext<'a>,
    ) {
        visitor.enter_fragment(it, ctx);

        walk_nodes(visitor, &it.nodes, ctx);

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
                walk_element(visitor, it, ctx);
            }
            Node::Interpolation(it) => walk_interpolation(visitor, &mut *it.borrow_mut(), ctx),
            Node::IfBlock(it) => {
                walk_if_block(visitor, it, ctx);
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
        cell: &RcCell<Element<'a>>,
        ctx: &mut VisitorContext<'a>,
    ) {
        let node_id = ctx.next_node_id();
        let it = &mut *cell.borrow_mut();

        it.set_node_id(node_id);
        ctx.add_default_element_flags(node_id);

        visitor.enter_element(it, ctx);

        ctx.push_stack(Ancestor::Element(node_id));
        walk_attributes(visitor, &mut it.attributes, ctx);
        walk_nodes(visitor, &it.nodes, ctx);
        ctx.pop_stack();

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
            Attribute::ExpressionAttribute(it) => walk_expression_attribute(visitor, it, ctx),
            Attribute::ClassDirective(it) => walk_class_directive_attribute(visitor, it, ctx),
            Attribute::BindDirective(it) => walk_bind_directive_attribute(visitor, it, ctx),
            Attribute::BooleanAttribute(it) => walk_boolean_attribute(visitor, it, ctx),
            Attribute::StringAttribute(it) => walk_string_attribute(visitor, it, ctx),
            Attribute::ConcatenationAttribute(it) => walk_concatenation_attribute(visitor, it, ctx),
        }

        visitor.exit_attribute(it, ctx);
    }

    pub fn walk_string_attribute<'a, V: TemplateVisitor<'a>>(
        visitor: &mut V,
        it: &mut StringAttribute<'a>,
        ctx: &mut VisitorContext<'a>,
    ) {
        visitor.enter_string_attribute(it, ctx);
        visitor.exit_string_attribute(it, ctx);
    }

    pub fn walk_boolean_attribute<'a, V: TemplateVisitor<'a>>(
        visitor: &mut V,
        it: &mut BooleanAttribute<'a>,
        ctx: &mut VisitorContext<'a>,
    ) {
        visitor.enter_boolean_attribute(it, ctx);
        visitor.exit_boolean_attribute(it, ctx);
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

    pub fn walk_bind_directive_attribute<'a, V: TemplateVisitor<'a>>(
        visitor: &mut V,
        it: &mut BindDirective<'a>,
        ctx: &mut VisitorContext<'a>,
    ) {
        visitor.enter_bind_directive_attribute(it, ctx);

        walk_expression(visitor, &mut it.expression, ctx);

        visitor.exit_bind_directive_attribute(it, ctx);
    }

    pub fn walk_concatenation_attribute<'a, V: TemplateVisitor<'a>>(
        visitor: &mut V,
        it: &mut ConcatenationAttribute<'a>,
        ctx: &mut VisitorContext<'a>,
    ) {
        visitor.enter_concatenation_attribute(it, ctx);

        for part in it.parts.iter_mut() {
            walk_concatenation_part(visitor, part, ctx);
        }

        visitor.exit_concatenation_attribute(it, ctx);
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
        cell: &RcCell<IfBlock<'a>>,
        ctx: &mut VisitorContext<'a>,
    ) {
        let it = &mut *cell.borrow_mut();
        let consequent_node_id = ctx.next_node_id();

        it.consequent.set_node_id(consequent_node_id);

        visitor.enter_if_block(it, ctx);
        ctx.push_stack(Ancestor::IfBlock(consequent_node_id));

        walk_expression(visitor, &mut it.test, ctx);

        walk_fragment(visitor, &mut it.consequent, ctx);

        if let Some(alternate) = &mut it.alternate {
            let alternate_node_id = ctx.next_node_id();
            ctx.pop_stack();
            ctx.push_stack(Ancestor::IfBlock(alternate_node_id));
            alternate.set_node_id(alternate_node_id);

            walk_fragment(visitor, alternate, ctx);
        }

        ctx.pop_stack();
        visitor.exit_if_block(it, ctx);
    }
}
