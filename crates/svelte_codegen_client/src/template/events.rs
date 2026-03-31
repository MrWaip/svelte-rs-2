//! Event handler, use:action, transition, animate, and legacy on:directive codegen.

use oxc_ast::ast::{Expression, Statement};

use svelte_ast::NodeId;

use crate::builder::Arg;
use crate::context::Ctx;

use super::expression::get_attr_expr;

// ---------------------------------------------------------------------------
// use:action directive codegen
// ---------------------------------------------------------------------------

/// Generate `$.action(target, ...)` for `use:action` on special elements (svelte:body, etc.).
pub(crate) fn gen_use_directive_on<'a>(
    ctx: &mut Ctx<'a>,
    ud: &svelte_ast::UseDirective,
    attr_id: NodeId,
    el_name: &str,
    init: &mut Vec<Statement<'a>>,
) {
    gen_use_directive(ctx, ud, attr_id, el_name, init);
}

/// Generate `$.action(el, ($$node) => name?.($$node), () => expr)`.
/// Reference: `UseDirective.js`.
pub(crate) fn gen_use_directive<'a>(
    ctx: &mut Ctx<'a>,
    ud: &svelte_ast::UseDirective,
    attr_id: NodeId,
    el_name: &str,
    init: &mut Vec<Statement<'a>>,
) {
    // Build handler params: ($$node) or ($$node, $$action_arg)
    let has_expr = ud.expression_span.is_some();
    let params = if has_expr {
        ctx.b.params(["$$node", "$$action_arg"])
    } else {
        ctx.b.params(["$$node"])
    };

    let name_expr = ctx.parsed.exprs.remove(&ud.name.start).unwrap();

    // Build optional call: name?.($$node) or name?.($$node, $$action_arg)
    let mut call_args: Vec<Arg<'a, '_>> = vec![Arg::Ident("$$node")];
    if has_expr {
        call_args.push(Arg::Ident("$$action_arg"));
    }
    let action_call = ctx.b.maybe_call_expr(name_expr, call_args);

    // Wrap in arrow: ($$node) => name?.($$node)
    let handler = ctx.b.arrow_expr(params, [ctx.b.expr_stmt(action_call)]);

    // Build $.action() args
    let mut args: Vec<Arg<'a, '_>> = vec![
        Arg::Ident(el_name),
        Arg::Expr(handler),
    ];

    // Optional third arg: () => expression
    if has_expr {
        let expr = get_attr_expr(ctx, attr_id);
        let thunk = ctx.b.thunk(expr);
        args.push(Arg::Expr(thunk));
    }

    let mut stmt = ctx.b.call_stmt("$.action", args);

    let blockers = ctx.analysis().attr_expression_blockers(attr_id);
    if !blockers.is_empty() {
        stmt = gen_run_after_blockers(ctx, stmt, &blockers);
    }

    init.push(stmt);
}

// ---------------------------------------------------------------------------
// {@attach expr} codegen
// ---------------------------------------------------------------------------

/// Generate `$.attach(el, () => expr)`.
/// Reference: `AttachTag.js`.
pub(crate) fn gen_attach_tag<'a>(
    ctx: &mut Ctx<'a>,
    attr_id: NodeId,
    el_name: &str,
    init: &mut Vec<Statement<'a>>,
) {
    let expr = get_attr_expr(ctx, attr_id);
    let thunk = ctx.b.thunk(expr);
    let mut stmt = ctx.b.call_stmt("$.attach", [Arg::Ident(el_name), Arg::Expr(thunk)]);

    let blockers = ctx.analysis().attr_expression_blockers(attr_id);
    if !blockers.is_empty() {
        stmt = gen_run_after_blockers(ctx, stmt, &blockers);
    }

    init.push(stmt);
}

// ---------------------------------------------------------------------------
// transition:/in:/out: directive codegen
// ---------------------------------------------------------------------------

