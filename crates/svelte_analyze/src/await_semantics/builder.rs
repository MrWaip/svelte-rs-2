use oxc_ast::ast::{
    Argument, ArrayExpression, ArrayExpressionElement, ArrowFunctionExpression,
    AssignmentExpression, AwaitExpression, BinaryExpression, CallExpression, ConditionalExpression,
    Expression, Function, LogicalExpression, MemberExpression, NewExpression, ObjectExpression,
    ObjectPropertyKind, SequenceExpression, Statement, TaggedTemplateExpression, TemplateLiteral,
};
use oxc_ast_visit::Visit;
use oxc_ast_visit::walk::{
    walk_arrow_function_expression, walk_assignment_target, walk_expression, walk_function,
};
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
        classify_statement(stmt, AwaitSemantics::TerminalInConstruct, &mut store);
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

    for stmt_id in const_tag_declarations(component) {
        let Some(stmt) = parsed.stmt(stmt_id) else {
            continue;
        };
        classify_statement(stmt, AwaitSemantics::NonTerminal, &mut store);
    }

    store
}

fn const_tag_declarations(component: &Component) -> Vec<OxcNodeId> {
    let store = &component.store;
    let mut out = Vec::new();

    for node in store.iter_nodes() {
        let Node::ConstTag(tag) = node else {
            continue;
        };
        out.push(tag.decl.id());
    }

    out
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

fn classify_statement(
    stmt: &Statement<'_>,
    terminal: AwaitSemantics,
    store: &mut AwaitSemanticsStore,
) {
    let mut collector = AwaitCollector::new(terminal);
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

    fn in_non_tail_position(&mut self, visit: impl FnOnce(&mut Self)) {
        self.last_stack.push(false);
        visit(self);
        self.last_stack.pop();
    }

    fn visit_argument(&mut self, arg: &Argument<'_>, is_last: bool) {
        match arg {
            Argument::SpreadElement(spread) => self.visit_child(&spread.argument, false),
            _ => {
                if let Some(expr) = arg.as_expression() {
                    self.visit_child(expr, is_last);
                }
            }
        }
    }
}

fn positions_its_own_children(expr: &Expression<'_>) -> bool {
    matches!(
        expr,
        Expression::ArrayExpression(_)
            | Expression::AssignmentExpression(_)
            | Expression::AwaitExpression(_)
            | Expression::BinaryExpression(_)
            | Expression::CallExpression(_)
            | Expression::ComputedMemberExpression(_)
            | Expression::ConditionalExpression(_)
            | Expression::LogicalExpression(_)
            | Expression::NewExpression(_)
            | Expression::ObjectExpression(_)
            | Expression::PrivateFieldExpression(_)
            | Expression::SequenceExpression(_)
            | Expression::StaticMemberExpression(_)
            | Expression::TaggedTemplateExpression(_)
            | Expression::TemplateLiteral(_)
            | Expression::ParenthesizedExpression(_)
            | Expression::TSAsExpression(_)
            | Expression::TSInstantiationExpression(_)
            | Expression::TSNonNullExpression(_)
            | Expression::TSSatisfiesExpression(_)
            | Expression::TSTypeAssertion(_)
    )
}

impl<'a> Visit<'a> for AwaitCollector {
    fn visit_expression(&mut self, expr: &Expression<'a>) {
        if let Expression::AwaitExpression(await_expr) = expr {
            let semantics = self.semantics_here();
            self.entries.push((await_expr.node_id(), semantics));
        }

        if positions_its_own_children(expr) {
            walk_expression(self, expr);
        } else {
            self.in_non_tail_position(|collector| walk_expression(collector, expr));
        }
    }

    fn visit_await_expression(&mut self, expr: &AwaitExpression<'a>) {
        self.visit_child(&expr.argument, false);
    }

    fn visit_array_expression(&mut self, expr: &ArrayExpression<'a>) {
        let last = expr.elements.len().saturating_sub(1);
        for (index, elem) in expr.elements.iter().enumerate() {
            match elem {
                ArrayExpressionElement::SpreadElement(spread) => {
                    self.visit_child(&spread.argument, false);
                }
                _ => {
                    if let Some(elem_expr) = elem.as_expression() {
                        self.visit_child(elem_expr, self.current_is_last() && index == last);
                    }
                }
            }
        }
    }

    fn visit_assignment_expression(&mut self, expr: &AssignmentExpression<'a>) {
        self.in_non_tail_position(|collector| walk_assignment_target(collector, &expr.left));
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
                self.visit_child(&member.expression, false);
            }
            MemberExpression::StaticMemberExpression(member) => {
                self.visit_child(&member.object, false);
            }
            MemberExpression::PrivateFieldExpression(member) => {
                self.visit_child(&member.object, false);
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
                    if let Some(key) = prop.key.as_expression() {
                        self.visit_child(key, false);
                    }
                    self.visit_child(&prop.value, self.current_is_last() && index == last);
                }
                ObjectPropertyKind::SpreadProperty(prop) => {
                    self.visit_child(&prop.argument, false);
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
        self.in_non_tail_position(|collector| collector.visit_template_literal(&expr.quasi));
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
