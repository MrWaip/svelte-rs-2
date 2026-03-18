use oxc_allocator::{Allocator, Box, CloneIn};
use oxc_ast::{
    ast::{
        self, Argument, ArrowFunctionExpression, AssignmentTarget,
        BindingIdentifier, CallExpression, ChainElement, ExportDefaultDeclarationKind,
        Expression, FormalParameters, Function, FunctionType, IdentifierReference,
        ImportDeclarationSpecifier, ImportOrExportKind, ModuleDeclaration,
        NumericLiteral, Program, Statement,
        StaticMemberExpression, StringLiteral, TemplateElementValue,
        TemplateLiteral, VariableDeclarationKind,
    },
    AstBuilder, NONE,
};
use oxc_parser::Parser as OxcParser;
use oxc_span::{Atom, SourceType, SPAN};
use oxc_syntax::node::NodeId as OxcNodeId;
use std::cell::Cell;

pub enum Arg<'a, 'short> {
    Str(String),
    Num(f64),
    Ident(&'short str),
    IdentRef(IdentifierReference<'a>),
    Expr(Expression<'a>),
    Arrow(ArrowFunctionExpression<'a>),
    Bool(bool),
    Spread(Expression<'a>),
}

pub enum AssignLeft<'a> {
    StaticMember(StaticMemberExpression<'a>),
    Ident(String),
}

pub enum AssignRight<'a> {
    Expr(Expression<'a>),
}

pub enum TemplatePart<'a> {
    Str(String),
    Expr(Expression<'a>),
}

pub struct Builder<'a> {
    pub ast: AstBuilder<'a>,
}

impl<'a> Builder<'a> {
    pub fn new(allocator: &'a Allocator) -> Self {
        Self { ast: AstBuilder::new(allocator) }
    }

    pub fn alloc<T>(&self, value: T) -> Box<'a, T> {
        self.ast.alloc(value)
    }

    pub fn alloc_str(&self, s: &str) -> &'a str {
        self.ast.allocator.alloc_str(s)
    }

    // -----------------------------------------------------------------------
    // Identifiers
    // -----------------------------------------------------------------------

    pub fn bid(&self, name: &str) -> BindingIdentifier<'a> {
        self.ast.binding_identifier(SPAN, self.ast.atom(name))
    }

    pub fn rid(&self, name: &str) -> IdentifierReference<'a> {
        self.ast.identifier_reference(SPAN, self.ast.atom(name))
    }

