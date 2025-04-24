use std::borrow::Cow;

use analyze_hir::HirAnalyses;
use ast_builder::Builder;
use hir::{ExpressionId, HirStore};
use oxc_ast::ast::{Expression, Statement};

use crate::context::OwnerContext;

// use super::context::OwnerContext;

pub struct TemplateTransformer<'hir> {
    pub(crate) analyses: &'hir HirAnalyses,
    pub(crate) b: &'hir Builder<'hir>,
    pub(crate) store: &'hir HirStore<'hir>,
    pub(crate) hoisted: Vec<Statement<'hir>>,
}

impl<'hir> TemplateTransformer<'hir> {
    pub const COMMENT_NODE_ANCHOR: &'static str = "<!>";

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

        self.transform_template(template)
    }

    fn transform_template(&mut self, template: &hir::Template) -> Vec<Statement<'hir>> {
        let content_type = self
            .analyses
            .get_common_content_type(&HirStore::TEMPLATE_OWNER_ID);

        self.transform_fragment(
            &template.node_ids,
            HirStore::TEMPLATE_OWNER_ID,
            content_type,
        )
    }

    pub(crate) fn transform_text<'local>(
        &mut self,
        it: &hir::Text<'hir>,
        owner_ctx: &mut OwnerContext<'hir, 'local>,
    ) {
        owner_ctx.push_template(Cow::Borrowed(it.value));
    }

    pub(crate) fn take_expression(&self, expression_id: ExpressionId) -> Expression<'hir> {
        let mut expression = self.store.get_expression_mut(expression_id);
        self.b.move_expr(&mut expression)
    }
}
