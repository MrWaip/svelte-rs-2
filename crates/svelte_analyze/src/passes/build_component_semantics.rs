use oxc_ast::ast::{ArrowFunctionExpression, Expression, Statement};
use smallvec::smallvec;
use svelte_ast::{
    Attribute, AwaitBlock, BindDirective, ClassDirective, Component, LetDirectiveLegacy, Node,
    NodeId, StyleDirective, StyleDirectiveValue,
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
            store: &component.store,
            source: &component.source,
            root: component.root,
            parsed,
            data,
        };
        builder.add_template(&mut walker);
    }

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

    if let Some(module_program) = parsed.module_program.as_ref()
        && let Some(span) = parsed.module_script_content_span
    {
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
    data.scoping = scoping;
}

struct AnalyzeTemplateWalker<'d, 'a> {
    store: &'d svelte_ast::AstStore,
    source: &'d str,
    root: svelte_ast::FragmentId,
    parsed: &'d mut JsAst<'a>,
    data: &'d mut AnalysisData<'a>,
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
                    if let (Some(expr), Some(tag_ref)) =
                        (self.parsed.pending_expr(el.tag_span.start), el.tag.as_ref())
                    {
                        ctx.visit_js_expression(tag_ref, expr);
                    }
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

        let body = block.body;
        let fallback = block.fallback;
        ctx.enter_fragment_scope_by_id(body);
        if let Some(ctx_ref) = block.context.as_ref()
            && let Some(stmt) = self.parsed.pending_stmt(ctx_ref.span.start)
        {
            ctx.visit_js_statement(ctx_ref, stmt);
        }
        if let Some(idx_ref) = block.index.as_ref()
            && let Some(stmt) = self.parsed.pending_stmt(idx_ref.span.start)
        {
            ctx.visit_js_statement(idx_ref, stmt);
        }
        if let Some(key_ref) = block.key.as_ref()
            && let Some(expr) = self.parsed.pending_expr(key_ref.span.start)
        {
            ctx.visit_js_expression(key_ref, expr);
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
                    if let Some(expr_ref) = dir.expression.as_ref()
                        && let Some(expr) = self.parsed.pending_expr(expr_ref.span.start)
                    {
                        ctx.visit_js_expression(expr_ref, expr);
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
        let component_has_slot_attr =
            attrs_static_slot_name(&node.attributes, self.source).is_some();
        let cn_fragment = node.fragment;
        let default_scope = if component_has_slot_attr {
            ctx.register_fragment_scope_by_id(cn_fragment);
            ctx.current_scope()
        } else {
            let scope = ctx.enter_fragment_scope_by_id(cn_fragment);
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
        self.walk_fragment(cn_fragment, ctx);
        ctx.leave_scope();

        let slot_frags: Vec<svelte_ast::FragmentId> =
            node.legacy_slots.iter().map(|s| s.fragment).collect();
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
        }
    }
}

fn bind_member_root_symbol<'a>(
    expr: &Expression<'a>,
    ctx: &TemplateBuildContext<'_, 'a>,
) -> Option<svelte_component_semantics::SymbolId> {
    attr_root_symbol(expr, ctx)
}

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

impl<'d, 'a> TemplateWalker<'a> for AnalyzeTemplateWalker<'d, 'a> {
    fn walk_template(&mut self, ctx: &mut TemplateBuildContext<'_, 'a>) {
        self.walk_fragment(self.root, ctx);
    }
}

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
        fragment_id: svelte_ast::FragmentId,
        expr_ids: &mut rustc_hash::FxHashMap<u32, oxc_syntax::node::NodeId>,
        stmt_ids: &mut rustc_hash::FxHashMap<u32, oxc_syntax::node::NodeId>,
    ) {
        let nodes = component.fragment_nodes(fragment_id).to_vec();
        for id in nodes {
            match component.store.get(id) {
                Node::Element(el) => {
                    for attr in &el.attributes {
                        walk_attr(attr, expr_ids, stmt_ids);
                    }
                    walk_fragment(component, el.fragment, expr_ids, stmt_ids);
                }
                Node::SlotElementLegacy(el) => {
                    for attr in &el.attributes {
                        walk_attr(attr, expr_ids, stmt_ids);
                    }
                    walk_fragment(component, el.fragment, expr_ids, stmt_ids);
                }
                Node::ComponentNode(cn) => {
                    for attr in &cn.attributes {
                        walk_attr(attr, expr_ids, stmt_ids);
                    }
                    let cn_fragment = cn.fragment;
                    let slot_frags: Vec<_> = cn.legacy_slots.iter().map(|s| s.fragment).collect();
                    walk_fragment(component, cn_fragment, expr_ids, stmt_ids);
                    for fid in slot_frags {
                        walk_fragment(component, fid, expr_ids, stmt_ids);
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
                    walk_fragment(component, block.consequent, expr_ids, stmt_ids);
                    if let Some(alt) = block.alternate {
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
                    walk_fragment(component, block.body, expr_ids, stmt_ids);
                    if let Some(fb) = block.fallback {
                        walk_fragment(component, fb, expr_ids, stmt_ids);
                    }
                }
                Node::SnippetBlock(block) => {
                    record_stmt(&block.decl, block.decl.span.start, stmt_ids);
                    walk_fragment(component, block.body, expr_ids, stmt_ids);
                }
                Node::KeyBlock(block) => {
                    record_expr(&block.expression, block.expression.span.start, expr_ids);
                    walk_fragment(component, block.fragment, expr_ids, stmt_ids);
                }
                Node::SvelteHead(head) => {
                    walk_fragment(component, head.fragment, expr_ids, stmt_ids);
                }
                Node::SvelteFragmentLegacy(node) => {
                    for attr in &node.attributes {
                        walk_attr(attr, expr_ids, stmt_ids);
                    }
                    walk_fragment(component, node.fragment, expr_ids, stmt_ids);
                }
                Node::SvelteElement(el) => {
                    if let Some(tag_ref) = el.tag.as_ref() {
                        record_expr(tag_ref, el.tag_span.start, expr_ids);
                    }
                    for attr in &el.attributes {
                        walk_attr(attr, expr_ids, stmt_ids);
                    }
                    walk_fragment(component, el.fragment, expr_ids, stmt_ids);
                }
                Node::SvelteBoundary(b) => {
                    for attr in &b.attributes {
                        walk_attr(attr, expr_ids, stmt_ids);
                    }
                    walk_fragment(component, b.fragment, expr_ids, stmt_ids);
                }
                Node::AwaitBlock(block) => {
                    record_expr(&block.expression, block.expression.span.start, expr_ids);
                    if let Some(p) = block.pending {
                        walk_fragment(component, p, expr_ids, stmt_ids);
                    }
                    if let Some(t) = block.then {
                        if let Some(r) = block.value.as_ref() {
                            record_stmt(r, r.span.start, stmt_ids);
                        }
                        walk_fragment(component, t, expr_ids, stmt_ids);
                    }
                    if let Some(c) = block.catch {
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

    walk_fragment(component, component.root, expr_ids, stmt_ids);
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