/// Generate `$.transition(flags, el, () => transitionFn, () => params)`.
/// Reference: `TransitionDirective.js`.
pub(crate) fn gen_transition_directive<'a>(
    ctx: &mut Ctx<'a>,
    td: &svelte_ast::TransitionDirective,
    attr_id: NodeId,
    el_name: &str,
    after_update: &mut Vec<Statement<'a>>,
) {
    // TRANSITION_IN = 1, TRANSITION_OUT = 2, TRANSITION_GLOBAL = 4
    let mut flags: u32 = if td.modifiers.iter().any(|m| m == "global") { 4 } else { 0 };
    match td.direction {
        svelte_ast::TransitionDirection::Both => flags |= 1 | 2,
        svelte_ast::TransitionDirection::In => flags |= 1,
        svelte_ast::TransitionDirection::Out => flags |= 2,
    }

    let name_expr = ctx.parsed.exprs.remove(&td.name.start).unwrap();
    let name_thunk = ctx.b.thunk(name_expr);

    let mut args: Vec<Arg<'a, '_>> = vec![
        Arg::Num(flags as f64),
        Arg::Ident(el_name),
        Arg::Expr(name_thunk),
    ];

    if td.expression_span.is_some() {
        let expr = get_attr_expr(ctx, attr_id);
        let thunk = ctx.b.thunk(expr);
        args.push(Arg::Expr(thunk));
    }

    let mut stmt = ctx.b.call_stmt("$.transition", args);

    let blockers = ctx.analysis().attr_expression_blockers(attr_id);
    if !blockers.is_empty() {
        stmt = gen_run_after_blockers(ctx, stmt, &blockers);
    }

    after_update.push(stmt);
}

/// Generate `$.animation(el, () => animateFn, () => params)`.
/// Reference: `AnimateDirective.js`.
pub(crate) fn gen_animate_directive<'a>(
    ctx: &mut Ctx<'a>,
    ad: &svelte_ast::AnimateDirective,
    attr_id: NodeId,
    el_name: &str,
    after_update: &mut Vec<Statement<'a>>,
) {
    let name_expr = ctx.parsed.exprs.remove(&ad.name.start).unwrap();
    let name_thunk = ctx.b.thunk(name_expr);

    let mut args: Vec<Arg<'a, '_>> = vec![
        Arg::Ident(el_name),
        Arg::Expr(name_thunk),
    ];

    if ad.expression_span.is_some() {
        let expr = get_attr_expr(ctx, attr_id);
        let thunk = ctx.b.thunk(expr);
        args.push(Arg::Expr(thunk));
    } else {
        args.push(Arg::Expr(ctx.b.null_expr()));
    }

    let mut stmt = ctx.b.call_stmt("$.animation", args);

    let blockers = ctx.analysis().attr_expression_blockers(attr_id);
    if !blockers.is_empty() {
        stmt = gen_run_after_blockers(ctx, stmt, &blockers);
    }

    after_update.push(stmt);
}

// ---------------------------------------------------------------------------
// Shared: run_after_blockers wrapping
// ---------------------------------------------------------------------------

/// Wrap a statement in `$.run_after_blockers(blockers, () => { stmt })`.
fn gen_run_after_blockers<'a>(ctx: &mut Ctx<'a>, stmt: Statement<'a>, blockers: &[u32]) -> Statement<'a> {
    let blockers_arr = ctx.b.promises_array(blockers);
    let thunk = ctx.b.thunk_block(vec![stmt]);
    ctx.b.call_stmt("$.run_after_blockers", [
        Arg::Expr(blockers_arr),
        Arg::Expr(thunk),
    ])
}

// LEGACY(svelte4): on:directive codegen
// ---------------------------------------------------------------------------

/// Generate `$.event()` calls for legacy `on:directive` syntax.
/// Reference: `OnDirective.js` + `shared/events.js` (`build_event`, `build_event_handler`).
pub(crate) fn gen_on_directive_legacy<'a>(
    ctx: &mut Ctx<'a>,
    od: &svelte_ast::OnDirectiveLegacy,
    attr_id: NodeId,
    el_name: &str,
    after_update: &mut Vec<Statement<'a>>,
) {
    // --- Build event handler ---
    let handler = if od.expression_span.is_none() {
        // Bubble event: function($$arg) { $.bubble_event.call(this, $$props, $$arg) }
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
        build_legacy_event_handler(ctx, expr)
    };

    // --- Apply modifier wrappers (order matches Svelte reference) ---
    let mods = od.parsed_modifiers();
    let mut wrapped = handler;
    for wrapper in mods.handler_wrappers() {
        let mut fn_name = String::with_capacity(2 + wrapper.len());
        fn_name.push_str("$.");
        fn_name.push_str(wrapper);
        wrapped = ctx.b.call_expr(&fn_name, [Arg::Expr(wrapped)]);
    }

    // --- Build $.event() call ---
    let mut args: Vec<Arg<'a, '_>> = vec![
        Arg::StrRef(&od.name),
        Arg::Ident(el_name),
        Arg::Expr(wrapped),
    ];
    if mods.capture || mods.passive.is_some() {
        args.push(Arg::Bool(mods.capture));
    }
    if let Some(p) = mods.passive {
        args.push(Arg::Bool(p));
    }

    after_update.push(ctx.b.call_stmt("$.event", args));
}

