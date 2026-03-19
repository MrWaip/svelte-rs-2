//! SvelteWindow code generation — `<svelte:window on:event bind:prop />`.

use oxc_ast::ast::Statement;

use svelte_ast::{Attribute, NodeId};

use crate::builder::Arg;
use crate::context::Ctx;

use super::attributes::{
    build_binding_getter, build_binding_setter_silent, gen_event_attr_on, gen_legacy_event_on,
};

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
                gen_legacy_event_on(ctx, od, attr_id, "$.window", stmts);
            }
            Attribute::ExpressionAttribute(ea) => {
                if let Some(event_name) = ea.event_name.as_deref() {
                    let attr_id = attr.id();
                    gen_event_attr_on(ctx, attr_id, event_name, "$.window", stmts);
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
    let is_rune = ctx.is_mutable_rune_target(bind.id);

    let var_name = if bind.shorthand {
        bind.name.clone()
    } else if let Some(span) = bind.expression_span {
        ctx.component.source_text(span).to_string()
    } else {
        return;
    };

    let stmt = match bind.name.as_str() {
        "scrollX" | "scrollY" => {
            let axis = if bind.name == "scrollX" { "x" } else { "y" };
            let getter = build_binding_getter(ctx, &var_name, is_rune);
            let setter = build_binding_setter_silent(ctx, var_name, is_rune);
            ctx.b.call_stmt("$.bind_window_scroll", [
                Arg::Str(axis.to_string()),
                Arg::Expr(getter),
                Arg::Expr(setter),
            ])
        }
        "innerWidth" | "innerHeight" | "outerWidth" | "outerHeight" => {
            let setter = build_binding_setter_silent(ctx, var_name, is_rune);
            ctx.b.call_stmt("$.bind_window_size", [
                Arg::Str(bind.name.clone()),
                Arg::Expr(setter),
            ])
        }
        "online" => {
            let setter = build_binding_setter_silent(ctx, var_name, is_rune);
            ctx.b.call_stmt("$.bind_online", [Arg::Expr(setter)])
        }
        "devicePixelRatio" => {
            let setter = build_binding_setter_silent(ctx, var_name, is_rune);
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
