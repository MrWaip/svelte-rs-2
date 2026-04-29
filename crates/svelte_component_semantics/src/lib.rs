pub mod builder;
pub mod pattern;
mod reference;
mod scope;
mod storage;
mod symbol;

pub use builder::{
    ComponentSemanticsBuilder, JsSemanticVisitor, TemplateBuildContext, TemplateWalker,
};
pub use pattern::{Access, BindingVisit, Step, walk_bindings};
pub use reference::Reference;
pub use storage::{ComponentSemantics, JsNode, JsStorage};
pub use svelte_ast::FragmentId;
pub use symbol::SymbolOwner;
pub use symbol::state as sym_state;

pub use oxc_syntax::node::NodeId as OxcNodeId;
pub use oxc_syntax::reference::{ReferenceFlags, ReferenceId};
pub use oxc_syntax::scope::{ScopeFlags, ScopeId};
pub use oxc_syntax::symbol::{SymbolFlags, SymbolId};
