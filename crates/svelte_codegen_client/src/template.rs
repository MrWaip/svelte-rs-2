//! Template and DOM traversal code generation.

use oxc_allocator::Allocator;
use oxc_ast::ast::{Expression, Statement};
use oxc_parser::Parser as OxcParser;
use oxc_semantic::SemanticBuilder;
use oxc_span::SourceType;
use oxc_traverse::{Traverse, TraverseCtx, traverse_mut};
use std::collections::HashSet;

use svelte_analyze::{ConcatPart, ContentType, FragmentItem, FragmentKey};
use svelte_ast::{Attribute, ConcatPart as AstConcatPart, EachBlock, Element, IfBlock, Node, NodeId};
use svelte_span::Span;

use crate::builder::{Arg, AssignLeft, AssignRight, Builder, TemplatePart};
use crate::context::Ctx;

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

/// Returns `(hoisted, body)` for the root fragment.
pub fn gen_root_fragment<'a>(ctx: &mut Ctx<'a>) -> (Vec<Statement<'a>>, Vec<Statement<'a>>) {
    let key = FragmentKey::Root;
    let ct = ctx.analysis.content_types.get(&key).copied().unwrap_or(ContentType::Empty);

    let mut hoisted = Vec::new();
    let mut body = Vec::new();

    match ct {
        ContentType::Empty => {}
        ContentType::StaticText => gen_root_static_text(ctx, &mut body),
        ContentType::DynamicText => gen_root_dynamic_text(ctx, &mut body),
        ContentType::SingleElement => gen_root_single_element(ctx, &mut hoisted, &mut body),
        ContentType::SingleBlock => gen_root_single_block(ctx, &mut body),
        ContentType::Mixed => gen_root_mixed(ctx, &mut hoisted, &mut body),
    }

    (hoisted, body)
}

// ---------------------------------------------------------------------------
// Root strategies
// ---------------------------------------------------------------------------

fn gen_root_static_text<'a>(ctx: &mut Ctx<'a>, body: &mut Vec<Statement<'a>>) {
    let lf = ctx.analysis.lowered_fragments.get(&FragmentKey::Root).unwrap();
    let text = static_text_of(&lf.items[0]);
    let name = ctx.gen_ident("text");
    body.push(ctx.b.call_stmt("$.next", []));
    let call = if text.is_empty() {
        ctx.b.call_expr("$.text", [])
    } else {
        ctx.b.call_expr("$.text", [Arg::Str(text)])
    };
    body.push(ctx.b.var_stmt(&name, call));
    body.push(ctx.b.call_stmt("$.append", [Arg::Ident("$$anchor"), Arg::Ident(&name)]));
}

fn gen_root_dynamic_text<'a>(ctx: &mut Ctx<'a>, body: &mut Vec<Statement<'a>>) {
    let item = {
        let lf = ctx.analysis.lowered_fragments.get(&FragmentKey::Root).unwrap();
        lf.items[0].clone()
    };
    let name = ctx.gen_ident("text");
    body.push(ctx.b.call_stmt("$.next", []));
    body.push(ctx.b.var_stmt(&name, ctx.b.call_expr("$.text", [])));

    emit_text_update(ctx, &item, &name, body);

    body.push(ctx.b.call_stmt("$.append", [Arg::Ident("$$anchor"), Arg::Ident(&name)]));
}

fn gen_root_single_element<'a>(
    ctx: &mut Ctx<'a>,
    hoisted: &mut Vec<Statement<'a>>,
    body: &mut Vec<Statement<'a>>,
) {
    let el_id = {
        let lf = ctx.analysis.lowered_fragments.get(&FragmentKey::Root).unwrap();
        match lf.items[0] { FragmentItem::Element(id) => id, _ => unreachable!() }
    };

    let tpl_name = ctx.gen_ident("root");
    let el = find_el(ctx, el_id);
    let html = element_html(ctx, &el);
    hoisted.push(ctx.b.var_stmt(
        &tpl_name,
        ctx.b.call_expr("$.template", [Arg::Expr(ctx.b.template_str_expr(&html))]),
    ));

    let el_name = ctx.gen_ident(&el.name.clone());
    body.push(ctx.b.var_stmt(&el_name, ctx.b.call_expr(&tpl_name, [])));

    let mut init = Vec::new();
    let mut update = Vec::new();
    let mut after_update = Vec::new();
    process_element(ctx, el_id, &el_name, &mut init, &mut update, hoisted, &mut after_update);
    body.extend(init);
    emit_template_effect(ctx, update, body);
    body.extend(after_update);
    body.push(ctx.b.call_stmt("$.append", [Arg::Ident("$$anchor"), Arg::Ident(&el_name)]));
}

fn gen_root_single_block<'a>(ctx: &mut Ctx<'a>, body: &mut Vec<Statement<'a>>) {
    let item = {
        let lf = ctx.analysis.lowered_fragments.get(&FragmentKey::Root).unwrap();
        lf.items[0].clone()
    };
    let frag = ctx.gen_ident("fragment");
    let node = ctx.gen_ident("node");
    body.push(ctx.b.var_stmt(&frag, ctx.b.call_expr("$.comment", [])));
    body.push(ctx.b.var_stmt(&node, ctx.b.call_expr("$.first_child", [Arg::Ident(&frag)])));

    match item {
        FragmentItem::IfBlock(id) => {
            let stmts = gen_if_block(ctx, id, ctx.b.rid_expr(&node), None);
            body.push(ctx.b.block_stmt(stmts));
        }
        FragmentItem::EachBlock(id) => {
            gen_each_block(ctx, id, ctx.b.rid_expr(&node), body);
        }
        _ => unreachable!(),
    }

    body.push(ctx.b.call_stmt("$.append", [Arg::Ident("$$anchor"), Arg::Ident(&frag)]));
}

