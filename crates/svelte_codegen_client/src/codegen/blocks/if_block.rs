use oxc_ast::ast::{Expression, Statement};
use svelte_analyze::{IfAlternate, IfAsyncKind, IfBlockSemantics, IfBranch, IfConditionKind};
use svelte_ast::NodeId;
use svelte_ast_builder::Arg;

use super::super::data_structures::EmitState;
use super::super::data_structures::{FragmentAnchor, FragmentCtx};
use super::super::{Codegen, CodegenError, Result};

struct BranchNames {
    consequent: String,
    derived: Option<String>,
}

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(in super::super) fn emit_if_block(
        &mut self,
        state: &mut EmitState<'a>,
        ctx: &FragmentCtx<'a>,
        id: NodeId,
        sem: IfBlockSemantics,
    ) -> Result<()> {
        let (needs_async, root_has_await, blockers) = match &sem.async_kind {
            IfAsyncKind::Sync => (false, false, Vec::new()),
            IfAsyncKind::Async {
                root_has_await,
                blockers,
            } => (true, *root_has_await, blockers.to_vec()),
        };

        let span_start = self.ctx.query.if_block(id).span.start;
        let is_elseif_root = sem.is_elseif_root;

        let pre_anchor = self.reserve_comment_anchor_pre(state, ctx);

        let mut decls: Vec<Statement<'a>> = Vec::new();
        let branch_names = self.build_if_branches(ctx, &sem, &mut decls)?;
        let alt_name = self.build_if_alternate(ctx, &sem, &mut decls)?;

        let render_body_stmt = self.build_if_chain(&sem, &branch_names, alt_name.as_deref())?;
        let render_fn = self
            .ctx
            .b
            .arrow(self.ctx.b.params(["$$render"]), [render_body_stmt]);

        if needs_async {
            let anchor_node = self.commit_comment_anchor(state, ctx, pre_anchor)?;
            let anchor_expr = self.ctx.b.rid_expr(&anchor_node);
            let if_anchor = self.ctx.b.rid_expr(&anchor_node);
            let mut if_args: Vec<Arg<'a, '_>> = vec![Arg::Expr(if_anchor), Arg::Arrow(render_fn)];
            if is_elseif_root {
                if_args.push(Arg::Bool(true));
            }
            let if_call = self.ctx.b.call_expr("$.if", if_args);
            decls.push(self.add_svelte_meta(if_call, span_start, "if"));

            let async_thunk = if root_has_await {
                let expr = self.take_node_expr(id)?;
                Some(self.ctx.b.async_thunk(expr))
            } else {
                None
            };
            let wrapped = self.emit_async_call_stmt(
                root_has_await,
                &blockers,
                anchor_expr,
                &anchor_node,
                "$$condition",
                async_thunk,
                decls,
            )?;
            state.init.push(wrapped);
            return Ok(());
        }

        let anchor_node = self.commit_comment_anchor(state, ctx, pre_anchor)?;
        let mut if_args: Vec<Arg<'a, '_>> = vec![Arg::Ident(&anchor_node), Arg::Arrow(render_fn)];
        if is_elseif_root {
            if_args.push(Arg::Bool(true));
        }
        let if_call = self.ctx.b.call_expr("$.if", if_args);
        decls.push(self.add_svelte_meta(if_call, span_start, "if"));

        if decls.len() == 1 {
            state.init.extend(decls);
        } else {
            state.init.push(self.ctx.b.block_stmt(decls));
        }
        Ok(())
    }

    fn build_if_branches(
        &mut self,
        parent_ctx: &FragmentCtx<'a>,
        sem: &IfBlockSemantics,
        stmts: &mut Vec<Statement<'a>>,
    ) -> Result<Vec<BranchNames>> {
        let mut out: Vec<BranchNames> = Vec::with_capacity(sem.branches.len());
        for branch in sem.branches.iter() {
            let consequent = match self.ctx.query.component.store.get(branch.block_id) {
                svelte_ast::Node::IfBlock(block) => &block.consequent,
                _ => return CodegenError::unexpected_node(branch.block_id, "IfBlock"),
            };
            let inner_ctx = parent_ctx.child_of_block(
                self.ctx,
                consequent,
                FragmentAnchor::CallbackParam {
                    name: "$$anchor".to_string(),
                    append_inside: false,
                },
            );
            let mut inner_state = EmitState::new();
            self.emit_fragment(&mut inner_state, &inner_ctx, consequent)?;
            let body = self.pack_callback_body(inner_state, "$$anchor")?;
            let consequent_name = self.ctx.state.gen_ident("consequent");
            let arrow = self
                .ctx
                .b
                .arrow_block_expr(self.ctx.b.params(["$$anchor"]), body);
            stmts.push(self.ctx.b.var_stmt(&consequent_name, arrow));

            let derived_name = match branch.condition {
                IfConditionKind::Memo => {
                    let expr = self.take_node_expr(branch.block_id)?;
                    let thunk = self
                        .ctx
                        .b
                        .arrow_expr(self.ctx.b.no_params(), [self.ctx.b.expr_stmt(expr)]);
                    let derived = self.ctx.b.call_expr("$.derived", [Arg::Expr(thunk)]);
                    let name = self.ctx.state.gen_ident("d");
                    stmts.push(self.ctx.b.var_stmt(&name, derived));
                    Some(name)
                }
                IfConditionKind::Raw => None,
                IfConditionKind::AsyncParam => None,
            };
            out.push(BranchNames {
                consequent: consequent_name,
                derived: derived_name,
            });
        }
        Ok(out)
    }

    fn build_if_alternate(
        &mut self,
        parent_ctx: &FragmentCtx<'a>,
        sem: &IfBlockSemantics,
        stmts: &mut Vec<Statement<'a>>,
    ) -> Result<Option<String>> {
        let IfAlternate::Fragment {
            last_branch_block_id,
        } = sem.final_alternate
        else {
            return Ok(None);
        };
        let alternate = match self.ctx.query.component.store.get(last_branch_block_id) {
            svelte_ast::Node::IfBlock(block) => match &block.alternate {
                Some(alt) => alt,
                None => return Ok(None),
            },
            _ => return CodegenError::unexpected_node(last_branch_block_id, "IfBlock"),
        };
        let inner_ctx = parent_ctx.child_of_block(
            self.ctx,
            alternate,
            FragmentAnchor::CallbackParam {
                name: "$$anchor".to_string(),
                append_inside: false,
            },
        );
        let mut inner_state = EmitState::new();
        self.emit_fragment(&mut inner_state, &inner_ctx, alternate)?;
        let body = self.pack_callback_body(inner_state, "$$anchor")?;
        let name = self.ctx.state.gen_ident("alternate");
        let arrow = self
            .ctx
            .b
            .arrow_block_expr(self.ctx.b.params(["$$anchor"]), body);
        stmts.push(self.ctx.b.var_stmt(&name, arrow));
        Ok(Some(name))
    }

    fn build_if_chain(
        &mut self,
        sem: &IfBlockSemantics,
        branch_names: &[BranchNames],
        alt_name: Option<&str>,
    ) -> Result<Statement<'a>> {
        let mut else_clause: Option<Statement<'a>> = alt_name.map(|name| {
            self.ctx
                .b
                .call_stmt("$$render", [Arg::Ident(name), Arg::Num(-1.0)])
        });

        for (i, branch) in sem.branches.iter().enumerate().rev() {
            let test = self.branch_condition_expr(branch, branch_names[i].derived.as_deref())?;
            let render_args: Vec<Arg<'a, '_>> = if i == 0 {
                vec![Arg::Ident(&branch_names[i].consequent)]
            } else {
                vec![Arg::Ident(&branch_names[i].consequent), Arg::Num(i as f64)]
            };
            let then_stmt = self.ctx.b.call_stmt("$$render", render_args);
            let if_stmt = self.ctx.b.if_stmt(test, then_stmt, else_clause.take());
            else_clause = Some(if_stmt);
        }

        else_clause.ok_or(super::super::CodegenError::UnexpectedChild {
            expected: "at least one if branch",
            got: "empty branches",
        })
    }

    fn branch_condition_expr(
        &mut self,
        branch: &IfBranch,
        derived_name: Option<&str>,
    ) -> Result<Expression<'a>> {
        match branch.condition {
            IfConditionKind::AsyncParam => {
                Ok(self.ctx.b.call_expr("$.get", [Arg::Ident("$$condition")]))
            }
            IfConditionKind::Memo => {
                let Some(name) = derived_name else {
                    return CodegenError::unexpected_node(
                        branch.block_id,
                        "Memo branch missing derived ident",
                    );
                };
                Ok(self.ctx.b.call_expr("$.get", [Arg::Ident(name)]))
            }
            IfConditionKind::Raw => self.take_node_expr(branch.block_id),
        }
    }
}
