//! Attribute processing (per-attribute and spread).

use oxc_ast::ast::{Expression, Statement};

use svelte_ast::{Attribute, Element, NodeId};

use crate::builder::{Arg, AssignLeft, ObjProp};
use crate::context::Ctx;

use super::expression::{build_attr_concat, get_attr_expr};

/// Build an object property for a directive expression.
/// Handles the three-way branch: mutated rune → `$.get(name)`, same-name → shorthand, else → key-value.
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
    el_id: NodeId,
    tag_name: &str,
    is_dyn: bool,
    has_use_directive: bool,
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
                let (event_name, capture) = if let Some(base) = svelte_js::strip_capture_event(raw_event_name) {
                    (base.to_string(), true)
                } else {
                    (raw_event_name.to_string(), false)
                };

                let expr_offset = a.expression_span.start;
                let has_call = ctx.analysis.attr_expression(attr_id).map_or(false, |e| e.has_call);
                let val = get_attr_expr(ctx, attr_id);
                let handler = build_event_handler_s5(ctx, attr_id, val, has_call, init);
                let handler = dev_event_handler(ctx, handler, &event_name, expr_offset);

                let passive = svelte_js::is_passive_event(&event_name);
                let delegated = !capture && svelte_js::is_delegatable_event(&event_name);

                if delegated {
                    let mut args: Vec<Arg<'a, '_>> = vec![
                        Arg::Str(event_name.clone()),
                        Arg::Ident(el_name),
                        Arg::Expr(handler),
                    ];
                    if passive {
                        args.push(Arg::Expr(ctx.b.void_zero_expr()));
                        args.push(Arg::Bool(true));
                    }
                    after_update.push(ctx.b.call_stmt("$.delegated", args));
                    if !ctx.delegated_events.contains(&event_name) {
                        ctx.delegated_events.push(event_name);
                    }
                } else {
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
                return;
            }
            let val = get_attr_expr(ctx, attr_id);
            if a.name == "value" && tag_name == "input" && ctx.has_bind_group(el_id) {
                // bind:group + value attr: use __value cache pattern
                emit_bind_group_value(ctx, el_name, attr_id, val, init, update);
                return;
            } else if a.name == "value" && tag_name == "input" {
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
        let class_attr_id = ctx.class_attr_id(el.id)
            .expect("has_class_attribute set but no class attr id");

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
        // Snapshot directive info to release the immutable borrow on ctx before the mutable loop.
        let dir_snapshot: Vec<_> = ctx.class_directive_info(el.id)
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

    let directives_are_dynamic = ctx.has_dynamic_class_directives(el.id);

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
/// Transform already rewrites rune references (e.g. `shade` → `$.get(shade)`),
/// so we consume the pre-transformed expressions directly.
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
            svelte_ast::ConcatPart::Dynamic(_) => {
                let expr = get_concat_part_expr(ctx, attr_id, dyn_idx);
                tpl_parts.push(TemplatePart::Expr(expr));
                dyn_idx += 1;
            }
        }
    }
    ctx.b.template_parts_expr(tpl_parts)
}

/// Build binding getter: `() => $.get(x)` for runes, `() => x` for plain vars.
pub(crate) fn build_binding_getter<'a>(ctx: &mut Ctx<'a>, var: &str, is_rune: bool) -> Expression<'a> {
    let body = if is_rune {
        ctx.b.call_expr("$.get", [Arg::Ident(var)])
    } else {
        ctx.b.rid_expr(var)
    };
    ctx.b.arrow_expr(ctx.b.no_params(), [ctx.b.expr_stmt(body)])
}

/// Build binding setter: `($$value) => $.set(x, $$value)` for runes, `($$value) => x = $$value` for plain.
pub(crate) fn build_binding_setter<'a>(ctx: &mut Ctx<'a>, var: String, is_rune: bool) -> Expression<'a> {
    let body = if is_rune {
        ctx.b.call_expr("$.set", [Arg::Ident(&var), Arg::Ident("$$value")])
    } else {
        ctx.b.assign_expr(
            AssignLeft::Ident(var),
            ctx.b.rid_expr("$$value"),
        )
    };
    ctx.b.arrow_expr(ctx.b.params(["$$value"]), [ctx.b.expr_stmt(body)])
}

