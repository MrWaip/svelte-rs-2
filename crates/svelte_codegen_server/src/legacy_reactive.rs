use oxc_ast::ast::Statement;
use svelte_analyze::reactivity_semantics::legacy_reactive::LegacyReactiveKind;

use crate::model::ServerCodegen;

impl<'a> ServerCodegen<'a> {
    pub(crate) fn legacy_reactive_hoist(&self) -> Option<Statement<'a>> {
        let legacy_reactive = self.analysis.reactivity.legacy_reactive();
        let mut names: Vec<&str> = Vec::new();
        for statement in legacy_reactive.iter_statements_topo() {
            match &statement.kind {
                LegacyReactiveKind::SimpleAssignment {
                    target_sym: target_symbol,
                    implicit_decl: true,
                } => push_name(self, &mut names, *target_symbol),
                LegacyReactiveKind::DestructureAssignment {
                    implicit_decl_syms, ..
                } => {
                    for symbol in implicit_decl_syms {
                        push_name(self, &mut names, *symbol);
                    }
                }
                LegacyReactiveKind::SimpleAssignment { .. }
                | LegacyReactiveKind::Block
                | LegacyReactiveKind::ExpressionOnly
                | LegacyReactiveKind::Conditional => {}
            }
        }
        if names.is_empty() {
            return None;
        }
        Some(self.b.let_names_stmt(&names))
    }
}

fn push_name<'a>(
    codegen: &ServerCodegen<'a>,
    names: &mut Vec<&'a str>,
    symbol: svelte_component_semantics::SymbolId,
) {
    let name = codegen.analysis.scoping.symbol_name(symbol);
    if !names.contains(&name) {
        names.push(name);
    }
}
