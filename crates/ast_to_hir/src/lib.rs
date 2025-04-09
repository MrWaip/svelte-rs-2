mod compress_nodes;
pub mod context;
mod trim_nodes;

use std::cell::RefCell;

use hir::{NodeId, OwnerNode};
use oxc_allocator::Allocator;
use oxc_ast::ast::Language;

use crate::context::ToHirContext;

pub struct AstToHir<'hir> {
    allocator: &'hir Allocator,
    builder: ast_builder::Builder<'hir>,
}

#[derive(Debug)]
pub struct AstToHirRet<'hir> {
    pub store: hir::HirStore<'hir>,
}

impl<'hir> AstToHir<'hir> {
    pub fn new(allocator: &'hir Allocator) -> Self {
        Self {
            builder: ast_builder::Builder::new_with_ast(allocator),
            allocator,
        }
    }

    pub fn traverse(&mut self, ast: ast::Ast<'hir>) -> AstToHirRet<'hir> {
        let hir_program = self.lower_root_script(ast.script);
        let mut ctx = ToHirContext::new(self.allocator, hir_program);

        self.lower_template(&mut ctx, ast.template.unwrap());

        AstToHirRet { store: ctx.store }
    }

    fn lower_root_script(&self, script: Option<ast::ScriptTag<'hir>>) -> hir::Program<'hir> {
        let hir_program = script
            .map(|script| hir::Program {
                language: script.language,
                program: RefCell::new(script.program),
            })
            .unwrap_or_else(|| {
                let oxc_program = self.builder.program(Vec::new());

                hir::Program {
                    language: Language::JavaScript,
                    program: RefCell::new(oxc_program),
                }
            });

        hir_program
    }

    fn lower_template(&mut self, ctx: &mut ToHirContext<'hir>, template: ast::Template<'hir>) {
        ctx.push_owner_node(|ctx, node_id, _| {
            let node_ids: Vec<NodeId> = self.lower_nodes(ctx, template.nodes.nodes);

            let template = hir::Template { node_ids, node_id };

            (hir::Node::Phantom, OwnerNode::Template(ctx.alloc(template)))
        });
    }

    fn lower_nodes(
        &self,
        ctx: &mut ToHirContext<'hir>,
        mut nodes: Vec<ast::Node<'hir>>,
    ) -> Vec<NodeId> {
        nodes = self.clean_comments(nodes, ctx);

        nodes = self.trim_text_nodes(nodes, ctx);

        self.compress_and_lower_nodes(nodes, ctx)
    }

    fn lower_node(&self, node: ast::Node<'hir>, ctx: &mut ToHirContext<'hir>) -> NodeId {
        match node {
            ast::Node::Element(cell) => self.lower_element(cell.unwrap(), ctx),
            ast::Node::Text(cell) => self.lower_text(cell.unwrap(), ctx),
            ast::Node::Interpolation(cell) => self.lower_interpolation(cell.unwrap(), ctx),
            ast::Node::IfBlock(cell) => self.lower_if_block(cell.unwrap(), ctx),
            ast::Node::VirtualConcatenation(_) => unreachable!(),
            ast::Node::ScriptTag(_) => todo!(),
            ast::Node::Comment(cell) => self.lower_comment(cell.unwrap(), ctx),
            ast::Node::EachBlock(cell) => self.lower_each_block(cell.unwrap(), ctx),
        }
    }

    fn lower_each_block(&self, node: ast::EachBlock<'hir>, ctx: &mut ToHirContext<'hir>) -> NodeId {
        ctx.push_owner_node(|ctx, self_node_id, owner_id| {
            let collection_id = ctx.push_expression(node.collection);
            let item_id = ctx.push_expression(node.item);

            let hir_each_block = hir::EachBlock {
                node_id: self_node_id,
                owner_id,
                node_ids: self.lower_nodes(ctx, node.nodes.nodes),
                index: None,
                collection: collection_id,
                key: None,
                item: item_id,
            };

            let hir_each_block = ctx.alloc(hir_each_block);

            (
                hir::Node::EachBlock(hir_each_block),
                hir::OwnerNode::EachBlock(hir_each_block),
            )
        })
    }

    fn lower_if_block(&self, if_block: ast::IfBlock<'hir>, ctx: &mut ToHirContext<'hir>) -> NodeId {
        ctx.push_owner_node(|ctx, self_node_id, owner_id| {
            let expression_id = ctx.push_expression(if_block.test);

            let hir_if_block = hir::IfBlock {
                node_id: self_node_id,
                owner_id,
                test: expression_id,
                consequent: self.lower_nodes(ctx, if_block.consequent.nodes),
                is_elseif: if_block.is_elseif,
                alternate: if_block
                    .alternate
                    .map(|fragment| self.lower_nodes(ctx, fragment.nodes)),
            };

            let hir_if_block = ctx.alloc(hir_if_block);

            (
                hir::Node::IfBlock(hir_if_block),
                hir::OwnerNode::IfBlock(hir_if_block),
            )
        })
    }

    fn lower_element(&self, element: ast::Element<'hir>, ctx: &mut ToHirContext<'hir>) -> NodeId {
        ctx.push_owner_node(|ctx, self_node_id, owner_id| {
            let name = ctx.alloc(element.name);

            let mut hir_element =
                hir::Element::new(owner_id, self_node_id, name, element.self_closing);

            self.lower_attributes(ctx, element.attributes, &mut hir_element);

            hir_element.node_ids = self.lower_nodes(ctx, element.nodes);

            let hir_element = ctx.alloc(hir_element);

            (
                hir::Node::Element(hir_element),
                hir::OwnerNode::Element(hir_element),
            )
        })
    }

    fn lower_attributes(
        &self,
        ctx: &mut ToHirContext<'hir>,
        attributes: Vec<ast::Attribute<'hir>>,
        hir_element: &mut hir::Element<'hir>,
    ) {
        for attribute in attributes {
            self.lower_attribute(ctx, attribute, hir_element);
        }
    }

    fn lower_attribute(
        &self,
        ctx: &mut ToHirContext<'hir>,
        attribute: ast::Attribute<'hir>,
        hir_element: &mut hir::Element<'hir>,
    ) {
        match attribute {
            ast::Attribute::ConcatenationAttribute(attr) => {
                hir_element
                    .attributes
                    .push_attr(self.lower_concatenation_attribute(ctx, attr));
            }
            ast::Attribute::ExpressionAttribute(attr) => {
                hir_element
                    .attributes
                    .push_attr(self.lower_expression_attribute(ctx, attr));
            }
            ast::Attribute::ClassDirective(attr) => {
                hir_element
                    .attributes
                    .push_class_directive(self.lower_class_directive(ctx, attr));
            }
            ast::Attribute::BindDirective(attr) => {
                hir_element
                    .attributes
                    .push_bind_directive(self.lower_bind_directive(ctx, attr));
            }
            ast::Attribute::BooleanAttribute(attr) => {
                hir_element
                    .attributes
                    .push_attr(self.lower_boolean_attribute(ctx, attr));
            }
            ast::Attribute::StringAttribute(attr) => {
                hir_element
                    .attributes
                    .push_attr(self.lower_string_attribute(ctx, attr));
            }
            ast::Attribute::SpreadAttribute(attr) => {
                hir_element
                    .attributes
                    .push_attr(self.lower_spread_attribute(ctx, attr));
            }
        };
    }

    fn lower_class_directive(
        &self,
        ctx: &mut ToHirContext<'hir>,
        attr: ast::ClassDirective<'hir>,
    ) -> &'hir hir::ClassDirective<'hir> {
        let expression_id = ctx.push_expression(attr.expression);

        let attr = hir::ClassDirective {
            name: attr.name,
            shorthand: attr.shorthand,
            expression_id,
        };

        ctx.alloc(attr)
    }

    fn lower_bind_directive(
        &self,
        ctx: &mut ToHirContext<'hir>,
        attr: ast::BindDirective<'hir>,
    ) -> &'hir hir::BindDirective<'hir> {
        let expression_id = ctx.push_expression(attr.expression);
        let attr = hir::BindDirective {
            expression_id,
            name: attr.name,
            shorthand: attr.shorthand,
        };

        ctx.alloc(attr)
    }

    fn lower_boolean_attribute(
        &self,
        ctx: &mut ToHirContext<'hir>,
        attr: ast::BooleanAttribute<'hir>,
    ) -> &'hir hir::Attribute<'hir> {
        let attribute = hir::BooleanAttribute { name: attr.name };

        ctx.alloc(hir::Attribute::BooleanAttribute(ctx.alloc(attribute)))
    }

    fn lower_concatenation_attribute(
        &self,
        ctx: &mut ToHirContext<'hir>,
        attr: ast::ConcatenationAttribute<'hir>,
    ) -> &'hir hir::Attribute<'hir> {
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

        ctx.alloc(hir::Attribute::ConcatenationAttribute(ctx.alloc(attribute)))
    }

    fn lower_expression_attribute(
        &self,
        ctx: &mut ToHirContext<'hir>,
        attr: ast::ExpressionAttribute<'hir>,
    ) -> &'hir hir::Attribute<'hir> {
        let expression_id = ctx.push_expression(attr.expression);

        let attribute = hir::ExpressionAttribute {
            name: attr.name,
            shorthand: attr.shorthand,
            expression_id,
        };

        ctx.alloc(hir::Attribute::ExpressionAttribute(ctx.alloc(attribute)))
    }

    fn lower_spread_attribute(
        &self,
        ctx: &mut ToHirContext<'hir>,
        attr: ast::SpreadAttribute<'hir>,
    ) -> &'hir hir::Attribute<'hir> {
        let expression_id = ctx.push_expression(attr.expression);

        let attribute = hir::SpreadAttribute { expression_id };

        ctx.alloc(hir::Attribute::SpreadAttribute(ctx.alloc(attribute)))
    }

    fn lower_string_attribute(
        &self,
        ctx: &mut ToHirContext<'hir>,
        attr: ast::StringAttribute<'hir>,
    ) -> &'hir hir::Attribute<'hir> {
        let attribute = hir::StringAttribute {
            name: attr.name,
            value: attr.value,
        };

        ctx.alloc(hir::Attribute::StringAttribute(ctx.alloc(attribute)))
    }

    fn lower_interpolation(
        &self,
        interpolation: ast::Interpolation<'hir>,
        ctx: &mut ToHirContext<'hir>,
    ) -> NodeId {
        ctx.push_node(|ctx, node_id, owner_id| {
            let expression_id = ctx.push_expression(interpolation.expression);

            let hir_interpolation = hir::Interpolation {
                node_id,
                owner_id,
                expression_id,
            };

            hir::Node::Interpolation(ctx.alloc(hir_interpolation))
        })
    }

    fn lower_text(&self, text: ast::Text<'hir>, ctx: &mut ToHirContext<'hir>) -> NodeId {
        ctx.push_node(|ctx, node_id, owner_id| {
            let hir_text = hir::Text {
                node_id,
                owner_id,
                value: text.value,
            };

            hir::Node::Text(ctx.alloc(hir_text))
        })
    }

    fn lower_comment(&self, comment: ast::Comment<'hir>, ctx: &mut ToHirContext<'hir>) -> NodeId {
        ctx.push_node(|ctx, node_id, owner_id| {
            let hir_text = hir::Comment {
                node_id,
                owner_id,
                value: comment.value,
            };

            hir::Node::Comment(ctx.alloc(hir_text))
        })
    }

    fn clean_comments(
        &self,
        nodes: Vec<ast::Node<'hir>>,
        ctx: &mut ToHirContext<'hir>,
    ) -> Vec<ast::Node<'hir>> {
        let mut result = vec![];
        let mut idx: usize = 0;

        while idx < nodes.len() {
            let prev = if idx == 0 { None } else { nodes.get(idx - 1) };
            let next = nodes.get(idx + 1);
            let current = &nodes[idx];
            idx += 1;

            if current.is_comment() {
                let Some(ast::Node::Text(prev_text)) = prev else {
                    continue;
                };

                let Some(ast::Node::Text(next_text)) = next else {
                    continue;
                };

                let mut new_string = prev_text.borrow().value.to_string();
                new_string.push_str(next_text.borrow().value);

                prev_text.borrow_mut().value = ctx.alloc(new_string);

                // skip next text node
                idx += 1;
                continue;
            }

            result.push(current.clone());
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use hir::HirStore;
    use parser::Parser;

    use super::*;

    #[test]
    fn smoke() {
        let allocator = Allocator::default();

        let mut lowerer = AstToHir::new(&allocator);
        let ast = Parser::new(
            r#"some text { name }<div class:toggle bind:value name="" ok title="idx: {idx}">inside div</div>{#if true}text{/if}"#,
            &allocator,
        )
        .parse()
        .unwrap();

        let store = lowerer.traverse(ast).store;

        assert!(store.nodes.len() == 6);
        assert!(store.owners.len() == 3);
        assert!(store.expressions.len() == 5);

        let hir::OwnerNode::Template(template) = store.owners.first().unwrap() else {
            unreachable!()
        };

        let hir::Node::Concatenation(concatenation) = store.nodes.get(NodeId::new(1)).unwrap()
        else {
            unreachable!();
        };

        assert!(template.node_ids.len() == 3);
        assert!(concatenation.owner_id == HirStore::TEMPLATE_OWNER_ID);
    }
}
