use oxc_ast::ast::{ArrowFunctionExpression, Expression, FormalParameters};
use oxc_ast_visit::Visit;
use oxc_semantic::ScopeId;
use svelte_ast::{
    AnimateDirective, AttachTag, Attribute, AwaitBlock, BindDirective, ClassDirective,
    ComponentNode, ConcatPart, ConcatenationAttribute, EachBlock, ExpressionAttribute,
    ExpressionTag, HtmlTag, IfBlock, KeyBlock, OnDirectiveLegacy, RenderTag, Shorthand,
    SpreadAttribute, StyleDirective, SvelteBody, SvelteBoundary, SvelteDocument,
    SvelteElement, SvelteWindow, TransitionDirective, UseDirective,
};
use svelte_parser::ParsedExprs;

use crate::data::AnalysisData;
use crate::js_analyze::analyze_expression;
use crate::scope::ComponentScoping;
use crate::walker::TemplateVisitor;

// ---------------------------------------------------------------------------
// JsMetadataVisitor — arrow scope registration + each-block index usage
// Merged into resolve_references composite walk.
// ---------------------------------------------------------------------------

pub(crate) struct JsMetadataVisitor<'a> {
    pub component: &'a svelte_ast::Component,
    pub parsed: &'a ParsedExprs<'a>,
}