fn gen_root_mixed<'a>(
    ctx: &mut Ctx<'a>,
    hoisted: &mut Vec<Statement<'a>>,
    body: &mut Vec<Statement<'a>>,
) {
    let items: Vec<_> = {
        let lf = ctx.analysis.lowered_fragments.get(&FragmentKey::Root).unwrap();
        lf.items.clone()
    };

    let starts_text = matches!(items.first(), Some(FragmentItem::TextConcat { .. }));
    if starts_text {
        body.push(ctx.b.call_stmt("$.next", []));
    }

    let tpl_name = ctx.gen_ident("root");
    let html = fragment_html(ctx, FragmentKey::Root);
    hoisted.push(ctx.b.var_stmt(
        &tpl_name,
        ctx.b.call_expr(
            "$.template",
            [Arg::Expr(ctx.b.template_str_expr(&html)), Arg::Num(1.0)],
        ),
    ));

    let frag = ctx.gen_ident("fragment");
    body.push(ctx.b.var_stmt(&frag, ctx.b.call_expr(&tpl_name, [])));

    let first_child = ctx.b.call_expr("$.first_child", [Arg::Ident(&frag)]);
    let mut init = Vec::new();
    let mut update = Vec::new();
    let mut after_update = Vec::new();
    let trailing = traverse_items(ctx, &items, first_child, &mut init, &mut update, hoisted, &mut after_update);
    emit_trailing_next(ctx, trailing, &mut init);
    body.extend(init);
    emit_template_effect(ctx, update, body);
    body.extend(after_update);
    body.push(ctx.b.call_stmt("$.append", [Arg::Ident("$$anchor"), Arg::Ident(&frag)]));
}

// ---------------------------------------------------------------------------
// Fragment codegen (used for if/each/nested fragments)
// ---------------------------------------------------------------------------

/// Generate statements for a fragment, destined for `($$anchor) => { ... }`.
fn gen_fragment<'a>(ctx: &mut Ctx<'a>, key: FragmentKey) -> Vec<Statement<'a>> {
    let ct = ctx.analysis.content_types.get(&key).copied().unwrap_or(ContentType::Empty);

    let mut body: Vec<Statement<'a>> = Vec::new();

    match ct {
        ContentType::Empty => {}
        ContentType::StaticText => {
            let text = {
                let lf = ctx.analysis.lowered_fragments.get(&key).unwrap();
                static_text_of(&lf.items[0])
            };
            let name = ctx.gen_ident("text");
            let call = if text.is_empty() {
                ctx.b.call_expr("$.text", [])
            } else {
                ctx.b.call_expr("$.text", [Arg::Str(text)])
            };
            body.push(ctx.b.var_stmt(&name, call));
            body.push(ctx.b.call_stmt("$.append", [Arg::Ident("$$anchor"), Arg::Ident(&name)]));
        }
        ContentType::DynamicText => {
            let item = {
                let lf = ctx.analysis.lowered_fragments.get(&key).unwrap();
                lf.items[0].clone()
            };
            let name = ctx.gen_ident("text");
            body.push(ctx.b.var_stmt(&name, ctx.b.call_expr("$.text", [])));
            emit_text_update(ctx, &item, &name, &mut body);
            body.push(ctx.b.call_stmt("$.append", [Arg::Ident("$$anchor"), Arg::Ident(&name)]));
        }
        ContentType::SingleElement => {
            let el_id = {
                let lf = ctx.analysis.lowered_fragments.get(&key).unwrap();
                match lf.items[0] { FragmentItem::Element(id) => id, _ => unreachable!() }
            };

            let tpl_name = ctx.gen_ident("root");
            let el = find_el(ctx, el_id);
            let html = element_html(ctx, &el);
            ctx.module_hoisted.push(ctx.b.var_stmt(
                &tpl_name,
                ctx.b.call_expr("$.template", [Arg::Expr(ctx.b.template_str_expr(&html))]),
            ));

            let el_name = ctx.gen_ident(&el.name.clone());

            let mut init = Vec::new();
            let mut update = Vec::new();
            let mut sub_hoisted = Vec::new();
            let mut after_update = Vec::new();
            process_element(ctx, el_id, &el_name, &mut init, &mut update, &mut sub_hoisted, &mut after_update);
            ctx.module_hoisted.extend(sub_hoisted);

            let mut result = Vec::new();
            result.push(ctx.b.var_stmt(&el_name, ctx.b.call_expr(&tpl_name, [])));
            result.extend(init);
            emit_template_effect(ctx, update, &mut result);
            result.extend(after_update);
            result.push(ctx.b.call_stmt("$.append", [Arg::Ident("$$anchor"), Arg::Ident(&el_name)]));
            return result;
        }
        ContentType::SingleBlock => {
            let item = {
                let lf = ctx.analysis.lowered_fragments.get(&key).unwrap();
                lf.items[0].clone()
            };
            let frag = ctx.gen_ident("fragment");
            let node = ctx.gen_ident("node");
            body.push(ctx.b.var_stmt(&frag, ctx.b.call_expr("$.comment", [])));
            body.push(ctx.b.var_stmt(
                &node,
                ctx.b.call_expr("$.first_child", [Arg::Ident(&frag)]),
            ));
            match item {
                FragmentItem::IfBlock(id) => {
                    let stmts = gen_if_block(ctx, id, ctx.b.rid_expr(&node), None);
                    body.push(ctx.b.block_stmt(stmts));
                }
                FragmentItem::EachBlock(id) => {
                    gen_each_block(ctx, id, ctx.b.rid_expr(&node), &mut body);
                }
                _ => unreachable!(),
            }
            body.push(ctx.b.call_stmt("$.append", [Arg::Ident("$$anchor"), Arg::Ident(&frag)]));
        }
        ContentType::Mixed => {
            let items: Vec<_> = {
                let lf = ctx.analysis.lowered_fragments.get(&key).unwrap();
                lf.items.clone()
            };

            let tpl_name = ctx.gen_ident("root");
            let html = fragment_html(ctx, key);
            ctx.module_hoisted.push(ctx.b.var_stmt(
                &tpl_name,
                ctx.b.call_expr("$.template", [Arg::Expr(ctx.b.template_str_expr(&html))]),
            ));

            let frag = ctx.gen_ident("fragment");
            body.push(ctx.b.var_stmt(&frag, ctx.b.call_expr(&tpl_name, [])));

            let first = ctx.b.call_expr("$.first_child", [Arg::Ident(&frag)]);
            let mut init = Vec::new();
            let mut update = Vec::new();
            let mut sub_hoisted = Vec::new();
            let mut after_update = Vec::new();
            let trailing = traverse_items(ctx, &items, first, &mut init, &mut update, &mut sub_hoisted, &mut after_update);
            ctx.module_hoisted.extend(sub_hoisted);
            emit_trailing_next(ctx, trailing, &mut init);
            body.extend(init);
            emit_template_effect(ctx, update, &mut body);
            body.extend(after_update);
            body.push(ctx.b.call_stmt("$.append", [Arg::Ident("$$anchor"), Arg::Ident(&frag)]));
            return body;
        }
    }

    body
}

