use oxc_ast::ast::{Expression, Statement};

use svelte_analyze::DeclarationSemantics;
use svelte_ast::{Attribute, NodeId};

use svelte_ast_builder::{Arg, ObjProp};
use crate::context::Ctx;

use super::expression::{build_attr_concat, get_attr_expr, MemoValueRef, TemplateMemoState};
use super::gen_fragment;

pub(crate) fn is_legacy_slot_element(ctx: &Ctx<'_>, el_id: NodeId) -> bool {
    matches!(
        ctx.query.component.store.get(el_id),
        svelte_ast::Node::SlotElementLegacy(_)
    )
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
    emit_slot_call(ctx, el_id, &node_name, body);
    body.push(ctx.b.call_stmt(
        "$.append",
        [Arg::Ident("$$anchor"), Arg::Ident(&fragment_name)],
    ));
}

pub(crate) fn emit_slot_call<'a>(
    ctx: &mut Ctx<'a>,
    el_id: NodeId,
    anchor_name: &str,
    body: &mut Vec<Statement<'a>>,
) {
    let slot_name = legacy_slot_name(ctx, el_id);
    let fallback = legacy_slot_fallback(ctx, el_id);
    let mut props = legacy_slot_props(ctx, el_id);
    let slot_stmt = ctx.b.call_stmt(
        "$.slot",
        [
            Arg::Ident(anchor_name),
            Arg::Ident("$$props"),
            Arg::StrRef(slot_name),
            Arg::Expr(props.expr),
            Arg::Expr(fallback),
        ],
    );

    let mut prelude = props.memo.derived_stmts(ctx, ctx.query.runes());
    if props.memo.has_async_values() || props.memo.has_blockers() {
        prelude.push(slot_stmt);
        let mut callback_params = vec![anchor_name.to_string()];
        callback_params.extend(props.memo.async_param_names());
        let callback = ctx.b.arrow_block_expr(
            ctx.b
                .params(callback_params.iter().map(|name| name.as_str())),
            prelude,
        );
        body.push(ctx.b.call_stmt(
            "$.async",
            [
                Arg::Ident(anchor_name),
                Arg::Expr(props.memo.blockers_expr(ctx)),
                Arg::Expr(props.memo.async_values_expr(ctx)),
                Arg::Expr(callback),
            ],
        ));
        return;
    }

    if prelude.is_empty() {
        body.push(slot_stmt);
    } else {
        prelude.push(slot_stmt);
        body.push(ctx.b.block_stmt(prelude));
    }
}

struct LegacySlotProps<'a> {
    expr: Expression<'a>,
    memo: TemplateMemoState<'a>,
}