/// Build binding setter with `true` flag: `($$value) => $.set(x, $$value, true)`.
/// Used by window/document bindings to prevent re-triggering reactivity.
pub(crate) fn build_binding_setter_silent<'a>(ctx: &mut Ctx<'a>, var: String, is_rune: bool) -> Expression<'a> {
    let body = if is_rune {
        ctx.b.call_expr("$.set", [
            Arg::Ident(&var),
            Arg::Ident("$$value"),
            Arg::Bool(true),
        ])
    } else {
        ctx.b.assign_expr(
            AssignLeft::Ident(var),
            ctx.b.rid_expr("$$value"),
        )
    };
    ctx.b.arrow_expr(ctx.b.params(["$$value"]), [ctx.b.expr_stmt(body)])
}

/// Build binding setter from a pre-transformed expression:
/// `($$value) => transformed_expr = $$value`.
/// Used for bind:group when the expression references each-block vars
/// (the expression already has $.get() applied by the transform phase).
fn build_binding_setter_from_expr<'a>(ctx: &mut Ctx<'a>, expr: Expression<'a>) -> Expression<'a> {
    let target = ctx.b.expr_to_assignment_target(expr);
    let assign = ctx.b.assign_expr_raw(target, ctx.b.rid_expr("$$value"));
    ctx.b.arrow_expr(ctx.b.params(["$$value"]), [ctx.b.expr_stmt(assign)])
}

