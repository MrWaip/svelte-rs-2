use oxc_ast::ast::{ArrowFunctionExpression, Expression, Statement};
use smallvec::smallvec;
use svelte_ast::{
    Attribute, AwaitBlock, BindDirective, ClassDirective, Component, Fragment, LetDirectiveLegacy,
    Node, NodeId, StyleDirective, StyleDirectiveValue,
};
use svelte_component_semantics::{
    ComponentSemanticsBuilder, ReferenceFlags, TemplateBuildContext, TemplateWalker,
};

use crate::scope::ComponentScoping;
use crate::types::data::{AnalysisData, JsAst};
use crate::utils::script_info;

pub(crate) fn build<'d, 'a>(
    component: &'d Component,
    parsed: &'d mut JsAst<'a>,
    data: &mut AnalysisData<'a>,
) {
    let mut builder = ComponentSemanticsBuilder::new();

    if let Some(module_program) = parsed.module_program.as_ref() {
        builder.add_module_program(module_program);
    }
    data.script.module_node_id_offset = 0;
    data.script.instance_node_id_offset = builder.next_node_id();

    if let Some(program) = parsed.program.as_ref() {
        builder.add_instance_program(program);
    }

    {
        let mut walker = AnalyzeTemplateWalker {
            component,
            parsed,
            data,
        };
        builder.add_template(&mut walker);
    }

    // Drain pending parser-staged expressions/statements into OxcNodeId-keyed
    // storage. The walker just bound every ExprRef.oxc_id via
    // TemplateBuildContext::visit_js_expression — collect those mappings here.
    let mut expr_id_map: rustc_hash::FxHashMap<u32, oxc_syntax::node::NodeId> =
        rustc_hash::FxHashMap::default();
    let mut stmt_id_map: rustc_hash::FxHashMap<u32, oxc_syntax::node::NodeId> =
        rustc_hash::FxHashMap::default();
    collect_ref_ids(component, &mut expr_id_map, &mut stmt_id_map);
    parsed.drain_pending(&expr_id_map, &stmt_id_map);

    let mut scoping = ComponentScoping::from_semantics(builder.finish());
    scoping.build_template_scope_set();

    if let Some(script) = data.script.info.as_mut() {
        script_info::enrich_from_component_scoping(&scoping, script);
        if let Some(program) = parsed.program.as_ref() {
            data.output.needs_context =
                crate::passes::js_analyze::needs_context_for_program(program, &scoping, script);
        }
    }

    if let Some(module_program) = parsed.module_program.as_ref() {
        if let Some(span) = parsed.module_script_content_span {
            let module_source = component.source_text(span);
            let mut module_info =
                script_info::extract_script_info(module_program, span.start, module_source, true);
            script_info::enrich_from_component_scoping(&scoping, &mut module_info);
            data.output.needs_context |= crate::passes::js_analyze::needs_context_for_program(
                module_program,
                &scoping,
                &module_info,
            );
        }
    }
    data.scoping = scoping;
}

struct AnalyzeTemplateWalker<'d, 'a, 'b> {
    component: &'d Component,
    parsed: &'d mut JsAst<'a>,
    data: &'b mut AnalysisData<'a>,
}

