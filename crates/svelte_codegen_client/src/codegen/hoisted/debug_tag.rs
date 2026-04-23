use svelte_ast::NodeId;
use svelte_ast_builder::{Arg, ObjProp};

use super::super::data_structures::{EmitState, FragmentCtx};
use super::super::{Codegen, Result};

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(super) fn emit_hoisted_debug_tag(
        &mut self,
        state: &mut EmitState<'a>,
        _ctx: &FragmentCtx<'a>,
        id: NodeId,
    ) -> Result<()> {
        let tag = self.ctx.debug_tag(id);
        let identifiers = tag.identifiers.clone();
        let runes = self.ctx.query.runes();

        let mut props: Vec<ObjProp<'a>> = Vec::with_capacity(identifiers.len());
        for span in &identifiers {
            let name = self.ctx.query.component.source_text(*span).to_string();
            let name_alloc: &str = self.ctx.b.alloc_str(&name);

            let ident_expr = self
                .ctx
                .state
                .parsed
                .expr_handle(span.start)
                .and_then(|handle| self.ctx.state.parsed.take_expr(handle))
                .unwrap_or_else(|| self.ctx.b.rid_expr(name_alloc));
            let snapshot = self.ctx.b.call_expr("$.snapshot", [Arg::Expr(ident_expr)]);

            let value = if runes {
                snapshot
            } else {
                self.ctx
                    .b
                    .call_expr("$.untrack", [Arg::Expr(self.ctx.b.thunk(snapshot))])
            };
            props.push(ObjProp::KeyValue(name_alloc, value));
        }

        let obj = self.ctx.b.object_expr(props);
        let log_call = self.ctx.b.call_stmt("console.log", [Arg::Expr(obj)]);
        let debugger = self.ctx.b.debugger_stmt();
        let thunk = self.ctx.b.thunk_block(vec![log_call, debugger]);
        state.init.push(
            self.ctx
                .b
                .call_stmt("$.template_effect", [Arg::Expr(thunk)]),
        );
        Ok(())
    }
}
