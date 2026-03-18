//! Attribute processing (per-attribute and spread).

use oxc_ast::ast::{Expression, Statement};

use svelte_ast::{Attribute, Element, NodeId};

use crate::builder::{Arg, AssignLeft, AssignRight, ObjProp};
use crate::context::Ctx;

use super::expression::{build_attr_concat, get_attr_expr};

/// Build an object property for a directive expression.
/// Handles the three-way branch: mutated rune → `$.get(name)`, same-name → shorthand, else → key-value.
fn build_directive_prop<'a>(
    ctx: &mut Ctx<'a>,
    name: &str,
    expr: Expression<'a>,
    same_name: bool,
) -> ObjProp<'a> {
    let name_alloc = ctx.b.alloc_str(name);
    if ctx.is_mutable_rune(name) {
        let get_call = ctx.b.call_expr("$.get", [Arg::Ident(name)]);
        ObjProp::KeyValue(name_alloc, get_call)
    } else if same_name {
        ObjProp::Shorthand(name_alloc)
    } else {
        ObjProp::KeyValue(name_alloc, expr)
    }
}

/// Process a single attribute (non-spread path).
///
/// `init` — simple attributes when non-dynamic (before children).
/// `directive_init` — directive statements like $.attach, $.action, $.bind_* (after children).
/// `after_update` — transition, animate, on:event directives (after template_effect).
pub(crate) fn process_attr<'a>(
    ctx: &mut Ctx<'a>,
    attr: &Attribute,
    el_name: &str,
    tag_name: &str,
    is_dyn: bool,
    init: &mut Vec<Statement<'a>>,
    update: &mut Vec<Statement<'a>>,
    directive_init: &mut Vec<Statement<'a>>,
    after_update: &mut Vec<Statement<'a>>,
) {
    let attr_id = attr.id();
    let target = if is_dyn {
        update as &mut Vec<_>
    } else {
        init as &mut Vec<_>
    };

    match attr {
        Attribute::StringAttribute(_) | Attribute::BooleanAttribute(_) => {
            // Static — already in template HTML
        }
        Attribute::ExpressionAttribute(a) if a.name == "class" => {
            // Handled by process_class_attribute_and_directives
            return;
        }
        Attribute::ExpressionAttribute(a) => {
            if let Some(event_name) = a.name.strip_prefix("on") {
                if svelte_js::is_delegatable_event(event_name) {
                    let event_str = event_name.to_string();
                    let val = get_attr_expr(ctx, attr_id);
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
            let val = get_attr_expr(ctx, attr_id);
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
            let val = build_attr_concat(ctx, attr_id, &a.parts);
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
            let val = get_attr_expr(ctx, attr_id);
            let name = ctx.component.source_text(a.expression_span).to_string();
            target.push(ctx.b.call_stmt(
                "$.set_attribute",
                [Arg::Ident(el_name), Arg::Str(name), Arg::Expr(val)],
            ));
        }
        Attribute::BindDirective(bind) => {
            if let Some(placement) = gen_bind_directive(ctx, bind, el_name, tag_name) {
                match placement {
                    BindPlacement::AfterUpdate(stmt) => after_update.push(stmt),
                    BindPlacement::Init(stmt) => directive_init.push(stmt),
                }
            }
        }
        Attribute::ShorthandOrSpread(_) | Attribute::ClassDirective(_) | Attribute::StyleDirective(_) => {
            // Spread handled by process_attrs_spread; class/style directives by dedicated functions
        }
        Attribute::UseDirective(ud) => {
            gen_use_directive(ctx, ud, attr_id, el_name, directive_init);
        }
        // LEGACY(svelte4): on:directive
        Attribute::OnDirectiveLegacy(od) => {
            gen_on_directive_legacy(ctx, od, attr_id, el_name, after_update);
        }
        Attribute::TransitionDirective(td) => {
            gen_transition_directive(ctx, td, attr_id, el_name, after_update);
        }
        Attribute::AnimateDirective(ad) => {
            gen_animate_directive(ctx, ad, attr_id, el_name, after_update);
        }
        Attribute::AttachTag(_) => {
            gen_attach_tag(ctx, attr_id, el_name, directive_init);
        }
    }
}

/// Generate `$.set_class(el, 1, value, null, prev, { ... })` for class expression attributes
/// and/or class:name directives. Handles all combinations:
/// - class={expr} only → `$.set_class(el, 1, $.clsx(expr))`
/// - class:name directives only → `$.set_class(el, 1, "", null, prev, { ... })`
/// - class={expr} + class:name → `$.set_class(el, 1, $.clsx(expr), null, prev, { ... })`
pub(crate) fn process_class_attribute_and_directives<'a>(
    ctx: &mut Ctx<'a>,
    el: &Element,
    el_name: &str,
    init: &mut Vec<Statement<'a>>,
    update: &mut Vec<Statement<'a>>,
) {
    let has_class_attr = ctx.has_class_attribute(el.id);
    let has_class_dirs = ctx.has_class_directives(el.id);

    if !has_class_attr && !has_class_dirs {
        return;
    }

    // --- Build class value ---
    let (class_value, class_attr_is_dynamic) = if has_class_attr {
        // Find the class expression attribute
        let class_attr = el.attributes.iter()
            .find(|a| matches!(a, Attribute::ExpressionAttribute(ea) if ea.name == "class"))
            .expect("has_class_attribute set but no ExpressionAttribute with name=class");
        let class_attr_id = class_attr.id();

        let mut expr = get_attr_expr(ctx, class_attr_id);
        if ctx.needs_clsx(class_attr_id) {
            expr = ctx.b.call_expr("$.clsx", [Arg::Expr(expr)]);
        }

        let is_dynamic = ctx.is_dynamic_attr(class_attr_id);
        (expr, is_dynamic)
    } else {
        // No class expression attribute — use static class or empty string
        let static_class = ctx.static_class(el.id).unwrap_or("");
        (ctx.b.str_expr(static_class), false)
    };

    // --- Build class directives object ---
    let directives_obj = if has_class_dirs {
        let class_dirs: Vec<_> = el.attributes.iter()
            .filter_map(|a| match a {
                Attribute::ClassDirective(cd) => Some(cd),
                _ => None,
            })
            .collect();

        let mut props: Vec<ObjProp<'a>> = Vec::new();
        for cd in &class_dirs {
            let name = &cd.name;
            let (expr, same_name) = if cd.expression_span.is_some() {
                let parsed = get_attr_expr(ctx, cd.id);
                let same = cd.expression_span
                    .map(|span| ctx.component.source_text(span).trim() == name.as_str())
                    .unwrap_or(false);
                (parsed, same)
            } else {
                (ctx.b.rid_expr(name), true)
            };
            props.push(build_directive_prop(ctx, name, expr, same_name));
        }
        Some(ctx.b.object_expr(props))
    } else {
        None
    };

    // --- Determine if any directive is dynamic ---
    let directives_are_dynamic = has_class_dirs && el.attributes.iter().any(|a| {
        matches!(a, Attribute::ClassDirective(_)) && ctx.is_dynamic_attr(a.id())
    });

    let has_state = class_attr_is_dynamic || directives_are_dynamic;

    // --- Generate $.set_class() call ---
    if let Some(dir_obj) = directives_obj {
        // With directives: $.set_class(el, 1, value, null, prev, { ... })
        if has_state {
            let classes_name = ctx.gen_ident("classes");
            let set_class_call = ctx.b.call_expr(
                "$.set_class",
                [
                    Arg::Ident(el_name),
                    Arg::Num(1.0),
                    Arg::Expr(class_value),
                    Arg::Expr(ctx.b.null_expr()),
                    Arg::Ident(&classes_name),
                    Arg::Expr(dir_obj),
                ],
            );
            let assign = ctx.b.assign_expr(
                AssignLeft::Ident(classes_name.clone()),
                AssignRight::Expr(set_class_call),
            );
            init.push(ctx.b.let_stmt(&classes_name));
            update.push(ctx.b.expr_stmt(assign));
        } else {
            // No state: use empty object {} as prev
            let set_class_call = ctx.b.call_expr(
                "$.set_class",
                [
                    Arg::Ident(el_name),
                    Arg::Num(1.0),
                    Arg::Expr(class_value),
                    Arg::Expr(ctx.b.null_expr()),
                    Arg::Expr(ctx.b.object_expr(vec![])),
                    Arg::Expr(dir_obj),
                ],
            );
            init.push(ctx.b.expr_stmt(set_class_call));
        }
    } else {
        // Without directives: $.set_class(el, 1, value)
        let set_class_call = ctx.b.call_expr(
            "$.set_class",
            [Arg::Ident(el_name), Arg::Num(1.0), Arg::Expr(class_value)],
        );
        let target = if has_state { &mut *update } else { &mut *init };
        target.push(ctx.b.expr_stmt(set_class_call));
    }
}

/// Generate `$.set_style(el, "static-style", styles_N, { ... })` for style:name directives.
/// When `|important` is used, generates `[{ normal }, { important }]` array format.
/// Pushes `let styles_N;` to `init` and the assignment to `update` for combined template_effect.
pub(crate) fn process_style_directives<'a>(
    ctx: &mut Ctx<'a>,
    el: &Element,
    el_name: &str,
    init: &mut Vec<Statement<'a>>,
    update: &mut Vec<Statement<'a>>,
) {
    use svelte_ast::StyleDirectiveValue;

    if !ctx.has_style_directives(el.id) {
        return;
    }

    let style_dirs: Vec<_> = el
        .attributes
        .iter()
        .filter_map(|a| match a {
            Attribute::StyleDirective(sd) => Some(sd),
            _ => None,
        })
        .collect();

    // Read precomputed static style value
    let static_style = ctx.static_style(el.id).unwrap_or("").to_string();

    let mut normal_props: Vec<ObjProp<'a>> = Vec::new();
    let mut important_props: Vec<ObjProp<'a>> = Vec::new();

    for sd in &style_dirs {
        let name = &sd.name;
        let target = if sd.important { &mut important_props } else { &mut normal_props };

        match &sd.value {
            StyleDirectiveValue::Shorthand => {
                let expr = ctx.b.rid_expr(name);
                target.push(build_directive_prop(ctx, name, expr, true));
            }
            StyleDirectiveValue::Expression(span) => {
                let parsed = get_attr_expr(ctx, sd.id);
                let expr_text = ctx.component.source_text(*span).trim();
                let same_name = expr_text == name.as_str();
                target.push(build_directive_prop(ctx, name, parsed, same_name));
            }
            StyleDirectiveValue::String(s) => {
                let name_alloc = ctx.b.alloc_str(name);
                target.push(ObjProp::KeyValue(name_alloc, ctx.b.str_expr(s)));
            }
            StyleDirectiveValue::Concatenation(parts) => {
                let name_alloc = ctx.b.alloc_str(name);
                let expr = build_style_concat(ctx, sd.id, parts);
                target.push(ObjProp::KeyValue(name_alloc, expr));
            }
        }
    }

    // Build the directives argument: single object or [normal, important] array
    let directives_expr = if important_props.is_empty() {
        ctx.b.object_expr(normal_props)
    } else {
        let normal_obj = ctx.b.object_expr(normal_props);
        let important_obj = ctx.b.object_expr(important_props);
        ctx.b.array_from_args([Arg::Expr(normal_obj), Arg::Expr(important_obj)])
    };

    let styles_name = ctx.gen_ident("styles");

    let set_style_call = ctx.b.call_expr(
        "$.set_style",
        [
            Arg::Ident(el_name),
            Arg::Str(static_style),
            Arg::Ident(&styles_name),
            Arg::Expr(directives_expr),
        ],
    );

    let assign = ctx.b.assign_expr(
        AssignLeft::Ident(styles_name.clone()),
        AssignRight::Expr(set_style_call),
    );

    init.push(ctx.b.let_stmt(&styles_name));
    update.push(ctx.b.expr_stmt(assign));
}

