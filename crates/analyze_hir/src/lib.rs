mod analises;
mod analyze_expression;
mod analyze_template;
mod visit;

pub use analises::HirAnalyses;
use oxc_allocator::Allocator;
use oxc_semantic::{ScopeTree, SymbolTable};

pub struct AnalyzeHir<'hir> {
    allocator: &'hir Allocator,
}

pub struct AnalyzeHirResult {}

impl<'hir> AnalyzeHir<'hir> {
    pub fn new(allocator: &'hir Allocator) -> Self {
        AnalyzeHir { allocator }
    }

    fn tmp(&self, program: Option<&hir::Program<'hir>>) {
        // let empty = self.b.program(vec![]);
        // let program = ast
        //     .script
        //     .as_ref()
        //     .map(|script| &script.program)
        //     .unwrap_or_else(|| &empty);

        // let ret = SemanticBuilder::new().build(&program);

        // if !ret.errors.is_empty() {
        //     todo!();
        // }

        // let (symbols, scopes) = ret.semantic.into_symbol_table_and_scope_tree();
    }

    pub fn analyze(&self, hir_store: &hir::HirStore) -> HirAnalyses {
        HirAnalyses::new(ScopeTree::default(), SymbolTable::default())
    }
}
