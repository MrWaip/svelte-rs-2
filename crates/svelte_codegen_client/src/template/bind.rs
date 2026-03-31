//! Bind directive codegen (`bind:value`, `bind:checked`, `bind:group`, etc.).

use oxc_ast::ast::{Expression, Statement};

use svelte_ast::NodeId;

use crate::builder::{Arg, AssignLeft};
use crate::context::Ctx;

use super::expression::get_attr_expr;

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
pub(crate) fn emit_bind_group_value<'a>(
    ctx: &mut Ctx<'a>,
    el_name: &str,
    _attr_id: NodeId,
    val_expr: Expression<'a>,
    init: &mut Vec<Statement<'a>>,
    _update: &mut Vec<Statement<'a>>,
) {
    let mut prefix = String::with_capacity(el_name.len() + 6);
    prefix.push_str(el_name);
    prefix.push_str("_value");
    let cache_name = ctx.gen_ident(&prefix);

    // var input_value;
    init.push(ctx.b.var_uninit_stmt(&cache_name));

    // Clone expression for reuse in __value body
    let val_expr2 = ctx.b.clone_expr(&val_expr);

    // cache = expr
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
pub(crate) enum BindPlacement<'a> {
    /// Most bindings: placed after attribute updates.
    AfterUpdate(Statement<'a>),
    /// `bind:this`: placed in init (before render effect).
    Init(Statement<'a>),
}

/// Generate a bind directive statement (getter/setter + runtime call).
pub(crate) fn gen_bind_directive<'a>(
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
        let bind_handle = bind.expression_span.and_then(|span| ctx.state.parsed.expr_handle(span.start));
        let is_seq = bind_handle
            .and_then(|handle| ctx.state.parsed.expr(handle))
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
                    Arg::StrRef(&bind.name), Arg::Ident(el_name), Arg::Expr(get_fn), Arg::Expr(set_fn),
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
        ctx.query.component.source_text(span).to_string()
    } else {
        return None;
    };

    // Pre-computed blocker indices from analyze
    let bind_blockers = ctx.bind_blockers(bind.id).to_vec();

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
            ctx.state.needs_binding_group = true;

            let has_each_var = ctx.parent_each_blocks(bind.id).is_some();

            // Getter and setter: when the expression references each-block vars,
            // use the pre-transformed expression (has $.get() applied).
            let (group_getter, setter) = if has_each_var {
                let bind_handle = bind.expression_span.and_then(|span| ctx.state.parsed.expr_handle(span.start));
                let getter_expr = bind_handle.and_then(|handle| ctx.state.parsed.expr(handle))
                    .map(|expr| ctx.b.clone_expr(expr))
                    .unwrap_or_else(|| ctx.b.rid_expr(&var_name));
                let setter_expr = bind_handle.and_then(|handle| ctx.state.parsed.expr(handle))
                    .map(|expr| ctx.b.clone_expr(expr))
                    .unwrap_or_else(|| ctx.b.rid_expr(&var_name));
                let getter = ctx.b.arrow_expr(ctx.b.no_params(), [ctx.b.expr_stmt(getter_expr)]);
                let setter = build_binding_setter_from_expr(ctx, setter_expr);
                (getter, setter)
            } else {
                let setter = build_binding_setter(ctx, var_name.clone(), is_rune);
                // Check if element has a dynamic value attribute — wrap getter in thunk
                let group_getter = if let Some(val_attr_id) = ctx.bind_group_value_attr(bind.id) {
                    let val_expr = ctx.state.parsed.expr(ctx.attr_expr_handle(val_attr_id))
                        .map(|expr| ctx.b.clone_expr(expr))
                        .unwrap_or_else(|| ctx.b.str_expr(""));
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
                Arg::StrRef("indeterminate"), Arg::StrRef("change"),
                Arg::Ident(el_name), Arg::Expr(setter), Arg::Expr(getter),
            ])
        }
        "open" => {
            let setter = build_binding_setter(ctx, var_name.clone(), is_rune);
            let getter = build_binding_getter(ctx, &var_name, is_rune);
            ctx.b.call_stmt("$.bind_property", [
                Arg::StrRef("open"), Arg::StrRef("toggle"),
                Arg::Ident(el_name), Arg::Expr(setter), Arg::Expr(getter),
            ])
        }

        // --- Contenteditable ---
        "innerHTML" | "innerText" | "textContent" => {
            let getter = build_binding_getter(ctx, &var_name, is_rune);
            let setter = build_binding_setter(ctx, var_name, is_rune);
            ctx.b.call_stmt("$.bind_content_editable", [
                Arg::StrRef(&bind.name), Arg::Ident(el_name),
                Arg::Expr(getter), Arg::Expr(setter),
            ])
        }

        // --- Dimension bindings (element size, readonly) ---
        "clientWidth" | "clientHeight" | "offsetWidth" | "offsetHeight" => {
            let setter = build_binding_setter(ctx, var_name, is_rune);
            ctx.b.call_stmt("$.bind_element_size", [
                Arg::Ident(el_name), Arg::StrRef(&bind.name), Arg::Expr(setter),
            ])
        }

        // --- Dimension bindings (resize observer, readonly) ---
        "contentRect" | "contentBoxSize" | "borderBoxSize" | "devicePixelContentBoxSize" => {
            let setter = build_binding_setter(ctx, var_name, is_rune);
            ctx.b.call_stmt("$.bind_resize_observer", [
                Arg::Ident(el_name), Arg::StrRef(&bind.name), Arg::Expr(setter),
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
                Arg::StrRef("duration"), Arg::StrRef("durationchange"),
                Arg::Ident(el_name), Arg::Expr(setter),
            ])
        }
        "videoWidth" => {
            let setter = build_binding_setter(ctx, var_name, is_rune);
            ctx.b.call_stmt("$.bind_property", [
                Arg::StrRef("videoWidth"), Arg::StrRef("resize"),
                Arg::Ident(el_name), Arg::Expr(setter),
            ])
        }
        "videoHeight" => {
            let setter = build_binding_setter(ctx, var_name, is_rune);
            ctx.b.call_stmt("$.bind_property", [
                Arg::StrRef("videoHeight"), Arg::StrRef("resize"),
                Arg::Ident(el_name), Arg::Expr(setter),
            ])
        }
        "naturalWidth" => {
            let setter = build_binding_setter(ctx, var_name, is_rune);
            ctx.b.call_stmt("$.bind_property", [
                Arg::StrRef("naturalWidth"), Arg::StrRef("load"),
                Arg::Ident(el_name), Arg::Expr(setter),
            ])
        }
        "naturalHeight" => {
            let setter = build_binding_setter(ctx, var_name, is_rune);
            ctx.b.call_stmt("$.bind_property", [
                Arg::StrRef("naturalHeight"), Arg::StrRef("load"),
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
    let mut stmt = stmt;
    if has_use_directive && bind.name != "this" {
        let effect_body = ctx.b.arrow_expr(ctx.b.no_params(), [stmt]);
        stmt = ctx.b.call_stmt("$.effect", [Arg::Expr(effect_body)]);

        if !bind_blockers.is_empty() {
            stmt = gen_run_after_blockers(ctx, stmt, &bind_blockers);
        }
        return Some(BindPlacement::Init(stmt));
    }

    // Wrap in $.run_after_blockers if bind target has blockers
    if !bind_blockers.is_empty() {
        stmt = gen_run_after_blockers(ctx, stmt, &bind_blockers);
    }

    Some(BindPlacement::AfterUpdate(stmt))
}

/// Wrap a statement in `$.run_after_blockers(blockers, () => { stmt })`.
fn gen_run_after_blockers<'a>(ctx: &mut Ctx<'a>, stmt: Statement<'a>, blockers: &[u32]) -> Statement<'a> {
    let blockers_arr = ctx.b.promises_array(blockers);
    let thunk = ctx.b.thunk_block(vec![stmt]);
    ctx.b.call_stmt("$.run_after_blockers", [
        Arg::Expr(blockers_arr),
        Arg::Expr(thunk),
    ])
}
