use oxc_ast::ast::{
    Argument, ArrayExpression, ArrowFunctionExpression, AssignmentExpression, AwaitExpression,
    BinaryExpression, CallExpression, ConditionalExpression, Expression, Function,
    LogicalExpression, MemberExpression, NewExpression, ObjectExpression, ObjectPropertyKind,
    SequenceExpression, Statement, TaggedTemplateExpression, TemplateLiteral,
};
use oxc_ast_visit::Visit;
use oxc_ast_visit::walk::{walk_arrow_function_expression, walk_expression, walk_function};
use oxc_semantic::ScopeFlags;
use smallvec::SmallVec;
use svelte_ast::{Component, FragmentId, Node, OxcNodeId};

use crate::types::data::JsAst;

use super::data::{AwaitSemantics, AwaitSemanticsStore};

pub(crate) fn build(component: &Component, parsed: &JsAst<'_>) -> AwaitSemanticsStore {
    let mut store = AwaitSemanticsStore::new();

    for expr in parsed.iter_exprs() {
        classify_expression(expr, AwaitSemantics::TerminalInConstruct, &mut store);
    }

    for stmt in parsed.iter_stmts() {
        classify_statement(stmt, &mut store);
    }

    for expr_id in fragment_interpolations(component) {
        let Some(expr) = parsed.expr(expr_id) else {
            continue;
        };
        classify_expression(
            expr,
            AwaitSemantics::TerminalInFragmentInterpolation,
            &mut store,
        );
    }

    store
}

fn fragment_interpolations(component: &Component) -> Vec<OxcNodeId> {
    let store = &component.store;
    let mut out = Vec::new();

    for index in 0..store.fragments_len() {
        let fragment = store.fragment(FragmentId(index));
        let inlined_into_element = fragment
            .owner
            .is_some_and(|owner| matches!(store.get(owner), Node::Element(_)));
        if inlined_into_element {
            continue;
        }

        for &node_id in &fragment.nodes {
            if let Node::ExpressionTag(tag) = store.get(node_id) {
                out.push(tag.expression.id());
            }
        }
    }

    out
}

fn classify_expression(
    expr: &Expression<'_>,
    terminal: AwaitSemantics,
    store: &mut AwaitSemanticsStore,
) {
    let mut collector = AwaitCollector::new(terminal);
    collector.visit_expression(expr);
    for (id, semantics) in collector.entries {
        store.set(id, semantics);
    }
}

fn classify_statement(stmt: &Statement<'_>, store: &mut AwaitSemanticsStore) {
    let mut collector = AwaitCollector::new(AwaitSemantics::TerminalInConstruct);
    collector.visit_statement(stmt);
    for (id, semantics) in collector.entries {
        store.set(id, semantics);
    }
}

struct AwaitCollector {
    terminal: AwaitSemantics,
    entries: SmallVec<[(OxcNodeId, AwaitSemantics); 4]>,
    last_stack: Vec<bool>,
    fn_depth: u32,
}

impl AwaitCollector {
    fn new(terminal: AwaitSemantics) -> Self {
        Self {
            terminal,
            entries: SmallVec::new(),
            last_stack: vec![true],
            fn_depth: 0,
        }
    }

    fn current_is_last(&self) -> bool {
        self.last_stack.last().copied().unwrap_or(true)
    }

    fn semantics_here(&self) -> AwaitSemantics {
        if self.fn_depth > 0 {
            AwaitSemantics::Detached
        } else if self.current_is_last() {
            self.terminal
        } else {
            AwaitSemantics::NonTerminal
        }
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

impl<'a> Visit<'a> for AwaitCollector {
    fn visit_expression(&mut self, expr: &Expression<'a>) {
        if let Expression::AwaitExpression(await_expr) = expr {
            let semantics = self.semantics_here();
            self.entries.push((await_expr.node_id(), semantics));
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
                ObjectPropertyKind::ObjectProperty(prop) => {
                    self.visit_child(&prop.value, self.current_is_last() && index == last);
                }
                ObjectPropertyKind::SpreadProperty(prop) => {
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

    fn visit_binary_expression(&mut self, expr: &BinaryExpression<'a>) {
        self.visit_child(&expr.left, false);
        self.visit_child(&expr.right, self.current_is_last());
    }

    fn visit_logical_expression(&mut self, expr: &LogicalExpression<'a>) {
        self.visit_child(&expr.left, false);
        self.visit_child(&expr.right, self.current_is_last());
    }

    fn visit_arrow_function_expression(&mut self, arrow: &ArrowFunctionExpression<'a>) {
        self.fn_depth += 1;
        walk_arrow_function_expression(self, arrow);
        self.fn_depth -= 1;
    }

    fn visit_function(&mut self, func: &Function<'a>, flags: ScopeFlags) {
        self.fn_depth += 1;
        walk_function(self, func, flags);
        self.fn_depth -= 1;
    }
}
