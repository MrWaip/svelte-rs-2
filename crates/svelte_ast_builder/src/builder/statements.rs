use super::*;

impl<'a> Builder<'a> {
    pub fn expr_stmt(&self, expr: Expression<'a>) -> Statement<'a> {
        Statement::ExpressionStatement(self.alloc(self.ast.expression_statement(SPAN, expr)))
    }

    pub fn debugger_stmt(&self) -> Statement<'a> {
        Statement::DebuggerStatement(self.alloc(self.ast.debugger_statement(SPAN)))
    }

    pub fn var_stmt(&self, name: &str, init: Expression<'a>) -> Statement<'a> {
        self.var_decl_stmt(name, init, VariableDeclarationKind::Var)
    }

    pub fn let_init_stmt(&self, name: &str, init: Expression<'a>) -> Statement<'a> {
        self.var_decl_stmt(name, init, VariableDeclarationKind::Let)
    }

    pub fn var_uninit_stmt(&self, name: &str) -> Statement<'a> {
        let pattern = self
            .ast
            .binding_pattern_binding_identifier(SPAN, self.ast.atom(name));
        let decl = self.ast.variable_declarator(
            SPAN,
            VariableDeclarationKind::Var,
            pattern,
            NONE,
            None,
            false,
        );
        let declaration = self.ast.variable_declaration(
            SPAN,
            VariableDeclarationKind::Var,
            self.ast.vec_from_array([decl]),
            false,
        );
        Statement::VariableDeclaration(self.alloc(declaration))
    }

    pub fn let_stmt(&self, name: &str) -> Statement<'a> {
        let pattern = self
            .ast
            .binding_pattern_binding_identifier(SPAN, self.ast.atom(name));
        let decl = self.ast.variable_declarator(
            SPAN,
            VariableDeclarationKind::Let,
            pattern,
            NONE,
            None,
            false,
        );
        let declaration = self.ast.variable_declaration(
            SPAN,
            VariableDeclarationKind::Let,
            self.ast.vec_from_array([decl]),
            false,
        );
        Statement::VariableDeclaration(self.alloc(declaration))
    }

    pub fn const_stmt(&self, name: &str, init: Expression<'a>) -> Statement<'a> {
        self.var_decl_stmt(name, init, VariableDeclarationKind::Const)
    }

    pub fn var_multi_stmt(&self, names: &[&str]) -> Statement<'a> {
        let decls: Vec<_> = names
            .iter()
            .map(|name| {
                let pattern = self
                    .ast
                    .binding_pattern_binding_identifier(SPAN, self.ast.atom(name));
                self.ast.variable_declarator(
                    SPAN,
                    VariableDeclarationKind::Var,
                    pattern,
                    NONE,
                    None,
                    false,
                )
            })
            .collect();
        let declaration = self.ast.variable_declaration(
            SPAN,
            VariableDeclarationKind::Var,
            self.ast.vec_from_iter(decls),
            false,
        );
        Statement::VariableDeclaration(self.alloc(declaration))
    }

