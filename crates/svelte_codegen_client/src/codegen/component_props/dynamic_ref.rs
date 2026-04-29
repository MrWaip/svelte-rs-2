use oxc_ast::ast::Expression;
use svelte_analyze::scope::SymbolId;
use svelte_analyze::{
    BindingSemantics, ConstBindingSemantics, PropBindingKind, PropBindingSemantics,
};
use svelte_ast::NodeId;
use svelte_ast_builder::Arg;

use super::super::{Codegen, CodegenError, Result};

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(in super::super) fn build_dynamic_component_ref(
        &mut self,
        component_id: NodeId,
        component_name: &str,
    ) -> Result<Expression<'a>> {
        if let Some((root_name, _)) = component_name.split_once('.') {
            let mut parts = component_name.split('.');
            let _ = parts.next();
            let mut expr = match self.ctx.component_binding_sym(component_id) {
                Some(sym_id) => self.build_component_binding_read_expr(sym_id)?,
                None => self.ctx.b.rid_expr(root_name),
            };
            for part in parts {
                expr = self.ctx.b.static_member_expr(expr, part);
            }
            return Ok(expr);
        }

        match self.ctx.component_binding_sym(component_id) {
            Some(sym_id) => self.build_component_binding_read_expr(sym_id),
            None => Ok(self.ctx.b.rid_expr(component_name)),
        }
    }

    pub(in super::super) fn build_component_binding_read_expr(
        &self,
        sym_id: SymbolId,
    ) -> Result<Expression<'a>> {
        let symbol_name = self.ctx.symbol_name(sym_id);
        let expr = match self.ctx.query.view.binding_semantics(sym_id) {
            BindingSemantics::Store(_) => self.ctx.b.call_expr(symbol_name, []),
            BindingSemantics::Prop(PropBindingSemantics {
                kind: PropBindingKind::NonSource,
                ..
            }) => {
                let Some(prop_name) = self.ctx.query.view.binding_origin_key(sym_id) else {
                    return CodegenError::unexpected_child(
                        "prop",
                        "NonSource prop binding missing origin key",
                    );
                };
                self.ctx
                    .b
                    .static_member_expr(self.ctx.b.rid_expr("$$props"), prop_name)
            }
            BindingSemantics::Prop(PropBindingSemantics {
                kind: PropBindingKind::Source { .. },
                ..
            }) => self.ctx.b.call_expr(symbol_name, []),
            BindingSemantics::State(state) if state.var_declared => self
                .ctx
                .b
                .call_expr("$.safe_get", [Arg::Ident(symbol_name)]),
            BindingSemantics::State(_) | BindingSemantics::Derived(_) => {
                self.ctx.b.call_expr("$.get", [Arg::Ident(symbol_name)])
            }
            BindingSemantics::Const(ConstBindingSemantics::ConstTag { destructured, .. }) => {
                if destructured {
                    self.ctx
                        .b
                        .call_expr("$.safe_get", [Arg::Ident(symbol_name)])
                } else {
                    self.ctx.b.call_expr("$.get", [Arg::Ident(symbol_name)])
                }
            }
            BindingSemantics::Contextual(_) => {
                self.ctx.b.call_expr("$.get", [Arg::Ident(symbol_name)])
            }
            BindingSemantics::NonReactive
            | BindingSemantics::Unresolved
            | BindingSemantics::OptimizedRune(_)
            | BindingSemantics::RuntimeRune { .. } => self.ctx.b.rid_expr(symbol_name),
            BindingSemantics::Prop(_) => self.ctx.b.rid_expr(symbol_name),

            BindingSemantics::LegacyBindableProp(_) => self
                .ctx
                .b
                .call_expr(symbol_name, std::iter::empty::<Arg<'_, '_>>()),

            BindingSemantics::LegacyState(state) => {
                let helper = if state.var_declared {
                    "$.safe_get"
                } else {
                    "$.get"
                };
                self.ctx.b.call_expr(helper, [Arg::Ident(symbol_name)])
            }
        };
        Ok(expr)
    }
}
