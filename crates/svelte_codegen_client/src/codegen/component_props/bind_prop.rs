use oxc_ast::ast::{Expression, Statement};
use svelte_analyze::scope::SymbolId;
use svelte_analyze::{BindingSemantics, ComponentBindTarget, PropBindingKind, PropBindingSemantics};
use svelte_ast::NodeId;
use svelte_ast_builder::{Arg, AssignLeft, ObjProp};

use super::super::Codegen;
use super::dispatch::{OwnershipBinding, PropOrSpread};

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(super) fn emit_bind_member_expr(
        &mut self,
        bind_id: NodeId,
        expr: Expression<'a>,
        items: &mut Vec<PropOrSpread<'a>>,
    ) -> super::super::Result<()> {
        let Expression::ObjectExpression(obj) = expr else {
            return super::super::CodegenError::unexpected_node(
                bind_id,
                "ComponentBind Expression: get/set object expected",
            );
        };
        let obj = obj.unbox();
        for prop in obj.properties {
            items.push(PropOrSpread::Prop(ObjProp::Raw(prop)));
        }
        Ok(())
    }

    pub(super) fn emit_bind_function_pair(
        &mut self,
        bind_id: NodeId,
        name: &str,
        expr: Expression<'a>,
        items: &mut Vec<PropOrSpread<'a>>,
        init_stmts: &mut Vec<Statement<'a>>,
    ) -> super::super::Result<()> {
        let Expression::SequenceExpression(seq) = expr else {
            return super::super::CodegenError::unexpected_node(
                bind_id,
                "ComponentBind FunctionPair: SequenceExpression expected",
            );
        };
        let seq = seq.unbox();
        let mut iter = seq.expressions.into_iter();
        let Some(get_expr) = iter.next() else {
            return super::super::CodegenError::unexpected_node(
                bind_id,
                "ComponentBind FunctionPair: missing getter",
            );
        };
        let Some(set_expr) = iter.next() else {
            return super::super::CodegenError::unexpected_node(
                bind_id,
                "ComponentBind FunctionPair: missing setter",
            );
        };

        let bind_get_name = self.ctx.state.gen_ident("bind_get");
        let bind_set_name = self.ctx.state.gen_ident("bind_set");
        let bind_get_ref: &'a str = self.ctx.b.alloc_str(&bind_get_name);
        let bind_set_ref: &'a str = self.ctx.b.alloc_str(&bind_set_name);

        init_stmts.push(self.ctx.b.var_stmt(bind_get_ref, get_expr));
        init_stmts.push(self.ctx.b.var_stmt(bind_set_ref, set_expr));

        let key = self.ctx.b.alloc_str(name);
        let getter_call = self.ctx.b.call_expr(bind_get_ref, []);
        items.push(PropOrSpread::Prop(ObjProp::Getter(key, getter_call)));
        let setter_call = self
            .ctx
            .b
            .call_expr(bind_set_ref, [Arg::Ident("$$value")]);
        items.push(PropOrSpread::Prop(ObjProp::Setter(
            key,
            "$$value",
            None,
            vec![self.ctx.b.expr_stmt(setter_call)],
        )));
        Ok(())
    }

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
                let set_body = self.ctx.b.call_expr(
                    "$.set",
                    [
                        Arg::Ident(source_ref),
                        Arg::Ident("$$value"),
                        Arg::Bool(true),
                    ],
                );
                items.push(PropOrSpread::Prop(ObjProp::Setter(
                    key,
                    "$$value",
                    None,
                    vec![self.ctx.b.expr_stmt(set_body)],
                )));
            }
            ComponentBindTarget::RuneDerived => {
                let get_body = self.ctx.b.call_expr("$.get", [Arg::Ident(source_ref)]);
                items.push(PropOrSpread::Prop(ObjProp::Getter(key, get_body)));
                let set_body = self.ctx.b.call_expr(
                    "$.set",
                    [Arg::Ident(source_ref), Arg::Ident("$$value")],
                );
                items.push(PropOrSpread::Prop(ObjProp::Setter(
                    key,
                    "$$value",
                    None,
                    vec![self.ctx.b.expr_stmt(set_body)],
                )));
            }
            ComponentBindTarget::LegacyState => {
                let get_body = self.ctx.b.call_expr("$.get", [Arg::Ident(source_ref)]);
                items.push(PropOrSpread::Prop(ObjProp::Getter(key, get_body)));
                let set_body = self.ctx.b.call_expr(
                    "$.set",
                    [Arg::Ident(source_ref), Arg::Ident("$$value")],
                );
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
        base_symbol: SymbolId,
        items: &mut Vec<PropOrSpread<'a>>,
    ) {
        let key = self.ctx.b.alloc_str(name);
        let base_name = self.ctx.query.view.symbol_name(base_symbol);
        let store_ref = format!("${base_name}");
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

        let base_expr = match self.ctx.query.view.binding_semantics(base_symbol) {
            BindingSemantics::Prop(PropBindingSemantics {
                kind: PropBindingKind::NonSource,
                ..
            }) => {
                let prop_name = self
                    .ctx
                    .query
                    .view
                    .binding_origin_key(base_symbol)
                    .unwrap_or(base_name);
                self.ctx
                    .b
                    .static_member_expr(self.ctx.b.rid_expr("$$props"), prop_name)
            }
            BindingSemantics::State(_) | BindingSemantics::Derived(_) => {
                let base_id: &str = self.ctx.b.alloc_str(base_name);
                self.ctx
                    .b
                    .call_expr("$.get", [Arg::Ident(base_id)])
            }
            _ => {
                let base_id: &str = self.ctx.b.alloc_str(base_name);
                self.ctx.b.rid_expr(base_id)
            }
        };
        let set_body = self
            .ctx
            .b
            .call_expr("$.store_set", [Arg::Expr(base_expr), Arg::Ident("$$value")]);
        items.push(PropOrSpread::Prop(ObjProp::Setter(
            key,
            "$$value",
            None,
            vec![self.ctx.b.expr_stmt(set_body)],
        )));
    }
}