    pub fn var_init_stmt(&self, declarator: oxc_ast::ast::VariableDeclarator<'a>) -> Statement<'a> {
        let declaration = self.ast.variable_declaration(
            SPAN,
            VariableDeclarationKind::Var,
            self.ast.vec_from_iter([declarator]),
            false,
        );
        Statement::VariableDeclaration(self.alloc(declaration))
    }

    pub fn let_multi_stmt(&self, declarators: Vec<(&str, Expression<'a>)>) -> Statement<'a> {
        self.var_decl_multi_stmt(declarators, VariableDeclarationKind::Let)
    }

    pub fn var_decl_multi_stmt(
        &self,
        declarators: Vec<(&str, Expression<'a>)>,
        kind: VariableDeclarationKind,
    ) -> Statement<'a> {
        let decls: Vec<_> = declarators
            .into_iter()
            .map(|(name, init)| {
                let pattern = self
                    .ast
                    .binding_pattern_binding_identifier(SPAN, self.ast.atom(name));
                self.ast
                    .variable_declarator(SPAN, kind, pattern, NONE, Some(init), false)
            })
            .collect();
        let declaration =
            self.ast
                .variable_declaration(SPAN, kind, self.ast.vec_from_iter(decls), false);
        Statement::VariableDeclaration(self.alloc(declaration))
    }

    pub fn const_array_destruct_stmt(&self, names: &[&str], init: Expression<'a>) -> Statement<'a> {
        let elements: Vec<_> = names
            .iter()
            .map(|name| {
                let pattern = self
                    .ast
                    .binding_pattern_binding_identifier(SPAN, self.ast.atom(name));
                Some(pattern)
            })
            .collect();
        let array_pattern = self
            .ast
            .array_pattern(SPAN, self.ast.vec_from_iter(elements), NONE);
        let pattern = ast::BindingPattern::ArrayPattern(self.alloc(array_pattern));
        let decl = self.ast.variable_declarator(
            SPAN,
            VariableDeclarationKind::Const,
            pattern,
            NONE,
            Some(init),
            false,
        );
        let declaration = self.ast.variable_declaration(
            SPAN,
            VariableDeclarationKind::Const,
            self.ast.vec_from_array([decl]),
            false,
        );
        Statement::VariableDeclaration(self.alloc(declaration))
    }

    pub fn var_array_destruct_stmt(&self, names: &[String], init: Expression<'a>) -> Statement<'a> {
        let elements: Vec<_> = names
            .iter()
            .map(|name| {
                let pattern = self
                    .ast
                    .binding_pattern_binding_identifier(SPAN, self.ast.atom(name.as_str()));
                Some(pattern)
            })
            .collect();
        let array_pattern = self
            .ast
            .array_pattern(SPAN, self.ast.vec_from_iter(elements), NONE);
        let pattern = ast::BindingPattern::ArrayPattern(self.alloc(array_pattern));
        let decl = self.ast.variable_declarator(
            SPAN,
            VariableDeclarationKind::Var,
            pattern,
            NONE,
            Some(init),
            false,
        );
        let declaration = self.ast.variable_declaration(
            SPAN,
            VariableDeclarationKind::Var,
            self.ast.vec_from_array([decl]),
            false,
        );
        Statement::VariableDeclaration(self.alloc(declaration))
    }

    pub fn var_object_destruct_stmt(
        &self,
        names: &[String],
        init: Expression<'a>,
    ) -> Statement<'a> {
        let properties: Vec<_> = names
            .iter()
            .map(|name| {
                let atom = self.ast.atom(name.as_str());
                let key = ast::PropertyKey::StaticIdentifier(
                    self.alloc(self.ast.identifier_name(SPAN, atom)),
                );
                let value = self
                    .ast
                    .binding_pattern_binding_identifier(SPAN, self.ast.atom(name.as_str()));
                self.ast.binding_property(SPAN, key, value, true, false)
            })
            .collect();
        let object_pattern =
            self.ast
                .object_pattern(SPAN, self.ast.vec_from_iter(properties), NONE);
        let pattern = ast::BindingPattern::ObjectPattern(self.alloc(object_pattern));
        let decl = self.ast.variable_declarator(
            SPAN,
            VariableDeclarationKind::Var,
            pattern,
            NONE,
            Some(init),
            false,
        );
        let declaration = self.ast.variable_declaration(
            SPAN,
            VariableDeclarationKind::Var,
            self.ast.vec_from_array([decl]),
            false,
        );
        Statement::VariableDeclaration(self.alloc(declaration))
    }

    pub fn shorthand_object_expr(&self, names: &[String]) -> Expression<'a> {
        let properties: Vec<_> = names
            .iter()
            .map(|name| {
                let atom = self.ast.atom(name.as_str());
                let key = ast::PropertyKey::StaticIdentifier(
                    self.alloc(self.ast.identifier_name(SPAN, atom)),
                );
                let value = self.rid_expr(name);
                self.ast.object_property_kind_object_property(
                    SPAN,
                    ast::PropertyKind::Init,
                    key,
                    value,
                    false,
                    true,
                    false,
                )
            })
            .collect();
        self.ast
            .expression_object(SPAN, self.ast.vec_from_iter(properties))
    }

    pub fn const_object_destruct_stmt(
        &self,
        names: &[String],
        init: Expression<'a>,
    ) -> Statement<'a> {
        let properties: Vec<_> = names
            .iter()
            .map(|name| {
                let atom = self.ast.atom(name.as_str());
                let key = ast::PropertyKey::StaticIdentifier(
                    self.alloc(self.ast.identifier_name(SPAN, atom)),
                );
                let value = self
                    .ast
                    .binding_pattern_binding_identifier(SPAN, self.ast.atom(name.as_str()));
                self.ast.binding_property(SPAN, key, value, true, false)
            })
            .collect();
        let object_pattern =
            self.ast
                .object_pattern(SPAN, self.ast.vec_from_iter(properties), NONE);
        let pattern = ast::BindingPattern::ObjectPattern(self.alloc(object_pattern));
        let decl = self.ast.variable_declarator(
            SPAN,
            VariableDeclarationKind::Const,
            pattern,
            NONE,
            Some(init),
            false,
        );
        let declaration = self.ast.variable_declaration(
            SPAN,
            VariableDeclarationKind::Const,
            self.ast.vec_from_array([decl]),
            false,
        );
        Statement::VariableDeclaration(self.alloc(declaration))
    }

    fn var_decl_stmt(
        &self,
        name: &str,
        init: Expression<'a>,
        kind: VariableDeclarationKind,
    ) -> Statement<'a> {
        let pattern = self
            .ast
            .binding_pattern_binding_identifier(SPAN, self.ast.atom(name));
        let decl = self
            .ast
            .variable_declarator(SPAN, kind, pattern, NONE, Some(init), false);
        let declaration =
            self.ast
                .variable_declaration(SPAN, kind, self.ast.vec_from_array([decl]), false);
        Statement::VariableDeclaration(self.alloc(declaration))
    }

    pub fn return_stmt(&self, expr: Expression<'a>) -> Statement<'a> {
        Statement::ReturnStatement(self.alloc(self.ast.return_statement(SPAN, Some(expr))))
    }

    pub fn block_stmt(&self, body: Vec<Statement<'a>>) -> Statement<'a> {
        let block = self.ast.block_statement(SPAN, self.ast.vec_from_iter(body));
        Statement::BlockStatement(self.alloc(block))
    }

    pub fn if_stmt(
        &self,
        test: Expression<'a>,
        consequent: Statement<'a>,
        alternate: Option<Statement<'a>>,
    ) -> Statement<'a> {
        Statement::IfStatement(self.alloc(self.ast.if_statement(SPAN, test, consequent, alternate)))
    }

    pub fn assign_stmt(&self, left: AssignLeft<'a>, right: Expression<'a>) -> Statement<'a> {
        self.expr_stmt(self.assign_expr(left, right))
    }

    pub fn assign_expr(&self, left: AssignLeft<'a>, right: Expression<'a>) -> Expression<'a> {
        let left = match left {
            AssignLeft::StaticMember(m) => AssignmentTarget::StaticMemberExpression(self.alloc(m)),
            AssignLeft::ComputedMember(m) => {
                AssignmentTarget::ComputedMemberExpression(self.alloc(m))
            }
            AssignLeft::Ident(name) => {
                let atom = self.ast.atom(&name);
                AssignmentTarget::AssignmentTargetIdentifier(
                    self.alloc(self.ast.identifier_reference(SPAN, atom)),
                )
            }
        };
        let assign =
            self.ast
                .assignment_expression(SPAN, ast::AssignmentOperator::Assign, left, right);
        Expression::AssignmentExpression(self.alloc(assign))
    }

    pub fn assign_expr_raw(
        &self,
        left: AssignmentTarget<'a>,
        right: Expression<'a>,
    ) -> Expression<'a> {
        let assign =
            self.ast
                .assignment_expression(SPAN, ast::AssignmentOperator::Assign, left, right);
        Expression::AssignmentExpression(self.alloc(assign))
    }

    pub fn expr_to_assignment_target(&self, expr: Expression<'a>) -> AssignmentTarget<'a> {
        match expr {
            Expression::Identifier(id) => AssignmentTarget::AssignmentTargetIdentifier(id),
            Expression::StaticMemberExpression(m) => AssignmentTarget::StaticMemberExpression(m),
            Expression::ComputedMemberExpression(m) => {
                AssignmentTarget::ComputedMemberExpression(m)
            }
            _ => panic!("cannot convert expression to assignment target"),
        }
    }
}