impl<'d, 'a, 'b> AnalyzeTemplateWalker<'d, 'a, 'b> {
    fn walk_fragment(&mut self, fragment: &Fragment, ctx: &mut TemplateBuildContext<'_, 'a>) {
        for &id in &fragment.nodes {
            match self.component.store.get(id) {
                Node::Element(el) => {
                    self.walk_attributes(&el.attributes, ctx);
                    self.walk_fragment(&el.fragment, ctx);
                }
                Node::SlotElementLegacy(el) => {
                    self.walk_attributes(&el.attributes, ctx);
                    self.walk_fragment(&el.fragment, ctx);
                }
                Node::ComponentNode(node) => self.walk_component_node(node, ctx),
                Node::ExpressionTag(tag) => {
                    if let Some(expr) = self.parsed.pending_expr(tag.expression.span.start) {
                        ctx.visit_js_expression(&tag.expression, expr);
                    }
                }
                Node::RenderTag(tag) => {
                    if let Some(expr) = self.parsed.pending_expr(tag.expression.span.start) {
                        ctx.visit_js_expression(&tag.expression, expr);
                    }
                }
                Node::HtmlTag(tag) => {
                    if let Some(expr) = self.parsed.pending_expr(tag.expression.span.start) {
                        ctx.visit_js_expression(&tag.expression, expr);
                    }
                }
                Node::ConstTag(tag) => {
                    if let Some(stmt) = self.parsed.pending_stmt(tag.decl.span.start) {
                        ctx.visit_js_statement(&tag.decl, stmt);
                    }
                }
                Node::EachBlock(block) => self.walk_each_block(block, ctx),
                Node::IfBlock(block) => {
                    if let Some(expr) = self.parsed.pending_expr(block.test.span.start) {
                        ctx.visit_js_expression(&block.test, expr);
                    }

                    ctx.enter_fragment_scope_by_id(block.consequent.id);
                    self.walk_fragment(&block.consequent, ctx);
                    ctx.leave_scope();

                    if let Some(alt) = &block.alternate {
                        ctx.enter_fragment_scope_by_id(alt.id);
                        self.walk_fragment(alt, ctx);
                        ctx.leave_scope();
                    }
                }
                Node::SnippetBlock(block) => {
                    let mut arrow_scope = None;
                    if let Some(stmt) = self.parsed.pending_stmt(block.decl.span.start) {
                        ctx.visit_js_statement(&block.decl, stmt);
                        // The arrow visitor created its own scope — read it back
                        if let Some(arrow) = extract_arrow_from_const(stmt) {
                            arrow_scope = arrow.scope_id.get();
                        }
                    }
                    let scope = arrow_scope.unwrap_or_else(|| {
                        let s = ctx.enter_child_scope();
                        ctx.leave_scope();
                        s
                    });
                    ctx.semantics_mut()
                        .set_fragment_scope_by_id(block.body.id, scope);
                    ctx.enter_scope(scope);
                    self.walk_fragment(&block.body, ctx);
                    ctx.leave_scope();
                }
                Node::KeyBlock(block) => {
                    if let Some(expr) = self.parsed.pending_expr(block.expression.span.start) {
                        ctx.visit_js_expression(&block.expression, expr);
                    }
                    ctx.enter_fragment_scope_by_id(block.fragment.id);
                    self.walk_fragment(&block.fragment, ctx);
                    ctx.leave_scope();
                }
                Node::SvelteHead(head) => {
                    ctx.enter_fragment_scope_by_id(head.fragment.id);
                    self.walk_fragment(&head.fragment, ctx);
                    ctx.leave_scope();
                }
                Node::SvelteFragmentLegacy(node) => {
                    self.walk_attributes(&node.attributes, ctx);
                    self.walk_fragment(&node.fragment, ctx);
                }
                Node::SvelteElement(el) => {
                    if let (Some(expr), Some(tag_ref)) =
                        (self.parsed.pending_expr(el.tag_span.start), el.tag.as_ref())
                    {
                        ctx.visit_js_expression(tag_ref, expr);
                    }
                    self.walk_attributes(&el.attributes, ctx);
                    ctx.enter_fragment_scope_by_id(el.fragment.id);
                    self.walk_fragment(&el.fragment, ctx);
                    ctx.leave_scope();
                }
                Node::SvelteBoundary(boundary) => {
                    self.walk_attributes(&boundary.attributes, ctx);
                    ctx.enter_fragment_scope_by_id(boundary.fragment.id);
                    self.walk_fragment(&boundary.fragment, ctx);
                    ctx.leave_scope();
                }
                Node::AwaitBlock(block) => self.walk_await_block(block, ctx),
                Node::SvelteWindow(node) => self.walk_attributes(&node.attributes, ctx),
                Node::SvelteDocument(node) => self.walk_attributes(&node.attributes, ctx),
                Node::SvelteBody(node) => self.walk_attributes(&node.attributes, ctx),
                Node::DebugTag(tag) => {
                    for ident_ref in &tag.identifier_refs {
                        if let Some(expr) = self.parsed.pending_expr(ident_ref.span.start) {
                            ctx.visit_js_expression(ident_ref, expr);
                        }
                    }
                }
                Node::Text(_) | Node::Comment(_) | Node::Error(_) => {}
            }
        }
    }