/// Emit the `__value` cache pattern for `value` attributes on elements with `bind:group`.
///
/// Produces:
/// ```js
/// var input_value;
/// $.template_effect(() => {
///     if (input_value !== (input_value = expr)) {
///         input.value = (input.__value = expr) ?? "";
///     }
/// });
/// ```
fn emit_bind_group_value<'a>(
    ctx: &mut Ctx<'a>,
    el_name: &str,
    _attr_id: NodeId,
    val_expr: Expression<'a>,
    init: &mut Vec<Statement<'a>>,
    _update: &mut Vec<Statement<'a>>,
) {
    let cache_name = ctx.gen_ident(&format!("{}_value", el_name));

    // var input_value;
    init.push(ctx.b.var_uninit_stmt(&cache_name));

    // Clone expression for reuse in __value body
    let val_expr2 = ctx.b.clone_expr(&val_expr);

    // cache = expr → assignment in the condition test
    let cache_assign = ctx.b.assign_expr(
        AssignLeft::Ident(cache_name.clone()),
        val_expr,
    );

    // cache !== (cache = expr)
    let test = ctx.b.ast.expression_binary(
        oxc_span::SPAN,
        ctx.b.rid_expr(&cache_name),
        oxc_ast::ast::BinaryOperator::StrictInequality,
        cache_assign,
    );

    // el.__value = expr (using cloned expression)
    let dunder_value_assign = ctx.b.assign_expr(
        AssignLeft::StaticMember(
            ctx.b.static_member(ctx.b.rid_expr(el_name), "__value"),
        ),
        val_expr2,
    );

    // (el.__value = expr) ?? ""
    let coalesced = ctx.b.logical_coalesce(dunder_value_assign, ctx.b.str_expr(""));

    // el.value = (el.__value = expr) ?? ""
    let value_assign = ctx.b.assign_stmt(
        AssignLeft::StaticMember(
            ctx.b.static_member(ctx.b.rid_expr(el_name), "value"),
        ),
        coalesced,
    );

    // if (cache !== (cache = expr)) { el.value = ... }
    let if_body = ctx.b.block_stmt(vec![value_assign]);
    let if_stmt = ctx.b.if_stmt(test, if_body, None);

    // $.template_effect(() => { if_stmt })
    let effect_fn = ctx.b.arrow_block_expr(ctx.b.no_params(), vec![if_stmt]);
    let effect_call = ctx.b.call_stmt("$.template_effect", [Arg::Expr(effect_fn)]);
    init.push(effect_call);
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
    has_use_directive: bool,
) -> Option<BindPlacement<'a>> {
    // Pre-computed by analysis — no string-based symbol re-resolution needed.
    let is_rune = ctx.is_mutable_rune_target(bind.id);

    // Function bindings: bind:prop={(get_fn), (set_fn)} via SequenceExpression.
    // bind:this has its own SequenceExpression handling below; bind:group disallows it.
    if bind.name != "this" && bind.expression_span.is_some() {
        let is_seq = ctx.parsed.attr_exprs.get(&bind.id)
            .is_some_and(|e| matches!(e, oxc_ast::ast::Expression::SequenceExpression(_)));
        if is_seq {
            let expr = get_attr_expr(ctx, bind.id);
            let oxc_ast::ast::Expression::SequenceExpression(seq) = expr else { unreachable!() };
            let seq = seq.unbox();
            let mut exprs = seq.expressions.into_iter();
            let get_fn = exprs.next().expect("SequenceExpression must have getter");
            let set_fn = exprs.next().expect("SequenceExpression must have setter");

            let stmt = match bind.name.as_str() {
                "value" if tag_name == "select" => ctx.b.call_stmt("$.bind_select_value", [
                    Arg::Ident(el_name), Arg::Expr(get_fn), Arg::Expr(set_fn),
                ]),
                "value" => ctx.b.call_stmt("$.bind_value", [
                    Arg::Ident(el_name), Arg::Expr(get_fn), Arg::Expr(set_fn),
                ]),
                "checked" => ctx.b.call_stmt("$.bind_checked", [
                    Arg::Ident(el_name), Arg::Expr(get_fn), Arg::Expr(set_fn),
                ]),
                "files" => ctx.b.call_stmt("$.bind_files", [
                    Arg::Ident(el_name), Arg::Expr(get_fn), Arg::Expr(set_fn),
                ]),
                "innerHTML" | "innerText" | "textContent" => ctx.b.call_stmt("$.bind_content_editable", [
                    Arg::Str(bind.name.clone()), Arg::Ident(el_name), Arg::Expr(get_fn), Arg::Expr(set_fn),
                ]),
                _ => ctx.b.call_stmt("$.bind_value", [
                    Arg::Ident(el_name), Arg::Expr(get_fn), Arg::Expr(set_fn),
                ]),
            };
            return Some(BindPlacement::AfterUpdate(stmt));
        }
    }

    let var_name = if bind.shorthand {
        bind.name.clone()
    } else if let Some(span) = bind.expression_span {
        ctx.component.source_text(span).to_string()
    } else {
        return None;
    };

    let stmt = match bind.name.as_str() {
        // --- Input/Form ---
        "value" if tag_name == "select" => {
            let getter = build_binding_getter(ctx, &var_name, is_rune);
            let setter = build_binding_setter(ctx, var_name, is_rune);
            ctx.b.call_stmt("$.bind_select_value", [
                Arg::Ident(el_name), Arg::Expr(getter), Arg::Expr(setter),
            ])
        }
        "value" => {
            let getter = build_binding_getter(ctx, &var_name, is_rune);
            let setter = build_binding_setter(ctx, var_name, is_rune);
            ctx.b.call_stmt("$.bind_value", [
                Arg::Ident(el_name), Arg::Expr(getter), Arg::Expr(setter),
            ])
        }
        "checked" => {
            let getter = build_binding_getter(ctx, &var_name, is_rune);
            let setter = build_binding_setter(ctx, var_name, is_rune);
            ctx.b.call_stmt("$.bind_checked", [
                Arg::Ident(el_name), Arg::Expr(getter), Arg::Expr(setter),
            ])
        }
        "group" => {
            ctx.needs_binding_group = true;

            let has_each_var = ctx.parent_each_blocks(bind.id).is_some();

            // Getter and setter: when the expression references each-block vars,
            // use the pre-transformed expression (has $.get() applied).
            let (group_getter, setter) = if has_each_var {
                let getter_expr = if let Some(expr) = ctx.parsed.attr_exprs.get(&bind.id) {
                    ctx.b.clone_expr(expr)
                } else {
                    ctx.b.rid_expr(&var_name)
                };
                let setter_expr = if let Some(expr) = ctx.parsed.attr_exprs.get(&bind.id) {
                    ctx.b.clone_expr(expr)
                } else {
                    ctx.b.rid_expr(&var_name)
                };
                let getter = ctx.b.arrow_expr(ctx.b.no_params(), [ctx.b.expr_stmt(getter_expr)]);
                let setter = build_binding_setter_from_expr(ctx, setter_expr);
                (getter, setter)
            } else {
                let setter = build_binding_setter(ctx, var_name.clone(), is_rune);
                // Check if element has a dynamic value attribute — wrap getter in thunk
                let group_getter = if let Some(val_attr_id) = ctx.bind_group_value_attr(bind.id) {
                    let val_expr = if let Some(expr) = ctx.parsed.attr_exprs.get(&val_attr_id) {
                        ctx.b.clone_expr(expr)
                    } else {
                        ctx.b.str_expr("")
                    };
                    let val_stmt = ctx.b.expr_stmt(val_expr);
                    let getter_expr = if is_rune {
                        ctx.b.call_expr("$.get", [Arg::Ident(&var_name)])
                    } else {
                        ctx.b.rid_expr(&var_name)
                    };
                    let return_stmt = ctx.b.return_stmt(getter_expr);
                    ctx.b.arrow_block_expr(ctx.b.no_params(), vec![val_stmt, return_stmt])
                } else {
                    build_binding_getter(ctx, &var_name, is_rune)
                };
                (group_getter, setter)
            };

            // Build index array from ancestor each blocks whose vars appear in the expression
            let index_array = if let Some(parent_eaches) = ctx.parent_each_blocks(bind.id) {
                let indexes: Vec<_> = parent_eaches.iter().map(|&each_id| {
                    let idx_name = ctx.each_index_name(each_id)
                        .or_else(|| ctx.group_index_names.get(&each_id).cloned())
                        .unwrap_or_else(|| panic!("missing index name for each block {:?}", each_id));
                    ctx.b.rid_expr(&idx_name)
                }).collect();
                ctx.b.array_expr(indexes)
            } else {
                ctx.b.empty_array_expr()
            };

            ctx.b.call_stmt("$.bind_group", [
                Arg::Ident("binding_group"),
                Arg::Expr(index_array),
                Arg::Ident(el_name),
                Arg::Expr(group_getter),
                Arg::Expr(setter),
            ])
        }
        "files" => {
            let getter = build_binding_getter(ctx, &var_name, is_rune);
            let setter = build_binding_setter(ctx, var_name, is_rune);
            ctx.b.call_stmt("$.bind_files", [
                Arg::Ident(el_name), Arg::Expr(getter), Arg::Expr(setter),
            ])
        }

        // --- Generic event-based bindings (bidirectional) ---
        "indeterminate" => {
            let setter = build_binding_setter(ctx, var_name.clone(), is_rune);
            let getter = build_binding_getter(ctx, &var_name, is_rune);
            ctx.b.call_stmt("$.bind_property", [
                Arg::Str("indeterminate".into()), Arg::Str("change".into()),
                Arg::Ident(el_name), Arg::Expr(setter), Arg::Expr(getter),
            ])
        }
        "open" => {
            let setter = build_binding_setter(ctx, var_name.clone(), is_rune);
            let getter = build_binding_getter(ctx, &var_name, is_rune);
            ctx.b.call_stmt("$.bind_property", [
                Arg::Str("open".into()), Arg::Str("toggle".into()),
                Arg::Ident(el_name), Arg::Expr(setter), Arg::Expr(getter),
            ])
        }

        // --- Contenteditable ---
        "innerHTML" | "innerText" | "textContent" => {
            let getter = build_binding_getter(ctx, &var_name, is_rune);
            let setter = build_binding_setter(ctx, var_name, is_rune);
            ctx.b.call_stmt("$.bind_content_editable", [
                Arg::Str(bind.name.clone()), Arg::Ident(el_name),
                Arg::Expr(getter), Arg::Expr(setter),
            ])
        }

        // --- Dimension bindings (element size, readonly) ---
        "clientWidth" | "clientHeight" | "offsetWidth" | "offsetHeight" => {
            let setter = build_binding_setter(ctx, var_name, is_rune);
            ctx.b.call_stmt("$.bind_element_size", [
                Arg::Ident(el_name), Arg::Str(bind.name.clone()), Arg::Expr(setter),
            ])
        }

        // --- Dimension bindings (resize observer, readonly) ---
        "contentRect" | "contentBoxSize" | "borderBoxSize" | "devicePixelContentBoxSize" => {
            let setter = build_binding_setter(ctx, var_name, is_rune);
            ctx.b.call_stmt("$.bind_resize_observer", [
                Arg::Ident(el_name), Arg::Str(bind.name.clone()), Arg::Expr(setter),
            ])
        }

        // --- Media R/W bindings ---
        "currentTime" => {
            let getter = build_binding_getter(ctx, &var_name, is_rune);
            let setter = build_binding_setter(ctx, var_name, is_rune);
            ctx.b.call_stmt("$.bind_current_time", [
                Arg::Ident(el_name), Arg::Expr(getter), Arg::Expr(setter),
            ])
        }
        "playbackRate" => {
            let getter = build_binding_getter(ctx, &var_name, is_rune);
            let setter = build_binding_setter(ctx, var_name, is_rune);
            ctx.b.call_stmt("$.bind_playback_rate", [
                Arg::Ident(el_name), Arg::Expr(getter), Arg::Expr(setter),
            ])
        }
        "paused" => {
            let getter = build_binding_getter(ctx, &var_name, is_rune);
            let setter = build_binding_setter(ctx, var_name, is_rune);
            ctx.b.call_stmt("$.bind_paused", [
                Arg::Ident(el_name), Arg::Expr(getter), Arg::Expr(setter),
            ])
        }
        "volume" => {
            let getter = build_binding_getter(ctx, &var_name, is_rune);
            let setter = build_binding_setter(ctx, var_name, is_rune);
            ctx.b.call_stmt("$.bind_volume", [
                Arg::Ident(el_name), Arg::Expr(getter), Arg::Expr(setter),
            ])
        }
        "muted" => {
            let getter = build_binding_getter(ctx, &var_name, is_rune);
            let setter = build_binding_setter(ctx, var_name, is_rune);
            ctx.b.call_stmt("$.bind_muted", [
                Arg::Ident(el_name), Arg::Expr(getter), Arg::Expr(setter),
            ])
        }

        // --- Media RO bindings (setter only) ---
        "buffered" => {
            let setter = build_binding_setter(ctx, var_name, is_rune);
            ctx.b.call_stmt("$.bind_buffered", [Arg::Ident(el_name), Arg::Expr(setter)])
        }
        "seekable" => {
            let setter = build_binding_setter(ctx, var_name, is_rune);
            ctx.b.call_stmt("$.bind_seekable", [Arg::Ident(el_name), Arg::Expr(setter)])
        }
        "seeking" => {
            let setter = build_binding_setter(ctx, var_name, is_rune);
            ctx.b.call_stmt("$.bind_seeking", [Arg::Ident(el_name), Arg::Expr(setter)])
        }
        "ended" => {
            let setter = build_binding_setter(ctx, var_name, is_rune);
            ctx.b.call_stmt("$.bind_ended", [Arg::Ident(el_name), Arg::Expr(setter)])
        }
        "readyState" => {
            let setter = build_binding_setter(ctx, var_name, is_rune);
            ctx.b.call_stmt("$.bind_ready_state", [Arg::Ident(el_name), Arg::Expr(setter)])
        }
        "played" => {
            let setter = build_binding_setter(ctx, var_name, is_rune);
            ctx.b.call_stmt("$.bind_played", [Arg::Ident(el_name), Arg::Expr(setter)])
        }

        // --- Media/Image RO event-based bindings (bind_property, no bidirectional) ---
        "duration" => {
            let setter = build_binding_setter(ctx, var_name, is_rune);
            ctx.b.call_stmt("$.bind_property", [
                Arg::Str("duration".into()), Arg::Str("durationchange".into()),
                Arg::Ident(el_name), Arg::Expr(setter),
            ])
        }
        "videoWidth" => {
            let setter = build_binding_setter(ctx, var_name, is_rune);
            ctx.b.call_stmt("$.bind_property", [
                Arg::Str("videoWidth".into()), Arg::Str("resize".into()),
                Arg::Ident(el_name), Arg::Expr(setter),
            ])
        }
        "videoHeight" => {
            let setter = build_binding_setter(ctx, var_name, is_rune);
            ctx.b.call_stmt("$.bind_property", [
                Arg::Str("videoHeight".into()), Arg::Str("resize".into()),
                Arg::Ident(el_name), Arg::Expr(setter),
            ])
        }
        "naturalWidth" => {
            let setter = build_binding_setter(ctx, var_name, is_rune);
            ctx.b.call_stmt("$.bind_property", [
                Arg::Str("naturalWidth".into()), Arg::Str("load".into()),
                Arg::Ident(el_name), Arg::Expr(setter),
            ])
        }
        "naturalHeight" => {
            let setter = build_binding_setter(ctx, var_name, is_rune);
            ctx.b.call_stmt("$.bind_property", [
                Arg::Str("naturalHeight".into()), Arg::Str("load".into()),
                Arg::Ident(el_name), Arg::Expr(setter),
            ])
        }

        // --- Misc ---
        "focused" => {
            let setter = build_binding_setter(ctx, var_name, is_rune);
            ctx.b.call_stmt("$.bind_focused", [Arg::Ident(el_name), Arg::Expr(setter)])
        }

        // --- bind:this ---
        "this" => {
            // SequenceExpression: bind:this={(getter_expr), (setter_expr)}
            // Reference: build_bind_this in shared/utils.js
            if let Some(span) = bind.expression_span {
                let expr = get_attr_expr(ctx, bind.id);
                if let Expression::SequenceExpression(seq) = expr {
                    let seq = seq.unbox();
                    let mut exprs = seq.expressions.into_iter();
                    let get_expr = exprs.next().expect("SequenceExpression must have getter");
                    let set_expr = exprs.next().expect("SequenceExpression must have setter");

                    // Unwrap getter arrow: drop params, keep body
                    let get_arrow = if let Expression::ArrowFunctionExpression(arrow) = get_expr {
                        let arrow = arrow.unbox();
                        let body_stmts: Vec<_> = arrow.body.unbox().statements.into_iter().collect();
                        ctx.b.arrow_expr(ctx.b.no_params(), body_stmts)
                    } else {
                        get_expr
                    };

                    // Unwrap setter arrow: keep first param or add `_`, keep body
                    let set_arrow = if let Expression::ArrowFunctionExpression(arrow) = set_expr {
                        let arrow = arrow.unbox();
                        let body_stmts: Vec<_> = arrow.body.unbox().statements.into_iter().collect();
                        let first_param = arrow.params.unbox().items.into_iter().next();
                        let params = if let Some(param) = first_param {
                            let mut items = ctx.b.ast.vec();
                            items.push(param);
                            ctx.b.ast.formal_parameters(
                                oxc_span::SPAN,
                                oxc_ast::ast::FormalParameterKind::ArrowFormalParameters,
                                items,
                                oxc_ast::NONE,
                            )
                        } else {
                            ctx.b.params(["_"])
                        };
                        ctx.b.arrow_expr(params, body_stmts)
                    } else {
                        set_expr
                    };

                    let stmt = ctx.b.call_stmt("$.bind_this", [
                        Arg::Ident(el_name), Arg::Expr(set_arrow), Arg::Expr(get_arrow),
                    ]);
                    return Some(BindPlacement::Init(stmt));
                }
                // Not a SequenceExpression — fall through to normal bind:this below,
                // but we already consumed the expression. Re-parse it.
                let _ = span; // expression was consumed, re-get below
            }

            // svelte:element uses proxy flag because the element type is unknown at compile time
            let setter = if tag_name.is_empty() && is_rune {
                let body = ctx.b.call_expr("$.set", [
                    Arg::Ident(&var_name), Arg::Ident("$$value"), Arg::Expr(ctx.b.bool_expr(true)),
                ]);
                ctx.b.arrow_expr(ctx.b.params(["$$value"]), [ctx.b.expr_stmt(body)])
            } else {
                build_binding_setter(ctx, var_name.clone(), is_rune)
            };
            let getter = build_binding_getter(ctx, &var_name, is_rune);
            let stmt = ctx.b.call_stmt("$.bind_this", [
                Arg::Ident(el_name), Arg::Expr(setter), Arg::Expr(getter),
            ]);
            return Some(BindPlacement::Init(stmt));
        }

        // Fallback for unknown bindings
        _ => {
            let getter = build_binding_getter(ctx, &var_name, is_rune);
            let setter = build_binding_setter(ctx, var_name, is_rune);
            ctx.b.call_stmt("$.bind_value", [
                Arg::Ident(el_name), Arg::Expr(getter), Arg::Expr(setter),
            ])
        }
    };

    // Wrap in $.effect() when element has use: directive (deferral)
    if has_use_directive && bind.name != "this" {
        let effect_body = ctx.b.arrow_expr(ctx.b.no_params(), [stmt]);
        let effect_stmt = ctx.b.call_stmt("$.effect", [Arg::Expr(effect_body)]);
        return Some(BindPlacement::Init(effect_stmt));
    }

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
                let has_use = ctx.has_use_directive(el.id);
                if let Some(placement) = gen_bind_directive(ctx, bind, el_name, &el.name, has_use) {
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
    let style_dirs: Vec<_> = el.attributes.iter()
        .filter_map(|a| match a {
            Attribute::StyleDirective(sd) => Some(sd),
            _ => None,
        })
        .collect();

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
            Arg::Str("".into()),
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

    let name_expr = ctx.parsed.directive_name_exprs.remove(&ud.id).unwrap();

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

    let name_expr = ctx.parsed.directive_name_exprs.remove(&td.id).unwrap();
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
    let name_expr = ctx.parsed.directive_name_exprs.remove(&ad.id).unwrap();
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

/// Build Svelte 5 event attribute handler (non-dev mode).
/// Reference: `events.js` `build_event_handler()`.
///
/// 1. Arrow/Function → pass through
/// 2. Identifier that is NOT an import → pass through
/// 3. If `has_call` → memoize with `$.derived`/`$.get`
/// 4. Remaining → wrap in `function(...$$args) { handler.apply(this, $$args) }`
pub(crate) fn build_event_handler_s5<'a>(
    ctx: &mut Ctx<'a>,
    attr_id: NodeId,
    handler: Expression<'a>,
    has_call: bool,
    init: &mut Vec<Statement<'a>>,
) -> Expression<'a> {
    match &handler {
        Expression::ArrowFunctionExpression(_) | Expression::FunctionExpression(_) => {
            return handler;
        }
        Expression::Identifier(_) => {
            // Non-import identifiers are stable references — no wrapping needed.
            // Import identifiers may be live bindings, so they need derived+get wrapping.
            let is_import = ctx.analysis.attr_expression(attr_id)
                .and_then(|info| info.references.first())
                .and_then(|r| r.symbol_id)
                .is_some_and(|sym| ctx.is_import_sym(sym));
            if !is_import {
                return handler;
            }
        }
        _ => {}
    }

    let mut handler = handler;

    if has_call {
        let id = ctx.gen_ident("event_handler");
        let thunk = ctx.b.thunk(handler);
        init.push(ctx.b.var_stmt(&id, ctx.b.call_expr("$.derived", [Arg::Expr(thunk)])));
        handler = ctx.b.call_expr("$.get", [Arg::Ident(&id)]);
    }

    let call = ctx.b.optional_member_call_expr(handler, "apply", [
        Arg::Expr(ctx.b.this_expr()),
        Arg::Ident("$$args"),
    ]);
    ctx.b.function_expr(ctx.b.rest_params("$$args"), vec![ctx.b.expr_stmt(call)])
}

