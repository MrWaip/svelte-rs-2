use oxc_ast::ast::{Declaration, Statement, TSModuleDeclaration, TSModuleDeclarationBody};

pub(super) fn namespace_has_non_type_node(decl: &TSModuleDeclaration<'_>) -> bool {
    let Some(body) = &decl.body else {
        return false;
    };
    match body {
        TSModuleDeclarationBody::TSModuleDeclaration(nested) => namespace_has_non_type_node(nested),
        TSModuleDeclarationBody::TSModuleBlock(block) => {
            block.body.iter().any(|stmt| !is_type_only_statement(stmt))
        }
    }
}

fn is_type_only_statement(stmt: &Statement<'_>) -> bool {
    match stmt {
        Statement::TSTypeAliasDeclaration(_)
        | Statement::TSInterfaceDeclaration(_)
        | Statement::TSImportEqualsDeclaration(_)
        | Statement::TSExportAssignment(_)
        | Statement::TSNamespaceExportDeclaration(_) => true,
        Statement::TSModuleDeclaration(decl) => !namespace_has_non_type_node(decl),
        Statement::VariableDeclaration(d) => d.declare,
        Statement::FunctionDeclaration(f) => f.declare,
        Statement::ClassDeclaration(c) => c.declare,
        Statement::ImportDeclaration(i) => i.import_kind.is_type(),
        Statement::ExportNamedDeclaration(e) => {
            if e.export_kind.is_type() {
                return true;
            }
            if let Some(decl) = &e.declaration {
                return match decl {
                    Declaration::TSTypeAliasDeclaration(_)
                    | Declaration::TSInterfaceDeclaration(_)
                    | Declaration::TSImportEqualsDeclaration(_) => true,
                    Declaration::TSModuleDeclaration(m) => !namespace_has_non_type_node(m),
                    Declaration::VariableDeclaration(v) => v.declare,
                    Declaration::FunctionDeclaration(f) => f.declare,
                    Declaration::ClassDeclaration(c) => c.declare,
                    _ => false,
                };
            }
            !e.specifiers.is_empty() && e.specifiers.iter().all(|s| s.export_kind.is_type())
        }
        Statement::ExportAllDeclaration(e) => e.export_kind.is_type(),
        _ => false,
    }
}
