use oxc_ast::ast::{
    AssignmentTarget, Class, ClassElement, Expression, MethodDefinitionKind, PropertyKey, Statement,
};
use rustc_hash::FxHashSet;
use svelte_diagnostics::{Diagnostic, DiagnosticKind};
use svelte_span::Span;

use svelte_component_semantics::OxcNodeId;

use crate::reactivity_semantics::data::ReactivitySemantics;
use crate::types::data::DeclaratorSemantics;

pub(super) fn is_class_field_rune(reactivity: &ReactivitySemantics, node: OxcNodeId) -> bool {
    match reactivity.declarator_semantics(node) {
        DeclaratorSemantics::ClassFieldState(_) | DeclaratorSemantics::ClassFieldDerived(_) => true,
        DeclaratorSemantics::None
        | DeclaratorSemantics::RuntimeRuneCall { .. }
        | DeclaratorSemantics::RuneProps
        | DeclaratorSemantics::LegacyProps
        | DeclaratorSemantics::LegacyState
        | DeclaratorSemantics::RuneState { .. }
        | DeclaratorSemantics::RuneDerived { .. }
        | DeclaratorSemantics::ConstTag { .. }
        | DeclaratorSemantics::LetCarrier { .. }
        | DeclaratorSemantics::EachItem
        | DeclaratorSemantics::AwaitValue
        | DeclaratorSemantics::SnippetParam => false,
    }
}

pub(super) fn check_class<'a>(
    class: &Class<'a>,
    reactivity: &ReactivitySemantics,
    diags: &mut Vec<Diagnostic>,
) {
    let mut declared: FxHashSet<&'a str> = FxHashSet::default();

    for element in &class.body.body {
        let ClassElement::PropertyDefinition(prop) = element else {
            continue;
        };
        if prop.r#static || prop.computed {
            continue;
        }
        let PropertyKey::StaticIdentifier(name_id) = &prop.key else {
            continue;
        };
        if !is_class_field_rune(reactivity, prop.node_id()) {
            continue;
        }
        declared.insert(name_id.name.as_str());
    }

    let Some(constructor) = class.body.body.iter().find_map(|el| match el {
        ClassElement::MethodDefinition(m) if m.kind == MethodDefinitionKind::Constructor => Some(m),
        _ => None,
    }) else {
        return;
    };
    let Some(body) = &constructor.value.body else {
        return;
    };
    for stmt in &body.statements {
        let Statement::ExpressionStatement(es) = stmt else {
            continue;
        };
        let Expression::AssignmentExpression(assign) = es.expression.get_inner_expression() else {
            continue;
        };
        let AssignmentTarget::StaticMemberExpression(member) = &assign.left else {
            continue;
        };
        if !matches!(
            member.object.get_inner_expression(),
            Expression::ThisExpression(_)
        ) {
            continue;
        }
        if !is_class_field_rune(reactivity, assign.node_id()) {
            continue;
        }
        let name = member.property.name.as_str();
        if declared.contains(name) {
            diags.push(Diagnostic::error(
                DiagnosticKind::StateFieldDuplicate {
                    name: name.to_string(),
                },
                Span::new(assign.span.start, assign.span.end),
            ));
        }
    }
}
