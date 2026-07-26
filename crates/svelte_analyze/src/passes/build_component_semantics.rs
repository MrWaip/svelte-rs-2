use oxc_ast::ast::{ArrowFunctionExpression, Expression, Statement};
use oxc_syntax::node::NodeId as OxcNodeId;
use rustc_hash::FxHashMap;
use smallvec::smallvec;
use std::slice;
use svelte_ast::{
    Attribute, AwaitBlock, BindDirective, ClassDirective, Component, LetDirectiveLegacy, Node,
    NodeId, SLOT_ATTRIBUTE, StyleDirective, StyleDirectiveValue,
};
use svelte_component_semantics::SymbolId;
use svelte_component_semantics::{
    ComponentSemanticsBuilder, ReferenceFlags, TemplateBuildContext, TemplateWalker,
};

use crate::scope::ComponentScoping;
use crate::types::data::{AnalysisData, JsAst};

pub(crate) fn build<'d, 'a>(
    component: &'d Component,
    parsed: &'d mut JsAst<'a>,
    data: &mut AnalysisData<'a>,
) {
    let node_count = component.node_count() as usize;
    let mut builder = ComponentSemanticsBuilder::with_capacity(node_count);

    if let Some(module_program) = parsed.module_program.as_ref() {
        builder.add_module_program(module_program);
    }

    if let Some(program) = parsed.program.as_ref() {
        builder.add_instance_program(program);
    }

    let expr_id_map: FxHashMap<u32, OxcNodeId> =
        FxHashMap::with_capacity_and_hasher(node_count, Default::default());
    let stmt_id_map: FxHashMap<u32, OxcNodeId> =
        FxHashMap::with_capacity_and_hasher(node_count, Default::default());

    let (expr_id_map, stmt_id_map) = {
        let mut walker = AnalyzeTemplateWalker {
            store: &component.store,
            source: &component.source,
            root: component.root,
            parsed,
            data,
            expr_id_map,
            stmt_id_map,
        };
        builder.add_template(&mut walker);
        (walker.expr_id_map, walker.stmt_id_map)
    };

    builder.finalize_unresolved_references();

    parsed.drain_pending(&expr_id_map, &stmt_id_map);

    let mut scoping = ComponentScoping::from_semantics(builder.finish());
    scoping.build_template_scope_set();
    data.scoping = scoping;
}

struct AnalyzeTemplateWalker<'d, 'a> {
    store: &'d svelte_ast::AstStore,
    source: &'d str,
    root: svelte_ast::FragmentId,
    parsed: &'d mut JsAst<'a>,
    data: &'d mut AnalysisData<'a>,
    expr_id_map: FxHashMap<u32, OxcNodeId>,
    stmt_id_map: FxHashMap<u32, OxcNodeId>,
}

fn record_expr_id(r: &svelte_ast::ExprRef, offset: u32, out: &mut FxHashMap<u32, OxcNodeId>) {
    let id = r.oxc_id.get();
    if id != OxcNodeId::DUMMY {
        out.insert(offset, id);
    }
}

fn record_stmt_id(r: &svelte_ast::StmtRef, offset: u32, out: &mut FxHashMap<u32, OxcNodeId>) {
    let id = r.oxc_id.get();
    if id != OxcNodeId::DUMMY {
        out.insert(offset, id);
    }
}

