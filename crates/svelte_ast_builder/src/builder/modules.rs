use super::*;

impl<'a> Builder<'a> {
    pub fn parse_expression(&self, text: &str) -> Expression<'a> {
        let alloc = self.ast.allocator;
        let arena_text: &'a str = alloc.alloc_str(text);
        match OxcParser::new(alloc, arena_text, SourceType::default()).parse_expression() {
            Ok(expr) => expr,
            Err(_) => {
                debug_assert!(false, "codegen: failed to parse expression: {text}");
                self.str_expr(text)
            }
        }
    }

    pub fn bare_import(&self, source: &str) -> Statement<'a> {
        let source_str = self.alloc_str(source);
        Statement::from(self.ast.module_declaration_import_declaration(
            SPAN,
            None,
            self.ast.string_literal(SPAN, source_str, None),
            None,
            NONE,
            ImportOrExportKind::Value,
        ))
    }

    pub fn import_all(&self, specifier: &str, source: &str) -> Statement<'a> {
        let spec_str = self.alloc_str(specifier);
        let source_str = self.alloc_str(source);
        let spec = ImportDeclarationSpecifier::ImportNamespaceSpecifier(
            self.ast.alloc_import_namespace_specifier(
                SPAN,
                self.ast.binding_identifier(SPAN, spec_str),
            ),
        );
        Statement::from(self.ast.module_declaration_import_declaration(
            SPAN,
            Some(self.ast.vec_from_array([spec])),
            self.ast.string_literal(SPAN, source_str, None),
            None,
            NONE,
            ImportOrExportKind::Value,
        ))
    }

    pub fn export_default(&self, declaration: ExportDefaultDeclarationKind<'a>) -> Statement<'a> {
        let res = self.ast.alloc_export_default_declaration(SPAN, declaration);
        Statement::from(ModuleDeclaration::ExportDefaultDeclaration(res))
    }

    pub fn program(
        &self,
        body: Vec<Statement<'a>>,
        comments: Vec<oxc_ast::Comment>,
        source_text: &'a str,
        span_end: u32,
    ) -> Program<'a> {
        self.ast.program(
            Span::new(0, span_end),
            SourceType::mjs(),
            source_text,
            self.ast.vec_from_iter(comments),
            None,
            self.ast.vec(),
            self.ast.vec_from_iter(body),
        )
    }
}
