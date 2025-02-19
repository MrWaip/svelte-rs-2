use ast::metadata::WithMetadata;
use ast_builder::{
    BuilderAssignmentLeft, BuilderAssignmentRight, BuilderFunctionArgument, BuilderStatement,
};
use oxc_ast::ast::Expression;

use crate::transform_template::{NodeContext, TransformTemplate};

impl<'a, 'link> TransformTemplate<'a, 'link> {
    pub(crate) fn transform_bind_directive<'local>(
        &mut self,
        directive: &mut ast::BindDirective<'a>,
        ctx: &mut NodeContext<'a, 'local>,
    ) {
        // let metadata = directive.get_metadata();
        let node_id = self.b.clone_expr(&ctx.current_node_anchor);

        let Expression::Identifier(ident) = self.b.ast.move_expression(&mut directive.expression)
        else {
            todo!()
        };

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
            ast::BindDirectiveKind::Value => {
                // call = b.call(`$.bind_value`, context.state.node, get, set);
                self.b.call_stmt(
                    "$.bind_value",
                    [
                        BuilderFunctionArgument::Expr(node_id),
                        BuilderFunctionArgument::Arrow(setter),
                    ],
                )
            }
        };

        ctx.push_after_update(stmt);
    }
}
