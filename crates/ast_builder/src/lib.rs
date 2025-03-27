use std::cell::Cell;

use ::ast::ConcatenationPart;
use oxc_allocator::{Allocator, Box, CloneIn};
use oxc_ast::{
    ast::{
        self, Argument, ArrayExpression, ArrowFunctionExpression, AssignmentExpression,
        AssignmentTarget, BindingIdentifier, BindingPatternKind, BlockStatement, BooleanLiteral,
        CallExpression, ExportDefaultDeclarationKind, Expression, FormalParameters, Function,
        FunctionType, IdentifierReference, IfStatement, ImportDeclarationSpecifier,
        ImportOrExportKind, LogicalOperator, ModuleDeclaration, ModuleExportName, NumericLiteral,
        ObjectExpression, ObjectProperty, ObjectPropertyKind, Program, SpreadElement, Statement,
        StaticMemberExpression, StringLiteral, TemplateElement, TemplateElementValue,
        TemplateLiteral, UnaryExpression, UnaryOperator, VariableDeclarationKind,
    },
    AstBuilder, NONE,
};
use oxc_span::{Atom, SourceType, SPAN};

pub enum BuilderFunctionArgument<'a, 'short> {
    Str(String),
    Num(f64),
    Ident(&'short str),
    IdentRef(IdentifierReference<'a>),
    Expr(Expression<'a>),
    TemplateStr(TemplateLiteral<'a>),
    Arrow(ArrowFunctionExpression<'a>),
    Bool(bool),
    Call(CallExpression<'a>),
    Array(ArrayExpression<'a>),
}

pub enum BuilderArrayElement {}

pub enum BuilderExpression<'a> {
    Call(CallExpression<'a>),
    Arrow(ArrowFunctionExpression<'a>),
    Assignment(AssignmentExpression<'a>),
    TemplateLiteral(TemplateLiteral<'a>),
    Ident(IdentifierReference<'a>),
    Expr(Expression<'a>),
    Array(ArrayExpression<'a>),
    Str(String),
}
pub enum BuilderStatement<'a> {
    Expr(Expression<'a>),
    Ident(IdentifierReference<'a>),
    Block(BlockStatement<'a>),
    Arrow(ArrowFunctionExpression<'a>),
    IfBlock(IfStatement<'a>),
}

pub enum BuilderAssignmentLeft<'short, 'a> {
    Ident(&'short str),
    IdentRef(IdentifierReference<'a>),
    StaticMemberExpression(StaticMemberExpression<'a>),
}

pub enum BuilderAssignmentRight<'a, 'short> {
    Expr(Expression<'a>),
    Ident(&'short str),
}

pub struct Builder<'a> {
    pub ast: AstBuilder<'a>,
}

pub enum TemplateLiteralPart<'a> {
    String(&'a str),
    Expression(Expression<'a>),
}

impl<'a> Builder<'a> {
    pub fn new(ast: AstBuilder<'a>) -> Builder<'a> {
        return Builder { ast };
    }

    pub fn new_with_ast(allocator: &'a Allocator) -> Self {
        let ast_builder = AstBuilder::new(allocator);
        return Self { ast: ast_builder };
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

    pub fn program(&self, body: Vec<Statement<'a>>) -> Program<'a> {
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

    pub fn params(&self, items: impl IntoIterator<Item = &'a str>) -> FormalParameters<'a> {
        let params = items.into_iter().map(|v| {
            let kind = self.ast.binding_pattern_kind_binding_identifier(SPAN, v);

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
        body: Vec<Statement<'a>>,
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

    pub fn bid(&self, name: &str) -> BindingIdentifier<'a> {
        return self.ast.binding_identifier(SPAN, name);
    }

    pub fn rid(&self, name: &str) -> IdentifierReference<'a> {
        return self.ast.identifier_reference(SPAN, name);
    }

    pub fn rid_expr(&self, name: &str) -> Expression<'a> {
        let rid = self.rid(name);

        return Expression::Identifier(self.alloc(rid));
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

    pub fn numeric_literal_expr(&self, value: f64) -> Expression<'a> {
        return Expression::NumericLiteral(self.alloc(self.numeric_literal(value)));
    }

    pub fn arg<'local>(&self, arg: BuilderFunctionArgument<'a, 'local>) -> Argument<'a> {
        match arg {
            BuilderFunctionArgument::Str(value) => {
                let value = self.string_literal(value.clone());
                Argument::StringLiteral(self.alloc(value))
            }
            BuilderFunctionArgument::Num(value) => {
                let value = self.numeric_literal(value);
                Argument::NumericLiteral(self.alloc(value))
            }
            BuilderFunctionArgument::Ident(value) => {
                let value = self.rid(value);
                Argument::Identifier(self.alloc(value))
            }
            BuilderFunctionArgument::Expr(expr) => Argument::from(expr),
            BuilderFunctionArgument::TemplateStr(template_literal) => {
                Argument::TemplateLiteral(self.alloc(template_literal))
            }
            BuilderFunctionArgument::Arrow(arrow_function_expression) => {
                Argument::ArrowFunctionExpression(self.alloc(arrow_function_expression))
            }
            BuilderFunctionArgument::Bool(value) => {
                Argument::BooleanLiteral(self.alloc(self.ast.boolean_literal(SPAN, value)))
            }
            BuilderFunctionArgument::Call(call_expression) => {
                Argument::CallExpression(self.alloc(call_expression))
            }
            BuilderFunctionArgument::IdentRef(ident) => Argument::Identifier(self.alloc(ident)),
            BuilderFunctionArgument::Array(array_expression) => {
                Argument::ArrayExpression(self.alloc(array_expression))
            }
        }
    }

    pub fn bool(&self, value: bool) -> BooleanLiteral {
        return self.ast.boolean_literal(SPAN, value);
    }

    pub fn bool_expr(&self, value: bool) -> Expression<'a> {
        return Expression::BooleanLiteral(self.alloc(self.bool(value)));
    }

    pub fn call<'short>(
        &self,
        callee: &str,
        args: impl IntoIterator<Item = BuilderFunctionArgument<'a, 'short>>,
    ) -> CallExpression<'a> {
        let ident = self.ast.expression_identifier_reference(SPAN, callee);
        let args = args.into_iter().map(|v| self.arg(v));

        let res = self
            .ast
            .call_expression(SPAN, ident, NONE, self.ast.vec_from_iter(args), false);

        return res;
    }

    pub fn call_expr<'short>(
        &self,
        callee: &str,
        args: impl IntoIterator<Item = BuilderFunctionArgument<'a, 'short>>,
    ) -> Expression<'a> {
        let expr = self.call(callee, args);

        return self.expr(BuilderExpression::Call(expr));
    }

    pub fn call_stmt<'short>(
        &self,
        callee: &str,
        args: impl IntoIterator<Item = BuilderFunctionArgument<'a, 'short>>,
    ) -> Statement<'a> {
        let expr = self.call(callee, args);

        return self.stmt(BuilderStatement::Expr(
            self.expr(BuilderExpression::Call(expr)),
        ));
    }

    pub fn alloc<T>(&self, value: T) -> Box<'a, T> {
        return self.ast.alloc(value);
    }

    pub fn var(&self, name: &str, init: BuilderExpression<'a>) -> Statement<'a> {
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
            Some(self.expr(init)),
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

    pub fn const_stmt(&self, name: &str, init: BuilderExpression<'a>) -> Statement<'a> {
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
            Some(self.expr(init)),
            false,
        );

        let declaration = self.ast.variable_declaration(
            SPAN,
            VariableDeclarationKind::Const,
            self.ast.vec_from_array([decl]),
            false,
        );

        return Statement::VariableDeclaration(self.alloc(declaration));
    }

    pub fn expr(&self, expr: BuilderExpression<'a>) -> Expression<'a> {
        return match expr {
            BuilderExpression::Call(value) => Expression::CallExpression(self.alloc(value)),
            BuilderExpression::TemplateLiteral(template_literal) => {
                Expression::TemplateLiteral(self.alloc(template_literal))
            }
            BuilderExpression::Ident(identifier_reference) => {
                Expression::Identifier(self.alloc(identifier_reference))
            }
            BuilderExpression::Expr(expression) => expression,
            BuilderExpression::Arrow(arrow_function_expression) => {
                Expression::ArrowFunctionExpression(self.alloc(arrow_function_expression))
            }
            BuilderExpression::Assignment(assignment_expression) => {
                Expression::AssignmentExpression(self.alloc(assignment_expression))
            }
            BuilderExpression::Array(array_expression) => {
                Expression::ArrayExpression(self.alloc(array_expression))
            }
            BuilderExpression::Str(value) => {
                Expression::StringLiteral(self.alloc(self.ast.string_literal(SPAN, value, None)))
            }
        };
    }

    pub fn stmt(&self, expr: BuilderStatement<'a>) -> Statement<'a> {
        return match expr {
            BuilderStatement::Expr(value) => Statement::ExpressionStatement(
                self.alloc(self.ast.expression_statement(SPAN, value)),
            ),
            BuilderStatement::Block(block_statement) => {
                Statement::BlockStatement(self.alloc(block_statement))
            }
            BuilderStatement::Arrow(arrow_function_expression) => {
                return self.stmt(BuilderStatement::Expr(
                    self.expr(BuilderExpression::Arrow(arrow_function_expression)),
                ));
            }
            BuilderStatement::IfBlock(if_statement) => {
                Statement::IfStatement(self.alloc(if_statement))
            }
            BuilderStatement::Ident(identifier_reference) => {
                return self.stmt(BuilderStatement::Expr(
                    self.expr(BuilderExpression::Ident(identifier_reference)),
                ));
            }
        };
    }

    pub fn empty_stmt(&self) -> Statement<'a> {
        return Statement::EmptyStatement(self.ast.alloc_empty_statement(SPAN));
    }

    pub fn template_literal(&self, parts: &mut Vec<ConcatenationPart<'a>>) -> TemplateLiteral<'a> {
        let mut quasis = self.ast.vec();
        let mut expressions = self.ast.vec();
        let mut iter = parts.iter_mut().peekable();
        let mut is_first = true;

        while let Some(part) = iter.next() {
            let tail = iter.peek().is_none();

            match part {
                ConcatenationPart::String(value) => {
                    let temp = self.ast.template_element(
                        SPAN,
                        tail,
                        TemplateElementValue {
                            cooked: None,
                            raw: Atom::from(*value),
                        },
                    );

                    quasis.push(temp);
                }
                ConcatenationPart::Expression(expression) => {
                    if is_first {
                        let temp = self.ast.template_element(
                            SPAN,
                            false,
                            TemplateElementValue {
                                cooked: None,
                                raw: Atom::from(""),
                            },
                        );

                        quasis.push(temp);
                    }

                    let expression = self.ast.move_expression(expression);

                    let expression = self.ast.expression_logical(
                        SPAN,
                        expression,
                        LogicalOperator::Coalesce,
                        self.ast.expression_string_literal(SPAN, "", None),
                    );

                    expressions.push(expression);

                    if tail {
                        let temp = self.ast.template_element(
                            SPAN,
                            true,
                            TemplateElementValue {
                                cooked: None,
                                raw: Atom::from(""),
                            },
                        );

                        quasis.push(temp);
                    }
                }
            }

            is_first = false;
        }

        let lit = self.ast.template_literal(SPAN, quasis, expressions);

        return lit;
    }

    pub fn template_literal2<'short>(
        &self,
        parts: impl IntoIterator<Item = TemplateLiteralPart<'a>>,
    ) -> TemplateLiteral<'a> {
        let mut quasis = self.ast.vec();
        let mut expressions = self.ast.vec();
        let mut iter = parts.into_iter().peekable();
        let mut is_first = true;

        while let Some(part) = iter.next() {
            let tail = iter.peek().is_none();

            match part {
                TemplateLiteralPart::String(value) => {
                    let temp = self.ast.template_element(
                        SPAN,
                        tail,
                        TemplateElementValue {
                            cooked: None,
                            raw: Atom::from(value),
                        },
                    );

                    quasis.push(temp);
                }
                TemplateLiteralPart::Expression(expression) => {
                    if is_first {
                        let temp = self.ast.template_element(
                            SPAN,
                            false,
                            TemplateElementValue {
                                cooked: None,
                                raw: Atom::from(""),
                            },
                        );

                        quasis.push(temp);
                    }

                    let expression = self.ast.expression_logical(
                        SPAN,
                        expression,
                        LogicalOperator::Coalesce,
                        self.ast.expression_string_literal(SPAN, "", None),
                    );

                    expressions.push(expression);

                    if tail {
                        let temp = self.ast.template_element(
                            SPAN,
                            true,
                            TemplateElementValue {
                                cooked: None,
                                raw: Atom::from(""),
                            },
                        );

                        quasis.push(temp);
                    }
                }
            }

            is_first = false;
        }

        let lit = self.ast.template_literal(SPAN, quasis, expressions);

        return lit;
    }

    pub fn template_literal2_expr<'short>(
        &self,
        parts: impl IntoIterator<Item = TemplateLiteralPart<'a>>,
    ) -> Expression<'a> {
        return Expression::TemplateLiteral(self.alloc(self.template_literal2(parts)));
    }

    pub fn arrow(
        &self,
        params: FormalParameters<'a>,
        statements: impl IntoIterator<Item = Statement<'a>>,
    ) -> ArrowFunctionExpression<'a> {
        let statements = self.ast.vec_from_iter(statements);
        let is_expr =
            statements.len() == 1 && matches!(&statements[0], Statement::ExpressionStatement(_));

        let body = self.ast.function_body(SPAN, self.ast.vec(), statements);

        let arrow = self
            .ast
            .arrow_function_expression(SPAN, is_expr, false, NONE, params, NONE, body);

        return arrow;
    }

    pub fn arrow_expr(
        &self,
        params: FormalParameters<'a>,
        statements: impl IntoIterator<Item = Statement<'a>>,
    ) -> Expression<'a> {
        let arrow = self.arrow(params, statements);

        return Expression::ArrowFunctionExpression(self.alloc(arrow));
    }

    pub fn clone_expr(&self, expr: &Expression<'a>) -> Expression<'a> {
        return expr.clone_in(&self.ast.allocator);
    }

    pub fn cheap_expr(&self) -> Expression<'a> {
        return self.ast.expression_boolean_literal(SPAN, false);
    }

    pub fn block(&self, body: Vec<Statement<'a>>) -> Statement<'a> {
        let block = self.ast.block_statement(SPAN, self.ast.vec_from_iter(body));

        return self.stmt(BuilderStatement::Block(block));
    }

    pub fn if_stmt(
        &self,
        test: Expression<'a>,
        consequent: Statement<'a>,
        alternate: Option<Statement<'a>>,
    ) -> Statement<'a> {
        let if_stmt = self.ast.if_statement(SPAN, test, consequent, alternate);

        return self.stmt(BuilderStatement::IfBlock(if_stmt));
    }

    pub fn template_from_str<'local>(&self, value: &'local str) -> TemplateLiteral<'a> {
        let mut quasis = self.ast.vec();

        quasis.push(TemplateElement {
            span: SPAN,
            tail: true,
            value: TemplateElementValue {
                cooked: None,
                raw: self.ast.atom(&value),
            },
        });

        return self.ast.template_literal(SPAN, quasis, self.ast.vec());
    }

    pub fn assignment_expression<'local>(
        &self,
        left: BuilderAssignmentLeft<'local, 'a>,
        right: BuilderAssignmentRight<'a, 'local>,
    ) -> AssignmentExpression<'a> {
        let left: ast::AssignmentTarget<'a> = match left {
            BuilderAssignmentLeft::Ident(value) => {
                AssignmentTarget::AssignmentTargetIdentifier(self.alloc(self.rid(value)))
            }
            BuilderAssignmentLeft::StaticMemberExpression(static_member_expression) => {
                AssignmentTarget::StaticMemberExpression(self.alloc(static_member_expression))
            }
            BuilderAssignmentLeft::IdentRef(ident) => {
                AssignmentTarget::AssignmentTargetIdentifier(self.alloc(ident))
            }
        };

        let right: Expression<'a> = match right {
            BuilderAssignmentRight::Expr(expression) => expression,
            BuilderAssignmentRight::Ident(value) => {
                self.expr(BuilderExpression::Ident(self.rid(value)))
            }
        };

        let assignment =
            self.ast
                .assignment_expression(SPAN, ast::AssignmentOperator::Assign, left, right);

        return assignment;
    }

    pub fn assignment_expression_expr<'local>(
        &self,
        left: BuilderAssignmentLeft<'local, 'a>,
        right: BuilderAssignmentRight<'a, 'local>,
    ) -> Expression<'a> {
        let assignment = self.assignment_expression(left, right);

        return self.expr(BuilderExpression::Assignment(assignment));
    }

    pub fn assignment_expression_stmt<'local>(
        &self,
        left: BuilderAssignmentLeft<'local, 'a>,
        right: BuilderAssignmentRight<'a, 'local>,
    ) -> Statement<'a> {
        return self.stmt(BuilderStatement::Expr(
            self.assignment_expression_expr(left, right),
        ));
    }

    pub fn move_expr(&self, expr: &mut Expression<'a>) -> Expression<'a> {
        return self.ast.move_expression(expr);
    }

    pub fn static_member_expr<'local>(
        &self,
        object: Expression<'a>,
        prop: &'local str,
    ) -> StaticMemberExpression<'a> {
        let property = self.ast.identifier_name(SPAN, prop);

        let expr = self
            .ast
            .static_member_expression(SPAN, object, property, false);

        return expr;
    }

    pub fn array(
        &self,
        elements: impl IntoIterator<Item = BuilderArrayElement>,
    ) -> ArrayExpression<'a> {
        let elements = self
            .ast
            .vec_from_iter(elements.into_iter().map(|item| match item {}));

        return self.ast.array_expression(SPAN, elements, None);
    }

    pub fn init_prop(&self, name: &str, value: Expression<'a>) -> ObjectPropertyKind<'a> {
        let mut shorthand = false;

        if let Expression::Identifier(ident) = &value {
            shorthand = ident.name.as_str() == name;
        }

        return ObjectPropertyKind::ObjectProperty(self.ast.alloc(ObjectProperty {
            computed: false,
            method: false,
            kind: ast::PropertyKind::Init,
            shorthand,
            span: SPAN,
            value,
            key: ast::PropertyKey::StaticIdentifier(self.ast.alloc_identifier_name(SPAN, name)),
        }));
    }

    pub fn spread_prop(&self, value: Expression<'a>) -> ObjectPropertyKind<'a> {
        return ObjectPropertyKind::SpreadProperty(self.ast.alloc(SpreadElement {
            argument: value,
            span: SPAN,
        }));
    }

    pub fn object(
        &self,
        props: impl IntoIterator<Item = ObjectPropertyKind<'a>>,
    ) -> ObjectExpression<'a> {
        return self
            .ast
            .object_expression(SPAN, self.ast.vec_from_iter(props), None);
    }

    pub fn object_expr(
        &self,
        props: impl IntoIterator<Item = ObjectPropertyKind<'a>>,
    ) -> Expression<'a> {
        return Expression::ObjectExpression(self.alloc(self.object(props)));
    }

    pub fn unary(&self, operator: UnaryOperator, argument: Expression<'a>) -> UnaryExpression<'a> {
        let unary = self.ast.unary_expression(SPAN, operator, argument);

        return unary;
    }

    pub fn unary_expr(&self, operator: UnaryOperator, argument: Expression<'a>) -> Expression<'a> {
        return Expression::UnaryExpression(self.alloc(self.unary(operator, argument)));
    }

    pub fn let_stmt(&self, name: &str, init: Option<BuilderExpression<'a>>) -> Statement<'a> {
        let ident = self.ast.binding_identifier(SPAN, name);
        let pattern = self.ast.binding_pattern(
            BindingPatternKind::BindingIdentifier(self.alloc(ident)),
            NONE,
            false,
        );

        let decl = self.ast.variable_declarator(
            SPAN,
            VariableDeclarationKind::Let,
            pattern,
            init.map(|e| self.expr(e)),
            false,
        );

        let declaration = self.ast.variable_declaration(
            SPAN,
            VariableDeclarationKind::Let,
            self.ast.vec_from_array([decl]),
            false,
        );

        return Statement::VariableDeclaration(self.alloc(declaration));
    }
}
