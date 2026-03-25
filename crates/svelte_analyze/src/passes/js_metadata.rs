use oxc_ast::ast::{ArrowFunctionExpression, Expression, FormalParameters};
use oxc_ast_visit::Visit;
use svelte_ast::{Attribute, EachBlock, NodeId};

use crate::passes::js_analyze::analyze_expression;
use crate::scope::ComponentScoping;
use crate::types::data::AnalysisData;
use crate::walker::{TemplateVisitor, VisitContext};

// ---------------------------------------------------------------------------
// JsMetadataVisitor — arrow scope registration + each-block index usage
// Merged into resolve_references composite walk.
//
// Arrow scanning is handled generically via visit_js_expression:
// the walker dispatches it for every parsed expression in the template,
// so no per-node-type boilerplate is needed.
// ---------------------------------------------------------------------------

pub(crate) struct JsMetadataVisitor<'a> {
    pub component: &'a svelte_ast::Component,
}

impl TemplateVisitor for JsMetadataVisitor<'_> {
    fn visit_js_expression(
        &mut self,
        _id: NodeId,
        expr: &Expression<'_>,
        ctx: &mut VisitContext<'_>,
    ) {
        let mut collector = ArrowScopeCollector {
            scoping: &mut ctx.data.scoping,
            scope: ctx.scope,
        };
        collector.visit_expression(expr);
    }

    fn visit_each_block(&mut self, block: &EachBlock, ctx: &mut VisitContext<'_>) {
        let Some(idx_span) = block.index_span else {
            return;
        };
        let idx_name = self.component.source_text(idx_span);
        if let Some(key_span) = block.key_span {
            let parsed = ctx.parsed().unwrap();
            if let Some(key_expr) = parsed.exprs.get(&key_span.start) {
                let info = analyze_expression(key_expr);
                if info.references.iter().any(|r| r.name.as_str() == idx_name) {
                    ctx.data.each_blocks.key_uses_index.insert(block.id);
                }
            }
        }
        if check_fragment_uses_name(&block.body, idx_name, ctx.data) {
            ctx.data.each_blocks.body_uses_index.insert(block.id);
        }
    }
}

// ---------------------------------------------------------------------------
// check_fragment_uses_name (sub-walk for each-block index detection)
// ---------------------------------------------------------------------------

/// Check if any expression in a fragment references a given name.
fn check_fragment_uses_name(
    fragment: &svelte_ast::Fragment,
    name: &str,
    data: &AnalysisData,
) -> bool {
    for node in &fragment.nodes {
        let refs_match = |id: svelte_ast::NodeId| -> bool {
            data.expressions
                .get(id)
                .is_some_and(|info| info.references.iter().any(|r| r.name.as_str() == name))
        };
        let attr_refs_match = |attrs: &[Attribute]| -> bool {
            attrs.iter().any(|a| {
                data.attr_expressions
                    .get(a.id())
                    .is_some_and(|info| info.references.iter().any(|r| r.name.as_str() == name))
            })
        };
        match node {
            svelte_ast::Node::ExpressionTag(t) if refs_match(t.id) => return true,
            svelte_ast::Node::Element(el) => {
                if attr_refs_match(&el.attributes) {
                    return true;
                }
                if check_fragment_uses_name(&el.fragment, name, data) {
                    return true;
                }
            }
            svelte_ast::Node::ComponentNode(cn) => {
                if attr_refs_match(&cn.attributes) {
                    return true;
                }
                if check_fragment_uses_name(&cn.fragment, name, data) {
                    return true;
                }
            }
            svelte_ast::Node::IfBlock(b) => {
                if refs_match(b.id) {
                    return true;
                }
                if check_fragment_uses_name(&b.consequent, name, data) {
                    return true;
                }
                if let Some(ref alt) = b.alternate {
                    if check_fragment_uses_name(alt, name, data) {
                        return true;
                    }
                }
            }
            svelte_ast::Node::EachBlock(b) => {
                if refs_match(b.id) {
                    return true;
                }
                if check_fragment_uses_name(&b.body, name, data) {
                    return true;
                }
            }
            svelte_ast::Node::RenderTag(t) if refs_match(t.id) => return true,
            svelte_ast::Node::HtmlTag(t) if refs_match(t.id) => return true,
            svelte_ast::Node::KeyBlock(b) => {
                if refs_match(b.id) {
                    return true;
                }
                if check_fragment_uses_name(&b.fragment, name, data) {
                    return true;
                }
            }
            svelte_ast::Node::ConstTag(t) if refs_match(t.id) => return true,
            svelte_ast::Node::SvelteElement(e) => {
                if !e.static_tag && refs_match(e.id) {
                    return true;
                }
                if attr_refs_match(&e.attributes) {
                    return true;
                }
                if check_fragment_uses_name(&e.fragment, name, data) {
                    return true;
                }
            }
            _ => {}
        }
    }
    false
}

// ---------------------------------------------------------------------------
// Arrow scope collector
// ---------------------------------------------------------------------------

struct ArrowScopeCollector<'s> {
    scoping: &'s mut ComponentScoping,
    scope: oxc_semantic::ScopeId,
}

impl<'a> Visit<'a> for ArrowScopeCollector<'_> {
    fn visit_arrow_function_expression(&mut self, arrow: &ArrowFunctionExpression<'a>) {
        let param_names = extract_arrow_param_names(&arrow.params);
        let arrow_scope =
            self.scoping
                .register_arrow_scope(arrow.span.start, self.scope, &param_names);
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
