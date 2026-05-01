use svelte_ast::{Node, NodeId};

use super::super::data_structures::EmitState;
use super::super::data_structures::FragmentCtx;
use super::super::{Codegen, CodegenError, Result};

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(in super::super) fn emit_fragment_child(
        &mut self,
        state: &mut EmitState<'a>,
        ctx: &FragmentCtx<'a>,
        id: NodeId,
    ) -> Result<()> {
        let node = self.ctx.query.component.store.get(id);
        match node {
            Node::Element(_)
            | Node::ComponentNode(_)
            | Node::SvelteComponentLegacy(_)
            | Node::SvelteElement(_)
            | Node::SvelteBoundary(_)
            | Node::SvelteWindow(_)
            | Node::SvelteDocument(_)
            | Node::SvelteBody(_)
            | Node::SvelteHead(_)
            | Node::SlotElementLegacy(_)
            | Node::SvelteFragmentLegacy(_) => {
                self.emit_element(state, ctx, id, None)?;
                Ok(())
            }
            Node::IfBlock(_) => {
                let sem = match self.ctx.query.analysis.block_semantics(id) {
                    svelte_analyze::BlockSemantics::If(s) => s.clone(),
                    _ => {
                        return CodegenError::unexpected_block_semantics(id, "IfBlock expected If");
                    }
                };
                self.emit_if_block(state, ctx, id, sem)
            }
            Node::EachBlock(_) => {
                let sem = match self.ctx.query.analysis.block_semantics(id) {
                    svelte_analyze::BlockSemantics::Each(s) => s.clone(),
                    _ => {
                        return CodegenError::unexpected_block_semantics(
                            id,
                            "EachBlock expected Each",
                        );
                    }
                };
                self.emit_each_block(state, ctx, id, sem)
            }
            Node::AwaitBlock(_) => {
                let sem = match self.ctx.query.analysis.block_semantics(id) {
                    svelte_analyze::BlockSemantics::Await(s) => s.clone(),
                    _ => {
                        return CodegenError::unexpected_block_semantics(
                            id,
                            "AwaitBlock expected Await",
                        );
                    }
                };
                self.emit_await_block(state, ctx, id, sem)
            }
            Node::KeyBlock(_) => {
                let sem = match self.ctx.query.analysis.block_semantics(id) {
                    svelte_analyze::BlockSemantics::Key(s) => s.clone(),
                    _ => {
                        return CodegenError::unexpected_block_semantics(
                            id,
                            "KeyBlock expected Key",
                        );
                    }
                };
                self.emit_key_block(state, ctx, id, sem)
            }
            Node::RenderTag(_) => {
                let sem = match self.ctx.query.analysis.block_semantics(id) {
                    svelte_analyze::BlockSemantics::Render(s) => s.clone(),
                    _ => {
                        return CodegenError::unexpected_block_semantics(
                            id,
                            "RenderTag expected Render",
                        );
                    }
                };
                self.emit_render_tag(state, ctx, id, sem)
            }
            Node::SnippetBlock(_) => {
                let sem = match self.ctx.query.analysis.block_semantics(id) {
                    svelte_analyze::BlockSemantics::Snippet(s) => s.clone(),
                    _ => {
                        return CodegenError::unexpected_block_semantics(
                            id,
                            "SnippetBlock expected Snippet",
                        );
                    }
                };
                self.emit_snippet_block(state, ctx, id, sem)
            }
            Node::HtmlTag(_) => self.emit_html_tag(state, ctx, id),
            Node::ConstTag(_) => CodegenError::unexpected_block_semantics(
                id,
                "ConstTag must be hoisted via prepare::HoistedBucket, not dispatched through emit_fragment_child",
            ),
            _ => CodegenError::unexpected_node(id, "fragment child (element-like or block)"),
        }
    }
}
