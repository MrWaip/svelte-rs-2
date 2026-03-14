//! Attribute processing (per-attribute and spread).

use oxc_ast::ast::Statement;

use svelte_ast::{Attribute, Element, NodeId};

use crate::builder::{Arg, AssignLeft, AssignRight, ObjProp};
use crate::context::Ctx;

use super::expression::{build_attr_concat, get_attr_expr};

/// Process a single attribute (non-spread path).
pub(crate) fn process_attr<'a>(
    ctx: &mut Ctx<'a>,
    attr: &Attribute,
    owner_id: NodeId,
    attr_idx: usize,
    el_name: &str,
    tag_name: &str,
    is_dyn: bool,
    init: &mut Vec<Statement<'a>>,
    update: &mut Vec<Statement<'a>>,
    after_update: &mut Vec<Statement<'a>>,
) {
    let target = if is_dyn {
        update as &mut Vec<_>
    } else {
        init as &mut Vec<_>
    };

    match attr {
        Attribute::StringAttribute(_) | Attribute::BooleanAttribute(_) => {
            // Static — already in template HTML
        }
        Attribute::ExpressionAttribute(a) => {
            if let Some(event_name) = a.name.strip_prefix("on") {
                if is_delegatable_event(event_name) {
                    let event_str = event_name.to_string();
                    let val = get_attr_expr(ctx, owner_id, attr_idx);
                    after_update.push(ctx.b.call_stmt(
                        "$.delegated",
                        [
                            Arg::Str(event_str.clone()),
                            Arg::Ident(el_name),
                            Arg::Expr(val),
                        ],
                    ));
                    if !ctx.delegated_events.contains(&event_str) {
                        ctx.delegated_events.push(event_str);
                    }
                    return;
                }
            }
            let val = get_attr_expr(ctx, owner_id, attr_idx);
            if a.name == "value" && tag_name == "input" {
                target.push(ctx.b.call_stmt(
                    "$.set_value",
                    [Arg::Ident(el_name), Arg::Expr(val)],
                ));
            } else {
                target.push(ctx.b.call_stmt(
                    "$.set_attribute",
                    [
                        Arg::Ident(el_name),
                        Arg::Str(a.name.clone()),
                        Arg::Expr(val),
                    ],
                ));
            }
        }
        Attribute::ConcatenationAttribute(a) => {
            let val = build_attr_concat(ctx, owner_id, attr_idx, &a.parts);
            target.push(ctx.b.call_stmt(
                "$.set_attribute",
                [
                    Arg::Ident(el_name),
                    Arg::Str(a.name.clone()),
                    Arg::Expr(val),
                ],
            ));
        }
        Attribute::ShorthandOrSpread(a) if !a.is_spread => {
            let val = get_attr_expr(ctx, owner_id, attr_idx);
            let name = ctx.component.source_text(a.expression_span).to_string();
            target.push(ctx.b.call_stmt(
                "$.set_attribute",
                [Arg::Ident(el_name), Arg::Str(name), Arg::Expr(val)],
            ));
        }
        Attribute::BindDirective(bind) => {
            if let Some(stmt) = gen_bind_directive(ctx, bind, el_name) {
                after_update.push(stmt);
            }
        }
        Attribute::ShorthandOrSpread(_) | Attribute::ClassDirective(_) => {
            // Spread handled by process_attrs_spread; class directives by process_class_directives
        }
    }
}

