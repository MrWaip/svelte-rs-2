use oxc_ast::ast::{Expression, MemberExpression};
use oxc_ast_visit::Visit;
use oxc_ast_visit::walk::{walk_call_expression, walk_member_expression};

use crate::types::data::{AnalysisData, BindingSemantics, PropBindingKind, PropBindingSemantics};
use crate::types::script::{RuneKind, ScriptInfo};

pub(crate) struct NeedsContextVisitor<'a> {
    scoping: &'a crate::scope::ComponentScoping<'a>,
    unsafe_prop_syms: rustc_hash::FxHashSet<crate::scope::SymbolId>,
    needs_context: bool,
}

impl<'a> NeedsContextVisitor<'a> {
    pub(crate) fn check(
        program: &oxc_ast::ast::Program<'a>,
        scoping: &'a crate::scope::ComponentScoping,
        script_info: &ScriptInfo,
    ) -> bool {
        let root = scoping.root_scope_id();
        let mut unsafe_prop_syms = rustc_hash::FxHashSet::default();

        for d in &script_info.declarations {
            if d.is_rune == Some(RuneKind::Props)
                && let Some(sym) = scoping.find_binding(root, d.name.as_str())
            {
                unsafe_prop_syms.insert(sym);
            }
        }
        if let Some(ref decl) = script_info.props_declaration {
            for p in &decl.props {
                if p.is_rest
                    && let Some(sym) = scoping.find_binding(root, p.local_name.as_str())
                {
                    unsafe_prop_syms.insert(sym);
                }
            }
        }

        let mut visitor = Self {
            scoping,
            unsafe_prop_syms,
            needs_context: false,
        };
        visitor.visit_program(program);
        visitor.needs_context
    }

    fn resolve_ref(
        &self,
        ident: &oxc_ast::ast::IdentifierReference<'_>,
    ) -> Option<crate::scope::SymbolId> {
        let ref_id = ident.reference_id.get()?;
        self.scoping.get_reference(ref_id).symbol_id()
    }

    fn is_safe_sym(&self, ident: &oxc_ast::ast::IdentifierReference<'_>) -> bool {
        let Some(sym_id) = self.resolve_ref(ident) else {
            return true;
        };
        !self.unsafe_prop_syms.contains(&sym_id) && !self.scoping.is_import(sym_id)
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
    fn visit_new_expression(&mut self, _it: &oxc_ast::ast::NewExpression<'a>) {
        self.needs_context = true;
    }

    fn visit_call_expression(&mut self, it: &oxc_ast::ast::CallExpression<'a>) {
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

pub(crate) fn classify_expression_needs_context(data: &mut AnalysisData) {
    let scoping = &data.scoping;
    let reactivity = &data.reactivity;
    for info in data
        .expressions
        .values_mut()
        .chain(data.attr_expressions.values_mut())
    {
        let needs_context = info.has_context_sensitive_shape()
            && info.ref_symbols().iter().any(|&sym| {
                if scoping.is_import(sym) {
                    return true;
                }
                matches!(
                    reactivity.binding_semantics(sym),
                    BindingSemantics::Prop(PropBindingSemantics {
                        kind: PropBindingKind::Source { .. } | PropBindingKind::NonSource,
                        ..
                    })
                )
            });
        info.set_needs_context(needs_context);
    }
}
