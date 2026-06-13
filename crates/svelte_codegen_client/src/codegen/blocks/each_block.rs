use crate::codegen::binding_pattern::{BindingPatternOutput, BindingPatternSource};
use crate::codegen::expr::coarse_wrap;
use oxc_allocator::CloneIn;
use oxc_ast::ast::{BindingPattern, Expression, Statement};
use svelte_analyze::{
    EachAsyncKind, EachBlockSemantics, EachCollectionSource, EachFlags, EachFlavor, EachIndexKind,
    EachItemKind, EachKeyKind,
};
use svelte_ast::NodeId;
use svelte_ast_builder::Arg;
use svelte_component_semantics::OxcNodeId;
use svelte_emit_builders::runes::rune_get;

use super::super::data_structures::EmitState;
use super::super::data_structures::{FragmentAnchor, FragmentCtx};
use super::super::{Codegen, CodegenError, Result};

const EACH_IS_CONTROLLED: u32 = 4;
const SYNTHETIC_ITEM_NAME: &str = "$$item";

struct EachEmit {
    flags: u32,
    item_param_name: String,
    user_index_name: Option<String>,
    render_index_name: Option<String>,
    collection_id_name: Option<String>,
    key_uses_index: bool,
    key_is_index: bool,
    has_fallback: bool,
    collection_source: EachCollectionSource,
    async_kind: EachAsyncKind,
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

        let async_thunk = match &plan.async_kind {
            EachAsyncKind::Awaited { .. } => {
                let collection_expr = self.take_node_expr(id)?;
                Some(self.ctx.b.async_thunk(collection_expr))
            }
            EachAsyncKind::Deferred { .. } | EachAsyncKind::Sync => None,
        };

        let item_pattern_node = match &sem.item {
            EachItemKind::Pattern(node) => Some(*node),
            _ => None,
        };

        let collection_fn = self.build_each_collection_fn(id, &plan)?;
        let key_fn = self.build_each_key_fn(id, &plan, context_pattern.as_ref())?;
        let frag_fn =
            self.build_each_fragment_fn(ctx, id, &plan, context_pattern, item_pattern_node)?;

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

