//! Attribute processing (per-attribute and spread).

use oxc_ast::ast::Statement;

use svelte_ast::{Attribute, BindDirectiveKind, Element};

use crate::builder::{Arg, AssignLeft, AssignRight, ObjProp};
use crate::context::Ctx;

use super::expression::{build_attr_concat, parse_expr};

/// Process a single attribute (non-spread path).
pub(crate) fn process_attr<'a>(
    ctx: &mut Ctx<'a>,
    attr: &Attribute,
    el_name: &str,
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
            let val = parse_expr(ctx, a.expression_span);
            target.push(ctx.b.call_stmt(
                "$.set_attribute",
                [
                    Arg::Ident(el_name),
                    Arg::Str(a.name.clone()),
                    Arg::Expr(val),
                ],
            ));
        }
        Attribute::ConcatenationAttribute(a) => {
            let val = build_attr_concat(ctx, &a.parts);
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
            let val = parse_expr(ctx, a.expression_span);
            let name = ctx.component.source_text(a.expression_span).to_string();
            target.push(ctx.b.call_stmt(
                "$.set_attribute",
                [Arg::Ident(el_name), Arg::Str(name), Arg::Expr(val)],
            ));
        }
        Attribute::BindDirective(bind) => {
            let var_name = if bind.shorthand {
                bind.name.clone()
            } else if let Some(span) = bind.expression_span {
                ctx.component.source_text(span).trim().to_string()
            } else {
                return;
            };

            let is_mutated_rune = ctx.analysis.mutated_runes.contains(&var_name)
                && ctx
                    .analysis
                    .symbol_by_name
                    .get(&var_name)
                    .is_some_and(|sid| ctx.analysis.runes.contains_key(sid));

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

            let stmt = match bind.kind {
                BindDirectiveKind::Checked => ctx.b.call_stmt(
                    "$.bind_checked",
                    [Arg::Ident(el_name), Arg::Expr(getter), Arg::Expr(setter)],
                ),
                BindDirectiveKind::Group => {
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
            after_update.push(stmt);
        }
        Attribute::ShorthandOrSpread(_) | Attribute::ClassDirective(_) => {
            // Spread handled by process_attrs_spread; class directives by process_class_directives
        }
    }
}

