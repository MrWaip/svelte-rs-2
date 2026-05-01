use oxc_ast::ast::{Expression, Statement};
use svelte_ast::{NodeId, SVELTE_COMPONENT, SVELTE_SELF};
use svelte_ast_builder::{Arg, ObjProp};

use super::super::data_structures::EmitState;
use super::super::data_structures::FragmentCtx;
use super::super::fragment::SlotFragmentOutcome;
use super::super::{Codegen, CodegenError, Result};

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(in crate::codegen) fn emit_component(
        &mut self,
        state: &mut EmitState<'a>,
        ctx: &FragmentCtx<'a>,
        el_id: NodeId,
        _existing_var: Option<&str>,
    ) -> Result<String> {
        let (cn_name, span_start, cn_fragment, named_slots) = {
            let Some(view) = self
                .ctx
                .query
                .component
                .store
                .get(el_id)
                .as_component_like()
            else {
                return CodegenError::unexpected_node(el_id, "component-like");
            };
            let named_slots: Vec<(String, NodeId)> = view
                .legacy_slots
                .iter()
                .map(|slot| {
                    (
                        slot.name.clone(),
                        self.ctx.query.component.fragment_nodes(slot.fragment)[0],
                    )
                })
                .collect();
            (
                view.name.to_string(),
                view.span.start,
                view.fragment,
                named_slots,
            )
        };

        let snippet_ids: Vec<NodeId> = self.ctx.component_snippets(el_id).to_vec();
        let is_dynamic = self.ctx.is_dynamic_component(el_id) || cn_name == SVELTE_COMPONENT;

        let mut props = self.build_component_props(el_id, &cn_name)?;

        let mut init_stmts: Vec<Statement<'a>> = Vec::new();
        let events = std::mem::take(&mut props.events);
        self.build_component_events(el_id, events, &mut props.items, &mut init_stmts)?;
        for stmt in init_stmts {
            state.init.push(stmt);
        }

        let snippet_children =
            self.build_component_snippet_children(&snippet_ids, &mut props.items)?;

        let default_has_let = self.default_slot_has_let_directive_legacy(el_id);
        let children_body = if default_has_let {
            self.build_component_default_children_with_let(ctx, el_id, cn_fragment)?
        } else {
            self.build_component_default_children(ctx, cn_fragment)?
        };

        let mut slot_entries: Vec<ObjProp<'a>> = Vec::new();
        for slot_key in &snippet_children.slot_keys {
            let key = self.ctx.b.alloc_str(slot_key);
            slot_entries.push(ObjProp::KeyValue(key, self.ctx.b.bool_expr(true)));
        }
        if let Some(arrow) = children_body {
            let arrow = self.maybe_wrap_slot_snippet_dev(arrow);
            if default_has_let {
                props
                    .items
                    .push(super::super::component_props::PropOrSpread::Prop(
                        ObjProp::KeyValue(
                            "children",
                            self.ctx.b.static_member_expr(
                                self.ctx.b.rid_expr("$"),
                                "invalid_default_snippet",
                            ),
                        ),
                    ));
                slot_entries.push(ObjProp::KeyValue("default", arrow));
            } else {
                slot_entries.push(ObjProp::KeyValue("default", self.ctx.b.bool_expr(true)));
                props
                    .items
                    .push(super::super::component_props::PropOrSpread::Prop(
                        ObjProp::KeyValue("children", arrow),
                    ));
            }
        }
        for (slot_name, slot_el_id) in named_slots {
            match self.emit_slot_fragment_legacy_component_only_dont_use(ctx, el_id, slot_el_id)? {
                SlotFragmentOutcome::Empty => continue,
                SlotFragmentOutcome::Arrow(arrow) => {
                    let arrow = self.maybe_wrap_slot_snippet_dev(arrow);
                    let key = self.ctx.b.alloc_str(&slot_name);
                    slot_entries.push(ObjProp::KeyValue(key, arrow));
                }
            }
        }
        if !slot_entries.is_empty() {
            props
                .items
                .push(super::super::component_props::PropOrSpread::Prop(
                    ObjProp::KeyValue("$$slots", self.ctx.b.object_expr(slot_entries)),
                ));
        }

        if self.ctx.query.component_needs_legacy_props_marker(el_id) {
            props
                .items
                .push(super::super::component_props::PropOrSpread::Prop(
                    ObjProp::KeyValue("$$legacy", self.ctx.b.bool_expr(true)),
                ));
        }

        let props_expr = self.build_props_expr(props.items);

        if is_dynamic {
            return self.emit_dynamic_component(
                state,
                ctx,
                el_id,
                &cn_name,
                props.bind_this,
                props.svelte_component_this,
                props_expr,
                snippet_children.decls,
                props.memo_decls,
                props.ownership_bindings,
                span_start,
            );
        }

        let anchor_expr = self.direct_anchor_expr(state, ctx)?;

        let callee: &str = if cn_name == SVELTE_SELF {
            self.ctx.state.name
        } else {
            self.ctx.b.alloc_str(&cn_name)
        };
        let component_call = self
            .ctx
            .b
            .call_expr(callee, [Arg::Expr(anchor_expr), Arg::Expr(props_expr)]);

        let final_expr = if let Some(bind_id) = props.bind_this {
            self.build_bind_this_call(el_id, bind_id, component_call)?
        } else {
            component_call
        };

        let component_stmt = if cn_name == SVELTE_SELF {
            self.ctx.b.expr_stmt(final_expr)
        } else {
            let extra_obj = self.ctx.b.object_expr([ObjProp::KeyValue(
                "componentTag",
                self.ctx.b.str_expr(&cn_name),
            )]);
            self.add_svelte_meta_with_extra(final_expr, span_start, "component", Some(extra_obj))
        };

        let ownership_stmts = self.build_ownership_binding_stmts(&props.ownership_bindings, callee);
        if snippet_children.decls.is_empty()
            && props.memo_decls.is_empty()
            && ownership_stmts.is_empty()
        {
            state.init.push(component_stmt);
        } else {
            let mut block = snippet_children.decls;
            block.extend(props.memo_decls);
            block.extend(ownership_stmts);
            block.push(component_stmt);
            state.init.push(self.ctx.b.block_stmt(block));
        }
        Ok(String::new())
    }

    fn build_ownership_binding_stmts(
        &self,
        bindings: &[super::super::component_props::OwnershipBinding<'a>],
        comp_id: &str,
    ) -> Vec<Statement<'a>> {
        bindings
            .iter()
            .map(|b| {
                self.ctx.b.call_stmt(
                    "$$ownership_validator.binding",
                    [
                        Arg::Str(b.name.clone()),
                        Arg::Ident(comp_id),
                        Arg::Ident(b.source_ident),
                    ],
                )
            })
            .collect()
    }

    fn maybe_wrap_slot_snippet_dev(&self, arrow: Expression<'a>) -> Expression<'a> {
        if !self.ctx.state.dev {
            return arrow;
        }
        let component = self.ctx.b.rid_expr(self.ctx.state.name);
        self.ctx
            .b
            .call_expr("$.wrap_snippet", [Arg::Expr(component), Arg::Expr(arrow)])
    }

    #[allow(clippy::too_many_arguments)]
    fn emit_dynamic_component(
        &mut self,
        state: &mut EmitState<'a>,
        ctx: &FragmentCtx<'a>,
        el_id: NodeId,
        cn_name: &str,
        bind_this_info: Option<NodeId>,
        svelte_component_this: Option<Expression<'a>>,
        props_expr: Expression<'a>,
        snippet_decls: Vec<Statement<'a>>,
        memo_decls: Vec<Statement<'a>>,
        ownership_bindings: Vec<super::super::component_props::OwnershipBinding<'a>>,
        span_start: u32,
    ) -> Result<String> {
        let anchor_node = self.comment_anchor_node_name(state, ctx)?;

        let (intermediate_ref, component_thunk): (&str, Expression<'a>) =
            if cn_name == SVELTE_COMPONENT {
                let Some(this_expr) = svelte_component_this else {
                    return CodegenError::unexpected_node(
                        el_id,
                        "<svelte:component> missing `this` attribute",
                    );
                };
                ("$$component", self.ctx.b.thunk(this_expr))
            } else {
                let intermediate = cn_name.replace('.', "_");
                let intermediate_name = self.ctx.state.gen_ident(&intermediate);
                let intermediate_ref: &str = self.ctx.b.alloc_str(&intermediate_name);
                let component_ref = self.build_dynamic_component_ref(el_id, cn_name)?;
                (intermediate_ref, self.ctx.b.thunk(component_ref))
            };

        let inner_call = self.ctx.b.call_expr(
            intermediate_ref,
            [Arg::Ident("$$anchor"), Arg::Expr(props_expr)],
        );
        let inner_final = if let Some(bind_id) = bind_this_info {
            self.build_bind_this_call(el_id, bind_id, inner_call)?
        } else {
            inner_call
        };
        let mut inner_body: Vec<Statement<'a>> = memo_decls;
        inner_body
            .extend(self.build_ownership_binding_stmts(&ownership_bindings, intermediate_ref));
        inner_body.push(self.ctx.b.expr_stmt(inner_final));
        let inner_arrow = self.ctx.b.arrow_block_expr(
            self.ctx.b.params(["$$anchor", intermediate_ref]),
            inner_body,
        );

        let component_call = self.ctx.b.call_expr(
            "$.component",
            [
                Arg::Ident(&anchor_node),
                Arg::Expr(component_thunk),
                Arg::Expr(inner_arrow),
            ],
        );

        let extra_obj = self.ctx.b.object_expr([ObjProp::KeyValue(
            "componentTag",
            self.ctx.b.str_expr(cn_name),
        )]);
        let component_stmt = self.add_svelte_meta_with_extra(
            component_call,
            span_start,
            "component",
            Some(extra_obj),
        );

        if snippet_decls.is_empty() {
            state.init.push(component_stmt);
        } else {
            let mut block = snippet_decls;
            block.push(component_stmt);
            state.init.push(self.ctx.b.block_stmt(block));
        }
        Ok(anchor_node)
    }
}