/// Build Svelte 5 event attribute handler (non-dev mode).
/// Reference: `events.js` `build_event_handler()`.
///
/// 1. Arrow/Function -> pass through
/// 2. Identifier that is NOT an import -> pass through
/// 3. If `has_call` -> memoize with `$.derived`/`$.get`
/// 4. Remaining -> wrap in `function(...$$args) { handler.apply(this, $$args) }`
pub(crate) fn build_event_handler_s5<'a>(
    ctx: &mut Ctx<'a>,
    attr_id: NodeId,
    handler: Expression<'a>,
    has_call: bool,
    init: &mut Vec<Statement<'a>>,
) -> Expression<'a> {
    match &handler {
        Expression::ArrowFunctionExpression(_) | Expression::FunctionExpression(_) => {
            return handler;
        }
        Expression::Identifier(_) => {
            // Non-import identifiers are stable references — no wrapping needed.
            // Import identifiers may be live bindings, so they need derived+get wrapping.
            let is_import = ctx.attr_is_import(attr_id);
            if !is_import {
                return handler;
            }
        }
        _ => {}
    }

    let mut handler = handler;

    if has_call {
        let id = ctx.gen_ident("event_handler");
        let thunk = ctx.b.thunk(handler);
        init.push(ctx.b.var_stmt(&id, ctx.b.call_expr("$.derived", [Arg::Expr(thunk)])));
        handler = ctx.b.call_expr("$.get", [Arg::Ident(&id)]);
    }

    let call = ctx.b.optional_member_call_expr(handler, "apply", [
        Arg::Expr(ctx.b.this_expr()),
        Arg::Ident("$$args"),
    ]);
    ctx.b.function_expr(ctx.b.rest_params("$$args"), vec![ctx.b.expr_stmt(call)])
}

/// In dev mode, convert arrow event handlers to named functions and wrap `$inspect.trace()`.
/// Reference: `events.js` `build_event()` lines 66-76.
/// `expr_offset` is the byte offset of the expression in the component source.
pub(crate) fn dev_event_handler<'a>(ctx: &mut Ctx<'a>, handler: Expression<'a>, event_name: &str, expr_offset: u32) -> Expression<'a> {
    if !ctx.dev {
        return handler;
    }

    let Expression::ArrowFunctionExpression(arrow) = handler else {
        return handler;
    };
    let arrow = arrow.unbox();

    // Convert arrow body to block statements
    let mut body = arrow.body.unbox();
    let mut stmts: Vec<Statement<'a>> = body.statements.drain(..).collect();

    // Check for $inspect.trace() as first statement
    let has_trace = stmts.first().is_some_and(|s| {
        if let Statement::ExpressionStatement(es) = s {
            is_inspect_trace_call(&es.expression)
        } else {
            false
        }
    });

    if has_trace {
        // Remove the trace statement
        let trace_stmt = stmts.remove(0);
        let Statement::ExpressionStatement(es) = trace_stmt else { unreachable!() };
        let es = es.unbox();
        let Expression::CallExpression(call) = es.expression else { unreachable!() };
        let mut call = call.unbox();

        // Extract explicit label or generate auto-label
        let label_expr = if !call.arguments.is_empty() {
            let mut dummy = oxc_ast::ast::Argument::from(ctx.b.cheap_expr());
            std::mem::swap(&mut call.arguments[0], &mut dummy);
            dummy.into_expression()
        } else {
            let (line, col) = crate::script::compute_line_col(ctx.source, expr_offset + arrow.span.start);
            let sanitized = crate::script::sanitize_location(ctx.filename);
            let label = format!("trace ({sanitized}:{line}:{col})");
            ctx.b.str_expr(&label)
        };
        let label_thunk = ctx.b.thunk(label_expr);

        let body_thunk = if arrow.r#async {
            ctx.b.async_thunk_block(stmts)
        } else {
            ctx.b.thunk_block(stmts)
        };

        let trace_call = ctx.b.call_expr("$.trace", [
            Arg::Expr(label_thunk),
            Arg::Expr(body_thunk),
        ]);
        let return_expr = if arrow.r#async {
            ctx.b.await_expr(trace_call)
        } else {
            trace_call
        };
        stmts = vec![ctx.b.return_stmt(return_expr)];
        ctx.has_tracing = true;
    }

    ctx.b.named_function_expr(
        event_name,
        arrow.params.unbox(),
        stmts,
        arrow.r#async,
    )
}

