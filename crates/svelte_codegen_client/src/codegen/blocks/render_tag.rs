use std::iter;
use svelte_emit_builders::runes::rune_get;

use oxc_allocator::{CloneIn, Vec as OxcVec};
use oxc_ast::ast::{Argument, ChainElement, Expression, Statement};
use oxc_span::GetSpan;

use crate::context::Ctx;

use svelte_analyze::scope::SymbolId;
use svelte_analyze::{
    BindingSemantics, RenderArgKind, RenderAsyncKind, RenderCallKind, RenderTagBlockSemantics,
};
use svelte_ast::NodeId;
use svelte_ast_builder::Arg;

use super::super::data_structures::EmitState;
use super::super::data_structures::{FragmentAnchor, FragmentCtx};
use super::super::{Codegen, CodegenError, Result};

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(in super::super) fn emit_render_tag(
        &mut self,
        state: &mut EmitState<'a>,
        ctx: &FragmentCtx<'a>,
        id: NodeId,
        sem: RenderTagBlockSemantics,
    ) -> Result<()> {
        let (needs_async, blockers) = match &sem.async_kind {
            RenderAsyncKind::Sync => (false, Vec::new()),
            RenderAsyncKind::Async { blockers } => (true, blockers.to_vec()),
        };

        let is_static_shape = render_callee_is_static(self.ctx, sem.callee_sym);
        let is_standalone = matches!(
            ctx.anchor,
            FragmentAnchor::Root
                | FragmentAnchor::CallbackParam {
                    append_inside: false,
                    ..
                }
                | FragmentAnchor::CallbackParam {
                    append_inside: true,
                    ..
                }
        );

        let anchor_expr = if needs_async || is_static_shape {
            match &ctx.anchor {
                FragmentAnchor::Root => self.ctx.b.rid_expr("$$anchor"),
                FragmentAnchor::CallbackParam { name, .. } => self.ctx.b.rid_expr(name),
                _ => {
                    let anchor_node = self.comment_anchor_node_name(state, ctx)?;
                    self.ctx.b.rid_expr(&anchor_node)
                }
            }
        } else {
            let anchor_node = self.comment_anchor_node_name(state, ctx)?;
            self.ctx.b.rid_expr(&anchor_node)
        };

        let expr = self.take_node_expr(id)?;
        let call = match expr.into_inner_expression() {
            Expression::CallExpression(call) => call,
            Expression::ChainExpression(chain) => match chain.unbox().expression {
                ChainElement::CallExpression(call) => call,
                _ => {
                    return CodegenError::unexpected_node(
                        id,
                        "render tag must wrap CallExpression",
                    );
                }
            },
            _ => return CodegenError::unexpected_node(id, "render tag must wrap CallExpression"),
        };
        let callee_span = call.callee.span();
        let unboxed = call.unbox();
        let callee_expr = unboxed.callee;
        let arguments = unboxed.arguments;

        let tag = self.ctx.render_tag(id);
        let callee_text: &'a str = self
            .ctx
            .query
            .component
            .source_text(svelte_span::Span::new(callee_span.start, callee_span.end));

        let anchor_name = if is_standalone { "$$anchor" } else { "node" };
        let mut sync_memo_count: u32 = 0;
        let mut async_memo_count: u32 = 0;
        for arg in &sem.args {
            match arg {
                RenderArgKind::NeedsMemo => sync_memo_count += 1,
                RenderArgKind::AwaitMemo { .. } => async_memo_count += 1,
                _ => {}
            }
        }

        let inner_anchor_expr = if needs_async {
            self.ctx.b.rid_expr(anchor_name)
        } else {
            anchor_expr.clone_in(self.ctx.b.ast.allocator)
        };

        let mut memo_stmts: Vec<Statement<'a>> = Vec::new();
        let mut async_values: Vec<Expression<'a>> = Vec::new();
        let arg_thunks = self.build_render_arg_thunks(
            id,
            arguments,
            &sem,
            &mut memo_stmts,
            &mut async_values,
            sync_memo_count,
        )?;
        let tag_span_start = tag.span.start;
        let final_expr = self.build_render_final_call(
            sem.call_kind,
            is_static_shape,
            callee_expr,
            callee_text,
            inner_anchor_expr,
            arg_thunks,
        );
        let final_stmt = self.add_svelte_meta(final_expr, tag_span_start, "render");

        if needs_async {
            self.emit_render_async_wrapped(
                state,
                &blockers,
                async_values,
                anchor_name,
                sync_memo_count,
                async_memo_count,
                anchor_expr,
                final_stmt,
                memo_stmts,
                is_standalone,
            );
        } else if memo_stmts.is_empty() {
            state.init.push(final_stmt);
        } else {
            memo_stmts.push(final_stmt);
            state.init.push(self.ctx.b.block_stmt(memo_stmts));
        }

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn emit_render_async_wrapped(
        &mut self,
        state: &mut EmitState<'a>,
        blockers: &[u32],
        async_values: Vec<Expression<'a>>,
        anchor_name: &str,
        sync_memo_count: u32,
        async_memo_count: u32,
        outer_anchor: Expression<'a>,
        final_stmt: Statement<'a>,
        mut memo_stmts: Vec<Statement<'a>>,
        is_standalone: bool,
    ) {
        memo_stmts.push(final_stmt);

        let blockers_expr = if blockers.is_empty() {
            self.ctx.b.void_zero_expr()
        } else {
            self.ctx.b.array_expr(blockers.iter().map(|&idx| {
                self.ctx.b.computed_member_expr(
                    self.ctx.b.rid_expr("$$promises"),
                    self.ctx.b.num_expr(idx as f64),
                )
            }))
        };

        let async_values_expr = if async_values.is_empty() {
            self.ctx.b.void_zero_expr()
        } else {
            let thunks: Vec<Expression<'a>> = async_values
                .into_iter()
                .map(|e| async_value_thunk(self.ctx, e))
                .collect();
            self.ctx.b.array_expr(thunks)
        };

        let callback_params = if async_memo_count > 0 {
            let mut names = vec![anchor_name.to_string()];
            for i in 0..async_memo_count {
                names.push(format!("${}", sync_memo_count + i));
            }
            self.ctx.b.params(names.iter().map(|s| s.as_str()))
        } else {
            self.ctx.b.params([anchor_name])
        };
        let callback = self.ctx.b.arrow_block_expr(callback_params, memo_stmts);

        state.init.push(self.ctx.b.call_stmt(
            "$.async",
            [
                Arg::Expr(outer_anchor),
                Arg::Expr(blockers_expr),
                Arg::Expr(async_values_expr),
                Arg::Expr(callback),
            ],
        ));
        if is_standalone {
            state
                .init
                .push(self.ctx.b.call_stmt("$.next", iter::empty::<Arg>()));
        }
    }

    fn build_render_arg_thunks<'alloc>(
        &mut self,
        _owner_id: NodeId,
        arguments: OxcVec<'alloc, Argument<'a>>,
        sem: &RenderTagBlockSemantics,
        memo_stmts: &mut Vec<Statement<'a>>,
        async_values: &mut Vec<Expression<'a>>,
        sync_memo_count: u32,
    ) -> Result<Vec<Arg<'a, 'static>>> {
        let mut thunks: Vec<Arg<'a, 'static>> = Vec::with_capacity(sem.args.len());
        let mut sync_seen: u32 = 0;
        let mut async_seen: u32 = 0;
        for (arg, arg_sem) in arguments.into_iter().zip(sem.args.iter()) {
            let arg_expr = arg.into_expression();
            match *arg_sem {
                RenderArgKind::PropPassthrough { sym } => {
                    let name = self.ctx.symbol_name(sym).to_string();
                    thunks.push(Arg::Expr(self.ctx.b.rid_expr(&name)));
                }
                RenderArgKind::NeedsMemo => {
                    let name = format!("${sync_seen}");
                    sync_seen += 1;
                    let helper = self.ctx.query.view.derived_helper();
                    let thunk = self.ctx.b.thunk(arg_expr);
                    let derived = self.ctx.b.call_expr(helper, [Arg::Expr(thunk)]);
                    memo_stmts.push(self.ctx.b.let_init_stmt(&name, derived));
                    let name_ref = self.ctx.b.alloc_str(&name);
                    let get = rune_get(&self.ctx.b, name_ref);
                    let read_thunk = self
                        .ctx
                        .b
                        .arrow_expr(self.ctx.b.no_params(), [self.ctx.b.expr_stmt(get)]);
                    thunks.push(Arg::Expr(read_thunk));
                }
                RenderArgKind::AwaitMemo { .. } => {
                    async_values.push(self.ctx.b.clone_expr(&arg_expr));
                    let param_name = format!("${}", sync_memo_count + async_seen);
                    async_seen += 1;
                    let param_ref = self.ctx.b.alloc_str(&param_name);
                    let get = rune_get(&self.ctx.b, param_ref);
                    let read_thunk = self
                        .ctx
                        .b
                        .arrow_expr(self.ctx.b.no_params(), [self.ctx.b.expr_stmt(get)]);
                    thunks.push(Arg::Expr(read_thunk));
                }
                RenderArgKind::InertThunk => {
                    thunks.push(Arg::Expr(self.ctx.b.thunk(arg_expr)));
                }
            }
        }
        Ok(thunks)
    }
}

