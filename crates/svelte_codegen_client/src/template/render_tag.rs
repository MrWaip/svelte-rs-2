//! RenderTag codegen — `{@render snippet(args)}`

use oxc_ast::ast::{Expression, Statement};
use oxc_span::GetSpan;

use svelte_analyze::RenderTagCalleeMode;
use svelte_ast::NodeId;

use crate::builder::Arg;
use crate::context::Ctx;

/// Generate a render tag call: `snippet(anchor, () => arg1, () => arg2, ...)`
///
/// Dynamic callees (non-normal bindings) use `$.snippet(anchor, getter, ...args)`.
/// Optional chaining (`{@render fn?.()}`) uses `callee?.(anchor, ...)` or nullish coalescing.
pub(crate) fn gen_render_tag<'a>(
    ctx: &mut Ctx<'a>,
    id: NodeId,
    anchor_expr: Expression<'a>,
    stmts: &mut Vec<Statement<'a>>,
) {
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

    // Build argument thunks (shared between dynamic and non-dynamic paths)
    let mut arg_thunks: Vec<Arg<'a, '_>> = Vec::new();
    let mut memo_counter: u32 = 0;
    let mut memo_stmts: Vec<Statement<'a>> = Vec::new();

    for (i, arg) in unboxed.arguments.into_iter().enumerate() {
        let arg_expr = arg.into_expression();

        // Prop-source args: pass the getter identifier directly without a thunk
        if let Some(Some(sym_id)) = prop_sources.and_then(|ps| ps.get(i)) {
            let name = ctx.analysis.scoping.symbol_name(*sym_id);
            arg_thunks.push(Arg::Expr(ctx.b.rid_expr(name)));
            continue;
        }

        let has_call = arg_has_call.get(i).copied().unwrap_or(false);

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

            let mut snippet_args: Vec<Arg<'a, '_>> = vec![Arg::Expr(anchor_expr), Arg::Expr(callee_arg)];
            snippet_args.extend(arg_thunks);

            ctx.b.call_stmt("$.snippet", snippet_args)
        }
        RenderTagCalleeMode::Chain => {
            // Non-dynamic optional: callee?.(anchor, ...args)
            let callee_expr = ctx.b.rid_expr(callee_text);
            let mut all_args: Vec<Arg<'a, '_>> = vec![Arg::Expr(anchor_expr)];
            all_args.extend(arg_thunks);
            let call_expr = ctx.b.maybe_call_expr(callee_expr, all_args);
            ctx.b.expr_stmt(call_expr)
        }
        RenderTagCalleeMode::Direct => {
            // Non-dynamic regular: callee(anchor, ...args)
            let mut all_args: Vec<Arg<'a, '_>> = vec![Arg::Expr(anchor_expr)];
            all_args.extend(arg_thunks);
            ctx.b.call_stmt(callee_text, all_args)
        }
    };

    if memo_stmts.is_empty() {
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
