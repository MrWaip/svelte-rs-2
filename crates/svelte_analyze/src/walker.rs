use oxc_semantic::ScopeId;
use svelte_ast::{
    AnimateDirective, AttachTag, Attribute, AwaitBlock, BindDirective, ClassDirective,
    ConcatPart, ConcatenationAttribute, ComponentNode, ConstTag, DebugTag, EachBlock, Element,
    ExpressionAttribute, ExpressionTag, Fragment, HtmlTag, IfBlock, KeyBlock, Node, NodeId,
    OnDirectiveLegacy, RenderTag, Shorthand, SnippetBlock, SpreadAttribute, StyleDirective,
    StyleDirectiveValue, SvelteBody, SvelteBoundary, SvelteDocument, SvelteElement, SvelteWindow,
    TransitionDirective, UseDirective,
};
use svelte_span::Span;

use crate::data::{AnalysisData, FragmentKey};

// ---------------------------------------------------------------------------
// TemplateVisitor trait
// ---------------------------------------------------------------------------

/// Trait for template analysis visitors.
///
/// Each analysis pass implements only the visit methods it cares about.
/// All methods have default no-op implementations. The `walk_template` function
/// handles structural recursion and scope tracking — visitors never need to
/// recurse manually.
///
/// Pass multiple visitors as `&mut [&mut dyn TemplateVisitor]` to walk once.
#[allow(unused_variables)]
pub(crate) trait TemplateVisitor {
    // --- Node visits ---
    fn visit_expression_tag(&mut self, tag: &ExpressionTag, scope: ScopeId, data: &mut AnalysisData) {}
    fn visit_render_tag(&mut self, tag: &RenderTag, scope: ScopeId, data: &mut AnalysisData) {}
    fn visit_html_tag(&mut self, tag: &HtmlTag, scope: ScopeId, data: &mut AnalysisData) {}
    fn visit_const_tag(&mut self, tag: &ConstTag, scope: ScopeId, data: &mut AnalysisData) {}
    fn visit_debug_tag(&mut self, tag: &DebugTag, scope: ScopeId, data: &mut AnalysisData) {}
    fn visit_element(&mut self, el: &Element, scope: ScopeId, data: &mut AnalysisData) {}
    fn visit_if_block(&mut self, block: &IfBlock, scope: ScopeId, data: &mut AnalysisData) {}
    fn visit_each_block(&mut self, block: &EachBlock, parent_scope: ScopeId, body_scope: ScopeId, data: &mut AnalysisData) {}
    fn visit_snippet_block(&mut self, block: &SnippetBlock, scope: ScopeId, data: &mut AnalysisData) {}
    fn visit_key_block(&mut self, block: &KeyBlock, scope: ScopeId, data: &mut AnalysisData) {}
    fn visit_component_node(&mut self, cn: &ComponentNode, scope: ScopeId, data: &mut AnalysisData) {}
    fn visit_svelte_element(&mut self, el: &SvelteElement, scope: ScopeId, data: &mut AnalysisData) {}
    fn visit_svelte_window(&mut self, w: &SvelteWindow, scope: ScopeId, data: &mut AnalysisData) {}
    fn visit_svelte_document(&mut self, doc: &SvelteDocument, scope: ScopeId, data: &mut AnalysisData) {}
    fn visit_svelte_body(&mut self, body: &SvelteBody, scope: ScopeId, data: &mut AnalysisData) {}
    fn visit_svelte_boundary(&mut self, boundary: &SvelteBoundary, scope: ScopeId, data: &mut AnalysisData) {}
    fn visit_await_block(&mut self, block: &AwaitBlock, scope: ScopeId, data: &mut AnalysisData) {}

    // --- Attribute visits (dispatched for ALL nodes with attributes) ---
    fn visit_expression_attribute(&mut self, attr: &ExpressionAttribute, scope: ScopeId, data: &mut AnalysisData) {}
    fn visit_concatenation_attribute(&mut self, attr: &ConcatenationAttribute, scope: ScopeId, data: &mut AnalysisData) {}
    fn visit_spread_attribute(&mut self, attr: &SpreadAttribute, scope: ScopeId, data: &mut AnalysisData) {}
    fn visit_shorthand(&mut self, attr: &Shorthand, scope: ScopeId, data: &mut AnalysisData) {}
    fn visit_class_directive(&mut self, dir: &ClassDirective, scope: ScopeId, data: &mut AnalysisData) {}
    fn visit_style_directive(&mut self, dir: &StyleDirective, scope: ScopeId, data: &mut AnalysisData) {}
    fn visit_bind_directive(&mut self, dir: &BindDirective, scope: ScopeId, data: &mut AnalysisData) {}
    fn visit_use_directive(&mut self, dir: &UseDirective, scope: ScopeId, data: &mut AnalysisData) {}
    fn visit_on_directive_legacy(&mut self, dir: &OnDirectiveLegacy, scope: ScopeId, data: &mut AnalysisData) {}
    fn visit_transition_directive(&mut self, dir: &TransitionDirective, scope: ScopeId, data: &mut AnalysisData) {}
    fn visit_animate_directive(&mut self, dir: &AnimateDirective, scope: ScopeId, data: &mut AnalysisData) {}
    fn visit_attach_tag(&mut self, tag: &AttachTag, scope: ScopeId, data: &mut AnalysisData) {}

