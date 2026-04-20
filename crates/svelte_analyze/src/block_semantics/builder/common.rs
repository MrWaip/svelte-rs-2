//! Shared helpers for per-kind Block Semantics builders.
//!
//! Each kind (`each`, `await`, ...) lives in its own submodule and runs its
//! own template walk. The helpers here are the AST-shape utilities that
//! multiple kinds need: extracting the first declarator of a
//! `VariableDeclaration`, resolving a `BindingPattern`'s node id, and
//! collecting the symbol ids of every leaf identifier inside a pattern.

use oxc_ast::ast::{
    AwaitExpression, BindingPattern, CallExpression, Expression, IdentifierReference, Statement,
    VariableDeclarator,
};
use oxc_ast_visit::Visit;
use smallvec::SmallVec;
use svelte_component_semantics::{ComponentSemantics, OxcNodeId, ReferenceId};

use crate::types::data::BlockerData;

/// First declarator of a `VariableDeclaration` statement. Template
/// introducer spans (`{#each ... as <decl>}`, `{:then <decl>}`,
/// `{:catch <decl>}`) are all pre-parsed as single-declarator
/// `let <pattern>`-ish statements, so the first declarator is the
/// binding pattern we care about.
pub(super) fn declarator_from_stmt<'a>(
    stmt: &'a Statement<'a>,
) -> Option<&'a VariableDeclarator<'a>> {
    let Statement::VariableDeclaration(decl) = stmt else {
        return None;
    };
    decl.declarations.first()
}

/// `BindingPattern` → its leaf `BindingIdentifier` if it is a plain
/// identifier (not a destructured / assignment pattern).
pub(super) fn binding_ident_of<'a>(
    pattern: &'a BindingPattern<'a>,
) -> Option<&'a oxc_ast::ast::BindingIdentifier<'a>> {
    match pattern {
        BindingPattern::BindingIdentifier(ident) => Some(ident),
        _ => None,
    }
}

/// Node id of a `BindingPattern` regardless of its variant. Used to
/// carry an AST-identity handle for destructured patterns in the
/// cluster's payload so the consumer can look the subtree up later.
pub(super) fn binding_pattern_node_id(pattern: &BindingPattern<'_>) -> OxcNodeId {
    match pattern {
        BindingPattern::BindingIdentifier(p) => p.node_id(),
        BindingPattern::ObjectPattern(p) => p.node_id(),
        BindingPattern::ArrayPattern(p) => p.node_id(),
        BindingPattern::AssignmentPattern(p) => p.node_id(),
    }
}

/// Single-walk classification of an expression for the two async facts
/// block_semantics payloads need: does the expression contain an `await`,
/// and which script-level `BlockerData` indices does it transitively
/// depend on. Shared by `{#await}` (await_.rs) and `{@const}`
/// (const_tag.rs) — `{#each}` needs additional per-ref facts (store /
/// external scope) and keeps its own superset collector.
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
        if let Some(idx) = blocker_data.symbol_blocker(sym) {
            if !blockers.contains(&idx) {
                blockers.push(idx);
            }
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

/// Single-walk classification for `{#if}` branch conditions. Returns
/// the three facts the IfBlock builder branches on:
///
/// * `has_await` — literal `await` appears anywhere in the expression.
///   Drives the root-branch `AsyncParam` lowering.
/// * `memoize` — the expression contains a `CallExpression` **and**
///   at least one identifier that resolves to a real symbol. Drives
///   `$.derived` memoization for non-async branches. A call without
///   any resolved ref (e.g. `$effect.pending()` — `$effect` is a rune
///   marker, not a binding) doesn't need memoization because nothing
///   reactive can invalidate the result; mirrors the legacy
///   `needs_memo = has_call && !ref_symbols.is_empty()` gate.
/// * `blockers` — sorted, de-duplicated script-level `BlockerData`
///   indices for every resolved identifier reference.
///
/// Separate from [`expression_async_facts`] only to avoid widening
/// that helper's return type for the other kinds that discard
/// `has_call`.
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
        if let Some(idx) = blocker_data.symbol_blocker(sym) {
            if !blockers.contains(&idx) {
                blockers.push(idx);
            }
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