// ---------------------------------------------------------------------------
// DOM traversal for Mixed fragments
// ---------------------------------------------------------------------------

/// `first_child_expr` is the initial anchor (e.g. `$.first_child(fragment)`).
/// Returns via `init` (DOM var declarations) and `update` (template_effect contents).
/// Returns trailing `sibling_offset` (number of unprocessed static siblings after the last var).
fn traverse_items<'a>(
    ctx: &mut Ctx<'a>,
    items: &[FragmentItem],
    first_child_expr: Expression<'a>,
    init: &mut Vec<Statement<'a>>,
    update: &mut Vec<Statement<'a>>,
    hoisted: &mut Vec<Statement<'a>>,
    after_update: &mut Vec<Statement<'a>>,
) -> usize {
    // Track current DOM position: (prev_expr or ident, offset since last named node)
    let mut prev_expr: Option<Expression<'a>> = Some(first_child_expr);
    let mut prev_ident: Option<String> = None;
    // sibling_offset: how many dom siblings since the last named node was assigned
    let mut sibling_offset: usize = 0;

    for item in items {
        let needs_var = item_needs_var(item, ctx);

        if needs_var {
            // Build the expression to get this node's DOM reference
            let node_expr: Expression<'a> = if let Some(ident) = &prev_ident {
                // We have a named prev node
                if sibling_offset == 1 {
                    ctx.b.call_expr("$.sibling", [Arg::Ident(ident)])
                } else {
                    ctx.b.call_expr("$.sibling", [Arg::Ident(ident), Arg::Num(sibling_offset as f64)])
                }
            } else {
                // First var: use the accumulated first_child expression
                let fc = prev_expr.take().unwrap();
                if sibling_offset == 0 {
                    // This item IS the first child
                    fc
                } else if sibling_offset == 1 {
                    ctx.b.call_expr("$.sibling", [Arg::Expr(fc)])
                } else {
                    ctx.b.call_expr("$.sibling", [Arg::Expr(fc), Arg::Num(sibling_offset as f64)])
                }
            };

            match item {
                FragmentItem::TextConcat { parts } => {
                    let name = ctx.gen_ident("text");
                    init.push(ctx.b.var_stmt(&name, node_expr));
                    prev_ident = Some(name.clone());
                    sibling_offset = 1;

                    // Generate content expression
                    let is_dyn = parts_are_dynamic(parts, ctx);
                    if is_dyn {
                        let expr = build_concat_from_parts(ctx, parts);
                        update.push(ctx.b.call_stmt(
                            "$.set_text",
                            [Arg::Ident(&name), Arg::Expr(expr)],
                        ));
                    } else {
                        // Static: assign nodeValue once
                        let expr = build_concat_from_parts(ctx, parts);
                        init.push(ctx.b.assign_stmt(
                            AssignLeft::StaticMember(ctx.b.static_member(ctx.b.rid_expr(&name), "nodeValue")),
                            AssignRight::Expr(expr),
                        ));
                    }
                }

                FragmentItem::Element(el_id) => {
                    let el = find_el(ctx, *el_id);
                    let el_name = ctx.gen_ident(&el.name.clone());
                    init.push(ctx.b.var_stmt(&el_name, node_expr));
                    prev_ident = Some(el_name.clone());
                    sibling_offset = 1;
                    process_element(ctx, *el_id, &el_name, init, update, hoisted, after_update);
                }

                FragmentItem::IfBlock(id) => {
                    let node_name = ctx.gen_ident("node");
                    init.push(ctx.b.var_stmt(&node_name, node_expr));
                    prev_ident = Some(node_name.clone());
                    sibling_offset = 1;

                    let stmts = gen_if_block(ctx, *id, ctx.b.rid_expr(&node_name), None);
                    init.push(ctx.b.block_stmt(stmts));
                }

                FragmentItem::EachBlock(id) => {
                    let node_name = ctx.gen_ident("node");
                    init.push(ctx.b.var_stmt(&node_name, node_expr));
                    prev_ident = Some(node_name.clone());
                    sibling_offset = 1;
                    gen_each_block(ctx, *id, ctx.b.rid_expr(&node_name), init);
                }
            }
        } else {
            // Static item — advance sibling offset
            sibling_offset += 1;
        }
    }

    sibling_offset
}

// ---------------------------------------------------------------------------
// Element processing
// ---------------------------------------------------------------------------

