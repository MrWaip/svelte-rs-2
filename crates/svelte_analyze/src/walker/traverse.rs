use super::*;

use super::dispatch::{
    dispatch_concat_exprs, dispatch_expr, dispatch_opt_expr, dispatch_opt_stmt, dispatch_stmt,
};

pub(crate) fn walk_template(
    fragment_id: svelte_ast::FragmentId,
    ctx: &mut VisitContext<'_, '_>,
    visitors: &mut [&mut dyn TemplateVisitor],
) {
    let fragment_nodes: Vec<NodeId> = ctx.store.fragment_nodes(fragment_id).to_vec();
    for (idx, id) in fragment_nodes.iter().copied().enumerate() {
        let node = ctx.store.get(id);
        let ignore_codes = scan_preceding_ignores(idx, &fragment_nodes, ctx);
        let has_ignores = !ignore_codes.is_empty();
        if has_ignores {
            ctx.push_ignore(ignore_codes);
        }

        let node_id = node_id_of(node);
        ctx.record_ignore_for_node(node_id);

        match node {
            Node::Text(text) => {
                for v in visitors.iter_mut() {
                    v.visit_text(text, ctx);
                }
            }
            Node::ExpressionTag(tag) => {
                for v in visitors.iter_mut() {
                    v.visit_expression_tag(tag, ctx);
                }
                dispatch_expr(visitors, tag.id, &tag.expression, ctx);
            }
            Node::Element(el) => {
                for v in visitors.iter_mut() {
                    v.visit_element(el, ctx);
                }
                let prev_name = ctx.replace_element_name(el.name.clone());
                ctx.push(ParentRef {
                    id: el.id,
                    kind: ParentKind::Element,
                });
                walk_attributes(&el.attributes, ctx, visitors);
                walk_template(el.fragment, ctx, visitors);
                ctx.pop();
                ctx.set_element_name(prev_name);
                for v in visitors.iter_mut() {
                    v.leave_element(el, ctx);
                }
            }
            Node::SlotElementLegacy(el) => {
                for v in visitors.iter_mut() {
                    v.visit_slot_element_legacy(el, ctx);
                }
                ctx.push(ParentRef {
                    id: el.id,
                    kind: ParentKind::SlotElementLegacy,
                });
                walk_attributes(&el.attributes, ctx, visitors);
                walk_template(el.fragment, ctx, visitors);
                ctx.pop();
                for v in visitors.iter_mut() {
                    v.leave_slot_element_legacy(el, ctx);
                }
            }
            Node::IfBlock(block) => {
                for v in visitors.iter_mut() {
                    v.visit_if_block(block, ctx);
                }
                dispatch_expr(visitors, block.id, &block.test, ctx);
                ctx.push(ParentRef {
                    id: block.id,
                    kind: ParentKind::IfBlock,
                });
                let saved = ctx.scope;
                ctx.scope = ctx.child_scope_by_id(block.consequent, saved);
                walk_template(block.consequent, ctx, visitors);
                if let Some(alt) = block.alternate {
                    ctx.scope = ctx.child_scope_by_id(alt, saved);
                    walk_template(alt, ctx, visitors);
                }
                ctx.scope = saved;
                ctx.pop();
            }
            Node::EachBlock(block) => {
                let body_scope = ctx.child_scope_by_id(block.body, ctx.scope);
                for v in visitors.iter_mut() {
                    v.visit_each_block(block, ctx);
                }
                dispatch_expr(visitors, block.id, &block.expression, ctx);
                ctx.push(ParentRef {
                    id: block.id,
                    kind: ParentKind::EachBlock,
                });
                let saved = ctx.scope;
                ctx.scope = body_scope;
                if let Some(ctx_ref) = block.context.as_ref() {
                    dispatch_stmt(visitors, block.id, ctx_ref, ctx);
                }
                dispatch_opt_stmt(visitors, block.id, block.index.as_ref(), ctx);
                if let (Some(key_id), Some(key_ref)) = (block.key_id, block.key.as_ref()) {
                    dispatch_expr(visitors, key_id, key_ref, ctx);
                }
                walk_template(block.body, ctx, visitors);
                ctx.scope = saved;
                if let Some(fb) = block.fallback {
                    walk_template(fb, ctx, visitors);
                }
                ctx.pop();
                for v in visitors.iter_mut() {
                    v.leave_each_block(block, ctx);
                }
            }
            Node::SnippetBlock(block) => {
                let body_scope = ctx.child_scope_by_id(block.body, ctx.scope);
                for v in visitors.iter_mut() {
                    v.visit_snippet_block(block, ctx);
                }
                dispatch_stmt(visitors, block.id, &block.decl, ctx);
                ctx.push(ParentRef {
                    id: block.id,
                    kind: ParentKind::SnippetBlock,
                });
                let saved = ctx.scope;
                ctx.scope = body_scope;
                walk_template(block.body, ctx, visitors);
                ctx.scope = saved;
                ctx.pop();
                for v in visitors.iter_mut() {
                    v.leave_snippet_block(block, ctx);
                }
            }
            Node::ComponentNode(cn) => {
                for v in visitors.iter_mut() {
                    v.visit_component_node(cn, ctx);
                }
                ctx.push(ParentRef {
                    id: cn.id,
                    kind: ParentKind::ComponentNode,
                });
                let saved = ctx.scope;
                let component_has_slot_attr =
                    attrs_static_slot_name(&cn.attributes, ctx.source).is_some();
                let default_scope = if component_has_slot_attr {
                    saved
                } else {
                    ctx.data
                        .scoping
                        .fragment_scope_by_id(cn.fragment)
                        .unwrap_or(saved)
                };

                for attr in &cn.attributes {
                    match attr {
                        Attribute::LetDirectiveLegacy(_) => {
                            ctx.scope = default_scope;
                            walk_attributes(std::slice::from_ref(attr), ctx, visitors);
                            ctx.scope = saved;
                        }
                        _ => walk_attributes(std::slice::from_ref(attr), ctx, visitors),
                    }
                }

                ctx.scope = default_scope;
                walk_template(cn.fragment, ctx, visitors);

                let slot_frags: Vec<svelte_ast::FragmentId> =
                    cn.legacy_slots.iter().map(|s| s.fragment).collect();
                for slot_fid in slot_frags {
                    ctx.scope = ctx
                        .data
                        .scoping
                        .fragment_scope_by_id(slot_fid)
                        .unwrap_or(saved);
                    walk_template(slot_fid, ctx, visitors);
                }
                ctx.scope = saved;
                ctx.pop();
            }
            Node::RenderTag(tag) => {
                for v in visitors.iter_mut() {
                    v.visit_render_tag(tag, ctx);
                }
                dispatch_expr(visitors, tag.id, &tag.expression, ctx);
            }
            Node::HtmlTag(tag) => {
                for v in visitors.iter_mut() {
                    v.visit_html_tag(tag, ctx);
                }
                dispatch_expr(visitors, tag.id, &tag.expression, ctx);
            }
            Node::ConstTag(tag) => {
                for v in visitors.iter_mut() {
                    v.visit_const_tag(tag, ctx);
                }
                dispatch_stmt(visitors, tag.id, &tag.decl, ctx);
            }
            Node::DebugTag(tag) => {
                for v in visitors.iter_mut() {
                    v.visit_debug_tag(tag, ctx);
                }
            }
            Node::KeyBlock(block) => {
                for v in visitors.iter_mut() {
                    v.visit_key_block(block, ctx);
                }
                dispatch_expr(visitors, block.id, &block.expression, ctx);
                ctx.push(ParentRef {
                    id: block.id,
                    kind: ParentKind::KeyBlock,
                });
                let saved = ctx.scope;
                ctx.scope = ctx.child_scope_by_id(block.fragment, saved);
                walk_template(block.fragment, ctx, visitors);
                ctx.scope = saved;
                ctx.pop();
            }
            Node::SvelteHead(head) => {
                ctx.push(ParentRef {
                    id: head.id,
                    kind: ParentKind::SvelteHead,
                });
                let saved = ctx.scope;
                ctx.scope = ctx.child_scope_by_id(head.fragment, saved);
                walk_template(head.fragment, ctx, visitors);
                ctx.scope = saved;
                ctx.pop();
            }
            Node::SvelteFragmentLegacy(el) => {
                for v in visitors.iter_mut() {
                    v.visit_svelte_fragment_legacy(el, ctx);
                }
                ctx.push(ParentRef {
                    id: el.id,
                    kind: ParentKind::SvelteFragmentLegacy,
                });
                walk_attributes(&el.attributes, ctx, visitors);
                walk_template(el.fragment, ctx, visitors);
                ctx.pop();
                for v in visitors.iter_mut() {
                    v.leave_svelte_fragment_legacy(el, ctx);
                }
            }
            Node::SvelteElement(el) => {
                for v in visitors.iter_mut() {
                    v.visit_svelte_element(el, ctx);
                }
                ctx.push(ParentRef {
                    id: el.id,
                    kind: ParentKind::SvelteElement,
                });
                if let Some(tag_ref) = el.tag.as_ref() {
                    dispatch_expr(visitors, el.id, tag_ref, ctx);
                }
                walk_attributes(&el.attributes, ctx, visitors);
                let saved = ctx.scope;
                ctx.scope = ctx.child_scope_by_id(el.fragment, saved);
                walk_template(el.fragment, ctx, visitors);
                ctx.scope = saved;
                ctx.pop();
                for v in visitors.iter_mut() {
                    v.leave_svelte_element(el, ctx);
                }
            }
            Node::SvelteWindow(w) => {
                for v in visitors.iter_mut() {
                    v.visit_svelte_window(w, ctx);
                }
                ctx.push(ParentRef {
                    id: w.id,
                    kind: ParentKind::SvelteWindow,
                });
                walk_attributes(&w.attributes, ctx, visitors);
                ctx.pop();
            }
            Node::SvelteDocument(d) => {
                for v in visitors.iter_mut() {
                    v.visit_svelte_document(d, ctx);
                }
                ctx.push(ParentRef {
                    id: d.id,
                    kind: ParentKind::SvelteDocument,
                });
                walk_attributes(&d.attributes, ctx, visitors);
                ctx.pop();
            }
            Node::SvelteBody(b) => {
                for v in visitors.iter_mut() {
                    v.visit_svelte_body(b, ctx);
                }
                ctx.push(ParentRef {
                    id: b.id,
                    kind: ParentKind::SvelteBody,
                });
                walk_attributes(&b.attributes, ctx, visitors);
                ctx.pop();
            }
            Node::SvelteBoundary(b) => {
                for v in visitors.iter_mut() {
                    v.visit_svelte_boundary(b, ctx);
                }
                ctx.push(ParentRef {
                    id: b.id,
                    kind: ParentKind::SvelteBoundary,
                });
                walk_attributes(&b.attributes, ctx, visitors);
                let saved = ctx.scope;
                ctx.scope = ctx.child_scope_by_id(b.fragment, saved);
                walk_template(b.fragment, ctx, visitors);
                ctx.scope = saved;
                ctx.pop();
            }
            Node::AwaitBlock(block) => {
                for v in visitors.iter_mut() {
                    v.visit_await_block(block, ctx);
                }
                dispatch_expr(visitors, block.id, &block.expression, ctx);
                ctx.push(ParentRef {
                    id: block.id,
                    kind: ParentKind::AwaitBlock,
                });
                let saved = ctx.scope;
                if let Some(p) = block.pending {
                    ctx.scope = ctx.child_scope_by_id(p, saved);
                    walk_template(p, ctx, visitors);
                }
                if let Some(t) = block.then {
                    ctx.scope = ctx.child_scope_by_id(t, saved);
                    dispatch_opt_stmt(visitors, block.id, block.value.as_ref(), ctx);
                    walk_template(t, ctx, visitors);
                }
                if let Some(c) = block.catch {
                    ctx.scope = ctx.child_scope_by_id(c, saved);
                    dispatch_opt_stmt(visitors, block.id, block.error.as_ref(), ctx);
                    walk_template(c, ctx, visitors);
                }
                ctx.scope = saved;
                ctx.pop();
            }
            Node::Comment(_) | Node::Error(_) => {}
        }

        if has_ignores {
            ctx.pop_ignore();
        }
    }
}

