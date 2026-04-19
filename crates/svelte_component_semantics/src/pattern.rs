//! Shared walker for OXC [`BindingPattern`].
//!
//! Every Svelte feature that introduces bindings via a destructuring pattern
//! (`$props()`, `$state()` / `$derived()` destructures, `{#each ... as pat}`,
//! `{#await ... then pat}`, `{#snippet name(pat)}`, `{@const pat = ...}`,
//! `let:foo={pat}`) shares the same structural shape: an object / array tree
//! with optional defaults and rest elements, yielding a flat set of
//! `BindingIdentifier` leaves.
//!
//! This module provides the single source of traversal. Consumers call
//! [`walk_bindings`] with a callback and receive each leaf (or rest-target)
//! with the full access path from root and any defaults encountered along
//! the way. All references point back into the OXC AST — **no copies**, no
//! parallel per-feature descriptor types.
//!
//! # Invariants
//! - `BindingIdentifier.symbol_id` is assumed resolved (semantic pass has
//!   run). The walker panics on unresolved ids — this matches our
//!   `ComponentSemantics` lifecycle, where analyze / transform / codegen
//!   always run after semantics.
//! - `path` is empty only for: (a) a plain identifier at the root, or
//!   (b) an object-rest directly on the root object pattern.
//! - `excluded` is non-empty only when `is_rest && path.last()` came from an
//!   `ObjectPattern`. Array-rest has no key exclusion semantics.
//! - No allocation is performed per leaf; a single scratch vector is reused
//!   for the whole walk.

use oxc_ast::ast::{BindingPattern, Expression, PropertyKey};
use smallvec::SmallVec;

use crate::SymbolId;

/// Callback-style traversal of a [`BindingPattern`].
///
/// The callback receives a view into the walker's scratch buffers; it must
/// not outlive the callback invocation. Collect any data into owned storage
/// inside the callback.
pub fn walk_bindings<'a, F>(pat: &'a BindingPattern<'a>, mut visit: F)
where
    F: FnMut(BindingVisit<'a, '_>),
{
    let mut path: SmallVec<[Step<'a>; 4]> = SmallVec::new();
    walk_inner(pat, &mut path, &mut visit);
}

/// Visit entry yielded by [`walk_bindings`] for each leaf or rest target.
pub struct BindingVisit<'a, 'p> {
    /// Resolved symbol of the bound identifier.
    pub symbol: SymbolId,
    /// Access path from the pattern root to this binding.
    /// Empty for a root-level identifier or a root-level object rest.
    pub path: &'p [Step<'a>],
    /// `true` when this binding is a rest element (`...rest`).
    pub is_rest: bool,
    /// Sibling keys to exclude at the rest's parent level.
    /// Non-empty only for object rest; empty for array rest or leaves.
    pub excluded: &'p [&'a PropertyKey<'a>],
}

/// One step in the access path: how to descend from parent to child,
/// plus any `AssignmentPattern` default that applied at this step.
#[derive(Clone, Copy)]
pub struct Step<'a> {
    pub access: Access<'a>,
    /// `AssignmentPattern.right` that wrapped this slot, if any.
    /// Semantically: `parent[access] ?? default` before further descent.
    pub default: Option<&'a Expression<'a>>,
}

#[derive(Clone, Copy)]
pub enum Access<'a> {
    /// Object property key. `computed` reflects `BindingProperty.computed`
    /// — `false` for plain `{ a }` / `{ "a": b }`, `true` for `{ [expr]: b }`.
    Key {
        key: &'a PropertyKey<'a>,
        computed: bool,
    },
    /// Array element position.
    Index(u32),
}

