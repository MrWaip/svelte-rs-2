use oxc_ast::ast::Expression;
use svelte_ast_builder::Arg;

use super::data_structures::{EmitState, FragmentAnchor, FragmentCtx, PreAnchor};
use super::{Codegen, CodegenError, Result};

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(in crate::codegen) fn reserve_comment_anchor_pre(
        &mut self,
        state: &mut EmitState<'a>,
        ctx: &FragmentCtx<'a>,
    ) -> PreAnchor {
        match &ctx.anchor {
            FragmentAnchor::Root
            | FragmentAnchor::CallbackParam {
                append_inside: false,
                ..
            } => {
                let (frag_name, node_name) = match state.pending_anchor_idents.take() {
                    Some((f, n)) if !n.is_empty() => (f, n),
                    Some((f, _)) => (f, self.ctx.state.gen_ident("node")),
                    None => {
                        let f = self.ctx.state.gen_ident("fragment");
                        let n = self.ctx.state.gen_ident("node");
                        (f, n)
                    }
                };
                PreAnchor {
                    node_name,
                    frag_name: Some(frag_name),
                    needs_template_comment: false,
                    is_child: false,
                    parent_var: None,
                    callback_param: None,
                    sibling_var: None,
                }
            }
            FragmentAnchor::CallbackParam {
                name,
                append_inside: true,
            } => {
                let node_name = self.ctx.state.gen_ident("node");
                PreAnchor {
                    node_name,
                    frag_name: None,
                    needs_template_comment: true,
                    is_child: false,
                    parent_var: None,
                    callback_param: Some(name.clone()),
                    sibling_var: None,
                }
            }
            FragmentAnchor::Child { parent_var } => {
                let node_name = self.ctx.state.gen_ident("node");
                PreAnchor {
                    node_name,
                    frag_name: None,
                    needs_template_comment: true,
                    is_child: true,
                    parent_var: Some(parent_var.clone()),
                    callback_param: None,
                    sibling_var: None,
                }
            }
            FragmentAnchor::SiblingVar { var } => PreAnchor {
                node_name: var.clone(),
                frag_name: None,
                needs_template_comment: false,
                is_child: false,
                parent_var: None,
                callback_param: None,
                sibling_var: Some(var.clone()),
            },
        }
    }

    pub(in crate::codegen) fn commit_comment_anchor(
        &mut self,
        state: &mut EmitState<'a>,
        _ctx: &FragmentCtx<'a>,
        pre: PreAnchor,
    ) -> Result<String> {
        if pre.sibling_var.is_some() {
            return Ok(pre.node_name);
        }
        if let Some(frag_name) = pre.frag_name {
            let b = &self.ctx.state.b;
            state
                .init
                .push(b.var_stmt(&frag_name, b.call_expr("$.comment", [])));
            state.init.push(b.var_stmt(
                &pre.node_name,
                b.call_expr("$.first_child", [Arg::Ident(&frag_name)]),
            ));
            state.root_var = Some(frag_name);
            return Ok(pre.node_name);
        }
        if pre.needs_template_comment {
            state.template.push_comment(None);
        }
        let Some(parent) = pre
            .parent_var
            .clone()
            .or_else(|| pre.callback_param.clone())
        else {
            return CodegenError::unexpected_child("parent_var or callback_param", "neither");
        };
        let b = &self.ctx.state.b;
        state.init.push(b.var_stmt(
            &pre.node_name,
            b.call_expr("$.child", [Arg::Ident(&parent)]),
        ));
        let _ = pre.is_child;
        Ok(pre.node_name)
    }

    pub(in crate::codegen) fn comment_anchor_node_name(
        &mut self,
        state: &mut EmitState<'a>,
        ctx: &FragmentCtx<'a>,
    ) -> Result<String> {
        match &ctx.anchor {
            FragmentAnchor::Root
            | FragmentAnchor::CallbackParam {
                append_inside: false,
                ..
            } => {
                debug_assert!(
                    state.root_var.is_none(),
                    "comment_anchor_node_name would overwrite existing root_var — \
                     caller must not have set it yet (Multi with blocks needs a different path)"
                );
                let (frag_name, node_name) = match state.pending_anchor_idents.take() {
                    Some((f, n)) if !n.is_empty() => (f, n),
                    Some((f, _)) => (f, self.ctx.state.gen_ident("node")),
                    None => {
                        let f = self.ctx.state.gen_ident("fragment");
                        let n = self.ctx.state.gen_ident("node");
                        (f, n)
                    }
                };
                let b = &self.ctx.state.b;
                state
                    .init
                    .push(b.var_stmt(&frag_name, b.call_expr("$.comment", [])));
                state.init.push(b.var_stmt(
                    &node_name,
                    b.call_expr("$.first_child", [Arg::Ident(&frag_name)]),
                ));
                state.root_var = Some(frag_name);
                Ok(node_name)
            }
            FragmentAnchor::CallbackParam {
                name,
                append_inside: true,
            } => {
                state.template.push_comment(None);
                let node_name = self.ctx.state.gen_ident("node");
                let parent = name.clone();
                let b = &self.ctx.state.b;
                state
                    .init
                    .push(b.var_stmt(&node_name, b.call_expr("$.child", [Arg::Ident(&parent)])));
                Ok(node_name)
            }
            FragmentAnchor::Child { parent_var } => {
                state.template.push_comment(None);
                let node_name = self.ctx.state.gen_ident("node");
                let parent = parent_var.clone();
                let b = &self.ctx.state.b;
                state
                    .init
                    .push(b.var_stmt(&node_name, b.call_expr("$.child", [Arg::Ident(&parent)])));
                Ok(node_name)
            }
            FragmentAnchor::SiblingVar { var } => Ok(var.clone()),
        }
    }

    pub(in crate::codegen) fn direct_anchor_expr(
        &mut self,
        state: &mut EmitState<'a>,
        ctx: &FragmentCtx<'a>,
    ) -> Result<Expression<'a>> {
        match &ctx.anchor {
            FragmentAnchor::Root => Ok(self.ctx.b.rid_expr("$$anchor")),
            FragmentAnchor::CallbackParam {
                append_inside: true,
                name,
            } => Ok(self.ctx.b.rid_expr(name)),
            FragmentAnchor::CallbackParam {
                append_inside: false,
                ..
            } => {
                let (frag_name, node_name) = match state.pending_anchor_idents.take() {
                    Some((f, n)) if !n.is_empty() => (f, n),
                    Some((f, _)) => (f, self.ctx.state.gen_ident("node")),
                    None => {
                        let f = self.ctx.state.gen_ident("fragment");
                        let n = self.ctx.state.gen_ident("node");
                        (f, n)
                    }
                };
                let b = &self.ctx.state.b;
                state
                    .init
                    .push(b.var_stmt(&frag_name, b.call_expr("$.comment", [])));
                state.init.push(b.var_stmt(
                    &node_name,
                    b.call_expr("$.first_child", [Arg::Ident(&frag_name)]),
                ));
                state.root_var = Some(frag_name.clone());
                Ok(b.rid_expr(&node_name))
            }
            FragmentAnchor::Child { parent_var } => {
                state.template.push_comment(None);
                let node_name = self.ctx.state.gen_ident("node");
                let parent = parent_var.clone();
                let b = &self.ctx.state.b;
                state
                    .init
                    .push(b.var_stmt(&node_name, b.call_expr("$.child", [Arg::Ident(&parent)])));
                Ok(b.rid_expr(&node_name))
            }
            FragmentAnchor::SiblingVar { var } => Ok(self.ctx.b.rid_expr(var)),
        }
    }
}