/// In dev mode, convert arrow event handlers to named functions and wrap `$inspect.trace()`.
/// Reference: `events.js` `build_event()` lines 66-76.
/// `expr_offset` is the byte offset of the expression in the component source.
fn dev_event_handler<'a>(ctx: &mut Ctx<'a>, handler: Expression<'a>, event_name: &str, expr_offset: u32) -> Expression<'a> {
    if !ctx.dev {
        return handler;
    }

    let Expression::ArrowFunctionExpression(arrow) = handler else {
        return handler;
    };
    let arrow = arrow.unbox();

    // Convert arrow body to block statements
    let mut body = arrow.body.unbox();
    let mut stmts: Vec<Statement<'a>> = body.statements.drain(..).collect();

    // Check for $inspect.trace() as first statement
    let has_trace = stmts.first().is_some_and(|s| {
        if let Statement::ExpressionStatement(es) = s {
            is_inspect_trace_call(&es.expression)
        } else {
            false
        }
    });

    if has_trace {
        // Remove the trace statement
        let trace_stmt = stmts.remove(0);
        let Statement::ExpressionStatement(es) = trace_stmt else { unreachable!() };
        let es = es.unbox();
        let Expression::CallExpression(call) = es.expression else { unreachable!() };
        let mut call = call.unbox();

        // Extract explicit label or generate auto-label
        let label_expr = if !call.arguments.is_empty() {
            let mut dummy = oxc_ast::ast::Argument::from(ctx.b.cheap_expr());
            std::mem::swap(&mut call.arguments[0], &mut dummy);
            dummy.into_expression()
        } else {
            let (line, col) = crate::script::compute_line_col(ctx.source, expr_offset + arrow.span.start);
            let sanitized = crate::script::sanitize_location(ctx.filename);
            let label = format!("trace ({sanitized}:{line}:{col})");
            ctx.b.str_expr(&label)
        };
        let label_thunk = ctx.b.thunk(label_expr);

        let body_thunk = if arrow.r#async {
            ctx.b.async_thunk_block(stmts)
        } else {
            ctx.b.thunk_block(stmts)
        };

        let trace_call = ctx.b.call_expr("$.trace", [
            Arg::Expr(label_thunk),
            Arg::Expr(body_thunk),
        ]);
        let return_expr = if arrow.r#async {
            ctx.b.await_expr(trace_call)
        } else {
            trace_call
        };
        stmts = vec![ctx.b.return_stmt(return_expr)];
        ctx.has_tracing = true;
    }

    ctx.b.named_function_expr(
        event_name,
        arrow.params.unbox(),
        stmts,
        arrow.r#async,
    )
}

