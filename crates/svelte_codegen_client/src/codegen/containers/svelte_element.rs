use oxc_ast::ast::{Expression, Statement};
use svelte_analyze::{FragmentKey, NamespaceKind};
use svelte_ast::{Attribute, NodeId};
use svelte_ast_builder::Arg;

use super::super::attributes::AttributeOwnerKind;
use super::super::data_structures::EmitState;
use super::super::data_structures::{FragmentAnchor, FragmentCtx};
use super::super::{Codegen, Result};

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(in crate::codegen) fn emit_svelte_element(
        &mut self,
        state: &mut EmitState<'a>,
        ctx: &FragmentCtx<'a>,
        el_id: NodeId,
        _existing_var: Option<&str>,
    ) -> Result<String> {
        let el = self.ctx.query.svelte_element(el_id);
        let static_tag = el.static_tag;
        let tag_span = el.tag_span;
        let attributes = el.attributes.clone();

        let tag_async_plan =
            super::super::data_structures::AsyncEmissionPlan::for_node(self.ctx, el_id);
        let needs_async_tag = tag_async_plan.needs_async();

        let anchor_node = self.comment_anchor_node_name(state, ctx)?;

        let tag_expr = if static_tag {
            let value = self.ctx.query.component.source_text(tag_span);
            self.ctx.b.str_expr(value)
        } else {
            self.take_node_expr(el_id)?
        };

        let el_span_start = el.span.start;
        let dev_loc: Option<(usize, usize)> = if self.ctx.state.dev {
            Some(crate::script::compute_line_col(
                self.ctx.state.source,
                el_span_start,
            ))
        } else {
            None
        };

        let mut dev_stmts: Vec<Statement<'a>> = Vec::new();
        let dev_void_tag_clone: Option<Expression<'a>> = if self.ctx.state.dev {
            use oxc_allocator::CloneIn;
            let validate_tag = tag_expr.clone_in(self.ctx.b.ast.allocator);
            let validate_thunk = self.ctx.b.thunk(validate_tag);
            dev_stmts.push(self.ctx.b.call_stmt(
                "$.validate_dynamic_element_tag",
                [Arg::Expr(validate_thunk)],
            ));
            Some(tag_expr.clone_in(self.ctx.b.ast.allocator))
        } else {
            None
        };

        let tag_async_thunk: Option<Expression<'a>> = if needs_async_tag {
            use oxc_allocator::CloneIn;
            let cloned = tag_expr.clone_in(self.ctx.b.ast.allocator);
            tag_async_plan.async_thunk(self.ctx, cloned)
        } else {
            None
        };

        let get_tag = if needs_async_tag {
            self.ctx
                .b
                .thunk(self.ctx.b.call_expr("$.get", [Arg::Ident("$$tag")]))
        } else {
            self.ctx.b.thunk(tag_expr)
        };

        let is_svg_or_mathml = matches!(
            self.ctx.query.view.namespace(el_id),
            Some(NamespaceKind::Svg | NamespaceKind::MathMl)
        );
        let is_svg_expr = self.ctx.b.bool_expr(is_svg_or_mathml);

        let el_name = self.ctx.state.gen_ident("$$element");

        let body_key = FragmentKey::SvelteElementBody(el_id);
        let child_ns = self
            .ctx
            .query
            .view
            .namespace(el_id)
            .unwrap_or(NamespaceKind::Html)
            .as_namespace();
        let inner_ctx = ctx.child_of_element(
            "",
            body_key,
            child_ns,
            FragmentAnchor::CallbackParam {
                name: "$$anchor".to_string(),
                append_inside: true,
            },
        );
        let mut inner_state = EmitState::new();

        let sole_static_class = if attributes.len() == 1 {
            if let Attribute::StringAttribute(sa) = &attributes[0] {
                if sa.name.eq_ignore_ascii_case("class") {
                    Some(
                        self.ctx
                            .query
                            .component
                            .source_text(sa.value_span)
                            .to_string(),
                    )
                } else {
                    None
                }
            } else {
                None
            }
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
            )?
        };

        self.emit_fragment(&mut inner_state, &inner_ctx, body_key)?;
        let inner_body: Vec<Statement<'a>> = self.pack_callback_body(inner_state, "$$anchor")?;

        let callback = if inner_body.is_empty() {
            None
        } else {
            Some(self.ctx.b.arrow_block_expr(
                self.ctx.b.params([el_name.as_str(), "$$anchor"]),
                inner_body,
            ))
        };

        if callback.is_some() {
            if let Some(void_tag) = dev_void_tag_clone {
                let void_thunk = self.ctx.b.thunk(void_tag);
                dev_stmts.push(
                    self.ctx
                        .b
                        .call_stmt("$.validate_void_dynamic_element", [Arg::Expr(void_thunk)]),
                );
            }
        }

        let needs_loc = dev_loc.is_some();
        let needs_ns = ns_thunk.is_some() || needs_loc;
        let needs_cb = callback.is_some() || needs_ns;

        let element_anchor_ident = if needs_async_tag {
            "node"
        } else {
            anchor_node.as_str()
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
        let final_stmt = if needs_async_tag {
            let anchor_expr = self.ctx.b.rid_expr(&anchor_node);
            let inner_stmts = if dev_stmts.is_empty() {
                vec![element_stmt]
            } else {
                dev_stmts.push(element_stmt);
                dev_stmts
            };
            tag_async_plan.emit_async_call_stmt(
                self.ctx,
                anchor_expr,
                "node",
                "$$tag",
                tag_async_thunk,
                inner_stmts,
            )
        } else if dev_stmts.is_empty() {
            element_stmt
        } else {
            dev_stmts.push(element_stmt);
            self.ctx.b.block_stmt(dev_stmts)
        };
        state.init.push(final_stmt);
        Ok(anchor_node)
    }
}
