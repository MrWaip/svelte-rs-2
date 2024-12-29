use std::cell::Cell;

use oxc_allocator::Box;
use oxc_ast::{
    ast::{
        self, Argument, BindingIdentifier, BindingPatternKind, CallExpression,
        ExportDefaultDeclarationKind, Expression, FormalParameters, Function, FunctionType,
        IdentifierReference, ImportDeclarationSpecifier, ImportOrExportKind, ModuleDeclaration,
        ModuleExportName, NumericLiteral, Program, Statement, StringLiteral,
        VariableDeclarationKind,
    },
    AstBuilder, NONE,
};
use oxc_span::{SourceType, SPAN};

pub struct Builder<'a> {
    pub ast: AstBuilder<'a>,
}

impl<'a> Builder<'a> {
    pub fn new(ast: AstBuilder<'a>) -> Builder<'a> {
        return Builder { ast };
    }

    pub fn import_all(&self, specifier: &'a str, source: &'a str) -> Statement<'a> {
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

    pub fn program(&self, body: std::vec::Vec<Statement<'a>>) -> Program<'a> {
        return Program {
            body: self.ast.vec_from_iter(body),
            span: SPAN,
            comments: self.ast.vec(),
            directives: self.ast.vec(),
            hashbang: None,
            source_text: "",
            source_type: SourceType::default(),
            scope_id: Cell::from(None),
        };
    }

    pub fn params(&self, items: std::vec::Vec<&str>) -> FormalParameters<'a> {
        let params = items.iter().map(|v| {
            let kind = self.ast.binding_pattern_kind_binding_identifier(SPAN, *v);

            let pattern = self.ast.binding_pattern(kind, NONE, false);

            self.ast
                .formal_parameter(SPAN, self.ast.vec(), pattern, None, false, false)
        });

        return self.ast.formal_parameters(
            SPAN,
            ast::FormalParameterKind::FormalParameter,
            self.ast.vec_from_iter(params),
            NONE,
        );
    }

    pub fn function_declaration(
        &self,
        id: BindingIdentifier<'a>,
        body: std::vec::Vec<Statement<'a>>,
        params: FormalParameters<'a>,
    ) -> Function<'a> {
        let body = self
            .ast
            .alloc_function_body(SPAN, self.ast.vec(), self.ast.vec_from_iter(body));

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

    pub fn bid(&self, name: &'a str) -> BindingIdentifier<'a> {
        return self.ast.binding_identifier(SPAN, name);
    }

    pub fn rid(&self, name: &'a str) -> IdentifierReference<'a> {
        return self.ast.identifier_reference(SPAN, name);
    }

    pub fn export_default(&self, declaration: ExportDefaultDeclarationKind<'a>) -> Statement<'a> {
        let ident = self.ast.identifier_name(SPAN, "default");

        let res = self.ast.alloc_export_default_declaration(
            SPAN,
            declaration,
            ModuleExportName::IdentifierName(ident),
        );

        let res = ModuleDeclaration::ExportDefaultDeclaration(res);

        return Statement::from(res);
    }

    pub fn string_literal(&self, value: String) -> StringLiteral<'a> {
        return self.ast.string_literal(SPAN, value, None);
    }

    pub fn numeric_literal(&self, value: f64) -> NumericLiteral<'a> {
        return self
            .ast
            .numeric_literal(SPAN, value, None, ast::NumberBase::Decimal);
    }

    pub fn call(&self, callee: &str, args: std::vec::Vec<Argument<'a>>) -> CallExpression<'a> {
        let ident = self.ast.expression_identifier_reference(SPAN, callee);
        let res = self
            .ast
            .call_expression(SPAN, ident, NONE, self.ast.vec_from_iter(args), false);

        return res;
    }

    pub fn alloc<T>(&self, value: T) -> Box<'a, T> {
        return self.ast.alloc(value);
    }

    pub fn var(&self, name: &str, init: Expression<'a>) -> Statement<'a> {
        let ident = self.ast.binding_identifier(SPAN, name);
        let pattern = self.ast.binding_pattern(
            BindingPatternKind::BindingIdentifier(self.alloc(ident)),
            NONE,
            false,
        );

        let decl = self.ast.variable_declarator(
            SPAN,
            VariableDeclarationKind::Var,
            pattern,
            Some(init),
            false,
        );

        let declaration = self.ast.variable_declaration(
            SPAN,
            VariableDeclarationKind::Var,
            self.ast.vec_from_array([decl]),
            false,
        );

        return Statement::VariableDeclaration(self.alloc(declaration));
    }
}
