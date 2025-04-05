use hir::NodeId;

use crate::{AstToHir, context::ToHirContext};

#[derive(Debug)]
pub(crate) enum Compressable<'hir> {
    Text(ast::Text<'hir>),
    Interpolation(ast::Interpolation<'hir>),
}

impl<'hir> AstToHir<'hir> {
    pub(crate) fn compress_and_lower_nodes(
        &self,
        nodes: Vec<ast::Node<'hir>>,
        ctx: &mut ToHirContext<'hir>,
    ) -> Vec<NodeId> {
        let mut to_compress: Vec<Compressable<'hir>> = Vec::new();
        let mut result: Vec<NodeId> = Vec::new();

        for node in nodes {
            match node {
                ast::Node::Text(cell) => {
                    to_compress.push(Compressable::Text(cell.unwrap()));
                    continue;
                }
                ast::Node::Interpolation(cell) => {
                    to_compress.push(Compressable::Interpolation(cell.unwrap()));
                    continue;
                }
                _ => (),
            };

            if to_compress.len() == 1 {
                let node_id: NodeId = match to_compress.pop().unwrap() {
                    Compressable::Text(text) => self.lower_text(text, ctx),
                    Compressable::Interpolation(interpolation) => {
                        self.lower_interpolation(interpolation, ctx)
                    }
                };

                result.push(node_id);
            } else if to_compress.len() > 1 {
                let node_id = self.lower_compressible_sequence(to_compress, ctx);
                to_compress = Vec::new();
                result.push(node_id);
            }

            let node_id = self.lower_node(node, ctx);

            result.push(node_id);
        }

        // edge case when sequence end with compressible node
        if to_compress.len() == 1 {
            let node_id: NodeId = match to_compress.pop().unwrap() {
                Compressable::Text(text) => self.lower_text(text, ctx),
                Compressable::Interpolation(interpolation) => {
                    self.lower_interpolation(interpolation, ctx)
                }
            };

            result.push(node_id);
        } else if to_compress.len() > 1 {
            let node_id = self.lower_compressible_sequence(to_compress, ctx);
            result.push(node_id);
        }

        return result;
    }

    fn lower_compressible_sequence(
        &self,
        nodes: Vec<Compressable<'hir>>,
        ctx: &mut ToHirContext<'hir>,
    ) -> NodeId {
        ctx.push_node(|ctx, node_id, owner_id| {
            let mut parts: Vec<hir::ConcatenationPart> = Vec::new();

            for node in nodes {
                match node {
                    Compressable::Text(text) => {
                        let part = hir::ConcatenationPart::Text(text.value);
                        parts.push(part);
                    }
                    Compressable::Interpolation(interpolation) => {
                        let expression_id = ctx.push_expression(interpolation.expression);
                        let part = hir::ConcatenationPart::Expression(expression_id);
                        parts.push(part);
                    }
                }
            }

            let concatenation = hir::Concatenation {
                node_id,
                owner_id,
                parts,
            };

            hir::Node::Concatenation(ctx.alloc(concatenation))
        })
    }
}