fn attrs_static_slot_name<'a>(attrs: &'a [Attribute], source: &'a str) -> Option<&'a str> {
    attrs.iter().find_map(|attr| match attr {
        Attribute::StringAttribute(attr) if attr.name == "slot" => {
            Some(attr.value_span.source_text(source))
        }
        _ => None,
    })
}

fn node_id_of(node: &Node) -> NodeId {
    match node {
        Node::ExpressionTag(n) => n.id,
        Node::Element(n) => n.id,
        Node::SlotElementLegacy(n) => n.id,
        Node::IfBlock(n) => n.id,
        Node::EachBlock(n) => n.id,
        Node::SnippetBlock(n) => n.id,
        Node::ComponentNode(n) => n.id,
        Node::RenderTag(n) => n.id,
        Node::HtmlTag(n) => n.id,
        Node::ConstTag(n) => n.id,
        Node::DebugTag(n) => n.id,
        Node::KeyBlock(n) => n.id,
        Node::SvelteHead(n) => n.id,
        Node::SvelteFragmentLegacy(n) => n.id,
        Node::SvelteElement(n) => n.id,
        Node::SvelteWindow(n) => n.id,
        Node::SvelteDocument(n) => n.id,
        Node::SvelteBody(n) => n.id,
        Node::SvelteBoundary(n) => n.id,
        Node::AwaitBlock(n) => n.id,
        Node::Text(n) => n.id,
        Node::Comment(n) => n.id,
        Node::Error(n) => n.id,
    }
}

