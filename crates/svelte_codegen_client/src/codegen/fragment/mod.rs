use svelte_emit_builders::runes::rune_get;
use crate::codegen::expr::coarse_wrap;
mod legacy_slot_fragment;
mod prepare;
mod process_children;
mod types;

use oxc_ast::ast::{Expression, Statement};
use std::iter::empty;
use svelte_analyze::{ComponentCssProp, ComponentCssPropValue, ComponentPropMemo};
use svelte_ast::{FragmentRole, NodeId};
use svelte_ast_builder::{Arg, ObjProp, TemplatePart};

use crate::codegen::concatenation::ConcatenationAnchor;
use crate::codegen::fragment::prepare::prepare;
use crate::codegen::fragment::types::{Child, ContentStrategy, HoistedBucket, StrategyKind};
use smallvec::SmallVec;

enum HoistedDispatchKind {
    Snippet,
    SvelteHead,
    SvelteWindow,
    SvelteDocument,
    SvelteBody,
}
use crate::codegen::CodegenError;

pub(in crate::codegen) use legacy_slot_fragment::SlotFragmentOutcome;

use super::data_structures::EmitState;
use super::data_structures::{ConcatPart, FragmentAnchor, FragmentCtx};
use super::{Codegen, Result};

pub(crate) enum FragmentEmitKind {
    Empty,
    Rendered,
}

fn single_fragment_anchor<'a>(
    ctx: &FragmentCtx<'a>,
) -> Result<ConcatenationAnchor> {
    match &ctx.anchor {
        FragmentAnchor::Root => Ok(ConcatenationAnchor::SingleFragmentRoot),
        FragmentAnchor::CallbackParam { append_inside, .. } => Ok(
            ConcatenationAnchor::SingleFragmentCallbackParam {
                append_inside: *append_inside,
            },
        ),
        FragmentAnchor::Child { parent_var } => Ok(ConcatenationAnchor::SingleFragmentChild {
            parent_var: parent_var.clone(),
        }),
        FragmentAnchor::SiblingVar { .. } => {
            CodegenError::unexpected_child("Single", "SiblingVar anchor")
        }
    }
}