    // --- Expression / Statement visits ---
    fn visit_expression(&mut self, node_id: NodeId, span: Span, scope: ScopeId, data: &mut AnalysisData) {}
    fn leave_expression(&mut self, node_id: NodeId, span: Span, scope: ScopeId, data: &mut AnalysisData) {}
    fn visit_statement(&mut self, node_id: NodeId, span: Span, scope: ScopeId, data: &mut AnalysisData) {}
    fn leave_statement(&mut self, node_id: NodeId, span: Span, scope: ScopeId, data: &mut AnalysisData) {}

    // --- Leave hooks ---
    fn leave_element(&mut self, el: &Element, scope: ScopeId, data: &mut AnalysisData) {}
    fn leave_each_block(&mut self, block: &EachBlock, parent_scope: ScopeId, body_scope: ScopeId, data: &mut AnalysisData) {}
    fn leave_snippet_block(&mut self, block: &SnippetBlock, scope: ScopeId, data: &mut AnalysisData) {}
}

// ---------------------------------------------------------------------------
// Walk
// ---------------------------------------------------------------------------

/// Dispatch `visit_expression` + `leave_expression` for a single span.
#[inline]
fn dispatch_expr(
    visitors: &mut [&mut dyn TemplateVisitor],
    id: NodeId,
    span: Span,
    scope: ScopeId,
    data: &mut AnalysisData,
) {
    for v in visitors.iter_mut() { v.visit_expression(id, span, scope, data); }
    for v in visitors.iter_mut() { v.leave_expression(id, span, scope, data); }
}

/// Dispatch `visit_expression` + `leave_expression` for an optional span.
#[inline]
fn dispatch_opt_expr(
    visitors: &mut [&mut dyn TemplateVisitor],
    id: NodeId,
    span: Option<Span>,
    scope: ScopeId,
    data: &mut AnalysisData,
) {
    if let Some(span) = span {
        dispatch_expr(visitors, id, span, scope, data);
    }
}

/// Dispatch `visit_statement` + `leave_statement` for a single span.
#[inline]
fn dispatch_stmt(
    visitors: &mut [&mut dyn TemplateVisitor],
    id: NodeId,
    span: Span,
    scope: ScopeId,
    data: &mut AnalysisData,
) {
    for v in visitors.iter_mut() { v.visit_statement(id, span, scope, data); }
    for v in visitors.iter_mut() { v.leave_statement(id, span, scope, data); }
}

/// Dispatch `visit_statement` + `leave_statement` for an optional span.
#[inline]
fn dispatch_opt_stmt(
    visitors: &mut [&mut dyn TemplateVisitor],
    id: NodeId,
    span: Option<Span>,
    scope: ScopeId,
    data: &mut AnalysisData,
) {
    if let Some(span) = span {
        dispatch_stmt(visitors, id, span, scope, data);
    }
}

/// Dispatch `visit_expression` + `leave_expression` for each dynamic part in a concatenation.
fn dispatch_concat_exprs(
    visitors: &mut [&mut dyn TemplateVisitor],
    id: NodeId,
    parts: &[ConcatPart],
    scope: ScopeId,
    data: &mut AnalysisData,
) {
    for part in parts {
        if let ConcatPart::Dynamic(span) = part {
            dispatch_expr(visitors, id, *span, scope, data);
        }
    }
}

