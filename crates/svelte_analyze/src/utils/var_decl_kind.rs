use oxc_ast::ast::VariableDeclarationKind;

pub fn is_let_or_var(kind: VariableDeclarationKind) -> bool {
    matches!(
        kind,
        VariableDeclarationKind::Let | VariableDeclarationKind::Var
    )
}
