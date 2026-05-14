use oxc_ast::ast::{
    AccessorProperty, Declaration, Decorator, FormalParameter, MethodDefinition,
    MethodDefinitionKind, Program, Statement, TSEnumDeclaration, TSModuleDeclaration,
    TSModuleDeclarationBody,
};
use oxc_ast_visit::Visit;
use oxc_ast_visit::walk::walk_method_definition;
use svelte_diagnostics::{Diagnostic, DiagnosticKind};
use svelte_span::Span;

pub(super) fn validate(program: &Program<'_>, diags: &mut Vec<Diagnostic>) {
    let mut walker = TypescriptValidator {
        diags,
        in_constructor: false,
    };
    walker.visit_program(program);
}

struct TypescriptValidator<'a> {
    diags: &'a mut Vec<Diagnostic>,
    in_constructor: bool,
}

impl TypescriptValidator<'_> {
    fn push(&mut self, feature: &str, span: oxc_span::Span) {
        self.diags.push(Diagnostic::error(
            DiagnosticKind::TypescriptInvalidFeature {
                feature: feature.to_string(),
            },
            Span::new(span.start, span.end),
        ));
    }
}

impl<'a> Visit<'a> for TypescriptValidator<'_> {
    fn visit_method_definition(&mut self, it: &MethodDefinition<'a>) {
        let prev = self.in_constructor;
        if it.kind == MethodDefinitionKind::Constructor {
            self.in_constructor = true;
        }
        walk_method_definition(self, it);
        self.in_constructor = prev;
    }

    fn visit_formal_parameter(&mut self, it: &FormalParameter<'a>) {
        if self.in_constructor && (it.accessibility.is_some() || it.readonly) {
            self.push("accessibility modifiers on constructor parameters", it.span);
        }
    }

    fn visit_decorator(&mut self, it: &Decorator<'a>) {
        self.push("decorators (related TSC proposal is not stage 4 yet)", it.span);
    }

    fn visit_accessor_property(&mut self, it: &AccessorProperty<'a>) {
        self.push(
            "accessor fields (related TSC proposal is not stage 4 yet)",
            it.span,
        );
    }

    fn visit_ts_enum_declaration(&mut self, it: &TSEnumDeclaration<'a>) {
        self.push("enums", it.span);
    }

    fn visit_ts_module_declaration(&mut self, it: &TSModuleDeclaration<'a>) {
        if namespace_has_non_type_node(it) {
            self.push("namespaces with non-type nodes", it.span);
        }
    }
}

fn namespace_has_non_type_node(decl: &TSModuleDeclaration<'_>) -> bool {
    let Some(body) = &decl.body else {
        return false;
    };
    match body {
        TSModuleDeclarationBody::TSModuleDeclaration(nested) => {
            namespace_has_non_type_node(nested)
        }
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