fn async_value_thunk<'a>(ctx: &mut Ctx<'a>, expr: Expression<'a>) -> Expression<'a> {
    if let Expression::AwaitExpression(await_expr) = expr {
        let inner = await_expr.unbox().argument;
        ctx.b
            .arrow_expr(ctx.b.no_params(), [ctx.b.expr_stmt(inner)])
    } else {
        ctx.b.async_arrow_expr_body(expr)
    }
}

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    fn build_render_final_call(
        &mut self,
        call_kind: RenderCallKind,
        is_static_shape: bool,
        callee_expr: Expression<'a>,
        callee_text: &'a str,
        anchor_expr: Expression<'a>,
        arg_thunks: Vec<Arg<'a, 'static>>,
    ) -> Expression<'a> {
        let is_chain = matches!(call_kind, RenderCallKind::OptionalChain);
        if !is_static_shape {
            let callee_arg = if is_chain {
                let coalesced = self
                    .ctx
                    .b
                    .logical_coalesce(callee_expr, self.ctx.b.rid_expr("$.noop"));
                self.ctx
                    .b
                    .arrow_expr(self.ctx.b.no_params(), [self.ctx.b.expr_stmt(coalesced)])
            } else {
                self.ctx.b.thunk(callee_expr)
            };
            let mut snippet_args: Vec<Arg<'a, '_>> =
                vec![Arg::Expr(anchor_expr), Arg::Expr(callee_arg)];
            snippet_args.extend(arg_thunks);
            return self.ctx.b.call_expr("$.snippet", snippet_args);
        }
        let callee = self.ctx.b.rid_expr(callee_text);
        let mut all_args: Vec<Arg<'a, '_>> = vec![Arg::Expr(anchor_expr)];
        all_args.extend(arg_thunks);
        if is_chain {
            self.ctx.b.maybe_call_expr(callee, all_args)
        } else {
            self.ctx.b.call_expr_callee(callee, all_args)
        }
    }
}

pub(crate) fn render_callee_is_static(ctx: &Ctx<'_>, callee_sym: Option<SymbolId>) -> bool {
    let Some(sym) = callee_sym else {
        return false;
    };
    match ctx.query.view.binding_semantics(sym) {
        BindingSemantics::MaybeReactive
        | BindingSemantics::NonReactive
        | BindingSemantics::Unresolved => true,
        BindingSemantics::Prop(_)
        | BindingSemantics::State(_)
        | BindingSemantics::Derived(_)
        | BindingSemantics::OptimizedDerived(_)
        | BindingSemantics::OptimizedRune(_)
        | BindingSemantics::RuntimeRune { .. }
        | BindingSemantics::Store(_)
        | BindingSemantics::LegacyBindableProp(_)
        | BindingSemantics::LegacyState(_)
        | BindingSemantics::Const(_)
        | BindingSemantics::Contextual(_)
        | BindingSemantics::LegacyApiExport => false,
    }
}
