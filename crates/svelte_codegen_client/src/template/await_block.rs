//! AwaitBlock code generation.

use oxc_ast::ast::{Expression, Statement};

use svelte_analyze::FragmentKey;
use svelte_ast::NodeId;

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
    let value_span = block.value_span;
    let error_span = block.error_span;

    // Expression thunk: () => promise (visit expression first for correct scope order)
    let expression = build_node_thunk(ctx, block_id);

    // Then callback (generated before pending to match reference compiler ordering)
    let then_fn = if has_then {
        let then_key = FragmentKey::AwaitThen(block_id);
        gen_await_callback(ctx, then_key, value_span)
    } else if has_catch {
        ctx.b.void_zero_expr()
    } else {
        ctx.b.null_expr()
    };

    // Catch callback
    let catch_fn = if has_catch {
        let catch_key = FragmentKey::AwaitCatch(block_id);
        gen_await_callback(ctx, catch_key, error_span)
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

    body.push(ctx.b.call_stmt("$.await", args));
}

/// Generate a callback for then/catch: `($$anchor, binding) => { ...fragment... }`
fn gen_await_callback<'a>(
    ctx: &mut Ctx<'a>,
    fragment_key: FragmentKey,
    binding_span: Option<svelte_span::Span>,
) -> Expression<'a> {
    let frag_body = gen_fragment(ctx, fragment_key);

    if let Some(span) = binding_span {
        let binding_text = ctx.component.source_text(span).to_string();
        let trimmed = binding_text.trim();

        if trimmed.starts_with('{') || trimmed.starts_with('[') {
            gen_destructured_callback(ctx, trimmed, frag_body)
        } else {
            ctx.b.arrow_block_expr(ctx.b.params(["$$anchor", trimmed]), frag_body)
        }
    } else {
        ctx.b.arrow_block_expr(ctx.b.params(["$$anchor"]), frag_body)
    }
}

/// Generate callback for destructured bindings.
fn gen_destructured_callback<'a>(
    ctx: &mut Ctx<'a>,
    pattern: &str,
    frag_body: Vec<Statement<'a>>,
) -> Expression<'a> {
    let inner = &pattern[1..pattern.len()-1];
    let names: Vec<String> = inner.split(',')
        .filter_map(|s| {
            let s = s.split('=').next().unwrap_or("").trim();
            let s = if let Some((_key, alias)) = s.split_once(':') {
                alias.trim()
            } else {
                s
            };
            if s.is_empty() { None } else { Some(s.to_string()) }
        })
        .collect();

    let mut declarations: Vec<Statement<'a>> = Vec::new();

    // var $$value = $.derived(() => { var { name, age } = $.get($$source); return { name, age }; });
    let get_source = ctx.b.call_expr("$.get", [Arg::Ident("$$source")]);
    let destruct_stmt = ctx.b.var_object_destruct_stmt(&names, get_source);
    let return_obj = ctx.b.shorthand_object_expr(&names);
    let return_stmt = ctx.b.return_stmt(return_obj);
    let derived_fn = ctx.b.thunk_block(vec![destruct_stmt, return_stmt]);
    let derived_call = ctx.b.call_expr("$.derived", [Arg::Expr(derived_fn)]);
    declarations.push(ctx.b.var_stmt("$$value", derived_call));

    // var name = $.derived(() => $.get($$value).name);
    for name in &names {
        let get_value = ctx.b.call_expr("$.get", [Arg::Ident("$$value")]);
        let member = ctx.b.static_member_expr(get_value, name);
        let getter_fn = ctx.b.arrow_expr(ctx.b.no_params(), [ctx.b.expr_stmt(member)]);
        let per_field_derived = ctx.b.call_expr("$.derived", [Arg::Expr(getter_fn)]);
        declarations.push(ctx.b.var_stmt(name, per_field_derived));
    }

    declarations.extend(frag_body);
    ctx.b.arrow_block_expr(ctx.b.params(["$$anchor", "$$source"]), declarations)
}
