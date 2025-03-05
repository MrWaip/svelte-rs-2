use std::borrow::Cow;

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

    pub(crate) fn transform_text<'local>(
        &mut self,
        it: &hir::Text<'hir>,
        owner_ctx: &mut OwnerContext<'hir, 'local>,
    ) {
        owner_ctx.push_template(Cow::Borrowed(it.value));
    }
}