impl TemplateVisitor for JsMetadataVisitor<'_> {
    fn visit_expression_tag(&mut self, tag: &ExpressionTag, scope: ScopeId, data: &mut AnalysisData) {
        scan_expr_arrows(self.parsed.exprs.get(&tag.expression_span.start), &mut data.scoping, scope);
    }

    fn visit_render_tag(&mut self, tag: &RenderTag, scope: ScopeId, data: &mut AnalysisData) {
        scan_expr_arrows(self.parsed.exprs.get(&tag.expression_span.start), &mut data.scoping, scope);
    }

    fn visit_html_tag(&mut self, tag: &HtmlTag, scope: ScopeId, data: &mut AnalysisData) {
        scan_expr_arrows(self.parsed.exprs.get(&tag.expression_span.start), &mut data.scoping, scope);
    }

    fn visit_const_tag(&mut self, tag: &svelte_ast::ConstTag, scope: ScopeId, data: &mut AnalysisData) {
        scan_expr_arrows(self.parsed.exprs.get(&tag.expression_span.start), &mut data.scoping, scope);
    }

    fn visit_if_block(&mut self, block: &IfBlock, scope: ScopeId, data: &mut AnalysisData) {
        scan_expr_arrows(self.parsed.exprs.get(&block.test_span.start), &mut data.scoping, scope);
    }

    fn visit_each_block(&mut self, block: &EachBlock, parent_scope: ScopeId, _body_scope: ScopeId, data: &mut AnalysisData) {
        scan_expr_arrows(self.parsed.exprs.get(&block.expression_span.start), &mut data.scoping, parent_scope);

        // Each-block index usage detection
        if let Some(idx_span) = block.index_span {
            let idx_name = self.component.source_text(idx_span);
            if let Some(key_span) = block.key_span {
                if let Some(key_expr) = self.parsed.exprs.get(&key_span.start) {
                    let info = analyze_expression(key_expr, key_span.start);
                    if info.references.iter().any(|r| r.name.as_str() == idx_name) {
                        data.each_blocks.key_uses_index.insert(block.id);
                    }
                }
            }
            if check_fragment_uses_name(&block.body, idx_name, data) {
                data.each_blocks.body_uses_index.insert(block.id);
            }
        }
    }

    fn visit_key_block(&mut self, block: &KeyBlock, scope: ScopeId, data: &mut AnalysisData) {
        scan_expr_arrows(self.parsed.exprs.get(&block.expression_span.start), &mut data.scoping, scope);
    }

    fn visit_await_block(&mut self, block: &AwaitBlock, scope: ScopeId, data: &mut AnalysisData) {
        scan_expr_arrows(self.parsed.exprs.get(&block.expression_span.start), &mut data.scoping, scope);
    }

    fn visit_expression_attribute(&mut self, attr: &ExpressionAttribute, scope: ScopeId, data: &mut AnalysisData) {
        scan_attr_arrows_by_offset(Some(attr.expression_span.start), &mut data.scoping, self.parsed, scope);
    }
    fn visit_concatenation_attribute(&mut self, attr: &ConcatenationAttribute, scope: ScopeId, data: &mut AnalysisData) {
        scan_concat_arrows(&attr.parts, &mut data.scoping, self.parsed, scope);
    }
    fn visit_spread_attribute(&mut self, attr: &SpreadAttribute, scope: ScopeId, data: &mut AnalysisData) {
        scan_attr_arrows_by_offset(Some(attr.expression_span.start), &mut data.scoping, self.parsed, scope);
    }
    fn visit_shorthand(&mut self, attr: &Shorthand, scope: ScopeId, data: &mut AnalysisData) {
        scan_attr_arrows_by_offset(Some(attr.expression_span.start), &mut data.scoping, self.parsed, scope);
    }
    fn visit_class_directive(&mut self, dir: &ClassDirective, scope: ScopeId, data: &mut AnalysisData) {
        scan_attr_arrows_by_offset(dir.expression_span.map(|s| s.start), &mut data.scoping, self.parsed, scope);
    }
    fn visit_style_directive(&mut self, dir: &StyleDirective, scope: ScopeId, data: &mut AnalysisData) {
        let offset = match &dir.value {
            svelte_ast::StyleDirectiveValue::Expression(span) => Some(span.start),
            _ => None,
        };
        scan_attr_arrows_by_offset(offset, &mut data.scoping, self.parsed, scope);
        if let svelte_ast::StyleDirectiveValue::Concatenation(parts) = &dir.value {
            scan_concat_arrows(parts, &mut data.scoping, self.parsed, scope);
        }
    }
    fn visit_bind_directive(&mut self, dir: &BindDirective, scope: ScopeId, data: &mut AnalysisData) {
        scan_attr_arrows_by_offset(dir.expression_span.map(|s| s.start), &mut data.scoping, self.parsed, scope);
    }
    fn visit_use_directive(&mut self, dir: &UseDirective, scope: ScopeId, data: &mut AnalysisData) {
        scan_attr_arrows_by_offset(dir.expression_span.map(|s| s.start), &mut data.scoping, self.parsed, scope);
    }
    fn visit_on_directive_legacy(&mut self, dir: &OnDirectiveLegacy, scope: ScopeId, data: &mut AnalysisData) {
        scan_attr_arrows_by_offset(dir.expression_span.map(|s| s.start), &mut data.scoping, self.parsed, scope);
    }
    fn visit_transition_directive(&mut self, dir: &TransitionDirective, scope: ScopeId, data: &mut AnalysisData) {
        scan_attr_arrows_by_offset(dir.expression_span.map(|s| s.start), &mut data.scoping, self.parsed, scope);
    }
    fn visit_animate_directive(&mut self, dir: &AnimateDirective, scope: ScopeId, data: &mut AnalysisData) {
        scan_attr_arrows_by_offset(dir.expression_span.map(|s| s.start), &mut data.scoping, self.parsed, scope);
    }
    fn visit_attach_tag(&mut self, tag: &AttachTag, scope: ScopeId, data: &mut AnalysisData) {
        scan_attr_arrows_by_offset(Some(tag.expression_span.start), &mut data.scoping, self.parsed, scope);
    }

    fn visit_component_node(&mut self, cn: &ComponentNode, scope: ScopeId, data: &mut AnalysisData) {
        for attr in &cn.attributes {
            scan_single_attr_arrows(attr, &mut data.scoping, self.parsed, scope);
        }
    }

    fn visit_svelte_element(&mut self, el: &SvelteElement, scope: ScopeId, data: &mut AnalysisData) {
        if !el.static_tag {
            scan_expr_arrows(self.parsed.exprs.get(&el.tag_span.start), &mut data.scoping, scope);
        }
        for attr in &el.attributes {
            scan_single_attr_arrows(attr, &mut data.scoping, self.parsed, scope);
        }
    }

    fn visit_svelte_window(&mut self, w: &SvelteWindow, scope: ScopeId, data: &mut AnalysisData) {
        for attr in &w.attributes {
            scan_single_attr_arrows(attr, &mut data.scoping, self.parsed, scope);
        }
    }

    fn visit_svelte_document(&mut self, doc: &SvelteDocument, scope: ScopeId, data: &mut AnalysisData) {
        for attr in &doc.attributes {
            scan_single_attr_arrows(attr, &mut data.scoping, self.parsed, scope);
        }
    }

    fn visit_svelte_body(&mut self, body: &SvelteBody, scope: ScopeId, data: &mut AnalysisData) {
        for attr in &body.attributes {
            scan_single_attr_arrows(attr, &mut data.scoping, self.parsed, scope);
        }
    }

    fn visit_svelte_boundary(&mut self, boundary: &SvelteBoundary, scope: ScopeId, data: &mut AnalysisData) {
        for attr in &boundary.attributes {
            scan_single_attr_arrows(attr, &mut data.scoping, self.parsed, scope);
        }
    }
}

