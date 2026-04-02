use super::*;

impl<'a> Builder<'a> {
    pub fn private_identifier(&self, name: &str) -> ast::PrivateIdentifier<'a> {
        self.ast.private_identifier(SPAN, self.ast.atom(name))
    }

    pub fn class_private_field(
        &self,
        name: &str,
        value: Option<Expression<'a>>,
    ) -> ast::ClassElement<'a> {
        let key = ast::PropertyKey::PrivateIdentifier(self.alloc(self.private_identifier(name)));
        ast::ClassElement::PropertyDefinition(self.alloc(self.ast.property_definition(
            SPAN,
            ast::PropertyDefinitionType::PropertyDefinition,
            self.ast.vec(),
            key,
            NONE,
            value,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            None,
        )))
    }

    pub fn class_getter(
        &self,
        key: ast::PropertyKey<'a>,
        body_stmts: Vec<Statement<'a>>,
    ) -> ast::ClassElement<'a> {
        let body =
            self.ast
                .alloc_function_body(SPAN, self.ast.vec(), self.ast.vec_from_iter(body_stmts));
        let func = self.ast.function(
            SPAN,
            FunctionType::FunctionExpression,
            None,
            false,
            false,
            false,
            NONE,
            NONE,
            self.no_params(),
            NONE,
            Some(body),
        );
        ast::ClassElement::MethodDefinition(self.alloc(self.ast.method_definition(
            SPAN,
            ast::MethodDefinitionType::MethodDefinition,
            self.ast.vec(),
            key,
            func,
            ast::MethodDefinitionKind::Get,
            false,
            false,
            false,
            false,
            None,
        )))
    }

    pub fn class_setter(
        &self,
        key: ast::PropertyKey<'a>,
        param_name: &str,
        body_stmts: Vec<Statement<'a>>,
    ) -> ast::ClassElement<'a> {
        let param_atom = self.ast.atom(param_name);
        let pattern = self
            .ast
            .binding_pattern_binding_identifier(SPAN, param_atom);
        let param = self.ast.formal_parameter(
            SPAN,
            self.ast.vec(),
            pattern,
            NONE,
            NONE,
            false,
            None,
            false,
            false,
        );
        let params = self.ast.formal_parameters(
            SPAN,
            ast::FormalParameterKind::FormalParameter,
            self.ast.vec_from_array([param]),
            NONE,
        );
        let body =
            self.ast
                .alloc_function_body(SPAN, self.ast.vec(), self.ast.vec_from_iter(body_stmts));
        let func = self.ast.function(
            SPAN,
            FunctionType::FunctionExpression,
            None,
            false,
            false,
            false,
            NONE,
            NONE,
            params,
            NONE,
            Some(body),
        );
        ast::ClassElement::MethodDefinition(self.alloc(self.ast.method_definition(
            SPAN,
            ast::MethodDefinitionType::MethodDefinition,
            self.ast.vec(),
            key,
            func,
            ast::MethodDefinitionKind::Set,
            false,
            false,
            false,
            false,
            None,
        )))
    }

    pub fn public_key(&self, name: &str) -> ast::PropertyKey<'a> {
        ast::PropertyKey::StaticIdentifier(
            self.alloc(self.ast.identifier_name(SPAN, self.ast.atom(name))),
        )
    }

    pub fn this_private_member(&self, name: &str) -> Expression<'a> {
        let this_expr = self.ast.expression_this(SPAN);
        let field = self.ast.private_field_expression(
            SPAN,
            this_expr,
            self.private_identifier(name),
            false,
        );
        Expression::PrivateFieldExpression(self.alloc(field))
    }
}
