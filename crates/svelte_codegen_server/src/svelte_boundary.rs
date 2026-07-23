use oxc_allocator::CloneIn;
use oxc_ast::ast::{Expression, Statement};
use svelte_analyze::{BlockSemantics, BoundaryBranch, ElementSemantics, Evaluation};
use svelte_ast::{Attribute, Node, NodeId, SvelteBoundary};
use svelte_ast_builder::{Arg, ObjProp};

use crate::error::{CodegenError, Result};
use crate::fragment::FragmentParent;
use crate::model::ServerCodegen;

impl<'a> ServerCodegen<'a> {
    pub(crate) fn svelte_boundary(&mut self, boundary: &'a SvelteBoundary) -> Result<()> {
        let sem = match self.analysis.element_semantics.query(boundary.id) {
            ElementSemantics::Boundary(sem) => *sem,
            _ => return Err(CodegenError::Unsupported(boundary.id, "boundary")),
        };

        if let (BoundaryBranch::None, BoundaryBranch::Attribute(attr_id)) =
            (sem.failed, sem.pending)
            && self.boundary_pending_nullish(boundary, attr_id)
        {
            return self.emit_boundary_nullish_pending(boundary, attr_id);
        }

        let has_failed = !matches!(sem.failed, BoundaryBranch::None);
        let excluded = boundary_branch_snippets(&sem);
        let inner = match sem.pending {
            BoundaryBranch::None => self.boundary_children_body(boundary, has_failed, &excluded)?,
            branch => self.boundary_pending_body(boundary, branch)?,
        };

        match sem.failed {
            BoundaryBranch::None => {
                for stmt in inner {
                    self.push_stmt(stmt);
                }
                Ok(())
            }
            branch => {
                let prop = self.boundary_failed_prop(boundary, branch)?;
                let body = self
                    .b
                    .arrow_block_expr(self.b.params(["$$renderer"]), inner);
                let props = self.b.object_expr([prop]);
                let call = self
                    .b
                    .call_stmt("$$renderer.boundary", [Arg::Expr(props), Arg::Expr(body)]);
                self.push_stmt(call);
                Ok(())
            }
        }
    }

    fn boundary_pending_nullish(&self, boundary: &SvelteBoundary, attr_id: NodeId) -> bool {
        boundary.attributes.iter().any(|attr| {
            attr.id() == attr_id
                && matches!(attr, Attribute::ExpressionAttribute(a)
                    if self
                        .analysis
                        .expression_data(a.id)
                        .is_some_and(|d| matches!(d.declared_evaluation, Evaluation::MaybeNullish { .. })))
        })
    }

    fn emit_boundary_nullish_pending(
        &mut self,
        boundary: &'a SvelteBoundary,
        attr_id: NodeId,
    ) -> Result<()> {
        let callee = self.take_boundary_attr_expr(boundary, attr_id)?;
        let test = callee.clone_in(self.b.ast.allocator);
        let call = self.b.call_expr_callee(callee, [Arg::Ident("$$renderer")]);
        let pending_body = self.b.block_stmt(vec![
            self.renderer_push_template_stmt("<!--[!-->"),
            self.b.expr_stmt(call),
            self.renderer_push_template_stmt("<!--]-->"),
        ]);
        let content =
            self.child_statements(|cg| cg.fragment(boundary.fragment, FragmentParent::Boundary))?;
        let children_body = self.b.block_stmt(vec![
            self.renderer_push_template_stmt("<!--[-->"),
            self.b.block_stmt(content),
            self.renderer_push_template_stmt("<!--]-->"),
        ]);
        let if_stmt = self.b.if_stmt(test, pending_body, Some(children_body));
        self.push_stmt(if_stmt);
        Ok(())
    }

