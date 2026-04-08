use oxc_ast::ast::{Expression, FormalParameters, IdentifierReference, Statement};
use oxc_ast_visit::Visit;
use smallvec::SmallVec;
use svelte_ast::{Attribute, EachBlock, NodeId, RenderTag, StyleDirectiveValue};
use svelte_span::Span;

use crate::passes::js_analyze;
use crate::scope::{ComponentScoping, SymbolId};
use crate::types::data::{AnalysisData, ExpressionInfo, ExpressionKind};
use crate::walker::{TemplateVisitor, VisitContext};

/// Create a CollectSymbolsVisitor for use after TemplateSemanticVisitor.
/// Consumes `ScopingBuilt` marker to enforce ordering.
pub(crate) fn make_visitor(_scoping: crate::types::markers::ScopingBuilt) -> CollectSymbolsVisitor {
    CollectSymbolsVisitor {
        pending_render_tag: None,
        pending_shorthand: None,
        pending_clsx: false,
    }
}

pub(crate) struct CollectSymbolsVisitor {
    pending_render_tag: Option<NodeId>,
    pending_shorthand: Option<(NodeId, String)>,
    pending_clsx: bool,
}

/// Resolve script-level store subscriptions from OXC unresolved references.
/// Called after the template walk, when ComponentScoping is fully built.
pub(crate) fn resolve_script_stores(data: &mut AnalysisData) {
    let candidates: Vec<String> = match &data.script.info {
        Some(s) => s.store_candidates.iter().map(|n| format!("${n}")).collect(),
        None => return,
    };
    for name in &candidates {
        try_mark_store(name, &mut data.scoping);
    }
}

// ---------------------------------------------------------------------------
// TemplateVisitor impl
// ---------------------------------------------------------------------------

impl TemplateVisitor for CollectSymbolsVisitor {
    fn visit_each_block(&mut self, block: &EachBlock, ctx: &mut VisitContext<'_>) {
        if let Some(key_id) = block.key_id {
            ctx.data.blocks.each_context.record_key_node_id(block.id, key_id);
        }
    }

    fn visit_expression(&mut self, node_id: NodeId, span: Span, ctx: &mut VisitContext<'_>) {
        store_expr_offset(node_id, span, ctx);
    }

    fn visit_js_expression(
        &mut self,
        node_id: NodeId,
        expr: &Expression<'_>,
        ctx: &mut VisitContext<'_>,
    ) {
        let info = build_expression_info(expr, &mut ctx.data.scoping);
        detect_each_index_usage(node_id, &info.ref_symbols, ctx.data);
        store_expression_info(node_id, info, ctx);
        classify_shorthand(node_id, expr, &mut self.pending_shorthand, ctx.data);
        classify_clsx(node_id, expr, &mut self.pending_clsx, ctx.data);
        classify_render_tag(node_id, expr, &mut self.pending_render_tag, ctx.data);
    }

    fn visit_js_statement(
        &mut self,
        node_id: NodeId,
        stmt: &Statement<'_>,
        ctx: &mut VisitContext<'_>,
    ) {
        if ctx.data.snippet_stmt_handle(node_id).is_none() {
            return;
        }
        let Some(params) = extract_arrow_params(stmt) else {
            return;
        };
        let symbols = collect_ref_symbols_from_formal_parameters(params, &mut ctx.data.scoping);
        if !symbols.is_empty() {
            ctx.data
                .template
                .template_semantics
                .stmt_ref_symbols
                .insert(node_id, symbols);
        }
    }

    fn visit_render_tag(&mut self, tag: &RenderTag, ctx: &mut VisitContext<'_>) {
        self.pending_render_tag = Some(tag.id);
        resolve_render_tag_callee(tag, ctx);
    }

    fn visit_const_tag(&mut self, tag: &svelte_ast::ConstTag, ctx: &mut VisitContext<'_>) {
        if let Some(handle) = ctx
            .parsed()
            .and_then(|p| p.expr_handle(tag.expression_span.start))
        {
            ctx.data
                .template
                .template_semantics
                .node_expr_handles
                .insert(tag.id, handle);
        }
        if let Some(handle) = ctx
            .parsed()
            .and_then(|p| p.stmt_handle(tag.expression_span.start))
        {
            ctx.data
                .template
                .template_semantics
                .const_tag_stmt_handles
                .insert(tag.id, handle);
        }
        // Build ExpressionInfo for the @const init expression so that
        // mark_const_tag_bindings can read ref_symbols for derived_deps.
        if let Some(init_expr) = ctx
            .parsed()
            .and_then(|p| p.stmt_handle(tag.expression_span.start))
            .and_then(|handle| ctx.parsed().and_then(|p| p.stmt(handle)))
            .and_then(|stmt| {
                if let oxc_ast::ast::Statement::VariableDeclaration(decl) = stmt {
                    decl.declarations.first().and_then(|d| d.init.as_ref())
                } else {
                    None
                }
            })
        {
            let info = build_expression_info(init_expr, &mut ctx.data.scoping);
            ctx.data.expressions.insert(tag.id, info);
        }
    }