/// Walk a template fragment, dispatching to all visitors at each node.
///
/// Scope tracking is built-in: EachBlock bodies use their child scope,
/// EachBlock fallbacks use the parent scope.
pub(crate) fn walk_template(
    fragment: &Fragment,
    data: &mut AnalysisData,
    scope: ScopeId,
    visitors: &mut [&mut dyn TemplateVisitor],
) {
    for node in &fragment.nodes {
        match node {
            Node::ExpressionTag(tag) => {
                for v in visitors.iter_mut() { v.visit_expression_tag(tag, scope, data); }
                dispatch_expr(visitors, tag.id, tag.expression_span, scope, data);
            }
            Node::Element(el) => {
                for v in visitors.iter_mut() { v.visit_element(el, scope, data); }
                walk_attributes(&el.attributes, scope, data, visitors);
                walk_template(&el.fragment, data, scope, visitors);
                for v in visitors.iter_mut() { v.leave_element(el, scope, data); }
            }
            Node::IfBlock(block) => {
                for v in visitors.iter_mut() { v.visit_if_block(block, scope, data); }
                dispatch_expr(visitors, block.id, block.test_span, scope, data);
                let cons_scope = data.scoping.fragment_scope(&FragmentKey::IfConsequent(block.id)).unwrap_or(scope);
                walk_template(&block.consequent, data, cons_scope, visitors);
                if let Some(alt) = &block.alternate {
                    let alt_scope = data.scoping.fragment_scope(&FragmentKey::IfAlternate(block.id)).unwrap_or(scope);
                    walk_template(alt, data, alt_scope, visitors);
                }
            }
            Node::EachBlock(block) => {
                let body_scope = data.scoping.node_scope(block.id).unwrap_or(scope);
                for v in visitors.iter_mut() { v.visit_each_block(block, scope, body_scope, data); }
                dispatch_expr(visitors, block.id, block.expression_span, scope, data);
                dispatch_stmt(visitors, block.id, block.context_span, body_scope, data);
                dispatch_opt_expr(visitors, block.id, block.key_span, body_scope, data);
                walk_template(&block.body, data, body_scope, visitors);
                if let Some(fb) = &block.fallback {
                    walk_template(fb, data, scope, visitors);
                }
                for v in visitors.iter_mut() { v.leave_each_block(block, scope, body_scope, data); }
            }
            Node::SnippetBlock(block) => {
                let body_scope = data.scoping.node_scope(block.id).unwrap_or(scope);
                for v in visitors.iter_mut() { v.visit_snippet_block(block, scope, data); }
                dispatch_opt_stmt(visitors, block.id, block.params_span, body_scope, data);
                walk_template(&block.body, data, body_scope, visitors);
                for v in visitors.iter_mut() { v.leave_snippet_block(block, scope, data); }
            }
            Node::ComponentNode(cn) => {
                for v in visitors.iter_mut() { v.visit_component_node(cn, scope, data); }
                walk_attributes(&cn.attributes, scope, data, visitors);
                walk_template(&cn.fragment, data, scope, visitors);
            }
            Node::RenderTag(tag) => {
                for v in visitors.iter_mut() { v.visit_render_tag(tag, scope, data); }
                dispatch_expr(visitors, tag.id, tag.expression_span, scope, data);
            }
            Node::HtmlTag(tag) => {
                for v in visitors.iter_mut() { v.visit_html_tag(tag, scope, data); }
                dispatch_expr(visitors, tag.id, tag.expression_span, scope, data);
            }
            Node::ConstTag(tag) => {
                for v in visitors.iter_mut() { v.visit_const_tag(tag, scope, data); }
                dispatch_stmt(visitors, tag.id, tag.expression_span, scope, data);
            }
            Node::DebugTag(tag) => {
                for v in visitors.iter_mut() { v.visit_debug_tag(tag, scope, data); }
            }
            Node::KeyBlock(block) => {
                for v in visitors.iter_mut() { v.visit_key_block(block, scope, data); }
                dispatch_expr(visitors, block.id, block.expression_span, scope, data);
                let child_scope = data.scoping.fragment_scope(&FragmentKey::KeyBlockBody(block.id)).unwrap_or(scope);
                walk_template(&block.fragment, data, child_scope, visitors);
            }
            Node::SvelteHead(head) => {
                let child_scope = data.scoping.fragment_scope(&FragmentKey::SvelteHeadBody(head.id)).unwrap_or(scope);
                walk_template(&head.fragment, data, child_scope, visitors);
            }
            Node::SvelteElement(el) => {
                for v in visitors.iter_mut() { v.visit_svelte_element(el, scope, data); }
                if !el.static_tag {
                    dispatch_expr(visitors, el.id, el.tag_span, scope, data);
                }
                walk_attributes(&el.attributes, scope, data, visitors);
                let child_scope = data.scoping.fragment_scope(&FragmentKey::SvelteElementBody(el.id)).unwrap_or(scope);
                walk_template(&el.fragment, data, child_scope, visitors);
            }
            Node::SvelteWindow(w) => {
                for v in visitors.iter_mut() { v.visit_svelte_window(w, scope, data); }
                walk_attributes(&w.attributes, scope, data, visitors);
            }
            Node::SvelteDocument(d) => {
                for v in visitors.iter_mut() { v.visit_svelte_document(d, scope, data); }
                walk_attributes(&d.attributes, scope, data, visitors);
            }
            Node::SvelteBody(b) => {
                for v in visitors.iter_mut() { v.visit_svelte_body(b, scope, data); }
                walk_attributes(&b.attributes, scope, data, visitors);
            }
            Node::SvelteBoundary(b) => {
                for v in visitors.iter_mut() { v.visit_svelte_boundary(b, scope, data); }
                walk_attributes(&b.attributes, scope, data, visitors);
                let child_scope = data.scoping.fragment_scope(&FragmentKey::SvelteBoundaryBody(b.id)).unwrap_or(scope);
                walk_template(&b.fragment, data, child_scope, visitors);
            }
            Node::AwaitBlock(block) => {
                for v in visitors.iter_mut() { v.visit_await_block(block, scope, data); }
                dispatch_expr(visitors, block.id, block.expression_span, scope, data);
                if let Some(ref p) = block.pending {
                    let pending_scope = data.scoping.fragment_scope(&FragmentKey::AwaitPending(block.id)).unwrap_or(scope);
                    walk_template(p, data, pending_scope, visitors);
                }
                if let Some(ref t) = block.then {
                    let then_scope = data.scoping.node_scope(block.id).unwrap_or(scope);
                    walk_template(t, data, then_scope, visitors);
                }
                if let Some(ref c) = block.catch {
                    let catch_scope = data.scoping.await_catch_scope(block.id).unwrap_or(scope);
                    walk_template(c, data, catch_scope, visitors);
                }
            }
            Node::Text(_) | Node::Comment(_) | Node::Error(_) => {}
        }
    }
}

