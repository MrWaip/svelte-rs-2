use std::iter::empty;
use std::mem;

use oxc_ast::ast::{Argument, CallExpression, Expression};
use svelte_analyze::{DeclaratorSemantics, RuntimeRuneKind, WarningCode};
use svelte_ast_builder::Arg;

use crate::model::ServerTransform;

impl<'a> ServerTransform<'_, 'a> {
    pub(crate) fn rewrite_runtime_rune_call(&self, expr: &mut Expression<'a>) -> bool {
        let Expression::CallExpression(call) = expr else {
            return false;
        };
        let DeclaratorSemantics::RuntimeRuneCall { kind } =
            self.analysis.declarator_semantics(call.node_id())
        else {
            return false;
        };
        let Some(replacement) = self.runtime_rune_value(call, kind) else {
            return false;
        };
        *expr = replacement;
        true
    }

    pub(crate) fn elide_state_snapshot_init(&self, declarator_init: &mut Expression<'a>) -> bool {
        let Expression::CallExpression(call) = declarator_init else {
            return false;
        };
        if !matches!(
            self.analysis.declarator_semantics(call.node_id()),
            DeclaratorSemantics::RuntimeRuneCall {
                kind: RuntimeRuneKind::StateSnapshot,
            }
        ) {
            return false;
        }
        *declarator_init = self.take_rune_arg(call);
        true
    }

    fn runtime_rune_value(
        &self,
        call: &mut CallExpression<'a>,
        kind: RuntimeRuneKind,
    ) -> Option<Expression<'a>> {
        match kind {
            RuntimeRuneKind::EffectTracking => Some(self.b.bool_expr(false)),
            RuntimeRuneKind::EffectPending => Some(self.b.num_expr(0.0)),
            RuntimeRuneKind::EffectRoot => Some(self.b.arrow_expr(self.b.no_params(), empty())),
            RuntimeRuneKind::StateEager => Some(self.take_rune_arg(call)),
            RuntimeRuneKind::StateSnapshot => {
                let suppress_warning =
                    self.dev && self.is_in_ignored_stmt(WarningCode::StateSnapshotUncloneable);
                let arg = self.take_rune_arg(call);
                let args = if suppress_warning {
                    vec![Arg::Expr(arg), Arg::Expr(self.b.bool_expr(true))]
                } else {
                    vec![Arg::Expr(arg)]
                };
                Some(self.b.call_expr("$.snapshot", args))
            }
            RuntimeRuneKind::Host
            | RuntimeRuneKind::Effect
            | RuntimeRuneKind::EffectPre
            | RuntimeRuneKind::InspectTrace => Some(self.b.void_zero_expr()),
            RuntimeRuneKind::PropsId => {
                Some(self.b.call_expr("$.props_id", [Arg::Ident("$$renderer")]))
            }
            RuntimeRuneKind::Inspect | RuntimeRuneKind::InspectWith | RuntimeRuneKind::Bindable => {
                None
            }
        }
    }

    fn take_rune_arg(&self, call: &mut CallExpression<'a>) -> Expression<'a> {
        let Some(first) = call.arguments.first_mut() else {
            return self.b.void_zero_expr();
        };
        if matches!(first, Argument::SpreadElement(_)) {
            return self.b.void_zero_expr();
        }
        let mut taken = Argument::from(self.b.cheap_expr());
        mem::swap(first, &mut taken);
        taken.into_expression()
    }
}
