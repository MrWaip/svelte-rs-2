//! SvelteWindow code generation — `<svelte:window on:event bind:prop />`.

use oxc_ast::ast::Statement;

use svelte_analyze::{BindPropertyKind, WindowBindKind};
use svelte_ast::{Attribute, NodeId};

use crate::context::Ctx;
use svelte_ast_builder::Arg;

use super::bind::{build_binding_getter, build_binding_setter_silent, gen_bind_property_stmt};
use super::events::{gen_event_attr_on, gen_legacy_event_on};

/// Generate event listeners and bindings for `<svelte:window>`.
///
/// Events → `$.event(name, $.window, handler)` pushed to init.
/// Bindings → `$.bind_window_scroll` / `$.bind_window_size` / `$.bind_online` / `$.bind_property`.
pub(crate) fn gen_svelte_window<'a>(ctx: &mut Ctx<'a>, id: NodeId, stmts: &mut Vec<Statement<'a>>) {
    let window = ctx.svelte_window(id);
    let attrs: Vec<_> = window.attributes.clone();

    for attr in &attrs {
        match attr {
            Attribute::OnDirectiveLegacy(od) => {
                let attr_id = attr.id();
                gen_legacy_event_on(ctx, od, attr_id, "$.window", stmts);
            }
            Attribute::ExpressionAttribute(ea) => {
                if let Some(event_name) = ea.event_name.as_deref() {
                    let attr_id = attr.id();
                    gen_event_attr_on(
                        ctx,
                        attr_id,
                        event_name,
                        "$.window",
                        stmts,
                        ea.expression_span.start,
                    );
                }
            }
            Attribute::BindDirective(bind) => {
                gen_window_binding(ctx, bind, stmts);
            }
            _ => {}
        }
    }
}

/// Generate window-specific bind directives.
fn gen_window_binding<'a>(
    ctx: &mut Ctx<'a>,
    bind: &svelte_ast::BindDirective,
    stmts: &mut Vec<Statement<'a>>,
) {
    let Some(bind_semantics) = ctx.bind_target_semantics(bind.id) else {
        return;
    };
    let is_rune = matches!(
        ctx.directive_root_reference_semantics(bind.id),
        svelte_analyze::ReferenceSemantics::SignalWrite { .. }
            | svelte_analyze::ReferenceSemantics::SignalUpdate { .. }
            | svelte_analyze::ReferenceSemantics::SignalRead { .. }
    );

    let var_name = if bind.shorthand {
        bind.name.clone()
    } else {
        ctx.query
            .component
            .source_text(bind.expression_span)
            .to_string()
    };

    let stmt = match bind_semantics.property() {
        BindPropertyKind::Window(WindowBindKind::ScrollX) => {
            let getter = build_binding_getter(ctx, &var_name, is_rune);
            let setter = build_binding_setter_silent(ctx, var_name, is_rune);
            ctx.b.call_stmt(
                "$.bind_window_scroll",
                [Arg::StrRef("x"), Arg::Expr(getter), Arg::Expr(setter)],
            )
        }
        BindPropertyKind::Window(WindowBindKind::ScrollY) => {
            let getter = build_binding_getter(ctx, &var_name, is_rune);
            let setter = build_binding_setter_silent(ctx, var_name, is_rune);
            ctx.b.call_stmt(
                "$.bind_window_scroll",
                [Arg::StrRef("y"), Arg::Expr(getter), Arg::Expr(setter)],
            )
        }
        BindPropertyKind::Window(
            kind @ (WindowBindKind::InnerWidth
            | WindowBindKind::InnerHeight
            | WindowBindKind::OuterWidth
            | WindowBindKind::OuterHeight),
        ) => {
            let setter = build_binding_setter_silent(ctx, var_name, is_rune);
            ctx.b.call_stmt(
                "$.bind_window_size",
                [Arg::StrRef(kind.name()), Arg::Expr(setter)],
            )
        }
        BindPropertyKind::Window(WindowBindKind::Online) => {
            let setter = build_binding_setter_silent(ctx, var_name, is_rune);
            ctx.b.call_stmt("$.bind_online", [Arg::Expr(setter)])
        }
        BindPropertyKind::Window(WindowBindKind::DevicePixelRatio) => {
            let setter = build_binding_setter_silent(ctx, var_name, is_rune);
            gen_bind_property_stmt(ctx, "devicePixelRatio", "resize", "$.window", setter, None)
        }
        _ => return,
    };

    stmts.push(stmt);
}
