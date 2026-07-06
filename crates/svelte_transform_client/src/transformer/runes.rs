use oxc_ast::ast::Expression;
use oxc_traverse::{Ancestor, TraverseCtx};

use svelte_analyze::{DeclaratorSemantics, RuntimeRuneKind};

use super::model::ComponentTransformer;

impl<'a> ComponentTransformer<'_, 'a> {
    pub(crate) fn rewrite_call_expression(&mut self, node: &mut Expression<'a>) {
        let Some(analysis) = self.analysis else {
            return;
        };
        let Expression::CallExpression(call) = &*node else {
            return;
        };
        let kind = match analysis.declarator_semantics(call.node_id()) {
            DeclaratorSemantics::RuntimeRuneCall { kind } => kind,
            DeclaratorSemantics::None
            | DeclaratorSemantics::RuneProps
            | DeclaratorSemantics::LegacyProps
            | DeclaratorSemantics::LegacyState
            | DeclaratorSemantics::RuneState { .. }
            | DeclaratorSemantics::RuneDerived { .. }
            | DeclaratorSemantics::ConstTag { .. }
            | DeclaratorSemantics::LetCarrier { .. }
            | DeclaratorSemantics::EachItem
            | DeclaratorSemantics::AwaitValue
            | DeclaratorSemantics::SnippetParam
            | DeclaratorSemantics::ClassFieldState(_)
            | DeclaratorSemantics::ClassFieldDerived(_) => return,
        };

        if matches!(kind, RuntimeRuneKind::Host) {
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

        let new_callee = match kind {
            RuntimeRuneKind::Effect => Some("$.user_effect"),
            RuntimeRuneKind::EffectPre => Some("$.user_pre_effect"),
            RuntimeRuneKind::EffectRoot => Some("$.effect_root"),
            RuntimeRuneKind::EffectTracking => Some("$.effect_tracking"),
            RuntimeRuneKind::PropsId
            | RuntimeRuneKind::EffectPending
            | RuntimeRuneKind::Host
            | RuntimeRuneKind::Inspect
            | RuntimeRuneKind::InspectWith
            | RuntimeRuneKind::InspectTrace
            | RuntimeRuneKind::StateSnapshot
            | RuntimeRuneKind::StateEager
            | RuntimeRuneKind::Bindable => None,
        };
        if let Some(callee_name) = new_callee {
            let Expression::CallExpression(call) = node else {
                unreachable!()
            };
            call.callee = self.b.rid_expr(callee_name);
        }
    }

    pub(crate) fn rewrite_member_expression(
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
