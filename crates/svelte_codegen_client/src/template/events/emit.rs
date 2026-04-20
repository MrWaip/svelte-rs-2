use oxc_ast::ast::Statement;

use svelte_ast::NodeId;

use crate::context::Ctx;
use svelte_ast_builder::Arg;

use super::handlers::{build_event_handler_s5, build_legacy_event_handler, dev_event_handler};

pub(crate) fn gen_on_directive_legacy<'a>(
    ctx: &mut Ctx<'a>,
    od: &svelte_ast::OnDirectiveLegacy,
    attr_id: NodeId,
    el_name: &str,
    after_update: &mut Vec<Statement<'a>>,
) {
    let expr_offset = od.expression_span.map(|span| span.start);
    let has_call = ctx.attr_expression(attr_id).is_some_and(|e| e.has_call());
    let handler = if od.expression_span.is_none() {
        let bubble_call = ctx
            .b
            .static_member_expr(ctx.b.rid_expr("$.bubble_event"), "call");
        let call = ctx.b.call_expr_callee(
            bubble_call,
            [
                Arg::Expr(ctx.b.this_expr()),
                Arg::Ident("$$props"),
                Arg::Ident("$$arg"),
            ],
        );
        ctx.b
            .function_expr(ctx.b.params(["$$arg"]), vec![ctx.b.expr_stmt(call)])
    } else {
        let expr = super::super::expression::get_attr_expr(ctx, attr_id);
        build_legacy_event_handler(
            ctx,
            attr_id,
            expr,
            has_call,
            after_update,
            expr_offset.expect("legacy event expression span missing"),
        )
    };
    let handler = if let Some(offset) = expr_offset {
        dev_event_handler(ctx, handler, &od.name, offset)
    } else {
        handler
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
        Arg::Ident(el_name),
        Arg::Expr(wrapped),
    ];
    if mods.capture || mods.passive.is_some() {
        args.push(if mods.capture {
            Arg::Bool(true)
        } else {
            Arg::Expr(ctx.b.void_zero_expr())
        });
    }
    if let Some(p) = mods.passive {
        args.push(Arg::Bool(p));
    }

    after_update.push(ctx.b.call_stmt("$.event", args));
}

pub(crate) fn gen_legacy_event_on<'a>(
    ctx: &mut Ctx<'a>,
    od: &svelte_ast::OnDirectiveLegacy,
    attr_id: NodeId,
    target: &str,
    stmts: &mut Vec<Statement<'a>>,
) {
    let expr_offset = od.expression_span.map(|span| span.start);
    let has_call = ctx.attr_expression(attr_id).is_some_and(|e| e.has_call());
    let handler = if od.expression_span.is_none() {
        let bubble_call = ctx
            .b
            .static_member_expr(ctx.b.rid_expr("$.bubble_event"), "call");
        let call = ctx.b.call_expr_callee(
            bubble_call,
            [
                Arg::Expr(ctx.b.this_expr()),
                Arg::Ident("$$props"),
                Arg::Ident("$$arg"),
            ],
        );
        ctx.b
            .function_expr(ctx.b.params(["$$arg"]), vec![ctx.b.expr_stmt(call)])
    } else {
        let expr = super::super::expression::get_attr_expr(ctx, attr_id);
        build_legacy_event_handler(
            ctx,
            attr_id,
            expr,
            has_call,
            stmts,
            expr_offset.expect("legacy event expression span missing"),
        )
    };
    let handler = if let Some(offset) = expr_offset {
        dev_event_handler(ctx, handler, &od.name, offset)
    } else {
        handler
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
        args.push(if mods.capture {
            Arg::Bool(true)
        } else {
            Arg::Expr(ctx.b.void_zero_expr())
        });
    }
    if let Some(p) = mods.passive {
        args.push(Arg::Bool(p));
    }

    stmts.push(ctx.b.call_stmt("$.event", args));
}

pub(crate) fn gen_event_attr_on<'a>(
    ctx: &mut Ctx<'a>,
    attr_id: NodeId,
    raw_event_name: &str,
    target: &str,
    stmts: &mut Vec<Statement<'a>>,
    expr_offset: u32,
) {
    let (event_name, capture) =
        if let Some(base) = svelte_analyze::strip_capture_event(raw_event_name) {
            (base.to_string(), true)
        } else {
            (raw_event_name.to_string(), false)
        };

    let has_call = ctx.attr_expression(attr_id).is_some_and(|e| e.has_call());
    let handler_expr = super::super::expression::get_attr_expr(ctx, attr_id);
    let handler = build_event_handler_s5(ctx, attr_id, handler_expr, has_call, stmts, expr_offset);
    let handler = dev_event_handler(ctx, handler, &event_name, expr_offset);

    let passive = svelte_analyze::is_passive_event(&event_name);
    let mut args: Vec<Arg<'a, '_>> = vec![
        Arg::StrRef(&event_name),
        Arg::Ident(target),
        Arg::Expr(handler),
    ];
    if capture || passive {
        args.push(if capture {
            Arg::Bool(true)
        } else {
            Arg::Expr(ctx.b.void_zero_expr())
        });
    }
    if passive {
        args.push(Arg::Bool(true));
    }
    stmts.push(ctx.b.call_stmt("$.event", args));
}
