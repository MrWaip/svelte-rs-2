use oxc_ast::ast::{BindingIdentifier, BindingPattern, Statement, VariableDeclarator};
use svelte_component_semantics::OxcNodeId;

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
