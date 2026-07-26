use compact_str::CompactString;
use oxc_ast::ast::Expression;
use std::iter::empty;
use svelte_ast_builder::{Arg, AssignLeft, TemplatePart};
use svelte_emit_builders::legacy_wrap;

use super::data_structures::{ConcatPart, EmitState, FragmentCtx, TemplateMemoState};
use super::expr::legacy_dep_expr;
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
        let shared_sync_before = state.shared_memo.sync_values.len();
        let shared_async_before = state.shared_memo.async_values.len();
        let (tpl_parts, mut needs_effect, extra_blockers) =
            self.build_concatenation_parts(ctx, &mut state.shared_memo, parts)?;
        if !extra_blockers.is_empty() {
            needs_effect = true;
        }
        let has_memo = state.shared_memo.sync_values.len() > shared_sync_before
            || state.shared_memo.async_values.len() > shared_async_before;
        let tpl_expr = self.assemble_concatenation_expr(tpl_parts);
        self.emit_concatenation_to_anchor(
            state,
            ctx,
            anchor,
            tpl_expr,
            needs_effect,
            has_memo,
            extra_blockers,
        )
    }

    fn build_concatenation_parts(
        &mut self,
        ctx: &FragmentCtx<'a>,
        memo_deps: &mut TemplateMemoState<'a>,
        parts: &[ConcatPart],
    ) -> Result<(Vec<TemplatePart<'a>>, bool, Vec<(String, usize)>)> {
        use svelte_analyze::ExpressionSemantics;

        let mut tpl_parts: Vec<TemplatePart<'a>> = Vec::with_capacity(parts.len());
        let mut needs_effect = false;
        let mut extra_blockers: Vec<(String, usize)> = Vec::new();

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
                && let Some(s) = data.evaluation.known_str()
            {
                if let Some(TemplatePart::Str(prev)) = tpl_parts.last_mut() {
                    prev.push_str(&s);
                } else {
                    tpl_parts.push(TemplatePart::Str(s));
                }
                continue;
            }

            let expr = self.take_node_expr(*id)?;
            use svelte_analyze::{Evaluation, Volatility};
            let source_defined = match self.ctx.query.view.expression_semantics(*id) {
                ExpressionSemantics::Expression(data) => match data.evaluation {
                    Evaluation::Known(_) | Evaluation::Defined { .. } => true,
                    Evaluation::MaybeNullish { .. } => false,
                },
                ExpressionSemantics::NonSpecial => false,
            };
            let const_blockers = self.ctx.const_tag_blocker_slots(*id);
            if !const_blockers.is_empty() {
                needs_effect = true;
                super::data_structures::extend_blocker_slots(&mut extra_blockers, const_blockers);
            }
            let plain_part = |expr: Expression<'a>| {
                let is_sequence = matches!(expr, Expression::SequenceExpression(_));
                (expr, source_defined && !is_sequence)
            };
            let (effective_expr, defined) = match self.ctx.query.view.expression_semantics(*id) {
                ExpressionSemantics::NonSpecial => plain_part(expr),
                ExpressionSemantics::Expression(data) => {
                    let expr = legacy_wrap::apply(
                        &self.ctx.b,
                        expr,
                        data.legacy_wrap,
                        &data.references,
                        |sym| legacy_dep_expr(self.ctx, sym),
                    );
                    let suspension = data.suspension;
                    match data.volatility {
                        Volatility::Heavy => {
                            memo_deps.push_node_deps(self.ctx, *id);
                            let cloned = self.ctx.b.clone_expr(&expr);
                            let index = memo_deps.sync_values_push(cloned);
                            needs_effect = true;
                            (memo_deps.sync_param_expr(self.ctx, index), false)
                        }
                        Volatility::Asynchronous => {
                            memo_deps.push_node_deps(self.ctx, *id);
                            let cloned = self.ctx.b.clone_expr(&expr);
                            let index = memo_deps.async_values_push(cloned, suspension);
                            needs_effect = true;
                            (memo_deps.async_param_expr(self.ctx, index), false)
                        }
                        Volatility::Reactive => {
                            needs_effect = true;
                            plain_part(expr)
                        }
                        Volatility::Static => plain_part(expr),
                    }
                }
            };
            tpl_parts.push(TemplatePart::Expr(effective_expr, defined));
        }

        Ok((tpl_parts, needs_effect, extra_blockers))
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
        has_memo: bool,
        extra_blockers: Vec<(String, usize)>,
    ) -> Result<()> {
        match anchor {
            ConcatenationAnchor::SiblingTextNode { node_var } => self.emit_to_sibling_text_node(
                state,
                &node_var,
                tpl_expr,
                needs_effect,
                has_memo,
                extra_blockers,
            ),
            ConcatenationAnchor::SingleFragmentChild { parent_var } => self
                .emit_to_single_fragment_child(
                    state,
                    ctx,
                    &parent_var,
                    tpl_expr,
                    needs_effect,
                    has_memo,
                    extra_blockers,
                ),
            ConcatenationAnchor::SingleFragmentRoot => self.emit_to_single_fragment_root(
                state,
                ctx,
                tpl_expr,
                needs_effect,
                has_memo,
                extra_blockers,
            ),
            ConcatenationAnchor::SingleFragmentCallbackParam { append_inside } => self
                .emit_to_single_fragment_callback_param(
                    state,
                    ctx,
                    append_inside,
                    tpl_expr,
                    needs_effect,
                    has_memo,
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
        _has_memo: bool,
        extra_blockers: Vec<(String, usize)>,
    ) -> Result<()> {
        let b = &self.ctx.state.b;
        let final_expr = match tpl_expr {
            ConcatenationExpr::Static(e) => e,
            ConcatenationExpr::BareExpr(expr) => expr,
            ConcatenationExpr::Template(e) => e,
        };
        if needs_effect {
            if !extra_blockers.is_empty() {
                super::data_structures::extend_blocker_slots(
                    &mut state.extra_blockers,
                    extra_blockers,
                );
            }
            state
                .update
                .push(b.call_stmt("$.set_text", [Arg::Ident(node_var), Arg::Expr(final_expr)]));
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
        _has_memo: bool,
        extra_blockers: Vec<(String, usize)>,
    ) -> Result<()> {
        if !needs_effect {
            let b = &self.ctx.state.b;
            let final_expr = match tpl_expr {
                ConcatenationExpr::Static(e) => e,
                ConcatenationExpr::BareExpr(expr) => expr,
                ConcatenationExpr::Template(e) => e,
            };
            let is_empty_string = matches!(
                final_expr.get_inner_expression(),
                Expression::StringLiteral(lit) if lit.value.is_empty()
            );
            if !is_empty_string {
                let member = b.static_member(b.rid_expr(parent_var), "textContent");
                state
                    .init
                    .push(b.assign_stmt(AssignLeft::StaticMember(member), final_expr));
            }
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
        if !extra_blockers.is_empty() {
            super::data_structures::extend_blocker_slots(&mut state.extra_blockers, extra_blockers);
        }
        let b = &self.ctx.state.b;
        state
            .update
            .push(b.call_stmt("$.set_text", [Arg::Ident(&name), Arg::Expr(final_expr)]));
        Ok(())
    }

    fn emit_to_single_fragment_root(
        &mut self,
        state: &mut EmitState<'a>,
        ctx: &FragmentCtx<'a>,
        tpl_expr: ConcatenationExpr<'a>,
        needs_effect: bool,
        _has_memo: bool,
        extra_blockers: Vec<(String, usize)>,
    ) -> Result<()> {
        let name = self.ctx.state.gen_ident("text");
        let b = &self.ctx.state.b;

        if role_needs_text_first_next(ctx.role) {
            state
                .init
                .push(b.call_stmt("$.next", empty::<Arg<'a, '_>>()));
        }
        state
            .init
            .push(b.var_stmt(&name, b.call_expr("$.text", empty::<Arg<'a, '_>>())));
        state.root_var = Some(CompactString::from(name.as_str()));

        self.finalize_text_node_emission(state, &name, tpl_expr, needs_effect, extra_blockers)
    }

    fn emit_to_single_fragment_callback_param(
        &mut self,
        state: &mut EmitState<'a>,
        ctx: &FragmentCtx<'a>,
        append_inside: bool,
        tpl_expr: ConcatenationExpr<'a>,
        needs_effect: bool,
        _has_memo: bool,
        extra_blockers: Vec<(String, usize)>,
    ) -> Result<()> {
        let name = self.ctx.state.gen_ident("text");
        let b = &self.ctx.state.b;
        if !append_inside && role_needs_text_first_next(ctx.role) {
            state
                .init
                .push(b.call_stmt("$.next", empty::<Arg<'a, '_>>()));
        }
        state
            .init
            .push(b.var_stmt(&name, b.call_expr("$.text", empty::<Arg<'a, '_>>())));
        state.root_var = Some(CompactString::from(name.as_str()));
        self.finalize_text_node_emission(state, &name, tpl_expr, needs_effect, extra_blockers)
    }

    fn finalize_text_node_emission(
        &mut self,
        state: &mut EmitState<'a>,
        node_var: &str,
        tpl_expr: ConcatenationExpr<'a>,
        needs_effect: bool,
        extra_blockers: Vec<(String, usize)>,
    ) -> Result<()> {
        let final_expr = match tpl_expr {
            ConcatenationExpr::Static(e) => e,
            ConcatenationExpr::BareExpr(expr) => expr,
            ConcatenationExpr::Template(e) => e,
        };
        if needs_effect {
            if !extra_blockers.is_empty() {
                super::data_structures::extend_blocker_slots(
                    &mut state.extra_blockers,
                    extra_blockers,
                );
            }
            let b = &self.ctx.state.b;
            state
                .update
                .push(b.call_stmt("$.set_text", [Arg::Ident(node_var), Arg::Expr(final_expr)]));
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
    Static(Expression<'a>),
    BareExpr(Expression<'a>),
    Template(Expression<'a>),
}