fn scan_preceding_ignores(
    idx: usize,
    fragment_nodes: &[NodeId],
    ctx: &mut VisitContext<'_, '_>,
) -> Vec<String> {
    let mut codes = Vec::new();
    if idx == 0 {
        return codes;
    }

    let mut i = idx;
    while i > 0 {
        i -= 1;
        let prev_node = ctx.store.get(fragment_nodes[i]);
        match prev_node {
            Node::Comment(comment) => {
                let span = comment.span;
                let start = span.start as usize;
                let end = span.end as usize;
                if end - start > 7 {
                    let inner = &ctx.source[start + 4..end - 3];
                    let inner_offset = span.start + 4;
                    let result = extract_svelte_ignore::extract_svelte_ignore(
                        inner_offset,
                        inner,
                        ctx.runes,
                    );
                    if !result.codes.is_empty() {
                        codes.extend(result.codes);
                    }
                    ctx.warnings_mut().extend(result.warnings);
                }
            }
            Node::Text(_) => continue,
            _ => break,
        }
    }

    codes
}

fn walk_attributes(
    attrs: &[Attribute],
    ctx: &mut VisitContext<'_, '_>,
    visitors: &mut [&mut dyn TemplateVisitor],
) {
    for attr in attrs {
        for v in visitors.iter_mut() {
            v.visit_attribute(attr, ctx);
        }
        match attr {
            Attribute::ExpressionAttribute(a) => {
                for v in visitors.iter_mut() {
                    v.visit_expression_attribute(a, ctx);
                }
                ctx.push(ParentRef {
                    id: a.id,
                    kind: ParentKind::ExpressionAttribute,
                });
                dispatch_expr(visitors, a.id, &a.expression, ctx);
                ctx.pop();
            }
            Attribute::ConcatenationAttribute(a) => {
                for v in visitors.iter_mut() {
                    v.visit_concatenation_attribute(a, ctx);
                }
                ctx.push(ParentRef {
                    id: a.id,
                    kind: ParentKind::ConcatenationAttribute,
                });
                dispatch_concat_exprs(visitors, &a.parts, ctx);
                for v in visitors.iter_mut() {
                    v.leave_concatenation_attribute(a, ctx);
                }
                ctx.pop();
            }
            Attribute::SpreadAttribute(a) => {
                for v in visitors.iter_mut() {
                    v.visit_spread_attribute(a, ctx);
                }
                ctx.push(ParentRef {
                    id: a.id,
                    kind: ParentKind::SpreadAttribute,
                });
                dispatch_expr(visitors, a.id, &a.expression, ctx);
                ctx.pop();
            }
            Attribute::ClassDirective(a) => {
                for v in visitors.iter_mut() {
                    v.visit_class_directive(a, ctx);
                }
                ctx.push(ParentRef {
                    id: a.id,
                    kind: ParentKind::ClassDirective,
                });
                dispatch_expr(visitors, a.id, &a.expression, ctx);
                ctx.pop();
            }
            Attribute::StyleDirective(a) => {
                for v in visitors.iter_mut() {
                    v.visit_style_directive(a, ctx);
                }
                ctx.push(ParentRef {
                    id: a.id,
                    kind: ParentKind::StyleDirective,
                });
                match &a.value {
                    StyleDirectiveValue::Expression => {
                        dispatch_expr(visitors, a.id, &a.expression, ctx);
                    }
                    StyleDirectiveValue::Concatenation(parts) => {
                        dispatch_concat_exprs(visitors, parts, ctx);
                    }
                    StyleDirectiveValue::String(_) => {}
                }
                for v in visitors.iter_mut() {
                    v.leave_style_directive(a, ctx);
                }
                ctx.pop();
            }
            Attribute::BindDirective(a) => {
                for v in visitors.iter_mut() {
                    v.visit_bind_directive(a, ctx);
                }
                ctx.push(ParentRef {
                    id: a.id,
                    kind: ParentKind::BindDirective,
                });
                dispatch_expr(visitors, a.id, &a.expression, ctx);
                ctx.pop();
            }
            Attribute::LetDirectiveLegacy(a) => {
                for v in visitors.iter_mut() {
                    v.visit_let_directive_legacy(a, ctx);
                }
                ctx.push(ParentRef {
                    id: a.id,
                    kind: ParentKind::LetDirectiveLegacy,
                });

                if let Some(stmt_ref) = a.binding.as_ref() {
                    dispatch_stmt(visitors, a.id, stmt_ref, ctx);
                }
                ctx.pop();
            }
            Attribute::UseDirective(a) => {
                for v in visitors.iter_mut() {
                    v.visit_use_directive(a, ctx);
                }
                ctx.push(ParentRef {
                    id: a.id,
                    kind: ParentKind::UseDirective,
                });
                dispatch_opt_expr(visitors, a.id, a.expression.as_ref(), ctx);
                ctx.pop();
            }
            Attribute::OnDirectiveLegacy(a) => {
                for v in visitors.iter_mut() {
                    v.visit_on_directive_legacy(a, ctx);
                }
                ctx.push(ParentRef {
                    id: a.id,
                    kind: ParentKind::OnDirectiveLegacy,
                });
                dispatch_opt_expr(visitors, a.id, a.expression.as_ref(), ctx);
                ctx.pop();
            }
            Attribute::TransitionDirective(a) => {
                for v in visitors.iter_mut() {
                    v.visit_transition_directive(a, ctx);
                }
                ctx.push(ParentRef {
                    id: a.id,
                    kind: ParentKind::TransitionDirective,
                });
                dispatch_opt_expr(visitors, a.id, a.expression.as_ref(), ctx);
                ctx.pop();
            }
            Attribute::AnimateDirective(a) => {
                for v in visitors.iter_mut() {
                    v.visit_animate_directive(a, ctx);
                }
                ctx.push(ParentRef {
                    id: a.id,
                    kind: ParentKind::AnimateDirective,
                });
                dispatch_opt_expr(visitors, a.id, a.expression.as_ref(), ctx);
                ctx.pop();
            }
            Attribute::AttachTag(a) => {
                for v in visitors.iter_mut() {
                    v.visit_attach_tag(a, ctx);
                }
                ctx.push(ParentRef {
                    id: a.id,
                    kind: ParentKind::AttachTag,
                });
                dispatch_expr(visitors, a.id, &a.expression, ctx);
                ctx.pop();
            }
            Attribute::StringAttribute(_) | Attribute::BooleanAttribute(_) => {}
        }
    }
}
