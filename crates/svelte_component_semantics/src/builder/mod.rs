mod js_visitor;
mod template;
#[cfg(test)]
mod tests;

pub use js_visitor::JsSemanticVisitor;
pub use template::{TemplateBuildContext, TemplateWalker};

use oxc_ast::ast::Program;
use oxc_ast_visit::Visit;
use oxc_syntax::scope::ScopeFlags;

use crate::storage::ComponentSemantics;
use crate::symbol::SymbolOwner;

/// Builds a `ComponentSemantics` graph by traversing JS programs.
///
/// Call order matters: module bindings must exist before instance resolution.
///
/// ```ignore
/// let mut builder = ComponentSemanticsBuilder::new();
/// builder.add_module_program(&module_program);  // first
/// builder.add_instance_program(&instance_program); // second
/// builder.add_template(&my_walker);  // third
/// let semantics = builder.finish();
/// ```
pub struct ComponentSemanticsBuilder {
    pub(crate) semantics: ComponentSemantics,
    /// Tracks the next available OxcNodeId across all programs.
    /// After each program traversal, bumped to max_seen + 1.
    next_node_id: u32,
}

impl ComponentSemanticsBuilder {
    pub fn new() -> Self {
        Self {
            semantics: ComponentSemantics::new(),
            next_node_id: 0,
        }
    }

    /// Process instance script (`<script>`). Traverses the program AST and
    /// registers all scopes, bindings, and references.
    pub fn add_instance_program(&mut self, program: &Program<'_>) {
        let root = self.semantics.root_scope_id();
        program.scope_id.set(Some(root));
        let mut visitor = js_visitor::JsSemanticVisitor::new_with_offset(
            &mut self.semantics,
            root,
            SymbolOwner::InstanceScript,
            self.next_node_id,
        );
        visitor.visit_program(program);
        self.next_node_id = visitor.max_node_id() + 1;
        self.semantics.set_instance_scope_id(root);
    }

    /// Process module script (`<script module>`). Creates a module scope above
    /// the instance root, traverses the program, and registers semantics.
    /// NodeIds are offset to avoid collision with instance script.
    pub fn add_module_program(&mut self, program: &Program<'_>) {
        let root = self.semantics.root_scope_id();

        let module_scope = self
            .semantics
            .add_scope(root, ScopeFlags::Top | ScopeFlags::Function);
        self.semantics.set_scope_parent_id(root, Some(module_scope));
        self.semantics.set_scope_parent_id(module_scope, None);
        self.semantics.set_module_scope_id(module_scope);
        program.scope_id.set(Some(module_scope));

        let mut visitor = js_visitor::JsSemanticVisitor::new_with_offset(
            &mut self.semantics,
            module_scope,
            SymbolOwner::ModuleScript,
            self.next_node_id,
        );
        visitor.visit_program(program);
        self.next_node_id = visitor.max_node_id() + 1;
    }

    /// Process template. The walker knows Svelte AST types and calls back
    /// into `TemplateBuildContext` to create scopes and analyze JS expressions.
    /// Template NodeIds are offset past script NodeIds.
    pub fn add_template(&mut self, walker: &mut impl TemplateWalker) {
        let root = self.semantics.root_scope_id();
        let mut ctx = TemplateBuildContext::new(&mut self.semantics, root, self.next_node_id);
        walker.walk_template(&mut ctx);
        self.next_node_id = ctx.max_node_id() + 1;
    }

    /// The next available OxcNodeId. Template code can use this to allocate
    /// node IDs from a shared counter.
    pub fn next_node_id(&self) -> u32 {
        self.next_node_id
    }

    /// Consume the builder and return the built semantics.
    pub fn finish(self) -> ComponentSemantics {
        self.semantics
    }
}

impl Default for ComponentSemanticsBuilder {
    fn default() -> Self {
        Self::new()
    }
}
