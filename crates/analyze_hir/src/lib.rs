mod analises;
mod analyze_expression;
mod analyze_template;
mod bitflags;
mod visit;

pub use analises::HirAnalyses;
use bitflags::{OwnerContentType, OwnerContentTypeFlags};
use hir::NodeId;
use oxc_allocator::Allocator;
use oxc_semantic::{ScopeTree, SemanticBuilder, SymbolTable};

pub struct AnalyzeHir<'hir> {
    allocator: &'hir Allocator,
}

pub struct AnalyzeHirResult {}

impl<'hir> AnalyzeHir<'hir> {
    pub fn new(allocator: &'hir Allocator) -> Self {
        AnalyzeHir { allocator }
    }

    fn oxc_semantic_pass(&self, program: &hir::Program<'hir>) -> (SymbolTable, ScopeTree) {
        let result = SemanticBuilder::new().build(&mut *program.program.borrow_mut());

        if !result.errors.is_empty() {
            todo!();
        }

        let (symbols, scopes) = result.semantic.into_symbol_table_and_scope_tree();

        return (symbols, scopes);
    }

    fn content_type_pass(&self, analyses: &mut HirAnalyses, store: &hir::HirStore<'hir>) {
        for (owner_id, owner) in store.owners.iter_enumerated() {
            let content_type: OwnerContentType = match owner {
                hir::OwnerNode::Element(element) => {
                    let flags = self.compute_content_type(&element.node_ids, store);
                    OwnerContentType::Common(flags)
                }
                hir::OwnerNode::Template(template) => {
                    let flags = self.compute_content_type(&template.node_ids, store);
                    OwnerContentType::Common(flags)
                }
                hir::OwnerNode::IfBlock(if_block) => {
                    let consequent_flags = self.compute_content_type(&if_block.consequent, store);
                    let alternate_flags = if_block
                        .alternate
                        .as_ref()
                        .map(|alternate| self.compute_content_type(&alternate, store))
                        .unwrap_or(OwnerContentTypeFlags::empty());

                    OwnerContentType::IfBlock(consequent_flags, alternate_flags)
                }
                hir::OwnerNode::EachBlock => todo!(),
                hir::OwnerNode::Phantom => todo!(),
            };

            analyses.set_content_type(owner_id, content_type);
        }
    }

    fn compute_content_type(
        &self,
        nodes: &Vec<NodeId>,
        store: &hir::HirStore<'hir>,
    ) -> OwnerContentTypeFlags {
        let mut flags = OwnerContentTypeFlags::empty();

        for node_id in nodes.iter() {
            let node = store.get_node(*node_id);
            flags.set_from(node);
        }

        return flags;
    }

    pub fn analyze(&self, hir_store: &hir::HirStore<'hir>) -> HirAnalyses {
        let (symbols, scopes) = self.oxc_semantic_pass(&hir_store.program);

        let mut analyses = HirAnalyses::new(symbols, scopes);

        self.content_type_pass(&mut analyses, hir_store);

        return analyses;
    }
}
