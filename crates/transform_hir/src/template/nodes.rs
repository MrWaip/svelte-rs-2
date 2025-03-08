use ast_builder::BuilderFunctionArgument as BArg;
use hir::NodeId;

use super::{
    context::OwnerContext, is_static::is_static_element, template_transformer::TemplateTransformer,
};

impl<'hir> TemplateTransformer<'hir> {
    pub(crate) fn transform_nodes<'local>(
        &mut self,
        nodes: &Vec<NodeId>,
        mut owner_ctx: OwnerContext<'hir, 'local>,
    ) {
        for node_id in nodes {
            let node = self.store.get_node(*node_id);

            if is_static_element(node, self.store, self.analyses) {
                owner_ctx.next_sibling();
            } else if matches!(node, hir::Node::EachBlock) {
                // if (node.type === 'EachBlock' && nodes.length === 1 && is_element)
                // node.metadata.is_controlled = true;
                todo!()
            } else if node.is_interpolation_like() {
                let name = self.analyses.generate_ident("text");
                owner_ctx.flush_node(false, &name);
            } else if node.is_text() {
                owner_ctx.next_sibling();
            } else {
                let name = match node {
                    hir::Node::Element(element) => element.name,
                    _ => "node",
                };

                let name = self.analyses.generate_ident(name);

                owner_ctx.flush_node(false, &name);
            }

            self.transform_node(node, &mut owner_ctx);
        }

        // if there are trailing static text nodes/elements,
        // traverse to the last (n - 1) one when hydrating
        if owner_ctx.trailing_static_nodes() {
            let offset = owner_ctx.sibling_offset() - 1;
            let mut args = vec![];

            if offset != 1 {
                args.push(BArg::Num(offset as f64));
            }

            owner_ctx.push_init(self.b.call_stmt("$.next", args));
        }
    }

    pub(crate) fn transform_node<'short>(
        &mut self,
        node: &hir::Node<'hir>,
        owner_ctx: &mut OwnerContext<'hir, 'short>,
    ) {
        match node {
            hir::Node::Text(it) => self.transform_text(it, owner_ctx),
            hir::Node::Interpolation(it) => self.transform_interpolation(it, owner_ctx),
            hir::Node::Element(it) => self.transform_element(it, owner_ctx),
            hir::Node::Concatenation(it) => self.transform_concatenation(it, owner_ctx),
            hir::Node::IfBlock(if_block) => todo!(),
            hir::Node::Comment => todo!(),
            hir::Node::EachBlock => todo!(),
            hir::Node::Script => todo!(),
            hir::Node::Phantom => todo!(),
        }
    }
}
