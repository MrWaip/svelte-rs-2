use crate::{AstToHir, context::ToHirContext};

impl AstToHir {
    pub(crate) fn trim_text_nodes<'hir>(
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

        for (idx, current) in nodes[start..end].iter().enumerate() {
            let prev = if idx == 0 { None } else { nodes.get(idx - 1) };
            let next = nodes.get(idx + 1);

            if let ast::Node::Text(cell) = &current {
                let mut text = cell.borrow_mut();

                if !prev.is_some_and(|node| node.is_interpolation()) {
                    text.trim_start_one_whitespace(&ctx.allocator);
                }

                if !next.is_some_and(|node| node.is_interpolation()) {
                    text.trim_end_one_whitespace(&ctx.allocator);
                }
            }

            new_nodes.push(current.clone());
        }

        return new_nodes;
    }
}
