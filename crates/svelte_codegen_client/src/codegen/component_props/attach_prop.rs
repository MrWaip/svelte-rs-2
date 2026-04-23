use oxc_syntax::node::NodeId as OxcNodeId;
use svelte_ast::NodeId;
use svelte_ast_builder::{Arg, ObjProp};

use super::super::{Codegen, CodegenError, Result};
use super::dispatch::PropOrSpread;

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(super) fn emit_component_prop_attach(
        &mut self,
        attr_id: NodeId,
        expr_id: OxcNodeId,
        is_dynamic: bool,
        items: &mut Vec<PropOrSpread<'a>>,
    ) -> Result<()> {
        let key_expr = self.ctx.b.call_expr("$.attachment", []);
        let Some(expr) = self.ctx.state.parsed.take_expr(expr_id) else {
            return CodegenError::missing_expression(attr_id);
        };
        let expr = self.maybe_wrap_legacy_slots_read(expr);
        if is_dynamic {
            let call = self.ctx.b.call_expr_callee(expr, [Arg::Ident("$$node")]);
            let wrapper = self
                .ctx
                .b
                .arrow_expr(self.ctx.b.params(["$$node"]), [self.ctx.b.expr_stmt(call)]);
            items.push(PropOrSpread::Prop(ObjProp::Computed(key_expr, wrapper)));
        } else {
            items.push(PropOrSpread::Prop(ObjProp::Computed(key_expr, expr)));
        }
        Ok(())
    }
}
