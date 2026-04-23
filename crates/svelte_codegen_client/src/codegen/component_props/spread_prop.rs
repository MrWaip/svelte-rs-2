use oxc_syntax::node::NodeId as OxcNodeId;
use svelte_ast::NodeId;

use super::super::{Codegen, CodegenError, Result};
use super::dispatch::PropOrSpread;

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(super) fn emit_component_prop_spread(
        &mut self,
        attr_id: NodeId,
        expr_id: OxcNodeId,
        is_dynamic: bool,
        items: &mut Vec<PropOrSpread<'a>>,
    ) -> Result<()> {
        let Some(expr) = self.ctx.state.parsed.take_expr(expr_id) else {
            return CodegenError::missing_expression(attr_id);
        };
        let spread_expr = if is_dynamic {
            self.ctx.b.thunk(expr)
        } else {
            expr
        };
        items.push(PropOrSpread::Spread(spread_expr));
        Ok(())
    }
}