/// Walk attributes, dispatching per-variant visit and expression/statement hooks to all visitors.
fn walk_attributes(
    attrs: &[Attribute],
    scope: ScopeId,
    data: &mut AnalysisData,
    visitors: &mut [&mut dyn TemplateVisitor],
) {
    for attr in attrs {
        match attr {
            Attribute::ExpressionAttribute(a) => {
                for v in visitors.iter_mut() { v.visit_expression_attribute(a, scope, data); }
                dispatch_expr(visitors, a.id, a.expression_span, scope, data);
            }
            Attribute::ConcatenationAttribute(a) => {
                for v in visitors.iter_mut() { v.visit_concatenation_attribute(a, scope, data); }
                dispatch_concat_exprs(visitors, a.id, &a.parts, scope, data);
            }
            Attribute::SpreadAttribute(a) => {
                for v in visitors.iter_mut() { v.visit_spread_attribute(a, scope, data); }
                dispatch_expr(visitors, a.id, a.expression_span, scope, data);
            }
            Attribute::Shorthand(a) => {
                for v in visitors.iter_mut() { v.visit_shorthand(a, scope, data); }
                dispatch_expr(visitors, a.id, a.expression_span, scope, data);
            }
            Attribute::ClassDirective(a) => {
                for v in visitors.iter_mut() { v.visit_class_directive(a, scope, data); }
                dispatch_opt_expr(visitors, a.id, a.expression_span, scope, data);
            }
            Attribute::StyleDirective(a) => {
                for v in visitors.iter_mut() { v.visit_style_directive(a, scope, data); }
                match &a.value {
                    StyleDirectiveValue::Expression(span) => {
                        dispatch_expr(visitors, a.id, *span, scope, data);
                    }
                    StyleDirectiveValue::Concatenation(parts) => {
                        dispatch_concat_exprs(visitors, a.id, parts, scope, data);
                    }
                    StyleDirectiveValue::Shorthand | StyleDirectiveValue::String(_) => {}
                }
            }
            Attribute::BindDirective(a) => {
                for v in visitors.iter_mut() { v.visit_bind_directive(a, scope, data); }
                dispatch_opt_expr(visitors, a.id, a.expression_span, scope, data);
            }
            Attribute::UseDirective(a) => {
                for v in visitors.iter_mut() { v.visit_use_directive(a, scope, data); }
                dispatch_opt_expr(visitors, a.id, a.expression_span, scope, data);
            }
            Attribute::OnDirectiveLegacy(a) => {
                for v in visitors.iter_mut() { v.visit_on_directive_legacy(a, scope, data); }
                dispatch_opt_expr(visitors, a.id, a.expression_span, scope, data);
            }
            Attribute::TransitionDirective(a) => {
                for v in visitors.iter_mut() { v.visit_transition_directive(a, scope, data); }
                dispatch_opt_expr(visitors, a.id, a.expression_span, scope, data);
            }
            Attribute::AnimateDirective(a) => {
                for v in visitors.iter_mut() { v.visit_animate_directive(a, scope, data); }
                dispatch_opt_expr(visitors, a.id, a.expression_span, scope, data);
            }
            Attribute::AttachTag(a) => {
                for v in visitors.iter_mut() { v.visit_attach_tag(a, scope, data); }
                dispatch_expr(visitors, a.id, a.expression_span, scope, data);
            }
            Attribute::StringAttribute(_) | Attribute::BooleanAttribute(_) => {}
        }
    }
}
