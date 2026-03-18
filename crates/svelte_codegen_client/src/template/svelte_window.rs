//! SvelteWindow code generation — `<svelte:window on:event bind:prop />`.

use oxc_ast::ast::{Expression, Statement};

use svelte_ast::{Attribute, NodeId};

use crate::builder::Arg;
use crate::context::Ctx;

use super::expression::get_attr_expr;

/// Generate event listeners and bindings for `<svelte:window>`.
///
/// Events → `$.event(name, $.window, handler)` pushed to init.
/// Bindings → `$.bind_window_scroll` / `$.bind_window_size` / `$.bind_online` / `$.bind_property`.
pub(crate) fn gen_svelte_window<'a>(
    ctx: &mut Ctx<'a>,
    id: NodeId,
    stmts: &mut Vec<Statement<'a>>,
) {
    let window = ctx.svelte_window(id);
    let attrs: Vec<_> = window.attributes.clone();

    for attr in &attrs {
        match attr {
            Attribute::OnDirectiveLegacy(od) => {
                let attr_id = attr.id();
                gen_legacy_event(ctx, od, attr_id, stmts);
            }
            Attribute::ExpressionAttribute(ea) => {
                if let Some(event_name) = ea.name.strip_prefix("on") {
                    let attr_id = attr.id();
                    gen_event_attr(ctx, attr_id, event_name, stmts);
                }
            }
            Attribute::BindDirective(bind) => {
                gen_window_binding(ctx, bind, stmts);
            }
            _ => {}
        }
    }
}

/// Legacy `on:event` → `$.event("name", $.window, handler)`.
fn gen_legacy_event<'a>(
    ctx: &mut Ctx<'a>,
    od: &svelte_ast::OnDirectiveLegacy,
    attr_id: NodeId,
    stmts: &mut Vec<Statement<'a>>,
) {
    let handler = if od.expression_span.is_none() {
        // Bubble event
        let bubble_call = ctx.b.static_member_expr(
            ctx.b.rid_expr("$.bubble_event"),
            "call",
        );
        let call = ctx.b.call_expr_callee(bubble_call, [
            Arg::Expr(ctx.b.this_expr()),
            Arg::Ident("$$props"),
            Arg::Ident("$$arg"),
        ]);
        ctx.b.function_expr(ctx.b.params(["$$arg"]), vec![ctx.b.expr_stmt(call)])
    } else {
        let expr = get_attr_expr(ctx, attr_id);
        build_event_handler(ctx, expr)
    };

    let mut wrapped = handler;
    for modifier in &[
        "stopPropagation",
        "stopImmediatePropagation",
        "preventDefault",
        "self",
        "trusted",
        "once",
    ] {
        if od.modifiers.iter().any(|m| m == modifier) {
            let fn_name = format!("$.{}", modifier);
            wrapped = ctx.b.call_expr(&fn_name, [Arg::Expr(wrapped)]);
        }
    }

    let capture = od.modifiers.iter().any(|m| m == "capture");
    let passive = od.modifiers.iter().find_map(|m| match m.as_str() {
        "passive" => Some(true),
        "nonpassive" => Some(false),
        _ => None,
    });

    let mut args: Vec<Arg<'a, '_>> = vec![
        Arg::Str(od.name.clone()),
        Arg::Ident("$.window"),
        Arg::Expr(wrapped),
    ];
    if capture || passive.is_some() {
        args.push(Arg::Bool(capture));
    }
    if let Some(p) = passive {
        args.push(Arg::Bool(p));
    }

    stmts.push(ctx.b.call_stmt("$.event", args));
}

/// Svelte 5 event attribute → `$.event(name, $.window, handler [, capture] [, passive])`.
fn gen_event_attr<'a>(
    ctx: &mut Ctx<'a>,
    attr_id: NodeId,
    raw_event_name: &str,
    stmts: &mut Vec<Statement<'a>>,
) {
    let (event_name, capture) = if let Some(base) = svelte_js::strip_capture_event(raw_event_name) {
        (base.to_string(), true)
    } else {
        (raw_event_name.to_string(), false)
    };

    let has_call = ctx.analysis.attr_expression(attr_id).map_or(false, |e| e.has_call);
    let handler_expr = get_attr_expr(ctx, attr_id);
    let handler = super::attributes::build_event_handler_s5(ctx, handler_expr, has_call, stmts);

    let passive = svelte_js::is_passive_event(&event_name);
    let mut args: Vec<Arg<'a, '_>> = vec![
        Arg::Str(event_name),
        Arg::Ident("$.window"),
        Arg::Expr(handler),
    ];
    if capture || passive {
        args.push(if capture { Arg::Bool(true) } else { Arg::Expr(ctx.b.void_zero_expr()) });
    }
    if passive {
        args.push(Arg::Bool(true));
    }
    stmts.push(ctx.b.call_stmt("$.event", args));
}

