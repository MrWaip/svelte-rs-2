use super::*;

#[allow(unused_variables)]
pub(crate) trait TemplateVisitor {
    fn visit_text(&mut self, text: &svelte_ast::Text, ctx: &mut VisitContext<'_, '_>) {}
    fn visit_expression_tag(&mut self, tag: &ExpressionTag, ctx: &mut VisitContext<'_, '_>) {}
    fn visit_render_tag(&mut self, tag: &RenderTag, ctx: &mut VisitContext<'_, '_>) {}
    fn visit_html_tag(&mut self, tag: &HtmlTag, ctx: &mut VisitContext<'_, '_>) {}
    fn visit_const_tag(&mut self, tag: &ConstTag, ctx: &mut VisitContext<'_, '_>) {}
    fn visit_debug_tag(&mut self, tag: &DebugTag, ctx: &mut VisitContext<'_, '_>) {}
    fn visit_element(&mut self, el: &Element, ctx: &mut VisitContext<'_, '_>) {}
    fn visit_slot_element_legacy(&mut self, el: &SlotElementLegacy, ctx: &mut VisitContext<'_, '_>) {}
    fn visit_if_block(&mut self, block: &IfBlock, ctx: &mut VisitContext<'_, '_>) {}
    fn visit_each_block(&mut self, block: &EachBlock, ctx: &mut VisitContext<'_, '_>) {}
    fn visit_snippet_block(&mut self, block: &SnippetBlock, ctx: &mut VisitContext<'_, '_>) {}
    fn visit_key_block(&mut self, block: &KeyBlock, ctx: &mut VisitContext<'_, '_>) {}
    fn visit_component_node(&mut self, cn: &ComponentNode, ctx: &mut VisitContext<'_, '_>) {}
    fn visit_svelte_fragment_legacy(
        &mut self,
        el: &SvelteFragmentLegacy,
        ctx: &mut VisitContext<'_, '_>,
    ) {
    }
    fn visit_svelte_element(&mut self, el: &SvelteElement, ctx: &mut VisitContext<'_, '_>) {}
    fn visit_svelte_window(&mut self, w: &SvelteWindow, ctx: &mut VisitContext<'_, '_>) {}
    fn visit_svelte_document(&mut self, doc: &SvelteDocument, ctx: &mut VisitContext<'_, '_>) {}
    fn visit_svelte_body(&mut self, body: &SvelteBody, ctx: &mut VisitContext<'_, '_>) {}
    fn visit_svelte_boundary(&mut self, boundary: &SvelteBoundary, ctx: &mut VisitContext<'_, '_>) {}
    fn visit_await_block(&mut self, block: &AwaitBlock, ctx: &mut VisitContext<'_, '_>) {}

    fn visit_attribute(&mut self, attr: &Attribute, ctx: &mut VisitContext<'_, '_>) {}
    fn visit_expression_attribute(
        &mut self,
        attr: &ExpressionAttribute,
        ctx: &mut VisitContext<'_, '_>,
    ) {
    }
    fn visit_concatenation_attribute(
        &mut self,
        attr: &ConcatenationAttribute,
        ctx: &mut VisitContext<'_, '_>,
    ) {
    }
    fn visit_spread_attribute(&mut self, attr: &SpreadAttribute, ctx: &mut VisitContext<'_, '_>) {}
    fn visit_class_directive(&mut self, dir: &ClassDirective, ctx: &mut VisitContext<'_, '_>) {}
    fn visit_style_directive(&mut self, dir: &StyleDirective, ctx: &mut VisitContext<'_, '_>) {}
    fn visit_bind_directive(&mut self, dir: &BindDirective, ctx: &mut VisitContext<'_, '_>) {}
    fn visit_let_directive_legacy(&mut self, dir: &LetDirectiveLegacy, ctx: &mut VisitContext<'_, '_>) {
    }
    fn visit_use_directive(&mut self, dir: &UseDirective, ctx: &mut VisitContext<'_, '_>) {}
    fn visit_on_directive_legacy(&mut self, dir: &OnDirectiveLegacy, ctx: &mut VisitContext<'_, '_>) {}
    fn visit_transition_directive(
        &mut self,
        dir: &TransitionDirective,
        ctx: &mut VisitContext<'_, '_>,
    ) {
    }
    fn visit_animate_directive(&mut self, dir: &AnimateDirective, ctx: &mut VisitContext<'_, '_>) {}
    fn visit_attach_tag(&mut self, tag: &AttachTag, ctx: &mut VisitContext<'_, '_>) {}

    fn visit_expression(&mut self, node_id: NodeId, span: Span, ctx: &mut VisitContext<'_, '_>) {}
    fn leave_expression(&mut self, node_id: NodeId, span: Span, ctx: &mut VisitContext<'_, '_>) {}
    fn visit_statement(&mut self, node_id: NodeId, span: Span, ctx: &mut VisitContext<'_, '_>) {}
    fn leave_statement(&mut self, node_id: NodeId, span: Span, ctx: &mut VisitContext<'_, '_>) {}

    fn visit_js_expression(
        &mut self,
        node_id: NodeId,
        expr: &Expression<'_>,
        ctx: &mut VisitContext<'_, '_>,
    ) {
    }
    fn visit_js_statement(
        &mut self,
        node_id: NodeId,
        stmt: &Statement<'_>,
        ctx: &mut VisitContext<'_, '_>,
    ) {
    }

    fn leave_element(&mut self, el: &Element, ctx: &mut VisitContext<'_, '_>) {}
    fn leave_slot_element_legacy(&mut self, el: &SlotElementLegacy, ctx: &mut VisitContext<'_, '_>) {}
    fn leave_svelte_fragment_legacy(
        &mut self,
        el: &SvelteFragmentLegacy,
        ctx: &mut VisitContext<'_, '_>,
    ) {
    }
    fn leave_svelte_element(&mut self, el: &SvelteElement, ctx: &mut VisitContext<'_, '_>) {}
    fn leave_each_block(&mut self, block: &EachBlock, ctx: &mut VisitContext<'_, '_>) {}
    fn leave_snippet_block(&mut self, block: &SnippetBlock, ctx: &mut VisitContext<'_, '_>) {}
    fn leave_concatenation_attribute(
        &mut self,
        attr: &ConcatenationAttribute,
        ctx: &mut VisitContext<'_, '_>,
    ) {
    }
    fn leave_style_directive(&mut self, dir: &StyleDirective, ctx: &mut VisitContext<'_, '_>) {}
}
