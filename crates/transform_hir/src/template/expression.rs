use hir::ExpressionId;
use oxc_ast::ast::Expression;

use crate::{context::OwnerContext, script::ScriptTransformer};

use super::template_transformer::TemplateTransformer;

impl<'hir> TemplateTransformer<'hir> {
    pub(crate) fn transform_expression_by_id<'short>(
        &mut self,
        expression_id: ExpressionId,
        ctx: &mut OwnerContext<'hir, 'short>,
    ) -> Expression<'hir> {
        let expression = self.take_expression(expression_id);

        self.transform_expression(expression, ctx)
    }

    pub(crate) fn transform_expression<'short>(
        &mut self,
        expression: Expression<'hir>,
        ctx: &mut OwnerContext<'hir, 'short>,
    ) -> Expression<'hir> {
        let mut transformer =
            ScriptTransformer::new(self.analyses, self.b, self.store, ctx.owner_id());

        transformer.transform_expression(expression)
    }
}
