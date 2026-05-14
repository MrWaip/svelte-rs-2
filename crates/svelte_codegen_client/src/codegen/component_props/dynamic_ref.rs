use oxc_ast::ast::Expression;
use svelte_ast::{Node, NodeId};

use super::super::{Codegen, CodegenError, Result};

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(in super::super) fn build_dynamic_component_ref(
        &mut self,
        component_id: NodeId,
    ) -> Result<Expression<'a>> {
        let node = self.ctx.query.component.store.get(component_id);
        let Node::ComponentNode(cn) = node else {
            return CodegenError::unexpected_node(component_id, "ComponentNode");
        };
        self.ctx
            .state
            .parsed
            .take_expr(cn.name.id())
            .ok_or(CodegenError::MissingExpression(component_id))
    }
}
