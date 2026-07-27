use oxc_ast::ast::{ChainElement, Expression, Statement};
use oxc_semantic::SymbolId;
use svelte_analyze::AttributeSemantics;
use svelte_ast::BindDirective;
use svelte_ast_builder::Arg;

use super::Codegen;
use super::expr::build_reactive_dep_expr_legacy;

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
    fn bind_each_item_store_backed(&self, bind: &BindDirective) -> bool {
        match self.ctx.query.analysis.attributes.get(bind.id) {
            AttributeSemantics::ElementBind(payload) => payload.each_item_store_backed,
            AttributeSemantics::ComponentBind(payload) => payload.each_item_store_backed,
            _ => false,
        }
    }

    fn bind_each_context_vars(&self, bind: &BindDirective) -> Vec<SymbolId> {
        match self.ctx.query.analysis.attributes.get(bind.id) {
            AttributeSemantics::ElementBind(payload) => payload.each_context_vars.to_vec(),
            AttributeSemantics::ComponentBind(payload) => payload.each_context_vars.to_vec(),
            _ => Vec::new(),
        }
    }

    fn clone_member_substituting_each_context(
        &self,
        expr: &Expression<'a>,
        each_context: &[SymbolId],
    ) -> Expression<'a> {
        match expr {
            Expression::Identifier(id) => {
                if each_context.is_empty() {
                    return self.ctx.b.clone_expr(expr);
                }
                let Some(sym) = self.ctx.query.view.symbol_for_identifier_reference(id) else {
                    return self.ctx.b.clone_expr(expr);
                };
                if !each_context.contains(&sym) {
                    return self.ctx.b.clone_expr(expr);
                }
                build_reactive_dep_expr_legacy(self.ctx, sym)
                    .unwrap_or_else(|| self.ctx.b.clone_expr(expr))
            }
            Expression::StaticMemberExpression(m) => {
                let object = self.clone_member_substituting_each_context(&m.object, each_context);
                self.ctx
                    .b
                    .static_member_expr(object, m.property.name.as_str())
            }
            Expression::ComputedMemberExpression(m) => {
                let object = self.clone_member_substituting_each_context(&m.object, each_context);
                let property =
                    self.clone_member_substituting_each_context(&m.expression, each_context);
                self.ctx.b.computed_member_expr(object, property)
            }
            Expression::ChainExpression(c) => match &c.expression {
                ChainElement::StaticMemberExpression(m) => {
                    let object =
                        self.clone_member_substituting_each_context(&m.object, each_context);
                    self.ctx
                        .b
                        .static_member_expr(object, m.property.name.as_str())
                }
                ChainElement::ComputedMemberExpression(m) => {
                    let object =
                        self.clone_member_substituting_each_context(&m.object, each_context);
                    let property =
                        self.clone_member_substituting_each_context(&m.expression, each_context);
                    self.ctx.b.computed_member_expr(object, property)
                }
                _ => self.ctx.b.clone_expr(expr),
            },
            other => self.ctx.b.clone_expr(other),
        }
    }

    pub(in crate::codegen) fn build_validate_binding_from_member(
        &self,
        bind: &BindDirective,
        member: &Expression<'a>,
    ) -> Option<Statement<'a>> {
        let each_context = self.bind_each_context_vars(bind);
        let (object, property) = match member.get_inner_expression() {
            Expression::StaticMemberExpression(m) => (
                self.clone_member_substituting_each_context(&m.object, &each_context),
                self.ctx.b.str_expr(m.property.name.as_str()),
            ),
            Expression::ComputedMemberExpression(m) => (
                self.clone_member_substituting_each_context(&m.object, &each_context),
                self.clone_member_substituting_each_context(&m.expression, &each_context),
            ),
            Expression::ChainExpression(c) => match &c.expression {
                ChainElement::StaticMemberExpression(m) => (
                    self.clone_member_substituting_each_context(&m.object, &each_context),
                    self.ctx.b.str_expr(m.property.name.as_str()),
                ),
                ChainElement::ComputedMemberExpression(m) => (
                    self.clone_member_substituting_each_context(&m.object, &each_context),
                    self.clone_member_substituting_each_context(&m.expression, &each_context),
                ),
                _ => return None,
            },
            _ => return None,
        };
        let source = self.ctx.query.component.source_text(bind.span).to_string();
        let (line, col) = self.ctx.state.line_index.line_col(bind.span.start);
        let object_body = if self.bind_each_item_store_backed(bind) {
            self.ctx
                .b
                .seq_expr([self.ctx.b.call_expr("$.mark_store_binding", []), object])
        } else {
            object
        };
        let object_thunk = self.ctx.b.thunk(object_body);
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
