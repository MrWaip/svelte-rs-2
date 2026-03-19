//! SvelteDocument code generation — `<svelte:document on:event bind:prop />`.

use oxc_ast::ast::Statement;

use svelte_ast::{Attribute, NodeId};

use crate::builder::Arg;
use crate::context::Ctx;

use super::attributes::{
    build_binding_setter_silent, gen_event_attr_on, gen_legacy_event_on,
};

/// Generate event listeners and bindings for `<svelte:document>`.
///
/// Events → `$.event(name, $.document, handler)` pushed to init.
/// Bindings → `$.bind_active_element` / `$.bind_property`.
pub(crate) fn gen_svelte_document<'a>(
    ctx: &mut Ctx<'a>,
    id: NodeId,
    stmts: &mut Vec<Statement<'a>>,
) {
    let doc = ctx.svelte_document(id);
    let attrs: Vec<_> = doc.attributes.clone();

    for attr in &attrs {
        match attr {
            Attribute::OnDirectiveLegacy(od) => {
                let attr_id = attr.id();
                gen_legacy_event_on(ctx, od, attr_id, "$.document", stmts);
            }
            Attribute::ExpressionAttribute(ea) => {
                if let Some(event_name) = ea.event_name.as_deref() {
                    let attr_id = attr.id();
                    gen_event_attr_on(ctx, attr_id, event_name, "$.document", stmts);
                }
            }
            Attribute::BindDirective(bind) => {
                gen_document_binding(ctx, bind, stmts);
            }
            _ => {}
        }
    }
}

/// Generate document-specific bind directives.
fn gen_document_binding<'a>(
    ctx: &mut Ctx<'a>,
    bind: &svelte_ast::BindDirective,
    stmts: &mut Vec<Statement<'a>>,
) {
    let is_rune = ctx.is_mutable_rune_target(bind.id);

    let var_name = if bind.shorthand {
        bind.name.clone()
    } else if let Some(span) = bind.expression_span {
        ctx.component.source_text(span).trim().to_string()
    } else {
        return;
    };

    let stmt = match bind.name.as_str() {
        "activeElement" => {
            let setter = build_binding_setter_silent(ctx, var_name, is_rune);
            ctx.b.call_stmt("$.bind_active_element", [Arg::Expr(setter)])
        }
        "fullscreenElement" => {
            let setter = build_binding_setter_silent(ctx, var_name, is_rune);
            ctx.b.call_stmt("$.bind_property", [
                Arg::Str("fullscreenElement".to_string()),
                Arg::Str("fullscreenchange".to_string()),
                Arg::Ident("$.document"),
                Arg::Expr(setter),
            ])
        }
        "pointerLockElement" => {
            let setter = build_binding_setter_silent(ctx, var_name, is_rune);
            ctx.b.call_stmt("$.bind_property", [
                Arg::Str("pointerLockElement".to_string()),
                Arg::Str("pointerlockchange".to_string()),
                Arg::Ident("$.document"),
                Arg::Expr(setter),
            ])
        }
        "visibilityState" => {
            let setter = build_binding_setter_silent(ctx, var_name, is_rune);
            ctx.b.call_stmt("$.bind_property", [
                Arg::Str("visibilityState".to_string()),
                Arg::Str("visibilitychange".to_string()),
                Arg::Ident("$.document"),
                Arg::Expr(setter),
            ])
        }
        _ => return,
    };

    stmts.push(stmt);
}
