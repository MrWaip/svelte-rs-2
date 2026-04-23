use svelte_analyze::normalize_regular_attribute_name;
use svelte_ast::{ConcatenationAttribute, NodeId};

use super::super::data_structures::EmitState;
use super::super::{Codegen, Result};

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(in super::super) fn emit_attr_concatenation(
        &mut self,
        state: &mut EmitState<'a>,
        owner_id: NodeId,
        owner_tag: &str,
        owner_var: &str,
        attr: &ConcatenationAttribute,
    ) -> Result<()> {
        if attr.name == "class" {
            return Ok(());
        }

        let attr_id = attr.id;
        let val = self.build_concat_expr_collapse_single(attr_id, &attr.parts)?;

        let html_attr_namespace = self.is_html_attr_namespace(owner_id);
        let attr_name = normalize_regular_attribute_name(&attr.name, html_attr_namespace);
        let attr_update = self.regular_attr_update(owner_id, owner_tag, &attr_name);

        let is_dyn = self.ctx.is_dynamic_attr(attr_id);
        let target = if is_dyn {
            &mut state.update
        } else {
            &mut state.init
        };
        self.push_regular_attr_update(target, owner_var, attr_update, val);

        Ok(())
    }
}