// ---------------------------------------------------------------------------
// check_fragment_uses_name (sub-walk for each-block index detection)
// ---------------------------------------------------------------------------

/// Check if any expression in a fragment references a given name.
fn check_fragment_uses_name(fragment: &svelte_ast::Fragment, name: &str, data: &AnalysisData) -> bool {
    for node in &fragment.nodes {
        let refs_match = |id: svelte_ast::NodeId| -> bool {
            data.expressions.get(id)
                .is_some_and(|info| info.references.iter().any(|r| r.name.as_str() == name))
        };
        let attr_refs_match = |attrs: &[Attribute]| -> bool {
            attrs.iter().any(|a| {
                data.attr_expressions.get(a.id())
                    .is_some_and(|info| info.references.iter().any(|r| r.name.as_str() == name))
            })
        };
        match node {
            svelte_ast::Node::ExpressionTag(t) if refs_match(t.id) => return true,
            svelte_ast::Node::Element(el) => {
                if attr_refs_match(&el.attributes) { return true; }
                if check_fragment_uses_name(&el.fragment, name, data) { return true; }
            }
            svelte_ast::Node::ComponentNode(cn) => {
                if attr_refs_match(&cn.attributes) { return true; }
                if check_fragment_uses_name(&cn.fragment, name, data) { return true; }
            }
            svelte_ast::Node::IfBlock(b) => {
                if refs_match(b.id) { return true; }
                if check_fragment_uses_name(&b.consequent, name, data) { return true; }
                if let Some(ref alt) = b.alternate {
                    if check_fragment_uses_name(alt, name, data) { return true; }
                }
            }
            svelte_ast::Node::EachBlock(b) => {
                if refs_match(b.id) { return true; }
                if check_fragment_uses_name(&b.body, name, data) { return true; }
            }
            svelte_ast::Node::RenderTag(t) if refs_match(t.id) => return true,
            svelte_ast::Node::HtmlTag(t) if refs_match(t.id) => return true,
            svelte_ast::Node::KeyBlock(b) => {
                if refs_match(b.id) { return true; }
                if check_fragment_uses_name(&b.fragment, name, data) { return true; }
            }
            svelte_ast::Node::ConstTag(t) if refs_match(t.id) => return true,
            svelte_ast::Node::SvelteElement(e) => {
                if !e.static_tag && refs_match(e.id) { return true; }
                if attr_refs_match(&e.attributes) { return true; }
                if check_fragment_uses_name(&e.fragment, name, data) { return true; }
            }
            _ => {}
        }
    }
    false
}

// ---------------------------------------------------------------------------
// Arrow scope helpers
// ---------------------------------------------------------------------------

/// Scan a single attribute for arrow function scopes.
fn scan_single_attr_arrows(
    attr: &Attribute,
    scoping: &mut ComponentScoping,
    parsed: &ParsedExprs<'_>,
    scope: ScopeId,
) {
    let expr_offset = get_attr_expr_offset(attr);
    if let Some(offset) = expr_offset {
        scan_expr_arrows(parsed.exprs.get(&offset), scoping, scope);
    }
    // Also scan concat part expressions
    let concat_parts: Option<&[ConcatPart]> = match attr {
        Attribute::ConcatenationAttribute(a) => Some(&a.parts),
        Attribute::StyleDirective(a) => match &a.value {
            svelte_ast::StyleDirectiveValue::Concatenation(parts) => Some(parts),
            _ => None,
        },
        _ => None,
    };
    if let Some(parts) = concat_parts {
        for part in parts {
            if let ConcatPart::Dynamic(span) = part {
                scan_expr_arrows(parsed.exprs.get(&span.start), scoping, scope);
            }
        }
    }
}

