use oxc_allocator::Vec as OxcVec;
use oxc_ast::ast::{Statement, VariableDeclarator};
use oxc_ast_visit::{VisitMut, walk_mut};
use svelte_analyze::{AnalysisData, DeclaratorSemantics};
use svelte_ast_builder::Builder;

pub(crate) struct ServerTransform<'b, 'a> {
    pub b: &'b Builder<'a>,
    pub analysis: &'b AnalysisData<'a>,
}

impl<'a> VisitMut<'a> for ServerTransform<'_, 'a> {
    fn visit_statements(&mut self, it: &mut OxcVec<'a, Statement<'a>>) {
        it.retain(|stmt| self.keep_statement(stmt));
        walk_mut::walk_statements(self, it);
    }

    fn visit_variable_declarator(&mut self, it: &mut VariableDeclarator<'a>) {
        self.declarator(it);
        walk_mut::walk_variable_declarator(self, it);
    }
}

impl<'a> ServerTransform<'_, 'a> {
    fn declarator(&mut self, declarator: &mut VariableDeclarator<'a>) {
        match self.analysis.declarator_semantics(declarator.node_id()) {
            DeclaratorSemantics::RuneState { kind } => self.rewrite_state(declarator, kind),
            DeclaratorSemantics::None
            | DeclaratorSemantics::RuntimeRuneCall { .. }
            | DeclaratorSemantics::RuneProps
            | DeclaratorSemantics::LegacyProps
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