    fn walk_each_block(
        &mut self,
        block: &svelte_ast::EachBlock,
        ctx: &mut TemplateBuildContext<'_, 'a>,
    ) {
        if let Some(expr) = self.parsed.pending_expr(block.expression.span.start) {
            ctx.visit_js_expression(&block.expression, expr);
        }

        ctx.enter_fragment_scope_by_id(block.body.id);
        if let Some(ctx_ref) = block.context.as_ref() {
            if let Some(stmt) = self.parsed.pending_stmt(ctx_ref.span.start) {
                ctx.visit_js_statement(ctx_ref, stmt);
            }
        }
        if let Some(idx_ref) = block.index.as_ref() {
            if let Some(stmt) = self.parsed.pending_stmt(idx_ref.span.start) {
                ctx.visit_js_statement(idx_ref, stmt);
            }
        }
        if let Some(key_ref) = block.key.as_ref() {
            if let Some(expr) = self.parsed.pending_expr(key_ref.span.start) {
                ctx.visit_js_expression(key_ref, expr);
            }
        }
        self.walk_fragment(&block.body, ctx);
        ctx.leave_scope();

        if let Some(fallback) = &block.fallback {
            self.walk_fragment(fallback, ctx);
        }
    }

    fn walk_await_block(&mut self, block: &AwaitBlock, ctx: &mut TemplateBuildContext<'_, 'a>) {
        if let Some(expr) = self.parsed.pending_expr(block.expression.span.start) {
            ctx.visit_js_expression(&block.expression, expr);
        }

        if let Some(pending) = &block.pending {
            ctx.enter_fragment_scope_by_id(pending.id);
            self.walk_fragment(pending, ctx);
            ctx.leave_scope();
        }
        if let Some(then_block) = &block.then {
            ctx.enter_fragment_scope_by_id(then_block.id);
            if let Some(value_ref) = block.value.as_ref() {
                if let Some(stmt) = self.parsed.pending_stmt(value_ref.span.start) {
                    ctx.visit_js_statement(value_ref, stmt);
                }
            }
            self.walk_fragment(then_block, ctx);
            ctx.leave_scope();
        }
        if let Some(catch_block) = &block.catch {
            ctx.enter_fragment_scope_by_id(catch_block.id);
            if let Some(error_ref) = block.error.as_ref() {
                if let Some(stmt) = self.parsed.pending_stmt(error_ref.span.start) {
                    ctx.visit_js_statement(error_ref, stmt);
                }
            }
            self.walk_fragment(catch_block, ctx);
            ctx.leave_scope();
        }
    }

