use crate::codegen::expr::coarse_wrap;
use svelte_emit_builders::runes::rune_get;
mod legacy_slot_fragment;
mod prepare;
mod process_children;
mod types;

use oxc_ast::ast::{Expression, Statement};
use std::iter::empty;
use svelte_analyze::{AttributeSemantics, ComponentCssPropValue, SnippetPlacement, Volatility};
use svelte_ast::{FragmentRole, NodeId};
use svelte_ast_builder::{Arg, ObjProp};

use crate::codegen::concatenation::ConcatenationAnchor;
use crate::codegen::fragment::prepare::prepare;
use crate::codegen::fragment::types::{Child, ContentStrategy, HoistedBucket, StrategyKind};
use smallvec::SmallVec;

enum HoistedDispatchKind {
    Snippet,
    DebugTag,
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

fn single_fragment_anchor<'a>(ctx: &FragmentCtx<'a>) -> Result<ConcatenationAnchor> {
    match &ctx.anchor {
        FragmentAnchor::Root => Ok(ConcatenationAnchor::SingleFragmentRoot),
        FragmentAnchor::CallbackParam { append_inside, .. } => {
            Ok(ConcatenationAnchor::SingleFragmentCallbackParam {
                append_inside: *append_inside,
            })
        }
        FragmentAnchor::Child { parent_var } => Ok(ConcatenationAnchor::SingleFragmentChild {
            parent_var: parent_var.clone(),
        }),
        FragmentAnchor::ElementContentChild { .. } => {
            CodegenError::unexpected_child("Single", "ElementContentChild anchor")
        }
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
        let component = self.ctx.query.component;
        let fragment_nodes: &'a [NodeId] = &component.store.fragment(fragment_id).nodes;
        let (children, raw_strategy) = prepare(fragment_nodes, &component.store, ctx, &mut bucket);
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

        let strategy_kind = StrategyKind::of(&strategy);

        let mut needs_reset = !matches!(
            strategy,
            ContentStrategy::Empty
                | ContentStrategy::SingleStatic
                | ContentStrategy::SingleElement(_)
                | ContentStrategy::CssWrappedComponent(_)
        );

        let mut local_snippets: SmallVec<[NodeId; 4]> = SmallVec::new();
        let mut hoisted_snippets: SmallVec<[NodeId; 4]> = SmallVec::new();
        for &id in &bucket.snippets {
            match self.snippet_placement(id) {
                SnippetPlacement::Local => local_snippets.push(id),
                SnippetPlacement::ModuleLevel | SnippetPlacement::InstanceLevel => {
                    hoisted_snippets.push(id)
                }
            }
        }
        local_snippets.sort_by_key(|id| id.0);
        for id in local_snippets {
            self.emit_inline_snippet_block(state, id)?;
        }

        {
            use svelte_analyze::{BlockSemantics, ConstTagAsyncKind};
            let recording_slot_const_tags = state.legacy_slot_record_const_tag_end;
            if recording_slot_const_tags {
                state.legacy_slot_const_tag_start = Some(state.init.len());
            }
            let has_async = self.ctx.state.experimental_async
                && bucket.const_tags.iter().any(|&id| {
                    let BlockSemantics::ConstTag(s) = self.ctx.query.analysis.block_semantics(id)
                    else {
                        return false;
                    };
                    match s.async_kind {
                        ConstTagAsyncKind::Awaited { .. } | ConstTagAsyncKind::Deferred { .. } => {
                            true
                        }
                        ConstTagAsyncKind::Sync => false,
                    }
                });
            let mut ordered: SmallVec<[NodeId; 4]> = bucket.const_tags.iter().copied().collect();
            ordered.sort_by_key(|&id| match self.ctx.query.analysis.block_semantics(id) {
                BlockSemantics::ConstTag(s) => s.order_rank,
                _ => 0,
            });
            if !state.skip_const_tags {
                if has_async {
                    self.emit_const_tags_async_batch(state, &ordered)?;
                } else {
                    for &id in &ordered {
                        self.emit_hoisted_const_tag(state, ctx, id)?;
                    }
                }
            }
            if recording_slot_const_tags {
                state.legacy_slot_record_const_tag_end = false;
                state.legacy_slot_const_tag_end = Some(state.init.len());
            }
        }

        let declaration_tag_ids: SmallVec<[NodeId; 2]> =
            bucket.declaration_tags.iter().copied().collect();
        self.emit_declaration_tags(state, &declaration_tag_ids)?;

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
                if self.is_standalone_static_component(*id));
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
            state
                .init
                .push(self.ctx.state.b.call_stmt("$.next", empty::<Arg<'a, '_>>()));
        }

        let init_len_before = state.init.len();
        let mut ordered: SmallVec<[(NodeId, HoistedDispatchKind); 4]> = SmallVec::new();
        for &id in &hoisted_snippets {
            ordered.push((id, HoistedDispatchKind::Snippet));
        }
        for &id in &bucket.debug_tags {
            ordered.push((id, HoistedDispatchKind::DebugTag));
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
                    self.emit_hoisted_snippet(state, ctx, id)?;
                }
                HoistedDispatchKind::DebugTag => {
                    self.emit_hoisted_debug_tag(state, ctx, id)?;
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

        let special_block_end = state.init.len();

        if needs_anchor_reserve && !pre_emit_frag_pending && !state.suppress_root_finalize {
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

        let strategy_is_text_root = match &strategy {
            ContentStrategy::SingleStatic
            | ContentStrategy::SingleExpr(_)
            | ContentStrategy::SingleConcat => true,
            ContentStrategy::Empty
            | ContentStrategy::SingleElement(_)
            | ContentStrategy::SingleBlock(_)
            | ContentStrategy::CssWrappedComponent(_)
            | ContentStrategy::ControlledEach(_)
            | ContentStrategy::Multi { .. } => false,
        };
        let anchor_is_root_text = match &ctx.anchor {
            FragmentAnchor::Root => true,
            FragmentAnchor::CallbackParam {
                append_inside: false,
                ..
            } => true,
            FragmentAnchor::CallbackParam {
                append_inside: true,
                ..
            } => false,
            FragmentAnchor::Child { .. }
            | FragmentAnchor::ElementContentChild { .. }
            | FragmentAnchor::SiblingVar { .. } => false,
        };
        let rotate_text_root_before_specials =
            special_block_end > init_len_before && strategy_is_text_root && anchor_is_root_text;

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
                self.emit_concatenation(state, ctx, anchor, &[ConcatPart::Expr(id)])?;
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
                if state.suppress_root_finalize {
                    self.process_children_with_prefix(state, ctx, &children, emitted_prefix_next)?;
                    needs_reset = state.last_fragment_needs_reset;
                } else {
                    let standalone = self.standalone_ctx_for_single(ctx, id);
                    let use_ctx = standalone.as_ref().unwrap_or(ctx);
                    self.emit_fragment_child(state, use_ctx, id)?;
                }
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

        if rotate_text_root_before_specials {
            state.init[init_len_before..].rotate_left(special_block_end - init_len_before);
        }

        state.last_fragment_needs_reset = needs_reset;

        for &id in &bucket.titles {
            self.emit_title_element(state, ctx, id)?;
        }

        if is_root_anchor
            && template_was_empty_before
            && !state.template.is_empty()
            && !state.suppress_root_finalize
        {
            self.finalize_root_template(state, ctx, strategy_kind, init_len_before, fragment_id)?;
        }

        Ok(FragmentEmitKind::Rendered)
    }

    pub(in crate::codegen) fn wrap_add_locations(
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

    pub(in crate::codegen) fn build_template_locations(
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
                if self.ctx.opaque_content(node_id) {
                    out.push(self.single_location(el.span.start));
                } else {
                    out.push(self.build_single_element_loc(ctx, el.span.start, el.fragment));
                }
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
            svelte_ast::Node::ComponentNode(cn) if self.ctx.has_component_css_props(node_id) => {
                out.push(self.single_location(cn.span.start));
            }
            svelte_ast::Node::SvelteComponentLegacy(cn)
                if self.ctx.has_component_css_props(node_id) =>
            {
                out.push(self.single_location(cn.span.start));
            }
            svelte_ast::Node::SvelteElement(el) if self.ctx.has_component_css_props(node_id) => {
                out.push(self.single_location(el.span.start));
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

    fn single_location(&self, span_start: u32) -> Expression<'a> {
        let (line, col) = self.ctx.state.line_index.line_col(span_start);
        let b = &self.ctx.state.b;
        b.array_expr(vec![b.num_expr(line as f64), b.num_expr(col as f64)])
    }

    fn build_single_element_loc(
        &self,
        ctx: &FragmentCtx<'a>,
        span_start: u32,
        fragment_id: svelte_ast::FragmentId,
    ) -> Expression<'a> {
        let (line, col) = self.ctx.state.line_index.line_col(span_start);
        let b = &self.ctx.state.b;
        let mut inner: Vec<Expression<'a>> = vec![b.num_expr(line as f64), b.num_expr(col as f64)];
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
        if !self.is_standalone_static_component(child_id) {
            return None;
        }
        let mut new_ctx = ctx.clone();
        new_ctx.anchor = FragmentAnchor::CallbackParam {
            name,
            append_inside: true,
        };
        Some(new_ctx)
    }

    fn is_standalone_static_component(&self, id: NodeId) -> bool {
        if self.ctx.state.hmr {
            return false;
        }
        let svelte_ast::Node::ComponentNode(_) = self.ctx.query.component.store.get(id) else {
            return false;
        };
        match self.ctx.expression_data(id).map(|d| d.volatility) {
            Some(Volatility::Static) | None => {}
            Some(Volatility::Reactive | Volatility::Heavy | Volatility::Asynchronous) => {
                return false;
            }
        }
        !self.ctx.has_component_css_props(id)
    }

    fn is_css_wrapped_component(&self, id: NodeId) -> bool {
        match self.ctx.query.component.store.get(id) {
            svelte_ast::Node::ComponentNode(_) => {
                match self.ctx.expression_data(id).map(|d| d.volatility) {
                    Some(Volatility::Static) | None => self.ctx.has_component_css_props(id),
                    Some(Volatility::Reactive | Volatility::Heavy | Volatility::Asynchronous) => {
                        false
                    }
                }
            }
            svelte_ast::Node::SvelteComponentLegacy(_) | svelte_ast::Node::SvelteSelf(_) => {
                self.ctx.has_component_css_props(id)
            }
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

        let frag = self.ctx.state.gen_ident("fragment");
        let node = self.ctx.state.gen_ident("node");
        let frag_stmt_idx = state.init.len();
        state.init.push(self.ctx.b.var_stmt(
            &node,
            self.ctx.b.call_expr("$.first_child", [Arg::Ident(&frag)]),
        ));

        self.emit_css_props_wrapper_block(state, ctx, component_id, &node, namespace)?;

        let dedup_key = (!self.ctx.state.dev).then(|| format!("{from_fn}\u{0}1\u{0}{html}"));
        let from_call = self.ctx.b.call_expr(
            from_fn,
            [Arg::Expr(self.ctx.b.template_str_expr(html)), Arg::Num(1.0)],
        );
        let from_call = if self.ctx.state.dev {
            let span_start = self
                .ctx
                .query
                .component
                .store
                .get(component_id)
                .span()
                .start;
            let locs = self
                .ctx
                .b
                .array_expr(vec![self.single_location(span_start)]);
            self.wrap_add_locations(from_call, locs)
        } else {
            from_call
        };
        let tpl_name = self.hoist_template_dedup(dedup_key, from_call);
        state.init.insert(
            frag_stmt_idx,
            self.ctx
                .b
                .var_stmt(&frag, self.ctx.b.call_expr(&tpl_name, [])),
        );

        let anchor_ident = match &ctx.anchor {
            FragmentAnchor::Root => "$$anchor".to_string(),
            FragmentAnchor::CallbackParam { name, .. } => name.clone(),
            FragmentAnchor::Child { parent_var }
            | FragmentAnchor::ElementContentChild { parent_var } => parent_var.clone(),
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
        let css_props: Vec<(NodeId, String, ComponentCssPropValue)> = {
            let Some(view) = self
                .ctx
                .query
                .component
                .store
                .get(component_id)
                .as_component_like()
            else {
                return CodegenError::semantic_mismatch(
                    component_id,
                    "component-like node expected",
                );
            };
            view.attributes
                .iter()
                .filter_map(|attribute| {
                    let attribute_id = attribute.id();
                    match self.ctx.query.analysis.attributes.get(attribute_id) {
                        AttributeSemantics::ComponentCssProp(value) => {
                            Some((attribute_id, attribute.name()?.to_string(), value.clone()))
                        }
                        _ => None,
                    }
                })
                .collect()
        };
        let mut prop_items: Vec<ObjProp<'a>> = Vec::with_capacity(css_props.len());
        let mut css_memo_decls: Vec<Statement<'a>> = Vec::new();
        let mut memo_counter: u32 = 0;
        for (attribute_id, name, value) in css_props {
            let key = self.ctx.b.alloc_str(&name);
            let expr = match value {
                ComponentCssPropValue::Expression(expression_id) => {
                    let Some(expression) = self.ctx.state.parsed.take_expr(expression_id) else {
                        return CodegenError::missing_expression(attribute_id);
                    };
                    let expression_data = self.ctx.expression_data(attribute_id).cloned();
                    let expression = coarse_wrap(self.ctx, expression, expression_data.as_ref());
                    let volatility = expression_data
                        .as_ref()
                        .map(|data| data.volatility)
                        .unwrap_or(Volatility::Static);
                    match volatility {
                        Volatility::Heavy | Volatility::Asynchronous => {
                            let helper = self.ctx.query.view.derived_helper();
                            let memo_name = format!("${memo_counter}");
                            memo_counter += 1;
                            let thunk = self.ctx.b.thunk(expression);
                            let derived = self.ctx.b.call_expr(helper, [Arg::Expr(thunk)]);
                            css_memo_decls.push(self.ctx.b.let_init_stmt(&memo_name, derived));
                            let memo_ref = self.ctx.b.alloc_str(&memo_name);
                            rune_get(&self.ctx.b, memo_ref)
                        }
                        Volatility::Static | Volatility::Reactive => expression,
                    }
                }
                ComponentCssPropValue::StaticString(span) => {
                    let value = self.ctx.query.component.source_text(span);
                    self.ctx.b.str_expr(value)
                }
                ComponentCssPropValue::Concatenation(plan) => {
                    let Some(view) = self
                        .ctx
                        .query
                        .component
                        .store
                        .get(component_id)
                        .as_component_like()
                    else {
                        return CodegenError::missing_expression(attribute_id);
                    };
                    let Some(svelte_ast::Attribute::ConcatenationAttribute(concat)) =
                        view.attributes.iter().find(|a| a.id() == attribute_id)
                    else {
                        return CodegenError::missing_expression(attribute_id);
                    };
                    self.build_concat_expr_from_plan(
                        attribute_id,
                        &concat.parts,
                        &plan,
                        &mut css_memo_decls,
                        &mut memo_counter,
                    )?
                }
                ComponentCssPropValue::Boolean => self.ctx.b.bool_expr(true),
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
        self.emit_component_inline_memo(state, &wrapper_ctx, component_id, memo_counter)?;
        let mut component_stmts: Vec<_> = state.init.drain(prev_init_len..).collect();

        let css_props_call = self.ctx.b.call_stmt(
            "$.css_props",
            [Arg::Ident(node_ident), Arg::Expr(props_thunk)],
        );

        let mut block: Vec<Statement<'a>> = Vec::new();
        block.extend(css_memo_decls);

        let inner_block = if component_stmts.len() == 1 {
            match component_stmts.pop() {
                Some(Statement::BlockStatement(inner)) => {
                    Some(inner.unbox().body.into_iter().collect::<Vec<_>>())
                }
                Some(other) => {
                    component_stmts.push(other);
                    None
                }
                None => None,
            }
        } else {
            None
        };

        if let Some(mut inner) = inner_block {
            let component_call = inner.pop();
            block.extend(inner);
            block.push(css_props_call);
            if let Some(call) = component_call {
                block.push(call);
            }
        } else {
            block.push(css_props_call);
            block.extend(component_stmts);
        }

        block.push(self.ctx.b.call_stmt("$.reset", [Arg::Ident(node_ident)]));

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
            svelte_analyze::BlockSemantics::Render(sem) => !sem.callee_volatility.is_volatile(),
            _ => false,
        }
    }

    fn template_from_fn(&self, fragment_id: svelte_ast::FragmentId) -> &'static str {
        super::namespace::from_namespace(self.ctx.query.view.fragment_namespace(fragment_id))
    }

    pub(crate) fn finalize_slot_root_template(
        &mut self,
        state: &mut EmitState<'a>,
        ctx: &FragmentCtx<'a>,
        init_len_before: usize,
        slot_el_id: NodeId,
        fragment_id: svelte_ast::FragmentId,
    ) -> Result<()> {
        self.finalize_root_template_inner(
            state,
            ctx,
            StrategyKind::SingleElement,
            init_len_before,
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
        fragment_id: svelte_ast::FragmentId,
    ) -> Result<()> {
        self.finalize_root_template_inner(
            state,
            ctx,
            strategy_kind,
            init_len_before,
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
        fragment_id: svelte_ast::FragmentId,
        slot_root_id: Option<NodeId>,
    ) -> Result<()> {
        let from_fn = self.template_from_fn(fragment_id);
        let html_str = state.template.as_html();
        let needs_import = state.template.needs_import_node;
        let flags: u32 = match (strategy_kind, needs_import) {
            (StrategyKind::Multi, false) => 1,
            (StrategyKind::Multi, true) => 3,
            (StrategyKind::SingleElement, true) => 2,
            (StrategyKind::SingleElement, false) => 0,
        };

        let dedup_key =
            (!self.ctx.state.dev).then(|| format!("{from_fn}\u{0}{flags}\u{0}{html_str}"));

        let mut from_html = {
            let b = &self.ctx.state.b;
            let tpl_expr = b.template_str_expr(&html_str);
            if flags == 0 {
                b.call_expr(from_fn, [Arg::Expr(tpl_expr)])
            } else {
                b.call_expr(from_fn, [Arg::Expr(tpl_expr), Arg::Num(flags as f64)])
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
        let tpl_name = self.hoist_template_dedup(dedup_key, from_html);

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

    fn hoist_template_dedup(
        &mut self,
        dedup_key: Option<String>,
        from_html: Expression<'a>,
    ) -> String {
        if let Some(name) = dedup_key
            .as_ref()
            .and_then(|key| self.ctx.state.hoisted_templates.get(key).cloned())
        {
            return name;
        }
        let name = self.ctx.state.gen_ident("root");
        let tpl_stmt = self.ctx.state.b.var_stmt(&name, from_html);
        self.hoist(tpl_stmt);
        if let Some(key) = dedup_key {
            self.ctx.state.hoisted_templates.insert(key, name.clone());
        }
        name
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
            FragmentAnchor::Child { .. } | FragmentAnchor::ElementContentChild { .. } => {
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
            FragmentAnchor::Child { parent_var }
            | FragmentAnchor::ElementContentChild { parent_var } => {
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
