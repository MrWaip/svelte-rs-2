//! Element processing (attributes, children).

use oxc_ast::ast::Statement;

use svelte_analyze::{ContentStrategy, FragmentItem, FragmentKey, LoweredTextPart};
use svelte_ast::NodeId;

use crate::builder::{Arg, AssignLeft};
use crate::context::Ctx;

use super::attributes::{
    process_attr, process_attrs_spread, process_class_attribute_and_directives,
    process_style_directives,
};
use super::each_block::gen_each_block;
use super::events::{
    gen_animate_directive, gen_attach_tag, gen_on_directive_legacy, gen_transition_directive,
    gen_use_directive,
};
use super::expression::{
    build_concat, build_fragment_local_blockers, emit_memoized_text_effect,
    emit_template_effect_with_memo, emit_trailing_next, get_node_expr, item_has_local_blockers,
    text_content_needs_memo, MemoAttr,
};
use super::from_template_fn_for_element;
use super::html::fragment_html;
use super::html_tag::gen_html_tag;
use super::traverse::traverse_items;

/// Process an element's attributes and children.
///
/// Statement ordering matches the Svelte reference compiler's two-state model:
/// - Simple attributes ($.set_attribute, etc.) → parent state → before children
/// - Directives ($.attach, $.action, $.bind_*) → element_state → after children
///
/// Merge order: simple attrs → children init → directive init → updates → after_update
/// Returns const-tag blocker expressions collected during child processing.
pub(crate) fn process_element<'a>(
    ctx: &mut Ctx<'a>,
    el_id: NodeId,
    el_name: &str,
    init: &mut Vec<Statement<'a>>,
    update: &mut Vec<Statement<'a>>,
    hoisted: &mut Vec<Statement<'a>>,
    after_update: &mut Vec<Statement<'a>>,
    memo_attrs: &mut Vec<MemoAttr<'a>>,
) {
    let child_key = FragmentKey::Element(el_id);
    let ct = ctx.content_type(&child_key);

    // $.remove_input_defaults for <input> elements with bind or dynamic value attribute
    if ctx.needs_input_defaults(el_id) {
        init.push(
            ctx.b
                .call_stmt("$.remove_input_defaults", [Arg::Ident(el_name)]),
        );
    }

    // --- Attributes (simple attrs go into init/update directly; directives into separate buffers) ---
    // Directive init/after_update are collected separately and merged after children.
    let mut directive_init: Vec<Statement<'a>> = Vec::new();
    let mut directive_after_update: Vec<Statement<'a>> = Vec::new();

    if ctx.has_spread(el_id) {
        let el = ctx.element(el_id);
        let el_tag = el.name.clone();
        let spread_attrs = el.attributes.clone();
        process_attrs_spread(
            ctx,
            el_id,
            &el_tag,
            &spread_attrs,
            el_name,
            true,
            false,
            init,
            after_update,
        );

        // Directives skipped by process_attrs_spread — handle them separately
        let el = ctx.element(el_id);
        let attrs: Vec<_> = el
            .attributes
            .iter()
            .filter_map(|attr| match attr {
                svelte_ast::Attribute::UseDirective(_)
                | svelte_ast::Attribute::OnDirectiveLegacy(_)
                | svelte_ast::Attribute::TransitionDirective(_)
                | svelte_ast::Attribute::AnimateDirective(_)
                | svelte_ast::Attribute::AttachTag(_) => Some((attr.clone(), attr.id())),
                _ => None,
            })
            .collect();
        for (attr, attr_id) in &attrs {
            match attr {
                svelte_ast::Attribute::UseDirective(ud) => {
                    gen_use_directive(ctx, ud, *attr_id, el_name, &mut directive_init);
                }
                svelte_ast::Attribute::OnDirectiveLegacy(od) => {
                    gen_on_directive_legacy(
                        ctx,
                        od,
                        *attr_id,
                        el_name,
                        &mut directive_after_update,
                    );
                }
                svelte_ast::Attribute::TransitionDirective(td) => {
                    gen_transition_directive(
                        ctx,
                        td,
                        *attr_id,
                        el_name,
                        &mut directive_after_update,
                    );
                }
                svelte_ast::Attribute::AnimateDirective(ad) => {
                    gen_animate_directive(ctx, ad, *attr_id, el_name, &mut directive_after_update);
                }
                svelte_ast::Attribute::AttachTag(_) => {
                    gen_attach_tag(ctx, *attr_id, el_name, &mut directive_init);
                }
                _ => unreachable!(),
            }
        }
    } else {
        let el = ctx.element(el_id);
        let has_use_directive = ctx.has_use_directive(el_id);
        let attr_dynamic: Vec<_> = el
            .attributes
            .iter()
            .map(|attr| ctx.is_dynamic_attr(attr.id()))
            .collect();

        // When there's a class expression attribute, Svelte emits class declarations
        // before event handler derived declarations (which are emitted during the attr loop).
        let has_class_attr = ctx.has_class_attribute(el_id);
        if has_class_attr {
            process_class_attribute_and_directives(ctx, el_id, el_name, init, update);
        }

        let el = ctx.element(el_id);
        let tag = el.name.clone();
        for (i, attr) in el.attributes.iter().enumerate() {
            process_attr(
                ctx,
                attr,
                el_name,
                el_id,
                &tag,
                attr_dynamic[i],
                has_use_directive,
                init,
                update,
                &mut directive_init,
                &mut directive_after_update,
                memo_attrs,
            );
        }
    }

    // Class directives only (no class expression attribute) — processed after attr loop
    if !ctx.has_spread(el_id) && !ctx.has_class_attribute(el_id) {
        process_class_attribute_and_directives(ctx, el_id, el_name, init, update);
    }

    // Style directives
    if !ctx.has_spread(el_id) {
        process_style_directives(ctx, el_id, el_name, init, update);
    }

    let is_selectedcontent = ctx.is_selectedcontent(el_id);

    // --- Children ---
    // Debug tags inside this element's fragment (before child DOM traversal)
    super::debug_tag::emit_debug_tags(ctx, child_key, init);

    let prev_bound_contenteditable = ctx.bound_contenteditable;
    if ctx.is_bound_contenteditable(el_id) {
        ctx.bound_contenteditable = true;
    }
    let has_state = ctx.has_dynamic_children(&child_key);

    if ctx.is_customizable_select(el_id) {
        emit_customizable_select(ctx, el_name, &child_key, init, hoisted);
    } else {
        match ct {
            ContentStrategy::Empty | ContentStrategy::Static(_) => {}

            ContentStrategy::DynamicText if !has_state => {
                let items: Vec<_> = ctx.lowered_fragment(&child_key).items.clone();

                if ctx.needs_textarea_value_lowering(el_id) {
                    // <textarea> with expression children: remove static child, set value property.
                    // Use the raw expression (bypasses build_concat constant folding) so the variable
                    // reference is preserved — $.set_value must receive the live binding, not a literal.
                    init.push(
                        ctx.b
                            .call_stmt("$.remove_textarea_child", [Arg::Ident(el_name)]),
                    );
                    let expr = extract_single_raw_expr_or_concat(ctx, &items[0]);
                    init.push(
                        ctx.b
                            .call_stmt("$.set_value", [Arg::Ident(el_name), Arg::Expr(expr)]),
                    );
                } else if !item_has_local_blockers(&items[0], ctx) {
                    // textContent shortcut
                    let expr = build_concat(ctx, &items[0]);
                    init.push(ctx.b.assign_stmt(
                        crate::builder::AssignLeft::StaticMember(
                            ctx.b.static_member(ctx.b.rid_expr(el_name), "textContent"),
                        ),
                        expr,
                    ));
                    // <option> with expression child and no explicit value attr: synthesize __value.
                    // Uses the raw expression (bypasses constant folding) so the variable reference
                    // is preserved for select-binding value comparisons at runtime.
                    if let Some(expr_id) = ctx.option_synthetic_value_expr(el_id) {
                        let raw_expr = get_node_expr(ctx, expr_id);
                        init.push(ctx.b.assign_stmt(
                            crate::builder::AssignLeft::StaticMember(
                                ctx.b.static_member(ctx.b.rid_expr(el_name), "__value"),
                            ),
                            raw_expr,
                        ));
                    }
                } else {
                    let text_name = ctx.gen_ident("text");
                    let child_call = ctx
                        .b
                        .call_expr("$.child", [Arg::Ident(el_name), Arg::Bool(true)]);
                    init.push(ctx.b.var_stmt(&text_name, child_call));
                    init.push(ctx.b.call_stmt("$.reset", [Arg::Ident(el_name)]));
                    let expr = build_concat(ctx, &items[0]);
                    update.push(
                        ctx.b
                            .call_stmt("$.set_text", [Arg::Ident(&text_name), Arg::Expr(expr)]),
                    );
                }
            }

            ContentStrategy::DynamicText => {
                let text_name = ctx.gen_ident("text");
                let items: Vec<_> = ctx.lowered_fragment(&child_key).items.clone();
                // is_text only for standalone {expression} (single Expr part, no surrounding text)
                let is_text = items.first().is_some_and(|item| item.is_standalone_expr());
                let child_call = if is_text {
                    ctx.b
                        .call_expr("$.child", [Arg::Ident(el_name), Arg::Bool(true)])
                } else {
                    ctx.b.call_expr("$.child", [Arg::Ident(el_name)])
                };
                init.push(ctx.b.var_stmt(&text_name, child_call));

                if ctx.bound_contenteditable {
                    let expr = build_concat(ctx, &items[0]);
                    // bound_contenteditable: nodeValue= in init instead of $.set_text() in update
                    init.push(ctx.b.assign_stmt(
                        crate::builder::AssignLeft::StaticMember(
                            ctx.b.static_member(ctx.b.rid_expr(&text_name), "nodeValue"),
                        ),
                        expr,
                    ));
                    init.push(ctx.b.call_stmt("$.reset", [Arg::Ident(el_name)]));
                } else {
                    init.push(ctx.b.call_stmt("$.reset", [Arg::Ident(el_name)]));
                    // Check if the expression needs call memoization (has_call + reactive refs)
                    let has_call = text_content_needs_memo(&items[0], ctx);
                    let has_async_value = if let FragmentItem::TextConcat { parts, .. } = &items[0]
                    {
                        parts.iter().any(|part| matches!(part, svelte_analyze::LoweredTextPart::Expr(id) if ctx.expr_has_await(*id)))
                    } else {
                        false
                    };
                    if has_call || has_async_value {
                        // Memoized form: $.template_effect(($0) => $.set_text(text, $0), [() => expr])
                        emit_memoized_text_effect(ctx, &items[0], &text_name, init);
                    } else {
                        let expr = build_concat(ctx, &items[0]);
                        if let FragmentItem::TextConcat { parts, .. } = &items[0] {
                            for part in parts {
                                if let svelte_analyze::LoweredTextPart::Expr(id) = part {
                                    let exprs = ctx.const_tag_blocker_exprs(*id);
                                    ctx.pending_const_blockers.extend(exprs);
                                }
                            }
                        }
                        update.push(
                            ctx.b
                                .call_stmt("$.set_text", [Arg::Ident(&text_name), Arg::Expr(expr)]),
                        );
                    }
                }
            }

            ContentStrategy::SingleBlock(FragmentItem::EachBlock(id)) => {
                // Controlled each block: element itself is the anchor, no $.child() traversal
                gen_each_block(ctx, id, ctx.b.rid_expr(el_name), true, init);
                init.push(ctx.b.call_stmt("$.reset", [Arg::Ident(el_name)]));
            }

            ContentStrategy::SingleBlock(FragmentItem::HtmlTag(id)) => {
                // Controlled html tag: element itself is the anchor
                gen_html_tag(ctx, id, ctx.b.rid_expr(el_name), true, init);
                init.push(ctx.b.call_stmt("$.reset", [Arg::Ident(el_name)]));
            }

            ContentStrategy::SingleElement(_)
            | ContentStrategy::SingleBlock(_)
            | ContentStrategy::Mixed { .. } => {
                // Clone needed: traverse_items borrows ctx mutably
                let child_items: Vec<_> = ctx.lowered_fragment(&child_key).items.clone();

                let first_is_text = child_items
                    .first()
                    .is_some_and(|item| item.is_standalone_expr());
                let first_child = if first_is_text {
                    ctx.b
                        .call_expr("$.child", [Arg::Ident(el_name), Arg::Bool(true)])
                } else {
                    ctx.b.call_expr("$.child", [Arg::Ident(el_name)])
                };
                let mut child_init = Vec::new();
                let mut child_update = Vec::new();
                let trailing = traverse_items(
                    ctx,
                    &child_items,
                    first_child,
                    &mut child_init,
                    &mut child_update,
                    hoisted,
                    after_update,
                    memo_attrs,
                );

                emit_trailing_next(ctx, trailing, &mut child_init);
                if !child_init.is_empty() {
                    child_init.push(ctx.b.call_stmt("$.reset", [Arg::Ident(el_name)]));
                }

                init.extend(child_init);
                update.extend(child_update);
            }
        }
    } // end else (non-customizable-select child path)

    ctx.bound_contenteditable = prev_bound_contenteditable;

    // <selectedcontent>: after children are processed, register a node-setter so the runtime
    // can swap in the cloned option content element.
    if is_selectedcontent {
        let assign = ctx.b.assign_expr(
            AssignLeft::Ident(el_name.to_string()),
            ctx.b.rid_expr("$$element"),
        );
        let setter = ctx
            .b
            .arrow_expr(ctx.b.params(["$$element"]), [ctx.b.expr_stmt(assign)]);
        init.push(ctx.b.call_stmt(
            "$.selectedcontent",
            [Arg::Ident(el_name), Arg::Expr(setter)],
        ));
    }

    // --- Merge directive statements after children (matching Svelte's element_state merge) ---
    init.extend(directive_init);
    after_update.extend(directive_after_update);
}

