//! Bind directive codegen (`bind:value`, `bind:checked`, `bind:group`, etc.).

use oxc_ast::ast::{Expression, Statement};

use svelte_analyze::{BindPropertyKind, BindTargetSemantics, ImageNaturalSizeKind, MediaBindKind};
use svelte_ast::NodeId;

use crate::context::Ctx;
use svelte_ast_builder::{Arg, AssignLeft};

use super::expression::get_attr_expr;

/// Unpack the `SequenceExpression(getter, setter)` that the transform phase
/// builds for every non-`bind:this` / non-user-sequence bind directive.
///
/// Returns `None` when the expression isn't a transform-built sequence —
/// currently only `bind:this` (handled via dedicated codegen) and
/// user-authored `bind:prop={(get), (set)}` sequences.
pub(crate) fn take_bind_getter_setter<'a>(
    ctx: &mut Ctx<'a>,
    bind_id: NodeId,
) -> Option<(Expression<'a>, Expression<'a>)> {
    let expr = get_attr_expr(ctx, bind_id);
    let Expression::SequenceExpression(seq) = expr else {
        return None;
    };
    let seq = seq.unbox();
    let mut exprs = seq.expressions.into_iter();
    let get = exprs
        .next()
        .expect("bind SequenceExpression missing getter");
    let set = exprs
        .next()
        .expect("bind SequenceExpression missing setter");
    Some((get, set))
}

/// Build binding getter: `() => $.get(x)` for runes, `() => x` for plain vars.
///
/// Used by callers that bypass the transform-built `SequenceExpression`
/// pair: `bind:this`, window/document bindings, and the component-prop
/// parent fallback. For all other bind sites the getter comes from the
/// transform phase directly.
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