/// Build a template literal for style directive concatenation values.
/// Applies `$.get()` wrapping for mutated rune references.
fn build_style_concat<'a>(
    ctx: &mut Ctx<'a>,
    attr_id: NodeId,
    parts: &[svelte_ast::ConcatPart],
) -> Expression<'a> {
    use crate::builder::TemplatePart;
    use super::expression::get_concat_part_expr;

    let mut tpl_parts: Vec<TemplatePart<'a>> = Vec::new();
    let mut dyn_idx = 0usize;
    for part in parts {
        match part {
            svelte_ast::ConcatPart::Static(s) => tpl_parts.push(TemplatePart::Str(s.clone())),
            svelte_ast::ConcatPart::Dynamic(span) => {
                // Check if the dynamic expression is a simple mutated rune reference
                let expr_text = ctx.component.source_text(*span).trim();
                let is_mutated_rune = ctx.is_mutable_rune(expr_text);

                if is_mutated_rune {
                    // Drain the pre-parsed expression (it's just `shade` as an identifier)
                    // and replace with `$.get(shade)` — the rune getter call.
                    let _ = get_concat_part_expr(ctx, attr_id, dyn_idx);
                    let get_call = ctx.b.call_expr("$.get", [Arg::Ident(expr_text)]);
                    tpl_parts.push(TemplatePart::Expr(get_call));
                } else {
                    let expr = get_concat_part_expr(ctx, attr_id, dyn_idx);
                    tpl_parts.push(TemplatePart::Expr(expr));
                }
                dyn_idx += 1;
            }
        }
    }
    ctx.b.template_parts_expr(tpl_parts)
}

