use oxc_ast::ast::{BindingIdentifier, BindingPattern, Statement, VariableDeclarator};
use smallvec::SmallVec;
use svelte_component_semantics::OxcNodeId;

use super::super::FragmentDeclarationAsyncKind;
use crate::expression_semantics::{ExpressionSemantics, Volatility};

pub(super) fn async_kind_from_expression(
    sem: &ExpressionSemantics,
) -> FragmentDeclarationAsyncKind {
    let ExpressionSemantics::Expression(data) = sem else {
        return FragmentDeclarationAsyncKind::Sync;
    };
    let blockers = data.blockers.clone();
    match data.volatility {
        Volatility::Asynchronous => FragmentDeclarationAsyncKind::Awaited {
            blockers,
            declaration_blockers: SmallVec::new(),
        },
        Volatility::Static | Volatility::Reactive | Volatility::Heavy => {
            if blockers.is_empty() {
                FragmentDeclarationAsyncKind::Sync
            } else {
                FragmentDeclarationAsyncKind::Deferred {
                    blockers,
                    declaration_blockers: SmallVec::new(),
                }
            }
        }
    }
}

pub(super) fn declarator_from_stmt<'a>(
    stmt: &'a Statement<'a>,
) -> Option<&'a VariableDeclarator<'a>> {
    let Statement::VariableDeclaration(decl) = stmt else {
        return None;
    };
    decl.declarations.first()
}

pub(super) fn binding_ident_of<'a>(
    pattern: &'a BindingPattern<'a>,
) -> Option<&'a BindingIdentifier<'a>> {
    match pattern {
        BindingPattern::BindingIdentifier(ident) => Some(ident),
        _ => None,
    }
}

pub(super) fn binding_pattern_node_id(pattern: &BindingPattern<'_>) -> OxcNodeId {
    match pattern {
        BindingPattern::BindingIdentifier(p) => p.node_id(),
        BindingPattern::ObjectPattern(p) => p.node_id(),
        BindingPattern::ArrayPattern(p) => p.node_id(),
        BindingPattern::AssignmentPattern(p) => p.node_id(),
    }
}
