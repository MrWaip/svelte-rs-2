//! SvelteBoundary codegen — `<svelte:boundary>...</svelte:boundary>`.
//!
//! Generates: `$.boundary(node, props, ($$anchor) => { ... })`

use oxc_ast::ast::{Expression, Statement};

use svelte_analyze::FragmentKey;
use svelte_ast::{Attribute, NodeId};

use crate::builder::{Arg, ObjProp};
use crate::context::Ctx;

use super::expression::get_attr_expr;
use super::gen_fragment;
use super::snippet::gen_snippet_block;

/// Build `const name = $.derived(() => expr)` statements from pre-cloned expressions.
/// Used to duplicate @const tags into boundary snippet bodies.
fn build_const_tag_stmts_from_cloned<'a>(
    ctx: &mut Ctx<'a>,
    entries: &mut Vec<(NodeId, Vec<String>, Expression<'a>)>,
) -> Vec<Statement<'a>> {
    let mut stmts = Vec::new();
    // Drain to take ownership of expressions
    for (cid, names, init_expr) in entries.drain(..) {
        if names.len() == 1 {
            let thunk = ctx.b.thunk(init_expr);
            let derived = ctx.b.call_expr("$.derived", [Arg::Expr(thunk)]);
            stmts.push(ctx.b.const_stmt(&names[0], derived));
        } else if names.len() > 1 {
            let tmp_name = ctx.transform_data.const_tag_tmp_names.get(&cid)
                .expect("destructured const tag must have tmp_name from transform");
            let tmp_name: &str = ctx.b.alloc_str(tmp_name);

            let destruct_stmt = ctx.b.const_object_destruct_stmt(&names, init_expr);

            let props: Vec<ObjProp<'a>> = names.iter()
                .map(|n| ObjProp::Shorthand(ctx.b.alloc_str(n)))
                .collect();
            let ret = ctx.b.return_stmt(ctx.b.object_expr(props));

            let thunk = ctx.b.arrow_block_expr(ctx.b.no_params(), [destruct_stmt, ret]);
            let derived = ctx.b.call_expr("$.derived", [Arg::Expr(thunk)]);
            stmts.push(ctx.b.const_stmt(tmp_name, derived));
        }
    }
    stmts
}