/// Where to place a bind directive statement.
enum BindPlacement<'a> {
    /// Most bindings: placed after attribute updates.
    AfterUpdate(Statement<'a>),
    /// `bind:this`: placed in init (before render effect).
    Init(Statement<'a>),
}

/// Generate a bind directive statement (getter/setter + runtime call).
fn gen_bind_directive<'a>(
    ctx: &mut Ctx<'a>,
    bind: &svelte_ast::BindDirective,
    el_name: &str,
    tag_name: &str,
) -> Option<BindPlacement<'a>> {
    let var_name = if bind.shorthand {
        bind.name.clone()
    } else if let Some(span) = bind.expression_span {
        ctx.component.source_text(span).trim().to_string()
    } else {
        return None;
    };

    // Build getter: () => $.get(x) for runes, () => x for plain vars
    let build_getter = |ctx: &mut Ctx<'a>, var: &str| -> Expression<'a> {
        let body = if ctx.is_mutable_rune(var) {
            ctx.b.call_expr("$.get", [Arg::Ident(var)])
        } else {
            ctx.b.rid_expr(var)
        };
        ctx.b.arrow_expr(ctx.b.no_params(), [ctx.b.expr_stmt(body)])
    };

    // Build setter: ($$value) => $.set(x, $$value) for runes, ($$value) => x = $$value for plain
    let build_setter = |ctx: &mut Ctx<'a>, var: String| -> Expression<'a> {
        let body = if ctx.is_mutable_rune(&var) {
            ctx.b.call_expr("$.set", [Arg::Ident(&var), Arg::Ident("$$value")])
        } else {
            ctx.b.assign_expr(
                AssignLeft::Ident(var),
                AssignRight::Expr(ctx.b.rid_expr("$$value")),
            )
        };
        ctx.b.arrow_expr(ctx.b.params(["$$value"]), [ctx.b.expr_stmt(body)])
    };

    let stmt = match bind.name.as_str() {
        // --- Input/Form ---
        "value" if tag_name == "select" => {
            let getter = build_getter(ctx, &var_name);
            let setter = build_setter(ctx, var_name);
            ctx.b.call_stmt("$.bind_select_value", [
                Arg::Ident(el_name), Arg::Expr(getter), Arg::Expr(setter),
            ])
        }
        "value" => {
            let getter = build_getter(ctx, &var_name);
            let setter = build_setter(ctx, var_name);
            ctx.b.call_stmt("$.bind_value", [
                Arg::Ident(el_name), Arg::Expr(getter), Arg::Expr(setter),
            ])
        }
        "checked" => {
            let getter = build_getter(ctx, &var_name);
            let setter = build_setter(ctx, var_name);
            ctx.b.call_stmt("$.bind_checked", [
                Arg::Ident(el_name), Arg::Expr(getter), Arg::Expr(setter),
            ])
        }
        "group" => {
            ctx.needs_binding_group = true;
            let getter = build_getter(ctx, &var_name);
            let setter = build_setter(ctx, var_name);
            ctx.b.call_stmt("$.bind_group", [
                Arg::Ident("binding_group"),
                Arg::Expr(ctx.b.empty_array_expr()),
                Arg::Ident(el_name),
                Arg::Expr(getter),
                Arg::Expr(setter),
            ])
        }
        "files" => {
            let getter = build_getter(ctx, &var_name);
            let setter = build_setter(ctx, var_name);
            ctx.b.call_stmt("$.bind_files", [
                Arg::Ident(el_name), Arg::Expr(getter), Arg::Expr(setter),
            ])
        }

        // --- Generic event-based bindings (bidirectional) ---
        "indeterminate" => {
            let setter = build_setter(ctx, var_name.clone());
            let getter = build_getter(ctx, &var_name);
            ctx.b.call_stmt("$.bind_property", [
                Arg::Str("indeterminate".into()), Arg::Str("change".into()),
                Arg::Ident(el_name), Arg::Expr(setter), Arg::Expr(getter),
            ])
        }
        "open" => {
            let setter = build_setter(ctx, var_name.clone());
            let getter = build_getter(ctx, &var_name);
            ctx.b.call_stmt("$.bind_property", [
                Arg::Str("open".into()), Arg::Str("toggle".into()),
                Arg::Ident(el_name), Arg::Expr(setter), Arg::Expr(getter),
            ])
        }

        // --- Contenteditable ---
        "innerHTML" | "innerText" | "textContent" => {
            let getter = build_getter(ctx, &var_name);
            let setter = build_setter(ctx, var_name);
            ctx.b.call_stmt("$.bind_content_editable", [
                Arg::Str(bind.name.clone()), Arg::Ident(el_name),
                Arg::Expr(getter), Arg::Expr(setter),
            ])
        }

        // --- Dimension bindings (element size, readonly) ---
        "clientWidth" | "clientHeight" | "offsetWidth" | "offsetHeight" => {
            let setter = build_setter(ctx, var_name);
            ctx.b.call_stmt("$.bind_element_size", [
                Arg::Ident(el_name), Arg::Str(bind.name.clone()), Arg::Expr(setter),
            ])
        }

        // --- Dimension bindings (resize observer, readonly) ---
        "contentRect" | "contentBoxSize" | "borderBoxSize" | "devicePixelContentBoxSize" => {
            let setter = build_setter(ctx, var_name);
            ctx.b.call_stmt("$.bind_resize_observer", [
                Arg::Ident(el_name), Arg::Str(bind.name.clone()), Arg::Expr(setter),
            ])
        }

        // --- Media R/W bindings ---
        "currentTime" => {
            let getter = build_getter(ctx, &var_name);
            let setter = build_setter(ctx, var_name);
            ctx.b.call_stmt("$.bind_current_time", [
                Arg::Ident(el_name), Arg::Expr(getter), Arg::Expr(setter),
            ])
        }
        "playbackRate" => {
            let getter = build_getter(ctx, &var_name);
            let setter = build_setter(ctx, var_name);
            ctx.b.call_stmt("$.bind_playback_rate", [
                Arg::Ident(el_name), Arg::Expr(getter), Arg::Expr(setter),
            ])
        }
        "paused" => {
            let getter = build_getter(ctx, &var_name);
            let setter = build_setter(ctx, var_name);
            ctx.b.call_stmt("$.bind_paused", [
                Arg::Ident(el_name), Arg::Expr(getter), Arg::Expr(setter),
            ])
        }
        "volume" => {
            let getter = build_getter(ctx, &var_name);
            let setter = build_setter(ctx, var_name);
            ctx.b.call_stmt("$.bind_volume", [
                Arg::Ident(el_name), Arg::Expr(getter), Arg::Expr(setter),
            ])
        }
        "muted" => {
            let getter = build_getter(ctx, &var_name);
            let setter = build_setter(ctx, var_name);
            ctx.b.call_stmt("$.bind_muted", [
                Arg::Ident(el_name), Arg::Expr(getter), Arg::Expr(setter),
            ])
        }

        // --- Media RO bindings (setter only) ---
        "buffered" => {
            let setter = build_setter(ctx, var_name);
            ctx.b.call_stmt("$.bind_buffered", [Arg::Ident(el_name), Arg::Expr(setter)])
        }
        "seekable" => {
            let setter = build_setter(ctx, var_name);
            ctx.b.call_stmt("$.bind_seekable", [Arg::Ident(el_name), Arg::Expr(setter)])
        }
        "seeking" => {
            let setter = build_setter(ctx, var_name);
            ctx.b.call_stmt("$.bind_seeking", [Arg::Ident(el_name), Arg::Expr(setter)])
        }
        "ended" => {
            let setter = build_setter(ctx, var_name);
            ctx.b.call_stmt("$.bind_ended", [Arg::Ident(el_name), Arg::Expr(setter)])
        }
        "readyState" => {
            let setter = build_setter(ctx, var_name);
            ctx.b.call_stmt("$.bind_ready_state", [Arg::Ident(el_name), Arg::Expr(setter)])
        }
        "played" => {
            let setter = build_setter(ctx, var_name);
            ctx.b.call_stmt("$.bind_played", [Arg::Ident(el_name), Arg::Expr(setter)])
        }

        // --- Media/Image RO event-based bindings (bind_property, no bidirectional) ---
        "duration" => {
            let setter = build_setter(ctx, var_name);
            ctx.b.call_stmt("$.bind_property", [
                Arg::Str("duration".into()), Arg::Str("durationchange".into()),
                Arg::Ident(el_name), Arg::Expr(setter),
            ])
        }
        "videoWidth" => {
            let setter = build_setter(ctx, var_name);
            ctx.b.call_stmt("$.bind_property", [
                Arg::Str("videoWidth".into()), Arg::Str("resize".into()),
                Arg::Ident(el_name), Arg::Expr(setter),
            ])
        }
        "videoHeight" => {
            let setter = build_setter(ctx, var_name);
            ctx.b.call_stmt("$.bind_property", [
                Arg::Str("videoHeight".into()), Arg::Str("resize".into()),
                Arg::Ident(el_name), Arg::Expr(setter),
            ])
        }
        "naturalWidth" => {
            let setter = build_setter(ctx, var_name);
            ctx.b.call_stmt("$.bind_property", [
                Arg::Str("naturalWidth".into()), Arg::Str("load".into()),
                Arg::Ident(el_name), Arg::Expr(setter),
            ])
        }
        "naturalHeight" => {
            let setter = build_setter(ctx, var_name);
            ctx.b.call_stmt("$.bind_property", [
                Arg::Str("naturalHeight".into()), Arg::Str("load".into()),
                Arg::Ident(el_name), Arg::Expr(setter),
            ])
        }

        // --- Misc ---
        "focused" => {
            let setter = build_setter(ctx, var_name);
            ctx.b.call_stmt("$.bind_focused", [Arg::Ident(el_name), Arg::Expr(setter)])
        }

        // --- bind:this ---
        "this" => {
            // svelte:element uses proxy flag because the element type is unknown at compile time
            let setter = if tag_name.is_empty() && ctx.is_mutable_rune(&var_name) {
                let body = ctx.b.call_expr("$.set", [
                    Arg::Ident(&var_name), Arg::Ident("$$value"), Arg::Expr(ctx.b.bool_expr(true)),
                ]);
                ctx.b.arrow_expr(ctx.b.params(["$$value"]), [ctx.b.expr_stmt(body)])
            } else {
                build_setter(ctx, var_name.clone())
            };
            let getter = build_getter(ctx, &var_name);
            let stmt = ctx.b.call_stmt("$.bind_this", [
                Arg::Ident(el_name), Arg::Expr(setter), Arg::Expr(getter),
            ]);
            return Some(BindPlacement::Init(stmt));
        }

        // Fallback for unknown bindings
        _ => {
            let getter = build_getter(ctx, &var_name);
            let setter = build_setter(ctx, var_name);
            ctx.b.call_stmt("$.bind_value", [
                Arg::Ident(el_name), Arg::Expr(getter), Arg::Expr(setter),
            ])
        }
    };

    Some(BindPlacement::AfterUpdate(stmt))
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

    for attr in &el.attributes {
        let attr_id = attr.id();
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
                    let expr = get_attr_expr(ctx, attr_id);
                    // Hoist event handlers (on*) with function values for stable identity
                    let is_event = a.name.starts_with("on");
                    let is_fn = matches!(
                        &expr,
                        Expression::ArrowFunctionExpression(_) | Expression::FunctionExpression(_)
                    );
                    if is_event && is_fn {
                        let handler_name = ctx.gen_ident("event_handler");
                        init.push(ctx.b.var_stmt(&handler_name, expr));
                        let name_alloc = ctx.b.alloc_str(&a.name);
                        props.push(ObjProp::KeyValue(name_alloc, ctx.b.rid_expr(&handler_name)));
                    } else {
                        let name_alloc = ctx.b.alloc_str(&a.name);
                        props.push(ObjProp::KeyValue(name_alloc, expr));
                    }
                }
            }
            Attribute::ConcatenationAttribute(a) => {
                let val = build_attr_concat(ctx, attr_id, &a.parts);
                let name_alloc = ctx.b.alloc_str(&a.name);
                props.push(ObjProp::KeyValue(name_alloc, val));
            }
            Attribute::ShorthandOrSpread(a) if a.is_spread => {
                let expr = get_attr_expr(ctx, attr_id);
                props.push(ObjProp::Spread(expr));
            }
            Attribute::ShorthandOrSpread(a) => {
                let name = ctx.component.source_text(a.expression_span).trim();
                let name_alloc = ctx.b.alloc_str(name);
                props.push(ObjProp::Shorthand(name_alloc));
            }
            Attribute::BindDirective(bind) => {
                if let Some(placement) = gen_bind_directive(ctx, bind, el_name, &el.name) {
                    match placement {
                        BindPlacement::AfterUpdate(stmt) => after_update.push(stmt),
                        BindPlacement::Init(stmt) => init.push(stmt),
                    }
                }
                continue;
            }
            Attribute::ClassDirective(_) | Attribute::StyleDirective(_) => continue,
            Attribute::UseDirective(_) => continue,
            // LEGACY(svelte4): on:directive handled separately
            Attribute::OnDirectiveLegacy(_) => continue,
            Attribute::TransitionDirective(_) => continue,
            Attribute::AnimateDirective(_) => continue,
            Attribute::AttachTag(_) => continue,
        }
    }

    // $.attribute_effect(el, () => ({...})) — skip if no renderable properties
    if !props.is_empty() {
        let obj = ctx.b.object_expr(props);
        let arrow = ctx.b.arrow_expr(ctx.b.no_params(), [ctx.b.expr_stmt(obj)]);
        init.push(ctx.b.call_stmt("$.attribute_effect", [Arg::Ident(el_name), Arg::Expr(arrow)]));
    }
}

