use oxc_ast::ast::{ArrowFunctionExpression, BindingPattern, Expression, FormalParameters};
use oxc_ast_visit::Visit;
use svelte_ast::{Attribute, Component, ConcatPart, Fragment, Node};
use svelte_parser::ParsedExprs;

use crate::data::{AnalysisData, FragmentKey};
use crate::js_analyze::extract_expression_info;
use crate::scope::{ComponentScoping, ScopeId};

// ---------------------------------------------------------------------------
// Combined pass: arrow scope registration + each-block index usage
// Must run after build_scoping (needs template scopes) and extract_all_expressions.
// ---------------------------------------------------------------------------

/// Single tree walk that registers arrow function scopes and computes each-block index usage.
pub(crate) fn compute_js_metadata(
    component: &Component,
    data: &mut AnalysisData,
    parsed: &ParsedExprs<'_>,
) {
    let root = data.scoping.root_scope_id();
    walk_fragment(&component.fragment, component, data, parsed, root);
}

fn walk_fragment(
    fragment: &Fragment,
    component: &Component,
    data: &mut AnalysisData,
    parsed: &ParsedExprs<'_>,
    scope: ScopeId,
) {
    for node in &fragment.nodes {
        walk_node(node, component, data, parsed, scope);
    }
}

fn walk_node(
    node: &Node,
    component: &Component,
    data: &mut AnalysisData,
    parsed: &ParsedExprs<'_>,
    scope: ScopeId,
) {
    match node {
        Node::ExpressionTag(tag) => {
            scan_expr_arrows(parsed.exprs.get(&tag.id), &mut data.scoping, scope);
        }
        Node::Element(el) => {
            scan_attrs_arrows(&el.attributes, &mut data.scoping, parsed, scope);
            walk_fragment(&el.fragment, component, data, parsed, scope);
        }
        Node::ComponentNode(cn) => {
            scan_attrs_arrows(&cn.attributes, &mut data.scoping, parsed, scope);
            walk_fragment(&cn.fragment, component, data, parsed, scope);
        }
        Node::IfBlock(block) => {
            scan_expr_arrows(parsed.exprs.get(&block.id), &mut data.scoping, scope);
            let cons_scope = data.scoping.fragment_scope(&FragmentKey::IfConsequent(block.id)).unwrap_or(scope);
            walk_fragment(&block.consequent, component, data, parsed, cons_scope);
            if let Some(alt) = &block.alternate {
                let alt_scope = data.scoping.fragment_scope(&FragmentKey::IfAlternate(block.id)).unwrap_or(scope);
                walk_fragment(alt, component, data, parsed, alt_scope);
            }
        }
        Node::EachBlock(block) => {
            scan_expr_arrows(parsed.exprs.get(&block.id), &mut data.scoping, scope);

            // --- each-block index usage (merged from compute_each_index_usage) ---
            if let Some(idx_span) = block.index_span {
                let idx_name = component.source_text(idx_span);
                if let Some(key_expr) = parsed.key_exprs.get(&block.id) {
                    let offset = parsed.key_expr_offsets.get(&block.id).copied().unwrap_or(0);
                    let info = extract_expression_info(key_expr, offset);
                    if info.references.iter().any(|r| r.name.as_str() == idx_name) {
                        data.each_blocks.key_uses_index.insert(block.id);
                    }
                }
                if check_fragment_uses_name(&block.body, idx_name, data) {
                    data.each_blocks.body_uses_index.insert(block.id);
                }
            }

            let body_scope = data.scoping.node_scope(block.id).unwrap_or(scope);
            walk_fragment(&block.body, component, data, parsed, body_scope);
            if let Some(fb) = &block.fallback {
                walk_fragment(fb, component, data, parsed, scope);
            }
        }
        Node::SnippetBlock(block) => {
            let snippet_scope = data.scoping.node_scope(block.id).unwrap_or(scope);
            walk_fragment(&block.body, component, data, parsed, snippet_scope);
        }
        Node::RenderTag(tag) => {
            scan_expr_arrows(parsed.exprs.get(&tag.id), &mut data.scoping, scope);
        }
        Node::HtmlTag(tag) => {
            scan_expr_arrows(parsed.exprs.get(&tag.id), &mut data.scoping, scope);
        }
        Node::ConstTag(tag) => {
            scan_expr_arrows(parsed.exprs.get(&tag.id), &mut data.scoping, scope);
        }
        Node::KeyBlock(block) => {
            scan_expr_arrows(parsed.exprs.get(&block.id), &mut data.scoping, scope);
            let child_scope = data.scoping.fragment_scope(&FragmentKey::KeyBlockBody(block.id)).unwrap_or(scope);
            walk_fragment(&block.fragment, component, data, parsed, child_scope);
        }
        Node::SvelteHead(head) => {
            let child_scope = data.scoping.fragment_scope(&FragmentKey::SvelteHeadBody(head.id)).unwrap_or(scope);
            walk_fragment(&head.fragment, component, data, parsed, child_scope);
        }
        Node::SvelteElement(el) => {
            if !el.static_tag {
                scan_expr_arrows(parsed.exprs.get(&el.id), &mut data.scoping, scope);
            }
            scan_attrs_arrows(&el.attributes, &mut data.scoping, parsed, scope);
            let child_scope = data.scoping.fragment_scope(&FragmentKey::SvelteElementBody(el.id)).unwrap_or(scope);
            walk_fragment(&el.fragment, component, data, parsed, child_scope);
        }
        Node::SvelteWindow(w) => {
            scan_attrs_arrows(&w.attributes, &mut data.scoping, parsed, scope);
        }
        Node::SvelteDocument(d) => {
            scan_attrs_arrows(&d.attributes, &mut data.scoping, parsed, scope);
        }
        Node::SvelteBody(b) => {
            scan_attrs_arrows(&b.attributes, &mut data.scoping, parsed, scope);
        }
        Node::SvelteBoundary(b) => {
            scan_attrs_arrows(&b.attributes, &mut data.scoping, parsed, scope);
            let child_scope = data.scoping.fragment_scope(&FragmentKey::SvelteBoundaryBody(b.id)).unwrap_or(scope);
            walk_fragment(&b.fragment, component, data, parsed, child_scope);
        }
        Node::AwaitBlock(block) => {
            scan_expr_arrows(parsed.exprs.get(&block.id), &mut data.scoping, scope);
            if let Some(ref p) = block.pending {
                let s = data.scoping.fragment_scope(&FragmentKey::AwaitPending(block.id)).unwrap_or(scope);
                walk_fragment(p, component, data, parsed, s);
            }
            if let Some(ref t) = block.then {
                let s = data.scoping.node_scope(block.id).unwrap_or(scope);
                walk_fragment(t, component, data, parsed, s);
            }
            if let Some(ref c) = block.catch {
                let s = data.scoping.await_catch_scope(block.id).unwrap_or(scope);
                walk_fragment(c, component, data, parsed, s);
            }
        }
        Node::DebugTag(_) | Node::Text(_) | Node::Comment(_) | Node::Error(_) => {}
    }
}