/// Generate `$.boundary(node, props, ($$anchor) => { ... })`.
pub(crate) fn gen_svelte_boundary<'a>(
    ctx: &mut Ctx<'a>,
    id: NodeId,
    node_expr: Expression<'a>,
    stmts: &mut Vec<Statement<'a>>,
) {
    let boundary = ctx.svelte_boundary(id);

    // Collect attribute info while borrowing ctx immutably.
    // Boundary only accepts ExpressionAttribute (onerror, failed, pending).
    let attr_infos: Vec<(&str, NodeId, bool)> = boundary
        .attributes
        .iter()
        .filter_map(|attr| {
            if let Attribute::ExpressionAttribute(a) = attr {
                let is_dynamic = ctx.is_dynamic_attr(a.id);
                Some((a.name.as_str(), a.id, is_dynamic))
            } else {
                None
            }
        })
        .collect();

    // Identify which snippet children are named "failed" or "pending" (go into props),
    // and collect all snippet block IDs for hoisting.
    let boundary = ctx.svelte_boundary(id);
    let mut hoisted_snippet_ids: Vec<(NodeId, Option<&str>)> = Vec::new();
    for node in &boundary.fragment.nodes {
        if let svelte_ast::Node::SnippetBlock(block) = node {
            let name = block.name.as_str();
            let prop_name = if name == "failed" || name == "pending" {
                Some(name)
            } else {
                None
            };
            hoisted_snippet_ids.push((block.id, prop_name));
        }
    }
    // Clone to release borrow
    let hoisted_snippet_ids: Vec<(NodeId, Option<String>)> = hoisted_snippet_ids
        .into_iter()
        .map(|(id, name)| (id, name.map(String::from)))
        .collect();

    // Build props object
    let mut props: Vec<ObjProp<'a>> = Vec::new();

    // Add attribute props (onerror, failed, pending as attributes).
    // Imports use getters (ES module live bindings), same as state.
    for (name, attr_id, is_dynamic) in &attr_infos {
        let key = ctx.b.alloc_str(name);
        let expr = get_attr_expr(ctx, *attr_id);
        let is_import = ctx.attr_is_import(*attr_id);
        if *is_dynamic || is_import {
            props.push(ObjProp::Getter(key, expr));
        } else {
            props.push(ObjProp::KeyValue(key, expr));
        }
    }

    // Clone const tag expressions BEFORE body generation consumes the originals.
    // The reference compiler duplicates @const tags into each hoisted snippet.
    let const_tag_ids: Vec<NodeId> = ctx
        .const_tags_for_fragment(&FragmentKey::SvelteBoundaryBody(id))
        .cloned()
        .unwrap_or_default();
    let has_const_tags = !const_tag_ids.is_empty();

    // For each snippet, we need a fresh copy of the const tag expressions.
    // Clone N copies (one per snippet) before the body generation consumes originals.
    let snippet_count = hoisted_snippet_ids.len();
    let mut cloned_exprs: Vec<Vec<(NodeId, Vec<String>, Expression<'a>)>> = Vec::new();
    if has_const_tags && snippet_count > 0 {
        // Collect info and clone expressions for each snippet
        let mut infos: Vec<(NodeId, Vec<String>)> = Vec::new();
        for &cid in &const_tag_ids {
            let names = ctx.const_tag_names(cid).cloned().unwrap_or_default();
            infos.push((cid, names));
        }

        for _ in 0..snippet_count {
            let mut set: Vec<(NodeId, Vec<String>, Expression<'a>)> = Vec::new();
            for (cid, names) in &infos {
                // Clone from the parsed expr cache (not removing — just borrowing and cloning)
                let cid_offset = ctx.node_expr_offset(*cid);
                if let Some(expr) = ctx.parsed.exprs.get(&cid_offset) {
                    let cloned = ctx.b.clone_expr(expr);
                    set.push((*cid, names.clone(), cloned));
                }
            }
            cloned_exprs.push(set);
        }
    }

    // Generate hoisted snippet declarations FIRST (ordering matters for ident numbering).
    // Snippets use cloned const tag expressions; the body uses originals.
    let mut hoisted_stmts: Vec<Statement<'a>> = Vec::new();
    for (i, (snippet_id, prop_name)) in hoisted_snippet_ids.iter().enumerate() {
        let const_tag_stmts = if has_const_tags && i < cloned_exprs.len() {
            build_const_tag_stmts_from_cloned(ctx, &mut cloned_exprs[i])
        } else {
            vec![]
        };

        let snippet_stmt = gen_snippet_block(ctx, *snippet_id, const_tag_stmts);
        hoisted_stmts.push(snippet_stmt);

        if let Some(name) = prop_name {
            let key = ctx.b.alloc_str(name);
            props.push(ObjProp::Shorthand(key));
        }
    }

    // TODO(Tier 2e): experimental.async — const tag scoping differs when async mode is enabled
    // TODO(Tier 6c): dev mode — $.wrap_snippet for snippet wrapping
    // TODO(Tier 6c): dev mode — handler wrapping for snippet params as event handlers

    // Generate body fragment (consumes const tag expressions via emit_const_tags)
    let body_stmts = gen_fragment(ctx, FragmentKey::SvelteBoundaryBody(id));

    let props_expr = ctx.b.object_expr(props);
    let body_fn = ctx.b.arrow_block_expr(ctx.b.params(["$$anchor"]), body_stmts);

    let boundary_call = ctx.b.call_stmt(
        "$.boundary",
        [Arg::Expr(node_expr), Arg::Expr(props_expr), Arg::Expr(body_fn)],
    );

    if hoisted_stmts.is_empty() {
        stmts.push(boundary_call);
    } else {
        // Wrap in a block: { ...hoisted; $.boundary(...) }
        hoisted_stmts.push(boundary_call);
        stmts.push(ctx.b.block_stmt(hoisted_stmts));
    }
}
