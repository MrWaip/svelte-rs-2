use oxc_ast::ast::Expression;
use oxc_syntax::node::NodeId as OxcNodeId;
use svelte_analyze::ComponentAttachEmit;
use svelte_ast::NodeId;
use svelte_ast_builder::{Arg, ObjProp};

use super::super::{Codegen, CodegenError, Result};
use super::dispatch::PropOrSpread;

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(super) fn emit_component_prop_attach(
        &mut self,
        attr_id: NodeId,
        expr_id: OxcNodeId,
        emit: ComponentAttachEmit,
        items: &mut Vec<PropOrSpread<'a>>,
    ) -> Result<()> {
        let key_expr = self.ctx.b.call_expr("$.attachment", []);
        let Some(expr) = self.ctx.state.parsed.take_expr(expr_id) else {
            return CodegenError::missing_expression(attr_id);
        };
        let value = match emit {
            ComponentAttachEmit::Inline => expr,
            ComponentAttachEmit::Wrapped => self.wrap_attach_callee(expr),
            ComponentAttachEmit::WrappedFallback => {
                let guarded = self.ctx.b.logical_or(expr, self.ctx.b.rid_expr("$.noop"));
                self.wrap_attach_callee(guarded)
            }
        };
        items.push(PropOrSpread::Prop(ObjProp::Computed(key_expr, value)));
        Ok(())
    }

    fn wrap_attach_callee(&self, callee: Expression<'a>) -> Expression<'a> {
        let call = self.ctx.b.call_expr_callee(callee, [Arg::Ident("$$node")]);
        self.ctx
            .b
            .arrow_expr(self.ctx.b.params(["$$node"]), [self.ctx.b.expr_stmt(call)])
    }
}