/// Check if an expression is `$inspect.trace(...)`.
fn is_inspect_trace_call(expr: &Expression) -> bool {
    if let Expression::CallExpression(call) = expr {
        if let Expression::StaticMemberExpression(member) = &call.callee {
            if member.property.name.as_str() == "trace" {
                if let Expression::Identifier(id) = &member.object {
                    return id.name.as_str() == "$inspect";
                }
            }
        }
    }
    false
}

/// Build event handler for legacy on:directive (non-dev mode).
/// Reference: `events.js` `build_event_handler()`.
pub(crate) fn build_legacy_event_handler<'a>(ctx: &mut Ctx<'a>, handler: Expression<'a>) -> Expression<'a> {
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

/// LEGACY(svelte4): Generate `$.event()` for legacy `on:directive` on a global element
/// (svelte:body, svelte:document, svelte:window). `target` is the element expression
/// (e.g. `"$.document.body"`, `"$.document"`, `"$.window"`).
pub(crate) fn gen_legacy_event_on<'a>(
    ctx: &mut Ctx<'a>,
    od: &svelte_ast::OnDirectiveLegacy,
    attr_id: NodeId,
    target: &str,
    stmts: &mut Vec<Statement<'a>>,
) {
    let handler = if od.expression_span.is_none() {
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
        let expr = super::expression::get_attr_expr(ctx, attr_id);
        build_legacy_event_handler(ctx, expr)
    };

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

    let capture = od.modifiers.iter().any(|m| m == "capture");
    let passive = od.modifiers.iter().find_map(|m| match m.as_str() {
        "passive" => Some(true),
        "nonpassive" => Some(false),
        _ => None,
    });

    let mut args: Vec<Arg<'a, '_>> = vec![
        Arg::Str(od.name.clone()),
        Arg::Ident(target),
        Arg::Expr(wrapped),
    ];
    if capture || passive.is_some() {
        args.push(Arg::Bool(capture));
    }
    if let Some(p) = passive {
        args.push(Arg::Bool(p));
    }

    stmts.push(ctx.b.call_stmt("$.event", args));
}

