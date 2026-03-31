//! SvelteDocument code generation — `<svelte:document on:event bind:prop />`.

use oxc_ast::ast::Statement;

use svelte_ast::{Attribute, NodeId};

use crate::builder::Arg;
use crate::context::Ctx;

use super::bind::build_binding_setter_silent;
use super::events::{gen_event_attr_on, gen_legacy_event_on};

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
                    gen_event_attr_on(ctx, attr_id, event_name, "$.document", stmts, ea.expression_span.start);
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
        ctx.query.component.source_text(span).to_string()
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
                Arg::StrRef("fullscreenElement"),
                Arg::StrRef("fullscreenchange"),
                Arg::Ident("$.document"),
                Arg::Expr(setter),
            ])
        }
        "pointerLockElement" => {
            let setter = build_binding_setter_silent(ctx, var_name, is_rune);
            ctx.b.call_stmt("$.bind_property", [
                Arg::StrRef("pointerLockElement"),
                Arg::StrRef("pointerlockchange"),
                Arg::Ident("$.document"),
                Arg::Expr(setter),
            ])
        }
        "visibilityState" => {
            let setter = build_binding_setter_silent(ctx, var_name, is_rune);
            ctx.b.call_stmt("$.bind_property", [
                Arg::StrRef("visibilityState"),
                Arg::StrRef("visibilitychange"),
                Arg::Ident("$.document"),
                Arg::Expr(setter),
            ])
        }
        _ => return,
    };

    stmts.push(stmt);
}