/// Generate `$.set_class(el, 1, "static-class", null, classes_N, { ... })` for class:name directives.
/// Pushes `let classes_N;` to `init` and the assignment to `update` for combined template_effect.
pub(crate) fn process_class_directives<'a>(
    ctx: &mut Ctx<'a>,
    el: &Element,
    el_name: &str,
    init: &mut Vec<Statement<'a>>,
    update: &mut Vec<Statement<'a>>,
) {
    if !ctx.analysis.element_has_class_directives.contains(&el.id) {
        return;
    }

    let class_dirs: Vec<_> = el
        .attributes
        .iter()
        .enumerate()
        .filter_map(|(idx, a)| match a {
            Attribute::ClassDirective(cd) => Some((idx, cd)),
            _ => None,
        })
        .collect();

    // Find static class value from precomputed span
    let static_class = ctx
        .analysis
        .element_static_class
        .get(&el.id)
        .map(|span| ctx.component.source_text(*span).to_string())
        .unwrap_or_default();

    let mut props: Vec<ObjProp<'a>> = Vec::new();

    for (attr_idx, cd) in &class_dirs {
        let name = &cd.name;

        let (expr, is_simple_ident_same_name) = if cd.expression_span.is_some() {
            let parsed = get_attr_expr(ctx, el.id, *attr_idx);
            // Check if expression text matches the class name (for shorthand output)
            let same_name = cd.expression_span
                .map(|span| ctx.component.source_text(span).trim() == name.as_str())
                .unwrap_or(false);
            (parsed, same_name)
        } else {
            // shorthand: class:name → identifier `name`
            (ctx.b.rid_expr(name), true)
        };

        let is_mutated_rune = ctx.analysis.is_mutable_rune(name);

        let name_alloc = ctx.b.alloc_str(name);

        if is_mutated_rune {
            let get_call = ctx.b.call_expr("$.get", [Arg::Ident(name)]);
            props.push(ObjProp::KeyValue(name_alloc, get_call));
        } else if is_simple_ident_same_name {
            props.push(ObjProp::Shorthand(name_alloc));
        } else {
            props.push(ObjProp::KeyValue(name_alloc, expr));
        }
    }

    let obj = ctx.b.object_expr(props);
    let classes_name = ctx.gen_ident("classes");

    // $.set_class(el, 1, "static-class", null, classes_N, { ... })
    let set_class_call = ctx.b.call_expr(
        "$.set_class",
        [
            Arg::Ident(el_name),
            Arg::Num(1.0),
            Arg::Str(static_class),
            Arg::Expr(ctx.b.null_expr()),
            Arg::Ident(&classes_name),
            Arg::Expr(obj),
        ],
    );

    // classes_N = $.set_class(...)
    let assign = ctx.b.assign_expr(
        AssignLeft::Ident(classes_name.clone()),
        AssignRight::Expr(set_class_call),
    );

    // let classes_N;
    init.push(ctx.b.let_stmt(&classes_name));
    // Push assignment to update vector for combined template_effect
    update.push(ctx.b.expr_stmt(assign));
}

/// Generate a bind directive statement (getter/setter + runtime call).
fn gen_bind_directive<'a>(
    ctx: &mut Ctx<'a>,
    bind: &svelte_ast::BindDirective,
    el_name: &str,
) -> Option<Statement<'a>> {
    let var_name = if bind.shorthand {
        bind.name.clone()
    } else if let Some(span) = bind.expression_span {
        ctx.component.source_text(span).trim().to_string()
    } else {
        return None;
    };

    let is_mutated_rune = ctx.analysis.is_mutable_rune(&var_name);

    let getter_body = if is_mutated_rune {
        ctx.b.call_expr("$.get", [Arg::Ident(&var_name)])
    } else {
        ctx.b.rid_expr(&var_name)
    };
    let getter = ctx
        .b
        .arrow_expr(ctx.b.no_params(), [ctx.b.expr_stmt(getter_body)]);

    let setter_body = if is_mutated_rune {
        ctx.b
            .call_expr("$.set", [Arg::Ident(&var_name), Arg::Ident("$$value")])
    } else {
        ctx.b.assign_expr(
            AssignLeft::Ident(var_name),
            AssignRight::Expr(ctx.b.rid_expr("$$value")),
        )
    };
    let setter = ctx.b.arrow_expr(
        ctx.b.params(["$$value"]),
        [ctx.b.expr_stmt(setter_body)],
    );

    let stmt = match bind.name.as_str() {
        "checked" => ctx.b.call_stmt(
            "$.bind_checked",
            [Arg::Ident(el_name), Arg::Expr(getter), Arg::Expr(setter)],
        ),
        "group" => {
            ctx.needs_binding_group = true;
            ctx.b.call_stmt(
                "$.bind_group",
                [
                    Arg::Ident("binding_group"),
                    Arg::Expr(ctx.b.empty_array_expr()),
                    Arg::Ident(el_name),
                    Arg::Expr(getter),
                    Arg::Expr(setter),
                ],
            )
        }
        _ => ctx.b.call_stmt(
            "$.bind_value",
            [Arg::Ident(el_name), Arg::Expr(getter), Arg::Expr(setter)],
        ),
    };

    Some(stmt)
}

