use oxc_ast::{Visit, ast::IdentifierReference};
use oxc_semantic::ScopeId;

use crate::{AnalyzeHir, HirAnalyses, SvelteRune, analyze_script::SvelteRuneKind};

struct Visitor<'hir> {
    analyses: &'hir mut HirAnalyses,
    scope_id: ScopeId,
}

impl<'hir> AnalyzeHir<'hir> {
    pub(crate) fn scope_adding_pass(
        &self,
        analyses: &mut HirAnalyses,
        store: &hir::HirStore<'hir>,
    ) {
        for node in store.nodes.iter() {
            match node {
                hir::Node::EachBlock(it) => {
                    let scope_id = analyses.add_scope();
                    let mut visit = Visitor::new(analyses, scope_id);

                    it.scope_id.set(Some(scope_id));

                    let expression = store.get_expression(it.item);
                    visit.visit_expression(&expression);
                }
                _ => (),
            };
        }
    }
}

impl<'hir> Visitor<'hir> {
    pub(crate) fn new(analyses: &'hir mut HirAnalyses, scope_id: ScopeId) -> Self {
        Self { analyses, scope_id }
    }
}

impl<'hir> Visit<'hir> for Visitor<'hir> {
    fn visit_identifier_reference(&mut self, it: &IdentifierReference<'hir>) {
        let symbol_id = self.analyses.add_binding(&it.name, self.scope_id);

        self.analyses.add_rune(
            symbol_id,
            SvelteRune {
                kind: SvelteRuneKind::State,
                mutated: false,
            },
        );
    }
}
