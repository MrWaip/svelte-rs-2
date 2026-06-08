use crate::codegen::expr::coarse_wrap;
use oxc_ast::ast::{Expression, Statement};
use svelte_analyze::Volatility;
use svelte_ast::{Attribute, Node, NodeId};
use svelte_ast_builder::{Arg, ObjProp};
use svelte_emit_builders::runes::rune_get;

use super::super::data_structures::EmitState;
use super::super::data_structures::{FragmentAnchor, FragmentCtx};
use super::super::{Codegen, CodegenError, FragmentEmitKind, Result};

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(in crate::codegen) fn emit_legacy_slot_like(
        &mut self,
        state: &mut EmitState<'a>,
        ctx: &FragmentCtx<'a>,
        el_id: NodeId,
        _existing_var: Option<&str>,
    ) -> Result<String> {
        let node = self.ctx.query.component.store.get(el_id);
        match node {
            Node::SlotElementLegacy(_) => self.emit_slot_element_legacy(state, ctx, el_id),
            Node::SvelteFragmentLegacy(_) => {
                CodegenError::not_implemented(el_id, "<svelte:fragment> (legacy slot wrapper)")
            }
            _ => CodegenError::unexpected_node(el_id, "SlotElementLegacy/SvelteFragmentLegacy"),
        }
    }

    fn emit_slot_element_legacy(
        &mut self,
        state: &mut EmitState<'a>,
        ctx: &FragmentCtx<'a>,
        el_id: NodeId,
    ) -> Result<String> {
        let attrs = match self.ctx.query.component.store.get(el_id) {
            Node::SlotElementLegacy(el) => el.attributes.clone(),
            _ => return CodegenError::unexpected_node(el_id, "SlotElementLegacy"),
        };

        let slot_name = self.legacy_slot_name(&attrs);
        let slot_name_alloc: &str = self.ctx.b.alloc_str(&slot_name);

        let mut props: Vec<ObjProp<'a>> = Vec::new();
        let mut spreads: Vec<Expression<'a>> = Vec::new();
        let mut memo_stmts: Vec<Statement<'a>> = Vec::new();
        let derived_fn = self.ctx.query.view.derived_helper();

        for attr in &attrs {
            let attr_id = attr.id();
            match attr {
                Attribute::StringAttribute(a) => {
                    if a.name == "name" || a.name == "slot" {
                        continue;
                    }
                    let key = self.ctx.b.alloc_str(&a.name);
                    let value = self
                        .ctx
                        .b
                        .str_expr(a.value(&self.ctx.query.component.source));
                    props.push(ObjProp::KeyValue(key, value));
                }
                Attribute::BooleanAttribute(a) => {
                    if a.name == "name" || a.name == "slot" {
                        continue;
                    }
                    let key = self.ctx.b.alloc_str(&a.name);
                    props.push(ObjProp::KeyValue(key, self.ctx.b.bool_expr(true)));
                }
                Attribute::SpreadAttribute(sa) => {
                    let value = self.take_attr_expr(attr_id, &sa.expression)?;
                    spreads.push(self.ctx.b.thunk(value));
                }
                Attribute::ExpressionAttribute(a) => {
                    if a.name == "name" || a.name == "slot" {
                        continue;
                    }
                    let key = self.ctx.b.alloc_str(&a.name);
                    let expr = self.take_attr_expr(attr_id, &a.expression)?;
                    let (memo, shorthand) = match self.ctx.query.analysis.attributes.get(attr_id) {
                        svelte_analyze::AttributeSemantics::ComponentProp(
                            svelte_analyze::ComponentPropSemantics::Expression(e),
                        ) => (e.memo, e.shorthand),
                        _ => (svelte_analyze::ComponentPropMemo::Inline, false),
                    };
                    let data = self.ctx.expression_data(attr_id).cloned();
                    match memo {
                        svelte_analyze::ComponentPropMemo::Derived => {
                            let name = format!("${}", memo_stmts.len());
                            let name_ref = self.ctx.b.alloc_str(&name);
                            let wrapped = coarse_wrap(self.ctx, expr, data.as_ref());
                            let derived = self
                                .ctx
                                .b
                                .call_expr(derived_fn, [Arg::Expr(self.ctx.b.thunk(wrapped))]);
                            memo_stmts.push(self.ctx.b.let_init_stmt(name_ref, derived));
                            let get_call = rune_get(&self.ctx.b, name_ref);
                            props.push(ObjProp::Getter(key, get_call));
                        }
                        svelte_analyze::ComponentPropMemo::Getter => {
                            let wrapped = coarse_wrap(self.ctx, expr, data.as_ref());
                            props.push(ObjProp::Getter(key, wrapped));
                        }
                        svelte_analyze::ComponentPropMemo::Inline if shorthand => {
                            props.push(ObjProp::Shorthand(key));
                        }
                        svelte_analyze::ComponentPropMemo::Inline => {
                            props.push(ObjProp::KeyValue(key, expr));
                        }
                    }
                }
                Attribute::ConcatenationAttribute(a) => {
                    if a.name == "name" || a.name == "slot" {
                        continue;
                    }
                    let key = self.ctx.b.alloc_str(&a.name);
                    let val = self.build_concat_expr_collapse_single(attr_id, &a.parts)?;
                    match self.ctx.expression_data(attr_id).map(|d| d.volatility) {
                        Some(
                            Volatility::Reactive | Volatility::Heavy | Volatility::Asynchronous,
                        ) => {
                            props.push(ObjProp::Getter(key, val));
                        }
                        Some(Volatility::Static) | None => {
                            props.push(ObjProp::KeyValue(key, val));
                        }
                    }
                }
                _ => {}
            }
        }

        let props_expr = if spreads.is_empty() {
            self.ctx.b.object_expr(props)
        } else {
            let mut args: Vec<Arg<'a, '_>> = Vec::with_capacity(spreads.len() + 1);
            args.push(Arg::Expr(self.ctx.b.object_expr(props)));
            for s in spreads {
                args.push(Arg::Expr(s));
            }
            self.ctx.b.call_expr("$.spread_props", args)
        };

        let anchor_node = self.comment_anchor_node_name(state, ctx)?;

        let fallback = self.build_legacy_slot_fallback(ctx, el_id)?;

        let let_stmts = self.emit_let_directive_legacy_stmts(el_id)?;

        let slot_stmt = self.ctx.b.call_stmt(
            "$.slot",
            [
                Arg::Ident(&anchor_node),
                Arg::Ident("$$props"),
                Arg::StrRef(slot_name_alloc),
                Arg::Expr(props_expr),
                Arg::Expr(fallback),
            ],
        );
        for stmt in let_stmts {
            state.init.push(stmt);
        }
        if memo_stmts.is_empty() {
            state.init.push(slot_stmt);
        } else {
            memo_stmts.push(slot_stmt);
            state.init.push(self.ctx.b.block_stmt(memo_stmts));
        }

        if matches!(ctx.anchor, FragmentAnchor::Child { .. }) {
            state.last_fragment_needs_reset = true;
        }

        Ok(anchor_node)
    }

    fn legacy_slot_name(&self, attrs: &[Attribute]) -> String {
        for attr in attrs {
            if let Attribute::StringAttribute(sa) = attr
                && sa.name == "name"
            {
                return sa.value(&self.ctx.query.component.source).to_string();
            }
        }
        "default".to_string()
    }

    fn build_legacy_slot_fallback(
        &mut self,
        parent_ctx: &FragmentCtx<'a>,
        el_id: NodeId,
    ) -> Result<Expression<'a>> {
        let el_fragment = match self.ctx.query.component.store.get(el_id) {
            svelte_ast::Node::Element(el) => el.fragment,
            svelte_ast::Node::SlotElementLegacy(el) => el.fragment,
            svelte_ast::Node::SvelteFragmentLegacy(el) => el.fragment,
            _ => return Ok(self.ctx.b.null_expr()),
        };
        let inner_ctx = parent_ctx.child_of_block(
            self.ctx,
            el_fragment,
            FragmentAnchor::CallbackParam {
                name: "$$anchor".to_string(),
                append_inside: false,
            },
        );
        let mut inner_state = EmitState::new();
        match self.emit_fragment(&mut inner_state, &inner_ctx, el_fragment)? {
            FragmentEmitKind::Empty => return Ok(self.ctx.b.null_expr()),
            FragmentEmitKind::Rendered => {}
        }
        let body: Vec<Statement<'a>> = self.pack_callback_body(inner_state, "$$anchor")?;
        Ok(self
            .ctx
            .b
            .arrow_block_expr(self.ctx.b.params(["$$anchor"]), body))
    }
}
