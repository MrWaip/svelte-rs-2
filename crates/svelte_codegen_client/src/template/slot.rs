use oxc_ast::ast::{Expression, Statement};

use svelte_ast::{Attribute, NodeId};

use crate::builder::{Arg, ObjProp};
use crate::context::Ctx;

use super::gen_fragment;

pub(crate) fn is_legacy_slot_element(ctx: &Ctx<'_>, el_id: NodeId) -> bool {
    !ctx.query.view.custom_element() && ctx.element(el_id).name == "slot"
}

pub(crate) fn emit_slot_template_anchor<'a>(
    ctx: &mut Ctx<'a>,
    el_id: NodeId,
    body: &mut Vec<Statement<'a>>,
) {
    let fragment_name = ctx.gen_ident("fragment");
    let node_name = ctx.gen_ident("node");
    body.push(
        ctx.b
            .var_stmt(&fragment_name, ctx.b.call_expr("$.comment", [])),
    );
    body.push(
        ctx.b.var_stmt(
            &node_name,
            ctx.b
                .call_expr("$.first_child", [Arg::Ident(&fragment_name)]),
        ),
    );
    emit_slot_call(ctx, el_id, ctx.b.rid_expr(&node_name), body);
    body.push(ctx.b.call_stmt(
        "$.append",
        [Arg::Ident("$$anchor"), Arg::Ident(&fragment_name)],
    ));
}

pub(crate) fn emit_slot_call<'a>(
    ctx: &mut Ctx<'a>,
    el_id: NodeId,
    anchor: Expression<'a>,
    body: &mut Vec<Statement<'a>>,
) {
    let slot_name = legacy_slot_name(ctx, el_id);
    let fallback = legacy_slot_fallback(ctx, el_id);
    let props = ctx.b.object_expr(std::iter::empty::<ObjProp<'a>>());

    body.push(ctx.b.call_stmt(
        "$.slot",
        [
            Arg::Expr(anchor),
            Arg::Ident("$$props"),
            Arg::StrRef(slot_name),
            Arg::Expr(props),
            Arg::Expr(fallback),
        ],
    ));
}

fn legacy_slot_name<'a>(ctx: &Ctx<'a>, el_id: NodeId) -> &'a str {
    let el = ctx.element(el_id);
    for attr in &el.attributes {
        if let Attribute::StringAttribute(attr) = attr {
            if attr.name == "name" {
                return ctx
                    .b
                    .alloc_str(ctx.query.component.source_text(attr.value_span));
            }
        }
    }
    "default"
}

fn legacy_slot_fallback<'a>(ctx: &mut Ctx<'a>, el_id: NodeId) -> Expression<'a> {
    let child_key = svelte_analyze::FragmentKey::Element(el_id);
    if matches!(
        ctx.content_type(&child_key),
        svelte_analyze::ContentStrategy::Empty
    ) {
        return ctx.b.null_expr();
    }

    let body = gen_fragment(ctx, child_key);
    ctx.b.arrow_block_expr(ctx.b.params(["$$anchor"]), body)
}
