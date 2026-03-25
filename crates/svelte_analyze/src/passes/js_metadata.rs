use oxc_ast::ast::{ArrowFunctionExpression, Expression, FormalParameters};
use oxc_ast_visit::Visit;
use svelte_ast::NodeId;

use crate::scope::ComponentScoping;
use crate::walker::{TemplateVisitor, VisitContext};

// ---------------------------------------------------------------------------
// JsMetadataVisitor — arrow scope registration
// Merged into resolve_references composite walk.
//
// Arrow scanning is handled generically via visit_js_expression:
// the walker dispatches it for every parsed expression in the template,
// so no per-node-type boilerplate is needed.
//
// Each-block index usage detection moved to post_resolve (SymbolId-based).
// ---------------------------------------------------------------------------

pub(crate) struct JsMetadataVisitor;

impl TemplateVisitor for JsMetadataVisitor {
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