/// Check if a fragment item needs a DOM variable.
/// Elements use precomputed `element_flags.needs_var` from the analysis phase.
pub(crate) fn item_needs_var(item: &svelte_analyze::FragmentItem, ctx: &Ctx<'_>) -> bool {
    match item {
        svelte_analyze::FragmentItem::TextConcat { has_expr, .. } => *has_expr,
        svelte_analyze::FragmentItem::Element(id) => ctx.needs_var(*id),
        svelte_analyze::FragmentItem::ComponentNode(_)
        | svelte_analyze::FragmentItem::IfBlock(_)
        | svelte_analyze::FragmentItem::EachBlock(_)
        | svelte_analyze::FragmentItem::RenderTag(_)
        | svelte_analyze::FragmentItem::HtmlTag(_)
        | svelte_analyze::FragmentItem::KeyBlock(_)
        | svelte_analyze::FragmentItem::SvelteElement(_)
        | svelte_analyze::FragmentItem::SvelteBoundary(_)
        | svelte_analyze::FragmentItem::AwaitBlock(_) => true,
    }
}

/// Build a value expression from a fragment item without constant folding.
///
/// When a fragment item is a single expression tag, returns the raw node expression
/// (variable reference preserved). Falls back to `build_concat` for multi-part items.
/// Used where the live binding must be passed (e.g. `$.set_value` for textarea).
fn extract_single_raw_expr_or_concat<'a>(
    ctx: &mut Ctx<'a>,
    item: &FragmentItem,
) -> oxc_ast::ast::Expression<'a> {
    if let FragmentItem::TextConcat { parts, .. } = item {
        if let [LoweredTextPart::Expr(nid)] = parts.as_slice() {
            return get_node_expr(ctx, *nid);
        }
    }
    build_concat(ctx, item)
}

