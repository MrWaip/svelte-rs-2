use oxc_ast::ast::{Argument, Expression, Statement};

use crate::builder::Arg;

use super::super::{compute_line_col, sanitize_location, ScriptTransformer};

pub(super) fn is_inspect_call(expr: &Expression) -> bool {
    match expr {
        Expression::CallExpression(call) => {
            if let Expression::Identifier(id) = &call.callee {
                if id.name.as_str() == "$inspect" {
                    return true;
                }
            }
            if let Expression::StaticMemberExpression(member) = &call.callee {
                if member.property.name.as_str() == "with" {
                    if let Expression::CallExpression(inner) = &member.object {
                        if let Expression::Identifier(id) = &inner.callee {
                            return id.name.as_str() == "$inspect";
                        }
                    }
                }
                if member.property.name.as_str() == "trace" {
                    if let Expression::Identifier(id) = &member.object {
                        return id.name.as_str() == "$inspect";
                    }
                }
            }
            false
        }
        _ => false,
    }
}

pub(super) fn is_inspect_trace_call(expr: &Expression) -> bool {
    if let Expression::CallExpression(call) = expr {
        if let Expression::StaticMemberExpression(member) = &call.callee {
            if member.property.name.as_str() == "trace" {
                if let Expression::Identifier(id) = &member.object {
                    return id.name.as_str() == "$inspect";
                }
            }
        }
    }
    false
}