fn process_element<'a>(
    ctx: &mut Ctx<'a>,
    el_id: NodeId,
    el_name: &str,
    init: &mut Vec<Statement<'a>>,
    update: &mut Vec<Statement<'a>>,
    hoisted: &mut Vec<Statement<'a>>,
    after_update: &mut Vec<Statement<'a>>,
) {
    let el = find_el(ctx, el_id);
    let child_key = FragmentKey::Element(el_id);
    let ct = ctx.analysis.content_types.get(&child_key).copied().unwrap_or(ContentType::Empty);

    // $.remove_input_defaults for <input> elements with any bind directive
    let is_input = el.name == "input";
    let has_bind = el.attributes.iter().any(|a| matches!(a, Attribute::BindDirective(_)));
    if is_input && has_bind {
        init.push(ctx.b.call_stmt("$.remove_input_defaults", [Arg::Ident(el_name)]));
    }

    // Attributes
    for (idx, attr) in el.attributes.iter().enumerate() {
        let is_dyn = ctx.analysis.dynamic_attrs.contains(&(el_id, idx));
        process_attr(ctx, attr, el_name, is_dyn, init, update, after_update);
    }

    // Children
    let has_state = has_dynamic_children(ctx, el_id);
    match ct {
        ContentType::Empty | ContentType::StaticText => {}

        ContentType::DynamicText if !has_state => {
            // textContent shortcut
            let items: Vec<_> = ctx.analysis.lowered_fragments
                .get(&child_key)
                .unwrap()
                .items
                .clone();
            let expr = build_concat(ctx, &items[0]);
            init.push(ctx.b.assign_stmt(
                AssignLeft::StaticMember(ctx.b.static_member(ctx.b.rid_expr(el_name), "textContent")),
                AssignRight::Expr(expr),
            ));
        }

        ContentType::DynamicText => {
            // $.child(el, true) + $.reset(el) + $.set_text in update
            let text_name = ctx.gen_ident("text");
            init.push(ctx.b.var_stmt(
                &text_name,
                ctx.b.call_expr("$.child", [Arg::Ident(el_name), Arg::Bool(true)]),
            ));
            init.push(ctx.b.call_stmt("$.reset", [Arg::Ident(el_name)]));

            let items: Vec<_> = ctx.analysis.lowered_fragments
                .get(&child_key)
                .unwrap()
                .items
                .clone();
            let expr = build_concat(ctx, &items[0]);
            update.push(ctx.b.call_stmt(
                "$.set_text",
                [Arg::Ident(&text_name), Arg::Expr(expr)],
            ));
        }

        ContentType::SingleElement | ContentType::SingleBlock | ContentType::Mixed => {
            let child_items: Vec<_> = ctx.analysis.lowered_fragments
                .get(&child_key)
                .unwrap()
                .items
                .clone();

            let first_child = ctx.b.call_expr("$.child", [Arg::Ident(el_name)]);
            let mut child_init = Vec::new();
            let mut child_update = Vec::new();
            let trailing = traverse_items(ctx, &child_items, first_child, &mut child_init, &mut child_update, hoisted, after_update);

            emit_trailing_next(ctx, trailing, &mut child_init);
            if !child_init.is_empty() {
                child_init.push(ctx.b.call_stmt("$.reset", [Arg::Ident(el_name)]));
            }

            init.extend(child_init);
            update.extend(child_update);
        }
    }
}

// ---------------------------------------------------------------------------
// Attribute processing
// ---------------------------------------------------------------------------

