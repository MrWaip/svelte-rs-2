use svelte_analyze::{ElementSemantics, head_hash};
use svelte_ast::{FragmentId, Node, NodeId};
use svelte_ast_builder::Arg;

use crate::error::Result;
use crate::fragment::FragmentParent;
use crate::model::ServerCodegen;

impl<'a> ServerCodegen<'a> {
    pub(crate) fn svelte_head(&mut self, id: NodeId) -> Result<()> {
        let Node::SvelteHead(head) = self.component.store.get(id) else {
            return Ok(());
        };
        let head_fragment = head.fragment;

        let body =
            self.child_statements(|cg| cg.fragment(head_fragment, FragmentParent::SvelteHead))?;

        let arrow = self.b.arrow_block_expr(self.b.params(["$$renderer"]), body);
        let hash = head_hash(self.filename);
        let call = self.b.call_stmt(
            "$.head",
            [Arg::Str(hash), Arg::Ident("$$renderer"), Arg::Expr(arrow)],
        );
        self.push_stmt(call);
        Ok(())
    }

    pub(crate) fn title_element(&mut self, id: NodeId) -> Result<()> {
        let Node::Element(el) = self.component.store.get(id) else {
            return Ok(());
        };

        let prev_save = self.save_block_awaits;
        self.save_block_awaits = true;
        let body = self.child_statements(|cg| {
            cg.push_text("<title>");
            cg.fragment(el.fragment, FragmentParent::Element(el))?;
            cg.push_text("</title>");
            Ok(())
        });
        self.save_block_awaits = prev_save;
        let body = body?;

        let arrow = self.b.arrow_block_expr(self.b.params(["$$renderer"]), body);
        let call = self.b.call_stmt("$$renderer.title", [Arg::Expr(arrow)]);
        self.push_stmt(call);
        Ok(())
    }

    pub(crate) fn emit_fragment_titles(&mut self, id: FragmentId) -> Result<()> {
        let node_ids: Vec<NodeId> = self.component.store.fragment(id).nodes.to_vec();
        for nid in node_ids {
            match self.analysis.element_semantics.query(nid) {
                ElementSemantics::HeadTitle => self.title_element(nid)?,
                ElementSemantics::None
                | ElementSemantics::RegularElement(_)
                | ElementSemantics::Boundary(_)
                | ElementSemantics::SvelteElement(_)
                | ElementSemantics::LegacySlot(_)
                | ElementSemantics::LegacyComponentSlots(_) => {}
            }
        }
        Ok(())
    }
}
