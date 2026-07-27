use std::mem;

use oxc_allocator::CloneIn;
use oxc_ast::ast::{AssignmentOperator, BindingPattern, Expression, Statement};
use oxc_span::SPAN;
use svelte_analyze::{AsyncEntryLocation, AwaitSemantics};
use svelte_ast_builder::Arg;
use svelte_component_semantics::OxcNodeId;
use svelte_emit_builders::async_entry::{
    EntryStatement, entry_thunk as async_entry_thunk, push_entry_statement,
    statement_entry_location,
};

use crate::derived::expand_derived_destructure_statements;
use crate::model::ServerTransform;

impl<'a> ServerTransform<'_, 'a> {
    pub(crate) fn rewrite_await(&self, it: &mut Expression<'a>) {
        let Expression::AwaitExpression(await_expr) = it else {
            return;
        };
        let id = await_expr.node_id();
        if !self.await_needs_save(id) {
            return;
        }
        let ast = self.b.ast;
        let arg = mem::replace(
            &mut await_expr.argument,
            ast.expression_identifier(SPAN, ""),
        );
        let saved = self.b.call_expr("$.save", [Arg::Expr(arg)]);
        *it = self.b.call_expr_callee(self.b.await_expr(saved), []);
    }

    fn await_needs_save(&self, id: OxcNodeId) -> bool {
        match self.analysis.await_semantics.query(id) {
            AwaitSemantics::NonTerminal | AwaitSemantics::TerminalInConstruct => true,
            AwaitSemantics::TerminalInFragmentInterpolation
            | AwaitSemantics::TerminalInReactiveDeclaration
            | AwaitSemantics::Detached => false,
        }
    }
}

impl<'a> ServerTransform<'_, 'a> {
    pub(crate) fn split_async_instance_body(
        &mut self,
        body: Vec<Statement<'a>>,
    ) -> Vec<Statement<'a>> {
        let b = self.b;
        let analysis = self.analysis;
        let blocker_data = analysis.blocker_data();
        if blocker_data.first_await_index().is_none() {
            return body;
        }

        let ident_gen = &mut *self.ident_gen;
        let entries = blocker_data.entries();
        let mut result = Vec::new();
        let mut buckets: Vec<Vec<EntryStatement<'a>>> =
            entries.iter().map(|_| Vec::new()).collect();

        for (i, stmt) in body.into_iter().enumerate() {
            let stmt = match stmt {
                Statement::ExportNamedDeclaration(export) => match export.unbox().declaration {
                    Some(decl) => Statement::from(decl),
                    None => continue,
                },
                other => other,
            };

            match stmt {
                Statement::VariableDeclaration(var_decl) => {
                    let declarators: Vec<_> = var_decl.unbox().declarations.into_iter().collect();
                    let locations: Vec<Option<AsyncEntryLocation>> = declarators
                        .iter()
                        .map(|declarator| {
                            blocker_data
                                .entry_location(declarator.node_id())
                                .or_else(|| {
                                    declarator_symbol_location(blocker_data, &declarator.id)
                                })
                        })
                        .collect();
                    let shared = locations.iter().flatten().next().copied();
                    for (declarator, own) in declarators.into_iter().zip(locations) {
                        let location = own.or(shared);
                        if own.is_none()
                            && let Some(location) = location
                        {
                            buckets[location.entry]
                                .push(EntryStatement::Plain(b.var_init_stmt(declarator)));
                            continue;
                        }
                        let declarator = match expand_derived_destructure_statements(
                            b, analysis, ident_gen, declarator,
                        ) {
                            Ok(stmts) => {
                                match location {
                                    Some(location) => {
                                        for stmt in stmts {
                                            buckets[location.entry]
                                                .push(EntryStatement::Plain(stmt));
                                        }
                                    }
                                    None => result.extend(stmts),
                                }
                                continue;
                            }
                            Err(declarator) => declarator,
                        };
                        let Some(location) = location else {
                            result.push(b.declarator_stmt(declarator));
                            continue;
                        };

                        let pattern = declarator.id.clone_in(b.ast.allocator);
                        if let Some(target) = b.binding_pattern_to_assignment_target(pattern) {
                            let init = declarator.init.unwrap_or_else(|| b.void_zero_expr());
                            let assign = b.ast.expression_assignment(
                                SPAN,
                                AssignmentOperator::Assign,
                                target,
                                init,
                            );
                            buckets[location.entry].push(EntryStatement::Value(assign));
                        } else {
                            buckets[location.entry]
                                .push(EntryStatement::Plain(b.var_init_stmt(declarator)));
                        }
                    }
                }
                Statement::FunctionDeclaration(_) => result.push(stmt),
                other => {
                    let Some(location) = statement_entry_location(blocker_data, &other, i) else {
                        result.push(other);
                        continue;
                    };
                    push_entry_statement(b, &mut buckets[location.entry], other, location.kind);
                }
            }
        }

        let hoisted_names: Vec<&str> = blocker_data
            .hoisted_names()
            .iter()
            .map(|name| b.alloc_str(name))
            .collect();
        if !hoisted_names.is_empty() {
            result.push(b.var_multi_stmt(&hoisted_names));
        }

        if !entries.is_empty() {
            let mut thunks: Vec<Expression<'a>> = Vec::with_capacity(entries.len());
            for (entry, statements) in entries.iter().zip(buckets) {
                thunks.push(async_entry_thunk(b, entry, statements));
            }
            let run_call = b.call_expr("$$renderer.run", [Arg::Expr(b.array_expr(thunks))]);
            result.push(b.var_stmt("$$promises", run_call));
        }

        result
    }
}

fn declarator_symbol_location(
    blocker_data: &svelte_analyze::BlockerData,
    pattern: &BindingPattern<'_>,
) -> Option<svelte_analyze::AsyncEntryLocation> {
    let mut found = None;
    svelte_component_semantics::walk_bindings(pattern, |v| {
        if found.is_none() {
            found = blocker_data.entry_location_of_symbol(v.symbol);
        }
    });
    found
}