fn process_attr<'a>(
    ctx: &mut Ctx<'a>,
    attr: &Attribute,
    el_name: &str,
    is_dyn: bool,
    init: &mut Vec<Statement<'a>>,
    update: &mut Vec<Statement<'a>>,
    after_update: &mut Vec<Statement<'a>>,
) {
    let target = if is_dyn { update as &mut Vec<_> } else { init as &mut Vec<_> };

    match attr {
        Attribute::StringAttribute(_) | Attribute::BooleanAttribute(_) => {
            // Static — already in template HTML, nothing to do at runtime
        }
        Attribute::ExpressionAttribute(a) => {
            let val = parse_expr(ctx, a.expression_span);
            target.push(ctx.b.call_stmt(
                "$.set_attribute",
                [Arg::Ident(el_name), Arg::Str(a.name.clone()), Arg::Expr(val)],
            ));
        }
        Attribute::ConcatenationAttribute(a) => {
            let val = build_attr_concat(ctx, &a.parts);
            target.push(ctx.b.call_stmt(
                "$.set_attribute",
                [Arg::Ident(el_name), Arg::Str(a.name.clone()), Arg::Expr(val)],
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

            let is_mutated_rune = ctx.mutated_runes.contains(&var_name)
                && ctx.analysis.symbol_by_name.get(&var_name)
                    .is_some_and(|sid| ctx.analysis.runes.contains_key(sid));

            // getter: () => $.get(value) OR () => name
            let getter_body = if is_mutated_rune {
                ctx.b.call_expr("$.get", [Arg::Ident(&var_name)])
            } else {
                ctx.b.rid_expr(&var_name)
            };
            let getter = ctx.b.arrow_expr(ctx.b.no_params(), [ctx.b.expr_stmt(getter_body)]);

            // setter: ($$value) => $.set(value, $$value) OR ($$value) => name = $$value
            let setter_body = if is_mutated_rune {
                ctx.b.call_expr("$.set", [Arg::Ident(&var_name), Arg::Ident("$$value")])
            } else {
                ctx.b.assign_expr(
                    AssignLeft::Ident(var_name),
                    AssignRight::Expr(ctx.b.rid_expr("$$value")),
                )
            };
            let setter = ctx.b.arrow_expr(ctx.b.params(["$$value"]), [ctx.b.expr_stmt(setter_body)]);

            after_update.push(ctx.b.call_stmt(
                "$.bind_value",
                [Arg::Ident(el_name), Arg::Expr(getter), Arg::Expr(setter)],
            ));
        }
        Attribute::ShorthandOrSpread(_)
        | Attribute::ClassDirective(_) => {
            // TODO: full support
        }
    }
}

// ---------------------------------------------------------------------------
// IfBlock
// ---------------------------------------------------------------------------

fn gen_if_block<'a>(
    ctx: &mut Ctx<'a>,
    block_id: NodeId,
    anchor: Expression<'a>,
    elseif_arg: Option<&str>,
) -> Vec<Statement<'a>> {
    let block = find_if(ctx, block_id);
    let has_alternate = block.alternate.is_some();
    let test_span = block.test_span;

    let consequent_key = FragmentKey::IfConsequent(block_id);
    let alternate_key = FragmentKey::IfAlternate(block_id);

    let mut stmts = Vec::new();

    // detect elseif before generating alternate
    let alt_is_elseif = has_alternate && {
        let lf = ctx.analysis.lowered_fragments.get(&alternate_key);
        lf.is_some_and(|lf| {
            lf.items.len() == 1
                && matches!(&lf.items[0], FragmentItem::IfBlock(id) if {
                    find_if(ctx, *id).elseif
                })
        })
    };

    // consequent fn: ($$anchor) => { ... }
    let cons_name = ctx.gen_ident("consequent");
    let cons_body = gen_fragment(ctx, consequent_key);
    let cons_fn = ctx.b.arrow_expr(ctx.b.params(["$$anchor"]), cons_body);
    stmts.push(ctx.b.var_stmt(&cons_name, cons_fn));

    // alternate fn (if any)
    let alt_name: Option<String> = if has_alternate {
        let alt_name = ctx.gen_ident("alternate");
        if alt_is_elseif {
            // elseif: skip gen_fragment, call gen_if_block for nested IfBlock directly
            let nested_id = {
                let lf = ctx.analysis.lowered_fragments.get(&alternate_key).unwrap();
                match &lf.items[0] {
                    FragmentItem::IfBlock(id) => *id,
                    _ => unreachable!(),
                }
            };
            let nested_stmts =
                gen_if_block(ctx, nested_id, ctx.b.rid_expr("$$anchor"), Some("$$elseif"));
            let alt_body = vec![ctx.b.block_stmt(nested_stmts)];
            let alt_fn = ctx.b.arrow_expr(ctx.b.params(["$$anchor", "$$elseif"]), alt_body);
            stmts.push(ctx.b.var_stmt(&alt_name, alt_fn));
        } else {
            let alt_body = gen_fragment(ctx, alternate_key);
            let alt_fn = ctx.b.arrow_expr(ctx.b.params(["$$anchor"]), alt_body);
            stmts.push(ctx.b.var_stmt(&alt_name, alt_fn));
        }
        Some(alt_name)
    } else {
        None
    };

    // render fn: ($$render) => { if (test) $$render(cons); else $$render(alt, false); }
    let test = parse_expr(ctx, test_span);
    let then_stmt = ctx.b.call_stmt("$$render", [Arg::Ident(&cons_name)]);
    let else_stmt = alt_name.as_ref().map(|an| {
        ctx.b.call_stmt("$$render", [Arg::Ident(an), Arg::Bool(false)])
    });
    let if_stmt = ctx.b.if_stmt(test, then_stmt, else_stmt);
    let render_fn = ctx.b.arrow(ctx.b.params(["$$render"]), [if_stmt]);

    // $.if(anchor, render_fn[, $$elseif])
    let mut args: Vec<Arg<'a, '_>> = vec![Arg::Expr(anchor), Arg::Arrow(render_fn)];
    if let Some(earg) = elseif_arg {
        args.push(Arg::Ident(earg));
    }
    stmts.push(ctx.b.call_stmt("$.if", args));

    stmts
}

// ---------------------------------------------------------------------------
// EachBlock
// ---------------------------------------------------------------------------

fn gen_each_block<'a>(
    ctx: &mut Ctx<'a>,
    block_id: NodeId,
    anchor: Expression<'a>,
    body: &mut Vec<Statement<'a>>,
) {
    let block = find_each(ctx, block_id);
    let body_key = FragmentKey::EachBody(block_id);
    let expr_span = block.expression_span;
    let context_span = block.context_span;

    let collection = parse_expr(ctx, expr_span);
    let collection_fn = ctx.b.arrow_expr(ctx.b.no_params(), [ctx.b.expr_stmt(collection)]);

    let context_name = ctx.component.source_text(context_span).to_string();
    let frag_body = gen_fragment(ctx, body_key);
    let frag_fn = ctx.b.arrow_expr(ctx.b.params(["$$anchor", &context_name]), frag_body);

    body.push(ctx.b.call_stmt(
        "$.each",
        [
            Arg::Expr(anchor),
            Arg::Num(16.0),
            Arg::Expr(collection_fn),
            Arg::IdentRef(ctx.b.rid("$.index")),
            Arg::Expr(frag_fn),
        ],
    ));
}

// ---------------------------------------------------------------------------
// Template HTML builder
// ---------------------------------------------------------------------------

fn fragment_html(ctx: &Ctx<'_>, key: FragmentKey) -> String {
    let Some(lf) = ctx.analysis.lowered_fragments.get(&key) else { return String::new() };
    let mut html = String::new();
    for item in &lf.items {
        match item {
            FragmentItem::TextConcat { parts } => {
                let has_expr = parts.iter().any(|p| matches!(p, ConcatPart::Expr(_)));
                if has_expr {
                    html.push(' ');
                } else {
                    for part in parts {
                        if let ConcatPart::Text(t) = part {
                            html.push_str(t);
                        }
                    }
                }
            }
            FragmentItem::Element(id) => {
                let el = find_el_ref(ctx, *id);
                html.push_str(&element_html(ctx, el));
            }
            FragmentItem::IfBlock(_) | FragmentItem::EachBlock(_) => html.push_str("<!>"),
        }
    }
    html
}

