use std::mem;

use oxc_ast::ast::{Expression, Statement};
use svelte_analyze::{LegacyDefaultSlot, Volatility};
use svelte_ast::{Node, NodeId};
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
        existing_var: Option<&str>,
    ) -> Result<String> {
        self.emit_component_impl(state, ctx, el_id, existing_var, None, 0)
    }

    pub(in crate::codegen) fn emit_component_with_hoisted_memo(
        &mut self,
        state: &mut EmitState<'a>,
        ctx: &FragmentCtx<'a>,
        el_id: NodeId,
        existing_var: Option<&str>,
        memo_decls_out: &mut Vec<Statement<'a>>,
        initial_memo_counter: u32,
    ) -> Result<String> {
        self.emit_component_impl(
            state,
            ctx,
            el_id,
            existing_var,
            Some(memo_decls_out),
            initial_memo_counter,
        )
    }

    fn emit_component_impl(
        &mut self,
        state: &mut EmitState<'a>,
        ctx: &FragmentCtx<'a>,
        el_id: NodeId,
        _existing_var: Option<&str>,
        memo_decls_out: Option<&mut Vec<Statement<'a>>>,
        initial_memo_counter: u32,
    ) -> Result<String> {
        let node = self.ctx.query.component.store.get(el_id);
        let is_svelte_component_legacy = matches!(node, Node::SvelteComponentLegacy(_));
        let is_svelte_self = matches!(node, Node::SvelteSelf(_));

        let (cn_name, span_start, cn_fragment, named_slots) = {
            let Some(view) = node.as_component_like() else {
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
            let cn_name = match node {
                Node::ComponentNode(cn) => self
                    .ctx
                    .query
                    .component
                    .source_text(cn.name.span)
                    .to_string(),
                Node::SvelteComponentLegacy(_) => svelte_ast::SVELTE_COMPONENT.to_string(),
                Node::SvelteSelf(_) => svelte_ast::SVELTE_SELF.to_string(),
                _ => unreachable!("component-like guard"),
            };
            (cn_name, view.span.start, view.fragment, named_slots)
        };

        let snippet_ids: Vec<NodeId> = self.ctx.component_snippets(el_id).to_vec();

        let mut props = self.build_component_props(el_id, initial_memo_counter)?;

        let mut init_stmts: Vec<Statement<'a>> = Vec::new();
        let events = mem::take(&mut props.events);
        self.build_component_events(el_id, events, &mut props.items, &mut init_stmts)?;
        let mut bind_init_stmts = mem::take(&mut props.bind_init_stmts);

        let (anchor_expr_early, dynamic_anchor_name) =
            match self.ctx.expression_data(el_id).map(|d| d.volatility) {
                Some(Volatility::Reactive | Volatility::Heavy | Volatility::Asynchronous) => {
                    (None, Some(self.comment_anchor_node_name(state, ctx)?))
                }
                Some(Volatility::Static) | None => {
                    let anchor = self.direct_anchor_expr(state, ctx)?;
                    for stmt in bind_init_stmts.drain(..) {
                        state.init.push(stmt);
                    }
                    for stmt in init_stmts.drain(..) {
                        state.init.push(stmt);
                    }
                    (Some(anchor), None)
                }
            };

        let reserved_intermediate: Option<&'a str> =
            if dynamic_anchor_name.is_some() && !is_svelte_component_legacy {
                let base = cn_name.replace('.', "_");
                let name = self.ctx.state.gen_ident(&base);
                Some(self.ctx.b.alloc_str(&name))
            } else {
                None
            };

        let snippet_children =
            self.build_component_snippet_children(&snippet_ids, &mut props.items)?;

        let children_body =
            self.build_component_default_children_with_let(ctx, el_id, cn_fragment)?;

        let mut slot_entries: Vec<ObjProp<'a>> = Vec::new();
        for slot_key in &snippet_children.slot_keys {
            let key = self.ctx.b.alloc_str(slot_key);
            slot_entries.push(ObjProp::KeyValue(key, self.ctx.b.bool_expr(true)));
        }
        if let Some(arrow) = children_body {
            match self.ctx.query.legacy_default_slot(el_id) {
                LegacyDefaultSlot::ChildrenProp => {
                    slot_entries.push(ObjProp::KeyValue("default", self.ctx.b.bool_expr(true)));
                    let arrow = self.maybe_wrap_slot_snippet_dev(arrow);
                    props
                        .items
                        .push(super::super::component_props::PropOrSpread::Prop(
                            ObjProp::KeyValue("children", arrow),
                        ));
                }
                LegacyDefaultSlot::SlotDefaultInvalid => {
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
                }
                LegacyDefaultSlot::SlotDefault => {
                    slot_entries.push(ObjProp::KeyValue("default", arrow));
                }
            }
        }
        for (slot_name, slot_el_id) in named_slots {
            match self.emit_slot_fragment_legacy_component_only_dont_use(ctx, el_id, slot_el_id)? {
                SlotFragmentOutcome::Empty => continue,
                SlotFragmentOutcome::Arrow(arrow) => {
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

        match self.ctx.expression_data(el_id).map(|d| d.volatility) {
            Some(Volatility::Reactive | Volatility::Heavy | Volatility::Asynchronous) => {
                let anchor_node = dynamic_anchor_name
                    .expect("dynamic component must have a pre-allocated anchor name");
                return self.emit_dynamic_component(
                    state,
                    el_id,
                    &cn_name,
                    is_svelte_component_legacy,
                    props.bind_this,
                    props.svelte_component_this,
                    props_expr,
                    snippet_children.decls,
                    props.memo_decls,
                    props.ownership_bindings,
                    bind_init_stmts,
                    init_stmts,
                    props.validate_binding_stmts,
                    span_start,
                    anchor_node,
                    reserved_intermediate,
                );
            }
            Some(Volatility::Static) | None => {}
        }

        let anchor_expr = anchor_expr_early
            .ok_or(())
            .or_else(|()| self.direct_anchor_expr(state, ctx))?;

        let callee: &str = if is_svelte_self {
            self.ctx.state.name
        } else {
            self.ctx.b.alloc_str(&cn_name)
        };
        let name_expr_id = match self.ctx.query.component.store.get(el_id) {
            Node::ComponentNode(cn) => Some(cn.name.id()),
            _ => None,
        };
        let callee_expr = match name_expr_id {
            Some(id) => self
                .ctx
                .state
                .parsed
                .take_expr(id)
                .ok_or(CodegenError::MissingExpression(el_id))?,
            None => self.ctx.b.rid_expr(callee),
        };
        let component_call = self
            .ctx
            .b
            .call_expr_callee(callee_expr, [Arg::Expr(anchor_expr), Arg::Expr(props_expr)]);

        let (final_expr, bind_this_validate) = if let Some(bind_id) = props.bind_this {
            self.build_bind_this_call(el_id, bind_id, component_call)?
        } else {
            (component_call, None)
        };

        let component_stmt = if self.ctx.has_component_css_props(el_id) {
            self.ctx.b.expr_stmt(final_expr)
        } else {
            let component_tag = if is_svelte_self {
                "svelte:self"
            } else {
                cn_name.as_str()
            };
            let extra_obj = self.ctx.b.object_expr([ObjProp::KeyValue(
                "componentTag",
                self.ctx.b.str_expr(component_tag),
            )]);
            self.add_svelte_meta_with_extra(final_expr, span_start, "component", Some(extra_obj))
        };

        let ownership_stmts = self.build_ownership_binding_stmts(&props.ownership_bindings, callee);
        let body_memo_decls: Vec<Statement<'a>> = if let Some(out) = memo_decls_out {
            out.extend(props.memo_decls);
            Vec::new()
        } else {
            props.memo_decls
        };
        if let Some(stmt) = bind_this_validate {
            state.init.push(stmt);
        }
        for stmt in props.validate_binding_stmts {
            state.init.push(stmt);
        }
        if snippet_children.decls.is_empty()
            && body_memo_decls.is_empty()
            && ownership_stmts.is_empty()
        {
            state.init.push(component_stmt);
        } else {
            let mut block = snippet_children.decls;
            block.extend(body_memo_decls);
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
                        Arg::Str(b.source_ident.to_string()),
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
        el_id: NodeId,
        cn_name: &str,
        is_svelte_component_legacy: bool,
        bind_this_info: Option<NodeId>,
        svelte_component_this: Option<Expression<'a>>,
        props_expr: Expression<'a>,
        snippet_decls: Vec<Statement<'a>>,
        memo_decls: Vec<Statement<'a>>,
        ownership_bindings: Vec<super::super::component_props::OwnershipBinding<'a>>,
        bind_init_stmts: Vec<Statement<'a>>,
        init_stmts: Vec<Statement<'a>>,
        validate_binding_stmts: Vec<Statement<'a>>,
        span_start: u32,
        anchor_node: String,
        reserved_intermediate: Option<&'a str>,
    ) -> Result<String> {
        for stmt in bind_init_stmts {
            state.init.push(stmt);
        }
        for stmt in init_stmts {
            state.init.push(stmt);
        }
        for stmt in validate_binding_stmts {
            state.init.push(stmt);
        }

        let (intermediate_ref, component_thunk): (&str, Expression<'a>) =
            if is_svelte_component_legacy {
                let Some(this_expr) = svelte_component_this else {
                    return CodegenError::unexpected_node(
                        el_id,
                        "<svelte:component> missing `this` attribute",
                    );
                };
                ("$$component", self.ctx.b.thunk(this_expr))
            } else {
                let Some(name) = reserved_intermediate else {
                    return CodegenError::unexpected_node(
                        el_id,
                        "dynamic component missing pre-reserved intermediate name",
                    );
                };
                let component_ref = self.build_dynamic_component_ref(el_id)?;
                (name, self.ctx.b.thunk(component_ref))
            };

        let inner_call = self.ctx.b.call_expr(
            intermediate_ref,
            [Arg::Ident("$$anchor"), Arg::Expr(props_expr)],
        );
        let (inner_final, inner_validate) = if let Some(bind_id) = bind_this_info {
            self.build_bind_this_call(el_id, bind_id, inner_call)?
        } else {
            (inner_call, None)
        };
        let mut inner_body: Vec<Statement<'a>> =
            self.build_ownership_binding_stmts(&ownership_bindings, intermediate_ref);
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

        if let Some(stmt) = inner_validate {
            state.init.push(stmt);
        }
        if snippet_decls.is_empty() && memo_decls.is_empty() {
            state.init.push(component_stmt);
        } else {
            let mut block = snippet_decls;
            block.extend(memo_decls);
            block.push(component_stmt);
            state.init.push(self.ctx.b.block_stmt(block));
        }
        Ok(anchor_node)
    }
}
