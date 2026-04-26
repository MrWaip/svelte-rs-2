use oxc_ast::ast::Expression;
use svelte_ast::NodeId;
use svelte_ast_builder::{Arg, AssignLeft, TemplatePart};

use super::super::data_structures::ConcatPart;
use super::super::data_structures::{EmitState, FragmentCtx, MemoValueRef, TemplateMemoState};
use super::super::{Codegen, CodegenError, Result};

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(in crate::codegen) fn emit_title_element(
        &mut self,
        state: &mut EmitState<'a>,
        _ctx: &FragmentCtx<'a>,
        id: NodeId,
    ) -> Result<()> {
        let el = match self.ctx.query.component.store.get(id) {
            svelte_ast::Node::Element(el) => el,
            _ => return CodegenError::unexpected_node(id, "title element"),
        };
        let fragment_nodes = self
            .ctx
            .query
            .component
            .store
            .fragment(el.fragment)
            .nodes
            .clone();

        let mut parts: Vec<ConcatPart> = Vec::new();
        for child_id in &fragment_nodes {
            match self.ctx.query.component.store.get(*child_id) {
                svelte_ast::Node::Text(t) => {
                    let text = t.value(&self.ctx.query.component.source).to_string();
                    parts.push(ConcatPart::StaticOwned(text));
                }
                svelte_ast::Node::ExpressionTag(ex) => {
                    parts.push(ConcatPart::Expr(ex.id));
                }
                _ => {}
            }
        }

        enum TitlePart<'a> {
            Str(String),
            Expr(Expression<'a>, bool),
            SyncMemo(usize, bool),
            AsyncMemo(usize, bool),
        }
        impl<'a> TitlePart<'a> {
            fn is_defined(&self) -> bool {
                match self {
                    Self::Str(_) => true,
                    Self::Expr(_, d) | Self::SyncMemo(_, d) | Self::AsyncMemo(_, d) => *d,
                }
            }
        }

        let mut memo = TemplateMemoState::default();
        let mut built_parts: Vec<TitlePart<'a>> = Vec::new();
        let mut has_state = false;
        let mut expr_count = 0usize;

        for part in &parts {
            match part {
                ConcatPart::Static(_) => {}
                ConcatPart::StaticOwned(s) => {
                    if let Some(TitlePart::Str(prev)) = built_parts.last_mut() {
                        prev.push_str(s);
                    } else {
                        built_parts.push(TitlePart::Str(s.clone()));
                    }
                }
                ConcatPart::Expr(eid) => {
                    expr_count += 1;
                    if self.ctx.is_dynamic(*eid)
                        || self.ctx.expr_has_await(*eid)
                        || self.ctx.expr_has_blockers(*eid)
                    {
                        has_state = true;
                    }
                    let expr = self.take_node_expr(*eid)?;
                    if let Some(known) = self.try_resolve_known_from_expr(&expr) {
                        if let Some(TitlePart::Str(prev)) = built_parts.last_mut() {
                            prev.push_str(&known);
                        } else {
                            built_parts.push(TitlePart::Str(known));
                        }
                        continue;
                    }
                    let defined = self.is_node_expr_definitely_defined(*eid, &expr);
                    let info_clone = self.ctx.expression(*eid).cloned();
                    if let Some(info) = info_clone {
                        let cloned_expr = self.ctx.b.clone_expr(&expr);
                        match memo.add_memoized_expr(self.ctx, &info, cloned_expr) {
                            Some(MemoValueRef::Sync(idx)) => {
                                built_parts.push(TitlePart::SyncMemo(idx, defined));
                            }
                            Some(MemoValueRef::Async(idx)) => {
                                built_parts.push(TitlePart::AsyncMemo(idx, defined));
                            }
                            None => {
                                built_parts.push(TitlePart::Expr(expr, defined));
                            }
                        }
                    } else {
                        built_parts.push(TitlePart::Expr(expr, defined));
                    }
                }
            }
        }

        let single_dynamic_expr = expr_count == 1
            && parts.len() == 1
            && matches!(parts.first(), Some(ConcatPart::Expr(_)));

        let part_expr = |cg: &Codegen<'a, 'ctx>,
                         memo: &TemplateMemoState<'a>,
                         p: TitlePart<'a>|
         -> Expression<'a> {
            match p {
                TitlePart::Str(s) => cg.ctx.b.str_expr(&s),
                TitlePart::Expr(e, _) => e,
                TitlePart::SyncMemo(i, _) => memo.sync_param_expr(cg.ctx, i),
                TitlePart::AsyncMemo(i, _) => memo.async_param_expr(cg.ctx, i),
            }
        };

        let value_expr = if built_parts.is_empty() {
            self.ctx.b.str_expr("")
        } else if built_parts.len() == 1 {
            match built_parts.pop() {
                Some(p) => part_expr(self, &memo, p),
                None => self.ctx.b.str_expr(""),
            }
        } else {
            let tpl: Vec<TemplatePart<'a>> = built_parts
                .into_iter()
                .map(|p| match p {
                    TitlePart::Str(s) => TemplatePart::Str(s),
                    other => {
                        let defined = other.is_defined();
                        TemplatePart::Expr(part_expr(self, &memo, other), defined)
                    }
                })
                .collect();
            self.ctx.b.template_parts_expr(tpl)
        };

        let value = if has_state && single_dynamic_expr {
            self.ctx
                .b
                .logical_coalesce(value_expr, self.ctx.b.str_expr(""))
        } else {
            value_expr
        };

        let b = &self.ctx.state.b;
        let doc_expr = b.static_member_expr(b.rid_expr("$"), "document");
        let title_member = b.static_member(doc_expr, "title");
        let assignment = b.assign_stmt(AssignLeft::StaticMember(title_member), value);

        if has_state {
            let params = memo.param_names();
            let arrow = if params.is_empty() {
                self.ctx.b.thunk_block(vec![assignment])
            } else {
                self.ctx.b.arrow_block_expr(
                    self.ctx.b.params(params.iter().map(|s| s.as_str())),
                    vec![assignment],
                )
            };
            if memo.has_deps() {
                super::super::effect::emit_effect_call_extern(
                    self.ctx,
                    "$.deferred_template_effect",
                    arrow,
                    &mut memo,
                    &mut state.init,
                );
            } else {
                state.init.push(
                    self.ctx
                        .b
                        .call_stmt("$.deferred_template_effect", [Arg::Expr(arrow)]),
                );
            }
        } else {
            let arrow = self.ctx.b.thunk_block(vec![assignment]);
            state
                .init
                .push(self.ctx.b.call_stmt("$.effect", [Arg::Expr(arrow)]));
        }
        Ok(())
    }
}
