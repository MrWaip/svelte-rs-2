use ast_builder::BuilderFunctionArgument;
use oxc_ast::ast::{
    AssignmentOperator, AssignmentTarget, Expression,
};

use super::script_transformer::ScriptTransformer;

impl<'hir> ScriptTransformer<'hir> {
    fn needs_proxy(
        &self,
        e: &Expression,
        operator: AssignmentOperator,
        ctx: &mut oxc_traverse::TraverseCtx<'hir>,
    ) -> bool {
        return false;
        // self.owner_ctx

        // self.store.get_owner(self.owner_ctx)

        let is_non_coercive_operator = operator.is_logical() || operator.is_assign();

        return self.should_proxy_rune_init(e) && is_non_coercive_operator;
    }

    pub(crate) fn transform_rune_assignment(
        &mut self,
        node: &mut Expression<'hir>,
        ctx: &mut oxc_traverse::TraverseCtx<'hir>,
    ) {
        let Expression::AssignmentExpression(assign) = node else {
            unreachable!();
        };

        let ident = if let AssignmentTarget::AssignmentTargetIdentifier(ident) = &assign.left {
            if self.get_rune_by_reference(ident).is_some() {
                Some(ident.name.as_str())
            } else {
                None
            }
        } else {
            None
        };

        if let Some(name) = ident {
            let right = self.b.ast.move_expression(&mut assign.right);
            let needs_proxy = self.needs_proxy(&right, assign.operator, ctx);

            let mut args = vec![
                BuilderFunctionArgument::Ident(name),
                BuilderFunctionArgument::Expr(right),
            ];

            if needs_proxy {
                args.push(BuilderFunctionArgument::Bool(true));
            }

            let call = self.b.call("$.set", args);

            *node = Expression::CallExpression(self.b.alloc(call));
        }
    }
}