fn element_html(ctx: &Ctx<'_>, el: &Element) -> String {
    let mut html = format!("<{}", el.name);
    for attr in &el.attributes {
        match attr {
            Attribute::StringAttribute(a) => {
                html.push_str(&format!(" {}=\"{}\"", a.name, ctx.component.source_text(a.value_span)));
            }
            Attribute::BooleanAttribute(a) => {
                html.push_str(&format!(" {}=\"\"", a.name));
            }
            _ => {}
        }
    }
    if el.self_closing {
        html.push('>');
        return html;
    }
    html.push('>');

    let child_key = FragmentKey::Element(el.id);
    let ct = ctx.analysis.content_types.get(&child_key).copied().unwrap_or(ContentType::Empty);
    let has_state = has_dynamic_children(ctx, el.id);

    match ct {
        ContentType::Empty => {}
        ContentType::StaticText => {
            let lf = ctx.analysis.lowered_fragments.get(&child_key).unwrap();
            html.push_str(&static_text_of(&lf.items[0]));
        }
        ContentType::DynamicText if !has_state => {
            // textContent shortcut — no placeholder
        }
        ContentType::DynamicText => {
            // space placeholder for the text node
            html.push(' ');
        }
        ContentType::SingleElement | ContentType::SingleBlock | ContentType::Mixed => {
            html.push_str(&fragment_html(ctx, child_key));
        }
    }

    html.push_str(&format!("</{}>", el.name));
    html
}

// ---------------------------------------------------------------------------
// Expression helpers
// ---------------------------------------------------------------------------

fn build_concat<'a>(ctx: &mut Ctx<'a>, item: &FragmentItem) -> Expression<'a> {
    match item {
        FragmentItem::TextConcat { parts } => build_concat_from_parts(ctx, parts),
        _ => ctx.b.str_expr(""),
    }
}

fn build_concat_from_parts<'a>(ctx: &mut Ctx<'a>, parts: &[ConcatPart]) -> Expression<'a> {
    // Single plain expression → no template literal wrapper
    if parts.len() == 1 {
        if let ConcatPart::Expr(nid) = parts[0] {
            let span = find_expr_span(ctx, nid);
            return parse_expr(ctx, span);
        }
    }

    let mut tpl_parts: Vec<TemplatePart<'a>> = Vec::new();
    for part in parts {
        match part {
            ConcatPart::Text(s) => tpl_parts.push(TemplatePart::Str(s.clone())),
            ConcatPart::Expr(nid) => {
                let span = find_expr_span(ctx, *nid);
                let expr = parse_expr(ctx, span);
                tpl_parts.push(TemplatePart::Expr(expr));
            }
        }
    }
    ctx.b.template_parts_expr(tpl_parts)
}

fn build_attr_concat<'a>(ctx: &mut Ctx<'a>, parts: &[AstConcatPart]) -> Expression<'a> {
    let mut tpl_parts: Vec<TemplatePart<'a>> = Vec::new();
    for part in parts {
        match part {
            AstConcatPart::Static(s) => tpl_parts.push(TemplatePart::Str(s.clone())),
            AstConcatPart::Dynamic(span) => {
                let expr = parse_expr(ctx, *span);
                tpl_parts.push(TemplatePart::Expr(expr));
            }
        }
    }
    ctx.b.template_parts_expr(tpl_parts)
}

fn static_text_of(item: &FragmentItem) -> String {
    match item {
        FragmentItem::TextConcat { parts } => {
            parts.iter().filter_map(|p| if let ConcatPart::Text(s) = p { Some(s.as_str()) } else { None }).collect()
        }
        _ => String::new(),
    }
}

/// Emit a text update (set_text or nodeValue assignment) depending on dynamism.
fn emit_text_update<'a>(
    ctx: &mut Ctx<'a>,
    item: &FragmentItem,
    node_name: &str,
    body: &mut Vec<Statement<'a>>,
) {
    let is_dyn = item_is_dynamic(item, ctx);
    let expr = build_concat(ctx, item);

    if is_dyn {
        let set = ctx.b.call_stmt("$.set_text", [Arg::Ident(node_name), Arg::Expr(expr)]);
        let eff = ctx.b.arrow(ctx.b.no_params(), [set]);
        body.push(ctx.b.call_stmt("$.template_effect", [Arg::Arrow(eff)]));
    } else {
        body.push(ctx.b.assign_stmt(
            AssignLeft::StaticMember(ctx.b.static_member(ctx.b.rid_expr(node_name), "nodeValue")),
            AssignRight::Expr(expr),
        ));
    }
}

fn emit_template_effect<'a>(
    ctx: &mut Ctx<'a>,
    update: Vec<Statement<'a>>,
    body: &mut Vec<Statement<'a>>,
) {
    if update.is_empty() {
        return;
    }
    if update.len() == 1 {
        if let [ref single] = update.as_slice() {
            // For a single call-expression statement, use inline arrow
            if matches!(single, Statement::ExpressionStatement(_)) {
                let eff = ctx.b.arrow(ctx.b.no_params(), update);
                body.push(ctx.b.call_stmt("$.template_effect", [Arg::Arrow(eff)]));
                return;
            }
        }
    }
    let eff = ctx.b.arrow(ctx.b.no_params(), update);
    body.push(ctx.b.call_stmt("$.template_effect", [Arg::Arrow(eff)]));
}

