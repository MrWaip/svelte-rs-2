use analyze_hir::HirAnalyses;
use ast_builder::{Builder, BuilderFunctionArgument as BArg};
use hir::{HirStore, NodeId};
use oxc_ast::ast::{Expression, Statement};

use super::context::{FragmentContext, OwnerContext};

pub struct TemplateTransformer<'hir> {
    pub(crate) analyses: &'hir HirAnalyses,
    pub(crate) b: &'hir Builder<'hir>,
    pub(crate) store: &'hir HirStore<'hir>,
    pub(crate) hoisted: Vec<Statement<'hir>>,
}

impl<'hir> TemplateTransformer<'hir> {
    pub fn new(
        analyses: &'hir HirAnalyses,
        builder: &'hir Builder<'hir>,
        store: &'hir HirStore<'hir>,
    ) -> Self {
        Self {
            analyses,
            b: builder,
            store,
            hoisted: Vec::new(),
        }
    }

    pub fn transform(&mut self) -> Vec<Statement<'hir>> {
        let template = self.store.get_template();

        return self.transform_template(template);
    }

    fn transform_template(&mut self, template: &hir::Template) -> Vec<Statement<'hir>> {
        return self.transform_fragment(&template.node_ids, HirStore::TEMPLATE_OWNER_ID);
    }

    pub(crate) fn transform_nodes<'local>(
        &mut self,
        nodes: &Vec<NodeId>,
        mut owner_ctx: OwnerContext<'hir, 'local>,
    ) {
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
        owner_ctx.push_template(it.value.to_string());
    }
}
