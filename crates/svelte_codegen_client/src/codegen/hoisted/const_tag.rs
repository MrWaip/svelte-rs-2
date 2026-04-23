use oxc_ast::ast::{BindingPattern, Expression, Statement};
use oxc_ast::AstKind;
use oxc_semantic::SymbolId;
use svelte_analyze::{BlockSemantics, ConstTagAsyncKind, ConstTagBlockSemantics};
use svelte_ast::NodeId;
use svelte_ast_builder::{Arg, AssignLeft, ObjProp};
use svelte_component_semantics::walk_bindings;

use super::super::data_structures::{EmitState, FragmentCtx};
use super::super::{Codegen, CodegenError, Result};

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(super) fn emit_hoisted_const_tag(
        &mut self,
        state: &mut EmitState<'a>,
        _ctx: &FragmentCtx<'a>,
        id: NodeId,
    ) -> Result<()> {
        let sem = match self.ctx.query.analysis.block_semantics(id) {
            BlockSemantics::ConstTag(s) => s.clone(),
            _ => return CodegenError::unexpected_block_semantics(id, "ConstTag expected"),
        };

        if matches!(sem.async_kind, ConstTagAsyncKind::Async { .. })
            && self.ctx.state.experimental_async
        {
            return self.emit_const_tag_async(state, id, sem);
        }

        self.emit_const_tag_sync(state, id, sem)
    }

    fn emit_const_tag_sync(
        &mut self,
        state: &mut EmitState<'a>,
        id: NodeId,
        sem: ConstTagBlockSemantics,
    ) -> Result<()> {
        let (bindings, is_destructured) = pattern_facts(self.ctx, sem.decl_node_id);
        let init_expr = self.take_const_tag_init(id)?;

        if is_destructured {
            self.build_sync_destructured(id, &bindings, init_expr, &mut state.init)?;
        } else {
            self.build_sync_simple(&bindings, init_expr, &mut state.init)?;
        }
        Ok(())
    }

    fn build_sync_simple(
        &mut self,
        bindings: &[SymbolId],
        init_expr: Expression<'a>,
        stmts: &mut Vec<Statement<'a>>,
    ) -> Result<()> {
        let Some(&sym) = bindings.first() else {
            return CodegenError::unexpected_child("at least one binding", "empty bindings");
        };
        let name = self.ctx.query.view.symbol_name(sym).to_string();
        let thunk = self.ctx.b.thunk(init_expr);
        let derived = self.ctx.b.call_expr("$.derived", [Arg::Expr(thunk)]);

        let final_expr = if self.ctx.state.dev {
            let name_str = self.ctx.b.alloc_str(&name);
            self.ctx
                .b
                .call_expr("$.tag", [Arg::Expr(derived), Arg::StrRef(name_str)])
        } else {
            derived
        };

        stmts.push(self.ctx.b.const_stmt(&name, final_expr));

        if self.ctx.state.dev {
            let name_str = self.ctx.b.alloc_str(&name);
            stmts.push(self.ctx.b.call_stmt("$.get", [Arg::Ident(name_str)]));
        }
        Ok(())
    }

    fn build_sync_destructured(
        &mut self,
        id: NodeId,
        bindings: &[SymbolId],
        init_expr: Expression<'a>,
        stmts: &mut Vec<Statement<'a>>,
    ) -> Result<()> {
        let Some(tmp_name) = self
            .ctx
            .transform_data
            .const_tag_tmp_names
            .get(&id)
            .cloned()
        else {
            return CodegenError::unexpected_node(id, "destructured const tag missing tmp_name");
        };
        let names: Vec<String> = bindings
            .iter()
            .map(|&s| self.ctx.query.view.symbol_name(s).to_string())
            .collect();
        let tmp_ref: &str = self.ctx.b.alloc_str(&tmp_name);

        let destruct_stmt = self.ctx.b.const_object_destruct_stmt(&names, init_expr);
        let props: Vec<ObjProp<'a>> = names
            .iter()
            .map(|n| ObjProp::Shorthand(self.ctx.b.alloc_str(n)))
            .collect();
        let ret = self.ctx.b.return_stmt(self.ctx.b.object_expr(props));
        let thunk = self
            .ctx
            .b
            .arrow_block_expr(self.ctx.b.no_params(), [destruct_stmt, ret]);
        let derived = self.ctx.b.call_expr("$.derived", [Arg::Expr(thunk)]);

        let final_expr = if self.ctx.state.dev {
            self.ctx
                .b
                .call_expr("$.tag", [Arg::Expr(derived), Arg::StrRef("[@const]")])
        } else {
            derived
        };

        stmts.push(self.ctx.b.const_stmt(tmp_ref, final_expr));
        Ok(())
    }

    fn emit_const_tag_async(
        &mut self,
        _state: &mut EmitState<'a>,
        id: NodeId,
        _sem: ConstTagBlockSemantics,
    ) -> Result<()> {
        CodegenError::not_implemented(id, "async {@const}")
    }

    pub(super) fn emit_const_tags_async_batch(
        &mut self,
        state: &mut EmitState<'a>,
        ids: &[NodeId],
    ) -> Result<()> {
        let promises_name = self.ctx.gen_ident("promises");
        let mut thunks: Vec<Expression<'a>> = Vec::new();

        for &id in ids {
            let sem: ConstTagBlockSemantics = match self.ctx.query.analysis.block_semantics(id) {
                BlockSemantics::ConstTag(s) => s.clone(),
                _ => continue,
            };
            let (bindings, is_destructured) = pattern_facts(self.ctx, sem.decl_node_id);
            let (has_await, blockers) = match &sem.async_kind {
                ConstTagAsyncKind::Async {
                    has_await,
                    blockers,
                } => (*has_await, blockers.to_vec()),
                ConstTagAsyncKind::Sync => (false, Vec::new()),
            };

            let init_expr = self.take_const_tag_init(id)?;

            if is_destructured {
                let Some(tmp_name) = self
                    .ctx
                    .transform_data
                    .const_tag_tmp_names
                    .get(&id)
                    .cloned()
                else {
                    return CodegenError::unexpected_node(
                        id,
                        "destructured const tag missing tmp_name",
                    );
                };
                let names: Vec<String> = bindings
                    .iter()
                    .map(|&s| self.ctx.query.view.symbol_name(s).to_string())
                    .collect();
                state.init.push(self.ctx.b.let_stmt(&tmp_name));
                build_blocker_thunks(self.ctx, &blockers, &mut thunks);

                let destruct_stmt = self.ctx.b.const_object_destruct_stmt(&names, init_expr);
                let props: Vec<ObjProp<'a>> = names
                    .iter()
                    .map(|n| ObjProp::Shorthand(self.ctx.b.alloc_str(n)))
                    .collect();
                let ret = self.ctx.b.return_stmt(self.ctx.b.object_expr(props));
                let block_arrow = self
                    .ctx
                    .b
                    .arrow_block_expr(self.ctx.b.no_params(), [destruct_stmt, ret]);
                let derived = create_derived(self.ctx, block_arrow, has_await);
                let final_derived = if self.ctx.state.dev {
                    self.ctx
                        .b
                        .call_expr("$.tag", [Arg::Expr(derived), Arg::StrRef("[@const]")])
                } else {
                    derived
                };
                let assignment = self
                    .ctx
                    .b
                    .assign_expr(AssignLeft::Ident(tmp_name.clone()), final_derived);
                let body = if has_await {
                    self.ctx.b.async_thunk(assignment)
                } else {
                    self.ctx.b.thunk(assignment)
                };
                thunks.push(body);
                let thunk_idx = thunks.len() - 1;
                for &sym_id in &bindings {
                    self.ctx
                        .const_tag_blockers
                        .insert(sym_id, (promises_name.clone(), thunk_idx));
                }
            } else {
                let Some(&sym) = bindings.first() else {
                    continue;
                };
                let name = self.ctx.query.view.symbol_name(sym).to_string();
                state.init.push(self.ctx.b.let_stmt(&name));
                build_blocker_thunks(self.ctx, &blockers, &mut thunks);

                let derived = create_derived(self.ctx, init_expr, has_await);
                let final_derived = if self.ctx.state.dev {
                    let name_str = self.ctx.b.alloc_str(&name);
                    self.ctx
                        .b
                        .call_expr("$.tag", [Arg::Expr(derived), Arg::StrRef(name_str)])
                } else {
                    derived
                };
                let assignment = self
                    .ctx
                    .b
                    .assign_expr(AssignLeft::Ident(name.clone()), final_derived);
                let body = if self.ctx.state.dev {
                    let name_str = self.ctx.b.alloc_str(&name);
                    let get_call = self.ctx.b.call_stmt("$.get", [Arg::Ident(name_str)]);
                    let assign_stmt = self.ctx.b.expr_stmt(assignment);
                    let body_stmts = vec![assign_stmt, get_call];
                    if has_await {
                        self.ctx.b.async_thunk_block(body_stmts)
                    } else {
                        self.ctx.b.thunk_block(body_stmts)
                    }
                } else if has_await {
                    self.ctx.b.async_thunk(assignment)
                } else {
                    self.ctx.b.thunk(assignment)
                };
                thunks.push(body);
                let thunk_idx = thunks.len() - 1;
                self.ctx
                    .const_tag_blockers
                    .insert(sym, (promises_name.clone(), thunk_idx));
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

    fn take_const_tag_init(&mut self, id: NodeId) -> Result<Expression<'a>> {
        let Some(handle) = self.ctx.query.view.const_tag_stmt_handle(id) else {
            return CodegenError::missing_expression(id);
        };
        let Some(stmt) = self.ctx.state.parsed.take_stmt(handle) else {
            return CodegenError::missing_expression(id);
        };
        let Statement::VariableDeclaration(mut decl) = stmt else {
            return CodegenError::unexpected_node(id, "const tag stmt must be VariableDeclaration");
        };
        if decl.declarations.is_empty() {
            return CodegenError::unexpected_node(id, "const tag stmt has no declarators");
        }
        let Some(init) = decl.declarations.remove(0).init.take() else {
            return CodegenError::unexpected_node(id, "const tag declarator must have init");
        };
        Ok(init)
    }
}

fn build_blocker_thunks<'a>(
    ctx: &mut crate::context::Ctx<'a>,
    blockers: &[u32],
    thunks: &mut Vec<Expression<'a>>,
) {
    if blockers.is_empty() {
        return;
    }
    if blockers.len() == 1 {
        let member = ctx.b.computed_member_expr(
            ctx.b.rid_expr("$$promises"),
            ctx.b.num_expr(blockers[0] as f64),
        );
        let promise_access = ctx.b.static_member_expr(member, "promise");
        thunks.push(ctx.b.thunk(promise_access));
    } else {
        let arr_elements: Vec<Expression<'a>> = blockers
            .iter()
            .map(|&idx| {
                ctx.b
                    .computed_member_expr(ctx.b.rid_expr("$$promises"), ctx.b.num_expr(idx as f64))
            })
            .collect();
        let arr = ctx.b.array_expr(arr_elements);
        let wait_call = ctx.b.call_expr("$.wait", [Arg::Expr(arr)]);
        thunks.push(ctx.b.thunk(wait_call));
    }
}

fn create_derived<'a>(
    ctx: &mut crate::context::Ctx<'a>,
    init: Expression<'a>,
    has_await: bool,
) -> Expression<'a> {
    if has_await {
        let thunk = ctx.b.async_thunk(init);
        ctx.b.call_expr("$.async_derived", [Arg::Expr(thunk)])
    } else {
        let thunk = ctx.b.thunk(init);
        ctx.b.call_expr("$.derived", [Arg::Expr(thunk)])
    }
}

fn pattern_facts(
    ctx: &crate::context::Ctx<'_>,
    decl_node_id: svelte_component_semantics::OxcNodeId,
) -> (Vec<SymbolId>, bool) {
    let Some(AstKind::VariableDeclaration(decl)) = ctx.query.view.scoping().js_kind(decl_node_id)
    else {
        return (Vec::new(), false);
    };
    let Some(declarator) = decl.declarations.first() else {
        return (Vec::new(), false);
    };
    let is_destructured = !matches!(declarator.id, BindingPattern::BindingIdentifier(_));
    let mut bindings: Vec<SymbolId> = Vec::new();
    walk_bindings(&declarator.id, |v| bindings.push(v.symbol));
    (bindings, is_destructured)
}
