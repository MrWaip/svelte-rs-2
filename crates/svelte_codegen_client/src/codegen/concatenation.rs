use svelte_ast_builder::{Arg, AssignLeft, TemplatePart};

use super::data_structures::{ConcatPart, EmitState, FragmentCtx, TemplateMemoState};
use super::fragment::role_needs_text_first_next;
use super::{Codegen, Result};

pub(in crate::codegen) enum ConcatenationAnchor {
    SiblingTextNode { node_var: String },
    SingleFragmentChild { parent_var: String },
    SingleFragmentRoot,
    SingleFragmentCallbackParam { append_inside: bool },
}

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(in crate::codegen) fn emit_concatenation(
        &mut self,
        state: &mut EmitState<'a>,
        ctx: &FragmentCtx<'a>,
        anchor: ConcatenationAnchor,
        parts: &[ConcatPart],
    ) -> Result<()> {
        let (tpl_parts, memo_deps, mut needs_effect, extra_blockers) =
            self.build_concatenation_parts(ctx, parts)?;
        if !extra_blockers.is_empty() {
            needs_effect = true;
        }
        let tpl_expr = self.assemble_concatenation_expr(tpl_parts);
        self.emit_concatenation_to_anchor(
            state,
            ctx,
            anchor,
            tpl_expr,
            needs_effect,
            memo_deps,
            extra_blockers,
        )
    }

    fn build_concatenation_parts(
        &mut self,
        ctx: &FragmentCtx<'a>,
        parts: &[ConcatPart],
    ) -> Result<(
        Vec<TemplatePart<'a>>,
        TemplateMemoState<'a>,
        bool,
        Vec<oxc_ast::ast::Expression<'a>>,
    )> {
        use svelte_analyze::{ExprKind, ExpressionSemantics, Memoization};

        let mut tpl_parts: Vec<TemplatePart<'a>> = Vec::with_capacity(parts.len());
        let mut memo_deps = TemplateMemoState::default();
        let mut needs_effect = false;
        let mut extra_blockers: Vec<oxc_ast::ast::Expression<'a>> = Vec::new();

        for part in parts {
            if let Some(s) = ctx.static_text_of(part) {
                if let Some(TemplatePart::Str(prev)) = tpl_parts.last_mut() {
                    prev.push_str(s);
                } else {
                    tpl_parts.push(TemplatePart::Str(s.to_string()));
                }
                continue;
            }
            let ConcatPart::Expr(id) = part else { continue };

            if let ExpressionSemantics::Expression(data) =
                self.ctx.query.view.expression_semantics(*id)
                && let ExprKind::Folded(s) = &data.kind
            {
                if let Some(TemplatePart::Str(prev)) = tpl_parts.last_mut() {
                    prev.push_str(s);
                } else {
                    tpl_parts.push(TemplatePart::Str(s.to_string()));
                }
                continue;
            }

            let expr = self.take_node_expr(*id)?;
            let defined = self.is_node_expr_definitely_defined(*id, &expr);
            let const_blockers = self.ctx.const_tag_blocker_exprs(*id);
            if !const_blockers.is_empty() {
                needs_effect = true;
                extra_blockers.extend(const_blockers);
            }
            let (effective_expr, part_needs_effect) =
                match self.ctx.query.view.expression_semantics(*id) {
                    ExpressionSemantics::NonSpecial => (expr, false),
                    ExpressionSemantics::Expression(data) => {
                        let part_needs_effect = data.is_dynamic();
                        let expr =
                            self.apply_legacy_wrap(expr, data.legacy_wrap, &data.references);
                        let effective = match data.memoization {
                            Memoization::None => expr,
                            Memoization::SyncMemo => {
                                memo_deps.push_node_deps(self.ctx, *id);
                                let cloned = self.ctx.b.clone_expr(&expr);
                                let index = memo_deps.sync_values_push(cloned);
                                memo_deps.sync_param_expr(self.ctx, index)
                            }
                            Memoization::AsyncMemo => {
                                memo_deps.push_node_deps(self.ctx, *id);
                                let cloned = self.ctx.b.clone_expr(&expr);
                                let index = memo_deps.async_values_push(cloned);
                                memo_deps.async_param_expr(self.ctx, index)
                            }
                        };
                        (effective, part_needs_effect)
                    }
                };
            if part_needs_effect {
                needs_effect = true;
            }
            tpl_parts.push(TemplatePart::Expr(effective_expr, defined));
        }

        Ok((tpl_parts, memo_deps, needs_effect, extra_blockers))
    }

    fn assemble_concatenation_expr(
        &self,
        tpl_parts: Vec<TemplatePart<'a>>,
    ) -> ConcatenationExpr<'a> {
        let b = &self.ctx.state.b;
        if tpl_parts.len() == 1 {
            match tpl_parts.into_iter().next() {
                Some(TemplatePart::Str(s)) => ConcatenationExpr::Static(b.str_expr(&s)),
                Some(TemplatePart::Expr(expr, _defined)) => ConcatenationExpr::BareExpr(expr),
                None => ConcatenationExpr::Template(b.template_parts_expr(Vec::new())),
            }
        } else {
            ConcatenationExpr::Template(b.template_parts_expr(tpl_parts))
        }
    }

    fn emit_concatenation_to_anchor(
        &mut self,
        state: &mut EmitState<'a>,
        ctx: &FragmentCtx<'a>,
        anchor: ConcatenationAnchor,
        tpl_expr: ConcatenationExpr<'a>,
        needs_effect: bool,
        mut memo_deps: TemplateMemoState<'a>,
        extra_blockers: Vec<oxc_ast::ast::Expression<'a>>,
    ) -> Result<()> {
        match anchor {
            ConcatenationAnchor::SiblingTextNode { node_var } => self.emit_to_sibling_text_node(
                state,
                &node_var,
                tpl_expr,
                needs_effect,
                memo_deps,
                extra_blockers,
            ),
            ConcatenationAnchor::SingleFragmentChild { parent_var } => self
                .emit_to_single_fragment_child(
                    state,
                    ctx,
                    &parent_var,
                    tpl_expr,
                    needs_effect,
                    &mut memo_deps,
                    extra_blockers,
                ),
            ConcatenationAnchor::SingleFragmentRoot => self.emit_to_single_fragment_root(
                state,
                ctx,
                tpl_expr,
                needs_effect,
                &mut memo_deps,
                extra_blockers,
            ),
            ConcatenationAnchor::SingleFragmentCallbackParam { append_inside } => self
                .emit_to_single_fragment_callback_param(
                    state,
                    ctx,
                    append_inside,
                    tpl_expr,
                    needs_effect,
                    &mut memo_deps,
                    extra_blockers,
                ),
        }
    }

    fn emit_to_sibling_text_node(
        &mut self,
        state: &mut EmitState<'a>,
        node_var: &str,
        tpl_expr: ConcatenationExpr<'a>,
        needs_effect: bool,
        mut memo_deps: TemplateMemoState<'a>,
        extra_blockers: Vec<oxc_ast::ast::Expression<'a>>,
    ) -> Result<()> {
        let b = &self.ctx.state.b;
        let final_expr = match tpl_expr {
            ConcatenationExpr::Static(e) => e,
            ConcatenationExpr::BareExpr(expr) => expr,
            ConcatenationExpr::Template(e) => e,
        };
        if needs_effect {
            if memo_deps.has_deps() {
                let param_names = memo_deps.param_names();
                let params = if param_names.is_empty() {
                    self.ctx.b.no_params()
                } else {
                    self.ctx.b.params(param_names.iter().map(|s| s.as_str()))
                };
                let set_text = self
                    .ctx
                    .b
                    .call_stmt("$.set_text", [Arg::Ident(node_var), Arg::Expr(final_expr)]);
                let callback = self.ctx.b.arrow_expr(params, [set_text]);
                crate::codegen::effect::emit_effect_call_extern(
                    self.ctx,
                    "$.template_effect",
                    callback,
                    &mut memo_deps,
                    &mut state.after_update,
                );
            } else {
                if !extra_blockers.is_empty() {
                    state.extra_blockers.extend(extra_blockers);
                }
                state.update.push(b.call_stmt(
                    "$.set_text",
                    [Arg::Ident(node_var), Arg::Expr(final_expr)],
                ));
            }
        } else {
            let member = b.static_member(b.rid_expr(node_var), "nodeValue");
            state
                .init
                .push(b.assign_stmt(AssignLeft::StaticMember(member), final_expr));
        }
        Ok(())
    }

    fn emit_to_single_fragment_child(
        &mut self,
        state: &mut EmitState<'a>,
        _ctx: &FragmentCtx<'a>,
        parent_var: &str,
        tpl_expr: ConcatenationExpr<'a>,
        needs_effect: bool,
        memo_deps: &mut TemplateMemoState<'a>,
        extra_blockers: Vec<oxc_ast::ast::Expression<'a>>,
    ) -> Result<()> {
        if !needs_effect {
            let b = &self.ctx.state.b;
            let final_expr = match tpl_expr {
                ConcatenationExpr::Static(e) => e,
                ConcatenationExpr::BareExpr(expr) => expr,
                ConcatenationExpr::Template(e) => e,
            };
            let member = b.static_member(b.rid_expr(parent_var), "textContent");
            state
                .init
                .push(b.assign_stmt(AssignLeft::StaticMember(member), final_expr));
            state.last_fragment_needs_reset = false;
            return Ok(());
        }

        let name = self.ctx.state.gen_ident("text");
        let is_bare = matches!(tpl_expr, ConcatenationExpr::BareExpr(_));
        let b = &self.ctx.state.b;
        state.template.push_text(" ");
        let child_args: Vec<Arg<'a, '_>> = if is_bare {
            vec![Arg::Ident(parent_var), Arg::Bool(true)]
        } else {
            vec![Arg::Ident(parent_var)]
        };
        state
            .init
            .push(b.var_stmt(&name, b.call_expr("$.child", child_args)));
        let final_expr = match tpl_expr {
            ConcatenationExpr::Static(e) => e,
            ConcatenationExpr::BareExpr(expr) => expr,
            ConcatenationExpr::Template(e) => e,
        };
        if !is_bare && state.bound_contenteditable {
            let b = &self.ctx.state.b;
            let member = b.static_member(b.rid_expr(&name), "nodeValue");
            state
                .init
                .push(b.assign_stmt(AssignLeft::StaticMember(member), final_expr));
            return Ok(());
        }
        if memo_deps.has_deps() {
            let param_names = memo_deps.param_names();
            let params = if param_names.is_empty() {
                self.ctx.b.no_params()
            } else {
                self.ctx.b.params(param_names.iter().map(|s| s.as_str()))
            };
            let set_text = self
                .ctx
                .b
                .call_stmt("$.set_text", [Arg::Ident(&name), Arg::Expr(final_expr)]);
            let callback = self.ctx.b.arrow_expr(params, [set_text]);
            crate::codegen::effect::emit_effect_call_extern(
                self.ctx,
                "$.template_effect",
                callback,
                memo_deps,
                &mut state.after_update,
            );
        } else {
            if !extra_blockers.is_empty() {
                state.extra_blockers.extend(extra_blockers);
            }
            let b = &self.ctx.state.b;
            state.update.push(b.call_stmt(
                "$.set_text",
                [Arg::Ident(&name), Arg::Expr(final_expr)],
            ));
        }
        Ok(())
    }

    fn emit_to_single_fragment_root(
        &mut self,
        state: &mut EmitState<'a>,
        ctx: &FragmentCtx<'a>,
        tpl_expr: ConcatenationExpr<'a>,
        needs_effect: bool,
        memo_deps: &mut TemplateMemoState<'a>,
        extra_blockers: Vec<oxc_ast::ast::Expression<'a>>,
    ) -> Result<()> {
        let name = self.ctx.state.gen_ident("text");
        let b = &self.ctx.state.b;

        if role_needs_text_first_next(ctx.role) {
            state
                .init
                .push(b.call_stmt("$.next", std::iter::empty::<Arg<'a, '_>>()));
        }
        state.init.push(b.var_stmt(
            &name,
            b.call_expr("$.text", std::iter::empty::<Arg<'a, '_>>()),
        ));
        state.root_var = Some(name.clone());

        self.finalize_text_node_emission(state, &name, tpl_expr, needs_effect, memo_deps, extra_blockers)
    }

    fn emit_to_single_fragment_callback_param(
        &mut self,
        state: &mut EmitState<'a>,
        ctx: &FragmentCtx<'a>,
        append_inside: bool,
        tpl_expr: ConcatenationExpr<'a>,
        needs_effect: bool,
        memo_deps: &mut TemplateMemoState<'a>,
        extra_blockers: Vec<oxc_ast::ast::Expression<'a>>,
    ) -> Result<()> {
        let name = self.ctx.state.gen_ident("text");
        let b = &self.ctx.state.b;
        if !append_inside && role_needs_text_first_next(ctx.role) {
            state
                .init
                .push(b.call_stmt("$.next", std::iter::empty::<Arg<'a, '_>>()));
        }
        state.init.push(b.var_stmt(
            &name,
            b.call_expr("$.text", std::iter::empty::<Arg<'a, '_>>()),
        ));
        state.root_var = Some(name.clone());
        self.finalize_text_node_emission(state, &name, tpl_expr, needs_effect, memo_deps, extra_blockers)
    }

    fn finalize_text_node_emission(
        &mut self,
        state: &mut EmitState<'a>,
        node_var: &str,
        tpl_expr: ConcatenationExpr<'a>,
        needs_effect: bool,
        memo_deps: &mut TemplateMemoState<'a>,
        extra_blockers: Vec<oxc_ast::ast::Expression<'a>>,
    ) -> Result<()> {
        let final_expr = match tpl_expr {
            ConcatenationExpr::Static(e) => e,
            ConcatenationExpr::BareExpr(expr) => expr,
            ConcatenationExpr::Template(e) => e,
        };
        if needs_effect {
            if memo_deps.has_deps() {
                let param_names = memo_deps.param_names();
                let params = if param_names.is_empty() {
                    self.ctx.b.no_params()
                } else {
                    self.ctx.b.params(param_names.iter().map(|s| s.as_str()))
                };
                let set_text = self.ctx.b.call_stmt(
                    "$.set_text",
                    [Arg::Ident(node_var), Arg::Expr(final_expr)],
                );
                let callback = self.ctx.b.arrow_expr(params, [set_text]);
                crate::codegen::effect::emit_effect_call_extern(
                    self.ctx,
                    "$.template_effect",
                    callback,
                    memo_deps,
                    &mut state.after_update,
                );
            } else {
                if !extra_blockers.is_empty() {
                    state.extra_blockers.extend(extra_blockers);
                }
                let b = &self.ctx.state.b;
                state.update.push(b.call_stmt(
                    "$.set_text",
                    [Arg::Ident(node_var), Arg::Expr(final_expr)],
                ));
            }
        } else {
            let b = &self.ctx.state.b;
            let member = b.static_member(b.rid_expr(node_var), "nodeValue");
            state
                .init
                .push(b.assign_stmt(AssignLeft::StaticMember(member), final_expr));
        }
        Ok(())
    }
}

enum ConcatenationExpr<'a> {
    Static(oxc_ast::ast::Expression<'a>),
    BareExpr(oxc_ast::ast::Expression<'a>),
    Template(oxc_ast::ast::Expression<'a>),
}
