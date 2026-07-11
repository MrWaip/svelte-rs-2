use std::mem;

use oxc_allocator::Vec as OxcVec;
use oxc_ast::ast::{BindingPattern, Statement, VariableDeclaration};
use oxc_span::SPAN;
use svelte_analyze::StateKind;

use super::inspect::{is_inspect_call, is_inspect_trace_call};
use super::model::ComponentTransformer;

impl<'a> ComponentTransformer<'_, 'a> {
    pub(crate) fn process_statement_block(&mut self, stmts: &mut OxcVec<'a, Statement<'a>>) {
        if !self.runes {
            self.rewrite_split_export_props_legacy(stmts);
        }
        self.process_statements(stmts);
    }

    pub(crate) fn strip_inspect_trace_statements(&mut self, stmts: &mut OxcVec<'a, Statement<'a>>) {
        if self.dev {
            return;
        }
        stmts.retain(|stmt| {
            let Statement::ExpressionStatement(es) = stmt else {
                return true;
            };
            !is_inspect_trace_call(&es.expression)
        });
    }

    fn process_statements(&mut self, stmts: &mut OxcVec<'a, Statement<'a>>) {
        let mut out = self.b.ast.vec_with_capacity(stmts.len());
        for stmt in stmts.drain(..) {
            let stmt = if self.strip_exports {
                match stmt {
                    Statement::ExportNamedDeclaration(export) => match export.unbox().declaration {
                        Some(decl) => Statement::from(decl),
                        None => continue,
                    },
                    other => other,
                }
            } else {
                stmt
            };

            match &stmt {
                Statement::ExpressionStatement(es) if !self.dev => {
                    if is_inspect_trace_call(&es.expression) {
                        continue;
                    }
                    if is_inspect_call(&es.expression) {
                        out.push(Statement::EmptyStatement(
                            self.b.ast.alloc_empty_statement(SPAN),
                        ));
                        out.push(Statement::EmptyStatement(
                            self.b.ast.alloc_empty_statement(SPAN),
                        ));
                        continue;
                    }
                }
                Statement::VariableDeclaration(decl) => {
                    if self.is_props_id_declaration(decl) || self.is_eager_state_declaration(decl) {
                        continue;
                    }
                }
                _ => {}
            }

            out.push(stmt);
        }
        mem::swap(stmts, &mut out);
    }

    fn is_eager_state_declaration(&self, decl: &VariableDeclaration<'a>) -> bool {
        let Some(analysis) = self.analysis.as_ref() else {
            return false;
        };
        decl.declarations.iter().all(|d| {
            let BindingPattern::BindingIdentifier(ident) = &d.id else {
                return false;
            };
            let Some(sym) = ident.symbol_id.get() else {
                return false;
            };
            analysis
                .binding_semantics(sym)
                .state()
                .is_some_and(|state| state.kind == StateKind::StateEager)
        })
    }

    pub(crate) fn split_top_level_multi_declarators(
        &mut self,
        stmts: &mut OxcVec<'a, Statement<'a>>,
    ) {
        let mut i = 0;
        while i < stmts.len() {
            let needs_split = matches!(
                &stmts[i],
                Statement::VariableDeclaration(decl) if decl.declarations.len() > 1
            );
            if !needs_split {
                i += 1;
                continue;
            }
            let Statement::VariableDeclaration(boxed) = stmts.remove(i) else {
                unreachable!()
            };
            let owned = boxed.unbox();
            let kind = owned.kind;
            let declare = owned.declare;
            let span = owned.span;
            let count = owned.declarations.len();
            for (k, d) in owned.declarations.into_iter().enumerate() {
                let mut decls = self.b.ast.vec_with_capacity(1);
                decls.push(d);
                let new_decl = self.b.ast.variable_declaration(span, kind, decls, declare);
                stmts.insert(
                    i + k,
                    Statement::VariableDeclaration(self.b.alloc(new_decl)),
                );
            }
            i += count;
        }
    }
}
