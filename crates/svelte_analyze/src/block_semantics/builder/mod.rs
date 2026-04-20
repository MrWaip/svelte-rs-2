//! Block Semantics builder. Builds a [`BlockSemanticsStore`] keyed by
//! block `NodeId`, reading raw facts from [`ComponentSemantics`] and the
//! Svelte AST — nothing else.
//!
//! Dependency boundary (see SEMANTIC_LAYER_ARCHITECTURE.md): this builder
//! deliberately does **not** accept `&AnalysisData`. Only AST,
//! `ComponentSemantics`, and — if needed in later slices —
//! `ReactivitySemantics` are permitted inputs. Pipeline code in
//! `passes::executor` assembles the result into `AnalysisData`.
//!
//! Layout:
//! - `walker` — cluster-wide template traversal; one walk drives every
//!   migrated kind's populator.
//! - `each`, `await_` — per-kind populators (free functions invoked
//!   from the walker's node dispatch).
//! - `common` — AST-shape helpers shared by populators.

mod await_;
mod common;
mod const_tag;
mod each;
mod render;
mod snippet;
mod walker;

use super::BlockSemanticsStore;
use crate::reactivity_semantics::data::ReactivitySemantics;
use crate::types::data::{BlockerData, ParserResult};
use svelte_ast::Component;
use svelte_component_semantics::ComponentSemantics;

/// Build the [`BlockSemanticsStore`] for one component.
///
/// Pipeline placement: see `passes::executor` `PassKey::BuildBlockSemantics`.
///
/// Inputs per SEMANTIC_LAYER_ARCHITECTURE.md Dependency Boundary:
/// - `&Component` + `&ParserResult` — AST surface.
/// - `&ComponentSemantics` — scopes, bindings, references. Also the source
///   of hoistable classification for `{#snippet}` blocks: references that
///   resolve to an instance-scope symbol are traced back to the enclosing
///   snippet's body scope.
/// - `&ReactivitySemantics` — reactive per-reference classification
///   (store dependency for `collection.uses_store`).
/// - `&BlockerData` — script-level async analysis used to fold
///   collection-expression async facts into the block's payload.
pub fn build(
    component: &Component,
    parsed: &ParserResult<'_>,
    semantics: &ComponentSemantics<'_>,
    reactivity: &ReactivitySemantics,
    blockers: &BlockerData,
    node_count: u32,
) -> BlockSemanticsStore {
    let mut store = BlockSemanticsStore::new(node_count);
    walker::populate(
        component, parsed, semantics, reactivity, blockers, &mut store,
    );
    store
}
