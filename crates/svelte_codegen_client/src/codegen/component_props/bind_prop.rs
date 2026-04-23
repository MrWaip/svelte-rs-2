use svelte_analyze::ComponentBindMode;
use svelte_ast::NodeId;
use svelte_ast_builder::{Arg, AssignLeft, ObjProp};

use super::super::{Codegen, CodegenError, Result};
use super::dispatch::PropOrSpread;

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(super) fn emit_component_prop_bind(
        &mut self,
        el_id: NodeId,
        name: &str,
        mode: ComponentBindMode,
        expr_name: Option<String>,
        items: &mut Vec<PropOrSpread<'a>>,
    ) -> Result<()> {
        let key = self.ctx.b.alloc_str(name);
        let name_ref = self.ctx.b.alloc_str(name);
        match mode {
            ComponentBindMode::PropSource => {
                let get_body = self.ctx.b.call_expr(name_ref, []);
                items.push(PropOrSpread::Prop(ObjProp::Getter(key, get_body)));
                let set_body = self.ctx.b.call_expr(name_ref, [Arg::Ident("$$value")]);
                items.push(PropOrSpread::Prop(ObjProp::Setter(
                    key,
                    "$$value",
                    None,
                    vec![self.ctx.b.expr_stmt(set_body)],
                )));
            }
            ComponentBindMode::Rune => {
                let get_body = self.ctx.b.call_expr("$.get", [Arg::Ident(name_ref)]);
                items.push(PropOrSpread::Prop(ObjProp::Getter(key, get_body)));
                let set_body = self
                    .ctx
                    .b
                    .call_expr("$.set", [Arg::Ident(name_ref), Arg::Ident("$$value")]);
                items.push(PropOrSpread::Prop(ObjProp::Setter(
                    key,
                    "$$value",
                    None,
                    vec![self.ctx.b.expr_stmt(set_body)],
                )));
            }
            ComponentBindMode::Plain => {
                let get_body = self.ctx.b.rid_expr(name_ref);
                items.push(PropOrSpread::Prop(ObjProp::Getter(key, get_body)));
                let set_body = self.ctx.b.assign_expr(
                    AssignLeft::Ident(name.to_string()),
                    self.ctx.b.rid_expr("$$value"),
                );
                items.push(PropOrSpread::Prop(ObjProp::Setter(
                    key,
                    "$$value",
                    None,
                    vec![self.ctx.b.expr_stmt(set_body)],
                )));
            }
            ComponentBindMode::StoreSub => {
                let Some(store_ref) = expr_name else {
                    return CodegenError::unexpected_node(
                        el_id,
                        "StoreSub bind must have expr_name",
                    );
                };
                let store_id = self.ctx.b.alloc_str(&store_ref);

                let mark_stmt = self
                    .ctx
                    .b
                    .expr_stmt(self.ctx.b.call_expr("$.mark_store_binding", []));
                let return_expr = self.ctx.b.call_expr(store_id, []);
                let return_stmt = self.ctx.b.return_stmt(return_expr);
                items.push(PropOrSpread::Prop(ObjProp::GetterBody(
                    key,
                    vec![mark_stmt, return_stmt],
                )));

                let base_name = &store_ref[1..];
                let base_id: &str = self.ctx.b.alloc_str(base_name);
                let set_body = self
                    .ctx
                    .b
                    .call_expr("$.store_set", [Arg::Ident(base_id), Arg::Ident("$$value")]);
                items.push(PropOrSpread::Prop(ObjProp::Setter(
                    key,
                    "$$value",
                    None,
                    vec![self.ctx.b.expr_stmt(set_body)],
                )));
            }
        }
        Ok(())
    }
}