/// Emit `$.customizable_select(el, () => { ... })` for elements with rich DOM content.
///
/// Creates a separate hoisted template for the element's children, then wraps them in a
/// callback that clones the template, initialises content, and appends it as a child.
/// Port of the `is_customizable_select_element` branch in reference `RegularElement.js`.
fn emit_customizable_select<'a>(
    ctx: &mut Ctx<'a>,
    el_name: &str,
    child_key: &FragmentKey,
    init: &mut Vec<oxc_ast::ast::Statement<'a>>,
    hoisted: &mut Vec<oxc_ast::ast::Statement<'a>>,
) {
    let (children_html, import_node) = fragment_html(ctx, *child_key);
    let flags = if import_node { 3.0 } else { 1.0 };
    let from_fn = from_template_fn_for_element(ctx, el_name);
    let tpl = ctx.b.call_expr(
        from_fn,
        [
            Arg::Expr(ctx.b.template_str_expr(&children_html)),
            Arg::Num(flags),
        ],
    );
    let tpl_name = ctx.gen_ident(&format!("{el_name}_content"));
    hoisted.push(ctx.b.var_stmt(&tpl_name, tpl));

    let anchor_name = ctx.gen_ident("anchor");
    let fragment_name = ctx.gen_ident("fragment");
    let first_child = ctx
        .b
        .call_expr("$.first_child", [Arg::Ident(&fragment_name)]);

    let child_items = ctx.lowered_fragment(child_key).items.clone();
    let mut child_init: Vec<oxc_ast::ast::Statement<'a>> = Vec::new();
    let mut child_update: Vec<oxc_ast::ast::Statement<'a>> = Vec::new();
    let mut child_after_update: Vec<oxc_ast::ast::Statement<'a>> = Vec::new();
    let mut child_memo: Vec<MemoAttr<'a>> = Vec::new();
    let trailing = traverse_items(
        ctx,
        &child_items,
        first_child,
        &mut child_init,
        &mut child_update,
        hoisted,
        &mut child_after_update,
        &mut child_memo,
    );
    emit_trailing_next(ctx, trailing, &mut child_init);

    let el_blockers = ctx.fragment_blockers(child_key).to_vec();
    let local_blockers = build_fragment_local_blockers(ctx, child_key);

    let mut body: Vec<oxc_ast::ast::Statement<'a>> = vec![
        ctx.b.var_stmt(
            &anchor_name,
            ctx.b.call_expr("$.child", [Arg::Ident(el_name)]),
        ),
        ctx.b
            .var_stmt(&fragment_name, ctx.b.call_expr(&tpl_name, [])),
    ];
    body.extend(child_init);
    emit_template_effect_with_memo(
        ctx,
        child_update,
        child_memo,
        el_blockers,
        local_blockers,
        &mut body,
    );
    body.extend(child_after_update);
    body.push(ctx.b.call_stmt(
        "$.append",
        [Arg::Ident(&anchor_name), Arg::Ident(&fragment_name)],
    ));

    let callback = ctx.b.arrow_block_expr(ctx.b.no_params(), body);
    init.push(ctx.b.call_stmt(
        "$.customizable_select",
        [Arg::Ident(el_name), Arg::Expr(callback)],
    ));
}
