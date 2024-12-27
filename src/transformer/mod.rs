use std::cell::Cell;

use oxc_allocator::{Allocator, Vec};
use oxc_ast::{
    ast::{
        self, BindingIdentifier, Declaration, ExportDefaultDeclaration,
        ExportDefaultDeclarationKind, Expression, Function, FunctionType,
        ImportDeclarationSpecifier, ImportOrExportKind, ModuleDeclaration, ModuleExportName,
        Program, Statement,
    },
    AstBuilder, NONE,
};
use oxc_span::{SourceType, SPAN};

use crate::ast::Ast;

pub mod builder;
pub mod visitor;

struct Builder<'a> {
    ast: AstBuilder<'a>,
}

impl<'a> Builder<'a> {
    fn import_all(&self, specifier: &'a str, source: &'a str) -> Statement<'a> {
        let spec = ImportDeclarationSpecifier::ImportNamespaceSpecifier(
            self.ast.alloc_import_namespace_specifier(
                SPAN,
                self.ast.binding_identifier(SPAN, specifier),
            ),
        );

        let stmt = Statement::from(self.ast.module_declaration_import_declaration(
            SPAN,
            Some(self.ast.vec_from_array([spec])),
            self.ast.string_literal(SPAN, source, None),
            None,
            NONE,
            ImportOrExportKind::Value,
        ));

        return stmt;
    }

    fn program(&self, body: Vec<'a, Statement<'a>>) -> Program<'a> {
        return Program {
            body,
            span: SPAN,
            comments: self.ast.vec(),
            directives: self.ast.vec(),
            hashbang: None,
            source_text: "",
            source_type: SourceType::default(),
            scope_id: Cell::from(None),
        };
    }

    fn function_declaration(
        &self,
        id: BindingIdentifier<'a>,
        body: Vec<'a, Statement<'a>>,
    ) -> Function<'a> {
        let params = self.ast.formal_parameters(
            SPAN,
            ast::FormalParameterKind::FormalParameter,
            self.ast.vec(),
            NONE,
        );

        let body = self.ast.alloc_function_body(SPAN, self.ast.vec(), body);

        let function = self.ast.function(
            SPAN,
            FunctionType::FunctionDeclaration,
            Some(id),
            false,
            false,
            false,
            NONE,
            NONE,
            params,
            NONE,
            Some(body),
        );

        return function;
    }

    fn id(&self, name: &'a str) -> BindingIdentifier<'a> {
        return self.ast.binding_identifier(SPAN, name);
    }

    fn export_default(&self, declaration: ExportDefaultDeclarationKind<'a>) -> Statement<'a> {
        let ident = self.ast.identifier_name(SPAN, "default");

        let res = self.ast.alloc_export_default_declaration(
            SPAN,
            declaration,
            ModuleExportName::IdentifierName(ident),
        );

        let res = ModuleDeclaration::ExportDefaultDeclaration(res);

        return Statement::from(res);
    }
}

pub fn transform_client<'a>(_ast: &Ast<'a>) -> String {
    let allocator = Allocator::default();
    let ast_builder = AstBuilder::new(&allocator);
    let b = Builder { ast: ast_builder };

    let mut imports = ast_builder.vec();
    let mut program_body = ast_builder.vec();
    let mut component_body = ast_builder.vec();
    let mut instance_body = ast_builder.vec();
    let mut template_body = ast_builder.vec();

    imports.push(b.import_all("$", "svelte/internal/client"));

    program_body.append(&mut imports);

    component_body.append(&mut instance_body);
    component_body.append(&mut template_body);

    let component = b.function_declaration(b.id("App"), component_body);

    program_body.push(
        b.export_default(ExportDefaultDeclarationKind::FunctionDeclaration(
            ast_builder.alloc(component),
        )),
    );

    return print(b.program(program_body));
}

fn print(program: Program) -> String {
    let codegen = oxc_codegen::Codegen::default();

    let result = codegen.build(&program);

    return result.code;
}

#[cfg(test)]
mod tests {
    use oxc_allocator::Allocator;

    use crate::parser::Parser;

    use super::*;

    #[test]
    fn smoke() {
        let allocator = Allocator::default();
        let mut parser = Parser::new("prefix <div>text</div>", &allocator);
        let ast = parser.parse().unwrap();

        let code = transform_client(&ast);

        print!("{}", code);
    }
}