// ---------------------------------------------------------------------------
// ---------------------------------------------------------------------------
// use:action directive codegen
// ---------------------------------------------------------------------------

/// Generate `$.action(target, ...)` for `use:action` on special elements (svelte:body, etc.).
pub(crate) fn gen_use_directive_on<'a>(
    ctx: &mut Ctx<'a>,
    ud: &svelte_ast::UseDirective,
    attr_id: NodeId,
    el_name: &str,
    init: &mut Vec<Statement<'a>>,
) {
    gen_use_directive(ctx, ud, attr_id, el_name, init);
}

/// Generate `$.action(el, ($$node) => name?.($$node), () => expr)`.
/// Reference: `UseDirective.js`.
fn gen_use_directive<'a>(
    ctx: &mut Ctx<'a>,
    ud: &svelte_ast::UseDirective,
    attr_id: NodeId,
    el_name: &str,
    init: &mut Vec<Statement<'a>>,
) {
    // Build handler params: ($$node) or ($$node, $$action_arg)
    let has_expr = ud.expression_span.is_some();
    let params = if has_expr {
        ctx.b.params(["$$node", "$$action_arg"])
    } else {
        ctx.b.params(["$$node"])
    };

    // Build directive name expression (handles dotted names like "a.b")
    let name_expr = build_directive_name_expr(ctx, &ud.name);

    // Build optional call: name?.($$node) or name?.($$node, $$action_arg)
    let mut call_args: Vec<Arg<'a, '_>> = vec![Arg::Ident("$$node")];
    if has_expr {
        call_args.push(Arg::Ident("$$action_arg"));
    }
    let action_call = ctx.b.maybe_call_expr(name_expr, call_args);

    // Wrap in arrow: ($$node) => name?.($$node)
    let handler = ctx.b.arrow_expr(params, [ctx.b.expr_stmt(action_call)]);

    // Build $.action() args
    let mut args: Vec<Arg<'a, '_>> = vec![
        Arg::Ident(el_name),
        Arg::Expr(handler),
    ];

    // Optional third arg: () => expression
    if has_expr {
        let expr = get_attr_expr(ctx, attr_id);
        let thunk = ctx.b.thunk(expr);
        args.push(Arg::Expr(thunk));
    }

    init.push(ctx.b.call_stmt("$.action", args));
}

