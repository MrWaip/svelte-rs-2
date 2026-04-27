//! Shared helpers used across builder_v2 submodules.
//!
//! Contains AST drill-down utilities for member-expression LHS detection
//! (`foo.bar.baz = x` → root `foo`) and property-key atom extraction.

use oxc_ast::ast::{AssignmentTarget, Expression, PropertyKey, SimpleAssignmentTarget};
use oxc_span::Ident;
use svelte_component_semantics::ReferenceId;

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
    // Returns a `'a`-bound `Ident` (the `oxc_span::Atom<'a>`), which the shared
    // `property_key_static_name` cannot — its return type is a borrowed `&str`.
    // Logic stays in lock-step with the shared helper: StaticIdentifier and
    // StringLiteral are recognized; computed keys return `None`.
    match key {
        PropertyKey::StaticIdentifier(ident) => Some(ident.name),
        PropertyKey::StringLiteral(lit) => Some(Ident::from(lit.value.as_str())),
        _ => None,
    }
}