// ---------------------------------------------------------------------------
// check_fragment_uses_name (moved from js_analyze.rs)
// ---------------------------------------------------------------------------

/// Check if any expression in a fragment references a given name.
fn check_fragment_uses_name(fragment: &Fragment, name: &str, data: &AnalysisData) -> bool {
    for node in &fragment.nodes {
        let refs_match = |id: svelte_ast::NodeId| -> bool {
            data.expressions.get(&id)
                .is_some_and(|info| info.references.iter().any(|r| r.name.as_str() == name))
        };
        let attr_refs_match = |attrs: &[Attribute]| -> bool {
            attrs.iter().any(|a| {
                data.attr_expressions.get(&a.id())
                    .is_some_and(|info| info.references.iter().any(|r| r.name.as_str() == name))
            })
        };
        match node {
            Node::ExpressionTag(t) if refs_match(t.id) => return true,
            Node::Element(el) => {
                if attr_refs_match(&el.attributes) { return true; }
                if check_fragment_uses_name(&el.fragment, name, data) { return true; }
            }
            Node::ComponentNode(cn) => {
                if attr_refs_match(&cn.attributes) { return true; }
                if check_fragment_uses_name(&cn.fragment, name, data) { return true; }
            }
            Node::IfBlock(b) => {
                if refs_match(b.id) { return true; }
                if check_fragment_uses_name(&b.consequent, name, data) { return true; }
                if let Some(ref alt) = b.alternate {
                    if check_fragment_uses_name(alt, name, data) { return true; }
                }
            }
            Node::EachBlock(b) => {
                if refs_match(b.id) { return true; }
                if check_fragment_uses_name(&b.body, name, data) { return true; }
            }
            Node::RenderTag(t) if refs_match(t.id) => return true,
            Node::HtmlTag(t) if refs_match(t.id) => return true,
            Node::KeyBlock(b) => {
                if refs_match(b.id) { return true; }
                if check_fragment_uses_name(&b.fragment, name, data) { return true; }
            }
            Node::ConstTag(t) if refs_match(t.id) => return true,
            Node::SvelteElement(e) => {
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

fn scan_attrs_arrows(
    attrs: &[Attribute],
    scoping: &mut ComponentScoping,
    parsed: &ParsedExprs<'_>,
    scope: ScopeId,
) {
    for attr in attrs {
        let attr_id = attr.id();
        scan_expr_arrows(parsed.attr_exprs.get(&attr_id), scoping, scope);
        let concat_parts: Option<&[ConcatPart]> = match attr {
            Attribute::ConcatenationAttribute(a) => Some(&a.parts),
            Attribute::StyleDirective(a) => match &a.value {
                svelte_ast::StyleDirectiveValue::Concatenation(parts) => Some(parts),
                _ => None,
            },
            _ => None,
        };
        if let Some(parts) = concat_parts {
            let dyn_count = parts.iter().filter(|p| matches!(p, ConcatPart::Dynamic(_))).count();
            for dyn_idx in 0..dyn_count {
                scan_expr_arrows(parsed.concat_part_exprs.get(&(attr_id, dyn_idx)), scoping, scope);
            }
        }
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
    let mut names = Vec::new();
    for param in &params.items {
        collect_binding_names(&param.pattern, &mut names);
    }
    if let Some(rest) = &params.rest {
        collect_binding_names(&rest.rest.argument, &mut names);
    }
    names
}

fn collect_binding_names(pattern: &BindingPattern<'_>, names: &mut Vec<String>) {
    match pattern {
        BindingPattern::BindingIdentifier(id) => {
            names.push(id.name.as_str().to_string());
        }
        BindingPattern::ObjectPattern(obj) => {
            for prop in &obj.properties {
                collect_binding_names(&prop.value, names);
            }
            if let Some(rest) = &obj.rest {
                collect_binding_names(&rest.argument, names);
            }
        }
        BindingPattern::ArrayPattern(arr) => {
            for elem in arr.elements.iter().flatten() {
                collect_binding_names(elem, names);
            }
            if let Some(rest) = &arr.rest {
                collect_binding_names(&rest.argument, names);
            }
        }
        BindingPattern::AssignmentPattern(assign) => {
            collect_binding_names(&assign.left, names);
        }
    }
}
