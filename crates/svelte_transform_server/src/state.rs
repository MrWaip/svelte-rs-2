use std::mem;

use oxc_ast::ast::{Argument, BindingPattern, CallExpression, Expression, VariableDeclarator};
use svelte_analyze::StateKind;

use crate::model::ServerTransform;

impl<'a> ServerTransform<'_, 'a> {
    pub(crate) fn rewrite_state(
        &mut self,
        declarator: &mut VariableDeclarator<'a>,
        kind: StateKind,
    ) {
        match kind {
            StateKind::State | StateKind::StateRaw | StateKind::StateEager => {}
        }
        if !matches!(&declarator.id, BindingPattern::BindingIdentifier(_)) {
            return;
        }
        let Some(init) = declarator.init.as_mut() else {
            return;
        };
        let Expression::CallExpression(call) = init else {
            return;
        };
        let Some(value) = self.take_first_argument(call) else {
            return;
        };
        declarator.init = Some(value);
    }

    fn take_first_argument(&self, call: &mut CallExpression<'a>) -> Option<Expression<'a>> {
        let Some(first) = call.arguments.first_mut() else {
            return Some(self.b.void_zero_expr());
        };
        if matches!(first, Argument::SpreadElement(_)) {
            return None;
        }
        let mut taken = Argument::from(self.b.cheap_expr());
        mem::swap(first, &mut taken);
        Some(taken.into_expression())
    }
}