    fn walk_attributes(
        &mut self,
        attributes: &[Attribute],
        ctx: &mut TemplateBuildContext<'_, 'a>,
    ) {
        for attr in attributes {
            match attr {
                Attribute::ExpressionAttribute(attr) => {
                    if let Some(expr) = self.parsed.pending_expr(attr.expression.span.start) {
                        ctx.visit_js_expression(&attr.expression, expr);
                    }
                }
                Attribute::SpreadAttribute(attr) => {
                    if let Some(expr) = self.parsed.pending_expr(attr.expression.span.start) {
                        ctx.visit_js_expression(&attr.expression, expr);
                    }
                }
                Attribute::ClassDirective(dir) => self.walk_class_directive(dir, ctx),
                Attribute::StyleDirective(dir) => self.walk_style_directive(dir, ctx),
                Attribute::BindDirective(dir) => self.walk_bind_directive(dir, ctx),
                Attribute::LetDirectiveLegacy(dir) => self.declare_let_directive_legacy(dir, ctx),
                Attribute::UseDirective(dir) => {
                    if let Some(expr) = self.parsed.pending_expr(dir.name_ref.span.start) {
                        ctx.visit_js_expression(&dir.name_ref, expr);
                    }
                    self.walk_optional_expr_attr(
                        dir.id,
                        dir.expression.as_ref(),
                        dir.expression.as_ref().map(|r| r.span),
                        ctx,
                    );
                }
                Attribute::TransitionDirective(dir) => {
                    if let Some(expr) = self.parsed.pending_expr(dir.name_ref.span.start) {
                        ctx.visit_js_expression(&dir.name_ref, expr);
                    }
                    self.walk_optional_expr_attr(
                        dir.id,
                        dir.expression.as_ref(),
                        dir.expression.as_ref().map(|r| r.span),
                        ctx,
                    );
                }
                Attribute::AnimateDirective(dir) => {
                    if let Some(expr) = self.parsed.pending_expr(dir.name_ref.span.start) {
                        ctx.visit_js_expression(&dir.name_ref, expr);
                    }
                    self.walk_optional_expr_attr(
                        dir.id,
                        dir.expression.as_ref(),
                        dir.expression.as_ref().map(|r| r.span),
                        ctx,
                    );
                }
                Attribute::AttachTag(tag) => {
                    if let Some(expr) = self.parsed.pending_expr(tag.expression.span.start) {
                        ctx.visit_js_expression(&tag.expression, expr);
                    }
                }
                Attribute::ConcatenationAttribute(attr) => {
                    for part in &attr.parts {
                        if let svelte_ast::ConcatPart::Dynamic { id, expr } = part {
                            self.record_dynamic_expr(*id, expr.span.start, expr, ctx);
                        }
                    }
                }
                Attribute::OnDirectiveLegacy(dir) => {
                    if let Some(expr_ref) = dir.expression.as_ref() {
                        if let Some(expr) = self.parsed.pending_expr(expr_ref.span.start) {
                            ctx.visit_js_expression(expr_ref, expr);
                        }
                    }
                }
                Attribute::StringAttribute(_) | Attribute::BooleanAttribute(_) => {}
            }
        }
    }

    fn walk_component_node(
        &mut self,
        node: &svelte_ast::ComponentNode,
        ctx: &mut TemplateBuildContext<'_, 'a>,
    ) {
        let component_has_slot_attr =
            attrs_static_slot_name(&node.attributes, &self.component.source).is_some();
        let default_scope = if component_has_slot_attr {
            ctx.register_fragment_scope_by_id(node.fragment.id);
            ctx.current_scope()
        } else {
            let scope = ctx.enter_fragment_scope_by_id(node.fragment.id);
            ctx.leave_scope();
            scope
        };

        for attr in &node.attributes {
            match attr {
                Attribute::LetDirectiveLegacy(dir) => {
                    ctx.enter_scope(default_scope);
                    self.declare_let_directive_legacy(dir, ctx);
                    ctx.leave_scope();
                }
                _ => self.walk_attributes(std::slice::from_ref(attr), ctx),
            }
        }

        ctx.enter_scope(default_scope);
        self.walk_fragment(&node.fragment, ctx);
        ctx.leave_scope();

        for slot in &node.legacy_slots {
            let wrapper_id = slot.fragment.nodes[0];
            let scope = ctx.enter_named_slot_scope(node.id, wrapper_id);
            debug_assert_eq!(scope, ctx.current_scope());
            self.walk_fragment(&slot.fragment, ctx);
            ctx.leave_scope();
        }
    }

    fn walk_bind_directive(&mut self, dir: &BindDirective, ctx: &mut TemplateBuildContext<'_, 'a>) {
        // `expression_span` is non-optional; shorthand `bind:name` carries the
        // span of `name` which the parser already parsed as
        // `Expression::Identifier(name)`. Reference-compiler parity
        // (phases/scope.js::BindDirective):
        //   Identifier expression      → root identifier is the write-site
        //   (`binding.reassigned = true`) — mark as Write so OXC records a
        //   write reference.
        //   MemberExpression expression → root identifier reads the object
        //   (`binding.mutated = true` only) — we mark the symbol mutated
        //   outside the reference-flag channel via mark_symbol_mutated.
        let Some(expr) = self.parsed.pending_expr(dir.expression.span.start) else {
            return;
        };
        match expr {
            Expression::Identifier(_) => {
                ctx.visit_js_expression_with_flags(&dir.expression, expr, ReferenceFlags::Write);
            }
            Expression::StaticMemberExpression(_) | Expression::ComputedMemberExpression(_) => {
                ctx.visit_js_expression(&dir.expression, expr);
                if let Some(sym_id) = bind_member_root_symbol(expr, ctx) {
                    ctx.mark_symbol_member_mutated(sym_id);
                }
            }
            _ => {
                // Reference compiler accepts arbitrary expressions only for
                // sequence `bind:value={(get, set)}`. Other shapes are
                // rejected later in validation; we still visit as read.
                ctx.visit_js_expression(&dir.expression, expr);
            }
        }
        if let Some(sym_id) = attr_root_symbol(expr, ctx) {
            self.data
                .template
                .template_semantics
                .node_ref_symbols
                .insert(dir.id, smallvec![sym_id]);
        }
    }