/// Build event handler for legacy on:directive (non-dev mode).
fn build_event_handler<'a>(ctx: &mut Ctx<'a>, handler: Expression<'a>) -> Expression<'a> {
    match &handler {
        Expression::ArrowFunctionExpression(_) | Expression::FunctionExpression(_) => handler,
        Expression::Identifier(_) => handler,
        _ => {
            let apply = ctx.b.static_member_expr(handler, "apply");
            let call = ctx.b.call_expr_callee(apply, [
                Arg::Expr(ctx.b.this_expr()),
                Arg::Ident("$$args"),
            ]);
            ctx.b.function_expr(ctx.b.rest_params("$$args"), vec![ctx.b.expr_stmt(call)])
        }
    }
}

/// Generate window-specific bind directives.
fn gen_window_binding<'a>(
    ctx: &mut Ctx<'a>,
    bind: &svelte_ast::BindDirective,
    stmts: &mut Vec<Statement<'a>>,
) {
    let var_name = if bind.shorthand {
        bind.name.clone()
    } else if let Some(span) = bind.expression_span {
        ctx.component.source_text(span).trim().to_string()
    } else {
        return;
    };

    let build_getter = |ctx: &mut Ctx<'a>, var: &str| -> Expression<'a> {
        let body = if ctx.is_mutable_rune(var) {
            ctx.b.call_expr("$.get", [Arg::Ident(var)])
        } else {
            ctx.b.rid_expr(var)
        };
        ctx.b.arrow_expr(ctx.b.no_params(), [ctx.b.expr_stmt(body)])
    };

    // Window binding setters use `$.set(var, $$value, true)` — the `true` prevents
    // re-triggering reactivity within the binding's own effect.
    let build_setter = |ctx: &mut Ctx<'a>, var: String| -> Expression<'a> {
        let body = if ctx.is_mutable_rune(&var) {
            ctx.b.call_expr("$.set", [
                Arg::Ident(&var),
                Arg::Ident("$$value"),
                Arg::Bool(true),
            ])
        } else {
            ctx.b.assign_expr(
                crate::builder::AssignLeft::Ident(var),
                crate::builder::AssignRight::Expr(ctx.b.rid_expr("$$value")),
            )
        };
        ctx.b.arrow_expr(ctx.b.params(["$$value"]), [ctx.b.expr_stmt(body)])
    };

    let stmt = match bind.name.as_str() {
        "scrollX" | "scrollY" => {
            let axis = if bind.name == "scrollX" { "x" } else { "y" };
            let getter = build_getter(ctx, &var_name);
            let setter = build_setter(ctx, var_name);
            ctx.b.call_stmt("$.bind_window_scroll", [
                Arg::Str(axis.to_string()),
                Arg::Expr(getter),
                Arg::Expr(setter),
            ])
        }
        "innerWidth" | "innerHeight" | "outerWidth" | "outerHeight" => {
            let setter = build_setter(ctx, var_name);
            ctx.b.call_stmt("$.bind_window_size", [
                Arg::Str(bind.name.clone()),
                Arg::Expr(setter),
            ])
        }
        "online" => {
            let setter = build_setter(ctx, var_name);
            ctx.b.call_stmt("$.bind_online", [Arg::Expr(setter)])
        }
        "devicePixelRatio" => {
            let setter = build_setter(ctx, var_name);
            ctx.b.call_stmt("$.bind_property", [
                Arg::Str("devicePixelRatio".to_string()),
                Arg::Str("resize".to_string()),
                Arg::Ident("$.window"),
                Arg::Expr(setter),
            ])
        }
        _ => return,
    };

    stmts.push(stmt);
}