impl<'a> ScriptTransformer<'_, 'a> {
    pub(super) fn rewrite_trace_function_body(
        &mut self,
        body: &mut oxc_ast::ast::FunctionBody<'a>,
    ) {
        if !self.dev {
            return;
        }
        let Some(Statement::ExpressionStatement(es)) = body.statements.first() else {
            return;
        };
        if !is_inspect_trace_call(&es.expression) {
            return;
        }

        let info = self
            .function_info_stack
            .last()
            .unwrap_or_else(|| panic!("$inspect.trace() outside function"));

        let trace_stmt = body.statements.remove(0);
        let Statement::ExpressionStatement(es) = trace_stmt else {
            unreachable!()
        };
        let Expression::CallExpression(call) = es.unbox().expression else {
            unreachable!()
        };
        let mut call = call.unbox();

        let label_expr = if !call.arguments.is_empty() {
            let mut dummy = oxc_ast::ast::Argument::from(self.b.cheap_expr());
            std::mem::swap(&mut call.arguments[0], &mut dummy);
            dummy.into_expression()
        } else {
            let func_name = info.name.as_deref().unwrap_or("trace");
            let full_offset = self.script_content_start + info.span_start;
            let (line, col) = compute_line_col(self.component_source, full_offset);
            let sanitized = sanitize_location(self.filename);
            let label = format!("{func_name} ({sanitized}:{line}:{col})");
            self.b.str_expr(&label)
        };
        let label_thunk = self.b.thunk(label_expr);

        let remaining: Vec<Statement<'a>> = body.statements.drain(..).collect();
        let body_thunk = if info.is_async {
            self.b.async_thunk_block(remaining)
        } else {
            self.b.thunk_block(remaining)
        };

        let trace_call = self
            .b
            .call_expr("$.trace", [Arg::Expr(label_thunk), Arg::Expr(body_thunk)]);
        let return_expr = if info.is_async {
            self.b.await_expr(trace_call)
        } else {
            trace_call
        };
        body.statements.push(self.b.return_stmt(return_expr));
        self.has_tracing = true;
    }

    pub(super) fn transform_inspect(&self, node: &mut Expression<'a>) -> Option<Expression<'a>> {
        let Expression::CallExpression(outer_call) = node else {
            return None;
        };

        if let Expression::StaticMemberExpression(member) = &outer_call.callee {
            if member.property.name.as_str() == "with" {
                if let Expression::CallExpression(inner_call) = &member.object {
                    if let Expression::Identifier(id) = &inner_call.callee {
                        if matches!(id.name.as_str(), "$inspect" | "$.inspect") {
                            let Expression::CallExpression(outer_call) = node else {
                                unreachable!()
                            };

                            let cb = if outer_call.arguments.is_empty() {
                                self.b.rid_expr("undefined")
                            } else {
                                let mut dummy = oxc_ast::ast::Argument::from(self.b.cheap_expr());
                                std::mem::swap(&mut outer_call.arguments[0], &mut dummy);
                                dummy.into_expression()
                            };

                            let Expression::StaticMemberExpression(member) =
                                self.b.move_expr(&mut outer_call.callee)
                            else {
                                unreachable!()
                            };
                            let member = member.unbox();
                            let Expression::CallExpression(inner_call) = member.object else {
                                unreachable!()
                            };
                            let mut inner_call = inner_call.unbox();

                            let thunk = {
                                let mut dummy = oxc_ast::ast::Argument::from(self.b.cheap_expr());
                                std::mem::swap(&mut inner_call.arguments[0], &mut dummy);
                                dummy.into_expression()
                            };

                            let inspector = self.build_inspect_arrow(cb);
                            return Some(
                                self.b.call_expr(
                                    "$.inspect",
                                    [Arg::Expr(thunk), Arg::Expr(inspector)],
                                ),
                            );
                        }
                    }
                }
            }
        }

        if let Expression::Identifier(id) = &outer_call.callee {
            if id.name.as_str() == "$inspect" {
                let Expression::CallExpression(call) = node else {
                    unreachable!()
                };
                let inspect_args: Vec<Expression<'a>> = call
                    .arguments
                    .drain(..)
                    .map(|a| a.into_expression())
                    .collect();

                let thunk = self.build_inspect_thunk(inspect_args);
                let console_log = self.b.static_member_expr(self.b.rid_expr("console"), "log");
                let log_call = self
                    .b
                    .call_expr_callee(console_log, [Arg::Spread(self.b.rid_expr("$$args"))]);
                let inspector = self
                    .b
                    .arrow_expr(self.b.rest_params("$$args"), [self.b.expr_stmt(log_call)]);

                return Some(self.b.call_expr(
                    "$.inspect",
                    [Arg::Expr(thunk), Arg::Expr(inspector), Arg::Bool(true)],
                ));
            }
        }

        None
    }

    fn build_inspect_thunk(&self, args: Vec<Expression<'a>>) -> Expression<'a> {
        let array_args: Vec<Arg<'a, '_>> = args.into_iter().map(Arg::Expr).collect();
        let array = self.b.array_from_args(array_args);
        self.b
            .arrow_expr(self.b.no_params(), [self.b.expr_stmt(array)])
    }

    fn build_inspect_arrow(&self, cb: Expression<'a>) -> Expression<'a> {
        let call = self
            .b
            .call_expr_callee(cb, [Arg::Spread(self.b.rid_expr("$$args"))]);
        self.b
            .arrow_expr(self.b.rest_params("$$args"), [self.b.expr_stmt(call)])
    }

    pub(super) fn transform_console_log(
        &mut self,
        node: &mut Expression<'a>,
    ) -> Option<Expression<'a>> {
        if !self.dev {
            return None;
        }
        let Expression::CallExpression(call) = node else {
            return None;
        };
        let Expression::StaticMemberExpression(member) = &call.callee else {
            return None;
        };
        let Expression::Identifier(console_id) = &member.object else {
            return None;
        };
        if console_id.name != "console" {
            return None;
        }
        let method_name = member.property.name.as_str();
        if !matches!(
            method_name,
            "debug"
                | "dir"
                | "error"
                | "group"
                | "groupCollapsed"
                | "info"
                | "log"
                | "trace"
                | "warn"
        ) {
            return None;
        }
        // Wrap only when at least one arg might contain state (not a trivially-static literal)
        let has_potential_state = call.arguments.iter().any(|arg| {
            !matches!(
                arg,
                Argument::StringLiteral(_)
                    | Argument::NumericLiteral(_)
                    | Argument::BooleanLiteral(_)
                    | Argument::NullLiteral(_)
            )
        });
        if !has_potential_state {
            return None;
        }
        // Rewrite: console.log(...$.log_if_contains_state("method", ...original_args))
        let method_str = method_name.to_string();
        let Expression::CallExpression(call) = node else {
            unreachable!()
        };
        let callee = self.b.move_expr(&mut call.callee);
        let mut inner_args: Vec<Arg<'a, '_>> = vec![Arg::Str(method_str)];
        for arg in call.arguments.drain(..) {
            match arg {
                Argument::SpreadElement(spread) => {
                    inner_args.push(Arg::Spread(spread.unbox().argument));
                }
                arg => inner_args.push(Arg::Expr(arg.into_expression())),
            }
        }
        let log_if_call = self.b.call_expr("$.log_if_contains_state", inner_args);
        Some(self.b.call_expr_callee(callee, [Arg::Spread(log_if_call)]))
    }
}
