use smallvec::SmallVec;
use svelte_ast::{Node, NodeId};
use svelte_ast_builder::{Arg, AssignLeft, TemplatePart};

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

        for child in children {
            match child {
                Child::Text(part) => {
                    if let Some(text) = ctx.static_text_of(part) {
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
                    emit_text_set(self, state, &node_name, *id)?;
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
                    emit_concat_set(self, state, ctx, &node_name, parts)?;
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
            }
        }

        if prev.is_none() && matches!(ctx.anchor, FragmentAnchor::Child { .. }) {
            return Ok(());
        }
        let _ = prefix_next_emitted;
        if skipped > 1 {
            let trailing = skipped - 1;
            let b = &self.ctx.state.b;
            if trailing == 1 {
                state
                    .init
                    .push(b.call_stmt("$.next", std::iter::empty::<Arg<'a, '_>>()));
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

fn emit_text_set<'a, 'ctx>(
    cg: &mut Codegen<'a, 'ctx>,
    state: &mut EmitState<'a>,
    node_name: &str,
    id: NodeId,
) -> Result<()> {
    let is_dyn = cg.ctx.is_dynamic(id);
    let expr = cg.take_node_expr(id)?;
    let b = &cg.ctx.state.b;
    if is_dyn {
        state
            .update
            .push(b.call_stmt("$.set_text", [Arg::Ident(node_name), Arg::Expr(expr)]));
    } else {
        let final_expr = match cg.try_resolve_known_from_expr(&expr) {
            Some(s) => b.str_expr(&s),
            None => expr,
        };
        let member = b.static_member(b.rid_expr(node_name), "nodeValue");
        state
            .init
            .push(b.assign_stmt(AssignLeft::StaticMember(member), final_expr));
    }
    Ok(())
}

fn emit_concat_set<'a, 'ctx>(
    cg: &mut Codegen<'a, 'ctx>,
    state: &mut EmitState<'a>,
    ctx: &FragmentCtx<'a>,
    node_name: &str,
    parts: &[ConcatPart],
) -> Result<()> {
    let is_dyn = parts.iter().any(|p| match p {
        ConcatPart::Expr(id) => cg.ctx.is_dynamic(*id),
        _ => false,
    });

    let mut tpl_parts: Vec<TemplatePart<'a>> = Vec::with_capacity(parts.len());
    for part in parts {
        if let Some(s) = ctx.static_text_of(part) {
            if let Some(TemplatePart::Str(prev)) = tpl_parts.last_mut() {
                prev.push_str(s);
            } else {
                tpl_parts.push(TemplatePart::Str(s.to_string()));
            }
        } else if let ConcatPart::Expr(id) = part {
            let expr = cg.take_node_expr(*id)?;
            if let Some(known) = cg.try_resolve_known_from_expr(&expr) {
                if let Some(TemplatePart::Str(prev)) = tpl_parts.last_mut() {
                    prev.push_str(&known);
                } else {
                    tpl_parts.push(TemplatePart::Str(known));
                }
            } else {
                let defined = cg.is_node_expr_definitely_defined(*id, &expr);
                let info = cg.ctx.expression(*id).cloned();
                let wrapped = cg.maybe_wrap_legacy_coarse_expr(expr, info.as_ref());
                tpl_parts.push(TemplatePart::Expr(wrapped, defined));
            }
        }
    }

    let b = &cg.ctx.state.b;
    let all_static = tpl_parts.iter().all(|p| matches!(p, TemplatePart::Str(_)));
    let tpl_expr = if all_static && tpl_parts.len() == 1 {
        match tpl_parts.into_iter().next() {
            Some(TemplatePart::Str(s)) => b.str_expr(&s),
            _ => b.template_parts_expr(Vec::new()),
        }
    } else {
        b.template_parts_expr(tpl_parts)
    };

    if is_dyn {
        state
            .update
            .push(b.call_stmt("$.set_text", [Arg::Ident(node_name), Arg::Expr(tpl_expr)]));
    } else {
        let member = b.static_member(b.rid_expr(node_name), "nodeValue");
        state
            .init
            .push(b.assign_stmt(AssignLeft::StaticMember(member), tpl_expr));
    }
    Ok(())
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

fn make_sibling_expr<'a, 'ctx>(
    cg: &mut Codegen<'a, 'ctx>,
    prev: &Option<String>,
    skipped: u32,
    initial: &mut Option<ChildAnchor>,
    is_text: bool,
) -> Result<oxc_ast::ast::Expression<'a>> {
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
