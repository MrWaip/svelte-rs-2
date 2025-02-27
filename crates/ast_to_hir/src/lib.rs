mod compress_nodes;
pub mod context;
mod trim_nodes;

use hir::{AttributeId, ExpressionId, NodeId, OwnerId, OwnerNode};
use oxc_allocator::Allocator;
use oxc_ast::ast::Expression;
use oxc_index::IndexVec;

use crate::context::ToHirContext;

pub struct AstToHir {}

#[derive(Debug)]
pub struct AstToHirRet<'hir> {
    pub nodes: IndexVec<NodeId, hir::Node<'hir>>,
    pub owners: IndexVec<OwnerId, hir::OwnerNode<'hir>>,
    pub expressions: IndexVec<ExpressionId, Expression<'hir>>,
    pub attributes: IndexVec<AttributeId, hir::Attribute<'hir>>,
}

impl AstToHir {
    pub fn new() -> Self {
        Self {}
    }

    pub fn traverse<'hir>(
        &mut self,
        ast: ast::Ast<'hir>,
        allocator: &'hir Allocator,
    ) -> AstToHirRet<'hir> {
        let mut ctx = ToHirContext::new(allocator);

        self.lower_template(&mut ctx, ast.template.unwrap());

        return AstToHirRet {
            nodes: ctx.nodes,
            owners: ctx.owners,
            attributes: ctx.attributes,
            expressions: ctx.expressions,
        };
    }

    fn lower_template<'hir>(
        &mut self,
        ctx: &mut ToHirContext<'hir>,
        template: ast::Template<'hir>,
    ) {
        ctx.push_root_owner(|ctx| {
            let node_ids: Vec<NodeId> = self.lower_nodes(ctx, template.nodes.nodes);

            let template = hir::Template { node_ids };

            return OwnerNode::Template(ctx.alloc(template));
        });
    }

    fn lower_nodes<'hir>(
        &self,
        ctx: &mut ToHirContext<'hir>,
        nodes: Vec<ast::Node<'hir>>,
    ) -> Vec<NodeId> {
        let trimmed = self.trim_text_nodes(nodes, ctx);

        return self.compress_and_lower_nodes(trimmed, ctx);
    }

    fn lower_node<'hir>(&self, node: ast::Node<'hir>, ctx: &mut ToHirContext<'hir>) -> NodeId {
        return match node {
            ast::Node::Element(cell) => self.lower_element(cell.unwrap(), ctx),
            ast::Node::Text(cell) => self.lower_text(cell.unwrap(), ctx),
            ast::Node::Interpolation(cell) => self.lower_interpolation(cell.unwrap(), ctx),
            ast::Node::IfBlock(cell) => self.lower_if_block(cell.unwrap(), ctx),
            ast::Node::VirtualConcatenation(_) => unreachable!(),
            ast::Node::ScriptTag(_) => todo!(),
        };
    }

    fn lower_if_block<'hir>(
        &self,
        if_block: ast::IfBlock<'hir>,
        ctx: &mut ToHirContext<'hir>,
    ) -> NodeId {
        return ctx.push_owner_node(|ctx, self_node_id, owner_id| {
            let expression_id = ctx.push_expression(if_block.test);

            let hir_if_block = hir::IfBlock {
                node_id: self_node_id,
                owner_id,
                test: expression_id,
                consequent: self.lower_nodes(ctx, if_block.consequent.nodes),
                is_elseif: if_block.is_elseif,
                alternate: if_block.alternate.map(|fragment| {
                    return self.lower_nodes(ctx, fragment.nodes);
                }),
            };

            let hir_if_block = ctx.alloc(hir_if_block);

            return (
                hir::Node::IfBlock(hir_if_block),
                hir::OwnerNode::IfBlock(hir_if_block),
            );
        });
    }

    fn lower_element<'hir>(
        &self,
        element: ast::Element<'hir>,
        ctx: &mut ToHirContext<'hir>,
    ) -> NodeId {
        return ctx.push_owner_node(|ctx, self_node_id, owner_id| {
            let hir_element = hir::Element {
                node_id: self_node_id,
                owner_id,
                name: ctx.alloc(element.name),
                node_ids: self.lower_nodes(ctx, element.nodes),
                attributes: self.lower_attributes(ctx, element.attributes),
            };

            let hir_element = ctx.alloc(hir_element);

            return (
                hir::Node::Element(hir_element),
                hir::OwnerNode::Element(hir_element),
            );
        });
    }

    fn lower_attributes<'hir>(
        &self,
        ctx: &mut ToHirContext<'hir>,
        attributes: Vec<ast::Attribute<'hir>>,
    ) -> Vec<hir::AttributeId> {
        attributes
            .into_iter()
            .map(|attr| self.lower_attribute(ctx, attr))
            .collect()
    }

    fn lower_attribute<'hir>(
        &self,
        ctx: &mut ToHirContext<'hir>,
        attribute: ast::Attribute<'hir>,
    ) -> hir::AttributeId {
        return match attribute {
            ast::Attribute::ExpressionAttribute(attr) => self.lower_expression_attribute(ctx, attr),
            ast::Attribute::ClassDirective(attr) => self.lower_class_directive(ctx, attr),
            ast::Attribute::BindDirective(attr) => self.lower_bind_directive(ctx, attr),
            ast::Attribute::BooleanAttribute(attr) => self.lower_boolean_attribute(ctx, attr),
            ast::Attribute::StringAttribute(attr) => self.lower_string_attribute(ctx, attr),
            ast::Attribute::ConcatenationAttribute(attr) => {
                self.lower_concatenation_attribute(ctx, attr)
            }
        };
    }

    fn lower_class_directive<'hir>(
        &self,
        ctx: &mut ToHirContext<'hir>,
        attr: ast::ClassDirective<'hir>,
    ) -> hir::AttributeId {
        let expression_id = ctx.push_expression(attr.expression);

        let attribute = hir::ClassDirective {
            name: attr.name,
            shorthand: attr.shorthand,
            expression_id,
        };

        return ctx.push_attribute(hir::Attribute::ClassDirective(ctx.alloc(attribute)));
    }

    fn lower_bind_directive<'hir>(
        &self,
        ctx: &mut ToHirContext<'hir>,
        attr: ast::BindDirective<'hir>,
    ) -> hir::AttributeId {
        let expression_id = ctx.push_expression(attr.expression);
        let attribute = hir::BindDirective {
            expression_id,
            name: attr.name,
            shorthand: attr.shorthand,
        };

        return ctx.push_attribute(hir::Attribute::BindDirective(ctx.alloc(attribute)));
    }

    fn lower_boolean_attribute<'hir>(
        &self,
        ctx: &mut ToHirContext<'hir>,
        attr: ast::BooleanAttribute<'hir>,
    ) -> hir::AttributeId {
        let attribute = hir::BooleanAttribute { name: attr.name };

        return ctx.push_attribute(hir::Attribute::BooleanAttribute(ctx.alloc(attribute)));
    }

    fn lower_concatenation_attribute<'hir>(
        &self,
        ctx: &mut ToHirContext<'hir>,
        attr: ast::ConcatenationAttribute<'hir>,
    ) -> hir::AttributeId {
        let parts: Vec<hir::ConcatenationAttributePart<'hir>> = attr
            .parts
            .into_iter()
            .map(|part| match part {
                ast::ConcatenationPart::String(value) => {
                    hir::ConcatenationAttributePart::String(value)
                }
                ast::ConcatenationPart::Expression(expression) => {
                    let expression_id = ctx.push_expression(expression);

                    hir::ConcatenationAttributePart::Expression(expression_id)
                }
            })
            .collect();

        let attribute = hir::ConcatenationAttribute {
            name: attr.name,
            parts,
        };

        return ctx.push_attribute(hir::Attribute::ConcatenationAttribute(ctx.alloc(attribute)));
    }

    fn lower_expression_attribute<'hir>(
        &self,
        ctx: &mut ToHirContext<'hir>,
        attr: ast::ExpressionAttribute<'hir>,
    ) -> hir::AttributeId {
        let expression_id = ctx.push_expression(attr.expression);

        let attribute = hir::ExpressionAttribute {
            name: attr.name,
            shorthand: attr.shorthand,
            expression_id,
        };

        return ctx.push_attribute(hir::Attribute::ExpressionAttribute(ctx.alloc(attribute)));
    }

    fn lower_string_attribute<'hir>(
        &self,
        ctx: &mut ToHirContext<'hir>,
        attr: ast::StringAttribute<'hir>,
    ) -> hir::AttributeId {
        let attribute = hir::StringAttribute {
            name: attr.name,
            value: attr.value,
        };

        return ctx.push_attribute(hir::Attribute::StringAttribute(ctx.alloc(attribute)));
    }

    fn lower_interpolation<'hir>(
        &self,
        interpolation: ast::Interpolation<'hir>,
        ctx: &mut ToHirContext<'hir>,
    ) -> NodeId {
        return ctx.push_node(|ctx, node_id, owner_id| {
            let expression_id = ctx.push_expression(interpolation.expression);

            let hir_interpolation = hir::Interpolation {
                node_id,
                owner_id,
                expression_id,
            };

            return hir::Node::Interpolation(ctx.alloc(hir_interpolation));
        });
    }

    fn lower_text<'hir>(&self, text: ast::Text<'hir>, ctx: &mut ToHirContext<'hir>) -> NodeId {
        return ctx.push_node(|ctx, node_id, owner_id| {
            let hir_text = hir::Text {
                node_id,
                owner_id,
                value: text.value,
            };

            return hir::Node::Text(ctx.alloc(hir_text));
        });
    }
}

#[cfg(test)]
mod tests {
    use parser::Parser;

    use super::*;

    #[test]
    fn smoke() {
        let allocator = Allocator::default();

        let mut lowerer = AstToHir::new();
        let ast = Parser::new(
            r#"some text { name }<div class:toggle bind:value name="" ok title="idx: {idx}">inside div</div>{#if true}text{/if}"#,
            &allocator,
        )
        .parse()
        .unwrap();

        let hir = lowerer.traverse(ast, &allocator);

        assert!(hir.nodes.len() == 5);
        assert!(hir.owners.len() == 3);
        assert!(hir.attributes.len() == 5);
        assert!(hir.expressions.len() == 5);

        let hir::OwnerNode::Template(template) = hir.owners.first().unwrap() else {
            unreachable!()
        };

        let hir::Node::Concatenation(concatenation) = hir.nodes.first().unwrap() else {
            unreachable!();
        };

        assert!(template.node_ids.len() == 3);
        assert!(concatenation.owner_id == OwnerId::new(0));
    }
}
