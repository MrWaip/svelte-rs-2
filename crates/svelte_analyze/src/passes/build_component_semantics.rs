use oxc_ast::ast::{ArrowFunctionExpression, Expression, Statement};
use smallvec::smallvec;
use svelte_ast::{
    Attribute, AwaitBlock, BindDirective, ClassDirective, Component, Fragment, Node, NodeId,
    StyleDirective, StyleDirectiveValue,
};
use svelte_component_semantics::{
    ComponentSemanticsBuilder, ReferenceFlags, TemplateBuildContext, TemplateWalker,
};

use crate::scope::ComponentScoping;
use crate::types::data::{AnalysisData, FragmentKey, ParserResult};
use crate::utils::script_info;

pub(crate) fn build(component: &Component, parsed: &ParserResult<'_>, data: &mut AnalysisData) {
    let mut builder = ComponentSemanticsBuilder::new();

    if let Some(module_program) = parsed.module_program.as_ref() {
        builder.add_module_program(module_program);
    }
    data.script.module_node_id_offset = 0;
    data.script.instance_node_id_offset = builder.next_node_id();

    if let Some(program) = parsed.program.as_ref() {
        builder.add_instance_program(program);
    }

    let mut walker = AnalyzeTemplateWalker {
        component,
        parsed,
        data,
    };
    builder.add_template(&mut walker);

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

struct AnalyzeTemplateWalker<'a, 'b> {
    component: &'a Component,
    parsed: &'a ParserResult<'a>,
    data: &'b mut AnalysisData,
}

impl AnalyzeTemplateWalker<'_, '_> {
    fn walk_fragment(&mut self, fragment: &Fragment, ctx: &mut TemplateBuildContext<'_>) {
        for &id in &fragment.nodes {
            match self.component.store.get(id) {
                Node::Element(el) => {
                    self.walk_attributes(&el.attributes, ctx);
                    self.walk_fragment(&el.fragment, ctx);
                }
                Node::ComponentNode(node) => {
                    ctx.register_fragment_scope(FragmentKey::ComponentNode(node.id));
                    self.walk_attributes(&node.attributes, ctx);
                    self.walk_fragment(&node.fragment, ctx);
                }
                Node::ExpressionTag(tag) => {
                    self.record_expr_handle(tag.id, tag.expression_span.start, false);
                    if let Some(expr) = self
                        .parsed
                        .expr_handle(tag.expression_span.start)
                        .and_then(|handle| self.parsed.expr(handle))
                    {
                        ctx.visit_js_expression(expr);
                    }
                }
                Node::RenderTag(tag) => {
                    self.record_expr_handle(tag.id, tag.expression_span.start, false);
                    if let Some(expr) = self
                        .parsed
                        .expr_handle(tag.expression_span.start)
                        .and_then(|handle| self.parsed.expr(handle))
                    {
                        ctx.visit_js_expression(expr);
                    }
                }
                Node::HtmlTag(tag) => {
                    self.record_expr_handle(tag.id, tag.expression_span.start, false);
                    if let Some(expr) = self
                        .parsed
                        .expr_handle(tag.expression_span.start)
                        .and_then(|handle| self.parsed.expr(handle))
                    {
                        ctx.visit_js_expression(expr);
                    }
                }
                Node::ConstTag(tag) => {
                    self.record_expr_handle(tag.id, tag.expression_span.start, false);
                    if let Some(handle) = self.parsed.stmt_handle(tag.expression_span.start) {
                        self.data
                            .template
                            .template_semantics
                            .const_tag_stmt_handles
                            .insert(tag.id, handle);
                    }
                    if let Some(expr) = self
                        .parsed
                        .expr_handle(tag.expression_span.start)
                        .and_then(|handle| self.parsed.expr(handle))
                    {
                        ctx.visit_js_expression(expr);
                    }
                    if let Some(stmt) = self
                        .parsed
                        .stmt_handle(tag.expression_span.start)
                        .and_then(|handle| self.parsed.stmt(handle))
                    {
                        ctx.visit_js_statement(stmt);
                    }
                }
                Node::EachBlock(block) => self.walk_each_block(block, ctx),
                Node::IfBlock(block) => {
                    if let Some(expr) = self
                        .parsed
                        .expr_handle(block.test_span.start)
                        .and_then(|handle| self.parsed.expr(handle))
                    {
                        self.record_expr_handle(block.id, block.test_span.start, false);
                        ctx.visit_js_expression(expr);
                    }

                    ctx.enter_fragment_scope(FragmentKey::IfConsequent(block.id));
                    self.walk_fragment(&block.consequent, ctx);
                    ctx.leave_scope();

                    if let Some(alt) = &block.alternate {
                        ctx.enter_fragment_scope(FragmentKey::IfAlternate(block.id));
                        self.walk_fragment(alt, ctx);
                        ctx.leave_scope();
                    }
                }
                Node::SnippetBlock(block) => {
                    // The snippet name is declared in the surrounding template scope; only the
                    // params/body live in the snippet's child scope.
                    let mut arrow_scope = None;
                    if let Some(stmt) = self
                        .parsed
                        .stmt_handle(block.expression_span.start)
                        .and_then(|handle| self.parsed.stmt(handle))
                    {
                        self.data
                            .template
                            .template_semantics
                            .snippet_stmt_handles
                            .insert(
                                block.id,
                                self.parsed
                                    .stmt_handle(block.expression_span.start)
                                    .unwrap(),
                            );
                        ctx.visit_js_statement(stmt);
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
                        .set_fragment_scope(FragmentKey::SnippetBody(block.id), scope);
                    ctx.enter_scope(scope);
                    self.walk_fragment(&block.body, ctx);
                    ctx.leave_scope();
                }
                Node::KeyBlock(block) => {
                    self.record_expr_handle(block.id, block.expression_span.start, false);
                    if let Some(expr) = self
                        .parsed
                        .expr_handle(block.expression_span.start)
                        .and_then(|handle| self.parsed.expr(handle))
                    {
                        ctx.visit_js_expression(expr);
                    }
                    ctx.enter_fragment_scope(FragmentKey::KeyBlockBody(block.id));
                    self.walk_fragment(&block.fragment, ctx);
                    ctx.leave_scope();
                }
                Node::SvelteHead(head) => {
                    ctx.enter_fragment_scope(FragmentKey::SvelteHeadBody(head.id));
                    self.walk_fragment(&head.fragment, ctx);
                    ctx.leave_scope();
                }
                Node::SvelteElement(el) => {
                    self.record_expr_handle(el.id, el.tag_span.start, false);
                    if let Some(expr) = self
                        .parsed
                        .expr_handle(el.tag_span.start)
                        .and_then(|handle| self.parsed.expr(handle))
                    {
                        ctx.visit_js_expression(expr);
                    }
                    self.walk_attributes(&el.attributes, ctx);
                    ctx.enter_fragment_scope(FragmentKey::SvelteElementBody(el.id));
                    self.walk_fragment(&el.fragment, ctx);
                    ctx.leave_scope();
                }
                Node::SvelteBoundary(boundary) => {
                    ctx.enter_fragment_scope(FragmentKey::SvelteBoundaryBody(boundary.id));
                    self.walk_fragment(&boundary.fragment, ctx);
                    ctx.leave_scope();
                }
                Node::AwaitBlock(block) => self.walk_await_block(block, ctx),
                Node::SvelteWindow(node) => self.walk_attributes(&node.attributes, ctx),
                Node::SvelteDocument(node) => self.walk_attributes(&node.attributes, ctx),
                Node::SvelteBody(node) => self.walk_attributes(&node.attributes, ctx),
                Node::DebugTag(tag) => {
                    for span in &tag.identifiers {
                        self.record_expr_handle(tag.id, span.start, false);
                        if let Some(expr) = self
                            .parsed
                            .expr_handle(span.start)
                            .and_then(|handle| self.parsed.expr(handle))
                        {
                            ctx.visit_js_expression(expr);
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
        ctx: &mut TemplateBuildContext<'_>,
    ) {
        self.record_expr_handle(block.id, block.expression_span.start, false);
        if let Some(expr) = self
            .parsed
            .expr_handle(block.expression_span.start)
            .and_then(|handle| self.parsed.expr(handle))
        {
            ctx.visit_js_expression(expr);
        }

        ctx.enter_fragment_scope(FragmentKey::EachBody(block.id));
        if let Some(span) = block.context_span {
            if let Some(handle) = self.parsed.stmt_handle(span.start) {
                self.data
                    .template
                    .template_semantics
                    .each_context_stmt_handles
                    .insert(block.id, handle);
                if let Some(stmt) = self.parsed.stmt(handle) {
                    ctx.visit_js_statement(stmt);
                }
            }
        }
        if let Some(span) = block.index_span {
            if let Some(handle) = self.parsed.stmt_handle(span.start) {
                self.data
                    .template
                    .template_semantics
                    .each_index_stmt_handles
                    .insert(block.id, handle);
                if let Some(stmt) = self.parsed.stmt(handle) {
                    ctx.visit_js_statement(stmt);
                }
            }
        }
        if let Some(span) = block.key_span {
            self.record_expr_handle(block.key_id.unwrap_or(block.id), span.start, false);
            if let Some(expr) = self
                .parsed
                .expr_handle(span.start)
                .and_then(|handle| self.parsed.expr(handle))
            {
                ctx.visit_js_expression(expr);
            }
        }
        self.walk_fragment(&block.body, ctx);
        ctx.leave_scope();

        if let Some(fallback) = &block.fallback {
            self.walk_fragment(fallback, ctx);
        }
    }

    fn walk_await_block(&mut self, block: &AwaitBlock, ctx: &mut TemplateBuildContext<'_>) {
        self.record_expr_handle(block.id, block.expression_span.start, false);
        if let Some(expr) = self
            .parsed
            .expr_handle(block.expression_span.start)
            .and_then(|handle| self.parsed.expr(handle))
        {
            ctx.visit_js_expression(expr);
        }

        if let Some(pending) = &block.pending {
            ctx.enter_fragment_scope(FragmentKey::AwaitPending(block.id));
            self.walk_fragment(pending, ctx);
            ctx.leave_scope();
        }
        if let Some(then_block) = &block.then {
            ctx.enter_fragment_scope(FragmentKey::AwaitThen(block.id));
            if let Some(span) = block.value_span {
                if let Some(handle) = self.parsed.stmt_handle(span.start) {
                    self.data
                        .template
                        .template_semantics
                        .await_value_stmt_handles
                        .insert(block.id, handle);
                    if let Some(stmt) = self.parsed.stmt(handle) {
                        ctx.visit_js_statement(stmt);
                    }
                }
            }
            self.walk_fragment(then_block, ctx);
            ctx.leave_scope();
        }
        if let Some(catch_block) = &block.catch {
            ctx.enter_fragment_scope(FragmentKey::AwaitCatch(block.id));
            if let Some(span) = block.error_span {
                if let Some(handle) = self.parsed.stmt_handle(span.start) {
                    self.data
                        .template
                        .template_semantics
                        .await_error_stmt_handles
                        .insert(block.id, handle);
                    if let Some(stmt) = self.parsed.stmt(handle) {
                        ctx.visit_js_statement(stmt);
                    }
                }
            }
            self.walk_fragment(catch_block, ctx);
            ctx.leave_scope();
        }
    }

    fn walk_attributes(&mut self, attributes: &[Attribute], ctx: &mut TemplateBuildContext<'_>) {
        for attr in attributes {
            match attr {
                Attribute::ExpressionAttribute(attr) => {
                    self.record_expr_handle(attr.id, attr.expression_span.start, true);
                    if let Some(expr) = self
                        .parsed
                        .expr_handle(attr.expression_span.start)
                        .and_then(|handle| self.parsed.expr(handle))
                    {
                        ctx.visit_js_expression(expr);
                    }
                }
                Attribute::SpreadAttribute(attr) => {
                    self.record_expr_handle(attr.id, attr.expression_span.start, true);
                    if let Some(expr) = self
                        .parsed
                        .expr_handle(attr.expression_span.start)
                        .and_then(|handle| self.parsed.expr(handle))
                    {
                        ctx.visit_js_expression(expr);
                    }
                }
                Attribute::Shorthand(attr) => {
                    self.record_expr_handle(attr.id, attr.expression_span.start, true);
                    if let Some(expr) = self
                        .parsed
                        .expr_handle(attr.expression_span.start)
                        .and_then(|handle| self.parsed.expr(handle))
                    {
                        ctx.visit_js_expression(expr);
                    }
                }
                Attribute::ClassDirective(dir) => self.walk_class_directive(dir, ctx),
                Attribute::StyleDirective(dir) => self.walk_style_directive(dir, ctx),
                Attribute::BindDirective(dir) => self.walk_bind_directive(dir, ctx),
                Attribute::UseDirective(dir) => {
                    self.walk_optional_expr_attr(dir.id, dir.expression_span, ctx)
                }
                Attribute::TransitionDirective(dir) => {
                    self.walk_optional_expr_attr(dir.id, dir.expression_span, ctx)
                }
                Attribute::AnimateDirective(dir) => {
                    self.walk_optional_expr_attr(dir.id, dir.expression_span, ctx)
                }
                Attribute::AttachTag(tag) => {
                    self.record_expr_handle(tag.id, tag.expression_span.start, true);
                    if let Some(expr) = self
                        .parsed
                        .expr_handle(tag.expression_span.start)
                        .and_then(|handle| self.parsed.expr(handle))
                    {
                        ctx.visit_js_expression(expr);
                    }
                }
                Attribute::ConcatenationAttribute(attr) => {
                    for part in &attr.parts {
                        if let svelte_ast::ConcatPart::Dynamic { id, span } = part {
                            self.record_dynamic_expr(*id, span.start, ctx);
                        }
                    }
                }
                Attribute::StringAttribute(_)
                | Attribute::BooleanAttribute(_)
                | Attribute::OnDirectiveLegacy(_) => {}
            }
        }
    }

    fn walk_bind_directive(&mut self, dir: &BindDirective, ctx: &mut TemplateBuildContext<'_>) {
        if dir.shorthand {
            if let Some(sym_id) =
                ctx.materialize_shorthand_reference(dir.name.as_str(), ReferenceFlags::Write)
            {
                self.data
                    .template
                    .template_semantics
                    .node_ref_symbols
                    .insert(dir.id, smallvec![sym_id]);
            }
            return;
        }
        self.walk_optional_expr_attr_with_flags(
            dir.id,
            dir.expression_span,
            ReferenceFlags::Write,
            ctx,
        );
    }

    fn walk_class_directive(&mut self, dir: &ClassDirective, ctx: &mut TemplateBuildContext<'_>) {
        if let Some(span) = dir.expression_span {
            self.walk_optional_expr_attr(dir.id, Some(span), ctx);
        } else if let Some(sym_id) =
            ctx.materialize_shorthand_reference(dir.name.as_str(), ReferenceFlags::Read)
        {
            self.data
                .template
                .template_semantics
                .node_ref_symbols
                .insert(dir.id, smallvec![sym_id]);
        }
    }

    fn walk_style_directive(&mut self, dir: &StyleDirective, ctx: &mut TemplateBuildContext<'_>) {
        match &dir.value {
            StyleDirectiveValue::Expression(span) => {
                self.walk_optional_expr_attr(dir.id, Some(*span), ctx);
            }
            StyleDirectiveValue::Shorthand => {
                if let Some(sym_id) =
                    ctx.materialize_shorthand_reference(dir.name.as_str(), ReferenceFlags::Read)
                {
                    self.data
                        .template
                        .template_semantics
                        .node_ref_symbols
                        .insert(dir.id, smallvec![sym_id]);
                }
            }
            StyleDirectiveValue::Concatenation(parts) => {
                for part in parts {
                    if let svelte_ast::ConcatPart::Dynamic { id, span } = part {
                        self.record_dynamic_expr(*id, span.start, ctx);
                    }
                }
            }
            StyleDirectiveValue::String(_) => {}
        }
    }

    fn walk_optional_expr_attr(
        &mut self,
        node_id: NodeId,
        span: Option<svelte_span::Span>,
        ctx: &mut TemplateBuildContext<'_>,
    ) {
        self.walk_optional_expr_attr_with_flags(node_id, span, ReferenceFlags::Read, ctx);
    }

    fn walk_optional_expr_attr_with_flags(
        &mut self,
        node_id: NodeId,
        span: Option<svelte_span::Span>,
        flags: ReferenceFlags,
        ctx: &mut TemplateBuildContext<'_>,
    ) {
        let Some(span) = span else {
            return;
        };
        self.record_expr_handle(node_id, span.start, true);
        if let Some(expr) = self
            .parsed
            .expr_handle(span.start)
            .and_then(|handle| self.parsed.expr(handle))
        {
            if flags == ReferenceFlags::Read {
                ctx.visit_js_expression(expr);
            } else {
                ctx.visit_js_expression_with_flags(expr, flags);
            }
        }
    }

    fn record_dynamic_expr(
        &mut self,
        node_id: NodeId,
        offset: u32,
        ctx: &mut TemplateBuildContext<'_>,
    ) {
        self.record_expr_handle(node_id, offset, true);
        if let Some(expr) = self
            .parsed
            .expr_handle(offset)
            .and_then(|handle| self.parsed.expr(handle))
        {
            ctx.visit_js_expression(expr);
        }
    }

    fn record_expr_handle(&mut self, node_id: NodeId, offset: u32, is_attr: bool) {
        let Some(handle) = self.parsed.expr_handle(offset) else {
            return;
        };
        if is_attr {
            self.data
                .template
                .template_semantics
                .attr_expr_handles
                .insert(node_id, handle);
        } else {
            self.data
                .template
                .template_semantics
                .node_expr_handles
                .insert(node_id, handle);
        }
    }
}

impl TemplateWalker for AnalyzeTemplateWalker<'_, '_> {
    fn walk_template(&mut self, ctx: &mut TemplateBuildContext<'_>) {
        self.walk_fragment(&self.component.fragment, ctx);
    }
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
