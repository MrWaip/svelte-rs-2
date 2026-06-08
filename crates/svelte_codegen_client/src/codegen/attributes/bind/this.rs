use std::iter;

use oxc_ast::ast::{Expression, Statement};
use svelte_analyze::{AttributeSemantics, ElementBindSemantics, HtmlBindKind};
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
        tag_name: &str,
    ) -> Result<Option<BindPlacement<'a>>> {
        let element_bind = match self.ctx.query.analysis.attributes.get(bind.id) {
            AttributeSemantics::ElementBind(b) => Some(b.clone()),
            _ => None,
        };

        let var_name = if bind.shorthand {
            bind.name.clone()
        } else {
            self.ctx
                .query
                .component
                .source_text(bind.expression.span)
                .to_string()
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

                let get_arrow = self.ctx.b.rewrap_arrow_body(get_expr);
                let set_arrow = self.ctx.b.rewrap_arrow_body_with_first_param(set_expr);

                let stmt = self.ctx.b.call_stmt(
                    "$.bind_this",
                    [
                        Arg::Ident(el_name),
                        Arg::Expr(set_arrow),
                        Arg::Expr(get_arrow),
                    ],
                );
                return Ok(Some(BindPlacement::Init(stmt)));
            }
        }

        if let Some(stmt) =
            self.try_emit_bind_this_each_reactive_member(&var_name, el_name, element_bind.as_ref())
        {
            return Ok(Some(BindPlacement::Init(stmt)));
        }

        let kind = element_bind.as_ref().map(|b| b.kind.clone());
        let store_base = match &kind {
            Some(HtmlBindKind::StoreSubscribed { base_symbol }) => Some(*base_symbol),
            _ => None,
        };
        let (setter, getter) = if let Some(sym) = store_base {
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
            (setter, getter)
        } else {
            match kind {
                Some(HtmlBindKind::BindableProp) => {
                    let name = self.ctx.b.alloc_str(&var_name);
                    let setter_body = self.ctx.b.call_expr(name, [Arg::Ident("$$value")]);
                    let setter = self.ctx.b.arrow_expr(
                        self.ctx.b.params(["$$value"]),
                        [self.ctx.b.expr_stmt(setter_body)],
                    );
                    let getter_body = self.ctx.b.call_expr(name, iter::empty::<Arg>());
                    let getter = self
                        .ctx
                        .b
                        .arrow_expr(self.ctx.b.no_params(), [self.ctx.b.expr_stmt(getter_body)]);
                    (setter, getter)
                }
                Some(HtmlBindKind::Rune) | Some(HtmlBindKind::LegacyState) => (
                    self.ctx
                        .b
                        .bind_this_setter_arrow(&var_name, true, tag_name.is_empty()),
                    self.ctx.b.bind_this_getter_arrow(&var_name, true),
                ),
                _ => (
                    self.ctx
                        .b
                        .bind_this_setter_arrow(&var_name, false, tag_name.is_empty()),
                    self.ctx.b.bind_this_getter_arrow(&var_name, false),
                ),
            }
        };
        let stmt = self.ctx.b.call_stmt(
            "$.bind_this",
            [Arg::Ident(el_name), Arg::Expr(setter), Arg::Expr(getter)],
        );
        Ok(Some(BindPlacement::Init(stmt)))
    }

    fn try_emit_bind_this_each_reactive_member(
        &mut self,
        var_name: &str,
        el_name: &str,
        element_bind: Option<&ElementBindSemantics>,
    ) -> Option<Statement<'a>> {
        let bind = element_bind?;
        if !matches!(bind.kind, HtmlBindKind::LegacyState) {
            return None;
        }
        if bind.each_context_vars.is_empty() {
            return None;
        }
        let root_len = root_ident_len(var_name);
        if root_len == 0 || root_len == var_name.len() {
            return None;
        }
        let root_name = &var_name[..root_len];
        let suffix = &var_name[root_len..];

        let each_context: Vec<String> = bind
            .each_context_vars
            .iter()
            .map(|&sym| self.ctx.symbol_name(sym).to_string())
            .collect();

        let assign_text = format!("$.get({root_name}){suffix} = $$value");
        let assign_expr = self.ctx.b.parse_expression(&assign_text);
        let root_alloc = self.ctx.b.alloc_str(root_name);
        let setter_body = self
            .ctx
            .b
            .call_expr("$.mutate", [Arg::Ident(root_alloc), Arg::Expr(assign_expr)]);
        let mut setter_params: Vec<&str> = vec!["$$value"];
        for v in &each_context {
            setter_params.push(v.as_str());
        }
        let setter = self.ctx.b.arrow_expr(
            self.ctx.b.params(setter_params),
            [self.ctx.b.expr_stmt(setter_body)],
        );

        let getter_text = format!("$.get({root_name}){suffix}");
        let getter_expr = self.ctx.b.parse_expression(&getter_text);
        let getter_expr = self.ctx.b.make_optional_chain(getter_expr);
        let getter_params: Vec<&str> = each_context.iter().map(String::as_str).collect();
        let getter = self.ctx.b.arrow_expr(
            self.ctx.b.params(getter_params),
            [self.ctx.b.expr_stmt(getter_expr)],
        );

        let dep_args: Vec<Arg<'_, '_>> = bind
            .each_context_vars
            .iter()
            .map(|&sym| {
                let expr = build_reactive_dep_expr_legacy(self.ctx, sym)
                    .unwrap_or_else(|| self.ctx.b.rid_expr(self.ctx.symbol_name(sym)));
                Arg::Expr(expr)
            })
            .collect();
        let dep_array = self.ctx.b.array_from_args(dep_args);
        let dep_thunk = self.ctx.b.thunk(dep_array);

        Some(self.ctx.b.call_stmt(
            "$.bind_this",
            [
                Arg::Ident(el_name),
                Arg::Expr(setter),
                Arg::Expr(getter),
                Arg::Expr(dep_thunk),
            ],
        ))
    }
}

fn root_ident_len(s: &str) -> usize {
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        let is_ident_char = b.is_ascii_alphanumeric() || b == b'_' || b == b'$';
        if !is_ident_char {
            break;
        }
        i += 1;
    }
    i
}