    pub fn rid_expr(&self, name: &str) -> Expression<'a> {
        Expression::Identifier(self.alloc(self.rid(name)))
    }

    // -----------------------------------------------------------------------
    // Literals
    // -----------------------------------------------------------------------

    pub fn bool_expr(&self, value: bool) -> Expression<'a> {
        Expression::BooleanLiteral(self.alloc(self.ast.boolean_literal(SPAN, value)))
    }

    pub fn null_expr(&self) -> Expression<'a> {
        self.ast.expression_null_literal(SPAN)
    }

    /// `void 0` expression — used where `undefined` is needed but `null` is semantically wrong.
    pub fn void_zero_expr(&self) -> Expression<'a> {
        self.ast.expression_unary(
            SPAN,
            ast::UnaryOperator::Void,
            self.num_expr(0.0),
        )
    }

    pub fn num(&self, value: f64) -> NumericLiteral<'a> {
        self.ast.numeric_literal(SPAN, value, None, ast::NumberBase::Decimal)
    }

    pub fn num_expr(&self, value: f64) -> Expression<'a> {
        Expression::NumericLiteral(self.alloc(self.num(value)))
    }

    pub fn str_lit(&self, value: &str) -> StringLiteral<'a> {
        self.ast.string_literal(SPAN, self.ast.atom(value), None)
    }

    pub fn str_expr(&self, value: &str) -> Expression<'a> {
        Expression::StringLiteral(self.alloc(self.str_lit(value)))
    }

    pub fn empty_array_expr(&self) -> Expression<'a> {
        Expression::ArrayExpression(self.alloc(
            self.ast.array_expression(SPAN, self.ast.vec()),
        ))
    }

    pub fn array_from_args<'short>(&self, args: impl IntoIterator<Item = Arg<'a, 'short>>) -> Expression<'a> {
        let elements = args.into_iter().map(|a| {
            if let Arg::Spread(e) = a {
                return ast::ArrayExpressionElement::SpreadElement(self.alloc(self.ast.spread_element(SPAN, e)));
            }
            let expr = match a {
                Arg::Str(v) => self.str_expr(&v),
                Arg::Num(v) => self.num_expr(v),
                Arg::Ident(v) => self.rid_expr(v),
                Arg::IdentRef(r) => Expression::Identifier(self.alloc(r)),
                Arg::Expr(e) => e,
                Arg::Arrow(a) => Expression::ArrowFunctionExpression(self.alloc(a)),
                Arg::Bool(v) => self.bool_expr(v),
                Arg::Spread(_) => unreachable!(),
            };
            ast::ArrayExpressionElement::from(expr)
        });
        Expression::ArrayExpression(self.alloc(
            self.ast.array_expression(SPAN, self.ast.vec_from_iter(elements)),
        ))
    }

    // -----------------------------------------------------------------------
    // Call expressions
    // -----------------------------------------------------------------------

    pub fn call<'short>(
        &self,
        callee: &str,
        args: impl IntoIterator<Item = Arg<'a, 'short>>,
    ) -> CallExpression<'a> {
        let ident = self.ast.expression_identifier(SPAN, self.ast.atom(callee));
        let args = args.into_iter().map(|a| self.arg_to_argument(a));
        self.ast.call_expression(SPAN, ident, NONE, self.ast.vec_from_iter(args), false)
    }

    pub fn call_expr<'short>(
        &self,
        callee: &str,
        args: impl IntoIterator<Item = Arg<'a, 'short>>,
    ) -> Expression<'a> {
        Expression::CallExpression(self.alloc(self.call(callee, args)))
    }

    pub fn call_stmt<'short>(
        &self,
        callee: &str,
        args: impl IntoIterator<Item = Arg<'a, 'short>>,
    ) -> Statement<'a> {
        let call = self.call(callee, args);
        self.expr_stmt(Expression::CallExpression(self.alloc(call)))
    }

    fn arg_to_argument<'short>(&self, arg: Arg<'a, 'short>) -> Argument<'a> {
        match arg {
            Arg::Str(v) => Argument::StringLiteral(self.alloc(self.str_lit(&v))),
            Arg::Num(v) => Argument::NumericLiteral(self.alloc(self.num(v))),
            Arg::Ident(v) => Argument::Identifier(self.alloc(self.rid(v))),
            Arg::IdentRef(r) => Argument::Identifier(self.alloc(r)),
            Arg::Expr(e) => Argument::from(e),
            Arg::Arrow(a) => Argument::ArrowFunctionExpression(self.alloc(a)),
            Arg::Bool(v) => Argument::BooleanLiteral(self.alloc(self.ast.boolean_literal(SPAN, v))),
            Arg::Spread(e) => Argument::SpreadElement(self.alloc(self.ast.spread_element(SPAN, e))),
        }
    }

    // -----------------------------------------------------------------------
    // Statements
    // -----------------------------------------------------------------------

    pub fn expr_stmt(&self, expr: Expression<'a>) -> Statement<'a> {
        Statement::ExpressionStatement(self.alloc(self.ast.expression_statement(SPAN, expr)))
    }

    pub fn var_stmt(&self, name: &str, init: Expression<'a>) -> Statement<'a> {
        self.var_decl_stmt(name, init, VariableDeclarationKind::Var)
    }

    /// `let name = init;` — initialized let declaration.
    pub fn let_init_stmt(&self, name: &str, init: Expression<'a>) -> Statement<'a> {
        self.var_decl_stmt(name, init, VariableDeclarationKind::Let)
    }

    /// `let name;` — uninitialized variable declaration.
    pub fn let_stmt(&self, name: &str) -> Statement<'a> {
        let pattern = self.ast.binding_pattern_binding_identifier(SPAN, self.ast.atom(name));
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

    /// `let a = init_a, b = init_b, ...;` — multi-declarator let statement.
    pub fn let_multi_stmt(&self, declarators: Vec<(&str, Expression<'a>)>) -> Statement<'a> {
        let decls: Vec<_> = declarators.into_iter().map(|(name, init)| {
            let pattern = self.ast.binding_pattern_binding_identifier(SPAN, self.ast.atom(name));
            self.ast.variable_declarator(SPAN, VariableDeclarationKind::Let, pattern, NONE, Some(init), false)
        }).collect();
        let declaration = self.ast.variable_declaration(
            SPAN,
            VariableDeclarationKind::Let,
            self.ast.vec_from_iter(decls),
            false,
        );
        Statement::VariableDeclaration(self.alloc(declaration))
    }

    /// `const [a, b] = init;` — array destructuring const declaration.
    pub fn const_array_destruct_stmt(&self, names: &[&str], init: Expression<'a>) -> Statement<'a> {
        let elements: Vec<_> = names.iter().map(|name| {
            let pattern = self.ast.binding_pattern_binding_identifier(SPAN, self.ast.atom(*name));
            Some(pattern)
        }).collect();
        let array_pattern = self.ast.array_pattern(SPAN, self.ast.vec_from_iter(elements), NONE);
        let pattern = ast::BindingPattern::ArrayPattern(self.alloc(array_pattern));
        let decl = self.ast.variable_declarator(
            SPAN, VariableDeclarationKind::Const, pattern, NONE, Some(init), false,
        );
        let declaration = self.ast.variable_declaration(
            SPAN,
            VariableDeclarationKind::Const,
            self.ast.vec_from_array([decl]),
            false,
        );
        Statement::VariableDeclaration(self.alloc(declaration))
    }

    /// `var { a, b } = init;` — object destructuring var declaration.
    pub fn var_object_destruct_stmt(&self, names: &[String], init: Expression<'a>) -> Statement<'a> {
        let properties: Vec<_> = names.iter().map(|name| {
            let atom = self.ast.atom(name.as_str());
            let key = ast::PropertyKey::StaticIdentifier(
                self.alloc(self.ast.identifier_name(SPAN, atom)),
            );
            let value = self.ast.binding_pattern_binding_identifier(SPAN, self.ast.atom(name.as_str()));
            self.ast.binding_property(SPAN, key, value, true, false)
        }).collect();
        let object_pattern = self.ast.object_pattern(
            SPAN,
            self.ast.vec_from_iter(properties),
            NONE,
        );
        let pattern = ast::BindingPattern::ObjectPattern(self.alloc(object_pattern));
        let decl = self.ast.variable_declarator(
            SPAN, VariableDeclarationKind::Var, pattern, NONE, Some(init), false,
        );
        let declaration = self.ast.variable_declaration(
            SPAN,
            VariableDeclarationKind::Var,
            self.ast.vec_from_array([decl]),
            false,
        );
        Statement::VariableDeclaration(self.alloc(declaration))
    }

    /// `{ a, b }` — shorthand object expression.
    pub fn shorthand_object_expr(&self, names: &[String]) -> Expression<'a> {
        let properties: Vec<_> = names.iter().map(|name| {
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
                false, true, false, // computed=false, shorthand=true, method=false
            )
        }).collect();
        self.ast.expression_object(
            SPAN,
            self.ast.vec_from_iter(properties),
        )
    }

    /// `const { a, b } = init;` — object destructuring const declaration.
    pub fn const_object_destruct_stmt(&self, names: &[String], init: Expression<'a>) -> Statement<'a> {
        let properties: Vec<_> = names.iter().map(|name| {
            let atom = self.ast.atom(name.as_str());
            let key = ast::PropertyKey::StaticIdentifier(
                self.alloc(self.ast.identifier_name(SPAN, atom)),
            );
            let value = self.ast.binding_pattern_binding_identifier(SPAN, self.ast.atom(name.as_str()));
            self.ast.binding_property(SPAN, key, value, true, false)
        }).collect();
        let object_pattern = self.ast.object_pattern(
            SPAN,
            self.ast.vec_from_iter(properties),
            NONE,
        );
        let pattern = ast::BindingPattern::ObjectPattern(self.alloc(object_pattern));
        let decl = self.ast.variable_declarator(
            SPAN, VariableDeclarationKind::Const, pattern, NONE, Some(init), false,
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
        let pattern = self.ast.binding_pattern_binding_identifier(SPAN, self.ast.atom(name));
        let decl = self.ast.variable_declarator(SPAN, kind, pattern, NONE, Some(init), false);
        let declaration = self.ast.variable_declaration(
            SPAN,
            kind,
            self.ast.vec_from_array([decl]),
            false,
        );
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

    pub fn assign_stmt(&self, left: AssignLeft<'a>, right: AssignRight<'a>) -> Statement<'a> {
        self.expr_stmt(self.assign_expr(left, right))
    }

    pub fn assign_expr(&self, left: AssignLeft<'a>, right: AssignRight<'a>) -> Expression<'a> {
        let left = match left {
            AssignLeft::StaticMember(m) => {
                AssignmentTarget::StaticMemberExpression(self.alloc(m))
            }
            AssignLeft::Ident(name) => {
                let atom = self.ast.atom(&name);
                AssignmentTarget::AssignmentTargetIdentifier(self.alloc(
                    self.ast.identifier_reference(SPAN, atom),
                ))
            }
        };
        let right = match right {
            AssignRight::Expr(e) => e,
        };
        let assign =
            self.ast.assignment_expression(SPAN, ast::AssignmentOperator::Assign, left, right);
        Expression::AssignmentExpression(self.alloc(assign))
    }

    // -----------------------------------------------------------------------
    // Functions & arrows
    // -----------------------------------------------------------------------

    pub fn no_params(&self) -> FormalParameters<'a> {
        self.params(std::iter::empty::<&str>())
    }

    pub fn params(&self, items: impl IntoIterator<Item = impl AsRef<str>>) -> FormalParameters<'a> {
        let params: Vec<_> = items.into_iter().map(|v| {
            let atom = self.ast.atom(v.as_ref());
            let pattern = self.ast.binding_pattern_binding_identifier(SPAN, atom);
            self.ast.formal_parameter(SPAN, self.ast.vec(), pattern, NONE, NONE, false, None, false, false)
        }).collect();
        self.ast.formal_parameters(
            SPAN,
            ast::FormalParameterKind::FormalParameter,
            self.ast.vec_from_iter(params),
            NONE,
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
        let body = self.ast.function_body(
            SPAN,
            self.ast.vec(),
            self.ast.vec_from_iter(statements),
        );
        self.ast.arrow_function_expression(SPAN, is_expr, false, NONE, params, NONE, body)
    }

    /// Arrow function that always uses block body (never expression body).
    pub fn arrow_block(
        &self,
        params: FormalParameters<'a>,
        statements: impl IntoIterator<Item = Statement<'a>>,
    ) -> ArrowFunctionExpression<'a> {
        let body = self.ast.function_body(
            SPAN,
            self.ast.vec(),
            self.ast.vec_from_iter(statements),
        );
        self.ast.arrow_function_expression(SPAN, false, false, NONE, params, NONE, body)
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

    /// `() => expr` — zero-arg arrow wrapping a single expression.
    /// `() => expr` — zero-arg arrow wrapping an expression.
    /// Optimizes `() => fn()` (zero-arg call to identifier/member) into just `fn`.
    pub fn thunk(&self, expr: Expression<'a>) -> Expression<'a> {
        if let Expression::CallExpression(call) = &expr {
            if call.arguments.is_empty()
                && !call.optional
                && matches!(
                    &call.callee,
                    Expression::Identifier(_) | Expression::StaticMemberExpression(_)
                )
            {
                // Unwrap: move callee out of the CallExpression
                if let Expression::CallExpression(call) = expr {
                    let call = call.unbox();
                    return call.callee;
                }
            }
        }
        self.arrow_expr(self.no_params(), [self.expr_stmt(expr)])
    }

    /// `() => { ...stmts }` — zero-arg arrow wrapping a block body.
    pub fn thunk_block(&self, stmts: Vec<Statement<'a>>) -> Expression<'a> {
        self.arrow_block_expr(self.no_params(), stmts)
    }

    /// `left ?? right` — nullish coalescing.
    pub fn logical_coalesce(&self, left: Expression<'a>, right: Expression<'a>) -> Expression<'a> {
        self.ast.expression_logical(SPAN, left, ast::LogicalOperator::Coalesce, right)
    }

    pub fn function_decl(
        &self,
        id: BindingIdentifier<'a>,
        body: Vec<Statement<'a>>,
        params: FormalParameters<'a>,
    ) -> Function<'a> {
        let body = self.ast.alloc_function_body(
            SPAN,
            self.ast.vec(),
            self.ast.vec_from_iter(body),
        );
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

    /// Create a function expression: `function(params) { body }`
    pub fn function_expr(
        &self,
        params: FormalParameters<'a>,
        body: Vec<Statement<'a>>,
    ) -> Expression<'a> {
        let body = self.ast.alloc_function_body(
            SPAN,
            self.ast.vec(),
            self.ast.vec_from_iter(body),
        );
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

    /// Create params with a rest element: `...name`
    pub fn rest_params(&self, name: &str) -> FormalParameters<'a> {
        let atom = self.ast.atom(name);
        let pattern = self.ast.binding_pattern_binding_identifier(SPAN, atom);
        let rest_element = self.ast.binding_rest_element(SPAN, pattern);
        let rest = self.ast.formal_parameter_rest(SPAN, self.ast.vec(), rest_element, NONE);
        self.ast.formal_parameters(
            SPAN,
            ast::FormalParameterKind::FormalParameter,
            self.ast.vec(),
            Some(rest),
        )
    }

    /// `this` expression
    pub fn this_expr(&self) -> Expression<'a> {
        self.ast.expression_this(SPAN)
    }

    /// Call expression with an arbitrary `Expression` as callee (not just a string identifier).
    pub fn call_expr_callee<'short>(
        &self,
        callee: Expression<'a>,
        args: impl IntoIterator<Item = Arg<'a, 'short>>,
    ) -> Expression<'a> {
        let args = args.into_iter().map(|a| self.arg_to_argument(a));
        Expression::CallExpression(self.alloc(
            self.ast.call_expression(SPAN, callee, NONE, self.ast.vec_from_iter(args), false),
        ))
    }

    /// Optional call expression: `callee?.(...args)` — ChainExpression wrapping an optional CallExpression.
    pub fn maybe_call_expr<'short>(
        &self,
        callee: Expression<'a>,
        args: impl IntoIterator<Item = Arg<'a, 'short>>,
    ) -> Expression<'a> {
        let args = args.into_iter().map(|a| self.arg_to_argument(a));
        let call = self.ast.call_expression(SPAN, callee, NONE, self.ast.vec_from_iter(args), true);
        Expression::ChainExpression(self.alloc(
            self.ast.chain_expression(SPAN, ChainElement::CallExpression(self.alloc(call))),
        ))
    }

    /// `object?.member(args)` — optional member access with a normal call.
    pub fn optional_member_call_expr<'short>(
        &self,
        object: Expression<'a>,
        member: &str,
        args: impl IntoIterator<Item = Arg<'a, 'short>>,
    ) -> Expression<'a> {
        let property = self.ast.identifier_name(SPAN, self.ast.atom(member));
        let callee_inner = self.ast.static_member_expression(SPAN, object, property, true);
        let callee = Expression::StaticMemberExpression(self.alloc(callee_inner));
        let args = args.into_iter().map(|a| self.arg_to_argument(a));
        let call = self.ast.call_expression(SPAN, callee, NONE, self.ast.vec_from_iter(args), false);
        Expression::ChainExpression(self.alloc(
            self.ast.chain_expression(SPAN, ChainElement::CallExpression(self.alloc(call))),
        ))
    }

    // -----------------------------------------------------------------------
    // Member expressions
    // -----------------------------------------------------------------------

    pub fn static_member(
        &self,
        object: Expression<'a>,
        prop: &str,
    ) -> StaticMemberExpression<'a> {
        let property = self.ast.identifier_name(SPAN, self.ast.atom(prop));
        self.ast.static_member_expression(SPAN, object, property, false)
    }

    pub fn static_member_expr(&self, object: Expression<'a>, prop: &str) -> Expression<'a> {
        Expression::StaticMemberExpression(self.alloc(self.static_member(object, prop)))
    }

    // -----------------------------------------------------------------------
    // Template literals
    // -----------------------------------------------------------------------

    pub fn template_str_expr(&self, value: &str) -> Expression<'a> {
        Expression::TemplateLiteral(self.alloc(self.template_str(value)))
    }

    pub fn template_str(&self, value: &str) -> TemplateLiteral<'a> {
        let mut quasis = self.ast.vec();
        quasis.push(self.ast.template_element(
            SPAN,
            TemplateElementValue {
                cooked: None,
                raw: self.ast.atom(value),
            },
            true,
            false,
        ));
        self.ast.template_literal(SPAN, quasis, self.ast.vec())
    }

    /// Build a template literal from alternating string and expression parts.
    /// Wraps each expression with `?? ""` for string coercion.
    pub fn template_parts(&self, parts: impl IntoIterator<Item = TemplatePart<'a>>) -> TemplateLiteral<'a> {
        let mut quasis = self.ast.vec();
        let mut expressions = self.ast.vec();
        let parts: Vec<_> = parts.into_iter().collect();
        let n = parts.len();

        if n == 0 {
            quasis.push(self.ast.template_element(
                SPAN,
                TemplateElementValue { cooked: None, raw: Atom::from("") },
                true,
                false,
            ));
            return self.ast.template_literal(SPAN, quasis, expressions);
        }

        let mut pending = String::new();

        for (i, part) in parts.into_iter().enumerate() {
            let is_last = i == n - 1;
            match part {
                TemplatePart::Str(s) => {
                    pending.push_str(&s);
                    if is_last {
                        quasis.push(self.ast.template_element(
                            SPAN,
                            TemplateElementValue { cooked: None, raw: self.ast.atom(&pending) },
                            true,
                            false,
                        ));
                    }
                }
                TemplatePart::Expr(expr) => {
                    quasis.push(self.ast.template_element(
                        SPAN,
                        TemplateElementValue { cooked: None, raw: self.ast.atom(&pending) },
                        false,
                        false,
                    ));
                    pending.clear();

                    let empty_str = self.ast.atom("");
                    let wrapped = self.ast.expression_logical(
                        SPAN,
                        expr,
                        ast::LogicalOperator::Coalesce,
                        self.ast.expression_string_literal(SPAN, empty_str, None),
                    );
                    expressions.push(wrapped);

                    if is_last {
                        quasis.push(self.ast.template_element(
                            SPAN,
                            TemplateElementValue { cooked: None, raw: Atom::from("") },
                            true,
                            false,
                        ));
                    }
                }
            }
        }

        self.ast.template_literal(SPAN, quasis, expressions)
    }

    pub fn template_parts_expr(
        &self,
        parts: impl IntoIterator<Item = TemplatePart<'a>>,
    ) -> Expression<'a> {
        Expression::TemplateLiteral(self.alloc(self.template_parts(parts)))
    }

    // -----------------------------------------------------------------------
    // Expression parsing
    // -----------------------------------------------------------------------

    /// Parse a JS expression from text. Falls back to a string literal on parse failure.
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

    // -----------------------------------------------------------------------
    // Clone / move helpers
    // -----------------------------------------------------------------------

    pub fn clone_expr(&self, expr: &Expression<'a>) -> Expression<'a> {
        expr.clone_in(self.ast.allocator)
    }

    pub fn move_expr(&self, expr: &mut Expression<'a>) -> Expression<'a> {
        std::mem::replace(expr, self.cheap_expr())
    }

    pub fn cheap_expr(&self) -> Expression<'a> {
        self.ast.expression_boolean_literal(SPAN, false)
    }

    // -----------------------------------------------------------------------
    // Module-level helpers
    // -----------------------------------------------------------------------

    pub fn import_all(&self, specifier: &str, source: &str) -> Statement<'a> {
        let spec_atom = self.ast.atom(specifier);
        let source_atom = self.ast.atom(source);
        let spec = ImportDeclarationSpecifier::ImportNamespaceSpecifier(
            self.ast.alloc_import_namespace_specifier(
                SPAN,
                self.ast.binding_identifier(SPAN, spec_atom),
            ),
        );
        Statement::from(self.ast.module_declaration_import_declaration(
            SPAN,
            Some(self.ast.vec_from_array([spec])),
            self.ast.string_literal(SPAN, source_atom, None),
            None,
            NONE,
            ImportOrExportKind::Value,
        ))
    }

    pub fn export_default(&self, declaration: ExportDefaultDeclarationKind<'a>) -> Statement<'a> {
        let res = self.ast.alloc_export_default_declaration(
            SPAN,
            declaration,
        );
        Statement::from(ModuleDeclaration::ExportDefaultDeclaration(res))
    }

    pub fn program(&self, body: Vec<Statement<'a>>) -> Program<'a> {
        Program {
            node_id: Cell::new(OxcNodeId::DUMMY),
            body: self.ast.vec_from_iter(body),
            span: SPAN,
            comments: self.ast.vec(),
            directives: self.ast.vec(),
            hashbang: None,
            source_text: "",
            source_type: SourceType::mjs(),
            scope_id: Cell::from(None),
        }
    }

    // -----------------------------------------------------------------------
    // Object expressions (for spread attributes)
    // -----------------------------------------------------------------------

    pub fn object_expr(
        &self,
        props: impl IntoIterator<Item = ObjProp<'a>>,
    ) -> Expression<'a> {
        let properties = props.into_iter().map(|p| self.obj_prop_to_ast(p));
        Expression::ObjectExpression(self.alloc(
            self.ast.object_expression(SPAN, self.ast.vec_from_iter(properties)),
        ))
    }

    fn obj_prop_to_ast(&self, prop: ObjProp<'a>) -> ast::ObjectPropertyKind<'a> {
        match prop {
            ObjProp::KeyValue(key, value) => {
                let key_node = if key.contains('-') {
                    ast::PropertyKey::StringLiteral(
                        self.alloc(self.str_lit(key)),
                    )
                } else {
                    let key_atom = self.ast.atom(key);
                    ast::PropertyKey::StaticIdentifier(
                        self.alloc(self.ast.identifier_name(SPAN, key_atom)),
                    )
                };
                let obj_prop = self.ast.object_property(
                    SPAN,
                    ast::PropertyKind::Init,
                    key_node,
                    value,
                    false,  // method
                    false,  // shorthand
                    false,  // computed
                );
                ast::ObjectPropertyKind::ObjectProperty(self.alloc(obj_prop))
            }
            ObjProp::Shorthand(name) => {
                let name_atom = self.ast.atom(name);
                let key_node = ast::PropertyKey::StaticIdentifier(
                    self.alloc(self.ast.identifier_name(SPAN, name_atom)),
                );
                let value = self.rid_expr(name);
                let obj_prop = self.ast.object_property(
                    SPAN,
                    ast::PropertyKind::Init,
                    key_node,
                    value,
                    false,  // method
                    true,   // shorthand
                    false,  // computed
                );
                ast::ObjectPropertyKind::ObjectProperty(self.alloc(obj_prop))
            }
            ObjProp::Spread(expr) => {
                let spread = self.ast.spread_element(SPAN, expr);
                ast::ObjectPropertyKind::SpreadProperty(self.alloc(spread))
            }
            ObjProp::Getter(name, expr) => {
                let name_atom = self.ast.atom(name);
                let key_node = ast::PropertyKey::StaticIdentifier(
                    self.alloc(self.ast.identifier_name(SPAN, name_atom)),
                );
                let body = self.ast.alloc_function_body(
                    SPAN,
                    self.ast.vec(),
                    self.ast.vec_from_array([self.return_stmt(expr)]),
                );
                let getter = self.ast.function(
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
                let value = Expression::FunctionExpression(self.alloc(getter));
                let obj_prop = self.ast.object_property(
                    SPAN,
                    ast::PropertyKind::Get,
                    key_node,
                    value,
                    false,  // method
                    false,  // shorthand
                    false,  // computed
                );
                ast::ObjectPropertyKind::ObjectProperty(self.alloc(obj_prop))
            }
        }
    }
}

/// Property in an object literal expression.
pub enum ObjProp<'a> {
    /// `key: value`
    KeyValue(&'a str, Expression<'a>),
    /// `name` (property shorthand, equivalent to `name: name`)
    Shorthand(&'a str),
    /// `...expr`
    Spread(Expression<'a>),
    /// `get name() { return expr }`
    Getter(&'a str, Expression<'a>),
}
