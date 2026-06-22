use oxc_ast::ast::Expression;
use svelte_ast::{Attribute, NodeId};
use svelte_ast_builder::Arg;

use super::super::{Codegen, CodegenError, Result};

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(in super::super) fn build_bind_this_call(
        &mut self,
        el_id: NodeId,
        bind_id: NodeId,
        value: Expression<'a>,
    ) -> Result<Expression<'a>> {
        let Some(view) = self
            .ctx
            .query
            .component
            .store
            .get(el_id)
            .as_component_like()
        else {
            return CodegenError::unexpected_node(el_id, "component-like");
        };
        let Some(bind) = view.attributes.iter().find_map(|a| {
            if let Attribute::BindDirective(b) = a {
                if b.id == bind_id { Some(b) } else { None }
            } else {
                None
            }
        }) else {
            return CodegenError::unexpected_node(el_id, "bind:this attribute must exist");
        };

        let Some(Expression::SequenceExpression(seq)) =
            self.ctx.state.parsed.take_expr(bind.expression.id())
        else {
            return CodegenError::unexpected_node(
                bind_id,
                "component bind:this must be lowered to a getter/setter sequence by the transform",
            );
        };
        let seq = seq.unbox();
        let mut exprs = seq.expressions.into_iter();
        let (Some(get_expr), Some(set_expr)) = (exprs.next(), exprs.next()) else {
            return CodegenError::unexpected_node(
                bind_id,
                "bind:this transformed sequence must carry getter and setter",
            );
        };
        let each_context: Vec<oxc_semantic::SymbolId> =
            match self.ctx.query.analysis.attributes.get(bind_id) {
                svelte_analyze::AttributeSemantics::ComponentBind(b) => {
                    b.each_context_vars.iter().copied().collect()
                }
                _ => Vec::new(),
            };
        let (setter, getter, dep) = self.build_bind_this_get_set(get_expr, set_expr, &each_context);
        let mut args = vec![Arg::Expr(value), Arg::Expr(setter), Arg::Expr(getter)];
        if let Some(dep) = dep {
            args.push(Arg::Expr(dep));
        }
        Ok(self.ctx.b.call_expr("$.bind_this", args))
    }
}
