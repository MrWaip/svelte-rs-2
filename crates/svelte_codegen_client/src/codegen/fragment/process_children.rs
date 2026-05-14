use std::iter;

use oxc_ast::ast::Expression;
use smallvec::SmallVec;
use svelte_ast::{Node, NodeId};
use svelte_ast_builder::Arg;

use crate::codegen::concatenation::ConcatenationAnchor;
use crate::codegen::data_structures::{ConcatPart, EmitState, FragmentAnchor, FragmentCtx};
use crate::codegen::fragment::types::{Child, ChildAnchor};
use crate::codegen::{Codegen, CodegenError, Result};

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(super) fn process_children_with_prefix(
        &mut self,
        state: &mut EmitState<'a>,
        ctx: &FragmentCtx<'a>,
        children: &[Child],
        prefix_next_emitted: bool,
    ) -> Result<()> {
        let initial = match &ctx.anchor {
            FragmentAnchor::Child { parent_var } => ChildAnchor::ElementChild {
                parent_var: parent_var.clone(),
            },
            FragmentAnchor::CallbackParam {
                name,
                append_inside: true,
            } => ChildAnchor::ElementChild {
                parent_var: name.clone(),
            },
            FragmentAnchor::Root
            | FragmentAnchor::CallbackParam {
                append_inside: false,
                ..
            } => {
                let frag_name = match state.pending_anchor_idents.as_ref() {
                    Some((f, _)) => f.clone(),
                    None => self.ctx.state.gen_ident("fragment"),
                };
                state.root_var = Some(frag_name.clone());
                ChildAnchor::FragmentFirstChild {
                    frag_var: frag_name,
                }
            }
            FragmentAnchor::SiblingVar { var } => ChildAnchor::RawIdent(var.clone()),
        };

        let mut prev: Option<String> = None;
        let mut skipped: u32 = 0;
        let mut initial_opt = Some(initial);

        for (idx, child) in children.iter().enumerate() {
            match child {
                Child::Text(part) => {
                    if let Some(text) = ctx.static_html_of(part) {
                        state.template.push_text(text);
                    }
                    skipped += 1;
                }
                Child::Expr(id) => {
                    state.template.push_text(" ");
                    let node_name = flush_sibling_var(
                        self,
                        state,
                        &mut prev,
                        &mut skipped,
                        &mut initial_opt,
                        true,
                        "text",
                    )?;
                    self.emit_concatenation(
                        state,
                        ctx,
                        ConcatenationAnchor::SiblingTextNode {
                            node_var: node_name,
                        },
                        &[ConcatPart::Expr(*id)],
                    )?;
                }
                Child::Concat(parts) => {
                    state.template.push_text(" ");
                    let is_standalone_expr =
                        parts.len() == 1 && matches!(parts.first(), Some(ConcatPart::Expr(_)));
                    let node_name = flush_sibling_var(
                        self,
                        state,
                        &mut prev,
                        &mut skipped,
                        &mut initial_opt,
                        is_standalone_expr,
                        "text",
                    )?;
                    self.emit_concatenation(
                        state,
                        ctx,
                        ConcatenationAnchor::SiblingTextNode {
                            node_var: node_name,
                        },
                        parts,
                    )?;
                }
                Child::Node(id) => {
                    emit_child_node(
                        self,
                        state,
                        ctx,
                        *id,
                        &mut prev,
                        &mut skipped,
                        &mut initial_opt,
                    )?;
                }
                Child::Comment(data) => {
                    if comment_needs_var_extraction(self, ctx, &children[idx + 1..]) {
                        flush_sibling_var(
                            self,
                            state,
                            &mut prev,
                            &mut skipped,
                            &mut initial_opt,
                            false,
                            "node",
                        )?;
                        state.template.push_comment(Some(data.clone()));
                    } else {
                        state.template.push_comment(Some(data.clone()));
                        skipped += 1;
                    }
                }
            }
        }

        if prev.is_none() && matches!(ctx.anchor, FragmentAnchor::Child { .. }) {
            state.last_fragment_needs_reset = false;
            return Ok(());
        }
        state.last_fragment_needs_reset = true;
        let _ = prefix_next_emitted;
        if skipped > 1 {
            let trailing = skipped - 1;
            let b = &self.ctx.state.b;
            if trailing == 1 {
                state
                    .init
                    .push(b.call_stmt("$.next", iter::empty::<Arg<'a, '_>>()));
            } else {
                state
                    .init
                    .push(b.call_stmt("$.next", [Arg::Num(trailing as f64)]));
            }
        }

        Ok(())
    }
}

