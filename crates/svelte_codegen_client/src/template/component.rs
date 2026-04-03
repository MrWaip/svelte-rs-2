//! Component instantiation codegen.

use oxc_ast::ast::{Expression, Statement};

use svelte_analyze::{ComponentBindMode, ComponentPropKind, ContentStrategy, FragmentKey};
use svelte_ast::{Attribute, NodeId};

use crate::builder::{Arg, AssignLeft, ObjProp};
use crate::context::Ctx;

use super::expression::{build_attr_concat, get_attr_expr};
use super::gen_fragment;
use super::snippet;

/// Generate `ComponentName($$anchor, { props })` call.
pub(crate) fn gen_component<'a>(
    ctx: &mut Ctx<'a>,
    id: NodeId,
    anchor: Expression<'a>,
    init: &mut Vec<Statement<'a>>,
) {
    let cn = ctx.component_node(id);
    let name: &str = &cn.name;

    // Snapshot pre-classified props to release immutable borrow before mutable ctx usage
    let prop_infos: Vec<_> = ctx
        .component_props(id)
        .iter()
        .map(|p| (p.kind.clone(), p.is_dynamic))
        .collect();

    // Items preserve attribute order for correct $.spread_props grouping
    let mut items: Vec<PropOrSpread<'a>> = Vec::new();
    let mut bind_this_info: Option<NodeId> = None;
    let mut memo_counter: u32 = 0;
    let mut memo_stmts: Vec<Statement<'a>> = Vec::new();
    // LEGACY(svelte4): on:directive events → $$events prop
    let mut events: Vec<(String, NodeId, bool, bool)> = Vec::new(); // (name, attr_id, has_expr, once)

    for (kind, is_dynamic) in prop_infos {
        match kind {
            ComponentPropKind::String { name, value_span } => {
                let value_text = ctx.query.component.source_text(value_span);
                let key = ctx.b.alloc_str(&name);
                items.push(PropOrSpread::Prop(ObjProp::KeyValue(
                    key,
                    ctx.b.str_expr(value_text),
                )));
            }
            ComponentPropKind::Boolean { name } => {
                let key = ctx.b.alloc_str(&name);
                items.push(PropOrSpread::Prop(ObjProp::KeyValue(
                    key,
                    ctx.b.bool_expr(true),
                )));
            }
            ComponentPropKind::Expression {
                name,
                attr_id,
                shorthand,
                needs_memo,
            } => {
                let key = ctx.b.alloc_str(&name);
                if needs_memo {
                    let mut memo_name = String::with_capacity(4);
                    memo_name.push('$');
                    memo_name.push_str(&memo_counter.to_string());
                    memo_counter += 1;
                    let expr = get_attr_expr(ctx, attr_id);
                    let thunk = ctx.b.thunk(expr);
                    let derived = ctx.b.call_expr("$.derived", [Arg::Expr(thunk)]);
                    memo_stmts.push(ctx.b.let_init_stmt(&memo_name, derived));
                    let memo_ref = ctx.b.alloc_str(&memo_name);
                    let get = ctx.b.call_expr("$.get", [Arg::Ident(memo_ref)]);
                    items.push(PropOrSpread::Prop(ObjProp::Getter(key, get)));
                } else {
                    let expr = get_attr_expr(ctx, attr_id);
                    if is_dynamic {
                        items.push(PropOrSpread::Prop(ObjProp::Getter(key, expr)));
                    } else if shorthand {
                        items.push(PropOrSpread::Prop(ObjProp::Shorthand(key)));
                    } else {
                        items.push(PropOrSpread::Prop(ObjProp::KeyValue(key, expr)));
                    }
                }
            }
            ComponentPropKind::Concatenation {
                name,
                attr_id,
                parts,
            } => {
                let key = ctx.b.alloc_str(&name);
                let val = build_attr_concat(ctx, attr_id, &parts);
                if is_dynamic {
                    items.push(PropOrSpread::Prop(ObjProp::Getter(key, val)));
                } else {
                    items.push(PropOrSpread::Prop(ObjProp::KeyValue(key, val)));
                }
            }
            ComponentPropKind::Shorthand { attr_id, name } => {
                let key = ctx.b.alloc_str(&name);
                let expr = get_attr_expr(ctx, attr_id);
                if is_dynamic {
                    items.push(PropOrSpread::Prop(ObjProp::Getter(key, expr)));
                } else {
                    items.push(PropOrSpread::Prop(ObjProp::Shorthand(key)));
                }
            }
            ComponentPropKind::Bind {
                name,
                bind_id: _,
                mode,
                ref expr_name,
            } => {
                let key = ctx.b.alloc_str(&name);
                let name_ref = ctx.b.alloc_str(&name);
                match mode {
                    ComponentBindMode::PropSource => {
                        let get_body = ctx.b.call_expr(name_ref, []);
                        items.push(PropOrSpread::Prop(ObjProp::Getter(key, get_body)));
                        let set_body = ctx.b.call_expr(name_ref, [Arg::Ident("$$value")]);
                        items.push(PropOrSpread::Prop(ObjProp::Setter(
                            key,
                            "$$value",
                            None,
                            vec![ctx.b.expr_stmt(set_body)],
                        )));
                    }
                    ComponentBindMode::Rune => {
                        let get_body = ctx.b.call_expr("$.get", [Arg::Ident(name_ref)]);
                        items.push(PropOrSpread::Prop(ObjProp::Getter(key, get_body)));
                        let set_body = ctx
                            .b
                            .call_expr("$.set", [Arg::Ident(name_ref), Arg::Ident("$$value")]);
                        items.push(PropOrSpread::Prop(ObjProp::Setter(
                            key,
                            "$$value",
                            None,
                            vec![ctx.b.expr_stmt(set_body)],
                        )));
                    }
                    ComponentBindMode::Plain => {
                        let get_body = ctx.b.rid_expr(name_ref);
                        items.push(PropOrSpread::Prop(ObjProp::Getter(key, get_body)));
                        let set_body = ctx
                            .b
                            .assign_expr(AssignLeft::Ident(name), ctx.b.rid_expr("$$value"));
                        items.push(PropOrSpread::Prop(ObjProp::Setter(
                            key,
                            "$$value",
                            None,
                            vec![ctx.b.expr_stmt(set_body)],
                        )));
                    }
                    ComponentBindMode::StoreSub => {
                        let store_ref = expr_name
                            .as_deref()
                            .unwrap_or_else(|| panic!("StoreSub bind must have expr_name"));
                        let store_id = ctx.b.alloc_str(store_ref);
                        // get value() { $.mark_store_binding(); return $count(); }
                        let mark_stmt =
                            ctx.b.expr_stmt(ctx.b.call_expr("$.mark_store_binding", []));
                        let return_expr = ctx.b.call_expr(store_id, []);
                        let return_stmt = ctx.b.return_stmt(return_expr);
                        items.push(PropOrSpread::Prop(ObjProp::GetterBody(
                            key,
                            vec![mark_stmt, return_stmt],
                        )));
                        // set value($$value) { $.store_set(count, $$value); }
                        let base_name = &store_ref[1..]; // strip '$' prefix
                        let base_id: &str = ctx.b.alloc_str(base_name);
                        let set_body = ctx
                            .b
                            .call_expr("$.store_set", [Arg::Ident(base_id), Arg::Ident("$$value")]);
                        items.push(PropOrSpread::Prop(ObjProp::Setter(
                            key,
                            "$$value",
                            None,
                            vec![ctx.b.expr_stmt(set_body)],
                        )));
                    }
                }
            }
            ComponentPropKind::BindThis { bind_id } => {
                bind_this_info = Some(bind_id);
            }
            ComponentPropKind::Attach { attr_id } => {
                let key_expr = ctx.b.call_expr("$.attachment", []);
                let expr = get_attr_expr(ctx, attr_id);
                if is_dynamic {
                    // Lazy evaluation so the runtime can track reactive dependencies
                    let call = ctx.b.call_expr_callee(expr, [Arg::Ident("$$node")]);
                    let wrapper = ctx
                        .b
                        .arrow_expr(ctx.b.params(["$$node"]), [ctx.b.expr_stmt(call)]);
                    items.push(PropOrSpread::Prop(ObjProp::Computed(key_expr, wrapper)));
                } else {
                    items.push(PropOrSpread::Prop(ObjProp::Computed(key_expr, expr)));
                }
            }
            ComponentPropKind::Spread { attr_id } => {
                let expr = get_attr_expr(ctx, attr_id);
                let spread_expr = if is_dynamic { ctx.b.thunk(expr) } else { expr };
                items.push(PropOrSpread::Spread(spread_expr));
            }
            ComponentPropKind::Event {
                name,
                attr_id,
                has_expression,
                has_once_modifier,
            } => {
                events.push((name, attr_id, has_expression, has_once_modifier));
                continue;
            }
        }
    }

    // LEGACY(svelte4): emit $$events prop from collected on: directives
    if !events.is_empty() {
        let event_props: Vec<ObjProp<'a>> = events
            .into_iter()
            .filter_map(|(name, attr_id, has_expression, has_once_modifier)| {
                let key = ctx.b.alloc_str(&name);
                if !has_expression {
                    // Bubble event (no handler expression) — skip, not supported on components
                    return None;
                }
                let is_shorthand = ctx.is_expression_shorthand(attr_id);
                if is_shorthand && !has_once_modifier {
                    // Consume the parsed expression to keep side table clean
                    let _ = get_attr_expr(ctx, attr_id);
                    Some(ObjProp::Shorthand(key))
                } else {
                    let handler = get_attr_expr(ctx, attr_id);
                    let handler = if has_once_modifier {
                        ctx.b.call_expr("$.once", [Arg::Expr(handler)])
                    } else {
                        handler
                    };
                    Some(ObjProp::KeyValue(key, handler))
                }
            })
            .collect();
        items.push(PropOrSpread::Prop(ObjProp::KeyValue(
            "$$events",
            ctx.b.object_expr(event_props),
        )));
    }

    // Named snippets declared inside this component's fragment
    let snippet_ids: Vec<NodeId> = ctx.component_snippets(id).to_vec();

    let mut snippet_decls: Vec<Statement<'a>> = Vec::new();
    let mut slot_entries: Vec<ObjProp<'a>> = Vec::new();
    for snippet_id in &snippet_ids {
        let snippet_name = ctx
            .snippet_block(*snippet_id)
            .name(ctx.state.source)
            .to_string();
        snippet_decls.push(snippet::gen_snippet_block(ctx, *snippet_id, vec![]));
        let key = ctx.b.alloc_str(&snippet_name);
        items.push(PropOrSpread::Prop(ObjProp::Shorthand(key)));
        let slot_key = if snippet_name == "children" {
            "default"
        } else {
            ctx.b.alloc_str(&snippet_name)
        };
        slot_entries.push(ObjProp::KeyValue(slot_key, ctx.b.bool_expr(true)));
    }

    // Add children prop if component has non-snippet content
    let children_ct = ctx.content_type(&FragmentKey::ComponentNode(id));

    if children_ct != ContentStrategy::Empty {
        let frag_key = FragmentKey::ComponentNode(id);

        // For text-first content (Static/DynamicText), gen_fragment doesn't emit $.next(),
        // so the component handler must. For Dynamic (mixed), gen_fragment handles it.
        let needs_next = matches!(
            children_ct,
            ContentStrategy::Static(_) | ContentStrategy::DynamicText
        );

        let mut body_stmts = Vec::new();
        if needs_next {
            body_stmts.push(ctx.b.call_stmt("$.next", []));
        }
        body_stmts.extend(gen_fragment(ctx, frag_key));

        let params = ctx.b.params(["$$anchor", "$$slotProps"]);
        let arrow = ctx.b.arrow_expr(params, body_stmts);
        items.push(PropOrSpread::Prop(ObjProp::KeyValue("children", arrow)));
        slot_entries.push(ObjProp::KeyValue("default", ctx.b.bool_expr(true)));
    }

    // Named slots: children with slot="name" attribute
    let named_slots: Vec<_> = ctx.component_named_slots(id).to_vec();
    for (slot_el_id, frag_key) in named_slots {
        let slot_ct = ctx.content_type(&frag_key);
        if slot_ct == ContentStrategy::Empty {
            continue;
        }

        // Recover slot name from the element's slot="..." attribute
        let slot_name = slot_name_from_element(ctx, slot_el_id);

        let needs_next = matches!(
            slot_ct,
            ContentStrategy::Static(_) | ContentStrategy::DynamicText
        );
        let mut slot_body = Vec::new();
        if needs_next {
            slot_body.push(ctx.b.call_stmt("$.next", []));
        }
        slot_body.extend(gen_fragment(ctx, frag_key));

        let params = ctx.b.params(["$$anchor", "$$slotProps"]);
        let arrow = ctx.b.arrow_expr(params, slot_body);
        let key = ctx.b.alloc_str(&slot_name);
        slot_entries.push(ObjProp::KeyValue(key, arrow));
    }

    if !slot_entries.is_empty() {
        items.push(PropOrSpread::Prop(ObjProp::KeyValue(
            "$$slots",
            ctx.b.object_expr(slot_entries),
        )));
    }

    let props_expr = build_props_expr(ctx, items);
    let is_dynamic = ctx.is_dynamic_component(id);

    if is_dynamic {
        // $.component(anchor, () => registry.Widget, ($$anchor, registry_Widget) => { ... })
        let intermediate = name.replace('.', "_");
        let intermediate_ref = ctx.b.alloc_str(&intermediate);

        // Inner call: registry_Widget($$anchor, props)
        let inner_call = ctx.b.call_expr(
            intermediate_ref,
            [Arg::Ident("$$anchor"), Arg::Expr(props_expr)],
        );

        let inner_final = if let Some(bind_id) = bind_this_info {
            build_bind_this_call(ctx, id, bind_id, inner_call)
        } else {
            inner_call
        };

        let mut inner_body = Vec::new();
        inner_body.extend(memo_stmts);
        inner_body.push(ctx.b.expr_stmt(inner_final));

        let inner_arrow = ctx
            .b
            .arrow_block_expr(ctx.b.params(["$$anchor", intermediate_ref]), inner_body);

        // Thunk: () => registry.Widget — build as member expression chain
        let component_ref = build_dotted_member_expr(ctx, name);
        let component_thunk = ctx.b.thunk(component_ref);

        let component_call = ctx.b.call_expr(
            "$.component",
            [
                Arg::Expr(anchor),
                Arg::Expr(component_thunk),
                Arg::Expr(inner_arrow),
            ],
        );

        let has_snippets = !snippet_decls.is_empty();
        if has_snippets {
            snippet_decls.push(ctx.b.expr_stmt(component_call));
            init.push(ctx.b.block_stmt(snippet_decls));
        } else {
            init.push(ctx.b.expr_stmt(component_call));
        }
    } else {
        let component_call = ctx
            .b
            .call_expr(name, [Arg::Expr(anchor), Arg::Expr(props_expr)]);

        let final_expr = if let Some(bind_id) = bind_this_info {
            build_bind_this_call(ctx, id, bind_id, component_call)
        } else {
            component_call
        };

        let has_snippets = !snippet_decls.is_empty();
        if has_snippets {
            snippet_decls.extend(memo_stmts);
            snippet_decls.push(ctx.b.expr_stmt(final_expr));
            init.push(ctx.b.block_stmt(snippet_decls));
        } else if memo_stmts.is_empty() {
            init.push(ctx.b.expr_stmt(final_expr));
        } else {
            memo_stmts.push(ctx.b.expr_stmt(final_expr));
            init.push(ctx.b.block_stmt(memo_stmts));
        }
    }
}

