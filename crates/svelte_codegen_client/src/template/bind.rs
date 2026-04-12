//! Bind directive codegen (`bind:value`, `bind:checked`, `bind:group`, etc.).

use oxc_ast::ast::{Expression, Statement};

use svelte_analyze::{BindPropertyKind, ImageNaturalSizeKind, MediaBindKind};
use svelte_ast::NodeId;

use crate::builder::{Arg, AssignLeft};
use crate::context::Ctx;

use super::expression::get_attr_expr;

/// Build binding getter: `() => $.get(x)` for runes, `() => x` for plain vars.
pub(crate) fn build_binding_getter<'a>(
    ctx: &mut Ctx<'a>,
    var: &str,
    is_rune: bool,
) -> Expression<'a> {
    let body = if is_rune {
        ctx.b.call_expr("$.get", [Arg::Ident(var)])
    } else {
        ctx.b.rid_expr(var)
    };
    ctx.b.arrow_expr(ctx.b.no_params(), [ctx.b.expr_stmt(body)])
}

/// Build binding setter: `($$value) => $.set(x, $$value)` for runes, `($$value) => x = $$value` for plain.
pub(crate) fn build_binding_setter<'a>(
    ctx: &mut Ctx<'a>,
    var: String,
    is_rune: bool,
) -> Expression<'a> {
    let body = if is_rune {
        ctx.b
            .call_expr("$.set", [Arg::Ident(&var), Arg::Ident("$$value")])
    } else {
        ctx.b
            .assign_expr(AssignLeft::Ident(var), ctx.b.rid_expr("$$value"))
    };
    ctx.b
        .arrow_expr(ctx.b.params(["$$value"]), [ctx.b.expr_stmt(body)])
}

/// Build binding setter with `true` flag: `($$value) => $.set(x, $$value, true)`.
/// Used by window/document bindings to prevent re-triggering reactivity.
pub(crate) fn build_binding_setter_silent<'a>(
    ctx: &mut Ctx<'a>,
    var: String,
    is_rune: bool,
) -> Expression<'a> {
    let body = if is_rune {
        ctx.b.call_expr(
            "$.set",
            [Arg::Ident(&var), Arg::Ident("$$value"), Arg::Bool(true)],
        )
    } else {
        ctx.b
            .assign_expr(AssignLeft::Ident(var), ctx.b.rid_expr("$$value"))
    };
    ctx.b
        .arrow_expr(ctx.b.params(["$$value"]), [ctx.b.expr_stmt(body)])
}

