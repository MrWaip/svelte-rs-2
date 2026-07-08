use std::mem;

use oxc_allocator::Vec as OxcVec;
use oxc_ast::NONE;
use oxc_ast::ast::{
    ArrowFunctionExpression, BindingPattern, Declaration, ExportNamedDeclaration, Expression,
    Function, Program, Statement, VariableDeclaration, VariableDeclarator,
};
use oxc_ast_visit::{VisitMut, walk_mut};
use oxc_semantic::ScopeFlags;
use svelte_analyze::{AnalysisData, DeclaratorSemantics, IdentGen};
use svelte_ast_builder::Builder;
use svelte_emit_builders::server_refs;

use crate::derived::expand_derived_destructure_declarators;
use crate::effect::RuneStatement;

pub(crate) struct ServerTransform<'b, 'a> {
    pub b: &'b Builder<'a>,
    pub analysis: &'b AnalysisData<'a>,
    pub ident_gen: &'b mut IdentGen,
    pub fn_depth: u32,
    pub dev: bool,
    pub strip_exports: bool,
}

impl<'a> VisitMut<'a> for ServerTransform<'_, 'a> {
    fn visit_program(&mut self, it: &mut Program<'a>) {
        self.split_top_level_multi_declarators(&mut it.body);
        walk_mut::walk_program(self, it);
    }

    fn visit_statements(&mut self, it: &mut OxcVec<'a, Statement<'a>>) {
        self.rewrite_statements(it);
        walk_mut::walk_statements(self, it);
    }

    fn visit_variable_declaration(&mut self, it: &mut VariableDeclaration<'a>) {
        if self.fn_depth > 0 {
            self.expand_nested_derived_destructure(it);
        }
        self.expand_legacy_destructure(it);
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
        if self.rewrite_store_mutation(it) {
            return;
        }
        let member_mutation = self.detect_store_member_mutation(it);
        walk_mut::walk_expression(self, it);
        self.rewrite_await(it);
        server_refs::rewrite_identifier_read(self.b, self.analysis, it);
        if let Some((dollar_name, base_sym)) = member_mutation {
            self.apply_store_member_mutation(it, &dollar_name, base_sym);
        }
    }
}

impl<'a> ServerTransform<'_, 'a> {
    fn split_top_level_multi_declarators(&mut self, it: &mut OxcVec<'a, Statement<'a>>) {
        let mut out = self.b.ast.vec_with_capacity(it.len());
        for stmt in it.drain(..) {
            match stmt {
                Statement::VariableDeclaration(decl) if decl.declarations.len() > 1 => {
                    for single in self.split_variable_declaration(decl.unbox()) {
                        out.push(Statement::VariableDeclaration(self.b.alloc(single)));
                    }
                }
                Statement::ExportNamedDeclaration(export)
                    if is_multi_declarator_export(&export) =>
                {
                    let export = export.unbox();
                    let export_kind = export.export_kind;
                    let export_span = export.span;
                    let Some(Declaration::VariableDeclaration(decl)) = export.declaration else {
                        continue;
                    };
                    for single in self.split_variable_declaration(decl.unbox()) {
                        let declaration = Declaration::VariableDeclaration(self.b.alloc(single));
                        let split = self.b.ast.export_named_declaration(
                            export_span,
                            Some(declaration),
                            self.b.ast.vec(),
                            None,
                            export_kind,
                            NONE,
                        );
                        out.push(Statement::ExportNamedDeclaration(self.b.alloc(split)));
                    }
                }
                other => out.push(other),
            }
        }
        mem::swap(it, &mut out);
    }

    fn split_variable_declaration(
        &self,
        decl: VariableDeclaration<'a>,
    ) -> Vec<VariableDeclaration<'a>> {
        let kind = decl.kind;
        let span = decl.span;
        let declare = decl.declare;
        let mut out = Vec::with_capacity(decl.declarations.len());
        for declarator in decl.declarations {
            let mut single = self.b.ast.vec_with_capacity(1);
            single.push(declarator);
            out.push(self.b.ast.variable_declaration(span, kind, single, declare));
        }
        out
    }

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
                Statement::ExportNamedDeclaration(export) if self.strip_exports => {
                    match export.unbox().declaration {
                        Some(decl) => Statement::from(decl),
                        None => continue,
                    }
                }
                other => other,
            };
            let stmt = match self.rewrite_rune_statement(stmt) {
                RuneStatement::Keep(stmt) => stmt,
                RuneStatement::Replace(stmts) => {
                    out.extend(stmts);
                    continue;
                }
            };
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
        if let BindingPattern::BindingIdentifier(id) = &declarator.id
            && let Some(sym) = id.symbol_id.get()
            && self.analysis.binding_semantics(sym).is_legacy_prop()
        {
            self.rewrite_legacy_prop(declarator);
            return;
        }
        match self.analysis.declarator_semantics(declarator.node_id()) {
            DeclaratorSemantics::RuneState { kind } => self.rewrite_state(declarator, kind),
            DeclaratorSemantics::RuneProps => self.rewrite_rune_props(declarator),
            DeclaratorSemantics::RuneDerived {
                kind, async_kind, ..
            } => self.rewrite_derived(declarator, kind, async_kind),
            DeclaratorSemantics::LegacyProps
            | DeclaratorSemantics::None
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

fn is_multi_declarator_export(export: &ExportNamedDeclaration<'_>) -> bool {
    matches!(
        &export.declaration,
        Some(Declaration::VariableDeclaration(decl)) if decl.declarations.len() > 1
    )
}
