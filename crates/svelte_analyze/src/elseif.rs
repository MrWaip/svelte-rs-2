use oxc_semantic::ScopeId;
use svelte_ast::{IfBlock, Node};

use crate::data::{AnalysisData, FragmentItem, FragmentKey};
use crate::walker::TemplateVisitor;

pub(crate) struct ElseifVisitor;

impl TemplateVisitor for ElseifVisitor {
    fn visit_if_block(&mut self, block: &IfBlock, _scope: ScopeId, data: &mut AnalysisData) {
        if let Some(alt) = &block.alternate {
            let alt_key = FragmentKey::IfAlternate(block.id);
            let is_elseif = data.lowered_fragments.get(&alt_key).is_some_and(|lf| {
                lf.items.len() == 1
                    && matches!(&lf.items[0], FragmentItem::IfBlock(id)
                        if alt.nodes.iter().any(|n| matches!(n, Node::IfBlock(ib) if ib.id == *id && ib.elseif)))
            });
            if is_elseif {
                data.alt_is_elseif.insert(block.id);
            }
        }
    }
}