/// Build binding setter from a pre-transformed expression:
/// `($$value) => transformed_expr = $$value`.
/// Used for bind:group when the expression references each-block vars
/// (the expression already has $.get() applied by the transform phase).
fn build_binding_setter_from_expr<'a>(ctx: &mut Ctx<'a>, expr: Expression<'a>) -> Expression<'a> {
    let target = ctx.b.expr_to_assignment_target(expr);
    let assign = ctx.b.assign_expr_raw(target, ctx.b.rid_expr("$$value"));
    ctx.b
        .arrow_expr(ctx.b.params(["$$value"]), [ctx.b.expr_stmt(assign)])
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
    let cache_assign = ctx
        .b
        .assign_expr(AssignLeft::Ident(cache_name.clone()), val_expr);

    // cache !== (cache = expr)
    let test = ctx.b.ast.expression_binary(
        oxc_span::SPAN,
        ctx.b.rid_expr(&cache_name),
        oxc_ast::ast::BinaryOperator::StrictInequality,
        cache_assign,
    );

    // el.__value = expr (using cloned expression)
    let dunder_value_assign = ctx.b.assign_expr(
        AssignLeft::StaticMember(ctx.b.static_member(ctx.b.rid_expr(el_name), "__value")),
        val_expr2,
    );

    // (el.__value = expr) ?? ""
    let coalesced = ctx
        .b
        .logical_coalesce(dunder_value_assign, ctx.b.str_expr(""));

    // el.value = (el.__value = expr) ?? ""
    let value_assign = ctx.b.assign_stmt(
        AssignLeft::StaticMember(ctx.b.static_member(ctx.b.rid_expr(el_name), "value")),
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

pub(crate) fn gen_bind_property_stmt<'a>(
    ctx: &mut Ctx<'a>,
    property_name: &'static str,
    event_name: &'static str,
    target_ident: &str,
    setter: Expression<'a>,
    getter: Option<Expression<'a>>,
) -> Statement<'a> {
    let mut args = vec![
        Arg::StrRef(property_name),
        Arg::StrRef(event_name),
        Arg::Ident(target_ident),
        Arg::Expr(setter),
    ];
    if let Some(getter) = getter {
        args.push(Arg::Expr(getter));
    }
    ctx.b.call_stmt("$.bind_property", args)
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
    let semantics = ctx.bind_target_semantics(bind.id)?;
    let bind_property = semantics.property();
    let is_rune = ctx.is_mutable_rune_target(bind.id);
    let is_prop_source = ctx.is_prop_source_node(bind.id);

    // Function bindings: bind:prop={(get_fn), (set_fn)} via SequenceExpression.
    // bind:this has its own SequenceExpression handling below; bind:group disallows it.
    if !semantics.is_this() && bind.expression_span.is_some() {
        let bind_handle = bind
            .expression_span
            .and_then(|span| ctx.state.parsed.expr_handle(span.start));
        let is_seq = bind_handle
            .and_then(|handle| ctx.state.parsed.expr(handle))
            .is_some_and(|e| matches!(e, oxc_ast::ast::Expression::SequenceExpression(_)));
        if is_seq {
            let expr = get_attr_expr(ctx, bind.id);
            let oxc_ast::ast::Expression::SequenceExpression(seq) = expr else {
                unreachable!()
            };
            let seq = seq.unbox();
            let mut exprs = seq.expressions.into_iter();
            let get_fn = exprs.next().expect("SequenceExpression must have getter");
            let set_fn = exprs.next().expect("SequenceExpression must have setter");

            let stmt = match bind_property {
                BindPropertyKind::Value if tag_name == "select" => ctx.b.call_stmt(
                    "$.bind_select_value",
                    [Arg::Ident(el_name), Arg::Expr(get_fn), Arg::Expr(set_fn)],
                ),
                BindPropertyKind::Value => ctx.b.call_stmt(
                    "$.bind_value",
                    [Arg::Ident(el_name), Arg::Expr(get_fn), Arg::Expr(set_fn)],
                ),
                BindPropertyKind::Checked => ctx.b.call_stmt(
                    "$.bind_checked",
                    [Arg::Ident(el_name), Arg::Expr(get_fn), Arg::Expr(set_fn)],
                ),
                BindPropertyKind::Files => ctx.b.call_stmt(
                    "$.bind_files",
                    [Arg::Ident(el_name), Arg::Expr(get_fn), Arg::Expr(set_fn)],
                ),
                BindPropertyKind::Indeterminate => gen_bind_property_stmt(
                    ctx,
                    "indeterminate",
                    "change",
                    el_name,
                    set_fn,
                    Some(get_fn),
                ),
                BindPropertyKind::Open => {
                    gen_bind_property_stmt(ctx, "open", "toggle", el_name, set_fn, Some(get_fn))
                }
                BindPropertyKind::ContentEditable(kind) => ctx.b.call_stmt(
                    "$.bind_content_editable",
                    [
                        Arg::StrRef(kind.name()),
                        Arg::Ident(el_name),
                        Arg::Expr(get_fn),
                        Arg::Expr(set_fn),
                    ],
                ),
                BindPropertyKind::Media(MediaBindKind::CurrentTime) => ctx.b.call_stmt(
                    "$.bind_current_time",
                    [Arg::Ident(el_name), Arg::Expr(get_fn), Arg::Expr(set_fn)],
                ),
                BindPropertyKind::Media(MediaBindKind::PlaybackRate) => ctx.b.call_stmt(
                    "$.bind_playback_rate",
                    [Arg::Ident(el_name), Arg::Expr(get_fn), Arg::Expr(set_fn)],
                ),
                BindPropertyKind::Media(MediaBindKind::Paused) => ctx.b.call_stmt(
                    "$.bind_paused",
                    [Arg::Ident(el_name), Arg::Expr(get_fn), Arg::Expr(set_fn)],
                ),
                BindPropertyKind::Media(MediaBindKind::Volume) => ctx.b.call_stmt(
                    "$.bind_volume",
                    [Arg::Ident(el_name), Arg::Expr(get_fn), Arg::Expr(set_fn)],
                ),
                BindPropertyKind::Media(MediaBindKind::Muted) => ctx.b.call_stmt(
                    "$.bind_muted",
                    [Arg::Ident(el_name), Arg::Expr(get_fn), Arg::Expr(set_fn)],
                ),
                _ => ctx.b.call_stmt(
                    "$.bind_value",
                    [Arg::Ident(el_name), Arg::Expr(get_fn), Arg::Expr(set_fn)],
                ),
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

    let stmt = match bind_property {
        // --- Input/Form ---
        BindPropertyKind::Value if tag_name == "select" => {
            let getter = build_binding_getter(ctx, &var_name, is_rune);
            let setter = build_binding_setter(ctx, var_name, is_rune);
            ctx.b.call_stmt(
                "$.bind_select_value",
                [Arg::Ident(el_name), Arg::Expr(getter), Arg::Expr(setter)],
            )
        }
        BindPropertyKind::Value => {
            let getter = build_binding_getter(ctx, &var_name, is_rune);
            let setter = build_binding_setter(ctx, var_name, is_rune);
            ctx.b.call_stmt(
                "$.bind_value",
                [Arg::Ident(el_name), Arg::Expr(getter), Arg::Expr(setter)],
            )
        }
        BindPropertyKind::Checked => {
            if is_prop_source {
                // Bindable props already lower to a prop accessor callable, so bind:checked
                // should pass it through directly instead of wrapping it in rune closures.
                ctx.b.call_stmt(
                    "$.bind_checked",
                    [Arg::Ident(el_name), Arg::Ident(&var_name)],
                )
            } else {
                let getter = build_binding_getter(ctx, &var_name, is_rune);
                let setter = build_binding_setter(ctx, var_name, is_rune);
                ctx.b.call_stmt(
                    "$.bind_checked",
                    [Arg::Ident(el_name), Arg::Expr(getter), Arg::Expr(setter)],
                )
            }
        }
        BindPropertyKind::Group => {
            ctx.state.needs_binding_group = true;

            let parent_eaches = ctx.parent_each_blocks(bind.id);
            let has_each_var = !parent_eaches.is_empty();

            // Getter and setter: when the expression references each-block vars,
            // use the pre-transformed expression (has $.get() applied).
            let (group_getter, setter) = if has_each_var {
                let bind_handle = bind
                    .expression_span
                    .and_then(|span| ctx.state.parsed.expr_handle(span.start));
                let getter_expr = bind_handle
                    .and_then(|handle| ctx.state.parsed.expr(handle))
                    .map(|expr| ctx.b.clone_expr(expr))
                    .unwrap_or_else(|| ctx.b.rid_expr(&var_name));
                let setter_expr = bind_handle
                    .and_then(|handle| ctx.state.parsed.expr(handle))
                    .map(|expr| ctx.b.clone_expr(expr))
                    .unwrap_or_else(|| ctx.b.rid_expr(&var_name));
                let getter = ctx
                    .b
                    .arrow_expr(ctx.b.no_params(), [ctx.b.expr_stmt(getter_expr)]);
                let setter = build_binding_setter_from_expr(ctx, setter_expr);
                (getter, setter)
            } else {
                let setter = build_binding_setter(ctx, var_name.clone(), is_rune);
                // Check if element has a dynamic value attribute — wrap getter in thunk
                let group_getter = if let Some(val_attr_id) = ctx.bind_group_value_attr(bind.id) {
                    let val_expr = ctx
                        .state
                        .parsed
                        .expr(ctx.attr_expr_handle(val_attr_id))
                        .map(|expr| ctx.b.clone_expr(expr))
                        .unwrap_or_else(|| ctx.b.str_expr(""));
                    let val_stmt = ctx.b.expr_stmt(val_expr);
                    let getter_expr = if is_rune {
                        ctx.b.call_expr("$.get", [Arg::Ident(&var_name)])
                    } else {
                        ctx.b.rid_expr(&var_name)
                    };
                    let return_stmt = ctx.b.return_stmt(getter_expr);
                    ctx.b
                        .arrow_block_expr(ctx.b.no_params(), vec![val_stmt, return_stmt])
                } else {
                    build_binding_getter(ctx, &var_name, is_rune)
                };
                (group_getter, setter)
            };

            // Build index array from ancestor each blocks whose vars appear in the expression
            let index_array = if !parent_eaches.is_empty() {
                let indexes: Vec<_> = parent_eaches
                    .iter()
                    .map(|&each_id| {
                        let idx_name = ctx
                            .each_index_name(each_id)
                            .or_else(|| ctx.group_index_names.get(&each_id).cloned())
                            .unwrap_or_else(|| {
                                panic!("missing index name for each block {:?}", each_id)
                            });
                        ctx.b.rid_expr(&idx_name)
                    })
                    .collect();
                ctx.b.array_expr(indexes)
            } else {
                ctx.b.empty_array_expr()
            };

            ctx.b.call_stmt(
                "$.bind_group",
                [
                    Arg::Ident("binding_group"),
                    Arg::Expr(index_array),
                    Arg::Ident(el_name),
                    Arg::Expr(group_getter),
                    Arg::Expr(setter),
                ],
            )
        }
        BindPropertyKind::Files => {
            let getter = build_binding_getter(ctx, &var_name, is_rune);
            let setter = build_binding_setter(ctx, var_name, is_rune);
            ctx.b.call_stmt(
                "$.bind_files",
                [Arg::Ident(el_name), Arg::Expr(getter), Arg::Expr(setter)],
            )
        }

        // --- Generic event-based bindings (bidirectional) ---
        BindPropertyKind::Indeterminate => {
            let setter = build_binding_setter(ctx, var_name.clone(), is_rune);
            let getter = build_binding_getter(ctx, &var_name, is_rune);
            gen_bind_property_stmt(
                ctx,
                "indeterminate",
                "change",
                el_name,
                setter,
                Some(getter),
            )
        }
        BindPropertyKind::Open => {
            let setter = build_binding_setter(ctx, var_name.clone(), is_rune);
            let getter = build_binding_getter(ctx, &var_name, is_rune);
            gen_bind_property_stmt(ctx, "open", "toggle", el_name, setter, Some(getter))
        }

        // --- Contenteditable ---
        BindPropertyKind::ContentEditable(kind) => {
            let getter = build_binding_getter(ctx, &var_name, is_rune);
            let setter = build_binding_setter(ctx, var_name, is_rune);
            ctx.b.call_stmt(
                "$.bind_content_editable",
                [
                    Arg::StrRef(kind.name()),
                    Arg::Ident(el_name),
                    Arg::Expr(getter),
                    Arg::Expr(setter),
                ],
            )
        }

        // --- Dimension bindings (element size, readonly) ---
        BindPropertyKind::ElementSize(kind) => {
            let setter = build_binding_setter(ctx, var_name, is_rune);
            ctx.b.call_stmt(
                "$.bind_element_size",
                [
                    Arg::Ident(el_name),
                    Arg::StrRef(kind.name()),
                    Arg::Expr(setter),
                ],
            )
        }

        // --- Dimension bindings (resize observer, readonly) ---
        BindPropertyKind::ResizeObserver(kind) => {
            let setter = build_binding_setter(ctx, var_name, is_rune);
            ctx.b.call_stmt(
                "$.bind_resize_observer",
                [
                    Arg::Ident(el_name),
                    Arg::StrRef(kind.name()),
                    Arg::Expr(setter),
                ],
            )
        }

        // --- Media R/W bindings ---
        BindPropertyKind::Media(MediaBindKind::CurrentTime) => {
            let getter = build_binding_getter(ctx, &var_name, is_rune);
            let setter = build_binding_setter(ctx, var_name, is_rune);
            ctx.b.call_stmt(
                "$.bind_current_time",
                [Arg::Ident(el_name), Arg::Expr(getter), Arg::Expr(setter)],
            )
        }
        BindPropertyKind::Media(MediaBindKind::PlaybackRate) => {
            let getter = build_binding_getter(ctx, &var_name, is_rune);
            let setter = build_binding_setter(ctx, var_name, is_rune);
            ctx.b.call_stmt(
                "$.bind_playback_rate",
                [Arg::Ident(el_name), Arg::Expr(getter), Arg::Expr(setter)],
            )
        }
        BindPropertyKind::Media(MediaBindKind::Paused) => {
            let getter = build_binding_getter(ctx, &var_name, is_rune);
            let setter = build_binding_setter(ctx, var_name, is_rune);
            ctx.b.call_stmt(
                "$.bind_paused",
                [Arg::Ident(el_name), Arg::Expr(getter), Arg::Expr(setter)],
            )
        }
        BindPropertyKind::Media(MediaBindKind::Volume) => {
            let getter = build_binding_getter(ctx, &var_name, is_rune);
            let setter = build_binding_setter(ctx, var_name, is_rune);
            ctx.b.call_stmt(
                "$.bind_volume",
                [Arg::Ident(el_name), Arg::Expr(getter), Arg::Expr(setter)],
            )
        }
        BindPropertyKind::Media(MediaBindKind::Muted) => {
            let getter = build_binding_getter(ctx, &var_name, is_rune);
            let setter = build_binding_setter(ctx, var_name, is_rune);
            ctx.b.call_stmt(
                "$.bind_muted",
                [Arg::Ident(el_name), Arg::Expr(getter), Arg::Expr(setter)],
            )
        }

        // --- Media RO bindings (setter only) ---
        BindPropertyKind::Media(MediaBindKind::Buffered) => {
            let setter = build_binding_setter(ctx, var_name, is_rune);
            ctx.b
                .call_stmt("$.bind_buffered", [Arg::Ident(el_name), Arg::Expr(setter)])
        }
        BindPropertyKind::Media(MediaBindKind::Seekable) => {
            let setter = build_binding_setter(ctx, var_name, is_rune);
            ctx.b
                .call_stmt("$.bind_seekable", [Arg::Ident(el_name), Arg::Expr(setter)])
        }
        BindPropertyKind::Media(MediaBindKind::Seeking) => {
            let setter = build_binding_setter(ctx, var_name, is_rune);
            ctx.b
                .call_stmt("$.bind_seeking", [Arg::Ident(el_name), Arg::Expr(setter)])
        }
        BindPropertyKind::Media(MediaBindKind::Ended) => {
            let setter = build_binding_setter(ctx, var_name, is_rune);
            ctx.b
                .call_stmt("$.bind_ended", [Arg::Ident(el_name), Arg::Expr(setter)])
        }
        BindPropertyKind::Media(MediaBindKind::ReadyState) => {
            let setter = build_binding_setter(ctx, var_name, is_rune);
            ctx.b.call_stmt(
                "$.bind_ready_state",
                [Arg::Ident(el_name), Arg::Expr(setter)],
            )
        }
        BindPropertyKind::Media(MediaBindKind::Played) => {
            let setter = build_binding_setter(ctx, var_name, is_rune);
            ctx.b
                .call_stmt("$.bind_played", [Arg::Ident(el_name), Arg::Expr(setter)])
        }

        // --- Media/Image RO event-based bindings (bind_property, no bidirectional) ---
        BindPropertyKind::Media(MediaBindKind::Duration) => {
            let setter = build_binding_setter(ctx, var_name, is_rune);
            gen_bind_property_stmt(ctx, "duration", "durationchange", el_name, setter, None)
        }
        BindPropertyKind::Media(MediaBindKind::VideoWidth) => {
            let setter = build_binding_setter(ctx, var_name, is_rune);
            gen_bind_property_stmt(ctx, "videoWidth", "resize", el_name, setter, None)
        }
        BindPropertyKind::Media(MediaBindKind::VideoHeight) => {
            let setter = build_binding_setter(ctx, var_name, is_rune);
            gen_bind_property_stmt(ctx, "videoHeight", "resize", el_name, setter, None)
        }
        BindPropertyKind::ImageNaturalSize(ImageNaturalSizeKind::NaturalWidth) => {
            let setter = build_binding_setter(ctx, var_name, is_rune);
            gen_bind_property_stmt(ctx, "naturalWidth", "load", el_name, setter, None)
        }
        BindPropertyKind::ImageNaturalSize(ImageNaturalSizeKind::NaturalHeight) => {
            let setter = build_binding_setter(ctx, var_name, is_rune);
            gen_bind_property_stmt(ctx, "naturalHeight", "load", el_name, setter, None)
        }

        // --- Misc ---
        BindPropertyKind::Focused => {
            let setter = build_binding_setter(ctx, var_name, is_rune);
            ctx.b
                .call_stmt("$.bind_focused", [Arg::Ident(el_name), Arg::Expr(setter)])
        }

        // --- bind:this ---
        BindPropertyKind::This => {
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
                        let body_stmts: Vec<_> =
                            arrow.body.unbox().statements.into_iter().collect();
                        ctx.b.arrow_expr(ctx.b.no_params(), body_stmts)
                    } else {
                        get_expr
                    };

                    // Unwrap setter arrow: keep first param or add `_`, keep body
                    let set_arrow = if let Expression::ArrowFunctionExpression(arrow) = set_expr {
                        let arrow = arrow.unbox();
                        let body_stmts: Vec<_> =
                            arrow.body.unbox().statements.into_iter().collect();
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

                    let stmt = ctx.b.call_stmt(
                        "$.bind_this",
                        [
                            Arg::Ident(el_name),
                            Arg::Expr(set_arrow),
                            Arg::Expr(get_arrow),
                        ],
                    );
                    return Some(BindPlacement::Init(stmt));
                }
                // Not a SequenceExpression — fall through to normal bind:this below,
                // but we already consumed the expression. Re-parse it.
                let _ = span; // expression was consumed, re-get below
            }

            // svelte:element uses proxy flag because the element type is unknown at compile time
            let setter = if tag_name.is_empty() && is_rune {
                let body = ctx.b.call_expr(
                    "$.set",
                    [
                        Arg::Ident(&var_name),
                        Arg::Ident("$$value"),
                        Arg::Expr(ctx.b.bool_expr(true)),
                    ],
                );
                ctx.b
                    .arrow_expr(ctx.b.params(["$$value"]), [ctx.b.expr_stmt(body)])
            } else {
                build_binding_setter(ctx, var_name.clone(), is_rune)
            };
            let getter = build_binding_getter(ctx, &var_name, is_rune);
            let stmt = ctx.b.call_stmt(
                "$.bind_this",
                [Arg::Ident(el_name), Arg::Expr(setter), Arg::Expr(getter)],
            );
            return Some(BindPlacement::Init(stmt));
        }

        BindPropertyKind::Window(_)
        | BindPropertyKind::Document(_)
        | BindPropertyKind::ComponentProp => unreachable!("unexpected bind property for element"),
    };

    // Wrap in $.effect() when element has use: directive (deferral)
    let mut stmt = stmt;
    if has_use_directive && !semantics.is_this() {
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
fn gen_run_after_blockers<'a>(
    ctx: &mut Ctx<'a>,
    stmt: Statement<'a>,
    blockers: &[u32],
) -> Statement<'a> {
    let blockers_arr = ctx.b.promises_array(blockers);
    let thunk = ctx.b.thunk_block(vec![stmt]);
    ctx.b.call_stmt(
        "$.run_after_blockers",
        [Arg::Expr(blockers_arr), Arg::Expr(thunk)],
    )
}
