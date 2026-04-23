use oxc_ast::ast::Expression;
use svelte_analyze::scope::SymbolId;
use svelte_analyze::{
    ConstDeclarationSemantics, DeclarationSemantics, PropDeclarationKind, PropDeclarationSemantics,
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
        let decl_node = self.ctx.query.scoping().symbol_declaration(sym_id);
        let expr = match self.ctx.query.view.declaration_semantics(decl_node) {
            DeclarationSemantics::Store(_) => self.ctx.b.call_expr(symbol_name, []),
            DeclarationSemantics::Prop(PropDeclarationSemantics {
                kind: PropDeclarationKind::NonSource,
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
            DeclarationSemantics::Prop(PropDeclarationSemantics {
                kind: PropDeclarationKind::Source { .. },
                ..
            }) => self.ctx.b.call_expr(symbol_name, []),
            DeclarationSemantics::State(state) if state.var_declared => self
                .ctx
                .b
                .call_expr("$.safe_get", [Arg::Ident(symbol_name)]),
            DeclarationSemantics::State(_) | DeclarationSemantics::Derived(_) => {
                self.ctx.b.call_expr("$.get", [Arg::Ident(symbol_name)])
            }
            DeclarationSemantics::Const(ConstDeclarationSemantics::ConstTag {
                destructured,
                ..
            }) => {
                if destructured {
                    self.ctx
                        .b
                        .call_expr("$.safe_get", [Arg::Ident(symbol_name)])
                } else {
                    self.ctx.b.call_expr("$.get", [Arg::Ident(symbol_name)])
                }
            }
            DeclarationSemantics::Contextual(_) => {
                self.ctx.b.call_expr("$.get", [Arg::Ident(symbol_name)])
            }
            DeclarationSemantics::NonReactive
            | DeclarationSemantics::Unresolved
            | DeclarationSemantics::OptimizedRune(_)
            | DeclarationSemantics::RuntimeRune { .. }
            | DeclarationSemantics::LetCarrier { .. } => self.ctx.b.rid_expr(symbol_name),
            DeclarationSemantics::Prop(_) => self.ctx.b.rid_expr(symbol_name),
        };
        Ok(expr)
    }
}
