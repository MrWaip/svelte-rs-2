use oxc_ast::ast::{BindingPattern, Expression, VariableDeclarator};
use svelte_ast_builder::{Arg, Builder};
use svelte_emit_builders::runtime::is_simple_expression;

use crate::model::ServerTransform;

impl<'a> ServerTransform<'_, 'a> {
    pub(crate) fn rewrite_legacy_prop(&mut self, declarator: &mut VariableDeclarator<'a>) {
        let BindingPattern::BindingIdentifier(id) = &declarator.id else {
            return;
        };
        let Some(symbol) = id.symbol_id.get() else {
            return;
        };
        let key = self
            .analysis
            .reactivity
            .legacy_bindable_prop_alias(symbol)
            .map(str::to_string)
            .unwrap_or_else(|| self.analysis.scoping.symbol_name(symbol).to_string());
        let prop = self
            .b
            .computed_member_expr(self.b.rid_expr("$$props"), self.b.str_expr(&key));
        let init = match declarator.init.as_mut() {
            Some(default) => {
                let default = self.b.move_expr(default);
                build_fallback_legacy(self.b, prop, default)
            }
            None => prop,
        };
        declarator.init = Some(init);
    }
}

fn build_fallback_legacy<'a>(
    b: &Builder<'a>,
    prop: Expression<'a>,
    fallback: Expression<'a>,
) -> Expression<'a> {
    if is_simple_expression(&fallback) {
        return b.call_expr("$.fallback", [Arg::Expr(prop), Arg::Expr(fallback)]);
    }
    let thunk = b.thunk(fallback);
    b.call_expr(
        "$.fallback",
        [Arg::Expr(prop), Arg::Expr(thunk), Arg::Bool(true)],
    )
}
