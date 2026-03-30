//! RenderTag codegen — `{@render snippet(args)}`

use oxc_ast::ast::{Expression, Statement};
use oxc_span::GetSpan;

use svelte_analyze::RenderTagCalleeMode;
use svelte_ast::NodeId;

use crate::builder::Arg;
use crate::context::Ctx;
use crate::template::expression::{MemoValueRef, TemplateMemoState};

/// Generate a render tag call: `snippet(anchor, () => arg1, () => arg2, ...)`
///
/// Dynamic callees (non-normal bindings) use `$.snippet(anchor, getter, ...args)`.
/// Optional chaining (`{@render fn?.()}`) uses `callee?.(anchor, ...)` or nullish coalescing.
///
/// `is_standalone` — true when the render tag is the sole child of a fragment
/// (uses `$$anchor` directly, needs `$.next()` after async wrapping).
pub(crate) fn gen_render_tag<'a>(
    ctx: &mut Ctx<'a>,
    id: NodeId,
    anchor_expr: Expression<'a>,
    is_standalone: bool,
    stmts: &mut Vec<Statement<'a>>,
) {
    let has_await = ctx.expr_has_await(id);
    let needs_async = has_await || ctx.expr_has_blockers(id);

    // When async-wrapped, the inner render call uses the callback param as anchor;
    // the outer anchor_expr is passed to $.async() separately.
    let anchor_name = if is_standalone { "$$anchor" } else { "node" };
    let (inner_anchor, outer_anchor) = if needs_async {
        (ctx.b.rid_expr(anchor_name), Some(anchor_expr))
    } else {
        (anchor_expr, None)
    };

    let tag = ctx.render_tag(id);
    let full_source = ctx.component.source_text(tag.expression_span);

    let mode = ctx.analysis.render_tag_callee_mode(id);

    // Pre-computed per-argument has_call flags
    let arg_has_call = ctx.analysis.render_tag_arg_has_call(id)
        .map(|v| v.to_vec())
        .unwrap_or_default();

    // Take ownership from ParsedExprs (already unwrapped from ChainExpression)
    let tag = ctx.render_tag(id);
    let offset = tag.expression_span.start;
    let expr = ctx.parsed.exprs.remove(&offset)
        .expect("render tag expression should be pre-parsed");

    let Expression::CallExpression(call) = expr else {
        debug_assert!(false, "render tag expression should be a CallExpression");
        return;
    };

    // Extract callee text from original source for non-dynamic path
    // (non-dynamic callees are normal bindings that haven't been transformed)
    let callee_span = call.callee.span();
    let callee_text = &full_source[callee_span.start as usize..callee_span.end as usize];

    let prop_sources = ctx.analysis.render_tag_prop_sources(id);

    // Unbox to separate callee expression and arguments
    let unboxed = call.unbox();
    let callee_expr = unboxed.callee;

    let arg_infos = ctx.analysis.render_tag_arg_infos(id)
        .map(|infos| infos.to_vec())
        .unwrap_or_default();

    // Build argument thunks (shared between dynamic and non-dynamic paths)
    let mut arg_thunks: Vec<Arg<'a, '_>> = Vec::new();
    let mut memo_counter: u32 = 0;
    let mut memo_stmts: Vec<Statement<'a>> = Vec::new();
    let mut deps = TemplateMemoState::default();

    for (i, arg) in unboxed.arguments.into_iter().enumerate() {
        let arg_expr = arg.into_expression();

        // Prop-source args: pass the getter identifier directly without a thunk
        if let Some(Some(sym_id)) = prop_sources.and_then(|ps| ps.get(i)) {
            let name = ctx.analysis.scoping.symbol_name(*sym_id);
            arg_thunks.push(Arg::Expr(ctx.b.rid_expr(name)));
            continue;
        }

        let has_call = arg_has_call.get(i).copied().unwrap_or(false);
        let arg_info = arg_infos.get(i);

        if let Some(info) = arg_info {
            if info.has_await && !info.ref_symbols.is_empty() {
                if let Some(MemoValueRef::Async(index)) =
                    deps.add_memoized_expr(ctx, info, ctx.b.clone_expr(&arg_expr))
                {
                    let async_param = deps.async_param_expr(ctx, index);
                    let param_expr = ctx.b.call_expr("$.get", [Arg::Expr(async_param)]);
                    let thunk = ctx.b.arrow_expr(ctx.b.no_params(), [ctx.b.expr_stmt(param_expr)]);
                    arg_thunks.push(Arg::Expr(thunk));
                    continue;
                }
            }
            deps.push_expr_info(ctx, info);
        }

        if has_call {
            let mut memo_name = String::with_capacity(4);
            memo_name.push('$');
            memo_name.push_str(&memo_counter.to_string());
            memo_counter += 1;
            let thunk = ctx.b.thunk(arg_expr);
            let derived = ctx.b.call_expr("$.derived", [Arg::Expr(thunk)]);
            memo_stmts.push(ctx.b.let_init_stmt(&memo_name, derived));
            let memo_ref = ctx.b.alloc_str(&memo_name);
            let get = ctx.b.call_expr("$.get", [Arg::Ident(memo_ref)]);
            let thunk = ctx.b.arrow_expr(ctx.b.no_params(), [ctx.b.expr_stmt(get)]);
            arg_thunks.push(Arg::Expr(thunk));
        } else {
            let thunk = ctx.b.arrow_expr(ctx.b.no_params(), [ctx.b.expr_stmt(arg_expr)]);
            arg_thunks.push(Arg::Expr(thunk));
        }
    }

    let final_stmt = match mode {
        RenderTagCalleeMode::DynamicRegular | RenderTagCalleeMode::DynamicChain => {
            // Dynamic callee: $.snippet(anchor, callee_getter, ...args)
            let callee_arg = build_dynamic_callee(ctx, callee_expr, mode.is_chain());

            let mut snippet_args: Vec<Arg<'a, '_>> = vec![Arg::Expr(inner_anchor), Arg::Expr(callee_arg)];
            snippet_args.extend(arg_thunks);

            ctx.b.call_stmt("$.snippet", snippet_args)
        }
        RenderTagCalleeMode::Chain => {
            // Non-dynamic optional: callee?.(anchor, ...args)
            let callee_expr = ctx.b.rid_expr(callee_text);
            let mut all_args: Vec<Arg<'a, '_>> = vec![Arg::Expr(inner_anchor)];
            all_args.extend(arg_thunks);
            let call_expr = ctx.b.maybe_call_expr(callee_expr, all_args);
            ctx.b.expr_stmt(call_expr)
        }
        RenderTagCalleeMode::Direct => {
            // Non-dynamic regular: callee(anchor, ...args)
            let mut all_args: Vec<Arg<'a, '_>> = vec![Arg::Expr(inner_anchor)];
            all_args.extend(arg_thunks);
            ctx.b.call_stmt(callee_text, all_args)
        }
    };

    if needs_async {
        let mut inner = memo_stmts;
        inner.push(final_stmt);
        for idx in ctx.analysis.expression_blockers(id) {
            deps.push_script_blocker(idx);
        }
        let callback_params = if deps.has_async_values() {
            let mut names = vec![anchor_name.to_string()];
            names.extend(deps.param_names());
            ctx.b.params(names.iter().map(|s| s.as_str()))
        } else {
            ctx.b.params([anchor_name])
        };
        let callback = ctx.b.arrow_block_expr(callback_params, inner);
        stmts.push(ctx.b.call_stmt("$.async", [
            Arg::Expr(outer_anchor.unwrap()),
            Arg::Expr(deps.blockers_expr(ctx)),
            Arg::Expr(deps.async_values_expr(ctx)),
            Arg::Expr(callback),
        ]));
        if is_standalone {
            stmts.push(ctx.b.call_stmt("$.next", std::iter::empty::<Arg>()));
        }
    } else if memo_stmts.is_empty() {
        stmts.push(final_stmt);
    } else {
        memo_stmts.push(final_stmt);
        stmts.push(ctx.b.block_stmt(memo_stmts));
    }
}

/// Build the callee argument for `$.snippet()`.
///
/// Uses the already-transformed callee expression from the AST.
/// Getter callees produce `thunk(callee_expr)` which unthunks `() => f()` to `f`.
/// Non-getter callees produce `thunk(callee_expr)` = `() => $.get(x)`.
/// Optional chaining adds `?? $.noop`.
fn build_dynamic_callee<'a>(
    ctx: &mut Ctx<'a>,
    callee_expr: Expression<'a>,
    is_chain: bool,
) -> Expression<'a> {
    if is_chain {
        // Optional: () => callee_expr ?? $.noop
        let coalesced = ctx.b.logical_coalesce(callee_expr, ctx.b.rid_expr("$.noop"));
        ctx.b.arrow_expr(ctx.b.no_params(), [ctx.b.expr_stmt(coalesced)])
    } else {
        // Regular: thunk(callee_expr) — unthunk optimizes () => f() to f
        ctx.b.thunk(callee_expr)
    }
}
