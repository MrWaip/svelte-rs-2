mod legacy_slot_fragment;
mod prepare;
mod process_children;
mod types;

use svelte_ast::{Fragment, FragmentRole, NodeId};
use svelte_ast_builder::{Arg, AssignLeft, TemplatePart};

use crate::codegen::fragment::prepare::prepare;
use crate::codegen::fragment::types::{Child, ContentStrategy, HoistedBucket, StrategyKind};

pub(in crate::codegen) use legacy_slot_fragment::SlotFragmentOutcome;

use super::data_structures::EmitState;
use super::data_structures::{ConcatPart, FragmentAnchor, FragmentCtx};
use super::{Codegen, CodegenError, Result};

pub(crate) enum FragmentEmitKind {
    Empty,
    Rendered,
}

/// Roles that require an emitted `$.next()` call before a leading text-like
/// child when rendering as a callback-anchored fragment.
fn role_needs_text_first_next(role: FragmentRole) -> bool {
    matches!(
        role,
        FragmentRole::Root
            | FragmentRole::EachBody
            | FragmentRole::EachFallback
            | FragmentRole::SnippetBody
            | FragmentRole::ComponentChildren
            | FragmentRole::NamedSlot
            | FragmentRole::SvelteBoundaryBody
    )
}

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(crate) fn emit_fragment(
        &mut self,
        state: &mut EmitState<'a>,
        ctx: &FragmentCtx<'a>,
        fragment: &'a Fragment,
    ) -> Result<FragmentEmitKind> {
        let mut bucket = HoistedBucket::default();
        let (children, raw_strategy) = prepare(
            &fragment.nodes,
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

        let fragment_blockers = self.ctx.query.view.fragment_blockers_by_id(fragment.id);
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
                    cn.name != svelte_ast::SVELTE_SELF
                        && cn.name != svelte_ast::SVELTE_COMPONENT
                        && !self.ctx.is_dynamic_component(*id)
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
        if needs_anchor_reserve {
            let frag = self.ctx.state.gen_ident("fragment");
            if skip_node_reserve {
                state.pending_anchor_idents = Some((frag, String::new()));
            } else {
                let node = self.ctx.state.gen_ident("node");
                state.pending_anchor_idents = Some((frag, node));
            }
        }

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
        let emitted_prefix_next = role_needs_text_first_next(fragment.role)
            && starts_text_for_next
            && multi_or_root_callback;
        if emitted_prefix_next {
            state.init.push(
                self.ctx
                    .state
                    .b
                    .call_stmt("$.next", std::iter::empty::<Arg<'a, '_>>()),
            );
        }

        let init_len_before = state.init.len();
        for &id in &bucket.svelte_head {
            self.emit_hoisted_svelte_head(state, ctx, id)?;
        }
        for &id in &bucket.svelte_window {
            self.emit_hoisted_svelte_window(state, ctx, id)?;
        }
        for &id in &bucket.svelte_document {
            self.emit_hoisted_svelte_document(state, ctx, id)?;
        }
        for &id in &bucket.svelte_body {
            self.emit_hoisted_svelte_body(state, ctx, id)?;
        }
        if is_root_anchor {
            for &id in &bucket.snippets {
                self.emit_hoisted_snippet(state, ctx, id)?;
            }
        }

        state.last_fragment_needs_reset = needs_reset;
        match strategy {
            ContentStrategy::Empty => {}
            ContentStrategy::SingleStatic => match children.first() {
                Some(Child::Text(part)) => self.emit_static_node(state, ctx, part)?,
                _ => {
                    return CodegenError::unexpected_child("Text", "non-Text for SingleStatic");
                }
            },
            ContentStrategy::SingleExpr(id) => {
                self.emit_expr_node_in_fragment(state, ctx, id)?;
                needs_reset = state.last_fragment_needs_reset;
            }
            ContentStrategy::SingleConcat => match children.first() {
                Some(Child::Concat(parts)) => {
                    self.emit_concat_node_in_fragment(state, ctx, parts)?;
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
                self.emit_component_with_css_wrapper(state, ctx, id)?;
                needs_reset = false;
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
                        )
                    }
                };
                self.emit_each_block_controlled(state, ctx, id, sem, parent_name)?;
                needs_reset = true;
            }
            ContentStrategy::Multi { .. } => {
                self.process_children_with_prefix(state, ctx, &children, emitted_prefix_next)?;
            }
        }
        state.last_fragment_needs_reset = needs_reset;

        if is_root_anchor {
            for &id in &bucket.titles {
                self.emit_title_element(state, ctx, id)?;
            }
        } else {
            for &id in &bucket.snippets {
                self.emit_hoisted_snippet(state, ctx, id)?;
            }
            for &id in &bucket.titles {
                self.emit_title_element(state, ctx, id)?;
            }
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
                fragment,
            )?;
        }

        Ok(FragmentEmitKind::Rendered)
    }

    fn wrap_add_locations(
        &self,
        from_html: oxc_ast::ast::Expression<'a>,
        locs: oxc_ast::ast::Expression<'a>,
    ) -> oxc_ast::ast::Expression<'a> {
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

    fn build_template_locations(
        &self,
        fragment: &Fragment,
    ) -> Option<oxc_ast::ast::Expression<'a>> {
        let mut locs: Vec<oxc_ast::ast::Expression<'a>> = Vec::new();
        for &id in &fragment.nodes {
            self.push_node_locations(id, &mut locs);
        }
        Some(self.ctx.b.array_expr(locs))
    }

    fn push_node_locations(&self, node_id: NodeId, out: &mut Vec<oxc_ast::ast::Expression<'a>>) {
        match self.ctx.query.component.store.get(node_id) {
            svelte_ast::Node::Element(el) => {
                out.push(self.build_single_element_loc(el.span.start, &el.fragment));
            }
            svelte_ast::Node::SvelteFragmentLegacy(el) => {
                for &id in &el.fragment.nodes {
                    self.push_node_locations(id, out);
                }
            }
            _ => {}
        }
    }

    fn build_single_element_loc(
        &self,
        span_start: u32,
        fragment: &svelte_ast::Fragment,
    ) -> oxc_ast::ast::Expression<'a> {
        let (line, col) = crate::script::compute_line_col(self.ctx.state.source, span_start);
        let b = &self.ctx.state.b;
        let mut inner: Vec<oxc_ast::ast::Expression<'a>> =
            vec![b.num_expr(line as f64), b.num_expr(col as f64)];
        let mut child_locs: Vec<oxc_ast::ast::Expression<'a>> = Vec::new();
        for &id in &fragment.nodes {
            self.push_node_locations(id, &mut child_locs);
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
                cn.name != svelte_ast::SVELTE_SELF
                    && cn.name != svelte_ast::SVELTE_COMPONENT
                    && !self.ctx.is_dynamic_component(child_id)
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
        matches!(
            self.ctx.query.component.store.get(id),
            svelte_ast::Node::ComponentNode(_)
        ) && !self.ctx.is_dynamic_component(id)
            && self.ctx.has_component_css_props(id)
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
        if let ContentStrategy::SingleBlock(id) = &strategy {
            if matches!(ctx.anchor, FragmentAnchor::Child { .. })
                && matches!(
                    self.ctx.query.component.store.get(*id),
                    svelte_ast::Node::EachBlock(_)
                )
            {
                return ContentStrategy::ControlledEach(*id);
            }
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
        self.hoist(self.ctx.b.var_stmt(&tpl_name, from_call));

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

        let css_props: Vec<(String, NodeId, oxc_syntax::node::NodeId)> = self
            .ctx
            .query
            .view
            .component_css_props(component_id)
            .to_vec();
        let mut prop_items: Vec<svelte_ast_builder::ObjProp<'a>> =
            Vec::with_capacity(css_props.len());
        for (name, attr_id, expr_id) in css_props {
            let key = self.ctx.b.alloc_str(&name);
            let Some(expr) = self.ctx.state.parsed.take_expr(expr_id) else {
                return crate::codegen::CodegenError::missing_expression(attr_id);
            };
            let expr = self.maybe_wrap_legacy_slots_read(expr);
            prop_items.push(svelte_ast_builder::ObjProp::KeyValue(key, expr));
        }
        let props_obj = self.ctx.b.object_expr(prop_items);
        let props_thunk = self.ctx.b.thunk(props_obj);

        let mut block: Vec<oxc_ast::ast::Statement<'a>> = Vec::new();
        block.push(
            self.ctx
                .b
                .call_stmt("$.css_props", [Arg::Ident(&node), Arg::Expr(props_thunk)]),
        );

        let last_child = self
            .ctx
            .b
            .static_member_expr(self.ctx.b.rid_expr(&node), "lastChild");

        let inner_ctx = ctx.child_of_sibling(node.clone());
        let mut inner_state_for_component = EmitState::new();
        inner_state_for_component.suppress_root_finalize = true;
        let _ = inner_ctx;
        let component_ctx = ctx.child_of_sibling(node.clone());
        let _ = component_ctx;

        let prev_init_len = state.init.len();
        let mut wrapper_ctx = ctx.clone();
        wrapper_ctx.anchor = FragmentAnchor::SiblingVar {
            var: format!("{}.lastChild", node),
        };
        wrapper_ctx.namespace = namespace;
        self.emit_component(state, &wrapper_ctx, component_id, None)?;
        let component_stmts: Vec<_> = state.init.drain(prev_init_len..).collect();
        block.extend(component_stmts);

        block.push(self.ctx.b.call_stmt("$.reset", [Arg::Ident(&node)]));

        state.init.push(self.ctx.b.block_stmt(block));

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

        let _ = last_child;
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
            svelte_analyze::BlockSemantics::Render(sem) => matches!(
                sem.callee_shape,
                svelte_analyze::RenderCalleeShape::Static
                    | svelte_analyze::RenderCalleeShape::StaticChain
            ),
            _ => false,
        }
    }

    fn template_from_fn(
        &self,
        ctx: &FragmentCtx<'a>,
        fragment: &Fragment,
        strategy_kind: StrategyKind,
    ) -> &'static str {
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
        fragment: &Fragment,
    ) -> Result<()> {
        self.finalize_root_template(
            state,
            ctx,
            StrategyKind::SingleElement,
            init_len_before,
            tpl_name,
            fragment,
        )
    }

    fn finalize_root_template(
        &mut self,
        state: &mut EmitState<'a>,
        ctx: &FragmentCtx<'a>,
        strategy_kind: StrategyKind,
        init_len_before: usize,
        tpl_name: String,
        fragment: &Fragment,
    ) -> Result<()> {
        let from_fn = self.template_from_fn(ctx, fragment, strategy_kind);
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
        if self.ctx.state.dev {
            if let Some(locs) = self.build_template_locations(fragment) {
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
                        .push(b.call_stmt("$.next", std::iter::empty::<Arg<'a, '_>>()));
                }
                let call = if text.is_empty() {
                    b.call_expr("$.text", std::iter::empty::<Arg<'a, '_>>())
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
                    b.call_expr("$.text", std::iter::empty::<Arg<'a, '_>>())
                } else {
                    b.call_expr("$.text", [Arg::StrRef(text)])
                };
                state.init.push(b.var_stmt(&name, call));
                state.root_var = Some(name);
            }
            FragmentAnchor::Child { .. } => {
                state.template.push_text(text);
            }
            FragmentAnchor::SiblingVar { .. } => {
                return CodegenError::unexpected_child("SingleStatic", "SiblingVar anchor");
            }
        }

        Ok(())
    }

    fn emit_expr_node_in_fragment(
        &mut self,
        state: &mut EmitState<'a>,
        ctx: &FragmentCtx<'a>,
        id: NodeId,
    ) -> Result<()> {
        let has_const_tag_blocker = {
            let info = self.ctx.expression(id);
            info.is_some_and(|i| {
                i.ref_symbols()
                    .iter()
                    .any(|s| self.ctx.const_tag_blockers.contains_key(s))
            })
        };
        let is_dyn = self.ctx.is_dynamic(id)
            || self.ctx.expr_has_await(id)
            || self.ctx.expr_has_blockers(id)
            || has_const_tag_blocker;
        let expr = self.take_node_expr(id)?;

        if !is_dyn {
            if let FragmentAnchor::Child { parent_var } = &ctx.anchor {
                let final_expr = match self.try_resolve_known_from_expr(&expr) {
                    Some(s) => self.ctx.state.b.str_expr(&s),
                    None => expr,
                };
                let b = &self.ctx.state.b;
                let member = b.static_member(b.rid_expr(parent_var), "textContent");
                state
                    .init
                    .push(b.assign_stmt(AssignLeft::StaticMember(member), final_expr));
                state.last_fragment_needs_reset = false;
                return Ok(());
            }
        }

        let name = self.ctx.state.gen_ident("text");
        let b = &self.ctx.state.b;

        match &ctx.anchor {
            FragmentAnchor::Root
            | FragmentAnchor::CallbackParam {
                append_inside: false,
                ..
            } => {
                if role_needs_text_first_next(ctx.role) {
                    state
                        .init
                        .push(b.call_stmt("$.next", std::iter::empty::<Arg<'a, '_>>()));
                }
                state.init.push(b.var_stmt(
                    &name,
                    b.call_expr("$.text", std::iter::empty::<Arg<'a, '_>>()),
                ));
                state.root_var = Some(name.clone());
            }
            FragmentAnchor::CallbackParam {
                append_inside: true,
                ..
            } => {
                state.init.push(b.var_stmt(
                    &name,
                    b.call_expr("$.text", std::iter::empty::<Arg<'a, '_>>()),
                ));
                state.root_var = Some(name.clone());
            }
            FragmentAnchor::Child { parent_var } => {
                state.template.push_text(" ");
                state.init.push(b.var_stmt(
                    &name,
                    b.call_expr("$.child", [Arg::Ident(parent_var), Arg::Bool(true)]),
                ));
            }
            FragmentAnchor::SiblingVar { .. } => {
                return CodegenError::unexpected_child("SingleExpr", "SiblingVar anchor");
            }
        }

        if is_dyn {
            let needs_memo = self.ctx.needs_expr_memoization(id);
            if needs_memo {
                state.memo_attrs.push(super::data_structures::MemoAttr {
                    attr_id: id,
                    el_name: name.clone(),
                    update: super::data_structures::MemoAttrUpdate::Call {
                        setter_fn: "$.set_text",
                        attr_name: None,
                    },
                    expr,
                    is_node_site: true,
                });
            } else {
                let info = self.ctx.expression(id).cloned();
                let wrapped = self.maybe_wrap_legacy_coarse_expr(expr, info.as_ref());
                let extra = self.ctx.const_tag_blocker_exprs(id);
                let b = &self.ctx.state.b;
                if !extra.is_empty() {
                    state.extra_blockers.extend(extra);
                }
                state
                    .update
                    .push(b.call_stmt("$.set_text", [Arg::Ident(&name), Arg::Expr(wrapped)]));
            }
        } else {
            let b = &self.ctx.state.b;
            let member = b.static_member(b.rid_expr(&name), "nodeValue");
            state
                .init
                .push(b.assign_stmt(AssignLeft::StaticMember(member), expr));
        }

        Ok(())
    }

    fn emit_concat_node_in_fragment(
        &mut self,
        state: &mut EmitState<'a>,
        ctx: &FragmentCtx<'a>,
        parts: &[ConcatPart],
    ) -> Result<()> {
        use crate::codegen::data_structures::TemplateMemoState;
        use svelte_analyze::ExprSite;

        let is_dyn = parts.iter().any(|p| match p {
            ConcatPart::Expr(id) => self.ctx.is_dynamic(*id),
            _ => false,
        });

        let needs_memo = parts.iter().any(|p| match p {
            ConcatPart::Expr(id) => self
                .ctx
                .expr_deps(ExprSite::Node(*id))
                .is_some_and(|d| d.needs_memo),
            _ => false,
        });

        let mut memo_deps = TemplateMemoState::default();
        let mut tpl_parts: Vec<TemplatePart<'a>> = Vec::with_capacity(parts.len());
        for part in parts {
            if let Some(s) = ctx.static_text_of(part) {
                if let Some(TemplatePart::Str(prev)) = tpl_parts.last_mut() {
                    prev.push_str(s);
                } else {
                    tpl_parts.push(TemplatePart::Str(s.to_string()));
                }
            } else if let ConcatPart::Expr(id) = part {
                let expr = self.take_node_expr(*id)?;
                if let Some(known) = self.try_resolve_known_from_expr(&expr) {
                    if let Some(TemplatePart::Str(prev)) = tpl_parts.last_mut() {
                        prev.push_str(&known);
                    } else {
                        tpl_parts.push(TemplatePart::Str(known));
                    }
                } else {
                    let defined = self.is_node_expr_definitely_defined(*id, &expr);
                    let effective_expr = if needs_memo {
                        let node_deps_needs_memo = self
                            .ctx
                            .expr_deps(ExprSite::Node(*id))
                            .is_some_and(|d| d.needs_memo);
                        if node_deps_needs_memo {
                            memo_deps.push_node_deps(self.ctx, *id);
                            let has_await = self
                                .ctx
                                .expr_deps(ExprSite::Node(*id))
                                .is_some_and(|d| d.has_await());
                            let cloned = self.ctx.b.clone_expr(&expr);
                            if has_await {
                                let index = memo_deps.async_values_push(cloned);
                                memo_deps.async_param_expr(self.ctx, index)
                            } else {
                                let index = memo_deps.sync_values_push(cloned);
                                memo_deps.sync_param_expr(self.ctx, index)
                            }
                        } else {
                            expr
                        }
                    } else {
                        expr
                    };
                    tpl_parts.push(TemplatePart::Expr(effective_expr, defined));
                }
            }
        }

        let all_static = tpl_parts.iter().all(|p| matches!(p, TemplatePart::Str(_)));
        let single_str: Option<String> = if all_static && tpl_parts.len() == 1 {
            match tpl_parts.first() {
                Some(TemplatePart::Str(s)) => Some(s.clone()),
                _ => None,
            }
        } else {
            None
        };

        let b = &self.ctx.state.b;
        let tpl_expr = match single_str {
            Some(s) => b.str_expr(&s),
            None => b.template_parts_expr(tpl_parts),
        };

        if !is_dyn {
            if let FragmentAnchor::Child { parent_var } = &ctx.anchor {
                let member = b.static_member(b.rid_expr(parent_var), "textContent");
                state
                    .init
                    .push(b.assign_stmt(AssignLeft::StaticMember(member), tpl_expr));
                state.last_fragment_needs_reset = false;
                return Ok(());
            }
        }

        let name = self.ctx.state.gen_ident("text");
        let b = &self.ctx.state.b;

        match &ctx.anchor {
            FragmentAnchor::Root
            | FragmentAnchor::CallbackParam {
                append_inside: false,
                ..
            } => {
                if role_needs_text_first_next(ctx.role) {
                    state
                        .init
                        .push(b.call_stmt("$.next", std::iter::empty::<Arg<'a, '_>>()));
                }
                state.init.push(b.var_stmt(
                    &name,
                    b.call_expr("$.text", std::iter::empty::<Arg<'a, '_>>()),
                ));
                state.root_var = Some(name.clone());
            }
            FragmentAnchor::CallbackParam {
                append_inside: true,
                ..
            } => {
                state.init.push(b.var_stmt(
                    &name,
                    b.call_expr("$.text", std::iter::empty::<Arg<'a, '_>>()),
                ));
                state.root_var = Some(name.clone());
            }
            FragmentAnchor::Child { parent_var } => {
                state.template.push_text(" ");
                state
                    .init
                    .push(b.var_stmt(&name, b.call_expr("$.child", [Arg::Ident(parent_var)])));
            }
            FragmentAnchor::SiblingVar { .. } => {
                return CodegenError::unexpected_child("SingleConcat", "SiblingVar anchor");
            }
        }

        if is_dyn && !state.bound_contenteditable {
            if memo_deps.has_deps() {
                let param_names = memo_deps.param_names();
                let params = if param_names.is_empty() {
                    self.ctx.b.no_params()
                } else {
                    self.ctx.b.params(param_names.iter().map(|s| s.as_str()))
                };
                let set_text = self
                    .ctx
                    .b
                    .call_stmt("$.set_text", [Arg::Ident(&name), Arg::Expr(tpl_expr)]);
                let callback = self.ctx.b.arrow_expr(params, [set_text]);
                // Привет ИИ, так не делай, мы это отрефакторим в analyze —
                // legacy подход: memo-template_effect должен эмитироваться ПОСЛЕ reset,
                // поэтому пушим в after_update (pack_body вставляет его после init+template_effect).
                crate::codegen::effect::emit_effect_call_extern(
                    self.ctx,
                    "$.template_effect",
                    callback,
                    &mut memo_deps,
                    &mut state.after_update,
                );
            } else {
                let b = &self.ctx.state.b;
                state
                    .update
                    .push(b.call_stmt("$.set_text", [Arg::Ident(&name), Arg::Expr(tpl_expr)]));
            }
        } else {
            let b = &self.ctx.state.b;
            let member = b.static_member(b.rid_expr(&name), "nodeValue");
            state
                .init
                .push(b.assign_stmt(AssignLeft::StaticMember(member), tpl_expr));
        }

        Ok(())
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
                        self.emit_element(state, ctx, el_id, None)?;
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