/// Scan an attribute for arrow scopes using just the expression offset.
fn scan_attr_arrows_by_offset(
    offset: Option<u32>,
    scoping: &mut ComponentScoping,
    parsed: &ParsedExprs<'_>,
    scope: ScopeId,
) {
    if let Some(offset) = offset {
        scan_expr_arrows(parsed.exprs.get(&offset), scoping, scope);
    }
}

/// Scan concat parts for arrow scopes.
fn scan_concat_arrows(
    parts: &[ConcatPart],
    scoping: &mut ComponentScoping,
    parsed: &ParsedExprs<'_>,
    scope: ScopeId,
) {
    for part in parts {
        if let ConcatPart::Dynamic(span) = part {
            scan_expr_arrows(parsed.exprs.get(&span.start), scoping, scope);
        }
    }
}

/// Get the offset for an attribute's primary expression.
fn get_attr_expr_offset(attr: &Attribute) -> Option<u32> {
    match attr {
        Attribute::ExpressionAttribute(a) => Some(a.expression_span.start),
        Attribute::ClassDirective(a) => a.expression_span.map(|s| s.start),
        Attribute::StyleDirective(a) => match &a.value {
            svelte_ast::StyleDirectiveValue::Expression(span) => Some(span.start),
            _ => None,
        },
        Attribute::BindDirective(a) => a.expression_span.map(|s| s.start),
        Attribute::SpreadAttribute(a) => Some(a.expression_span.start),
        Attribute::Shorthand(a) => Some(a.expression_span.start),
        Attribute::UseDirective(a) => a.expression_span.map(|s| s.start),
        Attribute::OnDirectiveLegacy(a) => a.expression_span.map(|s| s.start),
        Attribute::TransitionDirective(a) => a.expression_span.map(|s| s.start),
        Attribute::AnimateDirective(a) => a.expression_span.map(|s| s.start),
        Attribute::AttachTag(a) => Some(a.expression_span.start),
        Attribute::StringAttribute(_) | Attribute::BooleanAttribute(_)
        | Attribute::ConcatenationAttribute(_) => None,
    }
}

fn scan_expr_arrows<'a>(
    expr: Option<&Expression<'a>>,
    scoping: &mut ComponentScoping,
    scope: ScopeId,
) {
    let Some(expr) = expr else { return };
    let mut collector = ArrowScopeCollector { scoping, scope };
    collector.visit_expression(expr);
}

struct ArrowScopeCollector<'s> {
    scoping: &'s mut ComponentScoping,
    scope: ScopeId,
}

impl<'a> Visit<'a> for ArrowScopeCollector<'_> {
    fn visit_arrow_function_expression(&mut self, arrow: &ArrowFunctionExpression<'a>) {
        let param_names = extract_arrow_param_names(&arrow.params);
        let arrow_scope = self.scoping.register_arrow_scope(arrow.span.start, self.scope, &param_names);
        let parent_scope = self.scope;
        self.scope = arrow_scope;
        for stmt in &arrow.body.statements {
            self.visit_statement(stmt);
        }
        self.scope = parent_scope;
    }
}

fn extract_arrow_param_names(params: &FormalParameters<'_>) -> Vec<String> {
    let mut collector = BindingNameCollector { names: Vec::new() };
    collector.visit_formal_parameters(params);
    collector.names
}

/// Visitor that collects all binding identifier names from a pattern.
struct BindingNameCollector {
    names: Vec<String>,
}

impl<'a> Visit<'a> for BindingNameCollector {
    fn visit_binding_identifier(&mut self, ident: &oxc_ast::ast::BindingIdentifier<'a>) {
        self.names.push(ident.name.as_str().to_string());
    }
}
