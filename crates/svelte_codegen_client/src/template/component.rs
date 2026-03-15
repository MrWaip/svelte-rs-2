//! Component instantiation codegen.

use oxc_ast::ast::{Expression, Statement};

use svelte_analyze::{ContentType, FragmentKey};
use svelte_ast::{Attribute, NodeId};
use svelte_span::Span;

use crate::builder::{Arg, ObjProp};
use crate::context::Ctx;

use super::expression::{build_attr_concat, get_attr_expr};
use super::gen_fragment;

/// Collected attribute info from the immutable Component borrow.
enum AttrKind<'a> {
    String { name: &'a str, value_span: Span },
    Boolean { name: &'a str },
    Expression { name: &'a str, attr_idx: usize, shorthand: bool },
    Concatenation { name: &'a str, attr_idx: usize },
    Shorthand { attr_idx: usize },
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
    let attr_infos: Vec<(AttrKind<'_>, bool)> = cn.attributes.iter().enumerate().map(|(idx, attr)| {
        let is_dynamic = ctx.is_dynamic_attr(id, idx);
        let kind = match attr {
            Attribute::StringAttribute(a) => AttrKind::String {
                name: &a.name,
                value_span: a.value_span,
            },
            Attribute::BooleanAttribute(a) => AttrKind::Boolean {
                name: &a.name,
            },
            Attribute::ExpressionAttribute(a) => AttrKind::Expression {
                name: &a.name,
                attr_idx: idx,
                shorthand: a.shorthand,
            },
            Attribute::ConcatenationAttribute(a) => AttrKind::Concatenation {
                name: &a.name,
                attr_idx: idx,
            },
            Attribute::ShorthandOrSpread(a) if a.is_spread => AttrKind::Spread,
            Attribute::ShorthandOrSpread(_) => AttrKind::Shorthand {
                attr_idx: idx,
            },
            Attribute::BindDirective(_) | Attribute::ClassDirective(_) | Attribute::StyleDirective(_)
            | Attribute::OnDirectiveLegacy(_) => AttrKind::Skip,
        };
        (kind, is_dynamic)
    }).collect();

    let mut props: Vec<ObjProp<'a>> = Vec::new();

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
            AttrKind::Expression { name, attr_idx, shorthand } => {
                let key = ctx.b.alloc_str(name);
                let expr = get_attr_expr(ctx, id, attr_idx);
                if is_dynamic {
                    props.push(ObjProp::Getter(key, expr));
                } else if shorthand {
                    props.push(ObjProp::Shorthand(key));
                } else {
                    props.push(ObjProp::KeyValue(key, expr));
                }
            }
            AttrKind::Concatenation { name, attr_idx } => {
                let key = ctx.b.alloc_str(name);
                // Re-borrow to access ConcatPart slice
                let cn = ctx.component_node(id);
                if let Attribute::ConcatenationAttribute(a) = &cn.attributes[attr_idx] {
                    let val = build_attr_concat(ctx, id, attr_idx, &a.parts);
                    if is_dynamic {
                        props.push(ObjProp::Getter(key, val));
                    } else {
                        props.push(ObjProp::KeyValue(key, val));
                    }
                }
            }
            AttrKind::Shorthand { attr_idx } => {
                // Re-borrow to get the shorthand name
                let cn = ctx.component_node(id);
                let name_text = if let Attribute::ShorthandOrSpread(a) = &cn.attributes[attr_idx] {
                    ctx.component.source_text(a.expression_span).trim().to_string()
                } else {
                    unreachable!()
                };
                let key = ctx.b.alloc_str(&name_text);
                let expr = get_attr_expr(ctx, id, attr_idx);
                if is_dynamic {
                    props.push(ObjProp::Getter(key, expr));
                } else {
                    props.push(ObjProp::Shorthand(key));
                }
            }
            AttrKind::Spread | AttrKind::Skip => {}
        }
    }

    // Add children snippet if component has non-empty fragment
    let children_ct = ctx.content_type(&FragmentKey::ComponentNode(id));

    if children_ct != ContentType::Empty {
        let frag_key = FragmentKey::ComponentNode(id);

        // For text-first content (StaticText/DynamicText), gen_fragment doesn't emit $.next(),
        // so the component handler must. For Mixed, gen_fragment handles it.
        let needs_next = matches!(children_ct, ContentType::StaticText | ContentType::DynamicText);

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
    init.push(ctx.b.expr_stmt(
        ctx.b.call_expr(name, [Arg::Expr(anchor), Arg::Expr(props_expr)]),
    ));
}
