use std::mem;

use oxc_allocator::Vec as OxcVec;
use oxc_ast::NONE;
use oxc_ast::ast::{
    ArrowFunctionExpression, Class, Declaration, ExportNamedDeclaration, Expression, Function,
    ObjectProperty, Program, PropertyKind, Statement, VariableDeclaration, VariableDeclarator,
};
use oxc_ast_visit::{VisitMut, walk_mut};
use oxc_semantic::ScopeFlags;
use oxc_span::GetSpan;
use svelte_analyze::{AnalysisData, DeclaratorSemantics, IdentGen};
use svelte_ast_builder::Builder;
use svelte_emit_builders::server_refs;

use crate::effect::RuneStatement;

pub(crate) struct ServerTransform<'b, 'a> {
    pub b: &'b Builder<'a>,
    pub analysis: &'b AnalysisData<'a>,
    pub ident_gen: &'b mut IdentGen,
    pub fn_depth: u32,
    pub dev: bool,
    pub strip_exports: bool,
    pub enclosing_stmt_start: Vec<u32>,
}

impl<'a> ServerTransform<'_, 'a> {
    pub(crate) fn is_in_ignored_stmt(&self, code: &str) -> bool {
        let Some(&start) = self.enclosing_stmt_start.last() else {
            return false;
        };
        self.analysis
            .output
            .ignore_data
            .is_ignored_at_span(start, code)
    }
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

    fn visit_statement(&mut self, it: &mut Statement<'a>) {
        self.enclosing_stmt_start.push(it.span().start);
        walk_mut::walk_statement(self, it);
        self.enclosing_stmt_start.pop();
    }

    fn visit_variable_declaration(&mut self, it: &mut VariableDeclaration<'a>) {
        self.expand_rune_destructure_declaration(it);
        self.expand_legacy_destructure(it);
        walk_mut::walk_variable_declaration(self, it);
    }

    fn visit_variable_declarator(&mut self, it: &mut VariableDeclarator<'a>) {
        self.declarator(it);
        walk_mut::walk_variable_declarator(self, it);
        self.finish_legacy_prop(it);
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
        if self.rewrite_derived_write(it) {
            return;
        }
        if self.rewrite_private_derived_write(it) {
            return;
        }
        self.rewrite_runtime_rune_call(it);
        let member_mutation = self.detect_store_member_mutation(it);
        walk_mut::walk_expression(self, it);
        self.rewrite_await(it);
        self.rewrite_private_derived_read(it);
        server_refs::rewrite_identifier_read(self.b, self.analysis, it);
        if let Some((dollar_name, base_sym)) = member_mutation {
            self.apply_store_member_mutation(it, &dollar_name, base_sym);
        }
    }

    fn visit_class(&mut self, it: &mut Class<'a>) {
        walk_mut::walk_class(self, it);
        self.rewrite_server_class(it);
    }

    fn visit_object_property(&mut self, it: &mut ObjectProperty<'a>) {
        if !it.method
            && it.kind == PropertyKind::Init
            && matches!(&it.value, Expression::FunctionExpression(_))
        {
            it.method = true;
        }
        walk_mut::walk_object_property(self, it);
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

    fn is_props_id_declaration(&self, stmt: &Statement<'a>) -> bool {
        let Statement::VariableDeclaration(decl) = stmt else {
            return false;
        };
        for declarator in &decl.declarations {
            let Some(Expression::CallExpression(call)) = declarator.init.as_ref() else {
                continue;
            };
            if self
                .analysis
                .declarator_semantics(call.node_id())
                .is_props_id_call()
            {
                return true;
            }
        }
        false
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
        let mut props_id: Option<Statement<'a>> = None;
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
            if props_id.is_none() && self.is_props_id_declaration(&stmt) {
                props_id = Some(stmt);
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
        if let Some(props_id) = props_id {
            out.insert(0, props_id);
        }
        mem::swap(it, &mut out);
    }

    fn expand_rune_destructure_declaration(&mut self, it: &mut VariableDeclaration<'a>) {
        let mut rebuilt = self.b.ast.vec_with_capacity(it.declarations.len());
        for mut declarator in it.declarations.drain(..) {
            match self.expand_rune_destructure(&mut declarator) {
                Some(expanded) => rebuilt.extend(expanded),
                None => rebuilt.push(declarator),
            }
        }
        it.declarations = rebuilt;
    }

    fn declarator(&mut self, declarator: &mut VariableDeclarator<'a>) {
        if let Some(init) = declarator.init.as_mut()
            && self.elide_state_snapshot_init(init)
        {
            return;
        }
        if self.legacy_prop_symbol(declarator).is_some() {
            return;
        }
        match self.analysis.declarator_semantics(declarator.node_id()) {
            DeclaratorSemantics::RuneState { kind } => self.rewrite_state(declarator, kind),
            DeclaratorSemantics::RuneProps => self.rewrite_rune_props(declarator),
            DeclaratorSemantics::RuneDerived {
                kind,
                async_kind,
                source,
            } => self.rewrite_derived(declarator, kind, async_kind, source),
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
