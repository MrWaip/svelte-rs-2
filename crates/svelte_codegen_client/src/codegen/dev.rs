use std::mem;

use oxc_ast::ast::{ChainElement, Expression, Statement};
use svelte_ast::BindDirective;
use svelte_ast_builder::Arg;

use super::Codegen;

pub(in crate::codegen) fn getter_return_member<'a, 'b>(
    getter: &'b Expression<'a>,
) -> Option<&'b Expression<'a>> {
    let stmts = match getter.get_inner_expression() {
        Expression::ArrowFunctionExpression(a) => &a.body.statements,
        Expression::FunctionExpression(f) => &f.body.as_ref()?.statements,
        _ => return None,
    };
    for stmt in stmts {
        if let Statement::ReturnStatement(rs) = stmt {
            return rs.argument.as_ref();
        }
    }
    for stmt in stmts {
        if let Statement::ExpressionStatement(es) = stmt {
            return Some(&es.expression);
        }
    }
    None
}

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    fn strip_member_optional(&self, expr: Expression<'a>) -> Expression<'a> {
        match expr {
            Expression::ChainExpression(c) => {
                let inner = match c.unbox().expression {
                    ChainElement::StaticMemberExpression(m) => {
                        Expression::StaticMemberExpression(m)
                    }
                    ChainElement::ComputedMemberExpression(m) => {
                        Expression::ComputedMemberExpression(m)
                    }
                    ChainElement::PrivateFieldExpression(m) => {
                        Expression::PrivateFieldExpression(m)
                    }
                    ChainElement::CallExpression(m) => Expression::CallExpression(m),
                    ChainElement::TSNonNullExpression(m) => Expression::TSNonNullExpression(m),
                };
                self.strip_member_optional(inner)
            }
            Expression::StaticMemberExpression(mut m) => {
                m.optional = false;
                let obj = mem::replace(&mut m.object, self.ctx.b.cheap_expr());
                m.object = self.strip_member_optional(obj);
                Expression::StaticMemberExpression(m)
            }
            Expression::ComputedMemberExpression(mut m) => {
                m.optional = false;
                let obj = mem::replace(&mut m.object, self.ctx.b.cheap_expr());
                m.object = self.strip_member_optional(obj);
                Expression::ComputedMemberExpression(m)
            }
            other => other,
        }
    }

    pub(in crate::codegen) fn build_validate_binding_from_member(
        &self,
        bind: &BindDirective,
        member: &Expression<'a>,
    ) -> Option<Statement<'a>> {
        let (object, property) = match member.get_inner_expression() {
            Expression::StaticMemberExpression(m) => (
                self.ctx.b.clone_expr(&m.object),
                self.ctx.b.str_expr(m.property.name.as_str()),
            ),
            Expression::ComputedMemberExpression(m) => (
                self.ctx.b.clone_expr(&m.object),
                self.ctx.b.clone_expr(&m.expression),
            ),
            Expression::ChainExpression(c) => match &c.expression {
                ChainElement::StaticMemberExpression(m) => (
                    self.ctx.b.clone_expr(&m.object),
                    self.ctx.b.str_expr(m.property.name.as_str()),
                ),
                ChainElement::ComputedMemberExpression(m) => (
                    self.ctx.b.clone_expr(&m.object),
                    self.ctx.b.clone_expr(&m.expression),
                ),
                _ => return None,
            },
            _ => return None,
        };
        let object = self.strip_member_optional(object);
        let source = self.ctx.query.component.source_text(bind.span).to_string();
        let (line, col) = self.ctx.state.line_index.line_col(bind.span.start);
        let object_thunk = self.ctx.b.thunk(object);
        let property_thunk = self.ctx.b.thunk(property);
        let empty = self.ctx.b.empty_array_expr();
        Some(self.ctx.b.call_stmt(
            "$.validate_binding",
            [
                Arg::Str(source),
                Arg::Expr(empty),
                Arg::Expr(object_thunk),
                Arg::Expr(property_thunk),
                Arg::Num(line as f64),
                Arg::Num(col as f64),
            ],
        ))
    }

    pub(in crate::codegen) fn add_svelte_meta(
        &self,
        expression: Expression<'a>,
        span_start: u32,
        block_type: &str,
    ) -> Statement<'a> {
        self.add_svelte_meta_with_extra(expression, span_start, block_type, None)
    }

    pub(in crate::codegen) fn add_svelte_meta_with_extra(
        &self,
        expression: Expression<'a>,
        span_start: u32,
        block_type: &str,
        extra: Option<Expression<'a>>,
    ) -> Statement<'a> {
        if !self.ctx.state.dev {
            return self.ctx.b.expr_stmt(expression);
        }
        let (line, col) = self.ctx.state.line_index.line_col(span_start);
        let thunk = self
            .ctx
            .b
            .arrow_expr(self.ctx.b.no_params(), [self.ctx.b.expr_stmt(expression)]);
        let mut args = vec![
            Arg::Expr(thunk),
            Arg::Str(block_type.to_string()),
            Arg::Ident(self.ctx.state.name),
            Arg::Num(line as f64),
            Arg::Num(col as f64),
        ];
        if let Some(extra) = extra {
            args.push(Arg::Expr(extra));
        }
        self.ctx.b.call_stmt("$.add_svelte_meta", args)
    }
}
