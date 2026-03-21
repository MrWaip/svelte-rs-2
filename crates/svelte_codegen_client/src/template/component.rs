//! Component instantiation codegen.

use oxc_ast::ast::{Expression, Statement};

use svelte_analyze::{ContentStrategy, FragmentKey};
use svelte_ast::{Attribute, NodeId};
use svelte_span::Span;

use crate::builder::{Arg, AssignLeft, ObjProp};
use crate::context::Ctx;

use super::expression::{build_attr_concat, get_attr_expr};
use super::gen_fragment;

/// Collected attribute info from the immutable Component borrow.
enum AttrKind<'a> {
    String { name: &'a str, value_span: Span },
    Boolean { name: &'a str },
    Expression { name: &'a str, attr_id: NodeId, shorthand: bool },
    Concatenation { name: &'a str, attr_id: NodeId },
    Shorthand { attr_id: NodeId },
    BindThis { bind_id: NodeId },
    Spread,
    Skip,
}

/// Generate `ComponentName($$anchor, { props })` call.
pub(crate) fn gen_component<'a>(
    ctx: &mut Ctx<'a>,
    id: NodeId,
    anchor: Expression<'a>,
    init: &mut Vec<Statement<'a>>,
) {
    let cn = ctx.component_node(id);
    let name: &str = &cn.name;

    // Collect attribute info while borrowing ctx immutably
    let attr_infos: Vec<(AttrKind<'_>, bool)> = cn.attributes.iter().map(|attr| {
        let is_dynamic = ctx.is_dynamic_attr(attr.id());
        let kind = match attr {
            Attribute::StringAttribute(a) => AttrKind::String {
                name: &a.name,
                value_span: a.value_span,
            },
            Attribute::BooleanAttribute(a) => AttrKind::Boolean {
                name: &a.name,
            },
            Attribute::ExpressionAttribute(a) => {
                AttrKind::Expression {
                    name: &a.name,
                    attr_id: a.id,
                    shorthand: a.shorthand,
                }
            }
            Attribute::ConcatenationAttribute(a) => AttrKind::Concatenation {
                name: &a.name,
                attr_id: a.id,
            },
            Attribute::SpreadAttribute(_) => AttrKind::Spread,
            Attribute::Shorthand(a) => AttrKind::Shorthand {
                attr_id: a.id,
            },
            Attribute::BindDirective(b) if b.name == "this" => AttrKind::BindThis {
                bind_id: b.id,
            },
            Attribute::BindDirective(_) | Attribute::ClassDirective(_) | Attribute::StyleDirective(_)
            | Attribute::UseDirective(_) | Attribute::OnDirectiveLegacy(_)
            | Attribute::TransitionDirective(_)
            | Attribute::AnimateDirective(_)
            | Attribute::AttachTag(_) => AttrKind::Skip,
        };
        (kind, is_dynamic)
    }).collect();

    let mut props: Vec<ObjProp<'a>> = Vec::new();
    let mut bind_this_info: Option<NodeId> = None;
    let mut memo_counter: u32 = 0;
    let mut memo_stmts: Vec<Statement<'a>> = Vec::new();

    for (kind, is_dynamic) in attr_infos {
        match kind {
            AttrKind::String { name, value_span } => {
                let value_text = ctx.component.source_text(value_span);
                let key = ctx.b.alloc_str(name);
                props.push(ObjProp::KeyValue(key, ctx.b.str_expr(value_text)));
            }
            AttrKind::Boolean { name } => {
                let key = ctx.b.alloc_str(name);
                props.push(ObjProp::KeyValue(key, ctx.b.bool_expr(true)));
            }
            AttrKind::Expression { name, attr_id, shorthand } => {
                let needs_memo = ctx.analysis.component_attr_needs_memo(attr_id);
                let key = ctx.b.alloc_str(name);
                if needs_memo {
                    let memo_name = format!("${memo_counter}");
                    memo_counter += 1;
                    let expr = get_attr_expr(ctx, attr_id);
                    let thunk = ctx.b.thunk(expr);
                    let derived = ctx.b.call_expr("$.derived", [Arg::Expr(thunk)]);
                    memo_stmts.push(ctx.b.let_init_stmt(&memo_name, derived));
                    let memo_ref = ctx.b.alloc_str(&memo_name);
                    let get = ctx.b.call_expr("$.get", [Arg::Ident(memo_ref)]);
                    props.push(ObjProp::Getter(key, get));
                } else {
                    let expr = get_attr_expr(ctx, attr_id);
                    if is_dynamic {
                        props.push(ObjProp::Getter(key, expr));
                    } else if shorthand {
                        props.push(ObjProp::Shorthand(key));
                    } else {
                        props.push(ObjProp::KeyValue(key, expr));
                    }
                }
            }
            AttrKind::Concatenation { name, attr_id } => {
                let key = ctx.b.alloc_str(name);
                // Re-borrow to access concatenation parts
                let cn = ctx.component_node(id);
                let concat_attr = cn.attributes.iter().find(|a| a.id() == attr_id);
                if let Some(Attribute::ConcatenationAttribute(a)) = concat_attr {
                    let val = build_attr_concat(ctx, attr_id, &a.parts);
                    if is_dynamic {
                        props.push(ObjProp::Getter(key, val));
                    } else {
                        props.push(ObjProp::KeyValue(key, val));
                    }
                }
            }
            AttrKind::Shorthand { attr_id } => {
                // Re-borrow to get the shorthand name
                let cn = ctx.component_node(id);
                let shorthand_attr = cn.attributes.iter().find(|a| a.id() == attr_id);
                let name_text = if let Some(Attribute::Shorthand(a)) = shorthand_attr {
                    ctx.component.source_text(a.expression_span).trim().to_string()
                } else {
                    unreachable!()
                };
                let key = ctx.b.alloc_str(&name_text);
                let expr = get_attr_expr(ctx, attr_id);
                if is_dynamic {
                    props.push(ObjProp::Getter(key, expr));
                } else {
                    props.push(ObjProp::Shorthand(key));
                }
            }
            AttrKind::BindThis { bind_id } => {
                bind_this_info = Some(bind_id);
            }
            AttrKind::Spread | AttrKind::Skip => {}
        }
    }

    // Add children snippet if component has non-empty fragment
    let children_ct = ctx.content_type(&FragmentKey::ComponentNode(id));

    if children_ct != ContentStrategy::Empty {
        let frag_key = FragmentKey::ComponentNode(id);

        // For text-first content (Static/DynamicText), gen_fragment doesn't emit $.next(),
        // so the component handler must. For Dynamic (mixed), gen_fragment handles it.
        let needs_next = matches!(children_ct, ContentStrategy::Static(_) | ContentStrategy::DynamicText);

        let mut body_stmts = Vec::new();
        if needs_next {
            body_stmts.push(ctx.b.call_stmt("$.next", []));
        }
        body_stmts.extend(gen_fragment(ctx, frag_key));

        let params = ctx.b.params(["$$anchor", "$$slotProps"]);
        let arrow = ctx.b.arrow_expr(params, body_stmts);
        props.push(ObjProp::KeyValue("children", arrow));

        let slots_obj = ctx.b.object_expr([
            ObjProp::KeyValue("default", ctx.b.bool_expr(true))
        ]);
        props.push(ObjProp::KeyValue("$$slots", slots_obj));
    }

    let props_expr = ctx.b.object_expr(props);
    let component_call = ctx.b.call_expr(name, [Arg::Expr(anchor), Arg::Expr(props_expr)]);

    let final_expr = if let Some(bind_id) = bind_this_info {
        build_bind_this_call(ctx, id, bind_id, component_call)
    } else {
        component_call
    };

    if memo_stmts.is_empty() {
        init.push(ctx.b.expr_stmt(final_expr));
    } else {
        memo_stmts.push(ctx.b.expr_stmt(final_expr));
        init.push(ctx.b.block_stmt(memo_stmts));
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
    let _ = ctx.parsed.attr_exprs.remove(&bind_id);

    // Look up the bind directive from the component AST to get expression text
    let cn = ctx.component_node(component_id);
    let bind = cn.attributes.iter().find_map(|a| {
        if let Attribute::BindDirective(b) = a { if b.id == bind_id { Some(b) } else { None } } else { None }
    }).expect("bind:this attribute must exist");

    let var_name = if bind.shorthand {
        bind.name.clone()
    } else if let Some(span) = bind.expression_span {
        ctx.component.source_text(span).to_string()
    } else {
        // No expression — emit bare component call
        return value;
    };

    if svelte_js::is_simple_identifier(&var_name) {
        let is_rune = ctx.is_mutable_rune_target(bind_id);
        let expr_text = ctx.b.alloc_str(&var_name);

        let setter = if is_rune {
            let body = ctx.b.call_expr("$.set", [
                Arg::Ident(expr_text),
                Arg::Ident("$$value"),
                Arg::Expr(ctx.b.bool_expr(true)),
            ]);
            ctx.b
                .arrow_expr(ctx.b.params(["$$value"]), [ctx.b.expr_stmt(body)])
        } else {
            let body = ctx.b.assign_expr(
                AssignLeft::Ident(var_name),
                ctx.b.rid_expr("$$value"),
            );
            ctx.b
                .arrow_expr(ctx.b.params(["$$value"]), [ctx.b.expr_stmt(body)])
        };

        let getter = if is_rune {
            let body = ctx.b.call_expr("$.get", [Arg::Ident(expr_text)]);
            ctx.b
                .arrow_expr(ctx.b.no_params(), [ctx.b.expr_stmt(body)])
        } else {
            let body = ctx.b.rid_expr(expr_text);
            ctx.b
                .arrow_expr(ctx.b.no_params(), [ctx.b.expr_stmt(body)])
        };

        ctx.b.call_expr("$.bind_this", [
            Arg::Expr(value),
            Arg::Expr(setter),
            Arg::Expr(getter),
        ])
    } else {
        // Member/computed expression: re-parse from source to avoid reactive wrapping
        let each_context: Vec<String> = ctx.bind_each_context(bind_id)
            .cloned()
            .unwrap_or_default();

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
            ctx.b.call_expr("$.bind_this", [
                Arg::Expr(value),
                Arg::Expr(setter),
                Arg::Expr(getter),
            ])
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

            ctx.b.call_expr("$.bind_this", [
                Arg::Expr(value),
                Arg::Expr(setter),
                Arg::Expr(getter),
                Arg::Expr(context_thunk),
            ])
        }
    }
}
