use oxc_allocator::CloneIn;
use oxc_ast::ast::{BindingPattern, Expression, Statement};
use svelte_analyze::{
    EachAsyncKind, EachBlockSemantics, EachCollectionKind, EachFlags, EachFlavor, EachIndexKind,
    EachItemKind, FragmentKey,
};
use svelte_ast::NodeId;
use svelte_ast_builder::Arg;

use super::super::data_structures::EmitState;
use super::super::data_structures::{FragmentAnchor, FragmentCtx};
use super::super::{Codegen, CodegenError, Result};

fn array_element_name(pattern: &BindingPattern<'_>) -> Option<String> {
    match pattern {
        BindingPattern::BindingIdentifier(id) => Some(id.name.as_str().to_string()),
        BindingPattern::AssignmentPattern(assign) => match &assign.left {
            BindingPattern::BindingIdentifier(id) => Some(id.name.as_str().to_string()),
            _ => None,
        },
        _ => None,
    }
}

const EACH_IS_CONTROLLED: u32 = 4;
const SYNTHETIC_ITEM_NAME: &str = "$$item";

struct EachPlan {
    flags: u32,
    item_param_name: String,
    user_index_name: Option<String>,
    render_index_name: Option<String>,
    collection_id_name: Option<String>,
    key_uses_index: bool,
    has_fallback: bool,
    item_reactive: bool,
    is_prop_source: bool,
    needs_async: bool,
    has_await: bool,
    blockers: Vec<u32>,
}

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(in super::super) fn emit_each_block(
        &mut self,
        state: &mut EmitState<'a>,
        ctx: &FragmentCtx<'a>,
        id: NodeId,
        sem: EachBlockSemantics,
    ) -> Result<()> {
        self.emit_each_block_impl(state, ctx, id, sem, None)
    }

    pub(in super::super) fn emit_each_block_controlled(
        &mut self,
        state: &mut EmitState<'a>,
        ctx: &FragmentCtx<'a>,
        id: NodeId,
        sem: EachBlockSemantics,
        anchor_name: String,
    ) -> Result<()> {
        self.emit_each_block_impl(state, ctx, id, sem, Some(anchor_name))
    }

    fn emit_each_block_impl(
        &mut self,
        state: &mut EmitState<'a>,
        ctx: &FragmentCtx<'a>,
        id: NodeId,
        sem: EachBlockSemantics,
        controlled_anchor: Option<String>,
    ) -> Result<()> {
        let is_controlled = controlled_anchor.is_some();
        let anchor_node = match controlled_anchor {
            Some(name) => name,
            None => self.comment_anchor_node_name(state, ctx)?,
        };
        let span_start = self.ctx.query.each_block(id).span.start;

        let plan = self.build_each_plan(id, &sem, is_controlled)?;
        let context_pattern = self.take_each_context_pattern(id)?;

        let async_thunk = if plan.needs_async {
            let collection_expr = self.take_node_expr(id)?;
            Some(self.ctx.b.async_thunk(collection_expr))
        } else {
            None
        };

        let collection_fn = self.build_each_collection_fn(id, &plan)?;
        let key_fn = self.build_each_key_fn(id, &plan, context_pattern.as_ref())?;
        let frag_fn = self.build_each_fragment_fn(ctx, id, &plan, context_pattern)?;

        if plan.needs_async {
            let anchor_expr = self.ctx.b.rid_expr(&anchor_node);
            let mut args: Vec<Arg<'a, '_>> = vec![
                Arg::Ident(&anchor_node),
                Arg::Num(plan.flags as f64),
                Arg::Expr(collection_fn),
                Arg::Expr(key_fn),
                Arg::Expr(frag_fn),
            ];
            if plan.has_fallback {
                let fallback_fn = self.build_each_fallback_fn(ctx, id)?;
                args.push(Arg::Expr(fallback_fn));
            }
            let each_call = self.ctx.b.call_expr("$.each", args);
            let each_stmt = self.add_svelte_meta(each_call, span_start, "each");
            let wrapped = self.emit_async_call_stmt(
                plan.has_await,
                &plan.blockers,
                anchor_expr,
                &anchor_node,
                "$$collection",
                async_thunk,
                vec![each_stmt],
            )?;
            state.init.push(wrapped);
            return Ok(());
        }

        let mut args: Vec<Arg<'a, '_>> = vec![
            Arg::Ident(&anchor_node),
            Arg::Num(plan.flags as f64),
            Arg::Expr(collection_fn),
            Arg::Expr(key_fn),
            Arg::Expr(frag_fn),
        ];
        if plan.has_fallback {
            let fallback_fn = self.build_each_fallback_fn(ctx, id)?;
            args.push(Arg::Expr(fallback_fn));
        }
        let each_call = self.ctx.b.call_expr("$.each", args);
        state
            .init
            .push(self.add_svelte_meta(each_call, span_start, "each"));
        Ok(())
    }

    fn build_each_plan(
        &mut self,
        block_id: NodeId,
        sem: &EachBlockSemantics,
        is_controlled: bool,
    ) -> Result<EachPlan> {
        let block = self.ctx.query.each_block(block_id);

        let (body_uses_index, key_uses_index) = match sem.index {
            EachIndexKind::Declared {
                used_in_body,
                used_in_key,
                ..
            } => (used_in_body, used_in_key),
            EachIndexKind::Absent => (false, false),
        };

        let user_index_name = block
            .index_span
            .map(|s| self.ctx.query.component.source_text(s).trim().to_string());

        let needs_group_index = matches!(sem.flavor, EachFlavor::BindGroup);
        let needs_collection_id = sem.shadows_outer;

        let render_index_name = if !(body_uses_index || needs_group_index || needs_collection_id) {
            None
        } else if let Some(name) = &user_index_name {
            Some(name.clone())
        } else {
            let generated = self.ctx.state.gen_ident("$$index");
            if needs_group_index {
                self.ctx
                    .state
                    .group_index_names
                    .insert(block_id, generated.clone());
            }
            Some(generated)
        };

        let collection_id_name = needs_collection_id.then(|| self.ctx.state.gen_ident("$$array"));

        let mut flags: u32 = sem.each_flags.bits() as u32;
        if is_controlled {
            flags |= EACH_IS_CONTROLLED;
        }

        let item_param_name = match &sem.item {
            EachItemKind::Identifier(_) => block
                .context_span
                .map(|cs| self.ctx.query.component.source_text(cs).trim().to_string())
                .unwrap_or_else(|| SYNTHETIC_ITEM_NAME.to_string()),
            EachItemKind::Pattern(_) | EachItemKind::NoBinding => SYNTHETIC_ITEM_NAME.to_string(),
        };

        let (needs_async, has_await, blockers) = match &sem.async_kind {
            EachAsyncKind::Sync => (false, false, Vec::new()),
            EachAsyncKind::Async {
                has_await,
                blockers,
            } => (true, *has_await, blockers.to_vec()),
        };

        Ok(EachPlan {
            flags,
            item_param_name,
            user_index_name,
            render_index_name,
            collection_id_name,
            key_uses_index,
            has_fallback: block.fallback.is_some(),
            item_reactive: sem.each_flags.contains(EachFlags::ITEM_REACTIVE),
            is_prop_source: matches!(sem.collection_kind, EachCollectionKind::PropSource),
            needs_async,
            has_await,
            blockers,
        })
    }

    fn take_each_context_pattern(
        &mut self,
        block_id: NodeId,
    ) -> Result<Option<BindingPattern<'a>>> {
        let block = self.ctx.query.each_block(block_id);
        let Some(cs) = block.context_span else {
            return Ok(None);
        };
        let Some(handle) = self.ctx.state.parsed.stmt_handle(cs.start) else {
            return CodegenError::missing_expression(block_id);
        };
        let Some(stmt) = self.ctx.state.parsed.take_stmt(handle) else {
            return CodegenError::missing_expression(block_id);
        };
        let Statement::VariableDeclaration(mut var_decl) = stmt else {
            return CodegenError::unexpected_node(
                block_id,
                "each context stmt must be VariableDeclaration",
            );
        };
        if var_decl.declarations.is_empty() {
            return CodegenError::unexpected_node(block_id, "each context has no declarators");
        }
        Ok(Some(var_decl.declarations.remove(0).id))
    }

    fn build_each_collection_fn(
        &mut self,
        block_id: NodeId,
        plan: &EachPlan,
    ) -> Result<Expression<'a>> {
        if plan.needs_async {
            return Ok(self
                .ctx
                .b
                .thunk(self.ctx.b.call_expr("$.get", [Arg::Ident("$$collection")])));
        }
        if plan.is_prop_source {
            let block = self.ctx.query.each_block(block_id);
            let src = self
                .ctx
                .query
                .component
                .source_text(block.expression_span)
                .trim();
            return Ok(self.ctx.b.rid_expr(src));
        }
        let expr = self.take_node_expr(block_id)?;
        Ok(self.ctx.b.thunk(expr))
    }

    fn build_each_key_fn(
        &mut self,
        block_id: NodeId,
        plan: &EachPlan,
        context_pattern: Option<&BindingPattern<'a>>,
    ) -> Result<Expression<'a>> {
        let block = self.ctx.query.each_block(block_id);
        let Some(ks) = block.key_span else {
            return Ok(self.ctx.b.rid_expr("$.index"));
        };
        let Some(handle) = self.ctx.state.parsed.expr_handle(ks.start) else {
            return Ok(self.ctx.b.rid_expr("$.index"));
        };
        let Some(key_expr) = self.ctx.state.parsed.take_expr(handle) else {
            return Ok(self.ctx.b.rid_expr("$.index"));
        };

        let key_body = self.ctx.b.expr_stmt(key_expr);
        let ctx_param = match context_pattern {
            Some(pattern)
                if matches!(
                    pattern,
                    BindingPattern::ArrayPattern(_) | BindingPattern::ObjectPattern(_)
                ) =>
            {
                let cloned = pattern.clone_in(self.ctx.b.ast.allocator);
                self.ctx.b.formal_parameter_from_pattern(cloned)
            }
            _ => self.ctx.b.formal_parameter_from_str(&plan.item_param_name),
        };

        if plan.key_uses_index {
            let Some(idx_name) = plan.user_index_name.as_ref() else {
                return CodegenError::unexpected_node(
                    block_id,
                    "key_uses_index implies user-declared index binding",
                );
            };
            let idx_param = self.ctx.b.formal_parameter_from_str(idx_name);
            Ok(self.ctx.b.arrow_expr(
                self.ctx.b.formal_parameters([ctx_param, idx_param]),
                [key_body],
            ))
        } else {
            Ok(self
                .ctx
                .b
                .arrow_expr(self.ctx.b.formal_parameters([ctx_param]), [key_body]))
        }
    }

    fn build_each_fragment_fn(
        &mut self,
        parent_ctx: &FragmentCtx<'a>,
        block_id: NodeId,
        plan: &EachPlan,
        context_pattern: Option<BindingPattern<'a>>,
    ) -> Result<Expression<'a>> {
        let key = FragmentKey::EachBody(block_id);
        let inner_ctx = parent_ctx.child_of_block(
            key,
            FragmentAnchor::CallbackParam {
                name: "$$anchor".to_string(),
                append_inside: false,
            },
        );
        let mut inner_state = EmitState::new();
        self.emit_fragment(&mut inner_state, &inner_ctx, key)?;
        let mut frag_body = self.pack_callback_body(inner_state, "$$anchor")?;

        if let Some(pattern) = context_pattern {
            if matches!(
                pattern,
                BindingPattern::ArrayPattern(_) | BindingPattern::ObjectPattern(_)
            ) {
                let mut decls = self.build_each_destructure_decls(pattern, plan.item_reactive)?;
                decls.append(&mut frag_body);
                frag_body = decls;
            }
        }

        let arrow = match (&plan.render_index_name, &plan.collection_id_name) {
            (Some(idx), Some(arr)) => self.ctx.b.arrow_block_expr(
                self.ctx
                    .b
                    .params(["$$anchor", &plan.item_param_name, idx, arr]),
                frag_body,
            ),
            (Some(idx), None) => self.ctx.b.arrow_block_expr(
                self.ctx.b.params(["$$anchor", &plan.item_param_name, idx]),
                frag_body,
            ),
            (None, _) => self.ctx.b.arrow_block_expr(
                self.ctx.b.params(["$$anchor", &plan.item_param_name]),
                frag_body,
            ),
        };
        Ok(arrow)
    }

    fn build_each_destructure_decls(
        &mut self,
        pattern: BindingPattern<'a>,
        item_reactive: bool,
    ) -> Result<Vec<Statement<'a>>> {
        match pattern {
            BindingPattern::ArrayPattern(arr) => {
                Ok(self.build_each_array_destructure(arr.unbox(), item_reactive))
            }
            BindingPattern::ObjectPattern(obj) => {
                Ok(self.build_each_object_destructure(obj.unbox(), item_reactive))
            }
            _ => Ok(Vec::new()),
        }
    }

    fn item_read_expr(&self, item_reactive: bool) -> Expression<'a> {
        if item_reactive {
            self.ctx.b.call_expr("$.get", [Arg::Ident("$$item")])
        } else {
            self.ctx.b.rid_expr("$$item")
        }
    }

    fn build_each_array_destructure(
        &mut self,
        arr: oxc_ast::ast::ArrayPattern<'a>,
        item_reactive: bool,
    ) -> Vec<Statement<'a>> {
        let elements: Vec<_> = arr.elements.into_iter().flatten().collect();
        let count = elements.len();
        let array_name = self.ctx.state.gen_ident("$$array");

        let item_expr = self.item_read_expr(item_reactive);
        let to_array = self
            .ctx
            .b
            .call_expr("$.to_array", [Arg::Expr(item_expr), Arg::Num(count as f64)]);
        let derived = self
            .ctx
            .b
            .call_expr("$.derived", [Arg::Expr(self.ctx.b.thunk(to_array))]);

        let mut decls = vec![self.ctx.b.var_stmt(&array_name, derived)];
        for (i, elem) in elements.into_iter().enumerate() {
            let Some(name) = array_element_name(&elem) else {
                continue;
            };
            let get_array = self.ctx.b.call_expr("$.get", [Arg::Ident(&array_name)]);
            let access = self
                .ctx
                .b
                .computed_member_expr(get_array, self.ctx.b.num_expr(i as f64));
            decls.push(self.ctx.b.let_init_stmt(&name, self.ctx.b.thunk(access)));
        }
        decls
    }

    fn build_each_object_destructure(
        &mut self,
        obj: oxc_ast::ast::ObjectPattern<'a>,
        item_reactive: bool,
    ) -> Vec<Statement<'a>> {
        use oxc_ast::ast::PropertyKey;

        let mut decls = Vec::new();
        for prop in obj.properties {
            let key_name = match &prop.key {
                PropertyKey::StaticIdentifier(id) => Some(id.name.as_str().to_string()),
                _ => None,
            };
            match prop.value {
                BindingPattern::BindingIdentifier(id) => {
                    let name = id.name.as_str().to_string();
                    let prop_key = key_name.as_deref().unwrap_or(&name);
                    let item_expr = self.item_read_expr(item_reactive);
                    let member = self.ctx.b.static_member_expr(item_expr, prop_key);
                    decls.push(self.ctx.b.let_init_stmt(&name, self.ctx.b.thunk(member)));
                }
                BindingPattern::AssignmentPattern(assign) => {
                    let assign = assign.unbox();
                    let BindingPattern::BindingIdentifier(id) = assign.left else {
                        continue;
                    };
                    let name = id.name.as_str().to_string();
                    let prop_key = key_name.as_deref().unwrap_or(&name);
                    let item_expr = self.item_read_expr(item_reactive);
                    let member = self.ctx.b.static_member_expr(item_expr, prop_key);
                    let fallback = self
                        .ctx
                        .b
                        .call_expr("$.fallback", [Arg::Expr(member), Arg::Expr(assign.right)]);
                    let derived = self.ctx.b.call_expr(
                        "$.derived_safe_equal",
                        [Arg::Expr(self.ctx.b.thunk(fallback))],
                    );
                    decls.push(self.ctx.b.let_init_stmt(&name, derived));
                }
                _ => continue,
            }
        }
        decls
    }

    fn build_each_fallback_fn(
        &mut self,
        parent_ctx: &FragmentCtx<'a>,
        block_id: NodeId,
    ) -> Result<Expression<'a>> {
        let key = FragmentKey::EachFallback(block_id);
        let inner_ctx = parent_ctx.child_of_block(
            key,
            FragmentAnchor::CallbackParam {
                name: "$$anchor".to_string(),
                append_inside: false,
            },
        );
        let mut inner_state = EmitState::new();
        self.emit_fragment(&mut inner_state, &inner_ctx, key)?;
        let body = self.pack_callback_body(inner_state, "$$anchor")?;
        Ok(self
            .ctx
            .b
            .arrow_block_expr(self.ctx.b.params(["$$anchor"]), body))
    }
}
