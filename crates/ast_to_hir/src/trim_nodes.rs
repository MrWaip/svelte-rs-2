use crate::{AstToHir, context::ToHirContext};

impl<'hir> AstToHir<'hir> {
    pub(crate) fn trim_text_nodes(
        &self,
        nodes: Vec<ast::Node<'hir>>,
        ctx: &mut ToHirContext<'hir>,
    ) -> Vec<ast::Node<'hir>> {
        if nodes.is_empty() {
            return nodes;
        }

        let mut start: usize = 0;
        let mut end = nodes.len();
        let mut new_nodes: Vec<ast::Node<'hir>> = Vec::new();

        // trim left
        for node in nodes.iter() {
            if let ast::Node::Text(cell) = node {
                let mut text = cell.borrow_mut();

                if text.is_removable() {
                    start += 1;
                    continue;
                } else {
                    text.trim_start();
                    break;
                }
            } else {
                break;
            }
        }

        // trim right
        for node in nodes.iter().rev() {
            if let ast::Node::Text(cell) = node {
                let mut text = cell.borrow_mut();

                if text.is_removable() {
                    end -= 1;
                    continue;
                } else {
                    text.trim_end();
                    break;
                }
            } else {
                break;
            }
        }

        if start > end {
            return Vec::new();
        }

        for (idx, current) in nodes[start..end].iter().enumerate() {
            let prev = if idx == 0 { None } else { nodes.get(idx - 1) };
            let next = nodes.get(idx + 1);

            if let ast::Node::Text(cell) = &current {
                let mut text = cell.borrow_mut();

                if !prev.is_some_and(|node| node.is_interpolation()) {
                    text.trim_start_one_whitespace(ctx.allocator);
                }

                if !next.is_some_and(|node| node.is_interpolation()) {
                    text.trim_end_one_whitespace(ctx.allocator);
                }
            }

            new_nodes.push(current.clone());
        }

        new_nodes
    }
}

#[cfg(test)]
mod trim_nodes {
    use hir::NodeId;
    use oxc_allocator::Allocator;
    use oxc_index::IndexVec;
    use parser::Parser;

    static ALLOCATOR: std::sync::LazyLock<Allocator> =
        std::sync::LazyLock::new(Allocator::default);

    use super::*;

    fn prepare(text: &str) -> (&hir::Template, IndexVec<NodeId, hir::Node>) {
        let mut lowerer = AstToHir::new(&ALLOCATOR);
        let ast = Parser::new(text, &ALLOCATOR).parse().unwrap();

        let hir = lowerer.traverse(ast);

        let hir::OwnerNode::Template(template) = hir.store.owners.first().unwrap() else {
            unreachable!()
        };

        (template, hir.store.nodes)
    }

    #[test]
    fn trim_single_node() {
        let (template, _) = prepare(" \n\t\r");

        assert!(template.node_ids.is_empty())
    }

    #[test]
    fn trim_edges_of_single_node() {
        let (template, nodes) = prepare("\ttext\t");
        assert_eq!(template.node_ids.len(), 1);

        let hir::Node::Text(text) = &nodes[NodeId::new(1)] else {
            unreachable!()
        };

        assert_eq!(text.value, "text");
    }

    #[test]
    fn trim_around_element() {
        let (template, nodes) = prepare("\t<input />\t");

        assert_eq!(template.node_ids.len(), 1);
        assert!(nodes.get(NodeId::new(1)).unwrap().is_element())
    }

    #[test]
    fn trim_between_right() {
        let (template, nodes) = prepare("some_text      <input />\t");

        assert_eq!(template.node_ids.len(), 2);

        let text = nodes.get(NodeId::new(1)).unwrap().as_text().unwrap();
        assert_eq!(text.value, "some_text ");
    }

    #[test]
    fn trim_after_comment_remove() {
        let (template, _nodes) = prepare("<div>    <!-- comment -->   </div>");

        assert_eq!(template.node_ids.len(), 1);
    }

    #[test]
    fn trim_between_left() {
        let (template, nodes) = prepare("<input />     some_text");

        assert_eq!(template.node_ids.len(), 2);

        let text = nodes.last().unwrap().as_text().unwrap();
        assert_eq!(text.value, " some_text");
    }
}
