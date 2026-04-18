//! Shared helpers used across builder_v2 submodules.
//!
//! Contains AST drill-down utilities for member-expression LHS detection
//! (`foo.bar.baz = x` → root `foo`) and property-key atom extraction.

use oxc_ast::ast::{
    AssignmentTarget, Expression, PropertyKey, SimpleAssignmentTarget,
};
use oxc_span::Ident;
use svelte_component_semantics::ReferenceId;

use crate::scope::SymbolId;
use crate::types::data::AnalysisData;

pub(super) fn assignment_target_member_root_symbol(
    data: &AnalysisData,
    target: &AssignmentTarget<'_>,
) -> Option<SymbolId> {
    match target {
        AssignmentTarget::StaticMemberExpression(m) => expression_root_symbol(data, &m.object),
        AssignmentTarget::ComputedMemberExpression(m) => expression_root_symbol(data, &m.object),
        _ => None,
    }
}

pub(super) fn simple_assignment_target_member_root_symbol(
    data: &AnalysisData,
    target: &SimpleAssignmentTarget<'_>,
) -> Option<SymbolId> {
    match target {
        SimpleAssignmentTarget::StaticMemberExpression(m) => {
            expression_root_symbol(data, &m.object)
        }
        SimpleAssignmentTarget::ComputedMemberExpression(m) => {
            expression_root_symbol(data, &m.object)
        }
        _ => None,
    }
}

pub(super) fn expression_root_symbol(
    data: &AnalysisData,
    expr: &Expression<'_>,
) -> Option<SymbolId> {
    match expr {
        Expression::Identifier(id) => id
            .reference_id
            .get()
            .and_then(|ref_id| data.scoping.get_reference(ref_id).symbol_id()),
        Expression::StaticMemberExpression(m) => expression_root_symbol(data, &m.object),
        Expression::ComputedMemberExpression(m) => expression_root_symbol(data, &m.object),
        _ => None,
    }
}

pub(super) fn assignment_target_member_root_reference_id(
    target: &AssignmentTarget<'_>,
) -> Option<ReferenceId> {
    match target {
        AssignmentTarget::StaticMemberExpression(m) => expression_root_reference_id(&m.object),
        AssignmentTarget::ComputedMemberExpression(m) => expression_root_reference_id(&m.object),
        _ => None,
    }
}

pub(super) fn simple_assignment_target_member_root_reference_id(
    target: &SimpleAssignmentTarget<'_>,
) -> Option<ReferenceId> {
    match target {
        SimpleAssignmentTarget::StaticMemberExpression(m) => {
            expression_root_reference_id(&m.object)
        }
        SimpleAssignmentTarget::ComputedMemberExpression(m) => {
            expression_root_reference_id(&m.object)
        }
        _ => None,
    }
}

pub(super) fn expression_root_reference_id(expr: &Expression<'_>) -> Option<ReferenceId> {
    match expr {
        Expression::Identifier(id) => id.reference_id.get(),
        Expression::StaticMemberExpression(m) => expression_root_reference_id(&m.object),
        Expression::ComputedMemberExpression(m) => expression_root_reference_id(&m.object),
        _ => None,
    }
}

pub(super) fn property_key_atom<'a>(key: &PropertyKey<'a>) -> Option<Ident<'a>> {
    match key {
        PropertyKey::StaticIdentifier(ident) => Some(ident.name),
        PropertyKey::StringLiteral(lit) => Some(Ident::from(lit.value.as_str())),
        _ => None,
    }
}