/// Build a `($$value) => $.set(x, $$value, true)` setter arrow with the
/// silent (reactivity-feedback-break) flag. Used by window/document
/// bindings and `bind:this` on `svelte:element`.
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
    semantics: BindTargetSemantics,
    el_name: &str,
    tag_name: &str,
    has_use_directive: bool,
) -> Option<BindPlacement<'a>> {
    let bind_property = semantics.property();
    let bind_blockers = ctx.bind_blockers(bind.id).to_vec();

    // Every non-`bind:this` / non-user-authored-sequence bind gets its
    // getter/setter pair built up-front by the transform phase. The shape is
    // `SequenceExpression(() => <read>, ($$value) => <write>)` so codegen only
    // needs to pass the pair through to the correct runtime helper.
    //
    // `bind:this` keeps its dedicated handler below because the getter/setter
    // form depends on element identity rather than the expression's read/write
    // shape.
    // Bindable prop sources are emitted as `$.bind_*(el, <accessor>)` — the
    // compiled prop accessor already handles both read and write, so wrapping
    // it in getter/setter closures would double-indirect. The transform pass
    // leaves these expressions alone (see `walk_attrs` in svelte_transform);
    // here we detect the same shape and emit the short form directly.
    if !semantics.is_this() && matches!(bind_property, BindPropertyKind::Checked) {
        use svelte_analyze::{PropReferenceSemantics, ReferenceSemantics};
        let is_bindable_prop_source = matches!(
            ctx.directive_root_reference_semantics(bind.id),
            ReferenceSemantics::PropRead(PropReferenceSemantics::Source { bindable: true, .. })
                | ReferenceSemantics::PropMutation { bindable: true, .. }
        );
        if is_bindable_prop_source {
            let var_name = if bind.shorthand {
                bind.name.clone()
            } else {
                ctx.query
                    .component
                    .source_text(bind.expression_span)
                    .to_string()
            };
            // Consume the parsed expression so it isn't left dangling —
            // codegen uses the identifier name directly.
            if let Some(handle) = ctx.state.parsed.expr_handle(bind.expression_span.start) {
                let _ = ctx.state.parsed.take_expr(handle);
            }
            let var_alloc = ctx.b.alloc_str(&var_name);
            let stmt = ctx.b.call_stmt(
                "$.bind_checked",
                [Arg::Ident(el_name), Arg::Ident(var_alloc)],
            );
            let mut stmt = stmt;
            if has_use_directive {
                let effect_body = ctx.b.arrow_expr(ctx.b.no_params(), [stmt]);
                stmt = ctx.b.call_stmt("$.effect", [Arg::Expr(effect_body)]);
                if !bind_blockers.is_empty() {
                    stmt = gen_run_after_blockers(ctx, stmt, &bind_blockers);
                }
                return Some(BindPlacement::Init(stmt));
            }
            if !bind_blockers.is_empty() {
                stmt = gen_run_after_blockers(ctx, stmt, &bind_blockers);
            }
            return Some(BindPlacement::AfterUpdate(stmt));
        }
    }

    if !semantics.is_this() {
        let stmt_opt =
            take_bind_getter_setter(ctx, bind.id).map(|(get_fn, set_fn)| match bind_property {
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
                BindPropertyKind::ElementSize(kind) => ctx.b.call_stmt(
                    "$.bind_element_size",
                    [
                        Arg::Ident(el_name),
                        Arg::StrRef(kind.name()),
                        Arg::Expr(set_fn),
                    ],
                ),
                BindPropertyKind::ResizeObserver(kind) => ctx.b.call_stmt(
                    "$.bind_resize_observer",
                    [
                        Arg::Ident(el_name),
                        Arg::StrRef(kind.name()),
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
                // Readonly bindings: only the setter is meaningful; the getter is
                // discarded (runtime doesn't read it back).
                BindPropertyKind::Media(MediaBindKind::Buffered) => ctx
                    .b
                    .call_stmt("$.bind_buffered", [Arg::Ident(el_name), Arg::Expr(set_fn)]),
                BindPropertyKind::Media(MediaBindKind::Seekable) => ctx
                    .b
                    .call_stmt("$.bind_seekable", [Arg::Ident(el_name), Arg::Expr(set_fn)]),
                BindPropertyKind::Media(MediaBindKind::Seeking) => ctx
                    .b
                    .call_stmt("$.bind_seeking", [Arg::Ident(el_name), Arg::Expr(set_fn)]),
                BindPropertyKind::Media(MediaBindKind::Ended) => ctx
                    .b
                    .call_stmt("$.bind_ended", [Arg::Ident(el_name), Arg::Expr(set_fn)]),
                BindPropertyKind::Media(MediaBindKind::ReadyState) => ctx.b.call_stmt(
                    "$.bind_ready_state",
                    [Arg::Ident(el_name), Arg::Expr(set_fn)],
                ),
                BindPropertyKind::Media(MediaBindKind::Played) => ctx
                    .b
                    .call_stmt("$.bind_played", [Arg::Ident(el_name), Arg::Expr(set_fn)]),
                BindPropertyKind::Media(MediaBindKind::Duration) => {
                    gen_bind_property_stmt(ctx, "duration", "durationchange", el_name, set_fn, None)
                }
                BindPropertyKind::Media(MediaBindKind::VideoWidth) => {
                    gen_bind_property_stmt(ctx, "videoWidth", "resize", el_name, set_fn, None)
                }
                BindPropertyKind::Media(MediaBindKind::VideoHeight) => {
                    gen_bind_property_stmt(ctx, "videoHeight", "resize", el_name, set_fn, None)
                }
                BindPropertyKind::ImageNaturalSize(ImageNaturalSizeKind::NaturalWidth) => {
                    gen_bind_property_stmt(ctx, "naturalWidth", "load", el_name, set_fn, None)
                }
                BindPropertyKind::ImageNaturalSize(ImageNaturalSizeKind::NaturalHeight) => {
                    gen_bind_property_stmt(ctx, "naturalHeight", "load", el_name, set_fn, None)
                }
                BindPropertyKind::Focused => ctx
                    .b
                    .call_stmt("$.bind_focused", [Arg::Ident(el_name), Arg::Expr(set_fn)]),
                BindPropertyKind::Group => {
                    // Group needs index array and shared binding_group; getter/setter are already
                    // the transform-built pair from the primary expression.
                    ctx.state.needs_binding_group = true;
                    let parent_eaches = ctx.parent_each_blocks(bind.id);
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
                    // When the parent element also has a dynamic `value` attribute, the
                    // runtime needs to re-read it before computing the getter so the group
                    // index stays in sync. Wrap the transform-built getter so the
                    // `value` expression runs first as a side effect.
                    let getter = if let Some(val_attr_id) = ctx.bind_group_value_attr(bind.id) {
                        let val_expr = ctx
                            .state
                            .parsed
                            .expr(ctx.attr_expr_handle(val_attr_id))
                            .map(|expr| ctx.b.clone_expr(expr))
                            .unwrap_or_else(|| ctx.b.str_expr(""));
                        let val_stmt = ctx.b.expr_stmt(val_expr);
                        // Unwrap the arrow body so we can re-wrap it with the side-effect
                        // statement in front.
                        let body_expr = match get_fn {
                            Expression::ArrowFunctionExpression(arrow) => {
                                let arrow = arrow.unbox();
                                let mut stmts = arrow.body.unbox().statements.into_iter();
                                let first = stmts.next().expect("getter arrow missing body");
                                match first {
                                    oxc_ast::ast::Statement::ExpressionStatement(es) => {
                                        es.unbox().expression
                                    }
                                    _ => panic!("getter arrow body is not an ExpressionStatement"),
                                }
                            }
                            _ => panic!("bind:group getter is not an ArrowFunction"),
                        };
                        let return_stmt = ctx.b.return_stmt(body_expr);
                        ctx.b
                            .arrow_block_expr(ctx.b.no_params(), vec![val_stmt, return_stmt])
                    } else {
                        get_fn
                    };
                    ctx.b.call_stmt(
                        "$.bind_group",
                        [
                            Arg::Ident("binding_group"),
                            Arg::Expr(index_array),
                            Arg::Ident(el_name),
                            Arg::Expr(getter),
                            Arg::Expr(set_fn),
                        ],
                    )
                }
                BindPropertyKind::This
                | BindPropertyKind::Window(_)
                | BindPropertyKind::Document(_)
                | BindPropertyKind::ComponentProp => {
                    unreachable!("unexpected bind property routed through getter/setter path")
                }
            });

        if let Some(stmt) = stmt_opt {
            let mut stmt = stmt;
            if has_use_directive {
                let effect_body = ctx.b.arrow_expr(ctx.b.no_params(), [stmt]);
                stmt = ctx.b.call_stmt("$.effect", [Arg::Expr(effect_body)]);

                if !bind_blockers.is_empty() {
                    stmt = gen_run_after_blockers(ctx, stmt, &bind_blockers);
                }
                return Some(BindPlacement::Init(stmt));
            }
            if !bind_blockers.is_empty() {
                stmt = gen_run_after_blockers(ctx, stmt, &bind_blockers);
            }
            return Some(BindPlacement::AfterUpdate(stmt));
        }
        // No SequenceExpression available: user wrote `bind:prop={(g), (s)}`
        // literally. Fall through to the old path below.
    }

    let _ = bind_blockers; // blockers don't apply to bind:this below.

    // bind:this — expression is never routed through the transform phase
    // (see walk_attrs in svelte_transform), so identifiers retain their
    // original `reference_id` and the root-identifier semantics query is
    // reliable here.
    debug_assert!(matches!(bind_property, BindPropertyKind::This));
    let is_rune = matches!(
        ctx.directive_root_reference_semantics(bind.id),
        svelte_analyze::ReferenceSemantics::SignalWrite { .. }
            | svelte_analyze::ReferenceSemantics::SignalUpdate { .. }
            | svelte_analyze::ReferenceSemantics::SignalRead { .. }
    );
    let var_name = if bind.shorthand {
        bind.name.clone()
    } else {
        ctx.query
            .component
            .source_text(bind.expression_span)
            .to_string()
    };

    // User-authored sequence form: `bind:this={(get), (set)}`.
    if !bind.shorthand {
        let expr = get_attr_expr(ctx, bind.id);
        if let Expression::SequenceExpression(seq) = expr {
            let seq = seq.unbox();
            let mut exprs = seq.expressions.into_iter();
            let get_expr = exprs.next().expect("SequenceExpression must have getter");
            let set_expr = exprs.next().expect("SequenceExpression must have setter");

            let get_arrow = if let Expression::ArrowFunctionExpression(arrow) = get_expr {
                let arrow = arrow.unbox();
                let body_stmts: Vec<_> = arrow.body.unbox().statements.into_iter().collect();
                ctx.b.arrow_expr(ctx.b.no_params(), body_stmts)
            } else {
                get_expr
            };

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
    }

    // Plain identifier / member form: build getter/setter locally.
    // `svelte:element` passes `true` to `$.set` to flag the element type as
    // unknown at compile time (proxy-bind path).
    let setter_body = if is_rune {
        let mut args: Vec<Arg<'_, '_>> = vec![Arg::Ident(&var_name), Arg::Ident("$$value")];
        if tag_name.is_empty() {
            args.push(Arg::Expr(ctx.b.bool_expr(true)));
        }
        ctx.b.call_expr("$.set", args)
    } else {
        ctx.b.assign_expr(
            AssignLeft::Ident(var_name.clone()),
            ctx.b.rid_expr("$$value"),
        )
    };
    let setter = ctx
        .b
        .arrow_expr(ctx.b.params(["$$value"]), [ctx.b.expr_stmt(setter_body)]);
    let getter_body = if is_rune {
        ctx.b.call_expr("$.get", [Arg::Ident(&var_name)])
    } else {
        ctx.b.rid_expr(&var_name)
    };
    let getter = ctx
        .b
        .arrow_expr(ctx.b.no_params(), [ctx.b.expr_stmt(getter_body)]);
    let stmt = ctx.b.call_stmt(
        "$.bind_this",
        [Arg::Ident(el_name), Arg::Expr(setter), Arg::Expr(getter)],
    );
    Some(BindPlacement::Init(stmt))
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
