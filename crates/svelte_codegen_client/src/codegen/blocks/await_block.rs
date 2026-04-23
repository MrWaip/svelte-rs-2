use oxc_ast::ast::{Expression, Statement};
use svelte_analyze::scope::SymbolId;
use svelte_analyze::{
    AwaitBinding, AwaitBlockSemantics, AwaitBranch, AwaitDestructureKind, AwaitWrapper,
};
use svelte_ast::NodeId;
use svelte_ast_builder::Arg;

use super::super::data_structures::EmitState;
use super::super::data_structures::{FragmentAnchor, FragmentCtx};
use super::super::{Codegen, Result};

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(in super::super) fn emit_await_block(
        &mut self,
        state: &mut EmitState<'a>,
        ctx: &FragmentCtx<'a>,
        id: NodeId,
        sem: AwaitBlockSemantics,
    ) -> Result<()> {
        let anchor_node = self.comment_anchor_node_name(state, ctx)?;
        let span_start = self.ctx.query.await_block(id).span.start;

        let expression = self.build_await_expression(id, sem.expression_has_await)?;
        let then_fn = self.build_await_then_fn(ctx, id, &sem.then, &sem.catch)?;
        let catch_fn = self.build_await_catch_fn(ctx, id, &sem.catch)?;
        let pending_fn = self.build_await_pending_fn(ctx, id, &sem.pending)?;

        let has_then = matches!(sem.then, AwaitBranch::Present { .. });
        let has_catch = matches!(sem.catch, AwaitBranch::Present { .. });

        let mut args: Vec<Arg<'a, '_>> = vec![
            Arg::Ident(&anchor_node),
            Arg::Expr(expression),
            Arg::Expr(pending_fn),
        ];
        if has_then || has_catch {
            args.push(Arg::Expr(then_fn));
        }
        if has_catch {
            args.push(Arg::Expr(catch_fn));
        }

        let await_call = self.ctx.b.call_expr("$.await", args);
        let await_stmt = self.add_svelte_meta(await_call, span_start, "await");

        match sem.wrapper {
            AwaitWrapper::None => {
                state.init.push(await_stmt);
            }
            AwaitWrapper::AsyncWrap { blockers } => {
                let blockers_expr = if blockers.is_empty() {
                    self.ctx.b.empty_array_expr()
                } else {
                    self.ctx.b.promises_array(&blockers)
                };
                let empty_array = self.ctx.b.empty_array_expr();
                let inner_arrow = self
                    .ctx
                    .b
                    .arrow_block_expr(self.ctx.b.params([&anchor_node]), vec![await_stmt]);
                state.init.push(self.ctx.b.call_stmt(
                    "$.async",
                    [
                        Arg::Ident(&anchor_node),
                        Arg::Expr(blockers_expr),
                        Arg::Expr(empty_array),
                        Arg::Expr(inner_arrow),
                    ],
                ));
            }
        }
        Ok(())
    }

    fn build_await_expression(
        &mut self,
        block_id: NodeId,
        has_await: bool,
    ) -> Result<Expression<'a>> {
        let expr = self.take_node_expr(block_id)?;
        if has_await {
            Ok(self.ctx.b.async_thunk(expr))
        } else {
            Ok(self.ctx.b.thunk(expr))
        }
    }

    fn build_await_pending_fn(
        &mut self,
        parent_ctx: &FragmentCtx<'a>,
        block_id: NodeId,
        pending: &AwaitBranch,
    ) -> Result<Expression<'a>> {
        if !matches!(pending, AwaitBranch::Present { .. }) {
            return Ok(self.ctx.b.null_expr());
        }
        let pending_fragment = match self.ctx.query.component.store.get(block_id) {
            svelte_ast::Node::AwaitBlock(block) => match &block.pending {
                Some(p) => p,
                None => return Ok(self.ctx.b.null_expr()),
            },
            _ => return Ok(self.ctx.b.null_expr()),
        };
        let inner_ctx = parent_ctx.child_of_block(
            self.ctx,
            pending_fragment,
            FragmentAnchor::CallbackParam {
                name: "$$anchor".to_string(),
                append_inside: false,
            },
        );
        let mut inner_state = EmitState::new();
        self.emit_fragment(&mut inner_state, &inner_ctx, pending_fragment)?;
        let body = self.pack_callback_body(inner_state, "$$anchor")?;
        Ok(self
            .ctx
            .b
            .arrow_block_expr(self.ctx.b.params(["$$anchor"]), body))
    }

    fn build_await_then_fn(
        &mut self,
        parent_ctx: &FragmentCtx<'a>,
        block_id: NodeId,
        then_branch: &AwaitBranch,
        catch_branch: &AwaitBranch,
    ) -> Result<Expression<'a>> {
        let has_then = matches!(then_branch, AwaitBranch::Present { .. });
        let has_catch = matches!(catch_branch, AwaitBranch::Present { .. });
        if has_then {
            let then_fragment = match self.ctx.query.component.store.get(block_id) {
                svelte_ast::Node::AwaitBlock(block) => match &block.then {
                    Some(t) => t,
                    None => return Ok(self.ctx.b.null_expr()),
                },
                _ => return Ok(self.ctx.b.null_expr()),
            };
            self.build_await_branch_callback(parent_ctx, then_fragment, binding_of(then_branch))
        } else if has_catch {
            Ok(self.ctx.b.void_zero_expr())
        } else {
            Ok(self.ctx.b.null_expr())
        }
    }

    fn build_await_catch_fn(
        &mut self,
        parent_ctx: &FragmentCtx<'a>,
        block_id: NodeId,
        catch_branch: &AwaitBranch,
    ) -> Result<Expression<'a>> {
        if matches!(catch_branch, AwaitBranch::Present { .. }) {
            let catch_fragment = match self.ctx.query.component.store.get(block_id) {
                svelte_ast::Node::AwaitBlock(block) => match &block.catch {
                    Some(c) => c,
                    None => return Ok(self.ctx.b.null_expr()),
                },
                _ => return Ok(self.ctx.b.null_expr()),
            };
            self.build_await_branch_callback(parent_ctx, catch_fragment, binding_of(catch_branch))
        } else {
            Ok(self.ctx.b.null_expr())
        }
    }

    fn build_await_branch_callback(
        &mut self,
        parent_ctx: &FragmentCtx<'a>,
        fragment: &'a svelte_ast::Fragment,
        binding: &AwaitBinding,
    ) -> Result<Expression<'a>> {
        let inner_ctx = parent_ctx.child_of_block(
            self.ctx,
            fragment,
            FragmentAnchor::CallbackParam {
                name: "$$anchor".to_string(),
                append_inside: false,
            },
        );
        let mut inner_state = EmitState::new();
        self.emit_fragment(&mut inner_state, &inner_ctx, fragment)?;
        let frag_body = self.pack_callback_body(inner_state, "$$anchor")?;

        match binding {
            AwaitBinding::None => Ok(self
                .ctx
                .b
                .arrow_block_expr(self.ctx.b.params(["$$anchor"]), frag_body)),
            AwaitBinding::Identifier(sym) => {
                let name = self.ctx.query.view.symbol_name(*sym).to_string();
                Ok(self
                    .ctx
                    .b
                    .arrow_block_expr(self.ctx.b.params(["$$anchor", &name]), frag_body))
            }
            AwaitBinding::Pattern { kind, leaves, .. } => {
                self.build_await_destructured_callback(*kind, leaves, frag_body)
            }
        }
    }

    fn build_await_destructured_callback(
        &mut self,
        kind: AwaitDestructureKind,
        leaves: &[SymbolId],
        frag_body: Vec<Statement<'a>>,
    ) -> Result<Expression<'a>> {
        let names: Vec<String> = leaves
            .iter()
            .map(|&sym| self.ctx.query.view.symbol_name(sym).to_string())
            .collect();

        let mut decls: Vec<Statement<'a>> = Vec::new();

        let get_source = self.ctx.b.call_expr("$.get", [Arg::Ident("$$source")]);
        let destruct_stmt = match kind {
            AwaitDestructureKind::Array => self.ctx.b.var_array_destruct_stmt(&names, get_source),
            AwaitDestructureKind::Object => self.ctx.b.var_object_destruct_stmt(&names, get_source),
        };
        let return_obj = self.ctx.b.shorthand_object_expr(&names);
        let return_stmt = self.ctx.b.return_stmt(return_obj);
        let derived_fn = self.ctx.b.thunk_block(vec![destruct_stmt, return_stmt]);
        let derived_call = self.ctx.b.call_expr("$.derived", [Arg::Expr(derived_fn)]);
        decls.push(self.ctx.b.var_stmt("$$value", derived_call));

        for name in &names {
            let get_value = self.ctx.b.call_expr("$.get", [Arg::Ident("$$value")]);
            let member = self.ctx.b.static_member_expr(get_value, name);
            let getter_fn = self
                .ctx
                .b
                .arrow_expr(self.ctx.b.no_params(), [self.ctx.b.expr_stmt(member)]);
            let per_field_derived = self.ctx.b.call_expr("$.derived", [Arg::Expr(getter_fn)]);
            decls.push(self.ctx.b.var_stmt(name, per_field_derived));
        }

        decls.extend(frag_body);
        Ok(self
            .ctx
            .b
            .arrow_block_expr(self.ctx.b.params(["$$anchor", "$$source"]), decls))
    }
}

fn binding_of(branch: &AwaitBranch) -> &AwaitBinding {
    match branch {
        AwaitBranch::Present { binding } => binding,
        AwaitBranch::Absent => &AwaitBinding::None,
    }
}
