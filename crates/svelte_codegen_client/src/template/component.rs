//! Component instantiation codegen.

use oxc_ast::ast::{Expression, Statement};

use svelte_analyze::{ContentType, FragmentItem, FragmentKey};
use svelte_ast::{Attribute, NodeId};
use svelte_span::Span;

use crate::builder::{Arg, ObjProp};
use crate::context::Ctx;

use super::expression::{build_attr_concat, parse_expr};
use super::gen_fragment;

/// Collected attribute info from the immutable Component borrow.
enum AttrKind {
    String { name: String, value_span: Span },
    Boolean { name: String },
    Expression { name: String, span: Span, shorthand: bool },
    Concatenation { name: String, attr_idx: usize },
    Shorthand { span: Span },
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
    let name = cn.name.clone();
    let name_str: &str = ctx.b.alloc_str(&name);

    // Collect attribute info while borrowing ctx immutably
    let attr_infos: Vec<(AttrKind, bool)> = cn.attributes.iter().enumerate().map(|(idx, attr)| {
        let is_dynamic = ctx.analysis.dynamic_attrs.contains(&(id, idx));
        let kind = match attr {
            Attribute::StringAttribute(a) => AttrKind::String {
                name: a.name.clone(),
                value_span: a.value_span,
            },
            Attribute::BooleanAttribute(a) => AttrKind::Boolean {
                name: a.name.clone(),
            },
            Attribute::ExpressionAttribute(a) => AttrKind::Expression {
                name: a.name.clone(),
                span: a.expression_span,
                shorthand: a.shorthand,
            },
            Attribute::ConcatenationAttribute(a) => AttrKind::Concatenation {
                name: a.name.clone(),
                attr_idx: idx,
            },
            Attribute::ShorthandOrSpread(a) if a.is_spread => AttrKind::Spread,
            Attribute::ShorthandOrSpread(a) => AttrKind::Shorthand {
                span: a.expression_span,
            },
            Attribute::BindDirective(_) | Attribute::ClassDirective(_) => AttrKind::Skip,
        };
        (kind, is_dynamic)
    }).collect();

    let mut props: Vec<ObjProp<'a>> = Vec::new();

    for (kind, is_dynamic) in attr_infos {
        match kind {
            AttrKind::String { name, value_span } => {
                let value_text = ctx.component.source_text(value_span);
                let key = ctx.b.alloc_str(&name);
                props.push(ObjProp::KeyValue(key, ctx.b.str_expr(value_text)));
            }
            AttrKind::Boolean { name } => {
                let key = ctx.b.alloc_str(&name);
                props.push(ObjProp::KeyValue(key, ctx.b.bool_expr(true)));
            }
            AttrKind::Expression { name, span, shorthand } => {
                let key = ctx.b.alloc_str(&name);
                let expr = parse_expr(ctx, span);
                if is_dynamic {
                    props.push(ObjProp::Getter(key, expr));
                } else if shorthand {
                    props.push(ObjProp::Shorthand(key));
                } else {
                    props.push(ObjProp::KeyValue(key, expr));
                }
            }
            AttrKind::Concatenation { name, attr_idx } => {
                let key = ctx.b.alloc_str(&name);
                // Re-borrow to access ConcatPart slice
                let cn = ctx.component_node(id);
                if let Attribute::ConcatenationAttribute(a) = &cn.attributes[attr_idx] {
                    let val = build_attr_concat(ctx, &a.parts);
                    if is_dynamic {
                        props.push(ObjProp::Getter(key, val));
                    } else {
                        props.push(ObjProp::KeyValue(key, val));
                    }
                }
            }
            AttrKind::Shorthand { span } => {
                let name_text = ctx.component.source_text(span).trim();
                let key = ctx.b.alloc_str(name_text);
                let expr = parse_expr(ctx, span);
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
    let children_ct = ctx.analysis.content_types
        .get(&FragmentKey::ComponentNode(id))
        .copied()
        .unwrap_or(ContentType::Empty);

    if children_ct != ContentType::Empty {
        let frag_key = FragmentKey::ComponentNode(id);

        // is_text_first: skip over inserted anchor comment when first child is text
        let is_text_first = ctx.analysis.lowered_fragments
            .get(&frag_key)
            .and_then(|lf| lf.items.first())
            .is_some_and(|item| matches!(item, FragmentItem::TextConcat { .. }));

        let mut body_stmts = Vec::new();
        if is_text_first {
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
        ctx.b.call_expr(name_str, [Arg::Expr(anchor), Arg::Expr(props_expr)]),
    ));
}
