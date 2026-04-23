use oxc_ast::ast::Expression;
use svelte_analyze::ReferenceSemantics;
use svelte_ast::BindDirective;
use svelte_ast_builder::Arg;

use super::super::super::{Codegen, CodegenError, Result};

use super::placement::BindPlacement;

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(super) fn emit_bind_this(
        &mut self,
        bind: &BindDirective,
        el_name: &str,
        tag_name: &str,
    ) -> Result<Option<BindPlacement<'a>>> {
        let is_rune = matches!(
            self.ctx.directive_root_reference_semantics(bind.id),
            ReferenceSemantics::SignalWrite { .. }
                | ReferenceSemantics::SignalUpdate { .. }
                | ReferenceSemantics::SignalRead { .. }
        );
        let var_name = if bind.shorthand {
            bind.name.clone()
        } else {
            self.ctx
                .query
                .component
                .source_text(bind.expression_span)
                .to_string()
        };

        if !bind.shorthand {
            let expr = self.take_attr_expr(bind.id)?;
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

        let setter = self
            .ctx
            .b
            .bind_this_setter_arrow(&var_name, is_rune, tag_name.is_empty());
        let getter = self.ctx.b.bind_this_getter_arrow(&var_name, is_rune);
        let stmt = self.ctx.b.call_stmt(
            "$.bind_this",
            [Arg::Ident(el_name), Arg::Expr(setter), Arg::Expr(getter)],
        );
        Ok(Some(BindPlacement::Init(stmt)))
    }
}
