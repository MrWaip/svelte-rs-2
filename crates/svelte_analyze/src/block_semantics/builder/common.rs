use oxc_ast::ast::{
    AwaitExpression, BindingPattern, CallExpression, Expression, IdentifierReference, Statement,
    VariableDeclarator,
};
use oxc_ast_visit::Visit;
use smallvec::SmallVec;
use svelte_component_semantics::{ComponentSemantics, OxcNodeId, ReferenceId};

use crate::types::data::BlockerData;

pub(super) fn declarator_from_stmt<'a>(
    stmt: &'a Statement<'a>,
) -> Option<&'a VariableDeclarator<'a>> {
    let Statement::VariableDeclaration(decl) = stmt else {
        return None;
    };
    decl.declarations.first()
}

pub(super) fn binding_ident_of<'a>(
    pattern: &'a BindingPattern<'a>,
) -> Option<&'a oxc_ast::ast::BindingIdentifier<'a>> {
    match pattern {
        BindingPattern::BindingIdentifier(ident) => Some(ident),
        _ => None,
    }
}

pub(super) fn binding_pattern_node_id(pattern: &BindingPattern<'_>) -> OxcNodeId {
    match pattern {
        BindingPattern::BindingIdentifier(p) => p.node_id(),
        BindingPattern::ObjectPattern(p) => p.node_id(),
        BindingPattern::ArrayPattern(p) => p.node_id(),
        BindingPattern::AssignmentPattern(p) => p.node_id(),
    }
}

pub(super) fn expression_async_facts<'a>(
    expr: &Expression<'a>,
    semantics: &ComponentSemantics<'a>,
    blocker_data: &BlockerData,
) -> (bool, SmallVec<[u32; 2]>) {
    let mut collector = AsyncFactsCollector {
        refs: Vec::new(),
        has_await: false,
    };
    collector.visit_expression(expr);

    let mut blockers: SmallVec<[u32; 2]> = SmallVec::new();
    for ref_id in &collector.refs {
        let Some(sym) = semantics.get_reference(*ref_id).symbol_id() else {
            continue;
        };
        if let Some(idx) = blocker_data.symbol_blocker(sym)
            && !blockers.contains(&idx)
        {
            blockers.push(idx);
        }
    }
    blockers.sort_unstable();
    (collector.has_await, blockers)
}

struct AsyncFactsCollector {
    refs: Vec<ReferenceId>,
    has_await: bool,
}

impl<'a> Visit<'a> for AsyncFactsCollector {
    fn visit_identifier_reference(&mut self, ident: &IdentifierReference<'a>) {
        if let Some(ref_id) = ident.reference_id.get() {
            self.refs.push(ref_id);
        }
    }
    fn visit_await_expression(&mut self, expr: &AwaitExpression<'a>) {
        self.has_await = true;
        oxc_ast_visit::walk::walk_await_expression(self, expr);
    }
}

pub(super) fn expression_if_facts<'a>(
    expr: &Expression<'a>,
    semantics: &ComponentSemantics<'a>,
    blocker_data: &BlockerData,
) -> (bool, bool, SmallVec<[u32; 2]>) {
    let mut collector = IfFactsCollector {
        refs: Vec::new(),
        has_await: false,
        has_call: false,
    };
    collector.visit_expression(expr);

    let mut blockers: SmallVec<[u32; 2]> = SmallVec::new();
    let mut has_resolved_ref = false;
    for ref_id in &collector.refs {
        let Some(sym) = semantics.get_reference(*ref_id).symbol_id() else {
            continue;
        };
        has_resolved_ref = true;
        if let Some(idx) = blocker_data.symbol_blocker(sym)
            && !blockers.contains(&idx)
        {
            blockers.push(idx);
        }
    }
    blockers.sort_unstable();
    let memoize = collector.has_call && has_resolved_ref;
    (collector.has_await, memoize, blockers)
}

struct IfFactsCollector {
    refs: Vec<ReferenceId>,
    has_await: bool,
    has_call: bool,
}

impl<'a> Visit<'a> for IfFactsCollector {
    fn visit_identifier_reference(&mut self, ident: &IdentifierReference<'a>) {
        if let Some(ref_id) = ident.reference_id.get() {
            self.refs.push(ref_id);
        }
    }
    fn visit_await_expression(&mut self, expr: &AwaitExpression<'a>) {
        self.has_await = true;
        oxc_ast_visit::walk::walk_await_expression(self, expr);
    }
    fn visit_call_expression(&mut self, expr: &CallExpression<'a>) {
        self.has_call = true;
        oxc_ast_visit::walk::walk_call_expression(self, expr);
    }
}
