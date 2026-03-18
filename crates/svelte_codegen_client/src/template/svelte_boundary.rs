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

    // Add attribute props (onerror, failed, pending as attributes)
    for (name, attr_id, is_dynamic) in &attr_infos {
        let key = ctx.b.alloc_str(name);
        let expr = get_attr_expr(ctx, *attr_id);
        if *is_dynamic {
            props.push(ObjProp::Getter(key, expr));
        } else {
            props.push(ObjProp::KeyValue(key, expr));
        }
    }

    // Generate hoisted snippet declarations + add named ones to props
    let mut hoisted_stmts: Vec<Statement<'a>> = Vec::new();
    for (snippet_id, prop_name) in &hoisted_snippet_ids {
        // Generate const tags that need to be duplicated into snippets
        // (The reference compiler duplicates @const tags into hoisted snippets)
        let boundary = ctx.svelte_boundary(id);
        let const_tag_ids: Vec<NodeId> = boundary
            .fragment
            .nodes
            .iter()
            .filter_map(|n| {
                if let svelte_ast::Node::ConstTag(tag) = n {
                    Some(tag.id)
                } else {
                    None
                }
            })
            .collect();

        let snippet_stmt = gen_snippet_block(ctx, *snippet_id);

        // If there are const tags, inject their declarations into the snippet body.
        // The reference compiler duplicates const tags into each hoisted snippet.
        // For now we skip this duplication — it's a deferred item.
        let _ = const_tag_ids;

        hoisted_stmts.push(snippet_stmt);

        if let Some(name) = prop_name {
            let key = ctx.b.alloc_str(name);
            props.push(ObjProp::Shorthand(key));
        }
    }

    // Generate body fragment (excludes SnippetBlock and ConstTag children,
    // which are handled separately by gen_fragment through the lowered fragment)
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
