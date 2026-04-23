use oxc_ast::ast::{Expression, Statement};
use svelte_ast::NodeId;
use svelte_ast_builder::Arg;

use crate::context::Ctx;

use super::super::{Codegen, CodegenError, Result};

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(in super::super) fn build_event_handler_s5(
        &mut self,
        attr_id: NodeId,
        handler: Expression<'a>,
        has_call: bool,
        init: &mut Vec<Statement<'a>>,
        expr_offset: u32,
    ) -> Expression<'a> {
        if should_return_event_handler_directly(self.ctx, attr_id, &handler) {
            return handler;
        }

        let has_side_effects = self
            .ctx
            .attr_expression(attr_id)
            .is_some_and(|info| info.has_side_effects());
        let remove_parens = remove_parens_hint(&handler);
        let mut handler = handler;

        if has_call {
            let id = self.ctx.state.gen_ident("event_handler");
            let thunk = self.ctx.b.thunk(handler);
            init.push(
                self.ctx
                    .b
                    .var_stmt(&id, self.ctx.b.call_expr("$.derived", [Arg::Expr(thunk)])),
            );
            handler = self.ctx.b.call_expr("$.get", [Arg::Ident(&id)]);
        }

        build_event_apply_wrapper(
            self.ctx,
            handler,
            expr_offset,
            has_side_effects,
            remove_parens,
        )
    }

    pub(in super::super) fn dev_event_handler(
        &mut self,
        attr_id: NodeId,
        handler: Expression<'a>,
        event_name: &str,
        expr_offset: u32,
    ) -> Result<Expression<'a>> {
        if !self.ctx.state.dev {
            return Ok(handler);
        }

        let Expression::ArrowFunctionExpression(arrow) = handler else {
            return Ok(handler);
        };
        let arrow = arrow.unbox();
        let mut body = arrow.body.unbox();
        let mut stmts: Vec<Statement<'a>> = body.statements.drain(..).collect();

        let has_trace = stmts.first().is_some_and(|s| {
            if let Statement::ExpressionStatement(es) = s {
                is_inspect_trace_call(&es.expression)
            } else {
                false
            }
        });

        if has_trace {
            let trace_stmt = stmts.remove(0);
            let Statement::ExpressionStatement(es) = trace_stmt else {
                return CodegenError::unexpected_node(
                    attr_id,
                    "$inspect.trace guard reported true but first stmt is not ExpressionStatement",
                );
            };
            let es = es.unbox();
            let Expression::CallExpression(call) = es.expression else {
                return CodegenError::unexpected_node(
                    attr_id,
                    "$inspect.trace guard reported true but expression is not CallExpression",
                );
            };
            let mut call = call.unbox();

            let label_expr = if !call.arguments.is_empty() {
                let mut dummy = oxc_ast::ast::Argument::from(self.ctx.b.cheap_expr());
                std::mem::swap(&mut call.arguments[0], &mut dummy);
                dummy.into_expression()
            } else {
                let (line, col) = crate::script::compute_line_col(
                    self.ctx.state.source,
                    expr_offset + arrow.span.start,
                );
                let sanitized = crate::script::sanitize_location(self.ctx.state.filename);
                let label = format!("trace ({sanitized}:{line}:{col})");
                self.ctx.b.str_expr(&label)
            };
            let label_thunk = self.ctx.b.thunk(label_expr);

            let body_thunk = if arrow.r#async {
                self.ctx.b.async_thunk_block(stmts)
            } else {
                self.ctx.b.thunk_block(stmts)
            };

            let trace_call = self
                .ctx
                .b
                .call_expr("$.trace", [Arg::Expr(label_thunk), Arg::Expr(body_thunk)]);
            let return_expr = if arrow.r#async {
                self.ctx.b.await_expr(trace_call)
            } else {
                trace_call
            };
            stmts = vec![self.ctx.b.return_stmt(return_expr)];
            self.ctx.state.has_tracing = true;
        }

        Ok(self
            .ctx
            .b
            .named_function_expr(event_name, arrow.params.unbox(), stmts, arrow.r#async))
    }
}

fn should_return_event_handler_directly<'a>(
    ctx: &Ctx<'a>,
    attr_id: NodeId,
    handler: &Expression<'a>,
) -> bool {
    match handler {
        Expression::ArrowFunctionExpression(_) | Expression::FunctionExpression(_) => true,
        Expression::Identifier(_) if ctx.attr_is_function(attr_id) => true,
        Expression::Identifier(_) if !ctx.state.dev && !ctx.attr_is_import(attr_id) => true,
        _ => false,
    }
}

fn remove_parens_hint(expr: &Expression<'_>) -> bool {
    if let Expression::CallExpression(call) = expr {
        call.arguments.is_empty() && matches!(&call.callee, Expression::Identifier(_))
    } else {
        false
    }
}

fn build_event_apply_wrapper<'a>(
    ctx: &mut Ctx<'a>,
    handler: Expression<'a>,
    expr_offset: u32,
    has_side_effects: bool,
    remove_parens: bool,
) -> Expression<'a> {
    if !ctx.state.dev {
        let call = ctx.b.optional_member_call_expr(
            handler,
            "apply",
            [Arg::Expr(ctx.b.this_expr()), Arg::Ident("$$args")],
        );
        return ctx
            .b
            .function_expr(ctx.b.rest_params("$$args"), vec![ctx.b.expr_stmt(call)]);
    }

    let (line, col) = crate::script::compute_line_col(ctx.state.source, expr_offset);
    let location = ctx
        .b
        .array_expr([ctx.b.num_expr(line as f64), ctx.b.num_expr(col as f64)]);
    let mut args: Vec<Arg<'a, '_>> = vec![
        Arg::Expr(ctx.b.thunk(handler)),
        Arg::Expr(ctx.b.this_expr()),
        Arg::Ident("$$args"),
        Arg::Ident(ctx.state.name),
        Arg::Expr(location),
    ];

    if has_side_effects {
        args.push(Arg::Bool(true));
    } else if remove_parens {
        args.push(Arg::Expr(ctx.b.void_zero_expr()));
    }
    if remove_parens {
        args.push(Arg::Bool(true));
    }

    let call = ctx.b.call_expr("$.apply", args);
    ctx.b
        .function_expr(ctx.b.rest_params("$$args"), vec![ctx.b.expr_stmt(call)])
}

fn is_inspect_trace_call(expr: &Expression) -> bool {
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
