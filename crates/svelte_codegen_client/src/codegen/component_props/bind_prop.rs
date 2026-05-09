use svelte_analyze::ComponentBindTarget;
use svelte_ast::NodeId;
use svelte_ast_builder::{Arg, AssignLeft, ObjProp};

use super::super::Codegen;
use super::dispatch::{OwnershipBinding, PropOrSpread};

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(super) fn emit_bind_plain(
        &mut self,
        name: &str,
        source_text: &str,
        items: &mut Vec<PropOrSpread<'a>>,
    ) {
        let key = self.ctx.b.alloc_str(name);
        let source_ref = self.ctx.b.alloc_str(source_text);
        let get_body = self.ctx.b.rid_expr(source_ref);
        items.push(PropOrSpread::Prop(ObjProp::Getter(key, get_body)));
        let set_body = self.ctx.b.assign_expr(
            AssignLeft::Ident(source_text.to_string()),
            self.ctx.b.rid_expr("$$value"),
        );
        items.push(PropOrSpread::Prop(ObjProp::Setter(
            key,
            "$$value",
            None,
            vec![self.ctx.b.expr_stmt(set_body)],
        )));
    }

    pub(super) fn emit_bind_identifier(
        &mut self,
        _el_id: NodeId,
        name: &str,
        source_text: &str,
        target: ComponentBindTarget,
        items: &mut Vec<PropOrSpread<'a>>,
        ownership_bindings: &mut Vec<OwnershipBinding<'a>>,
    ) {
        let key = self.ctx.b.alloc_str(name);
        let source_ref = self.ctx.b.alloc_str(source_text);
        match target {
            ComponentBindTarget::PropSource | ComponentBindTarget::PropSourceOwned => {
                let get_body = self.ctx.b.call_expr(source_ref, []);
                items.push(PropOrSpread::Prop(ObjProp::Getter(key, get_body)));
                let set_body = self.ctx.b.call_expr(source_ref, [Arg::Ident("$$value")]);
                items.push(PropOrSpread::Prop(ObjProp::Setter(
                    key,
                    "$$value",
                    None,
                    vec![self.ctx.b.expr_stmt(set_body)],
                )));
                if matches!(target, ComponentBindTarget::PropSourceOwned) {
                    ownership_bindings.push(OwnershipBinding {
                        name: name.to_string(),
                        source_ident: source_ref,
                    });
                }
            }
            ComponentBindTarget::Rune => {
                let get_body = self.ctx.b.call_expr("$.get", [Arg::Ident(source_ref)]);
                items.push(PropOrSpread::Prop(ObjProp::Getter(key, get_body)));
                let set_body = self
                    .ctx
                    .b
                    .call_expr("$.set", [Arg::Ident(source_ref), Arg::Ident("$$value")]);
                items.push(PropOrSpread::Prop(ObjProp::Setter(
                    key,
                    "$$value",
                    None,
                    vec![self.ctx.b.expr_stmt(set_body)],
                )));
            }
            ComponentBindTarget::Plain => {
                self.emit_bind_plain(name, source_text, items);
            }
        }
    }

    pub(super) fn emit_bind_store(
        &mut self,
        name: &str,
        store_base: &str,
        items: &mut Vec<PropOrSpread<'a>>,
    ) {
        let key = self.ctx.b.alloc_str(name);
        let store_ref = format!("${store_base}");
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

        let base_id: &str = self.ctx.b.alloc_str(store_base);
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
