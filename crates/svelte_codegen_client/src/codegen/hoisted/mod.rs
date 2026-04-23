mod const_tag;
mod debug_tag;
mod snippet;
mod special_target;
mod svelte_head;
mod title;

use svelte_analyze::{BlockSemantics, ConstTagAsyncKind};

use super::data_structures::{EmitState, FragmentCtx};
use super::{Codegen, Result};

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(super) fn emit_hoisted_const_tags(
        &mut self,
        state: &mut EmitState<'a>,
        ctx: &FragmentCtx<'a>,
        bucket: &super::prepare::HoistedBucket,
    ) -> Result<()> {
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
        Ok(())
    }

    pub(super) fn emit_hoisted_svelte_head_only(
        &mut self,
        state: &mut EmitState<'a>,
        ctx: &FragmentCtx<'a>,
        bucket: &super::prepare::HoistedBucket,
    ) -> Result<()> {
        for &id in &bucket.svelte_head {
            self.emit_hoisted_svelte_head(state, ctx, id)?;
        }
        Ok(())
    }

    pub(super) fn emit_hoisted_special_targets(
        &mut self,
        state: &mut EmitState<'a>,
        ctx: &FragmentCtx<'a>,
        bucket: &super::prepare::HoistedBucket,
    ) -> Result<()> {
        for &id in &bucket.svelte_window {
            self.emit_hoisted_svelte_window(state, ctx, id)?;
        }
        for &id in &bucket.svelte_document {
            self.emit_hoisted_svelte_document(state, ctx, id)?;
        }
        for &id in &bucket.svelte_body {
            self.emit_hoisted_svelte_body(state, ctx, id)?;
        }
        Ok(())
    }

    pub(super) fn emit_hoisted_rest(
        &mut self,
        state: &mut EmitState<'a>,
        ctx: &FragmentCtx<'a>,
        bucket: &super::prepare::HoistedBucket,
    ) -> Result<()> {
        for &id in &bucket.snippets {
            self.emit_hoisted_snippet(state, ctx, id)?;
        }
        for &id in &bucket.titles {
            self.emit_title_element(state, ctx, id)?;
        }
        Ok(())
    }

    pub(super) fn emit_hoisted_snippets_only(
        &mut self,
        state: &mut EmitState<'a>,
        ctx: &FragmentCtx<'a>,
        bucket: &super::prepare::HoistedBucket,
    ) -> Result<()> {
        for &id in &bucket.snippets {
            self.emit_hoisted_snippet(state, ctx, id)?;
        }
        Ok(())
    }

    pub(super) fn emit_hoisted_titles_only(
        &mut self,
        state: &mut EmitState<'a>,
        ctx: &FragmentCtx<'a>,
        bucket: &super::prepare::HoistedBucket,
    ) -> Result<()> {
        for &id in &bucket.titles {
            self.emit_title_element(state, ctx, id)?;
        }
        Ok(())
    }
}
