use oxc_span::{GetSpan, GetSpanMut};

use super::super::ScriptTransformer;
use super::inspect::{is_inspect_call, is_inspect_trace_call};

impl<'a> ScriptTransformer<'_, 'a> {
    pub(super) fn process_statement_block(
        &mut self,
        stmts: &mut oxc_allocator::Vec<'a, oxc_ast::ast::Statement<'a>>,
    ) {
        self.strip_ts_specifiers_and_statements(stmts);
        self.strip_export_keywords(stmts);
        self.strip_prod_inspect(stmts);
        self.strip_props_id_declarations(stmts);
        self.process_sync_derived_destructuring(stmts);
        self.process_async_derived_destructuring(stmts);
        self.expand_state_destructuring(stmts);
        self.replace_props_declaration(stmts);
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
                if let oxc_ast::ast::Statement::ExportNamedDeclaration(export) = stmt {
                    if let Some(decl) = export.unbox().declaration {
                        stmts.insert(i, oxc_ast::ast::Statement::from(decl));
                        i += 1;
                    }
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
            if let oxc_ast::ast::Statement::VariableDeclaration(decl) = stmt {
                if Self::is_props_id_declaration(decl) {
                    return false;
                }
            }
            true
        });
    }

    fn replace_props_declaration(
        &mut self,
        stmts: &mut oxc_allocator::Vec<'a, oxc_ast::ast::Statement<'a>>,
    ) {
        if self.props_gen.is_none() {
            return;
        }

        let mut idx = None;
        let mut remove = Vec::new();
        let mut original_span = None;

        for (j, stmt) in stmts.iter().enumerate() {
            let oxc_ast::ast::Statement::VariableDeclaration(decl) = stmt else {
                continue;
            };
            if !Self::is_props_declaration(decl) && !self.is_explicit_props_declaration(stmt) {
                continue;
            }
            if idx.is_none() {
                idx = Some(j);
                original_span = Some(stmt.span());
            }
            remove.push(j);
        }

        let Some(j) = idx else { return };

        let mut replacement = self.gen_props_statements();
        if let Some(first) = replacement.first_mut() {
            *first.span_mut() = original_span.unwrap_or_else(|| stmts[j].span());
        }
        for remove_idx in remove.into_iter().rev() {
            stmts.remove(remove_idx);
        }
        for (k, stmt) in replacement.into_iter().enumerate() {
            stmts.insert(j + k, stmt);
        }
    }

    fn is_explicit_props_declaration(&self, stmt: &oxc_ast::ast::Statement<'a>) -> bool {
        let Some(props_gen) = &self.props_gen else {
            return false;
        };
        let span = stmt.span();
        let span_start = span.start + self.script_content_start;
        let span_end = span.end + self.script_content_start;
        props_gen
            .declaration_spans
            .iter()
            .any(|decl_span| decl_span.start == span_start && decl_span.end == span_end)
    }
}
