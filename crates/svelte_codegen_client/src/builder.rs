use oxc_allocator::{Allocator, Box, CloneIn};
use oxc_ast::{
    ast::{
        self, Argument, ArrowFunctionExpression, AssignmentTarget,
        BindingIdentifier, BindingPatternKind, CallExpression, ExportDefaultDeclarationKind,
        Expression, FormalParameters, Function, FunctionType, IdentifierReference,
        ImportDeclarationSpecifier, ImportOrExportKind, ModuleDeclaration, ModuleExportName,
        NumericLiteral, Program, Statement,
        StaticMemberExpression, StringLiteral, TemplateElement, TemplateElementValue,
        TemplateLiteral, VariableDeclarationKind,
    },
    AstBuilder, NONE,
};
use oxc_span::{Atom, SourceType, SPAN};
use std::cell::Cell;

pub enum Arg<'a, 'short> {
    Str(String),
    Num(f64),
    Ident(&'short str),
    IdentRef(IdentifierReference<'a>),
    Expr(Expression<'a>),
    Arrow(ArrowFunctionExpression<'a>),
    Bool(bool),
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

    // -----------------------------------------------------------------------
    // Identifiers
    // -----------------------------------------------------------------------

    pub fn bid(&self, name: &str) -> BindingIdentifier<'a> {
        self.ast.binding_identifier(SPAN, name)
    }

    pub fn rid(&self, name: &str) -> IdentifierReference<'a> {
        self.ast.identifier_reference(SPAN, name)
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

    pub fn num(&self, value: f64) -> NumericLiteral<'a> {
        self.ast.numeric_literal(SPAN, value, None, ast::NumberBase::Decimal)
    }

    pub fn num_expr(&self, value: f64) -> Expression<'a> {
        Expression::NumericLiteral(self.alloc(self.num(value)))
    }

    pub fn str_lit(&self, value: &str) -> StringLiteral<'a> {
        self.ast.string_literal(SPAN, value, None)
    }

