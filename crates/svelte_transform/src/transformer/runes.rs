use oxc_ast::ast::Expression;
use oxc_traverse::{Ancestor, TraverseCtx};

use svelte_analyze::{RuneKind, detect_rune_from_call};

use super::model::ComponentTransformer;

impl<'a> ComponentTransformer<'_, 'a> {
    pub(crate) fn rewrite_call_expression(&mut self, node: &mut Expression<'a>) {
        if !self
            .analysis
            .as_ref()
            .is_some_and(|a| a.uses_runes())
        {
            return;
        }
        let Expression::CallExpression(call) = &*node else {
            return;
        };
        let Some(rune_kind) = detect_rune_from_call(call) else {
            return;
        };

        if matches!(rune_kind, RuneKind::Host) {
            *node = self
                .b
                .static_member_expr(self.b.rid_expr("$$props"), "$$host");
            return;
        }

        let dev_snapshot_ignored =
            self.dev && self.is_in_ignored_stmt("state_snapshot_uncloneable");
        if self.rewrite_shared_call(node, dev_snapshot_ignored) {
            return;
        }

        let new_callee = match rune_kind {
            RuneKind::Effect => Some("$.user_effect"),
            RuneKind::EffectPre => Some("$.user_pre_effect"),
            RuneKind::EffectRoot => Some("$.effect_root"),
            RuneKind::EffectTracking => Some("$.effect_tracking"),
            _ => None,
        };
        if let Some(callee_name) = new_callee {
            let Expression::CallExpression(call) = node else {
                unreachable!()
            };
            call.callee = self.b.rid_expr(callee_name);
        }
    }

    pub(crate) fn rewrite_static_member_expression(
        &mut self,
        node: &mut Expression<'a>,
        ctx: &mut TraverseCtx<'a, ()>,
    ) {
        if self.analysis.is_none() {
            return;
        }
        let is_lhs = matches!(
            ctx.parent(),
            Ancestor::AssignmentExpressionLeft(_) | Ancestor::UpdateExpressionArgument(_)
        );
        self.rewrite_rest_prop_member(node, is_lhs);
    }

    pub(crate) fn rewrite_identifier_expression(&mut self, node: &mut Expression<'a>) {
        let _ = self.dispatch_identifier_read(node);
    }
}
