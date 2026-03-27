use oxc_ast::ast::{Expression, Statement};
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

use crate::types::data::{AnalysisData, FragmentKey, ParserResult};

// ---------------------------------------------------------------------------
// ParentKind / ParentRef
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum ParentKind {
    // Template nodes
    Element,
    IfBlock,
    EachBlock,
    SnippetBlock,
    ComponentNode,
    KeyBlock,
    SvelteHead,
    SvelteElement,
    SvelteWindow,
    SvelteDocument,
    SvelteBody,
    SvelteBoundary,
    AwaitBlock,
    // Attributes
    ExpressionAttribute,
    ConcatenationAttribute,
    SpreadAttribute,
    Shorthand,
    ClassDirective,
    StyleDirective,
    BindDirective,
    UseDirective,
    OnDirectiveLegacy,
    TransitionDirective,
    AnimateDirective,
    AttachTag,
}

impl ParentKind {
    /// Element or SvelteElement — nodes that render a DOM element.
    pub fn is_element(&self) -> bool {
        matches!(self, Self::Element | Self::SvelteElement)
    }

    pub fn is_attr(&self) -> bool {
        matches!(
            self,
            Self::ExpressionAttribute
                | Self::ConcatenationAttribute
                | Self::SpreadAttribute
                | Self::Shorthand
                | Self::ClassDirective
                | Self::StyleDirective
                | Self::BindDirective
                | Self::UseDirective
                | Self::OnDirectiveLegacy
                | Self::TransitionDirective
                | Self::AnimateDirective
                | Self::AttachTag
        )
    }

    /// Directives that require a DOM element reference.
    /// Exhaustive match — adding a new variant forces a decision here.
    pub fn needs_element_ref(&self) -> bool {
        match self {
            Self::BindDirective
            | Self::UseDirective
            | Self::TransitionDirective
            | Self::AnimateDirective
            | Self::AttachTag => true,

            Self::Element
            | Self::IfBlock
            | Self::EachBlock
            | Self::SnippetBlock
            | Self::ComponentNode
            | Self::KeyBlock
            | Self::SvelteHead
            | Self::SvelteElement
            | Self::SvelteWindow
            | Self::SvelteDocument
            | Self::SvelteBody
            | Self::SvelteBoundary
            | Self::AwaitBlock
            | Self::ExpressionAttribute
            | Self::ConcatenationAttribute
            | Self::SpreadAttribute
            | Self::Shorthand
            | Self::ClassDirective
            | Self::StyleDirective
            | Self::OnDirectiveLegacy => false,
        }
    }

