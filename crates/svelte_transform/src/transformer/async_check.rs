use oxc_ast::ast::{
    ArrowFunctionExpression, AwaitExpression, Class, Expression, ForOfStatement, Function,
};
use oxc_ast_visit::{Visit, walk::{walk_await_expression, walk_for_of_statement}};
use oxc_semantic::ScopeFlags;

pub(crate) fn is_expression_async(expr: &Expression<'_>) -> bool {
    let mut v = AwaitFinder { found: false };
    v.visit_expression(expr);
    v.found
}

struct AwaitFinder {
    found: bool,
}

impl<'a> Visit<'a> for AwaitFinder {
    fn visit_await_expression(&mut self, expr: &AwaitExpression<'a>) {
        self.found = true;
        walk_await_expression(self, expr);
    }

    fn visit_for_of_statement(&mut self, stmt: &ForOfStatement<'a>) {
        if stmt.r#await {
            self.found = true;
        }
        walk_for_of_statement(self, stmt);
    }

    fn visit_function(&mut self, _: &Function<'a>, _flags: ScopeFlags) {}
    fn visit_arrow_function_expression(&mut self, _: &ArrowFunctionExpression<'a>) {}
    fn visit_class(&mut self, _: &Class<'a>) {}
}