// ---------------------------------------------------------------------------
// {@attach expr} codegen
// ---------------------------------------------------------------------------

/// Generate `$.attach(el, () => expr)`.
/// Reference: `AttachTag.js`.
fn gen_attach_tag<'a>(
    ctx: &mut Ctx<'a>,
    attr_id: NodeId,
    el_name: &str,
    init: &mut Vec<Statement<'a>>,
) {
    let expr = get_attr_expr(ctx, attr_id);
    let thunk = ctx.b.thunk(expr);
    init.push(ctx.b.call_stmt("$.attach", [Arg::Ident(el_name), Arg::Expr(thunk)]));
}

// ---------------------------------------------------------------------------
// transition:/in:/out: directive codegen
// ---------------------------------------------------------------------------

/// Generate `$.transition(flags, el, () => transitionFn, () => params)`.
/// Reference: `TransitionDirective.js`.
fn gen_transition_directive<'a>(
    ctx: &mut Ctx<'a>,
    td: &svelte_ast::TransitionDirective,
    attr_id: NodeId,
    el_name: &str,
    after_update: &mut Vec<Statement<'a>>,
) {
    // TRANSITION_IN = 1, TRANSITION_OUT = 2, TRANSITION_GLOBAL = 4
    let mut flags: u32 = if td.modifiers.iter().any(|m| m == "global") { 4 } else { 0 };
    match td.direction {
        svelte_ast::TransitionDirection::Both => flags |= 1 | 2,
        svelte_ast::TransitionDirection::In => flags |= 1,
        svelte_ast::TransitionDirection::Out => flags |= 2,
    }

    let name_expr = build_directive_name_expr(ctx, &td.name);
    let name_thunk = ctx.b.thunk(name_expr);

    let mut args: Vec<Arg<'a, '_>> = vec![
        Arg::Num(flags as f64),
        Arg::Ident(el_name),
        Arg::Expr(name_thunk),
    ];

    if td.expression_span.is_some() {
        let expr = get_attr_expr(ctx, attr_id);
        let thunk = ctx.b.thunk(expr);
        args.push(Arg::Expr(thunk));
    }

    after_update.push(ctx.b.call_stmt("$.transition", args));
}