    fn visit_attribute(&mut self, attr: &Attribute, _ctx: &mut VisitContext<'_>) {
        set_pending_flags(attr, &mut self.pending_shorthand, &mut self.pending_clsx);
    }

    fn leave_concatenation_attribute(
        &mut self,
        attr: &svelte_ast::ConcatenationAttribute,
        ctx: &mut VisitContext<'_>,
    ) {
        merge_concat_expression_info(&attr.parts, attr.id, ctx);
    }

    fn leave_style_directive(
        &mut self,
        dir: &svelte_ast::StyleDirective,
        ctx: &mut VisitContext<'_>,
    ) {
        if let StyleDirectiveValue::Concatenation(parts) = &dir.value {
            merge_concat_expression_info(parts, dir.id, ctx);
        }
    }
}

// ---------------------------------------------------------------------------
// Extracted steps — each is a named function for flat visit_js_expression
// ---------------------------------------------------------------------------

/// Build a complete ExpressionInfo: analyze flags + collect resolved symbols in one pass.
pub(crate) fn build_expression_info(
    expr: &Expression<'_>,
    scoping: &mut ComponentScoping,
) -> ExpressionInfo {
    let mut info = js_analyze::analyze_expression(expr);
    info.ref_symbols = collect_ref_symbols_and_stores(expr, scoping);
    info
}

/// Single OXC walk: collect resolved SymbolIds + detect $store subscriptions.
fn collect_ref_symbols_and_stores(
    expr: &Expression<'_>,
    scoping: &mut ComponentScoping,
) -> SmallVec<[SymbolId; 2]> {
    collect_resolved_ref_symbols_with_options(scoping, true, |collector| {
        collector.visit_expression(expr);
    })
}

fn collect_ref_symbols_from_formal_parameters(
    params: &FormalParameters<'_>,
    scoping: &mut ComponentScoping,
) -> SmallVec<[SymbolId; 2]> {
    collect_resolved_ref_symbols(scoping, |collector| {
        collector.visit_formal_parameters(params)
    })
}

fn extract_arrow_params<'s, 'a: 's>(stmt: &'s Statement<'a>) -> Option<&'s FormalParameters<'a>> {
    let Statement::VariableDeclaration(decl) = stmt else {
        return None;
    };
    let declarator = decl.declarations.first()?;
    let Some(Expression::ArrowFunctionExpression(arrow)) = &declarator.init else {
        return None;
    };
    Some(&arrow.params)
}

fn detect_each_index_usage(
    node_id: NodeId,
    symbols: &SmallVec<[SymbolId; 2]>,
    data: &mut AnalysisData,
) {
    for &sym in symbols {
        if let Some(block_id) = data.each_block_for_index_sym(sym) {
            let is_key = data
                .each_key_node_id(block_id)
                .is_some_and(|kid| kid == node_id);
            if is_key {
                data.blocks.each_context.mark_key_uses_index(block_id);
            } else {
                data.blocks.each_context.mark_body_uses_index(block_id);
            }
        }
    }
}

fn store_expression_info(node_id: NodeId, info: ExpressionInfo, ctx: &mut VisitContext<'_>) {
    if ctx.parent().is_some_and(|p| p.kind.is_attr()) {
        ctx.data.attr_expressions.insert(node_id, info);
    } else {
        ctx.data.expressions.insert(node_id, info);
    }
}

fn store_expr_offset(node_id: NodeId, span: Span, ctx: &mut VisitContext<'_>) {
    let Some(handle) = ctx.parsed().and_then(|p| p.expr_handle(span.start)) else {
        return;
    };
    if ctx.parent().is_some_and(|p| p.kind.is_attr()) {
        ctx.data
            .template
            .template_semantics
            .attr_expr_handles
            .insert(node_id, handle);
    } else {
        ctx.data
            .template
            .template_semantics
            .node_expr_handles
            .insert(node_id, handle);
    }
}

fn classify_shorthand(
    _node_id: NodeId,
    expr: &Expression<'_>,
    pending: &mut Option<(NodeId, String)>,
    data: &mut AnalysisData,
) {
    if let Some((attr_id, name)) = pending.take() {
        if let Expression::Identifier(ident) = expr {
            if ident.name.as_str() == name {
                data.elements.flags.expression_shorthand.insert(attr_id);
            }
        }
    }
}

fn classify_clsx(
    node_id: NodeId,
    expr: &Expression<'_>,
    pending: &mut bool,
    data: &mut AnalysisData,
) {
    if !*pending {
        return;
    }
    *pending = false;
    if !matches!(
        expr,
        Expression::StringLiteral(_)
            | Expression::TemplateLiteral(_)
            | Expression::BinaryExpression(_)
    ) {
        data.elements.flags.needs_clsx.insert(node_id);
    }
}

fn classify_render_tag(
    node_id: NodeId,
    expr: &Expression<'_>,
    pending: &mut Option<NodeId>,
    data: &mut AnalysisData,
) {
    if pending.take() == Some(node_id) {
        js_analyze::classify_render_tag_args(expr, data, node_id);
    }
}