/// Events that Svelte delegates to the document root.
fn is_delegatable_event(name: &str) -> bool {
    matches!(
        name,
        "click"
            | "input"
            | "change"
            | "submit"
            | "focus"
            | "blur"
            | "keydown"
            | "keyup"
            | "keypress"
            | "mousedown"
            | "mouseup"
            | "mousemove"
            | "mouseenter"
            | "mouseleave"
            | "mouseover"
            | "mouseout"
            | "touchstart"
            | "touchend"
            | "touchmove"
            | "pointerdown"
            | "pointerup"
            | "pointermove"
            | "focusin"
            | "focusout"
            | "dblclick"
            | "contextmenu"
            | "auxclick"
    )
}

/// Generate `$.set_attributes(el, prevAttrs, { ...allAttrs })` for elements with spread.
pub(crate) fn process_attrs_spread<'a>(
    ctx: &mut Ctx<'a>,
    el: &Element,
    el_name: &str,
    init: &mut Vec<Statement<'a>>,
    after_update: &mut Vec<Statement<'a>>,
) {
    // Build object literal with all attributes
    let mut props: Vec<ObjProp<'a>> = Vec::new();

    for (attr_idx, attr) in el.attributes.iter().enumerate() {
        match attr {
            Attribute::BooleanAttribute(a) => {
                let name_alloc = ctx.b.alloc_str(&a.name);
                props.push(ObjProp::KeyValue(name_alloc, ctx.b.bool_expr(true)));
            }
            Attribute::StringAttribute(a) => {
                let val = ctx.component.source_text(a.value_span).to_string();
                let name_alloc = ctx.b.alloc_str(&a.name);
                props.push(ObjProp::KeyValue(name_alloc, ctx.b.str_expr(&val)));
            }
            Attribute::ExpressionAttribute(a) => {
                let expr_text = ctx.component.source_text(a.expression_span).trim();
                if a.name == expr_text {
                    // Property shorthand: name matches expression
                    let name_alloc = ctx.b.alloc_str(&a.name);
                    props.push(ObjProp::Shorthand(name_alloc));
                } else {
                    let expr = get_attr_expr(ctx, el.id, attr_idx);
                    let name_alloc = ctx.b.alloc_str(&a.name);
                    props.push(ObjProp::KeyValue(name_alloc, expr));
                }
            }
            Attribute::ConcatenationAttribute(a) => {
                let val = build_attr_concat(ctx, el.id, attr_idx, &a.parts);
                let name_alloc = ctx.b.alloc_str(&a.name);
                props.push(ObjProp::KeyValue(name_alloc, val));
            }
            Attribute::ShorthandOrSpread(a) if a.is_spread => {
                let expr = get_attr_expr(ctx, el.id, attr_idx);
                props.push(ObjProp::Spread(expr));
            }
            Attribute::ShorthandOrSpread(a) => {
                let name = ctx.component.source_text(a.expression_span).trim();
                let name_alloc = ctx.b.alloc_str(name);
                props.push(ObjProp::Shorthand(name_alloc));
            }
            Attribute::BindDirective(bind) => {
                if let Some(stmt) = gen_bind_directive(ctx, bind, el_name) {
                    after_update.push(stmt);
                }
                continue;
            }
            Attribute::ClassDirective(_) => continue,
        }
    }

    // $.attribute_effect(el, () => ({...}))
    let obj = ctx.b.object_expr(props);
    let arrow = ctx.b.arrow_expr(ctx.b.no_params(), [ctx.b.expr_stmt(obj)]);
    init.push(ctx.b.call_stmt("$.attribute_effect", [Arg::Ident(el_name), Arg::Expr(arrow)]));
}
