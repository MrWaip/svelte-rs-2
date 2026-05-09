use oxc_ast::ast::Expression;
use svelte_analyze::{AttributeSemantics, HtmlBindKind};
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
        let (is_rune, store_base) = match self.ctx.query.analysis.attributes.get(bind.id) {
            AttributeSemantics::ElementBind(b) => match &b.kind {
                HtmlBindKind::Rune => (true, None),
                HtmlBindKind::StoreSubscribed { base_symbol } => (false, Some(*base_symbol)),
                HtmlBindKind::Plain | HtmlBindKind::BindableProp => (false, None),
            },
            _ => (false, None),
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

        let (setter, getter) = if let Some(sym) = store_base {
            let base_name_owned = self.ctx.symbol_name(sym).to_string();
            let dollar_name_owned = format!("${base_name_owned}");
            let base_name = self.ctx.b.alloc_str(&base_name_owned);
            let dollar_alloc = self.ctx.b.alloc_str(&dollar_name_owned);
            let setter_body = self.ctx.b.call_expr(
                "$.store_set",
                [Arg::Ident(base_name), Arg::Ident("$$value")],
            );
            let setter = self
                .ctx
                .b
                .arrow_expr(self.ctx.b.params(["$$value"]), [self.ctx.b.expr_stmt(setter_body)]);
            let getter_body = self
                .ctx
                .b
                .call_expr_callee(self.ctx.b.rid_expr(dollar_alloc), std::iter::empty::<Arg>());
            let getter = self
                .ctx
                .b
                .arrow_expr(self.ctx.b.no_params(), [self.ctx.b.expr_stmt(getter_body)]);
            (setter, getter)
        } else {
            (
                self.ctx
                    .b
                    .bind_this_setter_arrow(&var_name, is_rune, tag_name.is_empty()),
                self.ctx.b.bind_this_getter_arrow(&var_name, is_rune),
            )
        };
        let stmt = self.ctx.b.call_stmt(
            "$.bind_this",
            [Arg::Ident(el_name), Arg::Expr(setter), Arg::Expr(getter)],
        );
        Ok(Some(BindPlacement::Init(stmt)))
    }
}