/// Generate Svelte 5 event attribute `$.event()` on a global element.
/// `target` is the element expression (e.g. `"$.document.body"`, `"$.window"`).
pub(crate) fn gen_event_attr_on<'a>(
    ctx: &mut Ctx<'a>,
    attr_id: NodeId,
    raw_event_name: &str,
    target: &str,
    stmts: &mut Vec<Statement<'a>>,
    expr_offset: u32,
) {
    let (event_name, capture) = if let Some(base) = svelte_js::strip_capture_event(raw_event_name) {
        (base.to_string(), true)
    } else {
        (raw_event_name.to_string(), false)
    };

    let has_call = ctx.analysis.attr_expression(attr_id).map_or(false, |e| e.has_call);
    let handler_expr = super::expression::get_attr_expr(ctx, attr_id);
    let handler = build_event_handler_s5(ctx, attr_id, handler_expr, has_call, stmts);
    let handler = dev_event_handler(ctx, handler, &event_name, expr_offset);

    let passive = svelte_js::is_passive_event(&event_name);
    let mut args: Vec<Arg<'a, '_>> = vec![
        Arg::Str(event_name.clone()),
        Arg::Ident(target),
        Arg::Expr(handler),
    ];
    if capture || passive {
        args.push(if capture { Arg::Bool(true) } else { Arg::Expr(ctx.b.void_zero_expr()) });
    }
    if passive {
        args.push(Arg::Bool(true));
    }
    stmts.push(ctx.b.call_stmt("$.event", args));
}
