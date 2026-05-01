use oxc_ast::ast::Expression;
use svelte_ast::{Attribute, NodeId};
use svelte_ast_builder::{Arg, AssignLeft};

use super::super::{Codegen, CodegenError, Result};

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(in super::super) fn build_bind_this_call(
        &mut self,
        el_id: NodeId,
        bind_id: NodeId,
        value: Expression<'a>,
    ) -> Result<Expression<'a>> {
        let Some(view) = self
            .ctx
            .query
            .component
            .store
            .get(el_id)
            .as_component_like()
        else {
            return CodegenError::unexpected_node(el_id, "component-like");
        };
        let Some(bind) = view.attributes.iter().find_map(|a| {
            if let Attribute::BindDirective(b) = a {
                if b.id == bind_id { Some(b) } else { None }
            } else {
                None
            }
        }) else {
            return CodegenError::unexpected_node(el_id, "bind:this attribute must exist");
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

        let bind_attr = Attribute::BindDirective(bind.clone());
        let is_rune = matches!(
            self.ctx.directive_root_reference_semantics(&bind_attr),
            svelte_analyze::ReferenceSemantics::SignalWrite { .. }
                | svelte_analyze::ReferenceSemantics::SignalUpdate { .. }
                | svelte_analyze::ReferenceSemantics::SignalRead { .. }
        );

        let _ = self.ctx.state.parsed.take_expr(bind.expression.id());

        if !svelte_analyze::is_simple_identifier(&var_name) {
            let each_context_syms = self.ctx.bind_each_context(bind_id).unwrap_or(&[]).to_vec();
            let each_context: Vec<String> = each_context_syms
                .iter()
                .map(|&sym| self.ctx.symbol_name(sym).to_string())
                .collect();

            let setter_body = format!("{var_name} = $$value");
            let setter_expr = self.ctx.b.parse_expression(&setter_body);
            let mut setter_params: Vec<&str> = vec!["$$value"];
            for v in &each_context {
                setter_params.push(v);
            }
            let setter = self.ctx.b.arrow_expr(
                self.ctx.b.params(setter_params),
                [self.ctx.b.expr_stmt(setter_expr)],
            );

            let getter_expr = self.ctx.b.parse_expression(&var_name);
            let getter_expr = self.ctx.b.make_optional_chain(getter_expr);
            let mut getter_params: Vec<&str> = Vec::new();
            for v in &each_context {
                getter_params.push(v);
            }
            let getter = self.ctx.b.arrow_expr(
                self.ctx.b.params(getter_params),
                [self.ctx.b.expr_stmt(getter_expr)],
            );

            if each_context.is_empty() {
                return Ok(self.ctx.b.call_expr(
                    "$.bind_this",
                    [Arg::Expr(value), Arg::Expr(setter), Arg::Expr(getter)],
                ));
            } else {
                let context_values: Vec<Arg<'_, '_>> = each_context
                    .iter()
                    .map(|v| {
                        let s = self.ctx.b.alloc_str(v);
                        Arg::Ident(s)
                    })
                    .collect();
                let context_array = self.ctx.b.array_from_args(context_values);
                let context_thunk = self.ctx.b.thunk(context_array);

                return Ok(self.ctx.b.call_expr(
                    "$.bind_this",
                    [
                        Arg::Expr(value),
                        Arg::Expr(setter),
                        Arg::Expr(getter),
                        Arg::Expr(context_thunk),
                    ],
                ));
            }
        }

        let expr_text = self.ctx.b.alloc_str(&var_name);

        let setter = if is_rune {
            let body = self.ctx.b.call_expr(
                "$.set",
                [
                    Arg::Ident(expr_text),
                    Arg::Ident("$$value"),
                    Arg::Expr(self.ctx.b.bool_expr(true)),
                ],
            );
            self.ctx
                .b
                .arrow_expr(self.ctx.b.params(["$$value"]), [self.ctx.b.expr_stmt(body)])
        } else {
            let body = self
                .ctx
                .b
                .assign_expr(AssignLeft::Ident(var_name), self.ctx.b.rid_expr("$$value"));
            self.ctx
                .b
                .arrow_expr(self.ctx.b.params(["$$value"]), [self.ctx.b.expr_stmt(body)])
        };

        let getter = if is_rune {
            let body = self.ctx.b.call_expr("$.get", [Arg::Ident(expr_text)]);
            self.ctx
                .b
                .arrow_expr(self.ctx.b.no_params(), [self.ctx.b.expr_stmt(body)])
        } else {
            let body = self.ctx.b.rid_expr(expr_text);
            self.ctx
                .b
                .arrow_expr(self.ctx.b.no_params(), [self.ctx.b.expr_stmt(body)])
        };

        Ok(self.ctx.b.call_expr(
            "$.bind_this",
            [Arg::Expr(value), Arg::Expr(setter), Arg::Expr(getter)],
        ))
    }
}
