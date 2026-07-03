use oxc_allocator::CloneIn;
use oxc_ast::ast::Statement;
use oxc_syntax::node::NodeId as OxcNodeId;
use svelte_analyze::{AttributeSemantics, BoundaryPropSemantics, Volatility};
use svelte_ast::{Attribute, Node, NodeId};
use svelte_ast_builder::{Arg, ObjProp};

use super::super::data_structures::EmitState;
use super::super::data_structures::{FragmentAnchor, FragmentCtx};
use super::super::{Codegen, CodegenError, Result};

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(in crate::codegen) fn emit_svelte_boundary(
        &mut self,
        state: &mut EmitState<'a>,
        ctx: &FragmentCtx<'a>,
        el_id: NodeId,
        _existing_var: Option<&str>,
    ) -> Result<String> {
        let boundary = self.ctx.query.svelte_boundary(el_id);

        let snippet_children: Vec<(NodeId, String)> = self
            .ctx
            .query
            .component
            .store
            .fragment(boundary.fragment)
            .nodes
            .iter()
            .filter_map(|&nid| {
                let Node::SnippetBlock(block) = self.ctx.query.component.store.get(nid) else {
                    return None;
                };
                let sem = match self.ctx.query.analysis.block_semantics(block.id) {
                    svelte_analyze::BlockSemantics::Snippet(s) => s.clone(),
                    _ => return None,
                };
                let name = self.ctx.query.view.symbol_name(sem.name).to_string();
                Some((block.id, name))
            })
            .collect();

        let attr_infos: Vec<(String, NodeId, OxcNodeId, Volatility)> = boundary
            .attributes
            .iter()
            .map(|attr| {
                let attr_id = attr.id();
                match self.ctx.query.analysis.attributes.get(attr_id) {
                    AttributeSemantics::BoundaryProp(BoundaryPropSemantics { volatility }) => {
                        match attr {
                            Attribute::ExpressionAttribute(a) => Ok(Some((
                                a.name.to_string(),
                                a.id,
                                a.expression.id(),
                                *volatility,
                            ))),
                            _ => CodegenError::semantic_mismatch(
                                attr_id,
                                "BoundaryProp payload requires ExpressionAttribute",
                            ),
                        }
                    }
                    AttributeSemantics::NonSpecial => Ok(None),
                    _ => CodegenError::semantic_mismatch(
                        attr_id,
                        "non-boundary semantics on <svelte:boundary>",
                    ),
                }
            })
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .flatten()
            .collect();

        let anchor_node = self.comment_anchor_node_name(state, ctx)?;

        let mut props: Vec<ObjProp<'a>> = Vec::new();
        for (name, attr_id, expr_id, volatility) in attr_infos {
            let key = self.ctx.b.alloc_str(&name);
            let Some(expr) = self.ctx.state.parsed.take_expr(expr_id) else {
                return CodegenError::missing_expression(attr_id);
            };
            match volatility {
                Volatility::Static => props.push(ObjProp::KeyValue(key, expr)),
                Volatility::Reactive | Volatility::Heavy | Volatility::Asynchronous => {
                    props.push(ObjProp::Getter(key, expr))
                }
            }
        }
        for (_, snippet_name) in &snippet_children {
            if snippet_name == "failed" || snippet_name == "pending" {
                let key = self.ctx.b.alloc_str(snippet_name);
                props.push(ObjProp::KeyValue(key, self.ctx.b.rid_expr(key)));
            }
        }
        let props_expr = self.ctx.b.object_expr(props);

        let inner_ctx = ctx.child_of_block(
            self.ctx,
            boundary.fragment,
            FragmentAnchor::CallbackParam {
                name: "$$anchor".to_string(),
                append_inside: false,
            },
        );

        let const_tag_ids: Vec<NodeId> = self
            .ctx
            .query
            .component
            .store
            .fragment(boundary.fragment)
            .nodes
            .iter()
            .filter(|&&nid| {
                matches!(
                    self.ctx.query.component.store.get(nid),
                    svelte_ast::Node::ConstTag(_)
                )
            })
            .copied()
            .collect();

        let duplicate_consts = !const_tag_ids.is_empty()
            && !snippet_children.is_empty()
            && !self.ctx.state.experimental_async;

        let body_const_stmts: Vec<Statement<'a>> = if duplicate_consts {
            let mut const_state = EmitState::new();
            for &cid in &const_tag_ids {
                self.emit_hoisted_const_tag(&mut const_state, &inner_ctx, cid)?;
            }
            const_state.init
        } else {
            Vec::new()
        };

        let mut snippet_decls: Vec<Statement<'a>> = Vec::new();
        for (snippet_id, _) in &snippet_children {
            let sem = match self.ctx.query.analysis.block_semantics(*snippet_id) {
                svelte_analyze::BlockSemantics::Snippet(s) => s.clone(),
                _ => {
                    return CodegenError::unexpected_block_semantics(
                        *snippet_id,
                        "boundary child must map to Snippet",
                    );
                }
            };
            let prepend: Vec<Statement<'a>> = body_const_stmts
                .iter()
                .filter(|stmt| matches!(stmt, Statement::VariableDeclaration(_)))
                .map(|stmt| stmt.clone_in(self.ctx.b.ast.allocator))
                .collect();
            snippet_decls.push(self.build_snippet_const_with_prefix(*snippet_id, &sem, prepend)?);
        }

        let mut inner_state = EmitState::new();
        inner_state.skip_snippets = true;
        inner_state.skip_const_tags = duplicate_consts;
        self.emit_fragment(&mut inner_state, &inner_ctx, boundary.fragment)?;
        if duplicate_consts {
            inner_state.init.splice(0..0, body_const_stmts);
        }
        let body_stmts = self.pack_callback_body(inner_state, "$$anchor")?;
        let body_fn = self
            .ctx
            .b
            .arrow_block_expr(self.ctx.b.params(["$$anchor"]), body_stmts);

        let boundary_call = self.ctx.b.call_stmt(
            "$.boundary",
            [
                Arg::Ident(&anchor_node),
                Arg::Expr(props_expr),
                Arg::Expr(body_fn),
            ],
        );

        if snippet_decls.is_empty() {
            state.init.push(boundary_call);
        } else {
            let mut block = snippet_decls;
            block.push(boundary_call);
            state.init.push(self.ctx.b.block_stmt(block));
        }

        Ok(anchor_node)
    }
}