/// Generate `$.animation(el, () => animateFn, () => params)`.
/// Reference: `AnimateDirective.js`.
fn gen_animate_directive<'a>(
    ctx: &mut Ctx<'a>,
    ad: &svelte_ast::AnimateDirective,
    attr_id: NodeId,
    el_name: &str,
    after_update: &mut Vec<Statement<'a>>,
) {
    let name_expr = build_directive_name_expr(ctx, &ad.name);
    let name_thunk = ctx.b.thunk(name_expr);

    let mut args: Vec<Arg<'a, '_>> = vec![
        Arg::Ident(el_name),
        Arg::Expr(name_thunk),
    ];

    if ad.expression_span.is_some() {
        let expr = get_attr_expr(ctx, attr_id);
        let thunk = ctx.b.thunk(expr);
        args.push(Arg::Expr(thunk));
    } else {
        args.push(Arg::Expr(ctx.b.null_expr()));
    }

    after_update.push(ctx.b.call_stmt("$.animation", args));
}

/// Parse a directive name like "a.b.c" into a member expression.
fn build_directive_name_expr<'a>(ctx: &Ctx<'a>, name: &str) -> Expression<'a> {
    let parts: Vec<&str> = name.split('.').collect();
    let mut expr = ctx.b.rid_expr(parts[0]);
    for part in &parts[1..] {
        expr = ctx.b.static_member_expr(expr, part);
    }
    expr
}