fn emit_child_node<'a, 'ctx>(
    cg: &mut Codegen<'a, 'ctx>,
    state: &mut EmitState<'a>,
    ctx: &FragmentCtx<'a>,
    id: NodeId,
    prev: &mut Option<String>,
    skipped: &mut u32,
    initial: &mut Option<ChildAnchor>,
) -> Result<()> {
    let node = cg.ctx.query.component.store.get(id);
    match node {
        Node::Element(el) => {
            if !cg.ctx.needs_var(id) {
                cg.emit_element_ghost(state, ctx, id)?;
                *skipped += 1;
                return Ok(());
            }

            let expr = make_sibling_expr(cg, prev, *skipped, initial, false)?;
            let prefix = cg.element_ident_prefix(&el.name);
            let el_name = cg.ctx.state.gen_ident(&prefix);
            let b = &cg.ctx.state.b;
            state.init.push(b.var_stmt(&el_name, expr));
            *prev = Some(el_name.clone());
            *skipped = 1;

            cg.emit_element(state, ctx, id, Some(&el_name))?;
            Ok(())
        }
        Node::ComponentNode(_)
        | Node::SvelteComponentLegacy(_)
        | Node::SvelteElement(_)
        | Node::SvelteBoundary(_)
        | Node::SlotElementLegacy(_)
        | Node::SvelteFragmentLegacy(_)
        | Node::IfBlock(_)
        | Node::EachBlock(_)
        | Node::AwaitBlock(_)
        | Node::KeyBlock(_)
        | Node::HtmlTag(_)
        | Node::RenderTag(_) => {
            let is_css_wrapped_component = matches!(
                node,
                Node::ComponentNode(_) | Node::SvelteComponentLegacy(_)
            ) && cg.ctx.has_component_css_props(id);

            if is_css_wrapped_component {
                let namespace = ctx.namespace;
                if matches!(namespace, svelte_ast::Namespace::Svg) {
                    state.template.push_element("g", false);
                } else {
                    state.template.push_element("svelte-css-wrapper", true);
                    state
                        .template
                        .set_attribute("style", Some("display: contents".to_string()));
                }
                state.template.push_comment(None);
                state.template.pop_element();

                let expr = make_sibling_expr(cg, prev, *skipped, initial, false)?;
                let node_name = match state.pending_anchor_idents.take() {
                    Some((_, n)) if !n.is_empty() => n,
                    _ => cg.ctx.state.gen_ident("node"),
                };
                let b = &cg.ctx.state.b;
                state.init.push(b.var_stmt(&node_name, expr));
                *prev = Some(node_name.clone());
                *skipped = 1;

                cg.emit_css_props_wrapper_block(state, ctx, id, &node_name, namespace)?;
                return Ok(());
            }

            state.template.push_comment(None);
            let expr = make_sibling_expr(cg, prev, *skipped, initial, false)?;
            let node_name = match state.pending_anchor_idents.take() {
                Some((_, n)) if !n.is_empty() => n,
                _ => cg.ctx.state.gen_ident("node"),
            };
            let b = &cg.ctx.state.b;
            state.init.push(b.var_stmt(&node_name, expr));
            *prev = Some(node_name.clone());
            *skipped = 1;

            let child_ctx = ctx.child_of_sibling(node_name.clone());
            cg.emit_fragment_child(state, &child_ctx, id)?;
            Ok(())
        }
        _ => CodegenError::unexpected_node(id, "Element or block-like child"),
    }
}

fn flush_sibling_var<'a, 'ctx>(
    cg: &mut Codegen<'a, 'ctx>,
    state: &mut EmitState<'a>,
    prev: &mut Option<String>,
    skipped: &mut u32,
    initial: &mut Option<ChildAnchor>,
    is_text: bool,
    name_hint: &str,
) -> Result<String> {
    let expr = make_sibling_expr(cg, prev, *skipped, initial, is_text)?;
    let reserved_node = if name_hint == "node" {
        state.pending_anchor_idents.take().map(|(_, n)| n)
    } else {
        None
    };
    let id = match reserved_node {
        Some(name) => name,
        None => cg.ctx.state.gen_ident(name_hint),
    };
    let b = &cg.ctx.state.b;
    state.init.push(b.var_stmt(&id, expr));
    *prev = Some(id.clone());
    *skipped = 1;
    Ok(id)
}

fn comment_needs_var_extraction<'a, 'ctx>(
    cg: &Codegen<'a, 'ctx>,
    ctx: &FragmentCtx<'a>,
    later: &[Child],
) -> bool {
    let parent_is_static_element = matches!(ctx.anchor, FragmentAnchor::Child { .. });
    if !parent_is_static_element {
        return true;
    }
    later.iter().any(|child| match child {
        Child::Text(_) | Child::Comment(_) => false,
        Child::Expr(_) | Child::Concat(_) => true,
        Child::Node(id) => {
            let node = cg.ctx.query.component.store.get(*id);
            !matches!(node, Node::Element(_)) || cg.ctx.needs_var(*id)
        }
    })
}

fn make_sibling_expr<'a, 'ctx>(
    cg: &mut Codegen<'a, 'ctx>,
    prev: &Option<String>,
    skipped: u32,
    initial: &mut Option<ChildAnchor>,
    is_text: bool,
) -> Result<Expression<'a>> {
    let b = &cg.ctx.state.b;
    if let Some(prev_name) = prev {
        if skipped == 0 {
            return Ok(b.rid_expr(prev_name));
        }
        let mut args: SmallVec<[Arg<'a, '_>; 3]> = SmallVec::new();
        args.push(Arg::Ident(prev_name));
        if is_text || skipped != 1 {
            args.push(Arg::Num(skipped as f64));
        }
        if is_text {
            args.push(Arg::Bool(true));
        }
        return Ok(b.call_expr("$.sibling", args));
    }

    let Some(anchor) = initial.take() else {
        return CodegenError::unexpected_child(
            "initial anchor",
            "anchor already consumed by previous child",
        );
    };
    let base_expr = match anchor {
        ChildAnchor::RawIdent(name) => b.rid_expr(&name),
        ChildAnchor::ElementChild { parent_var } => {
            if is_text {
                b.call_expr("$.child", [Arg::Ident(&parent_var), Arg::Bool(true)])
            } else {
                b.call_expr("$.child", [Arg::Ident(&parent_var)])
            }
        }
        ChildAnchor::FragmentFirstChild { frag_var } => {
            b.call_expr("$.first_child", [Arg::Ident(&frag_var)])
        }
    };
    if skipped == 0 {
        return Ok(base_expr);
    }
    let mut args: SmallVec<[Arg<'a, '_>; 3]> = SmallVec::new();
    args.push(Arg::Expr(base_expr));
    if is_text || skipped != 1 {
        args.push(Arg::Num(skipped as f64));
    }
    if is_text {
        args.push(Arg::Bool(true));
    }
    Ok(b.call_expr("$.sibling", args))
}
