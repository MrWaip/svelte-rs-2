use svelte_ast::NodeId;
use svelte_ast_builder::Arg;

use super::super::data_structures::AsyncEmissionPlan;
use super::super::data_structures::EmitState;
use super::super::data_structures::{FragmentAnchor, FragmentCtx};
use super::super::{Codegen, Result};

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(super) fn emit_html_tag(
        &mut self,
        state: &mut EmitState<'a>,
        ctx: &FragmentCtx<'a>,
        id: NodeId,
    ) -> Result<()> {
        let (anchor_name, is_controlled) = match &ctx.anchor {
            FragmentAnchor::Child { parent_var } => (parent_var.clone(), true),
            _ => (self.comment_anchor_node_name(state, ctx)?, false),
        };

        let is_svg = !is_controlled && self.ctx.query.view.html_tag_in_svg(id);
        let is_mathml = !is_controlled && self.ctx.query.view.html_tag_in_mathml(id);

        let plan = AsyncEmissionPlan::for_node(self.ctx, id);

        if plan.needs_async() {
            let expression = self.take_node_expr(id)?;

            let html_value = self
                .ctx
                .b
                .thunk(self.ctx.b.call_expr("$.get", [Arg::Ident("$$html")]));
            let mut html_args: Vec<Arg<'a, '_>> = vec![Arg::Ident("node"), Arg::Expr(html_value)];

            if is_controlled || is_svg || is_mathml {
                html_args.push(if is_controlled {
                    Arg::Bool(true)
                } else {
                    Arg::Expr(self.ctx.b.void_zero_expr())
                });
            }
            if is_svg || is_mathml {
                html_args.push(if is_svg {
                    Arg::Bool(true)
                } else {
                    Arg::Expr(self.ctx.b.void_zero_expr())
                });
            }
            if is_mathml {
                html_args.push(Arg::Bool(true));
            }

            let html_stmt = self.ctx.b.call_stmt("$.html", html_args);

            let anchor_expr = self.ctx.b.rid_expr(&anchor_name);
            let async_thunk = plan.async_thunk(self.ctx, expression);
            let async_stmt = plan.emit_async_call_stmt(
                self.ctx,
                anchor_expr,
                "node",
                "$$html",
                async_thunk,
                vec![html_stmt],
            );
            state.init.push(async_stmt);

            if is_controlled {
                state.last_fragment_needs_reset = false;
            }
            return Ok(());
        }

        let expr = self.take_node_expr(id)?;
        let thunk = self.ctx.b.thunk(expr);

        let mut args: Vec<Arg<'a, '_>> = vec![Arg::Ident(&anchor_name), Arg::Expr(thunk)];

        if is_controlled || is_svg || is_mathml {
            args.push(if is_controlled {
                Arg::Bool(true)
            } else {
                Arg::Expr(self.ctx.b.void_zero_expr())
            });
        }
        if is_svg || is_mathml {
            args.push(if is_svg {
                Arg::Bool(true)
            } else {
                Arg::Expr(self.ctx.b.void_zero_expr())
            });
        }
        if is_mathml {
            args.push(Arg::Bool(true));
        }

        state.init.push(self.ctx.b.call_stmt("$.html", args));

        if is_controlled {
            state.last_fragment_needs_reset = false;
        }

        Ok(())
    }
}
