use std::mem;

use oxc_allocator::Vec as OxcVec;
use oxc_ast::ast::{
    ArrowFunctionExpression, Expression, Function, Statement, VariableDeclaration,
    VariableDeclarator,
};
use oxc_ast_visit::{VisitMut, walk_mut};
use oxc_semantic::ScopeFlags;
use svelte_analyze::{AnalysisData, DeclaratorSemantics, IdentGen};
use svelte_ast_builder::Builder;
use svelte_emit_builders::server_refs;

use crate::derived::expand_derived_destructure_declarators;

pub(crate) struct ServerTransform<'b, 'a> {
    pub b: &'b Builder<'a>,
    pub analysis: &'b AnalysisData<'a>,
    pub ident_gen: &'b mut IdentGen,
    pub fn_depth: u32,
}

impl<'a> VisitMut<'a> for ServerTransform<'_, 'a> {
    fn visit_statements(&mut self, it: &mut OxcVec<'a, Statement<'a>>) {
        self.rewrite_statements(it);
        walk_mut::walk_statements(self, it);
    }

    fn visit_variable_declaration(&mut self, it: &mut VariableDeclaration<'a>) {
        if self.fn_depth > 0 {
            self.expand_nested_derived_destructure(it);
        }
        walk_mut::walk_variable_declaration(self, it);
    }

    fn visit_variable_declarator(&mut self, it: &mut VariableDeclarator<'a>) {
        self.declarator(it);
        walk_mut::walk_variable_declarator(self, it);
    }

    fn visit_function(&mut self, it: &mut Function<'a>, flags: ScopeFlags) {
        self.fn_depth += 1;
        walk_mut::walk_function(self, it, flags);
        self.fn_depth -= 1;
    }

    fn visit_arrow_function_expression(&mut self, it: &mut ArrowFunctionExpression<'a>) {
        self.fn_depth += 1;
        walk_mut::walk_arrow_function_expression(self, it);
        self.fn_depth -= 1;
    }

    fn visit_expression(&mut self, it: &mut Expression<'a>) {
        walk_mut::walk_expression(self, it);
        self.rewrite_await(it);
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

    fn expand_nested_derived_destructure(&mut self, it: &mut VariableDeclaration<'a>) {
        let mut rebuilt = self.b.ast.vec_with_capacity(it.declarations.len());
        for mut declarator in it.declarations.drain(..) {
            match expand_derived_destructure_declarators(
                self.b,
                self.analysis,
                self.ident_gen,
                &mut declarator,
            ) {
                Some(expanded) => rebuilt.extend(expanded),
                None => rebuilt.push(declarator),
            }
        }
        it.declarations = rebuilt;
    }

    fn declarator(&mut self, declarator: &mut VariableDeclarator<'a>) {
        match self.analysis.declarator_semantics(declarator.node_id()) {
            DeclaratorSemantics::RuneState { kind } => self.rewrite_state(declarator, kind),
            DeclaratorSemantics::LegacyProps => self.rewrite_legacy_prop(declarator),
            DeclaratorSemantics::RuneProps => self.rewrite_rune_props(declarator),
            DeclaratorSemantics::RuneDerived {
                kind, async_kind, ..
            } => self.rewrite_derived(declarator, kind, async_kind),
            DeclaratorSemantics::None
            | DeclaratorSemantics::RuntimeRuneCall { .. }
            | DeclaratorSemantics::LegacyState
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