    fn walk_class_directive(
        &mut self,
        dir: &ClassDirective,
        ctx: &mut TemplateBuildContext<'_, 'a>,
    ) {
        self.walk_expr_attr(dir.id, &dir.expression, dir.expression.span, ctx);
    }

    fn walk_style_directive(
        &mut self,
        dir: &StyleDirective,
        ctx: &mut TemplateBuildContext<'_, 'a>,
    ) {
        match &dir.value {
            StyleDirectiveValue::Expression => {
                self.walk_expr_attr(dir.id, &dir.expression, dir.expression.span, ctx);
            }
            StyleDirectiveValue::Concatenation(parts) => {
                for part in parts {
                    if let svelte_ast::ConcatPart::Dynamic { id, expr } = part {
                        self.record_dynamic_expr(*id, expr.span.start, expr, ctx);
                    }
                }
            }
            StyleDirectiveValue::String(_) => {}
        }
    }

    fn walk_expr_attr(
        &mut self,
        node_id: NodeId,
        expr_ref: &svelte_ast::ExprRef,
        span: svelte_span::Span,
        ctx: &mut TemplateBuildContext<'_, 'a>,
    ) {
        self.walk_expr_attr_with_flags(node_id, expr_ref, span, ReferenceFlags::Read, ctx);
    }

    fn walk_expr_attr_with_flags(
        &mut self,
        node_id: NodeId,
        expr_ref: &svelte_ast::ExprRef,
        span: svelte_span::Span,
        flags: ReferenceFlags,
        ctx: &mut TemplateBuildContext<'_, 'a>,
    ) {
        if let Some(expr) = self.parsed.pending_expr(span.start) {
            if flags == ReferenceFlags::Read {
                ctx.visit_js_expression(expr_ref, expr);
            } else {
                ctx.visit_js_expression_with_flags(expr_ref, expr, flags);
            }
            // Mirror the root-identifier `SymbolId` into `node_ref_symbols`
            // so lookups by `dir.id` (shorthand/class/style/bind) keep working
            // after the synthesized-expression rework. Consumers that need the
            // operation-level answer should prefer `reference_semantics(ref_id)`.
            if let Some(sym_id) = attr_root_symbol(expr, ctx) {
                self.data
                    .template
                    .template_semantics
                    .node_ref_symbols
                    .insert(node_id, smallvec![sym_id]);
            }
        }
    }

    /// Thin wrapper for directives that still carry `Option<Span>` and
    /// `Option<ExprRef>` (use/transition/animate).
    fn walk_optional_expr_attr(
        &mut self,
        node_id: NodeId,
        expr_ref: Option<&svelte_ast::ExprRef>,
        span: Option<svelte_span::Span>,
        ctx: &mut TemplateBuildContext<'_, 'a>,
    ) {
        if let (Some(span), Some(expr_ref)) = (span, expr_ref) {
            self.walk_expr_attr(node_id, expr_ref, span, ctx);
        }
    }

    fn record_dynamic_expr(
        &mut self,
        _node_id: NodeId,
        offset: u32,
        expr_ref: &svelte_ast::ExprRef,
        ctx: &mut TemplateBuildContext<'_, 'a>,
    ) {
        if let Some(expr) = self.parsed.pending_expr(offset) {
            ctx.visit_js_expression(expr_ref, expr);
        }
    }

    fn declare_let_directive_legacy(
        &mut self,
        dir: &LetDirectiveLegacy,
        ctx: &mut TemplateBuildContext<'_, 'a>,
    ) {
        if let Some(stmt) = self.parsed.pending_stmt(dir.name_span.start) {
            if let Some(binding_ref) = dir.binding.as_ref() {
                ctx.visit_js_statement(binding_ref, stmt);
            }
        }
    }
}

