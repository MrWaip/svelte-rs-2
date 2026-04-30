use oxc_ast::ast::{BindingPattern, Statement};
use oxc_span::{GetSpan, GetSpanMut};
use svelte_analyze::{BindingSemantics, StateKind};

use super::inspect::{is_inspect_call, is_inspect_trace_call};
use super::model::ComponentTransformer;

impl<'a> ComponentTransformer<'_, 'a> {
    pub(crate) fn process_statement_block(
        &mut self,
        stmts: &mut oxc_allocator::Vec<'a, oxc_ast::ast::Statement<'a>>,
    ) {
        self.strip_ts_specifiers_and_statements(stmts);

        self.process_legacy_export_props(stmts);
        self.strip_export_keywords(stmts);
        self.strip_prod_inspect(stmts);
        self.strip_props_id_declarations(stmts);
        self.strip_eager_state_declarations(stmts);

        self.replace_props_declaration(stmts);
        self.process_derived_destructuring(stmts);
        self.expand_state_destructuring(stmts);

        self.expand_legacy_state_destructuring(stmts);
    }

    fn strip_export_keywords(
        &self,
        stmts: &mut oxc_allocator::Vec<'a, oxc_ast::ast::Statement<'a>>,
    ) {
        if !self.strip_exports {
            return;
        }
        let mut i = 0;
        while i < stmts.len() {
            if let oxc_ast::ast::Statement::ExportNamedDeclaration(_) = &stmts[i] {
                let stmt = stmts.remove(i);
                if let oxc_ast::ast::Statement::ExportNamedDeclaration(export) = stmt
                    && let Some(decl) = export.unbox().declaration
                {
                    stmts.insert(i, oxc_ast::ast::Statement::from(decl));
                    i += 1;
                }
            } else {
                i += 1;
            }
        }
    }

    fn strip_prod_inspect(&self, stmts: &mut oxc_allocator::Vec<'a, oxc_ast::ast::Statement<'a>>) {
        if self.dev {
            return;
        }
        let mut i = 0;
        while i < stmts.len() {
            if let oxc_ast::ast::Statement::ExpressionStatement(es) = &stmts[i] {
                if is_inspect_trace_call(&es.expression) {
                    stmts.remove(i);
                    continue;
                }
                if is_inspect_call(&es.expression) {
                    stmts[i] = oxc_ast::ast::Statement::EmptyStatement(
                        self.b.ast.alloc_empty_statement(oxc_span::SPAN),
                    );
                    stmts.insert(
                        i + 1,
                        oxc_ast::ast::Statement::EmptyStatement(
                            self.b.ast.alloc_empty_statement(oxc_span::SPAN),
                        ),
                    );
                    i += 2;
                    continue;
                }
            }
            i += 1;
        }
    }

    fn strip_props_id_declarations(
        &self,
        stmts: &mut oxc_allocator::Vec<'a, oxc_ast::ast::Statement<'a>>,
    ) {
        stmts.retain(|stmt| {
            if let oxc_ast::ast::Statement::VariableDeclaration(decl) = stmt
                && Self::is_props_id_declaration(decl)
            {
                return false;
            }
            true
        });
    }

    fn strip_eager_state_declarations(&self, stmts: &mut oxc_allocator::Vec<'a, Statement<'a>>) {
        let Some(analysis) = self.analysis.as_ref() else {
            return;
        };
        stmts.retain(|stmt| {
            let Statement::VariableDeclaration(decl) = stmt else {
                return true;
            };
            !decl.declarations.iter().all(|d| {
                let BindingPattern::BindingIdentifier(ident) = &d.id else {
                    return false;
                };
                let Some(sym) = ident.symbol_id.get() else {
                    return false;
                };
                matches!(
                    analysis.binding_semantics(sym),
                    BindingSemantics::State(state) if state.kind == StateKind::StateEager
                )
            })
        });
    }

    fn replace_props_declaration(
        &mut self,
        stmts: &mut oxc_allocator::Vec<'a, oxc_ast::ast::Statement<'a>>,
    ) {
        for j in 0..stmts.len() {
            let is_candidate = matches!(
                &stmts[j],
                oxc_ast::ast::Statement::VariableDeclaration(decl)
                    if Self::is_props_declaration(decl)
            );
            if !is_candidate {
                continue;
            };

            let stmt_span = stmts[j].span();
            let replacement = {
                let oxc_ast::ast::Statement::VariableDeclaration(decl) = &mut stmts[j] else {
                    unreachable!()
                };
                self.try_gen_props_declaration_semantic(decl)
            };
            if let Some(mut replacement) = replacement {
                if let Some(first) = replacement.first_mut() {
                    *first.span_mut() = stmt_span;
                }
                stmts.remove(j);
                for (k, stmt) in replacement.into_iter().enumerate() {
                    stmts.insert(j + k, stmt);
                }
                return;
            }
        }
    }
}
