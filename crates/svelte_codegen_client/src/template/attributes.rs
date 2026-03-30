//! Attribute processing (per-attribute and spread).

use oxc_ast::ast::{Expression, Statement};

use svelte_ast::{Attribute, Element, NodeId};

use crate::builder::{Arg, AssignLeft, ObjProp};
use crate::context::Ctx;

use super::bind::{gen_bind_directive, BindPlacement, emit_bind_group_value};
use super::events::{
    build_event_handler_s5, dev_event_handler, gen_use_directive, gen_on_directive_legacy,
    gen_transition_directive, gen_animate_directive, gen_attach_tag,
};
use super::expression::{build_attr_concat, get_attr_expr, MemoAttr};

/// Build an object property for a directive expression.
/// Handles the three-way branch: mutated rune -> `$.get(name)`, same-name -> shorthand, else -> key-value.
fn build_directive_prop<'a>(
    ctx: &mut Ctx<'a>,
    directive_id: NodeId,
    name: &str,
    expr: Expression<'a>,
    same_name: bool,
) -> ObjProp<'a> {
    let name_alloc = ctx.b.alloc_str(name);
    // Pre-computed by analysis — no string-based symbol re-resolution.
    if ctx.is_mutable_rune_target(directive_id) {
        let get_call = ctx.b.call_expr("$.get", [Arg::Ident(name)]);
        ObjProp::KeyValue(name_alloc, get_call)
    } else if same_name && expr.is_identifier_reference() {
        // Only use shorthand when the expression is still a plain identifier.
        // Transform may have wrapped rune references with $.get(), making
        // shorthand incorrect.
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
    el_id: NodeId,
    tag_name: &str,
    is_dyn: bool,
    has_use_directive: bool,
    init: &mut Vec<Statement<'a>>,
    update: &mut Vec<Statement<'a>>,
    directive_init: &mut Vec<Statement<'a>>,
    after_update: &mut Vec<Statement<'a>>,
    memo_attrs: &mut Vec<MemoAttr<'a>>,
) {
    let attr_id = attr.id();
    let target = if is_dyn {
        update as &mut Vec<_>
    } else {
        init as &mut Vec<_>
    };

    match attr {
        Attribute::StringAttribute(a) if a.name == "value" && ctx.has_bind_group(el_id) => {
            // bind:group + static value: emit el.value = el.__value = "val"
            let val_text = ctx.component.source_text(a.value_span);
            let val_expr = ctx.b.str_expr(val_text);
            let __value_assign = ctx.b.assign_expr(
                AssignLeft::StaticMember(ctx.b.static_member(ctx.b.rid_expr(el_name), "__value")),
                val_expr,
            );
            let value_assign = ctx.b.assign_expr(
                AssignLeft::StaticMember(ctx.b.static_member(ctx.b.rid_expr(el_name), "value")),
                __value_assign,
            );
            init.push(ctx.b.expr_stmt(value_assign));
        }
        Attribute::StringAttribute(_) | Attribute::BooleanAttribute(_) => {
            // Static — already in template HTML
        }
        Attribute::ExpressionAttribute(a) if a.name == "class" => {
            // Handled by process_class_attribute_and_directives
            return;
        }
        Attribute::ExpressionAttribute(a) => {
            if let Some(raw_event_name) = a.event_name.as_deref() {
                let mode = ctx.event_handler_mode(attr_id)
                    .unwrap_or_else(|| panic!("missing event_handler_mode for attr {:?}", attr_id));
                let event_name = svelte_analyze::strip_capture_event(raw_event_name)
                    .unwrap_or(raw_event_name).to_string();

                let expr_offset = a.expression_span.start;
                let has_call = ctx.analysis.attr_expression(attr_id).map_or(false, |e| e.has_call);
                let val = get_attr_expr(ctx, attr_id);
                let handler = build_event_handler_s5(ctx, attr_id, val, has_call, init);
                let handler = dev_event_handler(ctx, handler, &event_name, expr_offset);

                match mode {
                    svelte_analyze::EventHandlerMode::Delegated { passive } => {
                        let mut args: Vec<Arg<'a, '_>> = vec![
                            Arg::StrRef(&event_name),
                            Arg::Ident(el_name),
                            Arg::Expr(handler),
                        ];
                        if passive {
                            args.push(Arg::Expr(ctx.b.void_zero_expr()));
                            args.push(Arg::Bool(true));
                        }
                        after_update.push(ctx.b.call_stmt("$.delegated", args));
                        ctx.add_delegated_event(event_name);
                    }
                    svelte_analyze::EventHandlerMode::Direct { capture, passive } => {
                        let mut args: Vec<Arg<'a, '_>> = vec![
                            Arg::Str(event_name),
                            Arg::Ident(el_name),
                            Arg::Expr(handler),
                        ];
                        if capture || passive {
                            args.push(if capture { Arg::Bool(true) } else { Arg::Expr(ctx.b.void_zero_expr()) });
                        }
                        if passive {
                            args.push(Arg::Bool(true));
                        }
                        after_update.push(ctx.b.call_stmt("$.event", args));
                    }
                }
                return;
            }
            let val = get_attr_expr(ctx, attr_id);
            if a.name == "value" && tag_name == "input" && ctx.has_bind_group(el_id) {
                // bind:group + value attr: use __value cache pattern
                emit_bind_group_value(ctx, el_name, attr_id, val, init, update);
                return;
            }
            // Memoize dynamic attrs with has_call/await — extract into dependency array
            let needs_memo = is_dyn
                && ctx.analysis.attr_expression(attr_id).is_some_and(|e| e.has_call || e.has_await);
            if needs_memo {
                let (setter_fn, attr_name) = if a.name == "value" && tag_name == "input" {
                    ("$.set_value", None)
                } else if a.name == "style" {
                    ("$.set_style", None)
                } else {
                    ("$.set_attribute", Some(a.name.clone()))
                };
                memo_attrs.push(MemoAttr {
                    attr_id,
                    setter_fn,
                    el_name: el_name.to_string(),
                    attr_name,
                    expr: val,
                });
            } else if a.name == "value" && tag_name == "input" {
                target.push(ctx.b.call_stmt(
                    "$.set_value",
                    [Arg::Ident(el_name), Arg::Expr(val)],
                ));
            } else if a.name == "style" {
                target.push(ctx.b.call_stmt(
                    "$.set_style",
                    [Arg::Ident(el_name), Arg::Expr(val)],
                ));
            } else {
                target.push(ctx.b.call_stmt(
                    "$.set_attribute",
                    [
                        Arg::Ident(el_name),
                        Arg::StrRef(&a.name),
                        Arg::Expr(val),
                    ],
                ));
            }
        }
        Attribute::ConcatenationAttribute(a) => {
            let val = build_attr_concat(ctx, attr_id, &a.parts);
            if a.name == "style" {
                target.push(ctx.b.call_stmt(
                    "$.set_style",
                    [Arg::Ident(el_name), Arg::Expr(val)],
                ));
            } else {
                target.push(ctx.b.call_stmt(
                    "$.set_attribute",
                    [
                        Arg::Ident(el_name),
                        Arg::StrRef(&a.name),
                        Arg::Expr(val),
                    ],
                ));
            }
        }
        Attribute::Shorthand(a) => {
            let val = get_attr_expr(ctx, attr_id);
            let name = ctx.component.source_text(a.expression_span).to_string();
            target.push(ctx.b.call_stmt(
                "$.set_attribute",
                [Arg::Ident(el_name), Arg::Str(name), Arg::Expr(val)],
            ));
        }
        Attribute::BindDirective(bind) => {
            if let Some(placement) = gen_bind_directive(ctx, bind, el_name, tag_name, has_use_directive) {
                match placement {
                    BindPlacement::AfterUpdate(stmt) => after_update.push(stmt),
                    BindPlacement::Init(stmt) => directive_init.push(stmt),
                }
            }
        }
        Attribute::SpreadAttribute(_) | Attribute::ClassDirective(_) | Attribute::StyleDirective(_) => {
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
/// - class={expr} only -> `$.set_class(el, 1, $.clsx(expr))`
/// - class:name directives only -> `$.set_class(el, 1, "", null, prev, { ... })`
/// - class={expr} + class:name -> `$.set_class(el, 1, $.clsx(expr), null, prev, { ... })`
pub(crate) fn process_class_attribute_and_directives<'a>(
    ctx: &mut Ctx<'a>,
    el_id: NodeId,
    el_name: &str,
    init: &mut Vec<Statement<'a>>,
    update: &mut Vec<Statement<'a>>,
) {
    let has_class_attr = ctx.has_class_attribute(el_id);
    let has_class_dirs = ctx.has_class_directives(el_id);

    if !has_class_attr && !has_class_dirs {
        return;
    }

    // --- Build class value ---
    let class_value = if has_class_attr {
        let class_attr_id = ctx.class_attr_id(el_id)
            .expect("has_class_attribute set but no class attr id");

        let mut expr = get_attr_expr(ctx, class_attr_id);
        if ctx.needs_clsx(class_attr_id) {
            expr = ctx.b.call_expr("$.clsx", [Arg::Expr(expr)]);
        }

        expr
    } else {
        // No class expression attribute — use static class or empty string
        let static_class = ctx.static_class(el_id).unwrap_or("");
        ctx.b.str_expr(static_class)
    };

    // --- Build class directives object ---
    let directives_obj = if has_class_dirs {
        // Snapshot directive info to release the immutable borrow on ctx before the mutable loop.
        let dir_snapshot: Vec<_> = ctx.class_directive_info(el_id)
            .expect("has_class_directives set but no directive info")
            .iter()
            .map(|cd| (cd.id, cd.name.clone(), cd.has_expression))
            .collect();

        let mut props: Vec<ObjProp<'a>> = Vec::new();
        for (id, name, has_expression) in &dir_snapshot {
            let (expr, same_name) = if *has_expression {
                let parsed = get_attr_expr(ctx, *id);
                (parsed, ctx.is_expression_shorthand(*id))
            } else {
                (ctx.b.rid_expr(name), true)
            };
            props.push(build_directive_prop(ctx, *id, name, expr, same_name));
        }
        Some(ctx.b.object_expr(props))
    } else {
        None
    };

    let has_state = ctx.class_needs_state(el_id);

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
                set_class_call,
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
    el_id: NodeId,
    el_name: &str,
    init: &mut Vec<Statement<'a>>,
    update: &mut Vec<Statement<'a>>,
) {
    use svelte_ast::StyleDirectiveValue;

    if !ctx.has_style_directives(el_id) {
        return;
    }

    let style_dirs = ctx.style_directives(el_id).to_vec();

    // Read precomputed static style value
    let static_style = ctx.static_style(el_id).unwrap_or("").to_string();

    let mut normal_props: Vec<ObjProp<'a>> = Vec::new();
    let mut important_props: Vec<ObjProp<'a>> = Vec::new();

    for sd in &style_dirs {
        let name = &sd.name;
        let target = if sd.important { &mut important_props } else { &mut normal_props };

        match &sd.value {
            StyleDirectiveValue::Shorthand => {
                let expr = ctx.b.rid_expr(name);
                target.push(build_directive_prop(ctx, sd.id, name, expr, true));
            }
            StyleDirectiveValue::Expression(_) => {
                let parsed = get_attr_expr(ctx, sd.id);
                let same_name = ctx.is_expression_shorthand(sd.id);
                target.push(build_directive_prop(ctx, sd.id, name, parsed, same_name));
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
        set_style_call,
    );

    init.push(ctx.b.let_stmt(&styles_name));
    update.push(ctx.b.expr_stmt(assign));
}

/// Build a template literal for style directive concatenation values.
/// Transform already rewrites rune references (e.g. `shade` -> `$.get(shade)`),
/// so we consume the pre-transformed expressions directly.
fn build_style_concat<'a>(
    ctx: &mut Ctx<'a>,
    _attr_id: NodeId,
    parts: &[svelte_ast::ConcatPart],
) -> Expression<'a> {
    use crate::builder::TemplatePart;
    use super::expression::get_concat_part_expr;

    let mut tpl_parts: Vec<TemplatePart<'a>> = Vec::new();
    for part in parts {
        match part {
            svelte_ast::ConcatPart::Static(s) => tpl_parts.push(TemplatePart::Str(s.clone())),
            svelte_ast::ConcatPart::Dynamic { span, .. } => {
                let expr = get_concat_part_expr(ctx, span.start);
                tpl_parts.push(TemplatePart::Expr(expr));
            }
        }
    }
    ctx.b.template_parts_expr(tpl_parts)
}

/// Generate `$.set_attributes(el, prevAttrs, { ...allAttrs })` for elements with spread.
pub(crate) fn process_attrs_spread<'a>(
    ctx: &mut Ctx<'a>,
    el_id: NodeId,
    el_tag: &str,
    attrs: &[Attribute],
    el_name: &str,
    init: &mut Vec<Statement<'a>>,
    after_update: &mut Vec<Statement<'a>>,
) {
    // Build object literal with all attributes
    let mut props: Vec<ObjProp<'a>> = Vec::new();

    for attr in attrs {
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
                if ctx.is_expression_shorthand(attr_id) {
                    // Property shorthand: name matches expression
                    let name_alloc = ctx.b.alloc_str(&a.name);
                    props.push(ObjProp::Shorthand(name_alloc));
                } else {
                    let expr = get_attr_expr(ctx, attr_id);
                    // Hoist event handlers (on*) with function values for stable identity
                    let is_event = a.event_name.is_some();
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
            Attribute::SpreadAttribute(_) => {
                let expr = get_attr_expr(ctx, attr_id);
                props.push(ObjProp::Spread(expr));
            }
            Attribute::Shorthand(a) => {
                let name = ctx.component.source_text(a.expression_span).trim();
                let name_alloc = ctx.b.alloc_str(name);
                props.push(ObjProp::Shorthand(name_alloc));
            }
            Attribute::BindDirective(bind) => {
                let has_use = ctx.has_use_directive(el_id);
                if let Some(placement) = gen_bind_directive(ctx, bind, el_name, el_tag, has_use) {
                    match placement {
                        BindPlacement::AfterUpdate(stmt) => after_update.push(stmt),
                        BindPlacement::Init(stmt) => init.push(stmt),
                    }
                }
                continue;
            }
            Attribute::ClassDirective(_) => continue,
        Attribute::StyleDirective(_) => continue,
            Attribute::UseDirective(_) => continue,
            // LEGACY(svelte4): on:directive handled separately
            Attribute::OnDirectiveLegacy(_) => continue,
            Attribute::TransitionDirective(_) => continue,
            Attribute::AnimateDirective(_) => continue,
            Attribute::AttachTag(_) => continue,
        }
    }

    // Collect style directives into [$.STYLE] computed property
    let style_dirs = ctx.style_directives(el_id).to_vec();

    if !style_dirs.is_empty() {
        use svelte_ast::StyleDirectiveValue;

        // Add `style: ""` for the static style base
        let style_key = ctx.b.alloc_str("style");
        props.push(ObjProp::KeyValue(style_key, ctx.b.str_expr("")));

        // Build style directives object
        let mut style_props: Vec<ObjProp<'a>> = Vec::new();
        for sd in &style_dirs {
            let name = &sd.name;
            match &sd.value {
                StyleDirectiveValue::Shorthand => {
                    style_props.push(build_directive_prop(ctx, sd.id, name, ctx.b.rid_expr(name), true));
                }
                StyleDirectiveValue::Expression(_) => {
                    let parsed = get_attr_expr(ctx, sd.id);
                    let same_name = ctx.is_expression_shorthand(sd.id);
                    style_props.push(build_directive_prop(ctx, sd.id, name, parsed, same_name));
                }
                StyleDirectiveValue::String(s) => {
                    let name_alloc = ctx.b.alloc_str(name);
                    style_props.push(ObjProp::KeyValue(name_alloc, ctx.b.str_expr(s)));
                }
                StyleDirectiveValue::Concatenation(parts) => {
                    let name_alloc = ctx.b.alloc_str(name);
                    let expr = build_style_concat(ctx, sd.id, parts);
                    style_props.push(ObjProp::KeyValue(name_alloc, expr));
                }
            }
        }

        let style_obj = ctx.b.object_expr(style_props);
        let style_key_expr = ctx.b.static_member_expr(ctx.b.rid_expr("$"), "STYLE");
        props.push(ObjProp::Computed(style_key_expr, style_obj));
    }

    // $.attribute_effect(el, () => ({...})) — skip if no renderable properties
    if !props.is_empty() {
        let obj = ctx.b.object_expr(props);
        let arrow = ctx.b.arrow_expr(ctx.b.no_params(), [ctx.b.expr_stmt(obj)]);
        init.push(ctx.b.call_stmt("$.attribute_effect", [Arg::Ident(el_name), Arg::Expr(arrow)]));
    }
}

// ---------------------------------------------------------------------------
// svelte:element class/style directive helpers
// ---------------------------------------------------------------------------

/// Generate `$.set_class(el, 0, "", null, {}, { name: expr, ... })` for class directives
/// on `<svelte:element>`. Flag 0 = no static class (element type unknown at compile time).
pub(crate) fn process_svelte_element_class_directives<'a>(
    ctx: &mut Ctx<'a>,
    el: &Element,
    el_name: &str,
    init: &mut Vec<Statement<'a>>,
) {
    let dir_snapshot: Vec<_> = match ctx.class_directive_info(el.id) {
        Some(dirs) => dirs.iter().map(|cd| (cd.id, cd.name.clone(), cd.has_expression)).collect(),
        None => return,
    };

    let mut props: Vec<ObjProp<'a>> = Vec::new();
    for (id, name, has_expression) in &dir_snapshot {
        let (expr, same_name) = if *has_expression {
            let parsed = get_attr_expr(ctx, *id);
            (parsed, ctx.is_expression_shorthand(*id))
        } else {
            (ctx.b.rid_expr(name), true)
        };
        props.push(build_directive_prop(ctx, *id, name, expr, same_name));
    }

    let dir_obj = ctx.b.object_expr(props);
    let set_class_call = ctx.b.call_stmt(
        "$.set_class",
        [
            Arg::Ident(el_name),
            Arg::Num(0.0),
            Arg::StrRef(""),
            Arg::Expr(ctx.b.null_expr()),
            Arg::Expr(ctx.b.object_expr(vec![])),
            Arg::Expr(dir_obj),
        ],
    );
    init.push(set_class_call);
}

/// Style directives on `<svelte:element>` are handled by `process_attrs_spread` via [$.STYLE] computed property.
/// This function is a no-op — style directives are already collected in the spread path.
pub(crate) fn process_svelte_element_style_directives<'a>(
    _ctx: &mut Ctx<'a>,
    _el: &Element,
    _el_name: &str,
    _init: &mut Vec<Statement<'a>>,
) {
    // Style directives are collected directly inside process_attrs_spread
}
