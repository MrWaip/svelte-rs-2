use oxc_ast::ast::Expression;
use oxc_ast_visit::{Visit, walk};

struct SimpleExprChecker(bool);

impl<'a> Visit<'a> for SimpleExprChecker {
    fn visit_expression(&mut self, expr: &Expression<'a>) {
        match expr {
            Expression::NumericLiteral(_)
            | Expression::StringLiteral(_)
            | Expression::BooleanLiteral(_)
            | Expression::NullLiteral(_)
            | Expression::Identifier(_)
            | Expression::ArrowFunctionExpression(_)
            | Expression::FunctionExpression(_) => {}
            Expression::ConditionalExpression(_)
            | Expression::BinaryExpression(_)
            | Expression::LogicalExpression(_) => walk::walk_expression(self, expr),
            _ => self.0 = false,
        }
    }
}

pub fn is_simple_expression(expr: &Expression<'_>) -> bool {
    let mut checker = SimpleExprChecker(true);
    checker.visit_expression(expr);
    checker.0
}
