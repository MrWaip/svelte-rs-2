//! SvelteDocument code generation — `<svelte:document on:event bind:prop />`.

use oxc_ast::ast::Statement;

use svelte_analyze::{BindPropertyKind, DocumentBindKind};
use svelte_ast::{Attribute, NodeId};

use crate::builder::Arg;
use crate::context::Ctx;

use super::bind::{build_binding_setter_silent, gen_bind_property_stmt};
use super::events::{gen_attach_tag, gen_event_attr_on, gen_legacy_event_on};

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
                    gen_event_attr_on(
                        ctx,
                        attr_id,
                        event_name,
                        "$.document",
                        stmts,
                        ea.expression_span.start,
                    );
                }
            }
            Attribute::BindDirective(bind) => {
                gen_document_binding(ctx, bind, stmts);
            }
            Attribute::AttachTag(_) => {
                let attr_id = attr.id();
                gen_attach_tag(ctx, attr_id, "$.document", stmts);
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
    let Some(bind_semantics) = ctx.bind_target_semantics(bind.id) else {
        return;
    };
    let is_rune = ctx.is_mutable_rune_target(bind.id);

    let var_name = if bind.shorthand {
        bind.name.clone()
    } else if let Some(span) = bind.expression_span {
        ctx.query.component.source_text(span).to_string()
    } else {
        return;
    };

    let stmt = match bind_semantics.property() {
        BindPropertyKind::Document(DocumentBindKind::ActiveElement) => {
            let setter = build_binding_setter_silent(ctx, var_name, is_rune);
            ctx.b
                .call_stmt("$.bind_active_element", [Arg::Expr(setter)])
        }
        BindPropertyKind::Document(DocumentBindKind::FullscreenElement) => {
            let setter = build_binding_setter_silent(ctx, var_name, is_rune);
            gen_bind_property_stmt(
                ctx,
                "fullscreenElement",
                "fullscreenchange",
                "$.document",
                setter,
                None,
            )
        }
        BindPropertyKind::Document(DocumentBindKind::PointerLockElement) => {
            let setter = build_binding_setter_silent(ctx, var_name, is_rune);
            gen_bind_property_stmt(
                ctx,
                "pointerLockElement",
                "pointerlockchange",
                "$.document",
                setter,
                None,
            )
        }
        BindPropertyKind::Document(DocumentBindKind::VisibilityState) => {
            let setter = build_binding_setter_silent(ctx, var_name, is_rune);
            gen_bind_property_stmt(
                ctx,
                "visibilityState",
                "visibilitychange",
                "$.document",
                setter,
                None,
            )
        }
        _ => return,
    };

    stmts.push(stmt);
}