fn walk_inner<'a, F>(pat: &'a BindingPattern<'a>, path: &mut SmallVec<[Step<'a>; 4]>, visit: &mut F)
where
    F: FnMut(BindingVisit<'a, '_>),
{
    match pat {
        BindingPattern::BindingIdentifier(ident) => {
            let symbol = ident
                .symbol_id
                .get()
                .expect("BindingIdentifier.symbol_id resolved before pattern walk");
            visit(BindingVisit {
                symbol,
                path,
                is_rest: false,
                excluded: &[],
            });
        }
        BindingPattern::AssignmentPattern(assign) => {
            // Default applies to the slot through which we reached this
            // pattern. If the caller pushed a Step for that slot, we
            // annotate it; otherwise (root-level `let { x } = y` is not
            // an AssignmentPattern — this case only arises under an
            // object/array property) there is nothing to annotate.
            if let Some(last) = path.last_mut() {
                last.default = Some(&assign.right);
            }
            walk_inner(&assign.left, path, visit);
        }
        BindingPattern::ObjectPattern(obj) => {
            for prop in &obj.properties {
                path.push(Step {
                    access: Access::Key {
                        key: &prop.key,
                        computed: prop.computed,
                    },
                    default: None,
                });
                walk_inner(&prop.value, path, visit);
                path.pop();
            }
            if let Some(rest) = &obj.rest {
                // ESTree restricts object rest argument to an identifier.
                let BindingPattern::BindingIdentifier(ident) = &rest.argument else {
                    return;
                };
                let Some(symbol) = ident.symbol_id.get() else {
                    return;
                };
                let excluded: SmallVec<[&'a PropertyKey<'a>; 4]> =
                    obj.properties.iter().map(|p| &p.key).collect();
                visit(BindingVisit {
                    symbol,
                    path,
                    is_rest: true,
                    excluded: &excluded,
                });
            }
        }
        BindingPattern::ArrayPattern(arr) => {
            for (i, el) in arr.elements.iter().enumerate() {
                let Some(el) = el else { continue };
                path.push(Step {
                    access: Access::Index(i as u32),
                    default: None,
                });
                walk_inner(el, path, visit);
                path.pop();
            }
            if let Some(rest) = &arr.rest {
                // Array rest argument is usually an identifier but the
                // ESTree spec allows nested patterns (rare). Recurse
                // with the current path; the caller sees any leaves as
                // non-rest bindings under this slot. For the common
                // identifier case, we emit a rest visit with empty
                // excluded list.
                if let BindingPattern::BindingIdentifier(ident) = &rest.argument {
                    let Some(symbol) = ident.symbol_id.get() else {
                        return;
                    };
                    visit(BindingVisit {
                        symbol,
                        path,
                        is_rest: true,
                        excluded: &[],
                    });
                } else {
                    walk_inner(&rest.argument, path, visit);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxc_allocator::Allocator;
    use oxc_ast::ast::Statement;
    use oxc_parser::Parser;
    use oxc_span::SourceType;

    /// Parse `let <pat> = 0;`, walk the first binding pattern, and
    /// return a compact textual summary per leaf / rest visit.
    fn summarize(source: &str) -> Vec<String> {
        let alloc = Allocator::default();
        let ret = Parser::new(&alloc, source, SourceType::mjs()).parse();
        assert!(ret.errors.is_empty(), "parse errors: {:?}", ret.errors);
        let stmt = ret.program.body.first().expect("one statement");
        let Statement::VariableDeclaration(decl) = stmt else {
            panic!("expected var declaration");
        };
        let declarator = decl.declarations.first().expect("one declarator");
        summarize_pat(&declarator.id)
    }

    /// Collect a compact textual summary of each visit for snapshot-style
    /// assertions. Symbol ids are replaced by ordinal numbers so the
    /// assertions don't depend on id assignment order.
    fn summarize_pat(pat: &BindingPattern<'_>) -> Vec<String> {
        // Assign a fresh symbol id to each BindingIdentifier so the
        // walker has something to yield. Tests don't run the semantic
        // pass; we seed ids manually with a counter.
        seed_symbol_ids(pat);

        let mut out: Vec<String> = Vec::new();
        walk_bindings(pat, |v| {
            let path = v
                .path
                .iter()
                .map(|s| {
                    let mut label = match s.access {
                        Access::Key {
                            key,
                            computed: false,
                        } => match key {
                            PropertyKey::StaticIdentifier(id) => format!(".{}", id.name),
                            PropertyKey::StringLiteral(s) => format!(".{:?}", s.value),
                            _ => ".?".into(),
                        },
                        Access::Key { computed: true, .. } => "[expr]".into(),
                        Access::Index(i) => format!("[{}]", i),
                    };
                    if s.default.is_some() {
                        label.push_str("={d}");
                    }
                    label
                })
                .collect::<String>();
            let tag = if v.is_rest {
                let excluded = v
                    .excluded
                    .iter()
                    .map(|k| match k {
                        PropertyKey::StaticIdentifier(id) => id.name.to_string(),
                        _ => "?".into(),
                    })
                    .collect::<Vec<_>>()
                    .join(",");
                format!("rest({}) excl=[{}]", sym_ordinal(v.symbol), excluded)
            } else {
                format!("leaf({})", sym_ordinal(v.symbol))
            };
            out.push(format!("{path} {tag}"));
        });
        out
    }

    fn seed_symbol_ids(pat: &BindingPattern<'_>) {
        use oxc_syntax::symbol::SymbolId;
        use std::cell::Cell;
        thread_local! {
            static COUNTER: Cell<u32> = const { Cell::new(0) };
        }
        fn next_id() -> SymbolId {
            COUNTER.with(|c| {
                let v = c.get();
                c.set(v + 1);
                SymbolId::from_usize(v as usize)
            })
        }
        fn visit(pat: &BindingPattern<'_>) {
            match pat {
                BindingPattern::BindingIdentifier(id) => {
                    id.symbol_id.set(Some(next_id()));
                }
                BindingPattern::AssignmentPattern(a) => visit(&a.left),
                BindingPattern::ObjectPattern(o) => {
                    for p in &o.properties {
                        visit(&p.value);
                    }
                    if let Some(r) = &o.rest {
                        visit(&r.argument);
                    }
                }
                BindingPattern::ArrayPattern(a) => {
                    for e in a.elements.iter().flatten() {
                        visit(e);
                    }
                    if let Some(r) = &a.rest {
                        visit(&r.argument);
                    }
                }
            }
        }
        COUNTER.with(|c| c.set(0));
        visit(pat);
    }

    fn sym_ordinal(sym: SymbolId) -> u32 {
        sym.index() as u32
    }

    #[test]
    fn plain_identifier_root() {
        assert_eq!(summarize("let x = 0;"), vec![" leaf(0)"]);
    }

    #[test]
    fn flat_object_destructure() {
        assert_eq!(
            summarize("let { a, b } = 0;"),
            vec![".a leaf(0)", ".b leaf(1)"]
        );
    }

    #[test]
    fn aliased_property() {
        // Origin key is "a"; local binding (symbol) is for `b`.
        assert_eq!(summarize("let { a: b } = 0;"), vec![".a leaf(0)"]);
    }

    #[test]
    fn leaf_default() {
        assert_eq!(summarize("let { a = 5 } = 0;"), vec![".a={d} leaf(0)"]);
    }

    #[test]
    fn intermediate_default() {
        // `a` slot has default `{}`; leaf `b` has no leaf-level default.
        assert_eq!(
            summarize("let { a: { b } = {} } = 0;"),
            vec![".a={d}.b leaf(0)"]
        );
    }

    #[test]
    fn both_intermediate_and_leaf_default() {
        assert_eq!(
            summarize("let { a: { b = 3 } = {} } = 0;"),
            vec![".a={d}.b={d} leaf(0)"]
        );
    }

    #[test]
    fn nested_object() {
        assert_eq!(
            summarize("let { a: { b: { c } } } = 0;"),
            vec![".a.b.c leaf(0)"]
        );
    }

    #[test]
    fn flat_array_destructure() {
        assert_eq!(
            summarize("let [a, b] = 0;"),
            vec!["[0] leaf(0)", "[1] leaf(1)"]
        );
    }

    #[test]
    fn array_with_hole() {
        assert_eq!(
            summarize("let [a, , c] = 0;"),
            vec!["[0] leaf(0)", "[2] leaf(1)"]
        );
    }

    #[test]
    fn mixed_object_array() {
        assert_eq!(
            summarize("let { users: [{ name }, second] } = 0;"),
            vec![".users[0].name leaf(0)", ".users[1] leaf(1)"]
        );
    }

    #[test]
    fn object_rest() {
        assert_eq!(
            summarize("let { a, b, ...rest } = 0;"),
            vec![".a leaf(0)", ".b leaf(1)", " rest(2) excl=[a,b]"]
        );
    }

    #[test]
    fn nested_object_rest() {
        assert_eq!(
            summarize("let { a: { b, ...inner }, ...outer } = 0;"),
            vec![".a.b leaf(0)", ".a rest(1) excl=[b]", " rest(2) excl=[a]"]
        );
    }

    #[test]
    fn array_rest() {
        // Array rest takes "everything not consumed" — path is the parent
        // (empty here), not a specific index.
        assert_eq!(
            summarize("let [a, ...rest] = 0;"),
            vec!["[0] leaf(0)", " rest(1) excl=[]"]
        );
    }

    #[test]
    fn computed_key() {
        assert_eq!(summarize("let { [k]: value } = 0;"), vec!["[expr] leaf(0)"]);
    }
}
