pub mod builder;
mod class_table;
pub mod pattern;
mod reference;
mod scope;
mod storage;
mod symbol;

pub use builder::{
    ComponentSemanticsBuilder, JsSemanticVisitor, TemplateBuildContext, TemplateWalker,
};
pub use class_table::{ClassFieldDecl, ClassTable};
pub use pattern::{
    Access, AssignmentTargetVisit, BindingVisit, Step, WriteAccess, WriteStep, WriteTarget,
    walk_assignment_target_idents, walk_assignment_targets, walk_bindings,
};
pub use reference::Reference;
pub use storage::{ComponentSemantics, JsNode, JsStorage, OriginKind};
pub use svelte_ast::FragmentId;
pub use symbol::SymbolOwner;
pub use symbol::state as sym_state;

pub use oxc_syntax::node::NodeId as OxcNodeId;
pub use oxc_syntax::reference::{ReferenceFlags, ReferenceId};
pub use oxc_syntax::scope::{ScopeFlags, ScopeId};
pub use oxc_syntax::symbol::{SymbolFlags, SymbolId};
