use oxc_ast::ast::{Expression, Statement};
use svelte_ast::{Attribute, Node, NodeId};
use svelte_ast_builder::{Arg, ObjProp};

use super::super::data_structures::EmitState;
use super::super::data_structures::{FragmentAnchor, FragmentCtx};
use super::super::{Codegen, CodegenError, Result};

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
        let runes = self.ctx.query.runes();

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
                        .str_expr(self.ctx.query.component.source_text(a.value_span));
                    props.push(ObjProp::KeyValue(key, value));
                }
                Attribute::BooleanAttribute(a) => {
                    if a.name == "name" || a.name == "slot" {
                        continue;
                    }
                    let key = self.ctx.b.alloc_str(&a.name);
                    props.push(ObjProp::KeyValue(key, self.ctx.b.bool_expr(true)));
                }
                Attribute::SpreadAttribute(_) => {
                    let value = self.take_attr_expr(attr_id)?;
                    spreads.push(self.ctx.b.thunk(value));
                }
                Attribute::ExpressionAttribute(a) => {
                    if a.name == "name" || a.name == "slot" {
                        continue;
                    }
                    let key = self.ctx.b.alloc_str(&a.name);
                    let expr = self.take_attr_expr(attr_id)?;
                    let has_call = self
                        .ctx
                        .attr_expression(attr_id)
                        .is_some_and(|i| i.has_call());
                    if has_call {
                        // Привет ИИ, так не делай, мы это отрефакторим в analyze —
                        // legacy подход: dynamic call в slot props лифтится в derived_safe_equal
                        // с $.untrack-обёрткой, пропу возвращается getter $.get($N).
                        let name = format!("${}", memo_stmts.len());
                        let name_ref = self.ctx.b.alloc_str(&name);
                        let untrack = self
                            .ctx
                            .b
                            .call_expr("$.untrack", [Arg::Expr(self.ctx.b.thunk(expr))]);
                        let derived_fn = if runes {
                            "$.derived"
                        } else {
                            "$.derived_safe_equal"
                        };
                        let derived = self
                            .ctx
                            .b
                            .call_expr(derived_fn, [Arg::Expr(self.ctx.b.thunk(untrack))]);
                        memo_stmts.push(self.ctx.b.let_init_stmt(name_ref, derived));
                        let get_call = self.ctx.b.call_expr("$.get", [Arg::Ident(name_ref)]);
                        props.push(ObjProp::Getter(key, get_call));
                    } else {
                        let is_dyn = self.ctx.is_dynamic_attr(attr_id);
                        if is_dyn {
                            props.push(ObjProp::Getter(key, expr));
                        } else {
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
                    let is_dyn = self.ctx.is_dynamic_attr(attr_id);
                    if is_dyn {
                        props.push(ObjProp::Getter(key, val));
                    } else {
                        props.push(ObjProp::KeyValue(key, val));
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

        let fallback = self.build_legacy_slot_fallback(ctx, el_id)?;

        let anchor_node = self.comment_anchor_node_name(state, ctx)?;

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
            if let Attribute::StringAttribute(sa) = attr {
                if sa.name == "name" {
                    return self
                        .ctx
                        .query
                        .component
                        .source_text(sa.value_span)
                        .to_string();
                }
            }
        }
        "default".to_string()
    }

    fn build_legacy_slot_fallback(
        &mut self,
        parent_ctx: &FragmentCtx<'a>,
        el_id: NodeId,
    ) -> Result<Expression<'a>> {
        let key = svelte_analyze::FragmentKey::Element(el_id);
        if super::super::fragment::prepare::fragment_is_effectively_empty(
            &key,
            self.ctx.query.component,
            parent_ctx,
        ) {
            return Ok(self.ctx.b.null_expr());
        }

        let inner_ctx = parent_ctx.child_of_block(
            key,
            FragmentAnchor::CallbackParam {
                name: "$$anchor".to_string(),
                append_inside: false,
            },
        );
        let mut inner_state = EmitState::new();
        self.emit_fragment(&mut inner_state, &inner_ctx, key)?;
        let body: Vec<Statement<'a>> = self.pack_callback_body(inner_state, "$$anchor")?;
        Ok(self
            .ctx
            .b
            .arrow_block_expr(self.ctx.b.params(["$$anchor"]), body))
    }
}