/// Build `$.bind_this(value, setter, getter[, context_thunk])` for component bind:this.
///
/// Looks up the BindDirective from the component AST to extract expression text.
/// For identifiers: builds setter/getter using rune-aware $.set/$.get or plain assignment.
/// For member expressions: re-parses from source because bind:this setter/getter params
/// shadow each-block context vars — `($$value, i) => refs[i]` needs raw `i`, not `$.get(i)`.
fn build_bind_this_call<'a>(
    ctx: &mut Ctx<'a>,
    component_id: NodeId,
    bind_id: NodeId,
    value: Expression<'a>,
) -> Expression<'a> {
    // Consume the transformed attr expr to keep the side table clean
    let cn_tmp = ctx.component_node(component_id);
    let bind_tmp = cn_tmp.attributes.iter().find_map(|a| {
        if let Attribute::BindDirective(b) = a {
            if b.id == bind_id {
                Some(b)
            } else {
                None
            }
        } else {
            None
        }
    });
    if let Some(b) = bind_tmp {
        if let Some(span) = b.expression_span {
            if let Some(handle) = ctx.state.parsed.expr_handle(span.start) {
                let _ = ctx.state.parsed.take_expr(handle);
            }
        }
    }

    // Look up the bind directive from the component AST to get expression text
    let cn = ctx.component_node(component_id);
    let bind = cn
        .attributes
        .iter()
        .find_map(|a| {
            if let Attribute::BindDirective(b) = a {
                if b.id == bind_id {
                    Some(b)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .expect("bind:this attribute must exist");

    let var_name = if bind.shorthand {
        bind.name.clone()
    } else if let Some(span) = bind.expression_span {
        ctx.query.component.source_text(span).to_string()
    } else {
        // No expression — emit bare component call
        return value;
    };

    if svelte_analyze::is_simple_identifier(&var_name) {
        let is_rune = ctx.is_mutable_rune_target(bind_id);
        let expr_text = ctx.b.alloc_str(&var_name);

        let setter = if is_rune {
            let body = ctx.b.call_expr(
                "$.set",
                [
                    Arg::Ident(expr_text),
                    Arg::Ident("$$value"),
                    Arg::Expr(ctx.b.bool_expr(true)),
                ],
            );
            ctx.b
                .arrow_expr(ctx.b.params(["$$value"]), [ctx.b.expr_stmt(body)])
        } else {
            let body = ctx
                .b
                .assign_expr(AssignLeft::Ident(var_name), ctx.b.rid_expr("$$value"));
            ctx.b
                .arrow_expr(ctx.b.params(["$$value"]), [ctx.b.expr_stmt(body)])
        };

        let getter = if is_rune {
            let body = ctx.b.call_expr("$.get", [Arg::Ident(expr_text)]);
            ctx.b.arrow_expr(ctx.b.no_params(), [ctx.b.expr_stmt(body)])
        } else {
            let body = ctx.b.rid_expr(expr_text);
            ctx.b.arrow_expr(ctx.b.no_params(), [ctx.b.expr_stmt(body)])
        };

        ctx.b.call_expr(
            "$.bind_this",
            [Arg::Expr(value), Arg::Expr(setter), Arg::Expr(getter)],
        )
    } else {
        // Member/computed expression: re-parse from source to avoid reactive wrapping
        let each_context: Vec<String> = ctx.bind_each_context(bind_id).cloned().unwrap_or_default();

        // Setter: ($$value[, ctx_vars]) => <expr> = $$value
        let setter_body = format!("{var_name} = $$value");
        let setter_expr = ctx.b.parse_expression(&setter_body);
        let mut setter_params: Vec<&str> = vec!["$$value"];
        for v in &each_context {
            setter_params.push(v);
        }
        let setter = ctx
            .b
            .arrow_expr(ctx.b.params(setter_params), [ctx.b.expr_stmt(setter_expr)]);

        // Getter: ([ctx_vars]) => <expr_with_optional_chaining>
        let getter_expr = ctx.b.parse_expression(&var_name);
        let getter_expr = ctx.b.make_optional_chain(getter_expr);
        let mut getter_params: Vec<&str> = Vec::new();
        for v in &each_context {
            getter_params.push(v);
        }
        let getter = ctx
            .b
            .arrow_expr(ctx.b.params(getter_params), [ctx.b.expr_stmt(getter_expr)]);

        if each_context.is_empty() {
            ctx.b.call_expr(
                "$.bind_this",
                [Arg::Expr(value), Arg::Expr(setter), Arg::Expr(getter)],
            )
        } else {
            // 4th arg: () => [ctx_var1, ctx_var2, ...]
            let context_values: Vec<Arg<'_, '_>> = each_context
                .iter()
                .map(|v| {
                    let s = ctx.b.alloc_str(v);
                    Arg::Ident(s)
                })
                .collect();
            let context_array = ctx.b.array_from_args(context_values);
            let context_thunk = ctx.b.thunk(context_array);

            ctx.b.call_expr(
                "$.bind_this",
                [
                    Arg::Expr(value),
                    Arg::Expr(setter),
                    Arg::Expr(getter),
                    Arg::Expr(context_thunk),
                ],
            )
        }
    }
}

/// Prop or spread item — preserves attribute order for correct $.spread_props grouping.
enum PropOrSpread<'a> {
    Prop(ObjProp<'a>),
    Spread(Expression<'a>),
}

/// Build the props expression: plain object when no spreads, `$.spread_props(...)` otherwise.
fn build_props_expr<'a>(ctx: &Ctx<'a>, items: Vec<PropOrSpread<'a>>) -> Expression<'a> {
    let has_spread = items.iter().any(|i| matches!(i, PropOrSpread::Spread(_)));

    if !has_spread {
        let props: Vec<ObjProp<'a>> = items
            .into_iter()
            .filter_map(|i| match i {
                PropOrSpread::Prop(p) => Some(p),
                _ => None,
            })
            .collect();
        return ctx.b.object_expr(props);
    }

    // Group consecutive props into objects, intersperse with spread expressions
    let mut args: Vec<Arg<'a, 'a>> = Vec::new();
    let mut current_props: Vec<ObjProp<'a>> = Vec::new();

    for item in items {
        match item {
            PropOrSpread::Prop(p) => current_props.push(p),
            PropOrSpread::Spread(expr) => {
                if !current_props.is_empty() {
                    args.push(Arg::Expr(
                        ctx.b.object_expr(std::mem::take(&mut current_props)),
                    ));
                }
                args.push(Arg::Expr(expr));
            }
        }
    }
    if !current_props.is_empty() {
        args.push(Arg::Expr(ctx.b.object_expr(current_props)));
    }

    ctx.b.call_expr("$.spread_props", args)
}

/// Build a member expression chain from a dotted name like `"registry.Widget"`.
/// Produces `registry.Widget` as `StaticMemberExpression(Identifier("registry"), "Widget")`.
fn build_dotted_member_expr<'a>(ctx: &Ctx<'a>, dotted_name: &str) -> Expression<'a> {
    let mut parts = dotted_name.split('.');
    let first = parts
        .next()
        .expect("dotted name must have at least one part");
    let mut expr = ctx.b.rid_expr(first);
    for part in parts {
        expr = ctx.b.static_member_expr(expr, part);
    }
    expr
}

/// Recover the slot name from an element's `slot="..."` attribute.
fn slot_name_from_element(ctx: &Ctx<'_>, el_id: NodeId) -> String {
    let el = ctx.element(el_id);
    for attr in &el.attributes {
        if let Attribute::StringAttribute(sa) = attr {
            if sa.name == "slot" {
                return ctx.query.component.source_text(sa.value_span).to_string();
            }
        }
    }
    unreachable!("named slot element must have slot attribute")
}