/// Check if an expression is `$inspect.trace(...)`.
fn is_inspect_trace_call(expr: &Expression) -> bool {
    if let Expression::CallExpression(call) = expr {
        if let Expression::StaticMemberExpression(member) = &call.callee {
            if member.property.name.as_str() == "trace" {
                if let Expression::Identifier(id) = &member.object {
                    return id.name.as_str() == "$inspect";
                }
            }
        }
    }
    false
}

/// Build event handler for legacy on:directive (non-dev mode).
/// Reference: `events.js` `build_event_handler()`.
pub(crate) fn build_legacy_event_handler<'a>(ctx: &mut Ctx<'a>, handler: Expression<'a>) -> Expression<'a> {
    match &handler {
        // Inline arrow/function — pass through
        Expression::ArrowFunctionExpression(_) | Expression::FunctionExpression(_) => handler,
        // Identifier — pass through in non-dev mode
        Expression::Identifier(_) => handler,
        // Other expressions (member, conditional, etc.) — wrap in function(...$$args) { handler.apply(this, $$args) }
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

/// LEGACY(svelte4): Generate `$.event()` for legacy `on:directive` on a global element
/// (svelte:body, svelte:document, svelte:window). `target` is the element expression
/// (e.g. `"$.document.body"`, `"$.document"`, `"$.window"`).
pub(crate) fn gen_legacy_event_on<'a>(
    ctx: &mut Ctx<'a>,
    od: &svelte_ast::OnDirectiveLegacy,
    attr_id: NodeId,
    target: &str,
    stmts: &mut Vec<Statement<'a>>,
) {
    let handler = if od.expression_span.is_none() {
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
        let expr = super::expression::get_attr_expr(ctx, attr_id);
        build_legacy_event_handler(ctx, expr)
    };

    let mods = od.parsed_modifiers();
    let mut wrapped = handler;
    for wrapper in mods.handler_wrappers() {
        let mut fn_name = String::with_capacity(2 + wrapper.len());
        fn_name.push_str("$.");
        fn_name.push_str(wrapper);
        wrapped = ctx.b.call_expr(&fn_name, [Arg::Expr(wrapped)]);
    }

    let mut args: Vec<Arg<'a, '_>> = vec![
        Arg::StrRef(&od.name),
        Arg::Ident(target),
        Arg::Expr(wrapped),
    ];
    if mods.capture || mods.passive.is_some() {
        args.push(Arg::Bool(mods.capture));
    }
    if let Some(p) = mods.passive {
        args.push(Arg::Bool(p));
    }

    stmts.push(ctx.b.call_stmt("$.event", args));
}

/// Generate Svelte 5 event attribute `$.event()` on a global element.
/// `target` is the element expression (e.g. `"$.document.body"`, `"$.window"`).
pub(crate) fn gen_event_attr_on<'a>(
    ctx: &mut Ctx<'a>,
    attr_id: NodeId,
    raw_event_name: &str,
    target: &str,
    stmts: &mut Vec<Statement<'a>>,
    expr_offset: u32,
) {
    let (event_name, capture) = if let Some(base) = svelte_analyze::strip_capture_event(raw_event_name) {
        (base.to_string(), true)
    } else {
        (raw_event_name.to_string(), false)
    };

    let has_call = ctx.analysis().attr_expression(attr_id).map_or(false, |e| e.has_call);
    let handler_expr = super::expression::get_attr_expr(ctx, attr_id);
    let handler = build_event_handler_s5(ctx, attr_id, handler_expr, has_call, stmts);
    let handler = dev_event_handler(ctx, handler, &event_name, expr_offset);

    let passive = svelte_analyze::is_passive_event(&event_name);
    let mut args: Vec<Arg<'a, '_>> = vec![
        Arg::StrRef(&event_name),
        Arg::Ident(target),
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