/// Drill down `member.object` chain to the root `IdentifierReference` and
/// resolve its `SymbolId` via the current template scope. Mirrors the
/// reference compiler's `utils/ast.js::object()` helper used by the
/// `BindDirective` updates pass in `phases/scope.js`.
fn bind_member_root_symbol<'a>(
    expr: &Expression<'a>,
    ctx: &TemplateBuildContext<'_, 'a>,
) -> Option<svelte_component_semantics::SymbolId> {
    attr_root_symbol(expr, ctx)
}

/// Resolve the root-identifier `SymbolId` of an attribute expression. Works
/// for any expression that eventually bottoms out in an `Identifier` after
/// drilling through member chains — the same shape reference compiler's
/// `utils/ast.js::object()` walks. Returns `None` for call expressions,
/// literals, or unresolved bindings.
fn attr_root_symbol<'a>(
    expr: &Expression<'a>,
    ctx: &TemplateBuildContext<'_, 'a>,
) -> Option<svelte_component_semantics::SymbolId> {
    let mut current = expr;
    loop {
        match current {
            Expression::StaticMemberExpression(m) => current = &m.object,
            Expression::ComputedMemberExpression(m) => current = &m.object,
            Expression::Identifier(ident) => {
                if let Some(ref_id) = ident.reference_id.get() {
                    return ctx.semantics().get_reference(ref_id).symbol_id();
                }
                return ctx
                    .semantics()
                    .find_binding(ctx.current_scope(), ident.name.as_str());
            }
            _ => return None,
        }
    }
}

fn attrs_static_slot_name<'a>(attributes: &'a [Attribute], source: &'a str) -> Option<&'a str> {
    attributes.iter().find_map(|attr| match attr {
        Attribute::StringAttribute(attr) if attr.name == "slot" => {
            Some(attr.value_span.source_text(source))
        }
        _ => None,
    })
}

impl<'d, 'a, 'b> TemplateWalker<'a> for AnalyzeTemplateWalker<'d, 'a, 'b> {
    fn walk_template(&mut self, ctx: &mut TemplateBuildContext<'_, 'a>) {
        self.walk_fragment(&self.component.fragment, ctx);
    }
}