    fn boundary_children_body(
        &mut self,
        boundary: &'a SvelteBoundary,
        children_only: bool,
        excluded: &[NodeId],
    ) -> Result<Vec<Statement<'a>>> {
        let content = self.child_statements(|cg| {
            if children_only {
                cg.emit_fragment_const_tags(boundary.fragment)?;
                cg.emit_boundary_child_snippets(boundary.fragment, excluded)?;
                cg.fragment_children_only(boundary.fragment, FragmentParent::Boundary)
            } else {
                cg.fragment(boundary.fragment, FragmentParent::Boundary)
            }
        })?;
        Ok(vec![
            self.renderer_push_template_stmt("<!--[-->"),
            self.b.block_stmt(content),
            self.renderer_push_template_stmt("<!--]-->"),
        ])
    }

    fn boundary_pending_body(
        &mut self,
        boundary: &'a SvelteBoundary,
        branch: BoundaryBranch,
    ) -> Result<Vec<Statement<'a>>> {
        let open = self.renderer_push_template_stmt("<!--[!-->");
        let close = self.renderer_push_template_stmt("<!--]-->");
        let middle = match branch {
            BoundaryBranch::Snippet(snippet_id) => {
                let Node::SnippetBlock(block) = self.component.store.get(snippet_id) else {
                    return Err(CodegenError::Unsupported(
                        snippet_id,
                        "boundary pending snippet",
                    ));
                };
                let content =
                    self.child_statements(|cg| cg.fragment(block.body, FragmentParent::Boundary))?;
                self.b.block_stmt(content)
            }
            BoundaryBranch::Attribute(attr_id) => {
                let callee = self.take_boundary_attr_expr(boundary, attr_id)?;
                let call = self.b.call_expr_callee(callee, [Arg::Ident("$$renderer")]);
                self.b.expr_stmt(call)
            }
            BoundaryBranch::None => unreachable!(),
        };
        Ok(vec![open, middle, close])
    }

    fn boundary_failed_prop(
        &mut self,
        boundary: &'a SvelteBoundary,
        branch: BoundaryBranch,
    ) -> Result<ObjProp<'a>> {
        match branch {
            BoundaryBranch::Snippet(snippet_id) => {
                let name: &'a str = match self.analysis.block_semantics(snippet_id) {
                    BlockSemantics::Snippet(sem) => self
                        .b
                        .alloc_str(self.analysis.scoping.symbol_name(sem.name)),
                    _ => {
                        return Err(CodegenError::Unsupported(
                            snippet_id,
                            "boundary failed snippet",
                        ));
                    }
                };
                let mut decls = Vec::new();
                self.emit_snippet_into(snippet_id, &mut decls)?;
                for decl in decls {
                    self.hoist_stmt(decl);
                }
                Ok(ObjProp::Shorthand(name))
            }
            BoundaryBranch::Attribute(attr_id) => {
                let expr = self.take_boundary_attr_expr(boundary, attr_id)?;
                Ok(ObjProp::KeyValue("failed", expr))
            }
            BoundaryBranch::None => unreachable!(),
        }
    }

    fn take_boundary_attr_expr(
        &mut self,
        boundary: &'a SvelteBoundary,
        attr_id: NodeId,
    ) -> Result<Expression<'a>> {
        for attr in &boundary.attributes {
            if attr.id() != attr_id {
                continue;
            }
            if let Attribute::ExpressionAttribute(a) = attr {
                return self.take_expression(a.id, &a.expression);
            }
        }
        Err(CodegenError::MissingExpression(attr_id))
    }

    fn emit_boundary_child_snippets(
        &mut self,
        fragment: svelte_ast::FragmentId,
        excluded: &[NodeId],
    ) -> Result<()> {
        let node_ids: Vec<NodeId> = self.component.store.fragment(fragment).nodes.to_vec();
        for nid in node_ids {
            if excluded.contains(&nid) {
                continue;
            }
            if matches!(self.component.store.get(nid), Node::SnippetBlock(_)) {
                let mut local = Vec::new();
                self.route_snippet(nid, &mut local)?;
                for decl in local {
                    self.hoist_stmt(decl);
                }
            }
        }
        Ok(())
    }
}

fn boundary_branch_snippets(sem: &svelte_analyze::BoundarySemantics) -> Vec<NodeId> {
    let mut out = Vec::new();
    if let BoundaryBranch::Snippet(id) = sem.failed {
        out.push(id);
    }
    if let BoundaryBranch::Snippet(id) = sem.pending {
        out.push(id);
    }
    out
}
