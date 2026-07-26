use crate::codegen::expr::coarse_wrap;
use svelte_analyze::block_semantics::{HtmlTagNamespace, HtmlTagSemantics};
use svelte_ast::NodeId;
use svelte_ast_builder::Arg;
use svelte_emit_builders::runes::rune_get;

use super::super::data_structures::AsyncEmission;
use super::super::data_structures::EmitState;
use super::super::data_structures::{FragmentAnchor, FragmentCtx};
use super::super::effect::suspending_block_thunk;
use super::super::{Codegen, Result};
use crate::context::Ctx;

fn push_html_trailing_args<'a, 'b>(
    ctx: &'b Ctx<'a>,
    args: &mut Vec<Arg<'a, 'b>>,
    is_controlled: bool,
    is_svg: bool,
    is_mathml: bool,
    hydration_ignored: bool,
) {
    let flags = [is_controlled, is_svg, is_mathml, hydration_ignored];
    let last_set = flags.iter().rposition(|&f| f);
    let Some(last) = last_set else {
        return;
    };
    for &flag in &flags[..=last] {
        args.push(if flag {
            Arg::Bool(true)
        } else {
            Arg::Expr(ctx.b.void_zero_expr())
        });
    }
}

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(super) fn emit_html_tag(
        &mut self,
        state: &mut EmitState<'a>,
        ctx: &FragmentCtx<'a>,
        id: NodeId,
        sem: HtmlTagSemantics,
    ) -> Result<()> {
        let (anchor_name, is_controlled) = match &ctx.anchor {
            FragmentAnchor::Child { parent_var }
            | FragmentAnchor::ElementContentChild { parent_var } => (parent_var.to_string(), true),
            _ => (self.comment_anchor_node_name(state, ctx)?, false),
        };

        let is_svg = !is_controlled && matches!(sem.parent_strategy, HtmlTagNamespace::Svg);
        let is_mathml = !is_controlled && matches!(sem.parent_strategy, HtmlTagNamespace::MathMl);
        let hydration_ignored = sem.hydration_html_changed_ignored;

        let plan = AsyncEmission::for_node(self.ctx, id);

        match &plan {
            AsyncEmission::Awaited { blockers } | AsyncEmission::Deferred { blockers } => {
                let blockers = blockers.to_vec();
                let suspension = self.ctx.expression_suspension(id);
                let expression = self.take_node_expr(id)?;
                let async_thunk = match &plan {
                    AsyncEmission::Awaited { .. } => {
                        Some(suspending_block_thunk(self.ctx, expression, suspension))
                    }
                    AsyncEmission::Deferred { .. } | AsyncEmission::Sync => None,
                };

                let html_value = self.ctx.b.thunk(rune_get(&self.ctx.b, "$$html"));
                let mut html_args: Vec<Arg<'a, '_>> =
                    vec![Arg::Ident("node"), Arg::Expr(html_value)];

                push_html_trailing_args(
                    self.ctx,
                    &mut html_args,
                    is_controlled,
                    is_svg,
                    is_mathml,
                    hydration_ignored,
                );

                let html_stmt = self.ctx.b.call_stmt("$.html", html_args);

                let anchor_expr = self.ctx.b.rid_expr(&anchor_name);
                let async_stmt = self.emit_async_call_stmt(
                    &blockers,
                    anchor_expr,
                    "node",
                    "$$html",
                    async_thunk,
                    vec![html_stmt],
                )?;
                state.init.push(async_stmt);

                if is_controlled {
                    state.last_fragment_needs_reset = false;
                }
                Ok(())
            }
            AsyncEmission::Sync => {
                let expr = self.take_node_expr(id)?;
                let expr = coarse_wrap(self.ctx, expr, self.ctx.expression_data(id));
                let thunk = self.ctx.b.thunk(expr);

                let mut args: Vec<Arg<'a, '_>> = vec![Arg::Ident(&anchor_name), Arg::Expr(thunk)];

                push_html_trailing_args(
                    self.ctx,
                    &mut args,
                    is_controlled,
                    is_svg,
                    is_mathml,
                    hydration_ignored,
                );

                state.init.push(self.ctx.b.call_stmt("$.html", args));

                if is_controlled {
                    state.last_fragment_needs_reset = false;
                }

                Ok(())
            }
        }
    }
}