fn legacy_slot_props<'a>(ctx: &mut Ctx<'a>, el_id: NodeId) -> LegacySlotProps<'a> {
    let attrs = match ctx.query.component.store.get(el_id) {
        svelte_ast::Node::SlotElementLegacy(el) => &el.attributes,
        _ => unreachable!("legacy slot helper requires SlotElementLegacy"),
    };

    let mut props = Vec::new();
    let mut spreads = Vec::new();
    let mut memo = TemplateMemoState::default();
    for attr in attrs {
        let attr_id = attr.id();
        match attr {
            Attribute::StringAttribute(a) => {
                if a.name == "name" || a.name == "slot" {
                    continue;
                }
                let name = ctx.b.alloc_str(&a.name);
                let value = ctx
                    .b
                    .str_expr(ctx.query.component.source_text(a.value_span));
                props.push(ObjProp::KeyValue(name, value));
            }
            Attribute::BooleanAttribute(a) => {
                if a.name == "name" || a.name == "slot" {
                    continue;
                }
                let name = ctx.b.alloc_str(&a.name);
                props.push(ObjProp::KeyValue(name, ctx.b.bool_expr(true)));
            }
            Attribute::ExpressionAttribute(a) => {
                if a.name == "name" || a.name == "slot" {
                    continue;
                }
                let value = get_attr_expr(ctx, attr_id);
                let expr = legacy_slot_attr_value(ctx, attr_id, value, &mut memo);
                let name = ctx.b.alloc_str(&a.name);
                let prop = if legacy_slot_prop_needs_getter(ctx, attr_id) {
                    ObjProp::Getter(name, expr)
                } else {
                    ObjProp::KeyValue(name, expr)
                };
                props.push(prop);
            }
            Attribute::ConcatenationAttribute(a) => {
                if a.name == "name" || a.name == "slot" {
                    continue;
                }
                let value = build_attr_concat(ctx, attr_id, &a.parts);
                let expr = legacy_slot_attr_value(ctx, attr_id, value, &mut memo);
                let name = ctx.b.alloc_str(&a.name);
                let prop = if legacy_slot_prop_needs_getter(ctx, attr_id) {
                    ObjProp::Getter(name, expr)
                } else {
                    ObjProp::KeyValue(name, expr)
                };
                props.push(prop);
            }
            Attribute::SpreadAttribute(_) => {
                let value = get_attr_expr(ctx, attr_id);
                spreads.push(ctx.b.thunk(value));
            }
            _ => {}
        }
    }

    let expr = if spreads.is_empty() {
        ctx.b.object_expr(props)
    } else {
        let mut args = Vec::with_capacity(spreads.len() + 1);
        args.push(Arg::Expr(ctx.b.object_expr(props)));
        args.extend(spreads.into_iter().map(Arg::Expr));
        ctx.b.call_expr("$.spread_props", args)
    };

    LegacySlotProps { expr, memo }
}

fn legacy_slot_attr_value<'a>(
    ctx: &mut Ctx<'a>,
    attr_id: NodeId,
    expr: Expression<'a>,
    memo: &mut TemplateMemoState<'a>,
) -> Expression<'a> {
    let Some(info) = ctx.attr_expression(attr_id) else {
        return expr;
    };
    let memo_expr = if info.has_call() && !info.has_await() {
        ctx.b.call_expr(
            "$.untrack",
            [Arg::Expr(ctx.b.thunk(ctx.b.clone_expr(&expr)))],
        )
    } else {
        ctx.b.clone_expr(&expr)
    };

    match memo.add_memoized_expr(ctx, info, memo_expr) {
        Some(MemoValueRef::Sync(index)) => memo.sync_derived_expr(ctx, index),
        Some(MemoValueRef::Async(index)) => memo.async_param_expr(ctx, index),
        None => expr,
    }
}

fn legacy_slot_prop_needs_getter(ctx: &Ctx<'_>, attr_id: NodeId) -> bool {
    let Some(info) = ctx.attr_expression(attr_id) else {
        return false;
    };

    ctx.has_state_attr(attr_id)
        || info.has_call()
        || info.has_await()
        || info.ref_symbols().iter().any(|&sym| {
            ctx.query.scoping().is_mutated(sym)
                || symbol_needs_slot_getter(ctx, sym)
        })
}

/// Does reading `sym` require a getter wrapper in a legacy `<slot>` prop?
/// True for any reactive declaration kind (state/derived/prop/store/const)
/// and for any contextual binding (each/snippet/await/let:).
fn symbol_needs_slot_getter(ctx: &Ctx<'_>, sym: svelte_analyze::scope::SymbolId) -> bool {
    let decl = ctx
        .query
        .view
        .declaration_semantics(ctx.query.scoping().symbol_declaration(sym));
    matches!(
        decl,
        DeclarationSemantics::State(_)
            | DeclarationSemantics::Derived(_)
            | DeclarationSemantics::Prop(_)
            | DeclarationSemantics::Store(_)
            | DeclarationSemantics::Const(_)
            | DeclarationSemantics::Contextual(_),
    )
}

fn legacy_slot_name<'a>(ctx: &Ctx<'a>, el_id: NodeId) -> &'a str {
    let attrs = match ctx.query.component.store.get(el_id) {
        svelte_ast::Node::SlotElementLegacy(el) => &el.attributes,
        _ => unreachable!("legacy slot helper requires SlotElementLegacy"),
    };
    for attr in attrs {
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
