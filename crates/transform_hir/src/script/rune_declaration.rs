use ast_builder::BuilderFunctionArgument;
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
                todo!();
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

    fn should_proxy_rune_init(&self, e: &Expression) -> bool {
        if e.is_literal() {
            return false;
        }

        if matches!(
            e,
            Expression::TemplateLiteral(_)
                | Expression::ArrowFunctionExpression(_)
                | Expression::FunctionExpression(_)
                | Expression::UnaryExpression(_)
                | Expression::BinaryExpression(_)
        ) {
            return false;
        }

        if let Expression::Identifier(id) = e {
            if id.name == "undefined" {
                return false;
            }

            // todo!();
        }

        // if (node.type === 'Identifier' && scope !== null) {
        // 	const binding = scope.get(node.name);
        // 	// Let's see if the reference is something that can be proxied
        // 	if (
        // 		binding !== null &&
        // 		!binding.reassigned &&
        // 		binding.initial !== null &&
        // 		binding.initial.type !== 'FunctionDeclaration' &&
        // 		binding.initial.type !== 'ClassDeclaration' &&
        // 		binding.initial.type !== 'ImportDeclaration' &&
        // 		binding.initial.type !== 'EachBlock' &&
        // 		binding.initial.type !== 'SnippetBlock'
        // 	) {
        // 		return should_proxy(binding.initial, null);
        // 	}
        // }

        return true;
    }
}
