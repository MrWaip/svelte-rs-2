//! AwaitBlock code generation.

use oxc_ast::ast::{Expression, Statement};

use svelte_analyze::FragmentKey;
use svelte_ast::NodeId;
use svelte_parser::{AwaitBindingInfo, DestructureKind};

use crate::builder::Arg;
use crate::context::Ctx;

use super::gen_fragment;
use super::expression::build_node_thunk;

/// Generate statements for an `{#await ...}` block.
///
/// Order matches the reference compiler: expression → then → catch → pending.
/// This matters because fragment generation assigns template variable numbers sequentially.
pub(crate) fn gen_await_block<'a>(
    ctx: &mut Ctx<'a>,
    block_id: NodeId,
    anchor: Expression<'a>,
    body: &mut Vec<Statement<'a>>,
) {
    let block = ctx.await_block(block_id);
    let has_pending = block.pending.is_some();
    let has_then = block.then.is_some();
    let has_catch = block.catch.is_some();
    let span_start = block.span.start;

    // Expression thunk: () => promise (visit expression first for correct scope order)
    let expression = build_node_thunk(ctx, block_id);

    // Then callback (generated before pending to match reference compiler ordering)
    let then_fn = if has_then {
        let then_key = FragmentKey::AwaitThen(block_id);
        let value_binding = ctx.await_value_binding(block_id).cloned();
        gen_await_callback(ctx, then_key, value_binding.as_ref())
    } else if has_catch {
        ctx.b.void_zero_expr()
    } else {
        ctx.b.null_expr()
    };

    // Catch callback
    let catch_fn = if has_catch {
        let catch_key = FragmentKey::AwaitCatch(block_id);
        let error_binding = ctx.await_error_binding(block_id).cloned();
        gen_await_callback(ctx, catch_key, error_binding.as_ref())
    } else {
        ctx.b.null_expr()
    };

    // Pending callback (generated last)
    let pending_fn = if has_pending {
        let pending_key = FragmentKey::AwaitPending(block_id);
        let frag_body = gen_fragment(ctx, pending_key);
        ctx.b.arrow_block_expr(ctx.b.params(["$$anchor"]), frag_body)
    } else {
        ctx.b.null_expr()
    };

    // Build args, trimming trailing nulls
    let mut args: Vec<Arg<'a, '_>> = vec![
        Arg::Expr(anchor),
        Arg::Expr(expression),
        Arg::Expr(pending_fn),
    ];

    if has_then || has_catch {
        args.push(Arg::Expr(then_fn));
    }
    if has_catch {
        args.push(Arg::Expr(catch_fn));
    }

    let await_call = ctx.b.call_expr("$.await", args);
    body.push(super::add_svelte_meta(ctx, await_call, span_start, "await"));
}

/// Generate a callback for then/catch: `($$anchor, binding) => { ...fragment... }`
fn gen_await_callback<'a>(
    ctx: &mut Ctx<'a>,
    fragment_key: FragmentKey,
    binding: Option<&AwaitBindingInfo>,
) -> Expression<'a> {
    let frag_body = gen_fragment(ctx, fragment_key);

    match binding {
        Some(AwaitBindingInfo::Simple(name)) => {
            ctx.b.arrow_block_expr(ctx.b.params(["$$anchor", name]), frag_body)
        }
        Some(AwaitBindingInfo::Destructured { kind, names }) => {
            gen_destructured_callback(ctx, *kind, names, frag_body)
        }
        None => {
            ctx.b.arrow_block_expr(ctx.b.params(["$$anchor"]), frag_body)
        }
    }
}

/// Generate callback for destructured bindings.
fn gen_destructured_callback<'a>(
    ctx: &mut Ctx<'a>,
    kind: DestructureKind,
    names: &[String],
    frag_body: Vec<Statement<'a>>,
) -> Expression<'a> {
    let mut declarations: Vec<Statement<'a>> = Vec::new();

    // var $$value = $.derived(() => { var [a, b] / { name, age } = $.get($$source); return { a, b / name, age }; });
    let get_source = ctx.b.call_expr("$.get", [Arg::Ident("$$source")]);
    let destruct_stmt = match kind {
        DestructureKind::Array => ctx.b.var_array_destruct_stmt(names, get_source),
        DestructureKind::Object => ctx.b.var_object_destruct_stmt(names, get_source),
    };
    let return_obj = ctx.b.shorthand_object_expr(names);
    let return_stmt = ctx.b.return_stmt(return_obj);
    let derived_fn = ctx.b.thunk_block(vec![destruct_stmt, return_stmt]);
    let derived_call = ctx.b.call_expr("$.derived", [Arg::Expr(derived_fn)]);
    declarations.push(ctx.b.var_stmt("$$value", derived_call));

    // var name = $.derived(() => $.get($$value).name);
    for name in names {
        let get_value = ctx.b.call_expr("$.get", [Arg::Ident("$$value")]);
        let member = ctx.b.static_member_expr(get_value, name);
        let getter_fn = ctx.b.arrow_expr(ctx.b.no_params(), [ctx.b.expr_stmt(member)]);
        let per_field_derived = ctx.b.call_expr("$.derived", [Arg::Expr(getter_fn)]);
        declarations.push(ctx.b.var_stmt(name, per_field_derived));
    }

    declarations.extend(frag_body);
    ctx.b.arrow_block_expr(ctx.b.params(["$$anchor", "$$source"]), declarations)
}
