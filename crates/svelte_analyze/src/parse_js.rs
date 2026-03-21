use oxc_ast::ast::{ArrowFunctionExpression, BindingPattern, Expression, FormalParameters};
use oxc_ast_visit::Visit;
use svelte_ast::{Attribute, Component, ConcatPart, Fragment, Node};
use svelte_types::ParsedExprs;

use crate::data::{AnalysisData, FragmentKey};
use crate::scope::{ComponentScoping, ScopeId};

// ---------------------------------------------------------------------------
// Arrow scope registration (runs after build_scoping)
// ---------------------------------------------------------------------------

/// Walk all parsed OXC expressions and register arrow function parameter scopes.
/// Must be called after `build_scoping` so that template scopes exist as parents.
pub(crate) fn register_arrow_scopes(
    component: &Component,
    data: &mut AnalysisData,
    parsed: &ParsedExprs<'_>,
) {
    let root = data.scoping.root_scope_id();
    walk_fragment_arrows(&component.fragment, component, &mut data.scoping, parsed, root);
}

fn walk_fragment_arrows(
    fragment: &Fragment,
    component: &Component,
    scoping: &mut ComponentScoping,
    parsed: &ParsedExprs<'_>,
    scope: ScopeId,
) {
    for node in &fragment.nodes {
        walk_node_arrows(node, component, scoping, parsed, scope);
    }
}

fn walk_node_arrows(
    node: &Node,
    component: &Component,
    scoping: &mut ComponentScoping,
    parsed: &ParsedExprs<'_>,
    scope: ScopeId,
) {
    match node {
        Node::ExpressionTag(tag) => {
            scan_expr_arrows(parsed.exprs.get(&tag.id), scoping, scope);
        }
        Node::Element(el) => {
            scan_attrs_arrows(&el.attributes, scoping, parsed, scope);
            walk_fragment_arrows(&el.fragment, component, scoping, parsed, scope);
        }
        Node::ComponentNode(cn) => {
            scan_attrs_arrows(&cn.attributes, scoping, parsed, scope);
            walk_fragment_arrows(&cn.fragment, component, scoping, parsed, scope);
        }
        Node::IfBlock(block) => {
            scan_expr_arrows(parsed.exprs.get(&block.id), scoping, scope);
            let cons_scope = scoping.fragment_scope(&FragmentKey::IfConsequent(block.id)).unwrap_or(scope);
            walk_fragment_arrows(&block.consequent, component, scoping, parsed, cons_scope);
            if let Some(alt) = &block.alternate {
                let alt_scope = scoping.fragment_scope(&FragmentKey::IfAlternate(block.id)).unwrap_or(scope);
                walk_fragment_arrows(alt, component, scoping, parsed, alt_scope);
            }
        }
        Node::EachBlock(block) => {
            scan_expr_arrows(parsed.exprs.get(&block.id), scoping, scope);
            let body_scope = scoping.node_scope(block.id).unwrap_or(scope);
            walk_fragment_arrows(&block.body, component, scoping, parsed, body_scope);
            if let Some(fb) = &block.fallback {
                walk_fragment_arrows(fb, component, scoping, parsed, scope);
            }
        }
        Node::SnippetBlock(block) => {
            let snippet_scope = scoping.node_scope(block.id).unwrap_or(scope);
            walk_fragment_arrows(&block.body, component, scoping, parsed, snippet_scope);
        }
        Node::RenderTag(tag) => {
            scan_expr_arrows(parsed.exprs.get(&tag.id), scoping, scope);
        }
        Node::HtmlTag(tag) => {
            scan_expr_arrows(parsed.exprs.get(&tag.id), scoping, scope);
        }
        Node::ConstTag(tag) => {
            scan_expr_arrows(parsed.exprs.get(&tag.id), scoping, scope);
        }
        Node::KeyBlock(block) => {
            scan_expr_arrows(parsed.exprs.get(&block.id), scoping, scope);
            let child_scope = scoping.fragment_scope(&FragmentKey::KeyBlockBody(block.id)).unwrap_or(scope);
            walk_fragment_arrows(&block.fragment, component, scoping, parsed, child_scope);
        }
        Node::SvelteHead(head) => {
            let child_scope = scoping.fragment_scope(&FragmentKey::SvelteHeadBody(head.id)).unwrap_or(scope);
            walk_fragment_arrows(&head.fragment, component, scoping, parsed, child_scope);
        }
        Node::SvelteElement(el) => {
            if !el.static_tag {
                scan_expr_arrows(parsed.exprs.get(&el.id), scoping, scope);
            }
            scan_attrs_arrows(&el.attributes, scoping, parsed, scope);
            let child_scope = scoping.fragment_scope(&FragmentKey::SvelteElementBody(el.id)).unwrap_or(scope);
            walk_fragment_arrows(&el.fragment, component, scoping, parsed, child_scope);
        }
        Node::SvelteWindow(w) => {
            scan_attrs_arrows(&w.attributes, scoping, parsed, scope);
        }
        Node::SvelteDocument(d) => {
            scan_attrs_arrows(&d.attributes, scoping, parsed, scope);
        }
        Node::SvelteBody(b) => {
            scan_attrs_arrows(&b.attributes, scoping, parsed, scope);
        }
        Node::SvelteBoundary(b) => {
            scan_attrs_arrows(&b.attributes, scoping, parsed, scope);
            let child_scope = scoping.fragment_scope(&FragmentKey::SvelteBoundaryBody(b.id)).unwrap_or(scope);
            walk_fragment_arrows(&b.fragment, component, scoping, parsed, child_scope);
        }
        Node::AwaitBlock(block) => {
            scan_expr_arrows(parsed.exprs.get(&block.id), scoping, scope);
            if let Some(ref p) = block.pending {
                let s = scoping.fragment_scope(&FragmentKey::AwaitPending(block.id)).unwrap_or(scope);
                walk_fragment_arrows(p, component, scoping, parsed, s);
            }
            if let Some(ref t) = block.then {
                let s = scoping.node_scope(block.id).unwrap_or(scope);
                walk_fragment_arrows(t, component, scoping, parsed, s);
            }
            if let Some(ref c) = block.catch {
                let s = scoping.await_catch_scope(block.id).unwrap_or(scope);
                walk_fragment_arrows(c, component, scoping, parsed, s);
            }
        }
        Node::DebugTag(_) | Node::Text(_) | Node::Comment(_) | Node::Error(_) => {}
    }
}

fn scan_attrs_arrows(
    attrs: &[Attribute],
    scoping: &mut ComponentScoping,
    parsed: &ParsedExprs<'_>,
    scope: ScopeId,
) {
    for attr in attrs {
        let attr_id = attr.id();
        scan_expr_arrows(parsed.attr_exprs.get(&attr_id), scoping, scope);
        // Concat part expressions
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

/// Scan a single OXC Expression for ArrowFunctionExpressions, registering scopes.
fn scan_expr_arrows<'a>(
    expr: Option<&Expression<'a>>,
    scoping: &mut ComponentScoping,
    scope: ScopeId,
) {
    let Some(expr) = expr else { return };
    let mut collector = ArrowScopeCollector { scoping, scope };
    collector.visit_expression(expr);
}

/// Visit-based collector: walks OXC expression AST, registers arrow param scopes.
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