        match &plan.async_kind {
            EachAsyncKind::Awaited { blockers } | EachAsyncKind::Deferred { blockers } => {
                let blockers = blockers.to_vec();
                let anchor_expr = self.ctx.b.rid_expr(&anchor_node);
                let each_stmt = self.add_svelte_meta(each_call, span_start, "each");
                let wrapped = self.emit_async_call_stmt(
                    &blockers,
                    anchor_expr,
                    &anchor_node,
                    "$$collection",
                    async_thunk,
                    vec![each_stmt],
                )?;
                state.init.push(wrapped);
                Ok(())
            }
            EachAsyncKind::Sync => {
                state
                    .init
                    .push(self.add_svelte_meta(each_call, span_start, "each"));
                Ok(())
            }
        }
    }

    fn build_each_plan(
        &mut self,
        block_id: NodeId,
        sem: &EachBlockSemantics,
        is_controlled: bool,
    ) -> Result<EachEmit> {
        let block = self.ctx.query.each_block(block_id);

        let (body_uses_index, key_uses_index) = match sem.index {
            EachIndexKind::Declared {
                used_in_body,
                used_in_key,
                ..
            } => (used_in_body, used_in_key),
            EachIndexKind::Absent => (false, false),
        };

        let user_index_name = block.index.as_ref().map(|r| {
            self.ctx
                .query
                .component
                .source_text(r.span)
                .trim()
                .to_string()
        });

        let needs_group_index = matches!(sem.flavor, EachFlavor::BindGroup);
        let needs_collection_id = sem.shadows_outer;
        let needs_store_index = sem.each_flags.contains(EachFlags::ITEM_REACTIVE)
            && match &sem.item {
                EachItemKind::Identifier(sym) => self.ctx.query.scoping().is_member_mutated(*sym),
                _ => false,
            };

        let render_index_name = if !(body_uses_index
            || needs_group_index
            || needs_collection_id
            || needs_store_index)
        {
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
                .context
                .as_ref()
                .map(|r| {
                    self.ctx
                        .query
                        .component
                        .source_text(r.span)
                        .trim()
                        .to_string()
                })
                .unwrap_or_else(|| SYNTHETIC_ITEM_NAME.to_string()),
            EachItemKind::Pattern(_) | EachItemKind::NoBinding => SYNTHETIC_ITEM_NAME.to_string(),
        };

        Ok(EachEmit {
            flags,
            item_param_name,
            user_index_name,
            render_index_name,
            collection_id_name,
            key_uses_index,
            key_is_index: matches!(sem.key, EachKeyKind::KeyedByIndex),
            has_fallback: block.fallback.is_some(),
            collection_source: sem.collection.source.clone(),
            async_kind: sem.async_kind.clone(),
        })
    }

    fn take_each_context_pattern(
        &mut self,
        block_id: NodeId,
    ) -> Result<Option<BindingPattern<'a>>> {
        let block = self.ctx.query.each_block(block_id);
        let Some(context_ref) = block.context.as_ref() else {
            return Ok(None);
        };
        let Some(stmt) = self.ctx.state.parsed.take_stmt(context_ref.id()) else {
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
        plan: &EachEmit,
    ) -> Result<Expression<'a>> {
        match &plan.async_kind {
            EachAsyncKind::Awaited { .. } | EachAsyncKind::Deferred { .. } => {
                Ok(self.ctx.b.thunk(rune_get(&self.ctx.b, "$$collection")))
            }
            EachAsyncKind::Sync => {
                if let EachCollectionSource::Prop { sym } = &plan.collection_source {
                    let name = self.ctx.query.symbol_name(*sym).to_string();
                    return Ok(self.ctx.b.rid_expr(&name));
                }
                let expr = self.take_node_expr(block_id)?;
                let wrapped = coarse_wrap(self.ctx, expr, self.ctx.expression_data(block_id));
                Ok(self.ctx.b.thunk(wrapped))
            }
        }
    }

    fn build_each_key_fn(
        &mut self,
        block_id: NodeId,
        plan: &EachEmit,
        context_pattern: Option<&BindingPattern<'a>>,
    ) -> Result<Expression<'a>> {
        if plan.key_is_index {
            return Ok(self.ctx.b.rid_expr("$.index"));
        }
        let block = self.ctx.query.each_block(block_id);
        let Some(key_ref) = block.key.as_ref() else {
            return Ok(self.ctx.b.rid_expr("$.index"));
        };
        let Some(key_expr) = self.ctx.state.parsed.take_expr(key_ref.id()) else {
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
        plan: &EachEmit,
        context_pattern: Option<BindingPattern<'a>>,
        item_pattern_node: Option<OxcNodeId>,
    ) -> Result<Expression<'a>> {
        let body = match self.ctx.query.component.store.get(block_id) {
            svelte_ast::Node::EachBlock(block) => block.body,
            _ => return CodegenError::unexpected_node(block_id, "EachBlock"),
        };
        let mut inner_ctx = parent_ctx.child_of_block(
            self.ctx,
            body,
            FragmentAnchor::CallbackParam {
                name: "$$anchor".to_string(),
                append_inside: false,
            },
        );
        inner_ctx.in_block_callback = true;
        let mut inner_state = EmitState::new();

        let saved_writeback_places = self.ctx.state.each_item_writeback_places.take();
        let each_item_decls: Option<Vec<Statement<'a>>> = match (context_pattern, item_pattern_node)
        {
            (Some(pattern), Some(decl_node))
                if matches!(
                    pattern,
                    BindingPattern::ArrayPattern(_) | BindingPattern::ObjectPattern(_)
                ) =>
            {
                let pattern_ref: &'a BindingPattern<'a> = self.ctx.b.ast.allocator.alloc(pattern);
                let BindingPatternOutput::EachItem {
                    decls,
                    writeback_places,
                } = self.emit_binding_pattern(
                    decl_node,
                    BindingPatternSource::EachItem {
                        block_id,
                        pattern: pattern_ref,
                    },
                )?
                else {
                    return CodegenError::unexpected_child(
                        "each-item binding output",
                        "other binding output",
                    );
                };
                self.ctx.state.each_item_writeback_places = Some(writeback_places);
                Some(decls)
            }
            _ => None,
        };

        self.emit_fragment(&mut inner_state, &inner_ctx, body)?;
        let mut frag_body = self.pack_callback_body(inner_state, "$$anchor")?;

        self.ctx.state.each_item_writeback_places = saved_writeback_places;

        if let Some(mut decls) = each_item_decls {
            decls.append(&mut frag_body);
            frag_body = decls;
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

    fn build_each_fallback_fn(
        &mut self,
        parent_ctx: &FragmentCtx<'a>,
        block_id: NodeId,
    ) -> Result<Expression<'a>> {
        let fallback = match self.ctx.query.component.store.get(block_id) {
            svelte_ast::Node::EachBlock(block) => match block.fallback {
                Some(fb) => fb,
                None => return Ok(self.ctx.b.null_expr()),
            },
            _ => return CodegenError::unexpected_node(block_id, "EachBlock"),
        };
        let mut inner_ctx = parent_ctx.child_of_block(
            self.ctx,
            fallback,
            FragmentAnchor::CallbackParam {
                name: "$$anchor".to_string(),
                append_inside: false,
            },
        );
        inner_ctx.in_block_callback = true;
        let mut inner_state = EmitState::new();
        self.emit_fragment(&mut inner_state, &inner_ctx, fallback)?;
        let body = self.pack_callback_body(inner_state, "$$anchor")?;
        Ok(self
            .ctx
            .b
            .arrow_block_expr(self.ctx.b.params(["$$anchor"]), body))
    }
}
