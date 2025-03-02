use ast_builder::{
    BuilderAssignmentLeft, BuilderAssignmentRight, BuilderExpression as BExpr,
    BuilderFunctionArgument as BArg, TemplateLiteralPart,
};
use hir::{NodeId, OwnerId};
use oxc_ast::ast::Statement;

use super::{
    context::{FragmentContext, OwnerContext},
    template_transformer::TemplateTransformer,
};

impl<'hir> TemplateTransformer<'hir> {
    pub(crate) fn transform_interpolation<'short>(
        &mut self,
        node: &hir::Interpolation,
        ctx: &mut OwnerContext<'hir, 'short>,
    ) {
    }

    pub(crate) fn transform_concatenation<'short>(
        &mut self,
        node: &hir::Concatenation<'hir>,
        ctx: &mut OwnerContext<'hir, 'short>,
    ) {
        let mut parts = Vec::new();

        for part in node.parts.iter() {
            match part {
                hir::ConcatenationPart::Text(value) => {
                    parts.push(TemplateLiteralPart::String(value));
                }
                hir::ConcatenationPart::Expression(expression_id) => {
                    let mut expr = self.store.get_expression_mut(*expression_id);

                    let expr = self.b.move_expr(&mut *expr);

                    parts.push(TemplateLiteralPart::Expression(expr));
                }
            }
        }

        let expression = self.b.template_literal2(parts);

        let prop = "nodeValue";

        let member = self
            .b
            .static_member_expr(self.b.expr(BExpr::Ident(self.b.rid("text"))), prop);

        let set_text = self.b.assignment_expression_stmt(
            BuilderAssignmentLeft::StaticMemberExpression(member),
            BuilderAssignmentRight::Expr(self.b.expr(BExpr::TemplateLiteral(expression))),
        );

        ctx.push_init(set_text);
    }
}
