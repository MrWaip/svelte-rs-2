use analyze_hir::HirAnalyses;
use ast_builder::{Builder, BuilderFunctionArgument as BArg};
use hir::{HirStore, NodeId};
use oxc_ast::ast::Statement;

use super::context::{FragmentContext, OwnerContext};

pub struct TemplateTransformer<'hir> {
    analyses: &'hir HirAnalyses,
    builder: &'hir Builder<'hir>,
    store: &'hir HirStore<'hir>,
}

impl<'hir> TemplateTransformer<'hir> {
    pub fn new(
        analyses: &'hir HirAnalyses,
        builder: &'hir Builder<'hir>,
        store: &'hir HirStore<'hir>,
    ) -> Self {
        Self {
            analyses,
            builder,
            store,
        }
    }

    pub fn transform(&mut self) -> Vec<Statement<'hir>> {
        let template = self.store.get_template();

        return self.transform_template(template);
    }

    fn transform_template(&mut self, template: &hir::Template) -> Vec<Statement<'hir>> {
        return self.transform_fragment(&template.node_ids);
    }

    fn transform_fragment(&mut self, nodes: &Vec<NodeId>) -> Vec<Statement<'hir>> {
        let mut context = FragmentContext::new();
        let mut body = Vec::new();

        let identifier = "fragment";

        self.transform_nodes(nodes, &mut context);

        body.extend(context.before_init);
        body.extend(context.init);

        if !context.update.is_empty() {
            // body.push(self.build_template_effect(context.update));
        }

        body.extend(context.after_update);

        let close = self.builder.call_stmt(
            "$.append",
            [BArg::Ident("$$anchor"), BArg::Ident(&identifier)],
        );

        body.push(close);

        return body;
    }

    fn transform_nodes<'local>(
        &mut self,
        nodes: &Vec<NodeId>,
        fragment_ctx: &'local mut FragmentContext<'hir>,
    ) {
        let mut owner_ctx = OwnerContext::new(fragment_ctx);

        for node_id in nodes {
            let node = self.store.get_node(*node_id);

            match node {
                hir::Node::Text(it) => self.transform_text(it, &mut owner_ctx),
                hir::Node::Interpolation(interpolation) => todo!(),
                hir::Node::Element(element) => todo!(),
                hir::Node::Comment => todo!(),
                hir::Node::IfBlock(if_block) => todo!(),
                hir::Node::EachBlock => todo!(),
                hir::Node::Script => todo!(),
                hir::Node::Concatenation(concatenation) => todo!(),
                hir::Node::Phantom => todo!(),
            }
        }
    }

    fn transform_text<'local>(
        &mut self,
        it: &hir::Text<'hir>,
        owner_ctx: &mut OwnerContext<'hir, 'local>,
    ) {
        //
    }
}
