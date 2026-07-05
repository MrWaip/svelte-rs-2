use oxc_ast::ast::{Expression, Statement};
use svelte_analyze::{DeclaratorSemantics, RuntimeRuneKind};

use crate::model::ServerTransform;

impl ServerTransform<'_, '_> {
    pub(crate) fn keep_statement(&self, stmt: &Statement<'_>) -> bool {
        let Statement::ExpressionStatement(es) = stmt else {
            return true;
        };
        let Expression::CallExpression(call) = &es.expression else {
            return true;
        };
        match self.analysis.declarator_semantics(call.node_id()) {
            DeclaratorSemantics::RuntimeRuneCall { kind } => match kind {
                RuntimeRuneKind::Effect
                | RuntimeRuneKind::EffectPre
                | RuntimeRuneKind::EffectRoot
                | RuntimeRuneKind::InspectTrace => false,
                RuntimeRuneKind::PropsId
                | RuntimeRuneKind::EffectTracking
                | RuntimeRuneKind::EffectPending
                | RuntimeRuneKind::Host
                | RuntimeRuneKind::Inspect
                | RuntimeRuneKind::InspectWith
                | RuntimeRuneKind::StateSnapshot
                | RuntimeRuneKind::StateEager
                | RuntimeRuneKind::Bindable => true,
            },
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
            | DeclaratorSemantics::ClassFieldDerived(_) => true,
        }
    }
}