/// Walk the template AST, collecting (span.start → ExprRef.id()) and
/// (span.start → StmtRef.id()) mappings for every bound ref. The walker
/// already filled every `ExprRef.oxc_id` Cell during the semantic pass
/// (`TemplateBuildContext::visit_js_expression`); we just read them back to
/// drive `JsAst::drain_pending`.
fn collect_ref_ids(
    component: &Component,
    expr_ids: &mut rustc_hash::FxHashMap<u32, oxc_syntax::node::NodeId>,
    stmt_ids: &mut rustc_hash::FxHashMap<u32, oxc_syntax::node::NodeId>,
) {
    fn record_expr(
        r: &svelte_ast::ExprRef,
        offset: u32,
        out: &mut rustc_hash::FxHashMap<u32, oxc_syntax::node::NodeId>,
    ) {
        let id = r.oxc_id.get();
        if id != oxc_syntax::node::NodeId::DUMMY {
            out.insert(offset, id);
        }
    }

    fn record_stmt(
        r: &svelte_ast::StmtRef,
        offset: u32,
        out: &mut rustc_hash::FxHashMap<u32, oxc_syntax::node::NodeId>,
    ) {
        let id = r.oxc_id.get();
        if id != oxc_syntax::node::NodeId::DUMMY {
            out.insert(offset, id);
        }
    }

    fn walk_attr(
        attr: &Attribute,
        expr_ids: &mut rustc_hash::FxHashMap<u32, oxc_syntax::node::NodeId>,
        stmt_ids: &mut rustc_hash::FxHashMap<u32, oxc_syntax::node::NodeId>,
    ) {
        match attr {
            Attribute::ExpressionAttribute(a) => {
                record_expr(&a.expression, a.expression.span.start, expr_ids);
            }
            Attribute::SpreadAttribute(a) => {
                record_expr(&a.expression, a.expression.span.start, expr_ids);
            }
            Attribute::ClassDirective(d) => {
                record_expr(&d.expression, d.expression.span.start, expr_ids);
            }
            Attribute::StyleDirective(d) => match &d.value {
                StyleDirectiveValue::Expression => {
                    record_expr(&d.expression, d.expression.span.start, expr_ids);
                }
                StyleDirectiveValue::Concatenation(parts) => {
                    for part in parts {
                        if let svelte_ast::ConcatPart::Dynamic { expr, .. } = part {
                            record_expr(expr, expr.span.start, expr_ids);
                        }
                    }
                }
                StyleDirectiveValue::String(_) => {}
            },
            Attribute::BindDirective(d) => {
                record_expr(&d.expression, d.expression.span.start, expr_ids);
            }
            Attribute::AttachTag(t) => {
                record_expr(&t.expression, t.expression.span.start, expr_ids);
            }
            Attribute::UseDirective(d) => {
                record_expr(&d.name_ref, d.name_ref.span.start, expr_ids);
                if let Some(r) = d.expression.as_ref() {
                    record_expr(r, r.span.start, expr_ids);
                }
            }
            Attribute::TransitionDirective(d) => {
                record_expr(&d.name_ref, d.name_ref.span.start, expr_ids);
                if let Some(r) = d.expression.as_ref() {
                    record_expr(r, r.span.start, expr_ids);
                }
            }
            Attribute::AnimateDirective(d) => {
                record_expr(&d.name_ref, d.name_ref.span.start, expr_ids);
                if let Some(r) = d.expression.as_ref() {
                    record_expr(r, r.span.start, expr_ids);
                }
            }
            Attribute::ConcatenationAttribute(a) => {
                for part in &a.parts {
                    if let svelte_ast::ConcatPart::Dynamic { expr, .. } = part {
                        record_expr(expr, expr.span.start, expr_ids);
                    }
                }
            }
            Attribute::OnDirectiveLegacy(d) => {
                if let Some(r) = d.expression.as_ref() {
                    record_expr(r, r.span.start, expr_ids);
                }
            }
            Attribute::LetDirectiveLegacy(d) => {
                if let Some(r) = d.binding.as_ref() {
                    record_stmt(r, d.name_span.start, stmt_ids);
                }
            }
            Attribute::StringAttribute(_) | Attribute::BooleanAttribute(_) => {}
        }
    }

    fn walk_fragment(
        component: &Component,
        fragment: &Fragment,
        expr_ids: &mut rustc_hash::FxHashMap<u32, oxc_syntax::node::NodeId>,
        stmt_ids: &mut rustc_hash::FxHashMap<u32, oxc_syntax::node::NodeId>,
    ) {
        for &id in &fragment.nodes {
            match component.store.get(id) {
                Node::Element(el) => {
                    for attr in &el.attributes {
                        walk_attr(attr, expr_ids, stmt_ids);
                    }
                    walk_fragment(component, &el.fragment, expr_ids, stmt_ids);
                }
                Node::SlotElementLegacy(el) => {
                    for attr in &el.attributes {
                        walk_attr(attr, expr_ids, stmt_ids);
                    }
                    walk_fragment(component, &el.fragment, expr_ids, stmt_ids);
                }
                Node::ComponentNode(cn) => {
                    for attr in &cn.attributes {
                        walk_attr(attr, expr_ids, stmt_ids);
                    }
                    walk_fragment(component, &cn.fragment, expr_ids, stmt_ids);
                    for slot in &cn.legacy_slots {
                        walk_fragment(component, &slot.fragment, expr_ids, stmt_ids);
                    }
                }
                Node::ExpressionTag(t) => {
                    record_expr(&t.expression, t.expression.span.start, expr_ids);
                }
                Node::RenderTag(t) => {
                    record_expr(&t.expression, t.expression.span.start, expr_ids);
                }
                Node::HtmlTag(t) => {
                    record_expr(&t.expression, t.expression.span.start, expr_ids);
                }
                Node::ConstTag(t) => {
                    record_stmt(&t.decl, t.decl.span.start, stmt_ids);
                }
                Node::IfBlock(block) => {
                    record_expr(&block.test, block.test.span.start, expr_ids);
                    walk_fragment(component, &block.consequent, expr_ids, stmt_ids);
                    if let Some(alt) = &block.alternate {
                        walk_fragment(component, alt, expr_ids, stmt_ids);
                    }
                }
                Node::EachBlock(block) => {
                    record_expr(&block.expression, block.expression.span.start, expr_ids);
                    if let Some(r) = block.context.as_ref() {
                        record_stmt(r, r.span.start, stmt_ids);
                    }
                    if let Some(r) = block.index.as_ref() {
                        record_stmt(r, r.span.start, stmt_ids);
                    }
                    if let Some(r) = block.key.as_ref() {
                        record_expr(r, r.span.start, expr_ids);
                    }
                    walk_fragment(component, &block.body, expr_ids, stmt_ids);
                    if let Some(fb) = &block.fallback {
                        walk_fragment(component, fb, expr_ids, stmt_ids);
                    }
                }
                Node::SnippetBlock(block) => {
                    record_stmt(&block.decl, block.decl.span.start, stmt_ids);
                    walk_fragment(component, &block.body, expr_ids, stmt_ids);
                }
                Node::KeyBlock(block) => {
                    record_expr(&block.expression, block.expression.span.start, expr_ids);
                    walk_fragment(component, &block.fragment, expr_ids, stmt_ids);
                }
                Node::SvelteHead(head) => {
                    walk_fragment(component, &head.fragment, expr_ids, stmt_ids);
                }
                Node::SvelteFragmentLegacy(node) => {
                    for attr in &node.attributes {
                        walk_attr(attr, expr_ids, stmt_ids);
                    }
                    walk_fragment(component, &node.fragment, expr_ids, stmt_ids);
                }
                Node::SvelteElement(el) => {
                    if let Some(tag_ref) = el.tag.as_ref() {
                        record_expr(tag_ref, el.tag_span.start, expr_ids);
                    }
                    for attr in &el.attributes {
                        walk_attr(attr, expr_ids, stmt_ids);
                    }
                    walk_fragment(component, &el.fragment, expr_ids, stmt_ids);
                }
                Node::SvelteBoundary(b) => {
                    for attr in &b.attributes {
                        walk_attr(attr, expr_ids, stmt_ids);
                    }
                    walk_fragment(component, &b.fragment, expr_ids, stmt_ids);
                }
                Node::AwaitBlock(block) => {
                    record_expr(&block.expression, block.expression.span.start, expr_ids);
                    if let Some(p) = &block.pending {
                        walk_fragment(component, p, expr_ids, stmt_ids);
                    }
                    if let Some(t) = &block.then {
                        if let Some(r) = block.value.as_ref() {
                            record_stmt(r, r.span.start, stmt_ids);
                        }
                        walk_fragment(component, t, expr_ids, stmt_ids);
                    }
                    if let Some(c) = &block.catch {
                        if let Some(r) = block.error.as_ref() {
                            record_stmt(r, r.span.start, stmt_ids);
                        }
                        walk_fragment(component, c, expr_ids, stmt_ids);
                    }
                }
                Node::SvelteWindow(node) => {
                    for attr in &node.attributes {
                        walk_attr(attr, expr_ids, stmt_ids);
                    }
                }
                Node::SvelteDocument(node) => {
                    for attr in &node.attributes {
                        walk_attr(attr, expr_ids, stmt_ids);
                    }
                }
                Node::SvelteBody(node) => {
                    for attr in &node.attributes {
                        walk_attr(attr, expr_ids, stmt_ids);
                    }
                }
                Node::DebugTag(tag) => {
                    for r in &tag.identifier_refs {
                        record_expr(r, r.span.start, expr_ids);
                    }
                }
                Node::Text(_) | Node::Comment(_) | Node::Error(_) => {}
            }
        }
    }

    walk_fragment(component, &component.fragment, expr_ids, stmt_ids);
}

fn extract_arrow_from_const<'a>(
    stmt: &'a Statement<'a>,
) -> Option<&'a ArrowFunctionExpression<'a>> {
    let Statement::VariableDeclaration(decl) = stmt else {
        return None;
    };
    let declarator = decl.declarations.first()?;
    let Expression::ArrowFunctionExpression(arrow) = declarator.init.as_ref()? else {
        return None;
    };
    Some(arrow)
}