/// Emit `$.next()` or `$.next(N)` for trailing static siblings after the last named var.
fn emit_trailing_next<'a>(ctx: &mut Ctx<'a>, trailing: usize, stmts: &mut Vec<Statement<'a>>) {
    if trailing <= 1 {
        return;
    }
    let offset = trailing - 1;
    if offset == 1 {
        stmts.push(ctx.b.call_stmt("$.next", []));
    } else {
        stmts.push(ctx.b.call_stmt("$.next", [Arg::Num(offset as f64)]));
    }
}

// ---------------------------------------------------------------------------
// Expression parsing + rune transformation
// ---------------------------------------------------------------------------

pub fn parse_expr<'a>(ctx: &mut Ctx<'a>, span: Span) -> Expression<'a> {
    let source = ctx.component.source_text(span);
    let mutated = ctx.mutated_runes.clone();
    let rune_names: HashSet<String> = ctx
        .analysis
        .symbol_by_name
        .iter()
        .filter_map(|(n, sid)| ctx.analysis.runes.contains_key(sid).then(|| n.clone()))
        .collect();
    parse_and_transform(ctx.b.ast.allocator, source, &mutated, &rune_names)
}

fn parse_and_transform<'a>(
    alloc: &'a Allocator,
    source: &'a str,
    mutated: &HashSet<String>,
    rune_names: &HashSet<String>,
) -> Expression<'a> {
    let b = Builder::new(alloc);
    let Ok(expr) = OxcParser::new(alloc, source, SourceType::default()).parse_expression() else {
        return b.str_expr(source);
    };
    let stmt = b.expr_stmt(expr);
    let mut program = b.program(vec![stmt]);

    let mut tr = RuneRefTransformer { b: &b, mutated, rune_names };
    let sem = SemanticBuilder::new().build(&program);
    let (symbols, scopes) = sem.semantic.into_symbol_table_and_scope_tree();
    traverse_mut(&mut tr, alloc, &mut program, symbols, scopes);

    if let Some(Statement::ExpressionStatement(mut es)) = program.body.into_iter().next() {
        b.move_expr(&mut es.expression)
    } else {
        b.str_expr(source)
    }
}

struct RuneRefTransformer<'b, 'a> {
    b: &'b Builder<'a>,
    mutated: &'b HashSet<String>,
    rune_names: &'b HashSet<String>,
}

