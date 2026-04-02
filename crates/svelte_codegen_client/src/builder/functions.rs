use super::*;

impl<'a> Builder<'a> {
    pub fn no_params(&self) -> FormalParameters<'a> {
        self.params(std::iter::empty::<&str>())
    }

    pub fn params(&self, items: impl IntoIterator<Item = impl AsRef<str>>) -> FormalParameters<'a> {
        let params: Vec<_> = items
            .into_iter()
            .map(|v| {
                let atom = self.ast.atom(v.as_ref());
                let pattern = self.ast.binding_pattern_binding_identifier(SPAN, atom);
                self.ast.formal_parameter(
                    SPAN,
                    self.ast.vec(),
                    pattern,
                    NONE,
                    NONE,
                    false,
                    None,
                    false,
                    false,
                )
            })
            .collect();
        self.ast.formal_parameters(
            SPAN,
            ast::FormalParameterKind::FormalParameter,
            self.ast.vec_from_iter(params),
            NONE,
        )
    }

    pub fn formal_parameters(
        &self,
        items: impl IntoIterator<Item = ast::FormalParameter<'a>>,
    ) -> FormalParameters<'a> {
        self.ast.formal_parameters(
            SPAN,
            ast::FormalParameterKind::FormalParameter,
            self.ast.vec_from_iter(items),
            NONE,
        )
    }

    pub fn formal_parameter_from_pattern(
        &self,
        pattern: ast::BindingPattern<'a>,
    ) -> ast::FormalParameter<'a> {
        self.ast.formal_parameter(
            SPAN,
            self.ast.vec(),
            pattern,
            NONE,
            NONE,
            false,
            None,
            false,
            false,
        )
    }

    pub fn formal_parameter_from_str(&self, name: &str) -> ast::FormalParameter<'a> {
        let atom = self.ast.atom(name);
        let pattern = self.ast.binding_pattern_binding_identifier(SPAN, atom);
        self.ast.formal_parameter(
            SPAN,
            self.ast.vec(),
            pattern,
            NONE,
            NONE,
            false,
            None,
            false,
            false,
        )
    }

    pub fn arrow(
        &self,
        params: FormalParameters<'a>,
        statements: impl IntoIterator<Item = Statement<'a>>,
    ) -> ArrowFunctionExpression<'a> {
        let statements: Vec<_> = statements.into_iter().collect();
        let is_expr =
            statements.len() == 1 && matches!(&statements[0], Statement::ExpressionStatement(_));
        let body = self
            .ast
            .function_body(SPAN, self.ast.vec(), self.ast.vec_from_iter(statements));
        self.ast
            .arrow_function_expression(SPAN, is_expr, false, NONE, params, NONE, body)
    }

    pub fn arrow_block(
        &self,
        params: FormalParameters<'a>,
        statements: impl IntoIterator<Item = Statement<'a>>,
    ) -> ArrowFunctionExpression<'a> {
        let body = self
            .ast
            .function_body(SPAN, self.ast.vec(), self.ast.vec_from_iter(statements));
        self.ast
            .arrow_function_expression(SPAN, false, false, NONE, params, NONE, body)
    }

    pub fn arrow_block_expr(
        &self,
        params: FormalParameters<'a>,
        statements: impl IntoIterator<Item = Statement<'a>>,
    ) -> Expression<'a> {
        Expression::ArrowFunctionExpression(self.alloc(self.arrow_block(params, statements)))
    }

    pub fn arrow_expr(
        &self,
        params: FormalParameters<'a>,
        statements: impl IntoIterator<Item = Statement<'a>>,
    ) -> Expression<'a> {
        Expression::ArrowFunctionExpression(self.alloc(self.arrow(params, statements)))
    }

    pub fn thunk(&self, expr: Expression<'a>) -> Expression<'a> {
        if let Expression::CallExpression(call) = &expr {
            if call.arguments.is_empty()
                && !call.optional
                && matches!(
                    &call.callee,
                    Expression::Identifier(_) | Expression::StaticMemberExpression(_)
                )
            {
                if let Expression::CallExpression(call) = expr {
                    let call = call.unbox();
                    return call.callee;
                }
            }
        }
        self.arrow_expr(self.no_params(), [self.expr_stmt(expr)])
    }

    pub fn thunk_block(&self, stmts: Vec<Statement<'a>>) -> Expression<'a> {
        self.arrow_block_expr(self.no_params(), stmts)
    }

    pub fn async_thunk(&self, expr: Expression<'a>) -> Expression<'a> {
        if let Expression::AwaitExpression(await_node) = expr {
            let inner = await_node.unbox().argument;
            return self.thunk(inner);
        }
        let await_expr = self.await_expr(expr);
        let body = self.ast.function_body(
            SPAN,
            self.ast.vec(),
            self.ast.vec_from_iter([self.expr_stmt(await_expr)]),
        );
        Expression::ArrowFunctionExpression(self.alloc(self.ast.arrow_function_expression(
            SPAN,
            true,
            true,
            NONE,
            self.no_params(),
            NONE,
            body,
        )))
    }

    pub fn await_expr(&self, expr: Expression<'a>) -> Expression<'a> {
        self.ast.expression_await(SPAN, expr)
    }

    pub fn async_arrow_expr_body(&self, expr: Expression<'a>) -> Expression<'a> {
        let body = self.ast.function_body(
            SPAN,
            self.ast.vec(),
            self.ast.vec_from_iter([self.expr_stmt(expr)]),
        );
        Expression::ArrowFunctionExpression(self.alloc(self.ast.arrow_function_expression(
            SPAN,
            true,
            true,
            NONE,
            self.no_params(),
            NONE,
            body,
        )))
    }

    pub fn async_thunk_block(&self, stmts: Vec<Statement<'a>>) -> Expression<'a> {
        let body = self
            .ast
            .function_body(SPAN, self.ast.vec(), self.ast.vec_from_iter(stmts));
        let arrow = self.ast.arrow_function_expression(
            SPAN,
            false,
            true,
            NONE,
            self.no_params(),
            NONE,
            body,
        );
        Expression::ArrowFunctionExpression(self.alloc(arrow))
    }

    pub fn logical_coalesce(&self, left: Expression<'a>, right: Expression<'a>) -> Expression<'a> {
        self.ast
            .expression_logical(SPAN, left, ast::LogicalOperator::Coalesce, right)
    }

    pub fn function_decl(
        &self,
        id: BindingIdentifier<'a>,
        body: Vec<Statement<'a>>,
        params: FormalParameters<'a>,
        body_span: Span,
    ) -> Function<'a> {
        let body =
            self.ast
                .alloc_function_body(body_span, self.ast.vec(), self.ast.vec_from_iter(body));
        self.ast.function(
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
        )
    }

    pub fn function_expr(
        &self,
        params: FormalParameters<'a>,
        body: Vec<Statement<'a>>,
    ) -> Expression<'a> {
        let body = self
            .ast
            .alloc_function_body(SPAN, self.ast.vec(), self.ast.vec_from_iter(body));
        Expression::FunctionExpression(self.alloc(self.ast.function(
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
        )))
    }

    pub fn named_function_expr(
        &self,
        name: &str,
        params: FormalParameters<'a>,
        body: Vec<Statement<'a>>,
        is_async: bool,
    ) -> Expression<'a> {
        let body = self
            .ast
            .alloc_function_body(SPAN, self.ast.vec(), self.ast.vec_from_iter(body));
        let id = self.ast.binding_identifier(SPAN, self.ast.atom(name));
        Expression::FunctionExpression(self.alloc(self.ast.function(
            SPAN,
            FunctionType::FunctionExpression,
            Some(id),
            false,
            is_async,
            false,
            NONE,
            NONE,
            params,
            NONE,
            Some(body),
        )))
    }

    pub fn rest_params(&self, name: &str) -> FormalParameters<'a> {
        let atom = self.ast.atom(name);
        let pattern = self.ast.binding_pattern_binding_identifier(SPAN, atom);
        let rest_element = self.ast.binding_rest_element(SPAN, pattern);
        let rest = self
            .ast
            .formal_parameter_rest(SPAN, self.ast.vec(), rest_element, NONE);
        self.ast.formal_parameters(
            SPAN,
            ast::FormalParameterKind::FormalParameter,
            self.ast.vec(),
            Some(rest),
        )
    }
}
