use std::mem;

use oxc_ast::ast::{AssignmentOperator, AssignmentTarget, BindingPattern, Expression, Statement};
use oxc_span::SPAN;
use svelte_ast_builder::{Arg, Builder};
use svelte_component_semantics::OxcNodeId;

use crate::derived::expand_derived_destructure_statements;
use crate::model::ServerTransform;

impl<'a> ServerTransform<'_, 'a> {
    pub(crate) fn rewrite_await(&self, it: &mut Expression<'a>) {
        let Expression::AwaitExpression(await_expr) = it else {
            return;
        };
        let id = await_expr.node_id();
        if !self.await_needs_save(id) {
            return;
        }
        let ast = self.b.ast;
        let arg = mem::replace(
            &mut await_expr.argument,
            ast.expression_identifier(SPAN, ast.atom("")),
        );
        await_expr.argument = self.b.call_expr("$.save", [Arg::Expr(arg)]);
        let awaited = mem::replace(it, ast.expression_identifier(SPAN, ast.atom("")));
        *it = self.b.call_expr_callee(awaited, []);
    }

    fn await_needs_save(&self, id: OxcNodeId) -> bool {
        self.analysis.pickled_awaits.contains(id)
            || self
                .analysis
                .expression_data_by_oxc(id)
                .is_some_and(|data| !data.blockers.is_empty() || !data.references.is_empty())
    }
}

impl<'a> ServerTransform<'_, 'a> {
    pub(crate) fn split_async_instance_body(
        &mut self,
        body: Vec<Statement<'a>>,
    ) -> Vec<Statement<'a>> {
        let b = self.b;
        let analysis = self.analysis;
        let blocker_data = analysis.blocker_data();
        let Some(first_await_idx) = blocker_data.first_await_index() else {
            return body;
        };

        let ident_gen = &mut *self.ident_gen;
        let mut result = Vec::new();
        let mut hoisted_names: Vec<&str> = Vec::new();
        let mut thunks: Vec<Expression<'a>> = Vec::new();

        for (i, stmt) in body.into_iter().enumerate() {
            if i < first_await_idx {
                result.push(stmt);
                continue;
            }

            let meta = blocker_data
                .stmt_meta(i)
                .expect("stmt_meta out of range for async instance body");
            let has_await = meta.has_await();

            for name in meta.hoist_names() {
                hoisted_names.push(b.alloc_str(name));
            }

            let stmt = match stmt {
                Statement::ExportNamedDeclaration(export) => match export.unbox().declaration {
                    Some(decl) => Statement::from(decl),
                    None => continue,
                },
                other => other,
            };

            match stmt {
                Statement::VariableDeclaration(var_decl) => {
                    for declarator in var_decl.unbox().declarations {
                        let declarator = match expand_derived_destructure_statements(
                            b, analysis, ident_gen, declarator,
                        ) {
                            Ok(stmts) => {
                                thunks.push(if has_await {
                                    b.async_thunk_block(stmts)
                                } else {
                                    b.thunk_block(stmts)
                                });
                                continue;
                            }
                            Err(declarator) => declarator,
                        };
                        if matches!(
                            declarator.init.as_ref().map(|e| e.get_inner_expression()),
                            Some(
                                Expression::ArrowFunctionExpression(_)
                                    | Expression::FunctionExpression(_)
                            )
                        ) {
                            result.push(b.var_init_stmt(declarator));
                            continue;
                        }

                        if let Some(target) = binding_to_assignment(&declarator.id, b) {
                            let init = declarator.init.unwrap_or_else(|| b.void_zero_expr());
                            let assign = b.ast.expression_assignment(
                                SPAN,
                                AssignmentOperator::Assign,
                                target,
                                init,
                            );
                            thunks.push(if has_await {
                                b.async_arrow_expr_body(assign)
                            } else {
                                b.thunk(assign)
                            });
                        } else {
                            let var_stmt = b.var_init_stmt(declarator);
                            thunks.push(if has_await {
                                b.async_thunk_block(vec![var_stmt])
                            } else {
                                b.thunk_block(vec![var_stmt])
                            });
                        }
                    }
                }
                Statement::FunctionDeclaration(_) => result.push(stmt),
                other => {
                    if has_await {
                        if let Statement::BlockStatement(block) = other {
                            thunks.push(
                                b.async_thunk_block(block.unbox().body.into_iter().collect()),
                            );
                        } else {
                            thunks.push(b.async_thunk_block(vec![other]));
                        }
                    } else {
                        thunks.push(b.thunk_block(vec![other]));
                    }
                }
            }
        }

        if !hoisted_names.is_empty() {
            result.push(b.var_multi_stmt(&hoisted_names));
        }

        if !thunks.is_empty() {
            let run_call = b.call_expr("$$renderer.run", [Arg::Expr(b.array_expr(thunks))]);
            result.push(b.var_stmt("$$promises", run_call));
        }

        result
    }
}

fn binding_to_assignment<'a>(
    pat: &BindingPattern<'a>,
    b: &Builder<'a>,
) -> Option<AssignmentTarget<'a>> {
    match pat {
        BindingPattern::BindingIdentifier(id) => {
            let ident = b.ast.identifier_reference(SPAN, id.name.as_str());
            Some(AssignmentTarget::AssignmentTargetIdentifier(b.alloc(ident)))
        }
        _ => None,
    }
}
