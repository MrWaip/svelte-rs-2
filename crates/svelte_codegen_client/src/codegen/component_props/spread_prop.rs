use svelte_ast::NodeId;

use super::super::{Codegen, Result};
use super::dispatch::PropOrSpread;

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(super) fn emit_component_prop_spread(
        &mut self,
        attr_id: NodeId,
        is_dynamic: bool,
        items: &mut Vec<PropOrSpread<'a>>,
    ) -> Result<()> {
        let expr = self.take_attr_expr(attr_id)?;
        let spread_expr = if is_dynamic {
            self.ctx.b.thunk(expr)
        } else {
            expr
        };
        items.push(PropOrSpread::Spread(spread_expr));
        Ok(())
    }
}
