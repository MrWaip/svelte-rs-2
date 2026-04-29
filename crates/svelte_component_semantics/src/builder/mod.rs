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

pub struct ComponentSemanticsBuilder<'a> {
    pub(crate) semantics: ComponentSemantics<'a>,

    next_node_id: u32,
}

impl<'a> ComponentSemanticsBuilder<'a> {
    pub fn new() -> Self {
        Self {
            semantics: ComponentSemantics::new(),
            next_node_id: 0,
        }
    }

    pub fn add_instance_program(&mut self, program: &Program<'a>) {
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

    pub fn add_module_program(&mut self, program: &Program<'a>) {
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

    pub fn add_template(&mut self, walker: &mut impl TemplateWalker<'a>) {
        let root = self.semantics.root_scope_id();
        let mut ctx = TemplateBuildContext::new(&mut self.semantics, root, self.next_node_id);
        walker.walk_template(&mut ctx);
        self.next_node_id = ctx.max_node_id() + 1;
    }

    pub fn next_node_id(&self) -> u32 {
        self.next_node_id
    }

    pub fn finish(self) -> ComponentSemantics<'a> {
        self.semantics
    }
}

impl<'a> Default for ComponentSemanticsBuilder<'a> {
    fn default() -> Self {
        Self::new()
    }
}
