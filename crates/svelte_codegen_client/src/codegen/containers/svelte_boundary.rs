use oxc_ast::ast::{Expression, Statement};
use rustc_hash::FxHashSet;
use svelte_analyze::{BlockSemantics, ConstTagBlockSemantics};
use svelte_ast::{Attribute, Node, NodeId};
use svelte_ast_builder::{Arg, ObjProp};

use super::super::data_structures::EmitState;
use super::super::data_structures::{FragmentAnchor, FragmentCtx};
use super::super::{Codegen, CodegenError, Result};

fn const_tag_bindings(
    ctx: &crate::context::Ctx<'_>,
    sem: &ConstTagBlockSemantics,
) -> (Vec<oxc_semantic::SymbolId>, bool) {
    use oxc_ast::AstKind;
    use oxc_ast::ast::BindingPattern;
    let Some(AstKind::VariableDeclaration(decl)) =
        ctx.query.view.scoping().js_kind(sem.decl_node_id)
    else {
        return (Vec::new(), false);
    };
    let Some(declarator) = decl.declarations.first() else {
        return (Vec::new(), false);
    };
    let is_destructured = !matches!(declarator.id, BindingPattern::BindingIdentifier(_));
    let mut bindings: Vec<oxc_semantic::SymbolId> = Vec::new();
    svelte_component_semantics::walk_bindings(&declarator.id, |v| bindings.push(v.symbol));
    (bindings, is_destructured)
}

fn build_const_tag_prefix_stmts<'a>(
    ctx: &mut crate::context::Ctx<'a>,
    cloned: &mut Vec<(Vec<String>, Expression<'a>)>,
) -> Vec<Statement<'a>> {
    let mut stmts: Vec<Statement<'a>> = Vec::new();
    for (names, init) in cloned.drain(..) {
        let Some(first) = names.first() else {
            continue;
        };
        let thunk = ctx.b.thunk(init);
        let derived = ctx.b.call_expr("$.derived", [Arg::Expr(thunk)]);
        stmts.push(ctx.b.const_stmt(first, derived));
    }
    stmts
}

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

        let attr_infos: Vec<(String, NodeId, oxc_syntax::node::NodeId, bool, bool)> = boundary
            .attributes
            .iter()
            .filter_map(|attr| match attr {
                Attribute::ExpressionAttribute(a) => {
                    let is_dynamic = self.ctx.is_dynamic_attr(a.id);
                    let is_import = self.ctx.attr_is_import(a.id);
                    Some((
                        a.name.to_string(),
                        a.id,
                        a.expression.id(),
                        is_dynamic,
                        is_import,
                    ))
                }
                _ => None,
            })
            .collect();

        let anchor_node = self.comment_anchor_node_name(state, ctx)?;

        let mut props: Vec<ObjProp<'a>> = Vec::new();
        for (name, attr_id, expr_id, is_dynamic, is_import) in attr_infos {
            let key = self.ctx.b.alloc_str(&name);
            let Some(expr) = self.ctx.state.parsed.take_expr(expr_id) else {
                return crate::codegen::CodegenError::missing_expression(attr_id);
            };
            let expr = self.maybe_wrap_legacy_slots_read(expr);
            if is_dynamic || is_import {
                props.push(ObjProp::Getter(key, expr));
            } else {
                props.push(ObjProp::KeyValue(key, expr));
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

        let mut const_binding_syms: FxHashSet<oxc_semantic::SymbolId> = FxHashSet::default();
        let mut const_tag_names: Vec<(NodeId, Vec<String>)> = Vec::new();
        for &cid in &const_tag_ids {
            let sem = match self.ctx.query.analysis.block_semantics(cid) {
                BlockSemantics::ConstTag(s) => s.clone(),
                _ => continue,
            };
            let (bindings, _) = const_tag_bindings(self.ctx, &sem);
            let names: Vec<String> = bindings
                .iter()
                .map(|&s| {
                    const_binding_syms.insert(s);
                    self.ctx.query.view.symbol_name(s).to_string()
                })
                .collect();
            const_tag_names.push((cid, names));
        }

        let mut cloned_exprs_per_snippet: Vec<Vec<(Vec<String>, Expression<'a>)>> = Vec::new();
        if !const_tag_ids.is_empty() && !snippet_children.is_empty() {
            for _ in 0..snippet_children.len() {
                let mut set: Vec<(Vec<String>, Expression<'a>)> = Vec::new();
                for (cid, names) in &const_tag_names {
                    let svelte_ast::Node::ConstTag(const_tag) =
                        self.ctx.query.component.store.get(*cid)
                    else {
                        continue;
                    };
                    if let Some(Statement::VariableDeclaration(decl)) =
                        self.ctx.state.parsed.stmt(const_tag.decl.id())
                        && let Some(init) = decl.declarations.first().and_then(|d| d.init.as_ref())
                    {
                        let cloned = self.ctx.b.clone_expr(init);
                        set.push((names.clone(), cloned));
                    }
                }
                cloned_exprs_per_snippet.push(set);
            }
        }

        let mut snippet_decls: Vec<oxc_ast::ast::Statement<'a>> = Vec::new();
        for (i, (snippet_id, _)) in snippet_children.iter().enumerate() {
            let sem = match self.ctx.query.analysis.block_semantics(*snippet_id) {
                svelte_analyze::BlockSemantics::Snippet(s) => s.clone(),
                _ => {
                    return CodegenError::unexpected_block_semantics(
                        *snippet_id,
                        "boundary child must map to Snippet",
                    );
                }
            };
            let snippet_uses_const = if !const_binding_syms.is_empty() {
                let body_id = match self.ctx.query.component.store.get(*snippet_id) {
                    Node::SnippetBlock(block) => block.body,
                    _ => continue,
                };
                self.ctx
                    .fragment_references_any_symbol(body_id, &const_binding_syms)
            } else {
                false
            };
            let prepend = if snippet_uses_const && i < cloned_exprs_per_snippet.len() {
                build_const_tag_prefix_stmts(self.ctx, &mut cloned_exprs_per_snippet[i])
            } else {
                Vec::new()
            };
            snippet_decls.push(self.build_snippet_const_with_prefix(*snippet_id, &sem, prepend)?);
        }

        let mut inner_state = EmitState::new();
        inner_state.skip_snippets = true;
        self.emit_fragment(&mut inner_state, &inner_ctx, boundary.fragment)?;
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
