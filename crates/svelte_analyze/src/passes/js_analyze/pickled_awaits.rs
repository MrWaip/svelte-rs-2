use oxc_ast::ast::{
    Argument, ArrayExpression, AssignmentExpression, AwaitExpression, CallExpression,
    ConditionalExpression, Expression, MemberExpression, NewExpression, ObjectExpression,
    SequenceExpression, TaggedTemplateExpression, TemplateLiteral,
};
use oxc_ast_visit::walk::{walk_arrow_function_expression, walk_expression, walk_function};
use oxc_ast_visit::Visit;
use oxc_semantic::ScopeFlags;
use smallvec::SmallVec;

use crate::types::data::{AnalysisData, ExprHandle, ParserResult};

pub(crate) fn classify_pickled_awaits(parsed: &ParserResult<'_>, data: &mut AnalysisData) {
    let expr_handles: Vec<_> = data
        .expressions
        .iter()
        .filter_map(|(node_id, _)| {
            data.template_semantics
                .node_expr_handles
                .get(node_id)
                .copied()
        })
        .collect();
    let attr_handles: Vec<_> = data
        .attr_expressions
        .iter()
        .filter_map(|(node_id, _)| {
            data.template_semantics
                .attr_expr_handles
                .get(node_id)
                .copied()
        })
        .collect();

    collect_pickled_await_offsets(parsed, data, expr_handles.into_iter());
    collect_pickled_await_offsets(parsed, data, attr_handles.into_iter());
}

fn collect_pickled_await_offsets(
    parsed: &ParserResult<'_>,
    data: &mut AnalysisData,
    handles: impl Iterator<Item = ExprHandle>,
) {
    for handle in handles {
        let Some(expr) = parsed.expr(handle) else {
            continue;
        };
        let mut collector = PickledAwaitCollector::new();
        collector.visit_expression(expr);
        data.pickled_await_offsets.extend_offsets(collector.offsets);
    }
}

struct PickledAwaitCollector {
    offsets: SmallVec<[u32; 4]>,
    last_stack: Vec<bool>,
    fn_depth: u32,
}

impl PickledAwaitCollector {
    fn new() -> Self {
        Self {
            offsets: SmallVec::new(),
            last_stack: vec![true],
            fn_depth: 0,
        }
    }

    fn current_is_last(&self) -> bool {
        self.last_stack.last().copied().unwrap_or(true)
    }

    fn visit_child(&mut self, expr: &Expression<'_>, is_last: bool) {
        self.last_stack.push(is_last);
        self.visit_expression(expr);
        self.last_stack.pop();
    }

    fn visit_argument(&mut self, arg: &Argument<'_>, is_last: bool) {
        if let Some(expr) = arg.as_expression() {
            self.visit_child(expr, is_last);
        }
    }
}

impl<'a> Visit<'a> for PickledAwaitCollector {
    fn visit_expression(&mut self, expr: &Expression<'a>) {
        if let Expression::AwaitExpression(await_expr) = expr {
            if self.fn_depth == 0 && !self.current_is_last() {
                self.offsets.push(await_expr.span.start);
            }
        }
        walk_expression(self, expr);
    }

    fn visit_await_expression(&mut self, expr: &AwaitExpression<'a>) {
        self.visit_child(&expr.argument, true);
    }

    fn visit_array_expression(&mut self, expr: &ArrayExpression<'a>) {
        let last = expr.elements.len().saturating_sub(1);
        for (index, elem) in expr.elements.iter().enumerate() {
            if let Some(elem_expr) = elem.as_expression() {
                self.visit_child(elem_expr, self.current_is_last() && index == last);
            }
        }
    }

    fn visit_assignment_expression(&mut self, expr: &AssignmentExpression<'a>) {
        self.visit_child(&expr.right, self.current_is_last());
    }

    fn visit_call_expression(&mut self, expr: &CallExpression<'a>) {
        self.visit_child(&expr.callee, false);
        let last = expr.arguments.len().saturating_sub(1);
        for (index, arg) in expr.arguments.iter().enumerate() {
            self.visit_argument(arg, self.current_is_last() && index == last);
        }
    }

    fn visit_conditional_expression(&mut self, expr: &ConditionalExpression<'a>) {
        self.visit_child(&expr.test, false);
        self.visit_child(&expr.consequent, self.current_is_last());
        self.visit_child(&expr.alternate, self.current_is_last());
    }

    fn visit_member_expression(&mut self, expr: &MemberExpression<'a>) {
        match expr {
            MemberExpression::ComputedMemberExpression(member) => {
                self.visit_child(&member.object, false);
                self.visit_child(&member.expression, self.current_is_last());
            }
            MemberExpression::StaticMemberExpression(member) => {
                self.visit_child(&member.object, self.current_is_last());
            }
            MemberExpression::PrivateFieldExpression(member) => {
                self.visit_child(&member.object, self.current_is_last());
            }
        }
    }

    fn visit_new_expression(&mut self, expr: &NewExpression<'a>) {
        self.visit_child(&expr.callee, false);
        let last = expr.arguments.len().saturating_sub(1);
        for (index, arg) in expr.arguments.iter().enumerate() {
            self.visit_argument(arg, self.current_is_last() && index == last);
        }
    }

    fn visit_object_expression(&mut self, expr: &ObjectExpression<'a>) {
        let last = expr.properties.len().saturating_sub(1);
        for (index, prop) in expr.properties.iter().enumerate() {
            match prop {
                oxc_ast::ast::ObjectPropertyKind::ObjectProperty(prop) => {
                    self.visit_child(&prop.value, self.current_is_last() && index == last);
                }
                oxc_ast::ast::ObjectPropertyKind::SpreadProperty(prop) => {
                    self.visit_child(&prop.argument, self.current_is_last() && index == last);
                }
            }
        }
    }

    fn visit_sequence_expression(&mut self, expr: &SequenceExpression<'a>) {
        let last = expr.expressions.len().saturating_sub(1);
        for (index, child) in expr.expressions.iter().enumerate() {
            self.visit_child(child, self.current_is_last() && index == last);
        }
    }

    fn visit_tagged_template_expression(&mut self, expr: &TaggedTemplateExpression<'a>) {
        self.visit_child(&expr.tag, false);
        self.visit_template_literal(&expr.quasi);
    }

    fn visit_template_literal(&mut self, expr: &TemplateLiteral<'a>) {
        let last = expr.expressions.len().saturating_sub(1);
        for (index, child) in expr.expressions.iter().enumerate() {
            self.visit_child(child, self.current_is_last() && index == last);
        }
    }

    fn visit_binary_expression(&mut self, expr: &oxc_ast::ast::BinaryExpression<'a>) {
        self.visit_child(&expr.left, false);
        self.visit_child(&expr.right, self.current_is_last());
    }

    fn visit_logical_expression(&mut self, expr: &oxc_ast::ast::LogicalExpression<'a>) {
        self.visit_child(&expr.left, false);
        self.visit_child(&expr.right, self.current_is_last());
    }

    fn visit_arrow_function_expression(
        &mut self,
        arrow: &oxc_ast::ast::ArrowFunctionExpression<'a>,
    ) {
        self.fn_depth += 1;
        walk_arrow_function_expression(self, arrow);
        self.fn_depth -= 1;
    }

    fn visit_function(&mut self, func: &oxc_ast::ast::Function<'a>, flags: ScopeFlags) {
        self.fn_depth += 1;
        walk_function(self, func, flags);
        self.fn_depth -= 1;
    }
}