fn set_pending_flags(
    attr: &Attribute,
    pending_shorthand: &mut Option<(NodeId, String)>,
    pending_clsx: &mut bool,
) {
    match attr {
        Attribute::ExpressionAttribute(ea) => {
            *pending_shorthand = Some((ea.id, ea.name.clone()));
            if ea.name == "class" {
                *pending_clsx = true;
            }
        }
        Attribute::ClassDirective(cd) => {
            if cd.expression_span.is_some() {
                *pending_shorthand = Some((cd.id, cd.name.clone()));
            }
        }
        Attribute::StyleDirective(sd) => {
            if matches!(sd.value, StyleDirectiveValue::Expression(_)) {
                *pending_shorthand = Some((sd.id, sd.name.clone()));
            }
        }
        _ => {}
    }
}

fn resolve_render_tag_callee(tag: &RenderTag, ctx: &mut VisitContext<'_>) {
    if let Some(parsed) = ctx.parsed() {
        if let Some(Expression::CallExpression(call)) = parsed
            .expr_handle(tag.expression_span.start)
            .and_then(|handle| parsed.expr(handle))
        {
            if let Expression::Identifier(ident) = &call.callee {
                if let Some(sym_id) = resolve_identifier_symbol(ident, &ctx.data.scoping) {
                    ctx.data.blocks.render_tag_callee_sym.insert(tag.id, sym_id);
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// If `name` is `$X`, mark `X` as a store subscription.
fn try_mark_store(name: &str, scoping: &mut ComponentScoping) {
    if !name.starts_with('$') || name.len() <= 1 {
        return;
    }
    let base = &name[1..];
    let root = scoping.root_scope_id();
    let Some(sym_id) = scoping.find_binding(root, base) else {
        return;
    };
    if !scoping.is_rune(sym_id) && !scoping.is_store(sym_id) {
        scoping.mark_store(sym_id);
    }
}

/// Merge ExpressionInfo from individual concat part entries into a single entry
/// for the parent attribute/directive.
fn merge_concat_expression_info(
    parts: &[svelte_ast::ConcatPart],
    parent_id: NodeId,
    ctx: &mut VisitContext<'_>,
) {
    let mut merged = ExpressionInfo {
        kind: ExpressionKind::Other,
        ref_symbols: SmallVec::new(),
        has_store_ref: false,
        has_side_effects: false,
        has_call: false,
        has_await: false,
        has_state_rune: false,
        has_store_member_mutation: false,
        needs_context: false,
        is_dynamic: false,
        has_state: false,
    };
    for part in parts {
        if let svelte_ast::ConcatPart::Dynamic { id, .. } = part {
            if let Some(info) = ctx.data.attr_expressions.get(*id) {
                merged.has_call |= info.has_call;
                merged.has_await |= info.has_await;
                merged.has_store_ref |= info.has_store_ref;
                merged.has_side_effects |= info.has_side_effects;
                merged.has_state_rune |= info.has_state_rune;
                merged.has_store_member_mutation |= info.has_store_member_mutation;
                merged.needs_context |= info.needs_context;
                for sym in &info.ref_symbols {
                    if !merged.ref_symbols.contains(sym) {
                        merged.ref_symbols.push(*sym);
                    }
                }
            }
        }
    }
    // Keep the merged parent attr/directive as a separate reverse-reference site:
    // callers may care about the holder node even when its dynamic parts were
    // already recorded individually.
    ctx.data.attr_expressions.insert(parent_id, merged);
}

fn resolve_identifier_symbol(
    ident: &IdentifierReference,
    scoping: &ComponentScoping,
) -> Option<SymbolId> {
    let ref_id = ident.reference_id.get()?;
    scoping.get_reference(ref_id).symbol_id()
}

struct ResolvedRefCollector<'s> {
    scoping: &'s mut ComponentScoping,
    symbols: SmallVec<[SymbolId; 2]>,
    mark_stores: bool,
}

impl<'a> Visit<'a> for ResolvedRefCollector<'_> {
    fn visit_identifier_reference(&mut self, ident: &IdentifierReference<'a>) {
        if let Some(ref_id) = ident.reference_id.get() {
            if let Some(sym_id) = self.scoping.get_reference(ref_id).symbol_id() {
                if !self.symbols.contains(&sym_id) {
                    self.symbols.push(sym_id);
                }
            }
        }
        if self.mark_stores {
            try_mark_store(ident.name.as_str(), self.scoping);
        }
    }
}

fn collect_resolved_ref_symbols(
    scoping: &mut ComponentScoping,
    visit: impl FnOnce(&mut ResolvedRefCollector<'_>),
) -> SmallVec<[SymbolId; 2]> {
    collect_resolved_ref_symbols_with_options(scoping, false, visit)
}

fn collect_resolved_ref_symbols_with_options(
    scoping: &mut ComponentScoping,
    mark_stores: bool,
    visit: impl FnOnce(&mut ResolvedRefCollector<'_>),
) -> SmallVec<[SymbolId; 2]> {
    let mut collector = ResolvedRefCollector {
        scoping,
        symbols: SmallVec::new(),
        mark_stores,
    };
    visit(&mut collector);
    collector.symbols
}
