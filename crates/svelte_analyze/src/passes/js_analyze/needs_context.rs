use oxc_ast::ast::{
    CallExpression, Expression, IdentifierReference, MemberExpression, NewExpression, Program,
};
use oxc_ast_visit::Visit;
use oxc_ast_visit::walk::{walk_call_expression, walk_member_expression};

use crate::reactivity_semantics::data::{BindingSemantics, ReactivitySemantics};
use crate::scope::{ComponentScoping, SymbolId};

pub(crate) struct NeedsContextVisitor<'a> {
    scoping: &'a ComponentScoping<'a>,
    reactivity: &'a ReactivitySemantics,
    needs_context: bool,
}

impl<'a> NeedsContextVisitor<'a> {
    pub(crate) fn check(
        program: &Program<'a>,
        scoping: &'a ComponentScoping,
        reactivity: &'a ReactivitySemantics,
    ) -> bool {
        let mut visitor = Self {
            scoping,
            reactivity,
            needs_context: false,
        };
        visitor.visit_program(program);
        visitor.needs_context
    }

    fn resolve_ref(
        &self,
        ident: &IdentifierReference<'_>,
    ) -> Option<SymbolId> {
        let ref_id = ident.reference_id.get()?;
        self.scoping.get_reference(ref_id).symbol_id()
    }

    fn is_safe_sym(&self, ident: &IdentifierReference<'_>) -> bool {
        let Some(sym_id) = self.resolve_ref(ident) else {
            return true;
        };
        !matches!(
            self.reactivity.binding_semantics(sym_id),
            BindingSemantics::MaybeReactive
                | BindingSemantics::Prop(_)
                | BindingSemantics::LegacyBindableProp(_)
        )
    }

    fn is_safe_expression_root(&self, expr: &Expression<'_>) -> bool {
        let mut node = expr;
        loop {
            match node {
                Expression::StaticMemberExpression(m) => node = &m.object,
                Expression::ComputedMemberExpression(m) => node = &m.object,
                Expression::TSAsExpression(t) => node = &t.expression,
                Expression::TSSatisfiesExpression(t) => node = &t.expression,
                Expression::TSNonNullExpression(t) => node = &t.expression,
                Expression::TSTypeAssertion(t) => node = &t.expression,
                Expression::TSInstantiationExpression(t) => node = &t.expression,
                _ => break,
            }
        }
        match node {
            Expression::Identifier(ident) => self.is_safe_sym(ident),
            _ => false,
        }
    }
}

impl<'a> Visit<'a> for NeedsContextVisitor<'a> {
    fn visit_new_expression(&mut self, _it: &NewExpression<'a>) {
        self.needs_context = true;
    }

    fn visit_call_expression(&mut self, it: &CallExpression<'a>) {
        if !self.is_safe_expression_root(&it.callee) {
            self.needs_context = true;
        }
        if !self.needs_context {
            walk_call_expression(self, it);
        }
    }

    fn visit_member_expression(&mut self, it: &MemberExpression<'a>) {
        let obj = match it {
            MemberExpression::StaticMemberExpression(m) => &m.object,
            MemberExpression::ComputedMemberExpression(m) => &m.object,
            _ => {
                walk_member_expression(self, it);
                return;
            }
        };
        if !self.is_safe_expression_root(obj) {
            self.needs_context = true;
        }
        if !self.needs_context {
            walk_member_expression(self, it);
        }
    }
}
