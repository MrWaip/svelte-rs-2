use ast_builder::{BuilderExpression, BuilderFunctionArgument};
use oxc_ast::ast::{BindingPatternKind, Expression};

use super::script_transformer::ScriptTransformer;

impl<'hir> ScriptTransformer<'hir> {
    pub(crate) fn transform_rune_declaration(
        &mut self,
        node: &mut oxc_ast::ast::VariableDeclarator<'hir>,
    ) {
        let BindingPatternKind::BindingIdentifier(id) = &node.id.kind else {
            return;
        };

        let Some(rune) = self.analyses.get_rune(id.symbol_id()) else {
            return;
        };

        let Some(rune_argument) = node.init.as_mut() else {
            return;
        };

        let rune_argument = self.b.ast.move_expression(rune_argument);

        if let Expression::CallExpression(mut call) = rune_argument {
            if rune.mutated {
                call.callee = self.b.expr(BuilderExpression::Ident(self.b.rid("$.state")));

                if call.arguments.is_empty() {
                    call.arguments.push(
                        self.b
                            .unary_expr(
                                oxc_ast::ast::UnaryOperator::Void,
                                self.b.numeric_literal_expr(0.0),
                            )
                            .into(),
                    );
                }

                node.init = Some(Expression::CallExpression(call))
            } else {
                let expr = if call.arguments.is_empty() {
                    self.b.unary_expr(
                        oxc_ast::ast::UnaryOperator::Void,
                        self.b.numeric_literal_expr(0.0),
                    )
                } else {
                    let mut argument = call.arguments.remove(0).into_expression();

                    if self.should_proxy_rune_init(&argument) {
                        argument = self
                            .b
                            .call_expr("$.proxy", [BuilderFunctionArgument::Expr(argument)])
                    }

                    argument
                };

                node.init = Some(expr);
            }
        }
    }
}
