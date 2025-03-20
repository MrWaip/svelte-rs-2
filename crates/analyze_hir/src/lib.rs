mod analises;
mod analyze_expression;
mod analyze_script;
mod analyze_template;
mod bitflags;
mod indentifier_gen;
mod visit;

pub use analises::HirAnalyses;
use analyze_script::AnalyzeScript;
pub use bitflags::{OwnerContentType, OwnerContentTypeFlags};
use hir::NodeId;
use oxc_allocator::Allocator;
use oxc_ast::visit::walk::walk_program;
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

    fn dynamic_markers_pass(&self, analyses: &mut HirAnalyses, store: &hir::HirStore<'hir>) {
        for owner in store.owners.iter().rev() {
            let self_node_id = owner.node_id();

            let mut dynamic = false;

            for node_id in owner.iter_nodes_rev() {
                if analyses.is_dynamic(node_id) {
                    dynamic = true;
                    continue;
                }

                let node = store.get_node(*node_id);

                if node.contains_expression() {
                    analyses.mark_node_as_dynamic(*node_id);
                    dynamic = true;
                    continue;
                }

                if let Some(element) = node.as_element() {
                    for attribute in element.attributes.iter_all() {
                        if attribute.is_dynamic() {
                            analyses.mark_node_as_dynamic(*node_id);
                            dynamic = true;
                            break;
                        }
                    }
                }
            }

            if dynamic {
                analyses.mark_node_as_dynamic(self_node_id);
            }
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

    pub fn script_pass(&self, analyses: &mut HirAnalyses, store: &hir::HirStore<'hir>) {
        let mut script_analyze = AnalyzeScript { analyses };
        let program = store.program.program.borrow_mut();

        walk_program(&mut script_analyze, &*program);
    }

    pub fn analyze(&self, hir_store: &hir::HirStore<'hir>) -> HirAnalyses {
        let (symbols, scopes) = self.oxc_semantic_pass(&hir_store.program);

        let mut analyses = HirAnalyses::new(symbols, scopes);

        self.content_type_pass(&mut analyses, hir_store);
        self.dynamic_markers_pass(&mut analyses, hir_store);
        self.script_pass(&mut analyses, hir_store);

        return analyses;
    }
}

#[cfg(test)]
mod tests {
    use ast_to_hir::AstToHir;
    use parser::Parser;

    use super::*;

    static ALLOCATOR: std::sync::LazyLock<Allocator> =
        std::sync::LazyLock::new(|| Allocator::default());

    #[test]
    fn dynamic_nodes_check() {
        let source = r#"
<div>
    {name}
</div>

<div>
        <span>some_text</span>

        <div>
            {name}
        </div>
</div>

<span></span>
"#;
        let mut parser = Parser::new(source, &ALLOCATOR);
        let analyze_hir = AnalyzeHir::new(&ALLOCATOR);

        let mut lowerer = AstToHir::new(&ALLOCATOR);

        let ast = parser.parse().unwrap();

        let hir = lowerer.traverse(ast);
        let analyses = analyze_hir.analyze(&hir.store);

        let template = hir.store.get_nth_owner(0);
        let first_root_div = hir.store.get_nth_owner(1);
        let second_root_div = hir.store.get_nth_owner(2);
        let first_nested_span = hir.store.get_nth_owner(3);
        let second_nested_div = hir.store.get_nth_owner(4);
        let last_root_span = hir.store.get_nth_owner(5);

        assert!(analyses.is_dynamic(&template.node_id()));
        assert!(analyses.is_dynamic(&first_root_div.node_id()));
        assert!(analyses.is_dynamic(&second_root_div.node_id()));
        assert!(!analyses.is_dynamic(&first_nested_span.node_id()));
        assert!(analyses.is_dynamic(&second_nested_div.node_id()));
        assert!(!analyses.is_dynamic(&last_root_span.node_id()));
    }
}