    /// Map Attribute variant → ParentKind. None for String/Boolean.
    /// Exhaustive match — new Attribute variants force an update.
    pub fn from_attr(attr: &Attribute) -> Option<Self> {
        match attr {
            Attribute::ExpressionAttribute(_) => Some(Self::ExpressionAttribute),
            Attribute::ConcatenationAttribute(_) => Some(Self::ConcatenationAttribute),
            Attribute::SpreadAttribute(_) => Some(Self::SpreadAttribute),
            Attribute::Shorthand(_) => Some(Self::Shorthand),
            Attribute::ClassDirective(_) => Some(Self::ClassDirective),
            Attribute::StyleDirective(_) => Some(Self::StyleDirective),
            Attribute::BindDirective(_) => Some(Self::BindDirective),
            Attribute::UseDirective(_) => Some(Self::UseDirective),
            Attribute::OnDirectiveLegacy(_) => Some(Self::OnDirectiveLegacy),
            Attribute::TransitionDirective(_) => Some(Self::TransitionDirective),
            Attribute::AnimateDirective(_) => Some(Self::AnimateDirective),
            Attribute::AttachTag(_) => Some(Self::AttachTag),
            Attribute::StringAttribute(_) | Attribute::BooleanAttribute(_) => None,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct ParentRef {
    pub id: NodeId,
    pub kind: ParentKind,
}

// ---------------------------------------------------------------------------
// VisitContext
// ---------------------------------------------------------------------------

/// Context passed to all TemplateVisitor methods.
///
/// Bundles scope, mutable analysis data, an optional parsed-expression store,
/// and a parent stack so visitors don't need to track parent context manually.
pub(crate) struct VisitContext<'a> {
    pub scope: ScopeId,
    pub data: &'a mut AnalysisData,
    /// Parsed JS expressions/statements. When set, the walker dispatches
    /// `visit_js_expression` / `visit_js_statement` after looking up the
    /// parsed AST node by span offset.
    parsed: Option<&'a ParserResult<'a>>,
    parents: Vec<ParentRef>,
    /// Tag name of the nearest enclosing Element (managed by walker).
    element_name: Option<String>,
}

impl<'a> VisitContext<'a> {
    pub fn new(scope: ScopeId, data: &'a mut AnalysisData) -> Self {
        Self {
            scope,
            data,
            parsed: None,
            parents: Vec::new(),
            element_name: None,
        }
    }

    pub fn with_parsed(
        scope: ScopeId,
        data: &'a mut AnalysisData,
        parsed: &'a ParserResult<'a>,
    ) -> Self {
        Self {
            scope,
            data,
            parsed: Some(parsed),
            parents: Vec::new(),
            element_name: None,
        }
    }

    /// Parsed JS expressions/statements, if set.
    /// Returns a copy of the inner reference — does NOT borrow self,
    /// so callers can use `ctx.data` mutably alongside the result.
    pub fn parsed(&self) -> Option<&'a ParserResult<'a>> {
        self.parsed
    }

    /// Immediate parent node/attribute, if any.
    pub fn parent(&self) -> Option<ParentRef> {
        self.parents.last().copied()
    }

    /// Ancestors from innermost to outermost.
    pub fn ancestors(&self) -> impl Iterator<Item = &ParentRef> {
        self.parents.iter().rev()
    }

    /// Tag name of the nearest enclosing Element (e.g. "div", "input").
    /// None when not inside an Element or inside a SvelteElement (dynamic tag).
    pub fn element_name(&self) -> Option<&str> {
        self.element_name.as_deref()
    }

    /// Nearest enclosing Element or SvelteElement in the parent stack, if any.
    pub fn nearest_element(&self) -> Option<NodeId> {
        self.ancestors()
            .find(|p| p.kind.is_element())
            .map(|p| p.id)
    }

    fn push(&mut self, r: ParentRef) {
        self.parents.push(r);
    }

    fn pop(&mut self) {
        self.parents.pop();
    }
}

// ---------------------------------------------------------------------------
// TemplateVisitor trait
// ---------------------------------------------------------------------------

/// Trait for template analysis visitors.
///
/// Each analysis pass implements only the visit methods it cares about.
/// All methods have default no-op implementations. The `walk_template` function
/// handles structural recursion, scope tracking, and parent stack — visitors
/// never need to recurse manually.
///
/// Pass multiple visitors as `&mut [&mut dyn TemplateVisitor]` to walk once.
#[allow(unused_variables)]
pub(crate) trait TemplateVisitor {
    // --- Node visits ---
    fn visit_expression_tag(&mut self, tag: &ExpressionTag, ctx: &mut VisitContext<'_>) {}
    fn visit_render_tag(&mut self, tag: &RenderTag, ctx: &mut VisitContext<'_>) {}
    fn visit_html_tag(&mut self, tag: &HtmlTag, ctx: &mut VisitContext<'_>) {}
    fn visit_const_tag(&mut self, tag: &ConstTag, ctx: &mut VisitContext<'_>) {}
    fn visit_debug_tag(&mut self, tag: &DebugTag, ctx: &mut VisitContext<'_>) {}
    fn visit_element(&mut self, el: &Element, ctx: &mut VisitContext<'_>) {}
    fn visit_if_block(&mut self, block: &IfBlock, ctx: &mut VisitContext<'_>) {}
    fn visit_each_block(&mut self, block: &EachBlock, ctx: &mut VisitContext<'_>) {}
    fn visit_snippet_block(&mut self, block: &SnippetBlock, ctx: &mut VisitContext<'_>) {}
    fn visit_key_block(&mut self, block: &KeyBlock, ctx: &mut VisitContext<'_>) {}
    fn visit_component_node(&mut self, cn: &ComponentNode, ctx: &mut VisitContext<'_>) {}
    fn visit_svelte_element(&mut self, el: &SvelteElement, ctx: &mut VisitContext<'_>) {}
    fn visit_svelte_window(&mut self, w: &SvelteWindow, ctx: &mut VisitContext<'_>) {}
    fn visit_svelte_document(&mut self, doc: &SvelteDocument, ctx: &mut VisitContext<'_>) {}
    fn visit_svelte_body(&mut self, body: &SvelteBody, ctx: &mut VisitContext<'_>) {}
    fn visit_svelte_boundary(&mut self, boundary: &SvelteBoundary, ctx: &mut VisitContext<'_>) {}
    fn visit_await_block(&mut self, block: &AwaitBlock, ctx: &mut VisitContext<'_>) {}

    // --- General attribute visit (fires for every attribute variant) ---
    fn visit_attribute(&mut self, attr: &Attribute, ctx: &mut VisitContext<'_>) {}

    // --- Attribute visits (dispatched for ALL nodes with attributes) ---
    fn visit_expression_attribute(&mut self, attr: &ExpressionAttribute, ctx: &mut VisitContext<'_>) {}
    fn visit_concatenation_attribute(&mut self, attr: &ConcatenationAttribute, ctx: &mut VisitContext<'_>) {}
    fn visit_spread_attribute(&mut self, attr: &SpreadAttribute, ctx: &mut VisitContext<'_>) {}
    fn visit_shorthand(&mut self, attr: &Shorthand, ctx: &mut VisitContext<'_>) {}
    fn visit_class_directive(&mut self, dir: &ClassDirective, ctx: &mut VisitContext<'_>) {}
    fn visit_style_directive(&mut self, dir: &StyleDirective, ctx: &mut VisitContext<'_>) {}
    fn visit_bind_directive(&mut self, dir: &BindDirective, ctx: &mut VisitContext<'_>) {}
    fn visit_use_directive(&mut self, dir: &UseDirective, ctx: &mut VisitContext<'_>) {}
    fn visit_on_directive_legacy(&mut self, dir: &OnDirectiveLegacy, ctx: &mut VisitContext<'_>) {}
    fn visit_transition_directive(&mut self, dir: &TransitionDirective, ctx: &mut VisitContext<'_>) {}
    fn visit_animate_directive(&mut self, dir: &AnimateDirective, ctx: &mut VisitContext<'_>) {}
    fn visit_attach_tag(&mut self, tag: &AttachTag, ctx: &mut VisitContext<'_>) {}

    // --- Expression / Statement visits ---
    fn visit_expression(&mut self, node_id: NodeId, span: Span, ctx: &mut VisitContext<'_>) {}
    fn leave_expression(&mut self, node_id: NodeId, span: Span, ctx: &mut VisitContext<'_>) {}
    fn visit_statement(&mut self, node_id: NodeId, span: Span, ctx: &mut VisitContext<'_>) {}
    fn leave_statement(&mut self, node_id: NodeId, span: Span, ctx: &mut VisitContext<'_>) {}

    // --- Parsed JS AST visits (only fire when VisitContext has parsed expressions) ---
    fn visit_js_expression(&mut self, node_id: NodeId, expr: &Expression<'_>, ctx: &mut VisitContext<'_>) {}
    fn visit_js_statement(&mut self, node_id: NodeId, stmt: &Statement<'_>, ctx: &mut VisitContext<'_>) {}

    // --- Leave hooks ---
    fn leave_element(&mut self, el: &Element, ctx: &mut VisitContext<'_>) {}
    fn leave_each_block(&mut self, block: &EachBlock, ctx: &mut VisitContext<'_>) {}
    fn leave_snippet_block(&mut self, block: &SnippetBlock, ctx: &mut VisitContext<'_>) {}
    fn leave_concatenation_attribute(&mut self, attr: &ConcatenationAttribute, ctx: &mut VisitContext<'_>) {}
    fn leave_style_directive(&mut self, dir: &StyleDirective, ctx: &mut VisitContext<'_>) {}
}

// ---------------------------------------------------------------------------
// Walk helpers
// ---------------------------------------------------------------------------

#[inline]
fn dispatch_expr(
    visitors: &mut [&mut dyn TemplateVisitor],
    id: NodeId,
    span: Span,
    ctx: &mut VisitContext<'_>,
) {
    // Copy parsed ref out so we can pass ctx mutably alongside expr
    let parsed = ctx.parsed;
    for v in visitors.iter_mut() { v.visit_expression(id, span, ctx); }
    if let Some(expr) = parsed.and_then(|p| p.exprs.get(&span.start)) {
        for v in visitors.iter_mut() { v.visit_js_expression(id, expr, ctx); }
    }
    for v in visitors.iter_mut() { v.leave_expression(id, span, ctx); }
}

#[inline]
fn dispatch_opt_expr(
    visitors: &mut [&mut dyn TemplateVisitor],
    id: NodeId,
    span: Option<Span>,
    ctx: &mut VisitContext<'_>,
) {
    if let Some(span) = span {
        dispatch_expr(visitors, id, span, ctx);
    }
}

#[inline]
fn dispatch_stmt(
    visitors: &mut [&mut dyn TemplateVisitor],
    id: NodeId,
    span: Span,
    ctx: &mut VisitContext<'_>,
) {
    let parsed = ctx.parsed;
    for v in visitors.iter_mut() { v.visit_statement(id, span, ctx); }
    if let Some(stmt) = parsed.and_then(|p| p.stmts.get(&span.start)) {
        for v in visitors.iter_mut() { v.visit_js_statement(id, stmt, ctx); }
    }
    for v in visitors.iter_mut() { v.leave_statement(id, span, ctx); }
}

#[inline]
fn dispatch_opt_stmt(
    visitors: &mut [&mut dyn TemplateVisitor],
    id: NodeId,
    span: Option<Span>,
    ctx: &mut VisitContext<'_>,
) {
    if let Some(span) = span {
        dispatch_stmt(visitors, id, span, ctx);
    }
}

fn dispatch_concat_exprs(
    visitors: &mut [&mut dyn TemplateVisitor],
    parts: &[ConcatPart],
    ctx: &mut VisitContext<'_>,
) {
    for part in parts {
        if let ConcatPart::Dynamic { id, span } = part {
            dispatch_expr(visitors, *id, *span, ctx);
        }
    }
}

// ---------------------------------------------------------------------------
// Walk
// ---------------------------------------------------------------------------

/// Walk a template fragment, dispatching to all visitors at each node.
///
/// Scope tracking and parent stack are managed via `ctx`. Visitors never need
/// to recurse manually.
pub(crate) fn walk_template(
    fragment: &Fragment,
    ctx: &mut VisitContext<'_>,
    visitors: &mut [&mut dyn TemplateVisitor],
) {
    for node in &fragment.nodes {
        match node {
            Node::ExpressionTag(tag) => {
                for v in visitors.iter_mut() { v.visit_expression_tag(tag, ctx); }
                dispatch_expr(visitors, tag.id, tag.expression_span, ctx);
            }
            Node::Element(el) => {
                for v in visitors.iter_mut() { v.visit_element(el, ctx); }
                let prev_name = ctx.element_name.replace(el.name.clone());
                ctx.push(ParentRef { id: el.id, kind: ParentKind::Element });
                walk_attributes(&el.attributes, ctx, visitors);
                walk_template(&el.fragment, ctx, visitors);
                ctx.pop();
                ctx.element_name = prev_name;
                for v in visitors.iter_mut() { v.leave_element(el, ctx); }
            }
            Node::IfBlock(block) => {
                for v in visitors.iter_mut() { v.visit_if_block(block, ctx); }
                dispatch_expr(visitors, block.id, block.test_span, ctx);
                ctx.push(ParentRef { id: block.id, kind: ParentKind::IfBlock });
                let saved = ctx.scope;
                let cons_scope = ctx.data.scoping.fragment_scope(&FragmentKey::IfConsequent(block.id)).unwrap_or(saved);
                ctx.scope = cons_scope;
                walk_template(&block.consequent, ctx, visitors);
                if let Some(alt) = &block.alternate {
                    let alt_scope = ctx.data.scoping.fragment_scope(&FragmentKey::IfAlternate(block.id)).unwrap_or(saved);
                    ctx.scope = alt_scope;
                    walk_template(alt, ctx, visitors);
                }
                ctx.scope = saved;
                ctx.pop();
            }
            Node::EachBlock(block) => {
                let body_scope = ctx.data.scoping.fragment_scope(&FragmentKey::EachBody(block.id)).unwrap_or(ctx.scope);
                for v in visitors.iter_mut() { v.visit_each_block(block, ctx); }
                dispatch_expr(visitors, block.id, block.expression_span, ctx);
                ctx.push(ParentRef { id: block.id, kind: ParentKind::EachBlock });
                let saved = ctx.scope;
                ctx.scope = body_scope;
                dispatch_stmt(visitors, block.id, block.context_span, ctx);
                dispatch_opt_stmt(visitors, block.id, block.index_span, ctx);
                if let (Some(key_id), Some(key_span)) = (block.key_id, block.key_span) {
                    dispatch_expr(visitors, key_id, key_span, ctx);
                }
                walk_template(&block.body, ctx, visitors);
                ctx.scope = saved;
                if let Some(fb) = &block.fallback {
                    walk_template(fb, ctx, visitors);
                }
                ctx.pop();
                for v in visitors.iter_mut() { v.leave_each_block(block, ctx); }
            }
            Node::SnippetBlock(block) => {
                let body_scope = ctx.data.scoping.fragment_scope(&FragmentKey::SnippetBody(block.id)).unwrap_or(ctx.scope);
                for v in visitors.iter_mut() { v.visit_snippet_block(block, ctx); }
                // const NAME = (PARAMS) => {} — dispatched in parent scope
                dispatch_stmt(visitors, block.id, block.expression_span, ctx);
                ctx.push(ParentRef { id: block.id, kind: ParentKind::SnippetBlock });
                let saved = ctx.scope;
                ctx.scope = body_scope;
                walk_template(&block.body, ctx, visitors);
                ctx.scope = saved;
                ctx.pop();
                for v in visitors.iter_mut() { v.leave_snippet_block(block, ctx); }
            }
            Node::ComponentNode(cn) => {
                for v in visitors.iter_mut() { v.visit_component_node(cn, ctx); }
                ctx.push(ParentRef { id: cn.id, kind: ParentKind::ComponentNode });
                walk_attributes(&cn.attributes, ctx, visitors);
                walk_template(&cn.fragment, ctx, visitors);
                ctx.pop();
            }
            Node::RenderTag(tag) => {
                for v in visitors.iter_mut() { v.visit_render_tag(tag, ctx); }
                dispatch_expr(visitors, tag.id, tag.expression_span, ctx);
            }
            Node::HtmlTag(tag) => {
                for v in visitors.iter_mut() { v.visit_html_tag(tag, ctx); }
                dispatch_expr(visitors, tag.id, tag.expression_span, ctx);
            }
            Node::ConstTag(tag) => {
                for v in visitors.iter_mut() { v.visit_const_tag(tag, ctx); }
                dispatch_stmt(visitors, tag.id, tag.expression_span, ctx);
            }
            Node::DebugTag(tag) => {
                for v in visitors.iter_mut() { v.visit_debug_tag(tag, ctx); }
            }
            Node::KeyBlock(block) => {
                for v in visitors.iter_mut() { v.visit_key_block(block, ctx); }
                dispatch_expr(visitors, block.id, block.expression_span, ctx);
                ctx.push(ParentRef { id: block.id, kind: ParentKind::KeyBlock });
                let saved = ctx.scope;
                let child_scope = ctx.data.scoping.fragment_scope(&FragmentKey::KeyBlockBody(block.id)).unwrap_or(saved);
                ctx.scope = child_scope;
                walk_template(&block.fragment, ctx, visitors);
                ctx.scope = saved;
                ctx.pop();
            }
            Node::SvelteHead(head) => {
                ctx.push(ParentRef { id: head.id, kind: ParentKind::SvelteHead });
                let saved = ctx.scope;
                let child_scope = ctx.data.scoping.fragment_scope(&FragmentKey::SvelteHeadBody(head.id)).unwrap_or(saved);
                ctx.scope = child_scope;
                walk_template(&head.fragment, ctx, visitors);
                ctx.scope = saved;
                ctx.pop();
            }
            Node::SvelteElement(el) => {
                for v in visitors.iter_mut() { v.visit_svelte_element(el, ctx); }
                ctx.push(ParentRef { id: el.id, kind: ParentKind::SvelteElement });
                if !el.static_tag {
                    dispatch_expr(visitors, el.id, el.tag_span, ctx);
                }
                walk_attributes(&el.attributes, ctx, visitors);
                let saved = ctx.scope;
                let child_scope = ctx.data.scoping.fragment_scope(&FragmentKey::SvelteElementBody(el.id)).unwrap_or(saved);
                ctx.scope = child_scope;
                walk_template(&el.fragment, ctx, visitors);
                ctx.scope = saved;
                ctx.pop();
            }
            Node::SvelteWindow(w) => {
                for v in visitors.iter_mut() { v.visit_svelte_window(w, ctx); }
                ctx.push(ParentRef { id: w.id, kind: ParentKind::SvelteWindow });
                walk_attributes(&w.attributes, ctx, visitors);
                ctx.pop();
            }
            Node::SvelteDocument(d) => {
                for v in visitors.iter_mut() { v.visit_svelte_document(d, ctx); }
                ctx.push(ParentRef { id: d.id, kind: ParentKind::SvelteDocument });
                walk_attributes(&d.attributes, ctx, visitors);
                ctx.pop();
            }
            Node::SvelteBody(b) => {
                for v in visitors.iter_mut() { v.visit_svelte_body(b, ctx); }
                ctx.push(ParentRef { id: b.id, kind: ParentKind::SvelteBody });
                walk_attributes(&b.attributes, ctx, visitors);
                ctx.pop();
            }
            Node::SvelteBoundary(b) => {
                for v in visitors.iter_mut() { v.visit_svelte_boundary(b, ctx); }
                ctx.push(ParentRef { id: b.id, kind: ParentKind::SvelteBoundary });
                walk_attributes(&b.attributes, ctx, visitors);
                let saved = ctx.scope;
                let child_scope = ctx.data.scoping.fragment_scope(&FragmentKey::SvelteBoundaryBody(b.id)).unwrap_or(saved);
                ctx.scope = child_scope;
                walk_template(&b.fragment, ctx, visitors);
                ctx.scope = saved;
                ctx.pop();
            }
            Node::AwaitBlock(block) => {
                for v in visitors.iter_mut() { v.visit_await_block(block, ctx); }
                dispatch_expr(visitors, block.id, block.expression_span, ctx);
                ctx.push(ParentRef { id: block.id, kind: ParentKind::AwaitBlock });
                let saved = ctx.scope;
                if let Some(ref p) = block.pending {
                    let pending_scope = ctx.data.scoping.fragment_scope(&FragmentKey::AwaitPending(block.id)).unwrap_or(saved);
                    ctx.scope = pending_scope;
                    walk_template(p, ctx, visitors);
                }
                if let Some(ref t) = block.then {
                    let then_scope = ctx.data.scoping.fragment_scope(&FragmentKey::AwaitThen(block.id)).unwrap_or(saved);
                    ctx.scope = then_scope;
                    dispatch_opt_stmt(visitors, block.id, block.value_span, ctx);
                    walk_template(t, ctx, visitors);
                }
                if let Some(ref c) = block.catch {
                    let catch_scope = ctx.data.scoping.fragment_scope(&FragmentKey::AwaitCatch(block.id)).unwrap_or(saved);
                    ctx.scope = catch_scope;
                    dispatch_opt_stmt(visitors, block.id, block.error_span, ctx);
                    walk_template(c, ctx, visitors);
                }
                ctx.scope = saved;
                ctx.pop();
            }
            Node::Text(_) | Node::Comment(_) | Node::Error(_) => {}
        }
    }
}

/// Walk attributes, dispatching per-variant visit and expression/statement hooks to all visitors.
fn walk_attributes(
    attrs: &[Attribute],
    ctx: &mut VisitContext<'_>,
    visitors: &mut [&mut dyn TemplateVisitor],
) {
    for attr in attrs {
        for v in visitors.iter_mut() { v.visit_attribute(attr, ctx); }
        match attr {
            Attribute::ExpressionAttribute(a) => {
                for v in visitors.iter_mut() { v.visit_expression_attribute(a, ctx); }
                ctx.push(ParentRef { id: a.id, kind: ParentKind::ExpressionAttribute });
                dispatch_expr(visitors, a.id, a.expression_span, ctx);
                ctx.pop();
            }
            Attribute::ConcatenationAttribute(a) => {
                for v in visitors.iter_mut() { v.visit_concatenation_attribute(a, ctx); }
                ctx.push(ParentRef { id: a.id, kind: ParentKind::ConcatenationAttribute });
                dispatch_concat_exprs(visitors, &a.parts, ctx);
                for v in visitors.iter_mut() { v.leave_concatenation_attribute(a, ctx); }
                ctx.pop();
            }
            Attribute::SpreadAttribute(a) => {
                for v in visitors.iter_mut() { v.visit_spread_attribute(a, ctx); }
                ctx.push(ParentRef { id: a.id, kind: ParentKind::SpreadAttribute });
                dispatch_expr(visitors, a.id, a.expression_span, ctx);
                ctx.pop();
            }
            Attribute::Shorthand(a) => {
                for v in visitors.iter_mut() { v.visit_shorthand(a, ctx); }
                ctx.push(ParentRef { id: a.id, kind: ParentKind::Shorthand });
                dispatch_expr(visitors, a.id, a.expression_span, ctx);
                ctx.pop();
            }
            Attribute::ClassDirective(a) => {
                for v in visitors.iter_mut() { v.visit_class_directive(a, ctx); }
                ctx.push(ParentRef { id: a.id, kind: ParentKind::ClassDirective });
                dispatch_opt_expr(visitors, a.id, a.expression_span, ctx);
                ctx.pop();
            }
            Attribute::StyleDirective(a) => {
                for v in visitors.iter_mut() { v.visit_style_directive(a, ctx); }
                ctx.push(ParentRef { id: a.id, kind: ParentKind::StyleDirective });
                match &a.value {
                    StyleDirectiveValue::Expression(span) => {
                        dispatch_expr(visitors, a.id, *span, ctx);
                    }
                    StyleDirectiveValue::Concatenation(parts) => {
                        dispatch_concat_exprs(visitors, parts, ctx);
                    }
                    StyleDirectiveValue::Shorthand | StyleDirectiveValue::String(_) => {}
                }
                for v in visitors.iter_mut() { v.leave_style_directive(a, ctx); }
                ctx.pop();
            }
            Attribute::BindDirective(a) => {
                for v in visitors.iter_mut() { v.visit_bind_directive(a, ctx); }
                ctx.push(ParentRef { id: a.id, kind: ParentKind::BindDirective });
                dispatch_opt_expr(visitors, a.id, a.expression_span, ctx);
                ctx.pop();
            }
            Attribute::UseDirective(a) => {
                for v in visitors.iter_mut() { v.visit_use_directive(a, ctx); }
                ctx.push(ParentRef { id: a.id, kind: ParentKind::UseDirective });
                dispatch_opt_expr(visitors, a.id, a.expression_span, ctx);
                ctx.pop();
            }
            Attribute::OnDirectiveLegacy(a) => {
                for v in visitors.iter_mut() { v.visit_on_directive_legacy(a, ctx); }
                ctx.push(ParentRef { id: a.id, kind: ParentKind::OnDirectiveLegacy });
                dispatch_opt_expr(visitors, a.id, a.expression_span, ctx);
                ctx.pop();
            }
            Attribute::TransitionDirective(a) => {
                for v in visitors.iter_mut() { v.visit_transition_directive(a, ctx); }
                ctx.push(ParentRef { id: a.id, kind: ParentKind::TransitionDirective });
                dispatch_opt_expr(visitors, a.id, a.expression_span, ctx);
                ctx.pop();
            }
            Attribute::AnimateDirective(a) => {
                for v in visitors.iter_mut() { v.visit_animate_directive(a, ctx); }
                ctx.push(ParentRef { id: a.id, kind: ParentKind::AnimateDirective });
                dispatch_opt_expr(visitors, a.id, a.expression_span, ctx);
                ctx.pop();
            }
            Attribute::AttachTag(a) => {
                for v in visitors.iter_mut() { v.visit_attach_tag(a, ctx); }
                ctx.push(ParentRef { id: a.id, kind: ParentKind::AttachTag });
                dispatch_expr(visitors, a.id, a.expression_span, ctx);
                ctx.pop();
            }
            Attribute::StringAttribute(_) | Attribute::BooleanAttribute(_) => {}
        }
    }
}
