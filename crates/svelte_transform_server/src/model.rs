use std::mem;

use oxc_allocator::Vec as OxcVec;
use oxc_ast::ast::{Expression, Statement, VariableDeclarator};
use oxc_ast_visit::{VisitMut, walk_mut};
use svelte_analyze::{AnalysisData, DeclaratorSemantics};
use svelte_ast_builder::Builder;
use svelte_emit_builders::server_refs;

pub(crate) struct ServerTransform<'b, 'a> {
    pub b: &'b Builder<'a>,
    pub analysis: &'b AnalysisData<'a>,
}

impl<'a> VisitMut<'a> for ServerTransform<'_, 'a> {
    fn visit_statements(&mut self, it: &mut OxcVec<'a, Statement<'a>>) {
        self.rewrite_statements(it);
        walk_mut::walk_statements(self, it);
    }

    fn visit_variable_declarator(&mut self, it: &mut VariableDeclarator<'a>) {
        self.declarator(it);
        walk_mut::walk_variable_declarator(self, it);
    }

    fn visit_expression(&mut self, it: &mut Expression<'a>) {
        walk_mut::walk_expression(self, it);
        server_refs::rewrite_identifier_read(self.b, self.analysis, it);
    }
}

impl<'a> ServerTransform<'_, 'a> {
    fn rewrite_statements(&mut self, it: &mut OxcVec<'a, Statement<'a>>) {
        let topo: Vec<_> = self
            .analysis
            .reactivity
            .legacy_reactive()
            .iter_statements_topo()
            .map(|statement| statement.stmt_node)
            .collect();

        let mut out = self.b.ast.vec_with_capacity(it.len());
        let mut reactive: Vec<(usize, Statement<'a>)> = Vec::new();
        for stmt in it.drain(..) {
            let stmt = match stmt {
                Statement::ExportNamedDeclaration(export) => match export.unbox().declaration {
                    Some(decl) => Statement::from(decl),
                    None => continue,
                },
                other => other,
            };
            if !self.keep_statement(&stmt) {
                continue;
            }
            let rank = match &stmt {
                Statement::LabeledStatement(labeled) => {
                    topo.iter().position(|node| *node == labeled.node_id())
                }
                _ => None,
            };
            match rank {
                Some(rank) => reactive.push((rank, stmt)),
                None => out.push(stmt),
            }
        }
        reactive.sort_by_key(|(rank, _)| *rank);
        out.extend(reactive.into_iter().map(|(_, stmt)| stmt));
        mem::swap(it, &mut out);
    }

    fn declarator(&mut self, declarator: &mut VariableDeclarator<'a>) {
        match self.analysis.declarator_semantics(declarator.node_id()) {
            DeclaratorSemantics::RuneState { kind } => self.rewrite_state(declarator, kind),
            DeclaratorSemantics::LegacyProps => self.rewrite_legacy_prop(declarator),
            DeclaratorSemantics::RuneProps => self.rewrite_rune_props(declarator),
            DeclaratorSemantics::None
            | DeclaratorSemantics::RuntimeRuneCall { .. }
            | DeclaratorSemantics::LegacyState
            | DeclaratorSemantics::RuneDerived { .. }
            | DeclaratorSemantics::ConstTag { .. }
            | DeclaratorSemantics::LetCarrier { .. }
            | DeclaratorSemantics::EachItem
            | DeclaratorSemantics::AwaitValue
            | DeclaratorSemantics::SnippetParam
            | DeclaratorSemantics::ClassFieldState(_)
            | DeclaratorSemantics::ClassFieldDerived(_) => {}
        }
    }
}