pub(in crate::codegen) fn role_needs_text_first_next(role: FragmentRole) -> bool {
    matches!(
        role,
        FragmentRole::Root
            | FragmentRole::EachBody
            | FragmentRole::EachFallback
            | FragmentRole::SnippetBody
            | FragmentRole::ComponentChildren
            | FragmentRole::SvelteBoundaryBody
    )
}

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(crate) fn emit_fragment(
        &mut self,
        state: &mut EmitState<'a>,
        ctx: &FragmentCtx<'a>,
        fragment_id: svelte_ast::FragmentId,
    ) -> Result<FragmentEmitKind> {
        let mut bucket = HoistedBucket::default();
        let fragment_nodes: Vec<NodeId> = self
            .ctx
            .query
            .component
            .store
            .fragment(fragment_id)
            .nodes
            .clone();
        let (children, raw_strategy) = prepare(
            &fragment_nodes,
            &self.ctx.query.component.store,
            ctx,
            &mut bucket,
        );
        let strategy = self.refine_strategy(raw_strategy, ctx);

        let bucket_effectively_empty = if state.skip_snippets {
            bucket.is_empty_ignoring_snippets()
        } else {
            bucket.is_empty()
        };
        if matches!(strategy, ContentStrategy::Empty) && bucket_effectively_empty {
            state.last_fragment_needs_reset = false;
            return Ok(FragmentEmitKind::Empty);
        }

        let fragment_blockers = self.ctx.query.view.fragment_blockers_by_id(fragment_id);
        if !fragment_blockers.is_empty() {
            state.script_blockers.extend_from_slice(fragment_blockers);
        }

        let is_root_anchor = matches!(
            ctx.anchor,
            FragmentAnchor::Root | FragmentAnchor::CallbackParam { .. }
        );
        let template_was_empty_before = state.template.is_empty();

        let will_css_wrap = matches!(strategy, ContentStrategy::CssWrappedComponent(_));
        let reserved_tpl_name = if is_root_anchor && !will_css_wrap {
            Some(self.ctx.state.gen_ident("root"))
        } else {
            None
        };

        let strategy_kind = StrategyKind::of(&strategy);

        let mut needs_reset = !matches!(
            strategy,
            ContentStrategy::Empty
                | ContentStrategy::SingleStatic
                | ContentStrategy::SingleElement(_)
                | ContentStrategy::CssWrappedComponent(_)
        );

        {
            use svelte_analyze::{BlockSemantics, ConstTagAsyncKind};
            let has_async = self.ctx.state.experimental_async
                && bucket.const_tags.iter().any(|&id| {
                    matches!(
                        self.ctx.query.analysis.block_semantics(id),
                        BlockSemantics::ConstTag(s)
                            if matches!(s.async_kind, ConstTagAsyncKind::Async { .. })
                    )
                });
            if has_async {
                self.emit_const_tags_async_batch(state, &bucket.const_tags)?;
            } else {
                for &id in &bucket.const_tags {
                    self.emit_hoisted_const_tag(state, ctx, id)?;
                }
            }
            for &id in &bucket.debug_tags {
                self.emit_hoisted_debug_tag(state, ctx, id)?;
            }
        }

        let multi_first_is_block = matches!(
            &strategy,
            ContentStrategy::Multi {
                first_is_block: true,
                ..
            }
        );
        let needs_anchor_reserve = is_root_anchor
            && match &strategy {
                ContentStrategy::SingleBlock(id) => !self.render_tag_uses_direct_anchor(*id),
                ContentStrategy::Multi { .. } => multi_first_is_block,
                ContentStrategy::SingleElement(id) => !matches!(
                    self.ctx.query.component.store.get(*id),
                    svelte_ast::Node::Element(_)
                ),
                _ => false,
            };
        let skip_node_reserve = needs_anchor_reserve
            && matches!(&strategy, ContentStrategy::SingleBlock(id) | ContentStrategy::SingleElement(id)
            if {
                let n = self.ctx.query.component.store.get(*id);
                if let svelte_ast::Node::ComponentNode(cn) = n {
                    !self.ctx.is_dynamic_component(*id)
                        && !self.ctx.has_component_css_props(*id)
                        && !cn.attributes.iter().any(|a| match a {
                            svelte_ast::Attribute::StringAttribute(attr) => attr.name.starts_with("--"),
                            svelte_ast::Attribute::ExpressionAttribute(attr) => attr.name.starts_with("--"),
                            svelte_ast::Attribute::ConcatenationAttribute(attr) => attr.name.starts_with("--"),
                            svelte_ast::Attribute::BooleanAttribute(attr) => attr.name.starts_with("--"),
                            _ => false,
                        })
                } else {
                    false
                }
            });
        let starts_text_for_next = matches!(
            &strategy,
            ContentStrategy::SingleStatic
                | ContentStrategy::SingleExpr(_)
                | ContentStrategy::SingleConcat
                | ContentStrategy::Multi {
                    first_is_text_like: true,
                    ..
                }
        );
        let multi_or_root_callback = matches!(&strategy, ContentStrategy::Multi { .. })
            && matches!(
                ctx.anchor,
                FragmentAnchor::CallbackParam { .. } | FragmentAnchor::Root
            );
        let emitted_prefix_next =
            role_needs_text_first_next(self.ctx.query.component.store.fragment(fragment_id).role)
                && starts_text_for_next
                && multi_or_root_callback;
        if emitted_prefix_next {
            state.init.push(
                self.ctx
                    .state
                    .b
                    .call_stmt("$.next", empty::<Arg<'a, '_>>()),
            );
        }

        let init_len_before = state.init.len();
        let emit_snippets_here = is_root_anchor;
        let mut ordered: SmallVec<[(NodeId, HoistedDispatchKind); 4]> = SmallVec::new();
        if emit_snippets_here {
            for &id in &bucket.snippets {
                ordered.push((id, HoistedDispatchKind::Snippet));
            }
        }
        for &id in &bucket.svelte_head {
            ordered.push((id, HoistedDispatchKind::SvelteHead));
        }
        for &id in &bucket.svelte_window {
            ordered.push((id, HoistedDispatchKind::SvelteWindow));
        }
        for &id in &bucket.svelte_document {
            ordered.push((id, HoistedDispatchKind::SvelteDocument));
        }
        for &id in &bucket.svelte_body {
            ordered.push((id, HoistedDispatchKind::SvelteBody));
        }
        ordered.sort_by_key(|&(id, _)| id.0);
        let pre_emit_frag_pending = needs_anchor_reserve
            && !skip_node_reserve
            && !matches!(&strategy, ContentStrategy::Multi { .. })
            && (!bucket.svelte_head.is_empty()
                || !bucket.svelte_window.is_empty()
                || !bucket.svelte_document.is_empty()
                || !bucket.svelte_body.is_empty())
            && state.root_var.is_none()
            && !state.anchor_comment_pre_emitted;
        let pre_emit_insert_idx = pre_emit_frag_pending.then_some(state.init.len());
        for (id, kind) in ordered {
            match kind {
                HoistedDispatchKind::Snippet => {
                    if ctx.in_block_callback {
                        self.emit_inline_snippet_block(state, id)?;
                    } else {
                        self.emit_hoisted_snippet(state, ctx, id)?;
                    }
                }
                HoistedDispatchKind::SvelteHead => {
                    self.emit_hoisted_svelte_head(state, ctx, id)?;
                }
                HoistedDispatchKind::SvelteWindow => {
                    self.emit_hoisted_svelte_window(state, ctx, id)?;
                }
                HoistedDispatchKind::SvelteDocument => {
                    self.emit_hoisted_svelte_document(state, ctx, id)?;
                }
                HoistedDispatchKind::SvelteBody => {
                    self.emit_hoisted_svelte_body(state, ctx, id)?;
                }
            }
        }

        if let Some(insert_idx) = pre_emit_insert_idx {
            let frag = self.ctx.state.gen_ident("fragment");
            let node = self.ctx.state.gen_ident("node");
            let b = &self.ctx.state.b;
            let stmt = b.var_stmt(&frag, b.call_expr("$.comment", empty::<Arg<'a, '_>>()));
            state.init.insert(insert_idx, stmt);
            state.pending_anchor_idents = Some((frag.clone(), node));
            state.root_var = Some(frag);
            state.anchor_comment_pre_emitted = true;
        }

        if needs_anchor_reserve && !pre_emit_frag_pending {
            let frag = self.ctx.state.gen_ident("fragment");
            if skip_node_reserve {
                state.pending_anchor_idents = Some((frag, String::new()));
            } else {
                let node = self.ctx.state.gen_ident("node");
                state.pending_anchor_idents = Some((frag, node));
            }
        }

        if is_root_anchor
            && matches!(
                &strategy,
                ContentStrategy::SingleExpr(_) | ContentStrategy::SingleConcat
            )
        {
            let _ = self.ctx.state.gen_ident("fragment");
        }

        state.last_fragment_needs_reset = needs_reset;
        match strategy {
            ContentStrategy::Empty => {}
            ContentStrategy::SingleStatic => match children.first() {
                Some(Child::Text(part)) => self.emit_static_node(state, ctx, part)?,
                Some(Child::Comment(_)) => self.emit_static_comment_anchor(state, ctx)?,
                _ => {
                    return CodegenError::unexpected_child("Text", "non-Text for SingleStatic");
                }
            },
            ContentStrategy::SingleExpr(id) => {
                let anchor = single_fragment_anchor(ctx)?;
                self.emit_concatenation(
                    state,
                    ctx,
                    anchor,
                    &[ConcatPart::Expr(id)],
                )?;
                needs_reset = state.last_fragment_needs_reset;
            }
            ContentStrategy::SingleConcat => match children.first() {
                Some(Child::Concat(parts)) => {
                    let anchor = single_fragment_anchor(ctx)?;
                    self.emit_concatenation(state, ctx, anchor, parts)?;
                    needs_reset = state.last_fragment_needs_reset;
                }
                _ => {
                    return CodegenError::unexpected_child("Concat", "non-Concat for SingleConcat");
                }
            },
            ContentStrategy::SingleElement(id) => {
                let standalone = self.standalone_ctx_for_single(ctx, id);
                let use_ctx = standalone.as_ref().unwrap_or(ctx);
                self.emit_element_in_fragment(state, use_ctx, id)?;
                needs_reset = state.last_fragment_needs_reset;
            }
            ContentStrategy::SingleBlock(id) => {
                let standalone = self.standalone_ctx_for_single(ctx, id);
                let use_ctx = standalone.as_ref().unwrap_or(ctx);
                self.emit_fragment_child(state, use_ctx, id)?;
            }
            ContentStrategy::CssWrappedComponent(id) => {
                let inline_into_parent = matches!(ctx.anchor, FragmentAnchor::Child { .. });
                self.emit_component_with_css_wrapper(state, ctx, id)?;
                needs_reset = inline_into_parent;
            }
            ContentStrategy::ControlledEach(id) => {
                let FragmentAnchor::Child { parent_var } = &ctx.anchor else {
                    return CodegenError::unexpected_child(
                        "ControlledEach",
                        "non-Child anchor for controlled each",
                    );
                };
                let parent_name = parent_var.clone();
                let sem = match self.ctx.query.analysis.block_semantics(id) {
                    svelte_analyze::BlockSemantics::Each(s) => s.clone(),
                    _ => {
                        return CodegenError::unexpected_block_semantics(
                            id,
                            "EachBlock expected Each semantics",
                        );
                    }
                };
                self.emit_each_block_controlled(state, ctx, id, sem, parent_name)?;
                needs_reset = true;
            }
            ContentStrategy::Multi { .. } => {
                self.process_children_with_prefix(state, ctx, &children, emitted_prefix_next)?;
                needs_reset = state.last_fragment_needs_reset;
            }
        }
        state.last_fragment_needs_reset = needs_reset;

        if !is_root_anchor {
            for &id in &bucket.snippets {
                self.emit_local_snippet_block(state, id)?;
            }
        }
        for &id in &bucket.titles {
            self.emit_title_element(state, ctx, id)?;
        }

        if is_root_anchor
            && template_was_empty_before
            && !state.template.is_empty()
            && !state.suppress_root_finalize
        {
            let tpl_name = reserved_tpl_name
                .clone()
                .unwrap_or_else(|| self.ctx.state.gen_ident("root"));
            self.finalize_root_template(
                state,
                ctx,
                strategy_kind,
                init_len_before,
                tpl_name,
                fragment_id,
            )?;
        }

        Ok(FragmentEmitKind::Rendered)
    }

    fn wrap_add_locations(
        &self,
        from_html: Expression<'a>,
        locs: Expression<'a>,
    ) -> Expression<'a> {
        let b = &self.ctx.state.b;
        let filename_member = b.computed_member_expr(
            b.rid_expr(self.ctx.state.name),
            b.static_member_expr(b.rid_expr("$"), "FILENAME"),
        );
        b.call_expr(
            "$.add_locations",
            [
                Arg::Expr(from_html),
                Arg::Expr(filename_member),
                Arg::Expr(locs),
            ],
        )
    }

    fn build_slot_root_locations(
        &self,
        ctx: &FragmentCtx<'a>,
        slot_el_id: NodeId,
    ) -> Option<Expression<'a>> {
        let node = self.ctx.query.component.store.get(slot_el_id);
        let (span_start, fragment_id) = match node {
            svelte_ast::Node::Element(el) => (el.span.start, el.fragment),
            svelte_ast::Node::SlotElementLegacy(el) => (el.span.start, el.fragment),
            svelte_ast::Node::SvelteFragmentLegacy(el) => (el.span.start, el.fragment),
            _ => return None,
        };
        let single = self.build_single_element_loc(ctx, span_start, fragment_id);
        Some(self.ctx.b.array_expr(vec![single]))
    }

    fn build_template_locations(
        &self,
        ctx: &FragmentCtx<'a>,
        fragment_id: svelte_ast::FragmentId,
    ) -> Option<Expression<'a>> {
        let mut locs: Vec<Expression<'a>> = Vec::new();
        let nodes = self
            .ctx
            .query
            .component
            .store
            .fragment(fragment_id)
            .nodes
            .clone();
        for id in nodes {
            self.push_node_locations(ctx, id, &mut locs);
        }
        Some(self.ctx.b.array_expr(locs))
    }

    fn push_node_locations(
        &self,
        ctx: &FragmentCtx<'a>,
        node_id: NodeId,
        out: &mut Vec<Expression<'a>>,
    ) {
        let node = self.ctx.query.component.store.get(node_id);
        if self.is_hoisted_out_of_template(ctx, node) {
            return;
        }
        match node {
            svelte_ast::Node::Element(el) => {
                out.push(self.build_single_element_loc(ctx, el.span.start, el.fragment));
            }
            svelte_ast::Node::SvelteFragmentLegacy(el) => {
                let nodes = self
                    .ctx
                    .query
                    .component
                    .store
                    .fragment(el.fragment)
                    .nodes
                    .clone();
                for id in nodes {
                    self.push_node_locations(ctx, id, out);
                }
            }
            _ => {}
        }
    }

    fn is_hoisted_out_of_template(&self, ctx: &FragmentCtx<'a>, node: &svelte_ast::Node) -> bool {
        match node {
            svelte_ast::Node::Error(_)
            | svelte_ast::Node::SnippetBlock(_)
            | svelte_ast::Node::ConstTag(_)
            | svelte_ast::Node::DebugTag(_)
            | svelte_ast::Node::SvelteHead(_)
            | svelte_ast::Node::SvelteWindow(_)
            | svelte_ast::Node::SvelteDocument(_)
            | svelte_ast::Node::SvelteBody(_) => true,
            svelte_ast::Node::Element(el) if ctx.inside_head && el.name == "title" => true,
            _ => false,
        }
    }

    fn build_single_element_loc(
        &self,
        ctx: &FragmentCtx<'a>,
        span_start: u32,
        fragment_id: svelte_ast::FragmentId,
    ) -> Expression<'a> {
        let (line, col) = self.ctx.state.line_index.line_col(span_start);
        let b = &self.ctx.state.b;
        let mut inner: Vec<Expression<'a>> =
            vec![b.num_expr(line as f64), b.num_expr(col as f64)];
        let mut child_locs: Vec<Expression<'a>> = Vec::new();
        let nodes = self
            .ctx
            .query
            .component
            .store
            .fragment(fragment_id)
            .nodes
            .clone();
        for id in nodes {
            self.push_node_locations(ctx, id, &mut child_locs);
        }
        if !child_locs.is_empty() {
            inner.push(b.array_expr(child_locs));
        }
        b.array_expr(inner)
    }

    fn standalone_ctx_for_single(
        &self,
        ctx: &FragmentCtx<'a>,
        child_id: NodeId,
    ) -> Option<FragmentCtx<'a>> {
        let name = match &ctx.anchor {
            FragmentAnchor::CallbackParam {
                append_inside: false,
                name,
            } => name.clone(),
            _ => return None,
        };
        let node = self.ctx.query.component.store.get(child_id);
        let is_component_standalone = match node {
            svelte_ast::Node::ComponentNode(cn) => {
                !self.ctx.is_dynamic_component(child_id)
                    && !self.ctx.has_component_css_props(child_id)
                    && !cn.attributes.iter().any(|a| match a {
                        svelte_ast::Attribute::StringAttribute(attr) => attr.name.starts_with("--"),
                        svelte_ast::Attribute::ExpressionAttribute(attr) => {
                            attr.name.starts_with("--")
                        }
                        svelte_ast::Attribute::ConcatenationAttribute(attr) => {
                            attr.name.starts_with("--")
                        }
                        svelte_ast::Attribute::BooleanAttribute(attr) => {
                            attr.name.starts_with("--")
                        }
                        _ => false,
                    })
            }
            _ => false,
        };
        if !is_component_standalone {
            return None;
        }
        let mut new_ctx = ctx.clone();
        new_ctx.anchor = FragmentAnchor::CallbackParam {
            name,
            append_inside: true,
        };
        Some(new_ctx)
    }

    fn is_css_wrapped_component(&self, id: NodeId) -> bool {
        let node = self.ctx.query.component.store.get(id);
        match node {
            svelte_ast::Node::ComponentNode(_) => {
                !self.ctx.is_dynamic_component(id) && self.ctx.has_component_css_props(id)
            }
            svelte_ast::Node::SvelteComponentLegacy(_) => self.ctx.has_component_css_props(id),
            _ => false,
        }
    }

    fn refine_strategy(&self, strategy: ContentStrategy, ctx: &FragmentCtx<'a>) -> ContentStrategy {
        match &strategy {
            ContentStrategy::SingleElement(id) | ContentStrategy::SingleBlock(id) => {
                if self.is_css_wrapped_component(*id) {
                    return ContentStrategy::CssWrappedComponent(*id);
                }
            }
            _ => {}
        }
        if let ContentStrategy::SingleBlock(id) = &strategy
            && matches!(ctx.anchor, FragmentAnchor::Child { .. })
            && matches!(
                self.ctx.query.component.store.get(*id),
                svelte_ast::Node::EachBlock(_)
            )
        {
            return ContentStrategy::ControlledEach(*id);
        }
        strategy
    }

    fn emit_component_with_css_wrapper(
        &mut self,
        state: &mut EmitState<'a>,
        ctx: &FragmentCtx<'a>,
        component_id: NodeId,
    ) -> Result<()> {
        use svelte_ast::Namespace;
        let namespace = ctx.namespace;

        if let FragmentAnchor::Child { parent_var } = &ctx.anchor {
            let parent_var = parent_var.clone();
            if matches!(namespace, Namespace::Svg) {
                state.template.push_element("g", false);
            } else {
                state.template.push_element("svelte-css-wrapper", true);
                state
                    .template
                    .set_attribute("style", Some("display: contents".to_string()));
            }
            state.template.push_comment(None);
            state.template.pop_element();

            let node = self.ctx.state.gen_ident("node");
            state.init.push(self.ctx.b.var_stmt(
                &node,
                self.ctx.b.call_expr("$.child", [Arg::Ident(&parent_var)]),
            ));
            self.emit_css_props_wrapper_block(state, ctx, component_id, &node, namespace)?;
            return Ok(());
        }

        let (html, from_fn) = if matches!(namespace, Namespace::Svg) {
            (
                "<g><!></g>",
                super::namespace::from_namespace(Namespace::Svg),
            )
        } else {
            (
                "<svelte-css-wrapper style=\"display: contents\"><!></svelte-css-wrapper>",
                super::namespace::from_namespace(Namespace::Html),
            )
        };

        let tpl_name = self.ctx.state.gen_ident("root");
        let from_call = self.ctx.b.call_expr(
            from_fn,
            [Arg::Expr(self.ctx.b.template_str_expr(html)), Arg::Num(1.0)],
        );

        let frag = self.ctx.state.gen_ident("fragment");
        let node = self.ctx.state.gen_ident("node");
        state.init.push(
            self.ctx
                .b
                .var_stmt(&frag, self.ctx.b.call_expr(&tpl_name, [])),
        );
        state.init.push(self.ctx.b.var_stmt(
            &node,
            self.ctx.b.call_expr("$.first_child", [Arg::Ident(&frag)]),
        ));

        self.emit_css_props_wrapper_block(state, ctx, component_id, &node, namespace)?;
        self.hoist(self.ctx.b.var_stmt(&tpl_name, from_call));

        let anchor_ident = match &ctx.anchor {
            FragmentAnchor::Root => "$$anchor".to_string(),
            FragmentAnchor::CallbackParam { name, .. } => name.clone(),
            FragmentAnchor::Child { parent_var } => parent_var.clone(),
            FragmentAnchor::SiblingVar { var } => var.clone(),
        };
        state.init.push(
            self.ctx
                .b
                .call_stmt("$.append", [Arg::Ident(&anchor_ident), Arg::Ident(&frag)]),
        );

        Ok(())
    }

    pub(super) fn emit_css_props_wrapper_block(
        &mut self,
        state: &mut EmitState<'a>,
        ctx: &FragmentCtx<'a>,
        component_id: NodeId,
        node_ident: &str,
        namespace: svelte_ast::Namespace,
    ) -> Result<()> {
        let css_props: Vec<ComponentCssProp> = self
            .ctx
            .query
            .view
            .component_css_props(component_id)
            .to_vec();
        let mut prop_items: Vec<ObjProp<'a>> = Vec::with_capacity(css_props.len());
        let mut css_memo_decls: Vec<Statement<'a>> = Vec::new();
        let mut memo_counter: u32 = 0;
        for prop in css_props {
            let key = self.ctx.b.alloc_str(&prop.name);
            let expr = match prop.value {
                ComponentCssPropValue::Expression(expr_id) => {
                    let Some(expr) = self.ctx.state.parsed.take_expr(expr_id) else {
                        return CodegenError::missing_expression(prop.attr_id);
                    };
                    let data = self.ctx.expression_data(prop.attr_id).cloned();
                    let expr = coarse_wrap(self.ctx, expr, data.as_ref());
                    let expr = self.maybe_wrap_legacy_slots_read(expr);
                    match prop.memo {
                        ComponentPropMemo::Derived => {
                            let helper = self.ctx.query.view.derived_helper();
                            let memo_name = format!("${memo_counter}");
                            memo_counter += 1;
                            let thunk = self.ctx.b.thunk(expr);
                            let derived = self.ctx.b.call_expr(helper, [Arg::Expr(thunk)]);
                            css_memo_decls
                                .push(self.ctx.b.let_init_stmt(&memo_name, derived));
                            let memo_ref = self.ctx.b.alloc_str(&memo_name);
                            rune_get(&self.ctx.b, memo_ref)
                        }
                        ComponentPropMemo::Inline | ComponentPropMemo::Getter => expr,
                    }
                }
                ComponentCssPropValue::StaticString(span) => {
                    let value = self.ctx.query.component.source_text(span);
                    self.ctx.b.str_expr(value)
                }
                ComponentCssPropValue::Concatenation => {
                    let Some(view) = self
                        .ctx
                        .query
                        .component
                        .store
                        .get(component_id)
                        .as_component_like()
                    else {
                        return CodegenError::missing_expression(prop.attr_id);
                    };
                    let Some(svelte_ast::Attribute::ConcatenationAttribute(concat)) = view
                        .attributes
                        .iter()
                        .find(|a| a.id() == prop.attr_id)
                    else {
                        return CodegenError::missing_expression(prop.attr_id);
                    };
                    let mut tpl_parts: Vec<TemplatePart<'a>> =
                        Vec::with_capacity(concat.parts.len());
                    for part in &concat.parts {
                        match part {
                            svelte_ast::ConcatPart::Static(s) => {
                                if let Some(TemplatePart::Str(prev)) = tpl_parts.last_mut() {
                                    prev.push_str(s);
                                } else {
                                    tpl_parts.push(TemplatePart::Str(s.clone()));
                                }
                            }
                            svelte_ast::ConcatPart::Dynamic { id, expr } => {
                                let part_expr = self.take_template_expr(*id, expr)?;
                                tpl_parts.push(TemplatePart::Expr(part_expr, false));
                            }
                        }
                    }
                    self.ctx.b.template_parts_expr(tpl_parts)
                }
            };
            prop_items.push(ObjProp::KeyValue(key, expr));
        }
        let props_obj = self.ctx.b.object_expr(prop_items);
        let props_thunk = self.ctx.b.thunk(props_obj);

        let prev_init_len = state.init.len();
        let mut wrapper_ctx = ctx.clone();
        wrapper_ctx.anchor = FragmentAnchor::SiblingVar {
            var: format!("{}.lastChild", node_ident),
        };
        wrapper_ctx.namespace = namespace;
        let mut hoisted_memo_decls: Vec<Statement<'a>> = Vec::new();
        self.emit_component_with_hoisted_memo(
            state,
            &wrapper_ctx,
            component_id,
            None,
            &mut hoisted_memo_decls,
            memo_counter,
        )?;
        let component_stmts: Vec<_> = state.init.drain(prev_init_len..).collect();

        let mut block: Vec<Statement<'a>> = Vec::new();
        block.extend(css_memo_decls);
        block.extend(hoisted_memo_decls);
        block.push(self.ctx.b.call_stmt(
            "$.css_props",
            [Arg::Ident(node_ident), Arg::Expr(props_thunk)],
        ));
        block.extend(component_stmts);
        block.push(
            self.ctx
                .b
                .call_stmt("$.reset", [Arg::Ident(node_ident)]),
        );

        state.init.push(self.ctx.b.block_stmt(block));
        Ok(())
    }

    fn render_tag_uses_direct_anchor(&self, id: NodeId) -> bool {
        if !matches!(
            self.ctx.query.component.store.get(id),
            svelte_ast::Node::RenderTag(_)
        ) {
            return false;
        }
        match self.ctx.query.analysis.block_semantics(id) {
            svelte_analyze::BlockSemantics::Render(sem) => {
                let Some(sym) = sem.callee_sym else {
                    return false;
                };
                matches!(
                    self.ctx.query.view.binding_semantics(sym),
                    svelte_analyze::BindingSemantics::MaybeReactive
                        | svelte_analyze::BindingSemantics::NonReactive
                        | svelte_analyze::BindingSemantics::Unresolved,
                )
            }
            _ => false,
        }
    }

    fn template_from_fn(
        &self,
        ctx: &FragmentCtx<'a>,
        fragment_id: svelte_ast::FragmentId,
        strategy_kind: StrategyKind,
    ) -> &'static str {
        let fragment = self.ctx.query.component.store.fragment(fragment_id);
        if let StrategyKind::SingleElement = strategy_kind {
            for &id in &fragment.nodes {
                if matches!(
                    self.ctx.query.component.store.get(id),
                    svelte_ast::Node::Element(_)
                ) {
                    if let Some(ns) = self.ctx.query.view.creation_namespace(id) {
                        return super::namespace::from_namespace(ns);
                    }
                    break;
                }
            }
        }
        if let StrategyKind::Multi = strategy_kind {
            let mut acc: Option<svelte_ast::Namespace> = None;
            for &id in &fragment.nodes {
                if let Some(ns) = self.ctx.query.view.creation_namespace(id) {
                    acc = match acc {
                        None => Some(ns),
                        Some(prev) if prev == ns => Some(prev),
                        Some(_) => Some(svelte_ast::Namespace::Html),
                    };
                    if matches!(acc, Some(svelte_ast::Namespace::Html)) {
                        break;
                    }
                }
            }
            if let Some(ns) = acc {
                return super::namespace::from_namespace(ns);
            }
        }
        super::namespace::from_namespace(ctx.namespace)
    }

    pub(crate) fn finalize_slot_root_template(
        &mut self,
        state: &mut EmitState<'a>,
        ctx: &FragmentCtx<'a>,
        init_len_before: usize,
        tpl_name: String,
        slot_el_id: NodeId,
        fragment_id: svelte_ast::FragmentId,
    ) -> Result<()> {
        self.finalize_root_template_inner(
            state,
            ctx,
            StrategyKind::SingleElement,
            init_len_before,
            tpl_name,
            fragment_id,
            Some(slot_el_id),
        )
    }

    fn finalize_root_template(
        &mut self,
        state: &mut EmitState<'a>,
        ctx: &FragmentCtx<'a>,
        strategy_kind: StrategyKind,
        init_len_before: usize,
        tpl_name: String,
        fragment_id: svelte_ast::FragmentId,
    ) -> Result<()> {
        self.finalize_root_template_inner(
            state,
            ctx,
            strategy_kind,
            init_len_before,
            tpl_name,
            fragment_id,
            None,
        )
    }

    fn finalize_root_template_inner(
        &mut self,
        state: &mut EmitState<'a>,
        ctx: &FragmentCtx<'a>,
        strategy_kind: StrategyKind,
        init_len_before: usize,
        tpl_name: String,
        fragment_id: svelte_ast::FragmentId,
        slot_root_id: Option<NodeId>,
    ) -> Result<()> {
        let from_fn = self.template_from_fn(ctx, fragment_id, strategy_kind);
        let html_str = state.template.as_html();
        let needs_import = state.template.needs_import_node;
        let mut from_html = {
            let b = &self.ctx.state.b;
            let tpl_expr = b.template_str_expr(&html_str);
            match (strategy_kind, needs_import) {
                (StrategyKind::Multi, false) => {
                    b.call_expr(from_fn, [Arg::Expr(tpl_expr), Arg::Num(1.0)])
                }
                (StrategyKind::Multi, true) => {
                    b.call_expr(from_fn, [Arg::Expr(tpl_expr), Arg::Num(3.0)])
                }
                (StrategyKind::SingleElement, true) => {
                    b.call_expr(from_fn, [Arg::Expr(tpl_expr), Arg::Num(2.0)])
                }
                (StrategyKind::SingleElement, false) => b.call_expr(from_fn, [Arg::Expr(tpl_expr)]),
            }
        };
        if state.template.contains_script_tag {
            from_html = self
                .ctx
                .state
                .b
                .call_expr("$.with_script", [Arg::Expr(from_html)]);
        }
        if self.ctx.state.dev {
            let locs = match slot_root_id {
                Some(id) => self.build_slot_root_locations(ctx, id),
                None => self.build_template_locations(ctx, fragment_id),
            };
            if let Some(locs) = locs {
                from_html = self.wrap_add_locations(from_html, locs);
            }
        }
        let tpl_stmt = self.ctx.state.b.var_stmt(&tpl_name, from_html);
        self.hoist(tpl_stmt);

        let var_name = match state.root_var.as_deref() {
            Some(name) => name.to_string(),
            None => match strategy_kind {
                StrategyKind::Multi => self.ctx.state.gen_ident("fragment"),
                StrategyKind::SingleElement => self.ctx.state.gen_ident("root"),
            },
        };

        let prefix_stmt = self
            .ctx
            .state
            .b
            .var_stmt(&var_name, self.ctx.state.b.call_expr(&tpl_name, []));

        state.init.insert(init_len_before, prefix_stmt);
        state.template = super::data_structures::Template::new();
        state.root_var = Some(var_name);

        Ok(())
    }

    fn emit_static_node(
        &mut self,
        state: &mut EmitState<'a>,
        ctx: &FragmentCtx<'a>,
        part: &ConcatPart,
    ) -> Result<()> {
        let Some(text) = ctx.static_text_of(part) else {
            return CodegenError::unexpected_child("Static", "Expr in SingleStatic");
        };

        match &ctx.anchor {
            FragmentAnchor::Root
            | FragmentAnchor::CallbackParam {
                append_inside: false,
                ..
            } => {
                let name = self.ctx.state.gen_ident("text");
                let b = &self.ctx.state.b;
                if role_needs_text_first_next(ctx.role) {
                    state
                        .init
                        .push(b.call_stmt("$.next", empty::<Arg<'a, '_>>()));
                }
                let call = if text.is_empty() {
                    b.call_expr("$.text", empty::<Arg<'a, '_>>())
                } else {
                    b.call_expr("$.text", [Arg::StrRef(text)])
                };
                state.init.push(b.var_stmt(&name, call));
                state.root_var = Some(name);
            }
            FragmentAnchor::CallbackParam {
                append_inside: true,
                ..
            } => {
                let name = self.ctx.state.gen_ident("text");
                let b = &self.ctx.state.b;
                let call = if text.is_empty() {
                    b.call_expr("$.text", empty::<Arg<'a, '_>>())
                } else {
                    b.call_expr("$.text", [Arg::StrRef(text)])
                };
                state.init.push(b.var_stmt(&name, call));
                state.root_var = Some(name);
            }
            FragmentAnchor::Child { .. } => {
                let html = ctx.static_html_of(part).unwrap_or(text);
                state.template.push_text(html);
            }
            FragmentAnchor::SiblingVar { .. } => {
                return CodegenError::unexpected_child("SingleStatic", "SiblingVar anchor");
            }
        }

        Ok(())
    }

    fn emit_static_comment_anchor(
        &mut self,
        state: &mut EmitState<'a>,
        ctx: &FragmentCtx<'a>,
    ) -> Result<()> {
        self.comment_anchor_node_name(state, ctx).map(drop)
    }

    fn emit_element_in_fragment(
        &mut self,
        state: &mut EmitState<'a>,
        ctx: &FragmentCtx<'a>,
        el_id: NodeId,
    ) -> Result<()> {
        let node = self.ctx.query.component.store.get(el_id);
        let is_html_element = matches!(node, svelte_ast::Node::Element(_));

        match &ctx.anchor {
            FragmentAnchor::Root | FragmentAnchor::CallbackParam { .. } => {
                if is_html_element {
                    let el_name = self.emit_element(state, ctx, el_id, None)?;
                    state.root_var = Some(el_name);
                } else {
                    self.emit_element(state, ctx, el_id, None)?;
                }
                Ok(())
            }
            FragmentAnchor::Child { parent_var } => {
                if let svelte_ast::Node::Element(el) = node {
                    if !self.ctx.needs_var(el_id) {
                        self.emit_element_ghost(state, ctx, el_id)?;
                    } else {
                        let el_name_hint = el.name.clone();
                        let prefix = self.element_ident_prefix(&el_name_hint);
                        let el_name = self.ctx.state.gen_ident(&prefix);
                        let b = &self.ctx.state.b;
                        state.init.push(
                            b.var_stmt(&el_name, b.call_expr("$.child", [Arg::Ident(parent_var)])),
                        );
                        self.emit_element(state, ctx, el_id, Some(&el_name))?;
                        state.last_fragment_needs_reset = true;
                    }
                } else {
                    self.emit_element(state, ctx, el_id, None)?;
                }
                Ok(())
            }
            FragmentAnchor::SiblingVar { .. } => {
                self.emit_element(state, ctx, el_id, None)?;
                Ok(())
            }
        }
    }
}