impl<'a> Traverse<'a> for RuneRefTransformer<'_, 'a> {
    fn enter_expression(&mut self, node: &mut Expression<'a>, _ctx: &mut TraverseCtx<'a>) {
        if let Expression::Identifier(id) = node {
            let name = id.name.as_str().to_string();
            if self.rune_names.contains(&name) && self.mutated.contains(&name) {
                *node = self.b.call_expr("$.get", [Arg::Ident(&name)]);
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Analysis helpers
// ---------------------------------------------------------------------------

fn item_needs_var(item: &FragmentItem, ctx: &Ctx<'_>) -> bool {
    match item {
        FragmentItem::TextConcat { parts } => parts.iter().any(|p| matches!(p, ConcatPart::Expr(_))),
        FragmentItem::Element(id) => element_needs_var(ctx, *id),
        FragmentItem::IfBlock(_) | FragmentItem::EachBlock(_) => true,
    }
}

fn element_needs_var(ctx: &Ctx<'_>, id: NodeId) -> bool {
    if ctx.analysis.node_needs_ref.contains(&id) {
        return true;
    }
    // Any non-static attribute (expression, shorthand, bind, etc.) needs a DOM ref
    let el = find_el_ref(ctx, id);
    let has_runtime_attrs = el.attributes.iter().any(|a| {
        !matches!(a, Attribute::StringAttribute(_) | Attribute::BooleanAttribute(_))
    });
    if has_runtime_attrs {
        return true;
    }
    let key = FragmentKey::Element(id);
    let ct = ctx.analysis.content_types.get(&key).copied().unwrap_or(ContentType::Empty);
    match ct {
        ContentType::Empty | ContentType::StaticText => false,
        ContentType::DynamicText | ContentType::SingleBlock => true,
        ContentType::SingleElement | ContentType::Mixed => {
            let Some(lf) = ctx.analysis.lowered_fragments.get(&key) else { return false };
            lf.items.iter().any(|item| item_needs_var(item, ctx))
        }
    }
}

fn item_is_dynamic(item: &FragmentItem, ctx: &Ctx<'_>) -> bool {
    match item {
        FragmentItem::TextConcat { parts } => parts_are_dynamic(parts, ctx),
        FragmentItem::Element(id) | FragmentItem::IfBlock(id) | FragmentItem::EachBlock(id) => {
            ctx.analysis.dynamic_nodes.contains(id)
        }
    }
}

fn parts_are_dynamic(parts: &[ConcatPart], ctx: &Ctx<'_>) -> bool {
    parts.iter().any(|p| {
        if let ConcatPart::Expr(id) = p {
            ctx.analysis.dynamic_nodes.contains(id)
        } else {
            false
        }
    })
}

fn has_dynamic_children(ctx: &Ctx<'_>, el_id: NodeId) -> bool {
    let key = FragmentKey::Element(el_id);
    let Some(lf) = ctx.analysis.lowered_fragments.get(&key) else { return false };
    lf.items.iter().any(|item| item_is_dynamic(item, ctx))
}

// ---------------------------------------------------------------------------
// AST lookups (immutable borrows of ctx)
// ---------------------------------------------------------------------------

fn find_el<'a>(ctx: &mut Ctx<'a>, id: NodeId) -> Element {
    find_el_ref(ctx, id).clone_el()
}

fn find_el_ref<'ctx>(ctx: &'ctx Ctx<'_>, id: NodeId) -> &'ctx Element {
    find_el_in_frag(&ctx.component.fragment, id).expect("element not found")
}

fn find_el_in_frag(frag: &svelte_ast::Fragment, id: NodeId) -> Option<&Element> {
    for n in &frag.nodes {
        match n {
            Node::Element(e) => {
                if e.id == id { return Some(e); }
                if let Some(f) = find_el_in_frag(&e.fragment, id) { return Some(f); }
            }
            Node::IfBlock(b) => {
                if let Some(f) = find_el_in_frag(&b.consequent, id) { return Some(f); }
                if let Some(a) = &b.alternate {
                    if let Some(f) = find_el_in_frag(a, id) { return Some(f); }
                }
            }
            Node::EachBlock(b) => {
                if let Some(f) = find_el_in_frag(&b.body, id) { return Some(f); }
                if let Some(fb) = &b.fallback {
                    if let Some(f) = find_el_in_frag(fb, id) { return Some(f); }
                }
            }
            _ => {}
        }
    }
    None
}

fn find_if<'ctx>(ctx: &'ctx Ctx<'_>, id: NodeId) -> &'ctx IfBlock {
    find_if_in_frag(&ctx.component.fragment, id).expect("if block not found")
}

fn find_if_in_frag(frag: &svelte_ast::Fragment, id: NodeId) -> Option<&IfBlock> {
    for n in &frag.nodes {
        match n {
            Node::IfBlock(b) => {
                if b.id == id { return Some(b); }
                if let Some(f) = find_if_in_frag(&b.consequent, id) { return Some(f); }
                if let Some(a) = &b.alternate {
                    if let Some(f) = find_if_in_frag(a, id) { return Some(f); }
                }
            }
            Node::Element(e) => {
                if let Some(f) = find_if_in_frag(&e.fragment, id) { return Some(f); }
            }
            _ => {}
        }
    }
    None
}

fn find_each<'ctx>(ctx: &'ctx Ctx<'_>, id: NodeId) -> &'ctx EachBlock {
    find_each_in_frag(&ctx.component.fragment, id).expect("each block not found")
}

fn find_each_in_frag(frag: &svelte_ast::Fragment, id: NodeId) -> Option<&EachBlock> {
    for n in &frag.nodes {
        match n {
            Node::EachBlock(b) => {
                if b.id == id { return Some(b); }
                if let Some(f) = find_each_in_frag(&b.body, id) { return Some(f); }
            }
            Node::Element(e) => {
                if let Some(f) = find_each_in_frag(&e.fragment, id) { return Some(f); }
            }
            Node::IfBlock(b) => {
                if let Some(f) = find_each_in_frag(&b.consequent, id) { return Some(f); }
                if let Some(a) = &b.alternate {
                    if let Some(f) = find_each_in_frag(a, id) { return Some(f); }
                }
            }
            _ => {}
        }
    }
    None
}

fn find_expr_span(ctx: &Ctx<'_>, id: NodeId) -> Span {
    find_expr_span_in(&ctx.component.fragment, id).expect("expr tag not found")
}

fn find_expr_span_in(frag: &svelte_ast::Fragment, id: NodeId) -> Option<Span> {
    for n in &frag.nodes {
        match n {
            Node::ExpressionTag(t) if t.id == id => return Some(t.expression_span),
            Node::Element(e) => {
                if let Some(s) = find_expr_span_in(&e.fragment, id) { return Some(s); }
            }
            Node::IfBlock(b) => {
                if let Some(s) = find_expr_span_in(&b.consequent, id) { return Some(s); }
                if let Some(a) = &b.alternate {
                    if let Some(s) = find_expr_span_in(a, id) { return Some(s); }
                }
            }
            Node::EachBlock(b) => {
                if let Some(s) = find_expr_span_in(&b.body, id) { return Some(s); }
            }
            _ => {}
        }
    }
    None
}

// ---------------------------------------------------------------------------
// Element clone helper
// ---------------------------------------------------------------------------

trait CloneEl {
    fn clone_el(&self) -> Element;
}

impl CloneEl for Element {
    fn clone_el(&self) -> Element {
        use svelte_ast::*;

        let attrs = self.attributes.iter().map(|a| match a {
            Attribute::StringAttribute(x) => Attribute::StringAttribute(StringAttribute {
                name: x.name.clone(),
                value_span: x.value_span,
            }),
            Attribute::BooleanAttribute(x) => {
                Attribute::BooleanAttribute(BooleanAttribute { name: x.name.clone() })
            }
            Attribute::ExpressionAttribute(x) => Attribute::ExpressionAttribute(ExpressionAttribute {
                name: x.name.clone(),
                expression_span: x.expression_span,
                shorthand: x.shorthand,
            }),
            Attribute::ConcatenationAttribute(x) => {
                Attribute::ConcatenationAttribute(ConcatenationAttribute {
                    name: x.name.clone(),
                    parts: x.parts.iter().map(|p| match p {
                        ConcatPart::Static(s) => ConcatPart::Static(s.clone()),
                        ConcatPart::Dynamic(sp) => ConcatPart::Dynamic(*sp),
                    }).collect(),
                })
            }
            Attribute::ShorthandOrSpread(x) => Attribute::ShorthandOrSpread(ShorthandOrSpread {
                expression_span: x.expression_span,
                is_spread: x.is_spread,
            }),
            Attribute::ClassDirective(x) => Attribute::ClassDirective(ClassDirective {
                name: x.name.clone(),
                expression_span: x.expression_span,
                shorthand: x.shorthand,
            }),
            Attribute::BindDirective(x) => Attribute::BindDirective(BindDirective {
                name: x.name.clone(),
                kind: x.kind,
                expression_span: x.expression_span,
                shorthand: x.shorthand,
            }),
        }).collect();

        Element {
            id: self.id,
            span: self.span,
            name: self.name.clone(),
            self_closing: self.self_closing,
            attributes: attrs,
            fragment: Fragment::empty(),
            kind: self.kind,
        }
    }
}
