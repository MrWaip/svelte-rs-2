use std::iter;

use oxc_ast::ast::Expression;
use svelte_analyze::{AttributeSemantics, HtmlBindKind};
use svelte_ast::BindDirective;
use svelte_ast_builder::Arg;

use super::super::super::{Codegen, CodegenError, Result};
use crate::codegen::expr::build_reactive_dep_expr_legacy;

use super::placement::BindPlacement;

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(super) fn emit_bind_this(
        &mut self,
        bind: &BindDirective,
        el_name: &str,
        _tag_name: &str,
    ) -> Result<Option<BindPlacement<'a>>> {
        let element_bind = match self.ctx.query.analysis.attributes.get(bind.id) {
            AttributeSemantics::ElementBind(b) => Some(b.clone()),
            _ => None,
        };

        if !bind.shorthand {
            let expr = self.take_attr_expr(bind.id, &bind.expression)?;
            if let Expression::SequenceExpression(seq) = expr {
                let seq = seq.unbox();
                let mut exprs = seq.expressions.into_iter();
                let Some(get_expr) = exprs.next() else {
                    return CodegenError::unexpected_node(
                        bind.id,
                        "bind:this SequenceExpression missing getter",
                    );
                };
                let Some(set_expr) = exprs.next() else {
                    return CodegenError::unexpected_node(
                        bind.id,
                        "bind:this SequenceExpression missing setter",
                    );
                };

                let each_context: Vec<oxc_semantic::SymbolId> = element_bind
                    .as_ref()
                    .map(|b| b.each_context_vars.iter().copied().collect())
                    .unwrap_or_default();
                let (setter, getter, dep) =
                    self.build_bind_this_get_set(get_expr, set_expr, &each_context);
                let mut args = vec![Arg::Ident(el_name), Arg::Expr(setter), Arg::Expr(getter)];
                if let Some(dep) = dep {
                    args.push(Arg::Expr(dep));
                }
                let stmt = self.ctx.b.call_stmt("$.bind_this", args);
                return Ok(Some(BindPlacement::Init(stmt)));
            }
        }

        let store_base = match element_bind.as_ref().map(|b| &b.kind) {
            Some(HtmlBindKind::StoreSubscribed { base_symbol }) => Some(*base_symbol),
            _ => None,
        };
        let Some(sym) = store_base else {
            return CodegenError::unexpected_node(
                bind.id,
                "bind:this off the reactive transform path must be store-subscribed",
            );
        };
        let base_name_owned = self.ctx.symbol_name(sym).to_string();
        let dollar_name_owned = format!("${base_name_owned}");
        let base_name = self.ctx.b.alloc_str(&base_name_owned);
        let dollar_alloc = self.ctx.b.alloc_str(&dollar_name_owned);
        let setter_body = self.ctx.b.call_expr(
            "$.store_set",
            [Arg::Ident(base_name), Arg::Ident("$$value")],
        );
        let setter = self.ctx.b.arrow_expr(
            self.ctx.b.params(["$$value"]),
            [self.ctx.b.expr_stmt(setter_body)],
        );
        let getter_body = self
            .ctx
            .b
            .call_expr_callee(self.ctx.b.rid_expr(dollar_alloc), iter::empty::<Arg>());
        let getter = self
            .ctx
            .b
            .arrow_expr(self.ctx.b.no_params(), [self.ctx.b.expr_stmt(getter_body)]);
        let stmt = self.ctx.b.call_stmt(
            "$.bind_this",
            [Arg::Ident(el_name), Arg::Expr(setter), Arg::Expr(getter)],
        );
        Ok(Some(BindPlacement::Init(stmt)))
    }

    pub(in crate::codegen) fn build_bind_this_get_set(
        &mut self,
        get_expr: Expression<'a>,
        set_expr: Expression<'a>,
        each_context: &[oxc_semantic::SymbolId],
    ) -> (Expression<'a>, Expression<'a>, Option<Expression<'a>>) {
        if each_context.is_empty() {
            let getter = self.ctx.b.rewrap_arrow_body(get_expr);
            let setter = self.ctx.b.rewrap_arrow_body_with_first_param(set_expr);
            return (setter, getter, None);
        }

        let each_names: Vec<String> = each_context
            .iter()
            .map(|&sym| self.ctx.symbol_name(sym).to_string())
            .collect();

        let getter = self
            .ctx
            .b
            .rewrap_arrow_with_params(get_expr, each_names.iter());

        let mut setter_params: Vec<String> = Vec::with_capacity(each_names.len() + 1);
        setter_params.push("$$value".to_string());
        setter_params.extend(each_names.iter().cloned());
        let setter = self
            .ctx
            .b
            .rewrap_arrow_with_params(set_expr, setter_params.iter());

        let dep_args: Vec<Arg<'_, '_>> = each_context
            .iter()
            .map(|&sym| {
                let expr = build_reactive_dep_expr_legacy(self.ctx, sym)
                    .unwrap_or_else(|| self.ctx.b.rid_expr(self.ctx.symbol_name(sym)));
                Arg::Expr(expr)
            })
            .collect();
        let dep_array = self.ctx.b.array_from_args(dep_args);
        let dep_thunk = self.ctx.b.thunk(dep_array);

        (setter, getter, Some(dep_thunk))
    }
}
