//! Shared helpers for per-kind Block Semantics builders.
//!
//! Each kind (`each`, `await`, ...) lives in its own submodule and runs its
//! own template walk. The helpers here are the AST-shape utilities that
//! multiple kinds need: extracting the first declarator of a
//! `VariableDeclaration`, resolving a `BindingPattern`'s node id, and
//! collecting the symbol ids of every leaf identifier inside a pattern.

use oxc_ast::ast::{BindingPattern, Statement, VariableDeclarator};
use smallvec::SmallVec;
use svelte_component_semantics::{OxcNodeId, SymbolId};

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

/// Recursively collect `SymbolId` for every leaf `BindingIdentifier`
/// inside a `BindingPattern`. Used to enumerate the symbols introduced
/// by a destructured template binding — each-block context, then /
/// catch value pattern, etc.
pub(super) fn collect_binding_pattern_symbols(
    pattern: &BindingPattern<'_>,
    out: &mut SmallVec<[SymbolId; 4]>,
) {
    use oxc_ast::ast::BindingPattern as BP;
    match pattern {
        BP::BindingIdentifier(ident) => {
            if let Some(sym) = ident.symbol_id.get() {
                out.push(sym);
            }
        }
        BP::ObjectPattern(obj) => {
            for prop in &obj.properties {
                collect_binding_pattern_symbols(&prop.value, out);
            }
            if let Some(rest) = &obj.rest {
                collect_binding_pattern_symbols(&rest.argument, out);
            }
        }
        BP::ArrayPattern(arr) => {
            for el in arr.elements.iter().flatten() {
                collect_binding_pattern_symbols(el, out);
            }
            if let Some(rest) = &arr.rest {
                collect_binding_pattern_symbols(&rest.argument, out);
            }
        }
        BP::AssignmentPattern(assign) => {
            collect_binding_pattern_symbols(&assign.left, out);
        }
    }
}