// LEGACY(svelte4): on:directive codegen
// ---------------------------------------------------------------------------

/// Generate `$.event()` calls for legacy `on:directive` syntax.
/// Reference: `OnDirective.js` + `shared/events.js` (`build_event`, `build_event_handler`).
fn gen_on_directive_legacy<'a>(
    ctx: &mut Ctx<'a>,
    od: &svelte_ast::OnDirectiveLegacy,
    attr_id: NodeId,
    el_name: &str,
    after_update: &mut Vec<Statement<'a>>,
) {
    // --- Build event handler ---
    let handler = if od.expression_span.is_none() {
        // Bubble event: function($$arg) { $.bubble_event.call(this, $$props, $$arg) }
        let bubble_call = ctx.b.static_member_expr(
            ctx.b.rid_expr("$.bubble_event"),
            "call",
        );
        let call = ctx.b.call_expr_callee(bubble_call, [
            Arg::Expr(ctx.b.this_expr()),
            Arg::Ident("$$props"),
            Arg::Ident("$$arg"),
        ]);
        ctx.b.function_expr(ctx.b.params(["$$arg"]), vec![ctx.b.expr_stmt(call)])
    } else {
        let expr = get_attr_expr(ctx, attr_id);
        build_legacy_event_handler(ctx, expr)
    };

    // --- Apply modifier wrappers (order matches Svelte reference) ---
    let mut wrapped = handler;
    for modifier in &[
        "stopPropagation",
        "stopImmediatePropagation",
        "preventDefault",
        "self",
        "trusted",
        "once",
    ] {
        if od.modifiers.iter().any(|m| m == modifier) {
            let fn_name = format!("$.{}", modifier);
            wrapped = ctx.b.call_expr(&fn_name, [Arg::Expr(wrapped)]);
        }
    }

    // --- Build $.event() call ---
    let capture = od.modifiers.iter().any(|m| m == "capture");
    let passive = od.modifiers.iter().find_map(|m| match m.as_str() {
        "passive" => Some(true),
        "nonpassive" => Some(false),
        _ => None,
    });

    let mut args: Vec<Arg<'a, '_>> = vec![
        Arg::Str(od.name.clone()),
        Arg::Ident(el_name),
        Arg::Expr(wrapped),
    ];
    if capture || passive.is_some() {
        args.push(Arg::Bool(capture));
    }
    if let Some(p) = passive {
        args.push(Arg::Bool(p));
    }

    after_update.push(ctx.b.call_stmt("$.event", args));
}

/// Build event handler for legacy on:directive (non-dev mode).
/// Reference: `events.js` `build_event_handler()`.
fn build_legacy_event_handler<'a>(ctx: &mut Ctx<'a>, handler: Expression<'a>) -> Expression<'a> {
    match &handler {
        // Inline arrow/function — pass through
        Expression::ArrowFunctionExpression(_) | Expression::FunctionExpression(_) => handler,
        // Identifier — pass through in non-dev mode
        Expression::Identifier(_) => handler,
        // Other expressions (member, conditional, etc.) — wrap in function(...$$args) { handler.apply(this, $$args) }
        _ => {
            let apply = ctx.b.static_member_expr(handler, "apply");
            let call = ctx.b.call_expr_callee(apply, [
                Arg::Expr(ctx.b.this_expr()),
                Arg::Ident("$$args"),
            ]);
            ctx.b.function_expr(ctx.b.rest_params("$$args"), vec![ctx.b.expr_stmt(call)])
        }
    }
}
