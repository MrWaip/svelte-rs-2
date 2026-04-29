use super::*;

impl<'a> Builder<'a> {
    pub fn new(allocator: &'a Allocator) -> Self {
        Self {
            ast: AstBuilder::new(allocator),
        }
    }

    pub fn alloc<T>(&self, value: T) -> Box<'a, T> {
        self.ast.alloc(value)
    }

    pub fn alloc_str(&self, s: &str) -> &'a str {
        self.ast.allocator.alloc_str(s)
    }

    pub fn bid(&self, name: &str) -> BindingIdentifier<'a> {
        self.ast.binding_identifier(SPAN, self.ast.atom(name))
    }

    pub fn rid(&self, name: &str) -> IdentifierReference<'a> {
        self.ast.identifier_reference(SPAN, self.ast.atom(name))
    }

    pub fn rid_expr(&self, name: &str) -> Expression<'a> {
        Expression::Identifier(self.alloc(self.rid(name)))
    }

    pub fn bool_expr(&self, value: bool) -> Expression<'a> {
        Expression::BooleanLiteral(self.alloc(self.ast.boolean_literal(SPAN, value)))
    }

    pub fn null_expr(&self) -> Expression<'a> {
        self.ast.expression_null_literal(SPAN)
    }

    pub fn void_zero_expr(&self) -> Expression<'a> {
        self.ast
            .expression_unary(SPAN, ast::UnaryOperator::Void, self.num_expr(0.0))
    }

    pub fn num(&self, value: f64) -> NumericLiteral<'a> {
        self.ast
            .numeric_literal(SPAN, value, None, ast::NumberBase::Decimal)
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
        Expression::ArrayExpression(self.alloc(self.ast.array_expression(SPAN, self.ast.vec())))
    }

    pub fn clone_expr(&self, expr: &Expression<'a>) -> Expression<'a> {
        expr.clone_in(self.ast.allocator)
    }

    pub fn move_expr(&self, expr: &mut Expression<'a>) -> Expression<'a> {
        std::mem::replace(expr, self.cheap_expr())
    }

    pub fn cheap_expr(&self) -> Expression<'a> {
        self.ast.expression_boolean_literal(SPAN, false)
    }

    pub fn seq_expr(&self, exprs: [Expression<'a>; 2]) -> Expression<'a> {
        self.ast
            .expression_sequence(SPAN, self.ast.vec_from_iter(exprs))
    }
}
