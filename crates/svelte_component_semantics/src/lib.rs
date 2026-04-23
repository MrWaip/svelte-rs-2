//! Component-wide semantic graph for `.svelte` compilation.
//!
//! Replaces `oxc_semantic::Scoping` and `oxc_semantic::SemanticBuilder` with a
//! storage and builder designed for Svelte's multi-source component model
//! (module script + instance script + template).
//!
//! # Architecture
//!
//! - **[`ComponentSemantics`]** — the source of truth for scopes, symbols,
//!   references, and fragment-scope mappings across the entire component.
//! - **[`ComponentSemanticsBuilder`]** — constructs `ComponentSemantics` by
//!   traversing JS programs (via OXC `Visit`) and template (via [`TemplateWalker`] trait).
//! - **[`JsSemanticVisitor`]** — OXC `Visit` implementation that registers
//!   scopes, bindings, references, and handles flag propagation. Forked from
//!   OXC `SemanticBuilder` v0.117.0.
//! - **[`TemplateBuildContext`]** — passed to the `TemplateWalker` impl;
//!   provides scope creation, JS expression analysis, and shorthand reference
//!   materialization without knowing Svelte AST types.
//!
//! # ID types
//!
//! All IDs are re-exported from `oxc_syntax` — they are the **same types**
//! stored in OXC AST node cells (`BindingIdentifier.symbol_id`,
//! `IdentifierReference.reference_id`, etc.). No mapping needed.
//!
//! - [`ScopeId`], [`SymbolId`], [`ReferenceId`] — semantic identifiers
//! - [`OxcNodeId`] — JS AST node identity, unified across the component
//!   (instance ids as-is, module ids offset, template ids continue the counter)
//! - [`FragmentId`] — identifies a template fragment that owns a scope
//! - [`SymbolOwner`] — which source region declared a symbol
//!   (`ModuleScript`, `InstanceScript`, `Template`, `Synthetic`)
//!
//! # Usage
//!
//! ```ignore
//! use svelte_component_semantics::*;
//!
//! let mut builder = ComponentSemanticsBuilder::new();
//!
//! // 1. Module first (its bindings must exist before instance resolution)
//! if let Some(module_program) = &module_program {
//!     builder.add_module_program(module_program);
//! }
//!
//! // 2. Instance second
//! if let Some(instance_program) = &instance_program {
//!     builder.add_instance_program(instance_program);
//! }
//!
//! // 3. Template third (via a TemplateWalker impl from svelte_analyze)
//! builder.add_template(&my_template_walker);
//!
//! let sem: ComponentSemantics = builder.finish();
//!
//! // Query API
//! let sym = sem.find_binding(sem.root_scope_id(), "count").unwrap();
//! assert_eq!(sem.symbol_name(sym), "count");
//! assert_eq!(sem.symbol_owner(sym), SymbolOwner::InstanceScript);
//! assert!(sem.is_mutated(sym));        // eager — O(1)
//! assert!(!sem.is_expr_local(sym));     // not inside a template JS expression
//! ```
//!
//! # Template integration
//!
//! This crate does **not** depend on Svelte AST for template traversal.
//! Instead, `svelte_analyze` implements [`TemplateWalker`] and calls back into
//! [`TemplateBuildContext`] to create scopes and analyze JS expressions:
//!
//! ```ignore
//! impl TemplateWalker for MyWalker {
//!     fn walk_template(&self, ctx: &mut TemplateBuildContext<'_>) {
//!         // Create scope for {#each} block
//!         let scope = ctx.enter_fragment_scope_by_id(fragment_id);
//!         // Analyze JS expression inside template
//!         ctx.visit_js_expression(&expr);
//!         // Shorthand bind:value → Write reference
//!         ctx.materialize_shorthand_reference("value", ReferenceFlags::Write);
//!         ctx.leave_scope();
//!     }
//! }
//! ```

pub mod builder;
pub mod pattern;
mod reference;
mod scope;
mod storage;
mod symbol;

pub use builder::{
    ComponentSemanticsBuilder, JsSemanticVisitor, TemplateBuildContext, TemplateWalker,
};
pub use pattern::{walk_bindings, Access, BindingVisit, Step};
pub use reference::Reference;
pub use storage::{ComponentSemantics, JsNode, JsStorage};
pub use svelte_ast::FragmentId;
pub use symbol::state as sym_state;
pub use symbol::SymbolOwner;

// Re-export OXC types used in our public API.
// These are the same types stored in OXC AST node cells, so our IDs are
// directly compatible with BindingIdentifier.symbol_id, IdentifierReference.reference_id, etc.
pub use oxc_syntax::node::NodeId as OxcNodeId;
pub use oxc_syntax::reference::{ReferenceFlags, ReferenceId};
pub use oxc_syntax::scope::{ScopeFlags, ScopeId};
pub use oxc_syntax::symbol::{SymbolFlags, SymbolId};