/// Generate `$.set_class(el, 1, "", null, classes, { ... })` for class:name directives.
pub(crate) fn process_class_directives<'a>(
    ctx: &mut Ctx<'a>,
    el: &Element,
    el_name: &str,
    init: &mut Vec<Statement<'a>>,
) {
    let class_dirs: Vec<_> = el
        .attributes
        .iter()
        .filter_map(|a| match a {
            Attribute::ClassDirective(cd) => Some(cd),
            _ => None,
        })
        .collect();

    if class_dirs.is_empty() {
        return;
    }

    let mut props: Vec<ObjProp<'a>> = Vec::new();

    for cd in &class_dirs {
        let name = &cd.name;

        let (expr, is_simple_ident_same_name) = if let Some(span) = cd.expression_span {
            let expr_text = ctx.component.source_text(span).trim().to_string();
            let parsed = parse_expr(ctx, span);
            let same_name = expr_text == *name;
            (parsed, same_name)
        } else {
            // shorthand: class:name → identifier `name`
            (ctx.b.rid_expr(name), true)
        };

        let is_mutated_rune = ctx.analysis.mutated_runes.contains(name)
            && ctx
                .analysis
                .symbol_by_name
                .get(name)
                .is_some_and(|sid| ctx.analysis.runes.contains_key(sid));

        let name_alloc = ctx.b.ast.allocator.alloc_str(name);

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

    // $.set_class(el, 1, "", null, classes, { ... })
    let set_class_call = ctx.b.call_expr(
        "$.set_class",
        [
            Arg::Ident(el_name),
            Arg::Num(1.0),
            Arg::Str(String::new()),
            Arg::Expr(ctx.b.null_expr()),
            Arg::Ident("classes"),
            Arg::Expr(obj),
        ],
    );

    // classes = $.set_class(...)
    let assign = ctx.b.assign_expr(
        AssignLeft::Ident("classes".to_string()),
        AssignRight::Expr(set_class_call),
    );

    // () => classes = $.set_class(...)
    let arrow = ctx.b.arrow_expr(ctx.b.no_params(), [ctx.b.expr_stmt(assign)]);

    // $.template_effect(() => ...)
    let effect_stmt = ctx.b.call_stmt("$.template_effect", [Arg::Expr(arrow)]);

    // let classes;
    init.push(ctx.b.let_stmt("classes"));
    init.push(effect_stmt);
}

/// Check if an element has any spread attribute.
pub(crate) fn has_spread(el: &Element) -> bool {
    el.attributes
        .iter()
        .any(|a| matches!(a, Attribute::ShorthandOrSpread(s) if s.is_spread))
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
        match attr {
            Attribute::BooleanAttribute(a) => {
                let name_alloc = ctx.b.ast.allocator.alloc_str(&a.name);
                props.push(ObjProp::KeyValue(name_alloc, ctx.b.bool_expr(true)));
            }
            Attribute::StringAttribute(a) => {
                let val = ctx.component.source_text(a.value_span).to_string();
                let name_alloc = ctx.b.ast.allocator.alloc_str(&a.name);
                props.push(ObjProp::KeyValue(name_alloc, ctx.b.str_expr(&val)));
            }
            Attribute::ExpressionAttribute(a) => {
                let expr = parse_expr(ctx, a.expression_span);
                let expr_text = ctx.component.source_text(a.expression_span).trim();
                if a.name == expr_text {
                    // Property shorthand: name matches expression
                    let name_alloc = ctx.b.ast.allocator.alloc_str(&a.name);
                    props.push(ObjProp::Shorthand(name_alloc));
                } else {
                    let name_alloc = ctx.b.ast.allocator.alloc_str(&a.name);
                    props.push(ObjProp::KeyValue(name_alloc, expr));
                }
            }
            Attribute::ConcatenationAttribute(a) => {
                let val = build_attr_concat(ctx, &a.parts);
                let name_alloc = ctx.b.ast.allocator.alloc_str(&a.name);
                props.push(ObjProp::KeyValue(name_alloc, val));
            }
            Attribute::ShorthandOrSpread(a) if a.is_spread => {
                // expression_span includes "..." prefix — skip it
                let span = svelte_span::Span::new(
                    a.expression_span.start + 3,
                    a.expression_span.end,
                );
                let expr = parse_expr(ctx, span);
                props.push(ObjProp::Spread(expr));
            }
            Attribute::ShorthandOrSpread(a) => {
                let name = ctx.component.source_text(a.expression_span).trim();
                let name_alloc = ctx.b.ast.allocator.alloc_str(name);
                props.push(ObjProp::Shorthand(name_alloc));
            }
            Attribute::BindDirective(bind) => {
                // Bind directives are handled separately
                let var_name = if bind.shorthand {
                    bind.name.clone()
                } else if let Some(span) = bind.expression_span {
                    ctx.component.source_text(span).trim().to_string()
                } else {
                    continue;
                };

                let is_mutated_rune = ctx.analysis.mutated_runes.contains(&var_name)
                    && ctx
                        .analysis
                        .symbol_by_name
                        .get(&var_name)
                        .is_some_and(|sid| ctx.analysis.runes.contains_key(sid));

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

                let stmt = match bind.kind {
                    BindDirectiveKind::Checked => ctx.b.call_stmt(
                        "$.bind_checked",
                        [Arg::Ident(el_name), Arg::Expr(getter), Arg::Expr(setter)],
                    ),
                    BindDirectiveKind::Group => {
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
                after_update.push(stmt);
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
