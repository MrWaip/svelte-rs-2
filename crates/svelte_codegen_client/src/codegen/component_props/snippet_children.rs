use oxc_ast::ast::Statement;
use svelte_analyze::BlockSemantics;
use svelte_ast::NodeId;
use svelte_ast_builder::ObjProp;

use super::super::{Codegen, CodegenError, Result};
use super::dispatch::PropOrSpread;

pub(in super::super) struct SnippetChildren<'a> {
    pub decls: Vec<Statement<'a>>,
    pub slot_keys: Vec<String>,
}

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(in super::super) fn build_component_snippet_children(
        &mut self,
        snippet_ids: &[NodeId],
        items: &mut Vec<PropOrSpread<'a>>,
    ) -> Result<SnippetChildren<'a>> {
        let mut out = SnippetChildren {
            decls: Vec::new(),
            slot_keys: Vec::new(),
        };

        for snippet_id in snippet_ids {
            let sem = match self.ctx.query.analysis.block_semantics(*snippet_id) {
                BlockSemantics::Snippet(s) => s.clone(),
                _ => {
                    return CodegenError::unexpected_block_semantics(
                        *snippet_id,
                        "component child must map to Snippet",
                    );
                }
            };
            let snippet_name = self.ctx.query.view.symbol_name(sem.name).to_string();
            out.decls.push(self.build_snippet_const(*snippet_id, &sem)?);
            let key = self.ctx.b.alloc_str(&snippet_name);
            items.push(PropOrSpread::Prop(ObjProp::Shorthand(key)));
            let slot_key = if snippet_name == "children" {
                "default".to_string()
            } else {
                snippet_name
            };
            out.slot_keys.push(slot_key);
        }
        Ok(out)
    }
}