    pub fn str_expr(&self, value: &str) -> Expression<'a> {
        Expression::StringLiteral(self.alloc(self.str_lit(value)))
    }

    // -----------------------------------------------------------------------
    // Call expressions
    // -----------------------------------------------------------------------

    pub fn call<'short>(
        &self,
        callee: &str,
        args: impl IntoIterator<Item = Arg<'a, 'short>>,
    ) -> CallExpression<'a> {
        let ident = self.ast.expression_identifier_reference(SPAN, callee);
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

    pub fn let_stmt_init(&self, name: &str, init: Expression<'a>) -> Statement<'a> {
        self.var_decl_stmt(name, init, VariableDeclarationKind::Let)
    }

    /// `let name;` — uninitialized variable declaration.
    pub fn let_stmt(&self, name: &str) -> Statement<'a> {
        let pattern = self.ast.binding_pattern(
            BindingPatternKind::BindingIdentifier(self.alloc(self.bid(name))),
            NONE,
            false,
        );
        let decl = self.ast.variable_declarator(
            SPAN,
            VariableDeclarationKind::Let,
            pattern,
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

    fn var_decl_stmt(
        &self,
        name: &str,
        init: Expression<'a>,
        kind: VariableDeclarationKind,
    ) -> Statement<'a> {
        let pattern = self.ast.binding_pattern(
            BindingPatternKind::BindingIdentifier(self.alloc(self.bid(name))),
            NONE,
            false,
        );
        let decl = self.ast.variable_declarator(SPAN, kind, pattern, Some(init), false);
        let declaration = self.ast.variable_declaration(
            SPAN,
            kind,
            self.ast.vec_from_array([decl]),
            false,
        );
        Statement::VariableDeclaration(self.alloc(declaration))
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
            AssignLeft::Ident(ref name) => {
                AssignmentTarget::AssignmentTargetIdentifier(self.alloc(
                    self.ast.identifier_reference(SPAN, name.as_str()),
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
        let params = items.into_iter().map(|v| {
            let kind = self.ast.binding_pattern_kind_binding_identifier(SPAN, v.as_ref());
            let pattern = self.ast.binding_pattern(kind, NONE, false);
            self.ast.formal_parameter(SPAN, self.ast.vec(), pattern, None, false, false)
        });
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

    pub fn arrow_expr(
        &self,
        params: FormalParameters<'a>,
        statements: impl IntoIterator<Item = Statement<'a>>,
    ) -> Expression<'a> {
        Expression::ArrowFunctionExpression(self.alloc(self.arrow(params, statements)))
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

    // -----------------------------------------------------------------------
    // Member expressions
    // -----------------------------------------------------------------------

    pub fn static_member(
        &self,
        object: Expression<'a>,
        prop: &str,
    ) -> StaticMemberExpression<'a> {
        let property = self.ast.identifier_name(SPAN, prop);
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
        quasis.push(TemplateElement {
            span: SPAN,
            tail: true,
            value: TemplateElementValue {
                cooked: None,
                raw: self.ast.atom(value),
            },
        });
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
                true,
                TemplateElementValue { cooked: None, raw: Atom::from("") },
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
                            true,
                            TemplateElementValue { cooked: None, raw: self.ast.atom(&pending) },
                        ));
                    }
                }
                TemplatePart::Expr(expr) => {
                    quasis.push(self.ast.template_element(
                        SPAN,
                        false,
                        TemplateElementValue { cooked: None, raw: self.ast.atom(&pending) },
                    ));
                    pending.clear();

                    let wrapped = self.ast.expression_logical(
                        SPAN,
                        expr,
                        ast::LogicalOperator::Coalesce,
                        self.ast.expression_string_literal(SPAN, "", None),
                    );
                    expressions.push(wrapped);

                    if is_last {
                        quasis.push(self.ast.template_element(
                            SPAN,
                            true,
                            TemplateElementValue { cooked: None, raw: Atom::from("") },
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
    // Clone / move helpers
    // -----------------------------------------------------------------------

    pub fn clone_expr(&self, expr: &Expression<'a>) -> Expression<'a> {
        expr.clone_in(self.ast.allocator)
    }

    pub fn move_expr(&self, expr: &mut Expression<'a>) -> Expression<'a> {
        self.ast.move_expression(expr)
    }

    pub fn cheap_expr(&self) -> Expression<'a> {
        self.ast.expression_boolean_literal(SPAN, false)
    }

    // -----------------------------------------------------------------------
    // Module-level helpers
    // -----------------------------------------------------------------------

    pub fn import_all(&self, specifier: &str, source: &str) -> Statement<'a> {
        let spec = ImportDeclarationSpecifier::ImportNamespaceSpecifier(
            self.ast.alloc_import_namespace_specifier(
                SPAN,
                self.ast.binding_identifier(SPAN, specifier),
            ),
        );
        Statement::from(self.ast.module_declaration_import_declaration(
            SPAN,
            Some(self.ast.vec_from_array([spec])),
            self.ast.string_literal(SPAN, source, None),
            None,
            NONE,
            ImportOrExportKind::Value,
        ))
    }

    pub fn export_default(&self, declaration: ExportDefaultDeclarationKind<'a>) -> Statement<'a> {
        let ident = self.ast.identifier_name(SPAN, "default");
        let res = self.ast.alloc_export_default_declaration(
            SPAN,
            declaration,
            ModuleExportName::IdentifierName(ident),
        );
        Statement::from(ModuleDeclaration::ExportDefaultDeclaration(res))
    }

    pub fn program(&self, body: Vec<Statement<'a>>) -> Program<'a> {
        Program {
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
            self.ast.object_expression(SPAN, self.ast.vec_from_iter(properties), None),
        ))
    }

    fn obj_prop_to_ast(&self, prop: ObjProp<'a>) -> ast::ObjectPropertyKind<'a> {
        match prop {
            ObjProp::KeyValue(key, value) => {
                let key_node = ast::PropertyKey::StaticIdentifier(
                    self.alloc(self.ast.identifier_name(SPAN, key)),
                );
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
                let key_node = ast::PropertyKey::StaticIdentifier(
                    self.alloc(self.ast.identifier_name(SPAN, name)),
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
}
