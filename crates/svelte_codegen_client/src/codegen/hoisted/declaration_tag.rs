use oxc_ast::ast::{BindingPattern, Expression, Statement};
use oxc_semantic::SymbolId;
use svelte_ast::NodeId;
use svelte_ast_builder::{Arg, AssignLeft};
use svelte_component_semantics::walk_bindings;

use super::super::data_structures::EmitState;
use super::super::{Codegen, CodegenError, Result};
use super::const_tag::build_blocker_thunks;

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(in crate::codegen) fn emit_declaration_tags(
        &mut self,
        state: &mut EmitState<'a>,
        ids: &[NodeId],
    ) -> Result<()> {
        let first_async = if self.ctx.state.experimental_async {
            ids.iter().position(|&id| self.declaration_tag_is_async(id))
        } else {
            None
        };
        match first_async {
            None => {
                for &id in ids {
                    self.emit_hoisted_declaration_tag(state, id)?;
                }
            }
            Some(idx) => {
                for &id in &ids[..idx] {
                    self.emit_hoisted_declaration_tag(state, id)?;
                }
                self.emit_declaration_tags_async_batch(state, &ids[idx..])?;
            }
        }
        Ok(())
    }

    fn declaration_tag_is_async(&self, id: NodeId) -> bool {
        self.ctx
            .expression_data(id)
            .is_some_and(|d| d.volatility.is_asynchronous() || !d.blockers.is_empty())
    }

    pub(in crate::codegen) fn emit_hoisted_declaration_tag(
        &mut self,
        state: &mut EmitState<'a>,
        id: NodeId,
    ) -> Result<()> {
        let svelte_ast::Node::DeclarationTag(tag) = self.ctx.query.component.store.get(id) else {
            return CodegenError::unexpected_child("declaration tag", "statements");
        };
        let Some(stmt) = self.ctx.state.parsed.take_stmt(tag.declaration.id()) else {
            return CodegenError::unexpected_child("declaration tag statement", "statements");
        };
        state.init.push(stmt);
        Ok(())
    }

    fn emit_declaration_tags_async_batch(
        &mut self,
        state: &mut EmitState<'a>,
        ids: &[NodeId],
    ) -> Result<()> {
        let promises_name = self.ctx.gen_ident("promises");
        let mut thunks: Vec<Expression<'a>> = Vec::new();

        for &id in ids {
            let is_awaited = self
                .ctx
                .expression_data(id)
                .is_some_and(|d| d.volatility.is_asynchronous());
            let blockers = self
                .ctx
                .expression_data(id)
                .map(|d| d.blockers.to_vec())
                .unwrap_or_default();

            let decl_stmt_id = {
                let svelte_ast::Node::DeclarationTag(tag) = self.ctx.query.component.store.get(id)
                else {
                    return CodegenError::unexpected_child("declaration tag", "statements");
                };
                tag.declaration.id()
            };
            let Some(stmt) = self.ctx.state.parsed.take_stmt(decl_stmt_id) else {
                return CodegenError::unexpected_child("declaration tag statement", "statements");
            };
            let Statement::VariableDeclaration(mut decl) = stmt else {
                state.init.push(stmt);
                continue;
            };

            build_blocker_thunks(self.ctx, &blockers, &mut thunks);

            for mut declarator in decl.declarations.drain(..) {
                let mut symbols: Vec<SymbolId> = Vec::new();
                let mut names: Vec<String> = Vec::new();
                walk_bindings(&declarator.id, |v| {
                    symbols.push(v.symbol);
                    names.push(self.ctx.query.view.symbol_name(v.symbol).to_string());
                });

                let single_ident_init = match &declarator.id {
                    BindingPattern::BindingIdentifier(_) => declarator.init.take(),
                    _ => None,
                };
                let thunk = if let Some(init) = single_ident_init {
                    let target = self.ctx.b.alloc_str(&names[0]);
                    state.init.push(self.ctx.b.let_stmt(target));
                    let assignment = self
                        .ctx
                        .b
                        .assign_expr(AssignLeft::Ident(target.to_string()), init);
                    if is_awaited {
                        self.ctx.b.async_arrow_expr_body(assignment)
                    } else {
                        self.ctx.b.thunk(assignment)
                    }
                } else {
                    for name in &names {
                        state
                            .init
                            .push(self.ctx.b.let_stmt(self.ctx.b.alloc_str(name)));
                    }
                    let var_stmt = self.ctx.b.var_init_stmt(declarator);
                    if is_awaited {
                        self.ctx.b.async_thunk_block(vec![var_stmt])
                    } else {
                        self.ctx.b.thunk_block(vec![var_stmt])
                    }
                };
                thunks.push(thunk);
                let thunk_idx = thunks.len() - 1;
                for sym in symbols {
                    self.ctx
                        .const_tag_blockers
                        .insert(sym, (promises_name.clone(), thunk_idx));
                }
            }
        }

        if !thunks.is_empty() {
            let thunks_array = self.ctx.b.array_expr(thunks);
            let run_call = self.ctx.b.call_expr("$.run", [Arg::Expr(thunks_array)]);
            state
                .init
                .push(self.ctx.b.var_stmt(&promises_name, run_call));
        }
        Ok(())
    }
}
