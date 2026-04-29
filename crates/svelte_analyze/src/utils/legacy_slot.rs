use oxc_ast::ast::{BindingPattern, Statement, VariableDeclarationKind, VariableDeclarator};

pub(crate) fn legacy_slot_declarator<'a>(
    stmt: &'a Statement<'a>,
) -> Option<&'a VariableDeclarator<'a>> {
    let Statement::VariableDeclaration(decl) = stmt else {
        return None;
    };
    if decl.kind != VariableDeclarationKind::Const {
        return None;
    }
    decl.declarations.first()
}

pub(crate) fn legacy_slot_pattern<'a>(stmt: &'a Statement<'a>) -> Option<&'a BindingPattern<'a>> {
    Some(&legacy_slot_declarator(stmt)?.id)
}
