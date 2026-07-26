use std::mem;
use svelte_emit_builders::runes::rune_get;

use oxc_ast::ast::{Expression, Statement};
use svelte_analyze::{AttributeSemantics, NamespaceKind, SkipCause, SvelteElementTag};
use svelte_ast::{Attribute, Node, NodeId};
use svelte_ast_builder::Arg;

use super::super::attributes::AttributeOwnerKind;
use super::super::data_structures::AsyncEmission;
use super::super::data_structures::EmitState;
use super::super::data_structures::{FragmentAnchor, FragmentCtx};
use super::super::effect::suspending_block_thunk;
use super::super::{Codegen, CodegenError, Result};

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    fn svelte_element_tag_expr(&mut self, el_id: NodeId) -> Result<Expression<'a>> {
        match self.ctx.query.analysis.svelte_element_tag(el_id).cloned() {
            Some(SvelteElementTag::Known(name)) => Ok(self.ctx.b.str_expr(&name)),
            Some(SvelteElementTag::Dynamic(oxc_id)) => {
                match self.ctx.state.parsed.take_expr(oxc_id) {
                    Some(expr) => Ok(expr),
                    None => CodegenError::missing_expression(el_id),
                }
            }
            None => CodegenError::missing_expression(el_id),
        }
    }

    fn svelte_element_tag_read(
        &self,
        plan: &AsyncEmission,
        tag_expr: &Expression<'a>,
    ) -> Expression<'a> {
        match plan {
            AsyncEmission::Awaited { .. } | AsyncEmission::Deferred { .. } => {
                rune_get(&self.ctx.b, "$$tag")
            }
            AsyncEmission::Sync => {
                use oxc_allocator::CloneIn;
                tag_expr.clone_in(self.ctx.b.ast.allocator)
            }
        }
    }

    pub(in crate::codegen) fn emit_svelte_element(
        &mut self,
        state: &mut EmitState<'a>,
        ctx: &FragmentCtx<'a>,
        el_id: NodeId,
        _existing_var: Option<&str>,
    ) -> Result<String> {
        let el = self.ctx.query.svelte_element(el_id);
        let attributes: Vec<Attribute> = el
            .attributes
            .iter()
            .filter(|a| {
                !matches!(
                    self.ctx.query.analysis.attributes.get(a.id()),
                    AttributeSemantics::Skip(SkipCause::TagCarrier)
                )
            })
            .cloned()
            .collect();

        let tag_async_plan = AsyncEmission::for_node(self.ctx, el_id);

        let anchor_node = self.comment_anchor_node_name(state, ctx)?;

        let tag_expr = self.svelte_element_tag_expr(el_id)?;

        let el_span_start = el.span.start;
        let dev_loc: Option<(usize, usize)> = if self.ctx.state.dev {
            Some(self.ctx.state.line_index.line_col(el_span_start))
        } else {
            None
        };

        let tag_async_thunk: Option<Expression<'a>> = match &tag_async_plan {
            AsyncEmission::Awaited { .. } => {
                use oxc_allocator::CloneIn;
                let cloned = tag_expr.clone_in(self.ctx.b.ast.allocator);
                let suspension = self.ctx.expression_suspension(el_id);
                Some(suspending_block_thunk(self.ctx, cloned, suspension))
            }
            AsyncEmission::Deferred { .. } | AsyncEmission::Sync => None,
        };

        let mut dev_stmts: Vec<Statement<'a>> = Vec::new();
        if self.ctx.state.dev {
            let tag_read = self.svelte_element_tag_read(&tag_async_plan, &tag_expr);
            let validate_thunk = self.ctx.b.thunk(tag_read);
            dev_stmts.push(self.ctx.b.call_stmt(
                "$.validate_dynamic_element_tag",
                [Arg::Expr(validate_thunk)],
            ));
        }

        let get_tag = self
            .ctx
            .b
            .thunk(self.svelte_element_tag_read(&tag_async_plan, &tag_expr));

        let is_svg_or_mathml = matches!(
            self.ctx.query.view.namespace(el_id),
            Some(NamespaceKind::Svg | NamespaceKind::MathMl)
        );
        let is_svg_expr = self.ctx.b.bool_expr(is_svg_or_mathml);

        let el_name = self.ctx.state.gen_ident("$$element");

        let svelte_el_fragment = match self.ctx.query.component.store.get(el_id) {
            Node::SvelteElement(el) => el.fragment,
            _ => return CodegenError::unexpected_node(el_id, "SvelteElement"),
        };
        let child_ns = self
            .ctx
            .query
            .view
            .namespace(el_id)
            .unwrap_or(NamespaceKind::Html)
            .as_namespace();
        let inner_ctx = ctx.child_of_element(
            self.ctx,
            "",
            svelte_el_fragment,
            child_ns,
            FragmentAnchor::callback_param("$$anchor", false),
        );
        let mut inner_state = EmitState::new();

        let sole_static_class = if attributes.len() == 1 {
            if let Attribute::StringAttribute(sa) = &attributes[0] {
                if sa.name.eq_ignore_ascii_case("class") {
                    Some(sa.value(&self.ctx.query.component.source).to_string())
                } else {
                    None
                }
            } else {
                None
            }
        } else if attributes.is_empty()
            && self.ctx.is_css_scoped(el_id)
            && !self.ctx.css_hash().is_empty()
        {
            Some(String::new())
        } else {
            None
        };

        let ns_thunk: Option<Expression<'a>> = if let Some(class_value) = sole_static_class {
            let hash = self.ctx.css_hash().to_string();
            let scoped_class = if self.ctx.is_css_scoped(el_id) && !hash.is_empty() {
                if class_value.is_empty() {
                    hash
                } else {
                    format!("{class_value} {hash}")
                }
            } else {
                class_value
            };
            inner_state.init.push(self.ctx.b.call_stmt(
                "$.set_class",
                [Arg::Ident(&el_name), Arg::Num(0.0), Arg::Str(scoped_class)],
            ));
            None
        } else {
            self.emit_dom_attributes_with_kind(
                &mut inner_state,
                el_id,
                "",
                &el_name,
                &attributes,
                AttributeOwnerKind::SvelteElement,
                false,
            )?
        };

        inner_state
            .init
            .extend(mem::take(&mut inner_state.pending_element_init));
        inner_state
            .init
            .extend(mem::take(&mut inner_state.pending_pre_update));

        if !inner_state.update.is_empty() {
            let attr_updates = mem::take(&mut inner_state.update);
            let attr_memo = mem::take(&mut inner_state.shared_memo);
            let attr_blockers = mem::take(&mut inner_state.script_blockers);
            let attr_extra = mem::take(&mut inner_state.extra_blockers);
            super::super::effect::emit_template_effect_with_memo(
                self.ctx,
                &mut inner_state.init,
                attr_updates,
                attr_memo,
                attr_blockers,
                attr_extra,
            )?;
        }

        self.emit_fragment(&mut inner_state, &inner_ctx, svelte_el_fragment)?;
        let inner_body: Vec<Statement<'a>> = self.pack_callback_body(inner_state, "$$anchor")?;

        let callback = if inner_body.is_empty() {
            None
        } else {
            Some(self.ctx.b.arrow_block_expr(
                self.ctx.b.params([el_name.as_str(), "$$anchor"]),
                inner_body,
            ))
        };

        let has_child_nodes = self
            .ctx
            .query
            .analysis
            .fragment_has_children_by_id(svelte_el_fragment);
        if has_child_nodes && self.ctx.state.dev {
            let tag_read = self.svelte_element_tag_read(&tag_async_plan, &tag_expr);
            let void_thunk = self.ctx.b.thunk(tag_read);
            dev_stmts.push(
                self.ctx
                    .b
                    .call_stmt("$.validate_void_dynamic_element", [Arg::Expr(void_thunk)]),
            );
        }

        let needs_loc = dev_loc.is_some();
        let needs_ns = ns_thunk.is_some() || needs_loc;
        let needs_cb = callback.is_some() || needs_ns;

        let element_anchor_ident = match &tag_async_plan {
            AsyncEmission::Awaited { .. } | AsyncEmission::Deferred { .. } => "node",
            AsyncEmission::Sync => anchor_node.as_str(),
        };
        let mut args: Vec<Arg<'a, '_>> = vec![
            Arg::Ident(element_anchor_ident),
            Arg::Expr(get_tag),
            Arg::Expr(is_svg_expr),
        ];
        if needs_cb {
            match callback {
                Some(cb) => args.push(Arg::Expr(cb)),
                None => args.push(Arg::Expr(self.ctx.b.void_zero_expr())),
            }
        }
        if needs_ns {
            match ns_thunk {
                Some(thunk) => args.push(Arg::Expr(thunk)),
                None => args.push(Arg::Expr(self.ctx.b.void_zero_expr())),
            }
        }
        if let Some((line, col)) = dev_loc {
            let loc = self.ctx.b.array_expr([
                self.ctx.b.num_expr(line as f64),
                self.ctx.b.num_expr(col as f64),
            ]);
            args.push(Arg::Expr(loc));
        }

        let element_stmt = self.ctx.b.call_stmt("$.element", args);
        let final_stmt = match &tag_async_plan {
            AsyncEmission::Awaited { blockers } | AsyncEmission::Deferred { blockers } => {
                let blockers = blockers.to_vec();
                let anchor_expr = self.ctx.b.rid_expr(&anchor_node);
                let inner_stmts = if dev_stmts.is_empty() {
                    vec![element_stmt]
                } else {
                    dev_stmts.push(element_stmt);
                    dev_stmts
                };
                self.emit_async_call_stmt(
                    &blockers,
                    anchor_expr,
                    "node",
                    "$$tag",
                    tag_async_thunk,
                    inner_stmts,
                )?
            }
            AsyncEmission::Sync => {
                if dev_stmts.is_empty() {
                    element_stmt
                } else {
                    dev_stmts.push(element_stmt);
                    self.ctx.b.block_stmt(dev_stmts)
                }
            }
        };
        state.init.push(final_stmt);
        Ok(anchor_node)
    }
}
