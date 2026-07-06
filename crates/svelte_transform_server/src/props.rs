use std::mem;

use oxc_ast::ast::{
    Argument, BindingPattern, BindingProperty, Expression, PropertyKey, VariableDeclarator,
};
use oxc_span::SPAN;
use svelte_ast_builder::{Arg, Builder};
use svelte_emit_builders::runtime::is_simple_expression;

use crate::model::ServerTransform;

impl<'a> ServerTransform<'_, 'a> {
    pub(crate) fn rewrite_rune_props(&mut self, declarator: &mut VariableDeclarator<'a>) {
        self.erase_bindable(&mut declarator.id);

        match &mut declarator.id {
            BindingPattern::ObjectPattern(obj) => {
                if obj.rest.is_some() {
                    let slots = self.shorthand_binding_property("$$slots");
                    let events = self.shorthand_binding_property("$$events");
                    obj.properties.push(slots);
                    obj.properties.push(events);
                }
            }
            BindingPattern::BindingIdentifier(id) => {
                let rest_pattern = self.b.ast.binding_pattern_binding_identifier(SPAN, id.name);
                let rest = self.b.ast.binding_rest_element(SPAN, rest_pattern);
                let mut properties = self.b.ast.vec();
                properties.push(self.shorthand_binding_property("$$slots"));
                properties.push(self.shorthand_binding_property("$$events"));
                let object =
                    self.b
                        .ast
                        .object_pattern(SPAN, properties, Some(self.b.ast.alloc(rest)));
                declarator.id = BindingPattern::ObjectPattern(self.b.ast.alloc(object));
            }
            _ => {}
        }

        declarator.init = Some(self.b.rid_expr("$$props"));
    }

    fn shorthand_binding_property(&self, name: &str) -> BindingProperty<'a> {
        let atom = self.b.ast.atom(name);
        let key =
            PropertyKey::StaticIdentifier(self.b.alloc(self.b.ast.identifier_name(SPAN, atom)));
        let value = self.b.ast.binding_pattern_binding_identifier(SPAN, atom);
        self.b.ast.binding_property(SPAN, key, value, true, false)
    }

    fn erase_bindable(&self, pattern: &mut BindingPattern<'a>) {
        let BindingPattern::ObjectPattern(obj) = pattern else {
            return;
        };
        for prop in obj.properties.iter_mut() {
            let BindingPattern::AssignmentPattern(assign) = &mut prop.value else {
                continue;
            };
            let BindingPattern::BindingIdentifier(id) = &assign.left else {
                continue;
            };
            let Some(symbol) = id.symbol_id.get() else {
                continue;
            };
            if !self.analysis.binding_semantics(symbol).is_bindable() {
                continue;
            }
            self.unwrap_bindable_default(&mut assign.right);
        }
    }

    fn unwrap_bindable_default(&self, right: &mut Expression<'a>) {
        let replaced = mem::replace(right, self.b.void_zero_expr());
        let Expression::CallExpression(call) = replaced.into_inner_expression() else {
            return;
        };
        let mut call = call.unbox();
        match call.arguments.first_mut() {
            Some(arg) if !matches!(arg, Argument::SpreadElement(_)) => {
                let mut taken = Argument::from(self.b.cheap_expr());
                mem::swap(arg, &mut taken);
                *right = taken.into_expression();
            }
            _ => {}
        }
    }
}

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