impl<'d, 'a> AnalyzeTemplateWalker<'d, 'a> {
    fn walk_fragment(
        &mut self,
        fragment_id: svelte_ast::FragmentId,
        ctx: &mut TemplateBuildContext<'_, 'a>,
    ) {
        let nodes: &'d [NodeId] = self.store.fragment_nodes(fragment_id);
        for &id in nodes {
            match self.store.get(id) {
                Node::Element(el) => {
                    self.walk_attributes(&el.attributes, ctx);
                    self.walk_fragment(el.fragment, ctx);
                }
                Node::SlotElementLegacy(el) => {
                    self.walk_attributes(&el.attributes, ctx);
                    self.walk_fragment(el.fragment, ctx);
                }
                Node::ComponentNode(node) => self.walk_component_node(node, ctx),
                Node::SvelteComponentLegacy(node) => self.walk_svelte_component_legacy(node, ctx),
                Node::SvelteSelf(node) => self.walk_svelte_self(node, ctx),
                Node::ExpressionTag(tag) => {
                    if let Some(expr) = self.parsed.pending_expr(tag.expression.span.start) {
                        ctx.visit_js_expression(&tag.expression, expr);
                        record_expr_id(
                            &tag.expression,
                            tag.expression.span.start,
                            &mut self.expr_id_map,
                        );
                    }
                }
                Node::RenderTag(tag) => {
                    if let Some(expr) = self.parsed.pending_expr(tag.expression.span.start) {
                        ctx.visit_js_expression(&tag.expression, expr);
                        record_expr_id(
                            &tag.expression,
                            tag.expression.span.start,
                            &mut self.expr_id_map,
                        );
                    }
                }
                Node::HtmlTag(tag) => {
                    if let Some(expr) = self.parsed.pending_expr(tag.expression.span.start) {
                        ctx.visit_js_expression(&tag.expression, expr);
                        record_expr_id(
                            &tag.expression,
                            tag.expression.span.start,
                            &mut self.expr_id_map,
                        );
                    }
                }
                Node::ConstTag(tag) => {
                    if let Some(stmt) = self.parsed.pending_stmt(tag.decl.span.start) {
                        ctx.visit_js_statement(&tag.decl, stmt);
                        record_stmt_id(&tag.decl, tag.decl.span.start, &mut self.stmt_id_map);
                    }
                }
                Node::DeclarationTag(tag) => {
                    if let Some(stmt) = self.parsed.pending_stmt(tag.declaration.span.start) {
                        ctx.visit_js_statement(&tag.declaration, stmt);
                        record_stmt_id(
                            &tag.declaration,
                            tag.declaration.span.start,
                            &mut self.stmt_id_map,
                        );
                    }
                }
                Node::EachBlock(block) => self.walk_each_block(block, ctx),
                Node::IfBlock(block) => {
                    if let Some(expr) = self.parsed.pending_expr(block.test.span.start) {
                        ctx.visit_js_expression(&block.test, expr);
                        record_expr_id(&block.test, block.test.span.start, &mut self.expr_id_map);
                    }

                    let consequent = block.consequent;
                    let alternate = block.alternate;
                    ctx.enter_fragment_scope_by_id(consequent);
                    self.walk_fragment(consequent, ctx);
                    ctx.leave_scope();

                    if let Some(alt) = alternate {
                        ctx.enter_fragment_scope_by_id(alt);
                        self.walk_fragment(alt, ctx);
                        ctx.leave_scope();
                    }
                }
                Node::SnippetBlock(block) => {
                    let mut arrow_scope = None;
                    if let Some(stmt) = self.parsed.pending_stmt(block.decl.span.start) {
                        ctx.visit_js_statement(&block.decl, stmt);
                        record_stmt_id(&block.decl, block.decl.span.start, &mut self.stmt_id_map);
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
                        .set_fragment_scope_by_id(block.body, scope);
                    ctx.enter_scope(scope);
                    self.walk_fragment(block.body, ctx);
                    ctx.leave_scope();
                }
                Node::KeyBlock(block) => {
                    if let Some(expr) = self.parsed.pending_expr(block.expression.span.start) {
                        ctx.visit_js_expression(&block.expression, expr);
                        record_expr_id(
                            &block.expression,
                            block.expression.span.start,
                            &mut self.expr_id_map,
                        );
                    }
                    let f = block.fragment;
                    ctx.enter_fragment_scope_by_id(f);
                    self.walk_fragment(f, ctx);
                    ctx.leave_scope();
                }
                Node::SvelteHead(head) => {
                    let f = head.fragment;
                    ctx.enter_fragment_scope_by_id(f);
                    self.walk_fragment(f, ctx);
                    ctx.leave_scope();
                }
                Node::SvelteFragmentLegacy(node) => {
                    self.walk_attributes(&node.attributes, ctx);
                    self.walk_fragment(node.fragment, ctx);
                }
                Node::SvelteElement(el) => {
                    self.walk_attributes(&el.attributes, ctx);
                    ctx.enter_fragment_scope_by_id(el.fragment);
                    self.walk_fragment(el.fragment, ctx);
                    ctx.leave_scope();
                }
                Node::SvelteBoundary(boundary) => {
                    self.walk_attributes(&boundary.attributes, ctx);
                    ctx.enter_fragment_scope_by_id(boundary.fragment);
                    self.walk_fragment(boundary.fragment, ctx);
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
                            record_expr_id(ident_ref, ident_ref.span.start, &mut self.expr_id_map);
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
            record_expr_id(
                &block.expression,
                block.expression.span.start,
                &mut self.expr_id_map,
            );
        }

        let body = block.body;
        let fallback = block.fallback;
        ctx.enter_fragment_scope_by_id(body);
        if let Some(ctx_ref) = block.context.as_ref()
            && let Some(stmt) = self.parsed.pending_stmt(ctx_ref.span.start)
        {
            ctx.visit_js_statement(ctx_ref, stmt);
            record_stmt_id(ctx_ref, ctx_ref.span.start, &mut self.stmt_id_map);
        }
        if let Some(idx_ref) = block.index.as_ref()
            && let Some(stmt) = self.parsed.pending_stmt(idx_ref.span.start)
        {
            ctx.visit_js_statement(idx_ref, stmt);
            record_stmt_id(idx_ref, idx_ref.span.start, &mut self.stmt_id_map);
        }
        if let Some(key_ref) = block.key.as_ref()
            && let Some(expr) = self.parsed.pending_expr(key_ref.span.start)
        {
            ctx.visit_js_expression(key_ref, expr);
            record_expr_id(key_ref, key_ref.span.start, &mut self.expr_id_map);
        }
        self.walk_fragment(body, ctx);
        ctx.leave_scope();

        if let Some(fb) = fallback {
            self.walk_fragment(fb, ctx);
        }
    }

    fn walk_await_block(&mut self, block: &'d AwaitBlock, ctx: &mut TemplateBuildContext<'_, 'a>) {
        if let Some(expr) = self.parsed.pending_expr(block.expression.span.start) {
            ctx.visit_js_expression(&block.expression, expr);
            record_expr_id(
                &block.expression,
                block.expression.span.start,
                &mut self.expr_id_map,
            );
        }

        if let Some(p) = block.pending {
            ctx.enter_fragment_scope_by_id(p);
            self.walk_fragment(p, ctx);
            ctx.leave_scope();
        }
        if let Some(t) = block.then {
            ctx.enter_fragment_scope_by_id(t);
            if let Some(vr) = block.value.as_ref()
                && let Some(stmt) = self.parsed.pending_stmt(vr.span.start)
            {
                ctx.visit_js_statement(vr, stmt);
                record_stmt_id(vr, vr.span.start, &mut self.stmt_id_map);
            }
            self.walk_fragment(t, ctx);
            ctx.leave_scope();
        }
        if let Some(c) = block.catch {
            ctx.enter_fragment_scope_by_id(c);
            if let Some(er) = block.error.as_ref()
                && let Some(stmt) = self.parsed.pending_stmt(er.span.start)
            {
                ctx.visit_js_statement(er, stmt);
                record_stmt_id(er, er.span.start, &mut self.stmt_id_map);
            }
            self.walk_fragment(c, ctx);
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
                        record_expr_id(
                            &attr.expression,
                            attr.expression.span.start,
                            &mut self.expr_id_map,
                        );
                    }
                }
                Attribute::SpreadAttribute(attr) => {
                    if let Some(expr) = self.parsed.pending_expr(attr.expression.span.start) {
                        ctx.visit_js_expression(&attr.expression, expr);
                        record_expr_id(
                            &attr.expression,
                            attr.expression.span.start,
                            &mut self.expr_id_map,
                        );
                    }
                }
                Attribute::ClassDirective(dir) => self.walk_class_directive(dir, ctx),
                Attribute::StyleDirective(dir) => self.walk_style_directive(dir, ctx),
                Attribute::BindDirective(dir) => self.walk_bind_directive(dir, ctx),
                Attribute::LetDirectiveLegacy(dir) => self.declare_let_directive_legacy(dir, ctx),
                Attribute::UseDirective(dir) => {
                    if let Some(expr) = self.parsed.pending_expr(dir.name_ref.span.start) {
                        ctx.visit_js_expression(&dir.name_ref, expr);
                        record_expr_id(
                            &dir.name_ref,
                            dir.name_ref.span.start,
                            &mut self.expr_id_map,
                        );
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
                        record_expr_id(
                            &dir.name_ref,
                            dir.name_ref.span.start,
                            &mut self.expr_id_map,
                        );
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
                        record_expr_id(
                            &dir.name_ref,
                            dir.name_ref.span.start,
                            &mut self.expr_id_map,
                        );
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
                        record_expr_id(
                            &tag.expression,
                            tag.expression.span.start,
                            &mut self.expr_id_map,
                        );
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
                    if let Some(expr_ref) = dir.expression.as_ref()
                        && let Some(expr) = self.parsed.pending_expr(expr_ref.span.start)
                    {
                        ctx.visit_js_expression(expr_ref, expr);
                        record_expr_id(expr_ref, expr_ref.span.start, &mut self.expr_id_map);
                    }
                }
                Attribute::StringAttribute(_) | Attribute::BooleanAttribute(_) => {}
            }
        }
    }

    fn walk_component_node(
        &mut self,
        node: &'d svelte_ast::ComponentNode,
        ctx: &mut TemplateBuildContext<'_, 'a>,
    ) {
        if let Some(expr) = self.parsed.pending_expr(node.name.span.start) {
            ctx.visit_js_expression(&node.name, expr);
            record_expr_id(&node.name, node.name.span.start, &mut self.expr_id_map);
        }
        self.walk_component_like(&node.attributes, node.fragment, &node.legacy_slots, ctx);
    }

    fn walk_svelte_component_legacy(
        &mut self,
        node: &'d svelte_ast::SvelteComponentLegacy,
        ctx: &mut TemplateBuildContext<'_, 'a>,
    ) {
        self.walk_component_like(&node.attributes, node.fragment, &node.legacy_slots, ctx);
    }

    fn walk_svelte_self(
        &mut self,
        node: &'d svelte_ast::SvelteSelf,
        ctx: &mut TemplateBuildContext<'_, 'a>,
    ) {
        self.walk_component_like(&node.attributes, node.fragment, &node.legacy_slots, ctx);
    }

    fn walk_component_like(
        &mut self,
        attributes: &'d [Attribute],
        cn_fragment: svelte_ast::FragmentId,
        legacy_slots: &'d [svelte_ast::LegacySlot],
        ctx: &mut TemplateBuildContext<'_, 'a>,
    ) {
        let component_has_slot_attr = attrs_static_slot_name(attributes, self.source).is_some();
        let default_scope = if component_has_slot_attr {
            ctx.register_fragment_scope_by_id(cn_fragment);
            ctx.current_scope()
        } else {
            let scope = ctx.enter_fragment_scope_by_id(cn_fragment);
            ctx.leave_scope();
            scope
        };

        for attr in attributes {
            match attr {
                Attribute::LetDirectiveLegacy(dir) => {
                    ctx.enter_scope(default_scope);
                    self.declare_let_directive_legacy(dir, ctx);
                    ctx.leave_scope();
                }
                _ => self.walk_attributes(slice::from_ref(attr), ctx),
            }
        }

        ctx.enter_scope(default_scope);
        self.walk_fragment(cn_fragment, ctx);
        ctx.leave_scope();

        let slot_frags: Vec<svelte_ast::FragmentId> =
            legacy_slots.iter().map(|s| s.fragment).collect();
        for slot_fid in slot_frags {
            let scope = ctx.enter_fragment_scope_by_id(slot_fid);
            debug_assert_eq!(scope, ctx.current_scope());
            self.walk_fragment(slot_fid, ctx);
            ctx.leave_scope();
        }
    }

    fn walk_bind_directive(&mut self, dir: &BindDirective, ctx: &mut TemplateBuildContext<'_, 'a>) {
        let Some(expr) = self.parsed.pending_expr(dir.expression.span.start) else {
            return;
        };
        match expr.get_inner_expression() {
            Expression::Identifier(_) => {
                ctx.visit_js_expression_with_flags(
                    &dir.expression,
                    expr,
                    ReferenceFlags::Read | ReferenceFlags::Write,
                );
            }
            Expression::StaticMemberExpression(_) | Expression::ComputedMemberExpression(_) => {
                ctx.visit_js_expression(&dir.expression, expr);
                if !bind_member_root_is_store_sub(expr)
                    && let Some(sym_id) = bind_member_root_symbol(expr, ctx)
                {
                    ctx.mark_symbol_member_mutated(sym_id);
                }
            }
            _ => {
                ctx.visit_js_expression(&dir.expression, expr);
            }
        }
        record_expr_id(
            &dir.expression,
            dir.expression.span.start,
            &mut self.expr_id_map,
        );
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
            record_expr_id(expr_ref, span.start, &mut self.expr_id_map);

            if let Some(sym_id) = attr_root_symbol(expr, ctx) {
                self.data
                    .template
                    .template_semantics
                    .node_ref_symbols
                    .insert(node_id, smallvec![sym_id]);
            }
        }
    }

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
            record_expr_id(expr_ref, offset, &mut self.expr_id_map);
        }
    }

    fn declare_let_directive_legacy(
        &mut self,
        dir: &LetDirectiveLegacy,
        ctx: &mut TemplateBuildContext<'_, 'a>,
    ) {
        if let Some(stmt) = self.parsed.pending_stmt(dir.name_span.start)
            && let Some(binding_ref) = dir.binding.as_ref()
        {
            ctx.visit_js_statement(binding_ref, stmt);
            record_stmt_id(binding_ref, dir.name_span.start, &mut self.stmt_id_map);
        }
    }
}

