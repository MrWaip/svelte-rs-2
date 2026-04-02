//! SvelteBody code generation — `<svelte:body onclick={handler} use:action />`.

use oxc_ast::ast::Statement;

use svelte_ast::{Attribute, NodeId};

use crate::context::Ctx;

use super::events::{gen_event_attr_on, gen_legacy_event_on, gen_use_directive_on};

/// Generate event listeners and actions for `<svelte:body>`.
///
/// Events → `$.event(name, $.document.body, handler)` pushed to init.
/// Actions → `$.action($.document.body, handler, thunk)`.
pub(crate) fn gen_svelte_body<'a>(ctx: &mut Ctx<'a>, id: NodeId, stmts: &mut Vec<Statement<'a>>) {
    let body = ctx.svelte_body(id);
    let attrs: Vec<_> = body.attributes.clone();

    for attr in &attrs {
        match attr {
            Attribute::OnDirectiveLegacy(od) => {
                let attr_id = attr.id();
                gen_legacy_event_on(ctx, od, attr_id, "$.document.body", stmts);
            }
            Attribute::ExpressionAttribute(ea) => {
                if let Some(event_name) = ea.event_name.as_deref() {
                    let attr_id = attr.id();
                    gen_event_attr_on(
                        ctx,
                        attr_id,
                        event_name,
                        "$.document.body",
                        stmts,
                        ea.expression_span.start,
                    );
                }
            }
            Attribute::UseDirective(ud) => {
                let attr_id = attr.id();
                gen_use_directive_on(ctx, ud, attr_id, "$.document.body", stmts);
            }
            _ => {}
        }
    }
}
