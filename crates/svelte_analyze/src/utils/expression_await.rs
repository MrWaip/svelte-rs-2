use oxc_ast::ast::{
    ArrowFunctionExpression, AwaitExpression, CallExpression, Expression, Function, SpreadElement,
    Statement,
};
use oxc_ast_visit::Visit;
use oxc_ast_visit::walk::walk_call_expression;
use oxc_semantic::ScopeFlags;

struct AwaitFinder {
    found: bool,
}

impl<'a> Visit<'a> for AwaitFinder {
    fn visit_await_expression(&mut self, _expr: &AwaitExpression<'a>) {
        self.found = true;
    }

    fn visit_arrow_function_expression(&mut self, _arrow: &ArrowFunctionExpression<'a>) {}

    fn visit_function(&mut self, _func: &Function<'a>, _flags: ScopeFlags) {}
}

pub(crate) fn expression_has_await(expr: &Expression<'_>) -> bool {
    let mut finder = AwaitFinder { found: false };
    finder.visit_expression(expr);
    finder.found
}

pub(crate) fn statement_has_await(stmt: &Statement<'_>) -> bool {
    let mut finder = AwaitFinder { found: false };
    finder.visit_statement(stmt);
    finder.found
}

struct CallOrAwaitFinder {
    found: bool,
}

impl<'a> Visit<'a> for CallOrAwaitFinder {
    fn visit_await_expression(&mut self, _expr: &AwaitExpression<'a>) {
        self.found = true;
    }

    fn visit_call_expression(&mut self, expr: &CallExpression<'a>) {
        self.found = true;
        walk_call_expression(self, expr);
    }

    fn visit_spread_element(&mut self, _spread: &SpreadElement<'a>) {
        self.found = true;
    }

    fn visit_arrow_function_expression(&mut self, _arrow: &ArrowFunctionExpression<'a>) {}

    fn visit_function(&mut self, _func: &Function<'a>, _flags: ScopeFlags) {}
}

pub fn expression_calls_or_awaits(expr: &Expression<'_>) -> bool {
    let mut finder = CallOrAwaitFinder { found: false };
    finder.visit_expression(expr);
    finder.found
}

#[cfg(test)]
mod tests {
    use super::expression_has_await;
    use oxc_allocator::Allocator;
    use oxc_ast::ast::Statement;
    use oxc_parser::Parser;
    use oxc_span::SourceType;

    fn has_await(source: &str) -> bool {
        let allocator = Allocator::default();
        let ret = Parser::new(&allocator, source, SourceType::mjs()).parse();
        let stmt = ret.program.body.first().expect("one statement");
        let expr = match stmt {
            Statement::ExpressionStatement(s) => &s.expression,
            _ => panic!("expected expression statement"),
        };
        expression_has_await(expr)
    }

    #[test]
    fn direct_await() {
        assert!(has_await("await foo()"));
    }

    #[test]
    fn nested_await_in_call() {
        assert!(has_await("foo(await bar())"));
    }

    #[test]
    fn await_in_member_and_ternary() {
        assert!(has_await("cond ? await a() : b"));
    }

    #[test]
    fn no_await() {
        assert!(!has_await("foo() + bar"));
    }

    #[test]
    fn await_inside_arrow_does_not_count() {
        assert!(!has_await("async () => await foo()"));
    }

    #[test]
    fn await_inside_function_does_not_count() {
        assert!(!has_await("(async function () { return await foo(); })"));
    }

    #[test]
    fn await_outside_arrow_still_counts() {
        assert!(has_await("(async () => await inner())(await outer())"));
    }
}