fn bind_member_root_symbol<'a>(
    expr: &Expression<'a>,
    ctx: &TemplateBuildContext<'_, 'a>,
) -> Option<SymbolId> {
    attr_root_symbol(expr, ctx)
}

fn bind_member_root_is_store_sub<'a>(expr: &Expression<'a>) -> bool {
    let mut current = expr.get_inner_expression();
    loop {
        match current {
            Expression::StaticMemberExpression(m) => current = m.object.get_inner_expression(),
            Expression::ComputedMemberExpression(m) => current = m.object.get_inner_expression(),
            Expression::Identifier(ident) => {
                return svelte_ast::store_subscription_base(ident.name.as_str()).is_some();
            }
            _ => return false,
        }
    }
}

fn attr_root_symbol<'a>(
    expr: &Expression<'a>,
    ctx: &TemplateBuildContext<'_, 'a>,
) -> Option<SymbolId> {
    let mut current = expr.get_inner_expression();
    loop {
        match current {
            Expression::StaticMemberExpression(m) => current = m.object.get_inner_expression(),
            Expression::ComputedMemberExpression(m) => current = m.object.get_inner_expression(),
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
        Attribute::StringAttribute(attr) if attr.name == SLOT_ATTRIBUTE => {
            Some(attr.value_span.source_text(source))
        }
        _ => None,
    })
}

impl<'d, 'a> TemplateWalker<'a> for AnalyzeTemplateWalker<'d, 'a> {
    fn walk_template(&mut self, ctx: &mut TemplateBuildContext<'_, 'a>) {
        self.walk_fragment(self.root, ctx);
    }
}

fn extract_arrow_from_const<'a>(
    stmt: &'a Statement<'a>,
) -> Option<&'a ArrowFunctionExpression<'a>> {
    let Statement::VariableDeclaration(decl) = stmt else {
        return None;
    };
    let declarator = decl.declarations.first()?;
    let Expression::ArrowFunctionExpression(arrow) =
        declarator.init.as_ref()?.get_inner_expression()
    else {
        return None;
    };
    Some(arrow)
}
