use oxc_ast::ast::Statement;
use oxc_syntax::node::NodeId as OxcNodeId;
use svelte_analyze::ComponentSpreadEmit;
use svelte_ast::NodeId;
use svelte_ast_builder::Arg;
use svelte_emit_builders::runes::rune_get;

use super::super::async_values::AsyncValues;
use super::super::{Codegen, CodegenError, Result};
use super::dispatch::PropOrSpread;

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(super) fn emit_component_prop_spread(
        &mut self,
        attr_id: NodeId,
        expr_id: OxcNodeId,
        emit: ComponentSpreadEmit,
        items: &mut Vec<PropOrSpread<'a>>,
        memo_decls: &mut Vec<Statement<'a>>,
        memo_counter: &mut u32,
        async_values: &mut AsyncValues<'a>,
    ) -> Result<()> {
        let Some(expr) = self.ctx.state.parsed.take_expr(expr_id) else {
            return CodegenError::missing_expression(attr_id);
        };
        let spread_expr = match emit {
            ComponentSpreadEmit::AwaitedThunk => {
                let suspension = self
                    .ctx
                    .expression_data(attr_id)
                    .map(|d| d.suspension)
                    .unwrap_or_default();
                let memo_ref = async_values.push(self.ctx, expr, suspension);
                let get = rune_get(&self.ctx.b, memo_ref);
                self.ctx.b.thunk(get)
            }
            ComponentSpreadEmit::MemoThunk => {
                let helper = self.ctx.query.view.derived_helper();
                let memo_name = format!("${memo_counter}");
                *memo_counter += 1;
                let thunk = self.ctx.b.thunk(expr);
                let derived = self.ctx.b.call_expr(helper, [Arg::Expr(thunk)]);
                memo_decls.push(self.ctx.b.let_init_stmt(&memo_name, derived));
                let memo_ref = self.ctx.b.alloc_str(&memo_name);
                let get = rune_get(&self.ctx.b, memo_ref);
                self.ctx.b.thunk(get)
            }
            ComponentSpreadEmit::Thunk => self.ctx.b.thunk(expr),
            ComponentSpreadEmit::Inline => expr,
        };
        items.push(PropOrSpread::Spread(spread_expr));
        Ok(())
    }
}
