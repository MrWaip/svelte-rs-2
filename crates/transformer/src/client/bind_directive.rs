use ast_builder::{
    BuilderAssignmentLeft, BuilderAssignmentRight, BuilderExpression, BuilderFunctionArgument,
    BuilderStatement,
};
use oxc_ast::ast::Expression;

use crate::transform_template::{NodeContext, TransformTemplate};

impl<'a, 'link> TransformTemplate<'a, 'link> {
    pub(crate) fn transform_bind_directive<'local>(
        &mut self,
        directive: &mut ast::BindDirective<'a>,
        ctx: &mut NodeContext<'a, 'local>,
    ) {
        let node_id = self.b.clone_expr(&ctx.current_node_anchor);

        let expression = self.b.ast.move_expression(&mut directive.expression);

        let Expression::Identifier(ident) = expression else {
            todo!()
        };

        let getter_expr =
            self.transform_expression(&mut self.b.expr(BuilderExpression::Ident(ident.clone())));

        let getter = self.b.arrow(
            self.b.params([]),
            [self.b.stmt(BuilderStatement::Expr(getter_expr))],
        );

        let mut assignment = self.b.assignment_expression_expr(
            BuilderAssignmentLeft::IdentRef(ident.unbox()),
            BuilderAssignmentRight::Ident("$$value"),
        );

        assignment = self.transform_expression(&mut assignment);

        let setter = self.b.arrow(
            self.b.params(["$$value"]),
            [self.b.stmt(BuilderStatement::Expr(assignment))],
        );

        let stmt = match directive.kind {
            ast::BindDirectiveKind::Unknown => todo!(),
            ast::BindDirectiveKind::Value => self.b.call_stmt(
                "$.bind_value",
                [
                    BuilderFunctionArgument::Expr(node_id),
                    BuilderFunctionArgument::Arrow(getter),
                    BuilderFunctionArgument::Arrow(setter),
                ],
            ),
            ast::BindDirectiveKind::Group => todo!(),
            ast::BindDirectiveKind::Checked => todo!(),
        };

        ctx.push_after_update(stmt);
    }
}
