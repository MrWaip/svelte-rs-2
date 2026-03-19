//! AST-to-AST transformation pass for Svelte expressions.
//!
//! Transforms rune references, prop accesses, and each-block variables
//! in pre-parsed Expression ASTs. Runs between analyze and codegen.

mod data;
pub mod rune_refs;

pub use data::TransformData;

use oxc_allocator::Allocator;
use oxc_ast::ast::*;

use svelte_analyze::scope::ScopeId;
use svelte_analyze::{AnalysisData, FragmentKey, IdentGen, ParsedExprs};
use svelte_ast::{
    Attribute, Component, Fragment, Node, NodeId,
};


/// Transform all parsed template expressions in-place.
///
/// Rewrites:
/// - Rune references → `$.get(name)` / `$.set(name, val)` / `$.update(name)`
/// - Prop sources → `name()` (thunk call)
/// - Prop non-sources → `$$props.name`
/// - Each-block context variables → `$.get(name)`
/// - Snippet parameters → `name()` (thunk call)
/// - Destructured const aliases → `$.get(tmp).prop`
///
/// Must be called AFTER all analysis passes are complete.
pub fn transform_component<'a>(
    alloc: &'a Allocator,
    component: &Component,
    analysis: &AnalysisData,
    parsed: &mut ParsedExprs<'a>,
    ident_gen: &mut IdentGen,
) -> TransformData {
    let root_scope = analysis.scoping.root_scope_id();

    let mut ctx = TransformCtx {
        alloc,
        analysis,
        ident_gen,
        transform_data: TransformData::new(),
    };

    walk_fragment(&mut ctx, &component.fragment, component, parsed, root_scope);
    ctx.transform_data
}

struct TransformCtx<'a, 'b> {
    alloc: &'a Allocator,
    analysis: &'b AnalysisData,
    ident_gen: &'b mut IdentGen,
    transform_data: TransformData,
}

// ---------------------------------------------------------------------------
// Template tree walker (tracks scope)
// ---------------------------------------------------------------------------

fn walk_fragment<'a>(
    ctx: &mut TransformCtx<'a, '_>,
    fragment: &Fragment,
    component: &Component,
    parsed: &mut ParsedExprs<'a>,
    scope: ScopeId,
) {
    for node in &fragment.nodes {
        walk_node(ctx, node, component, parsed, scope);
    }
}

fn walk_node<'a>(
    ctx: &mut TransformCtx<'a, '_>,
    node: &Node,
    component: &Component,
    parsed: &mut ParsedExprs<'a>,
    scope: ScopeId,
) {
    match node {
        Node::ExpressionTag(tag) => {
            transform_node_expr(ctx, tag.id, parsed, scope);
        }
        Node::Element(el) => {
            transform_attrs(ctx, &el.attributes, parsed, scope);
            walk_fragment(ctx, &el.fragment, component, parsed, scope);
        }
        Node::ComponentNode(cn) => {
            transform_attrs(ctx, &cn.attributes, parsed, scope);
            walk_fragment(ctx, &cn.fragment, component, parsed, scope);
        }
        Node::IfBlock(block) => {
            transform_node_expr(ctx, block.id, parsed, scope);
            let cons_scope = ctx.analysis.scoping.fragment_scope(&FragmentKey::IfConsequent(block.id)).unwrap_or(scope);
            walk_fragment(ctx, &block.consequent, component, parsed, cons_scope);
            if let Some(alt) = &block.alternate {
                let alt_scope = ctx.analysis.scoping.fragment_scope(&FragmentKey::IfAlternate(block.id)).unwrap_or(scope);
                walk_fragment(ctx, alt, component, parsed, alt_scope);
            }
        }
        Node::EachBlock(block) => {
            transform_node_expr(ctx, block.id, parsed, scope);

            // Each block body uses child scope (context + index vars)
            let body_scope = ctx.analysis.scoping.node_scope(block.id).unwrap_or(scope);
            walk_fragment(ctx, &block.body, component, parsed, body_scope);

            // Fallback uses parent scope
            if let Some(fb) = &block.fallback {
                walk_fragment(ctx, fb, component, parsed, scope);
            }
        }
        Node::SnippetBlock(block) => {
            let snippet_scope = ctx.analysis.scoping.node_scope(block.id).unwrap_or(scope);
            walk_fragment(ctx, &block.body, component, parsed, snippet_scope);
        }
        Node::RenderTag(tag) => {
            transform_node_expr(ctx, tag.id, parsed, scope);
        }
        Node::HtmlTag(tag) => {
            transform_node_expr(ctx, tag.id, parsed, scope);
        }
        Node::ConstTag(tag) => {
            transform_node_expr(ctx, tag.id, parsed, scope);

            let names = ctx.analysis.const_tags.names(tag.id).cloned().unwrap_or_default();
            if names.len() > 1 {
                let tmp = ctx.ident_gen.gen("computed_const");
                ctx.transform_data.const_tag_tmp_names.insert(tag.id, tmp);
            }
        }
        Node::KeyBlock(block) => {
            transform_node_expr(ctx, block.id, parsed, scope);
            let child_scope = ctx.analysis.scoping.fragment_scope(&FragmentKey::KeyBlockBody(block.id)).unwrap_or(scope);
            walk_fragment(ctx, &block.fragment, component, parsed, child_scope);
        }
        Node::SvelteHead(head) => {
            let child_scope = ctx.analysis.scoping.fragment_scope(&FragmentKey::SvelteHeadBody(head.id)).unwrap_or(scope);
            walk_fragment(ctx, &head.fragment, component, parsed, child_scope);
        }
        Node::SvelteElement(el) => {
            if !el.static_tag {
                transform_node_expr(ctx, el.id, parsed, scope);
            }
            transform_attrs(ctx, &el.attributes, parsed, scope);
            let child_scope = ctx.analysis.scoping.fragment_scope(&FragmentKey::SvelteElementBody(el.id)).unwrap_or(scope);
            walk_fragment(ctx, &el.fragment, component, parsed, child_scope);
        }
        Node::SvelteWindow(w) => {
            transform_attrs(ctx, &w.attributes, parsed, scope);
        }
        Node::SvelteDocument(d) => {
            transform_attrs(ctx, &d.attributes, parsed, scope);
        }
        Node::SvelteBody(b) => {
            transform_attrs(ctx, &b.attributes, parsed, scope);
        }
        Node::SvelteBoundary(b) => {
            transform_attrs(ctx, &b.attributes, parsed, scope);
            let child_scope = ctx.analysis.scoping.fragment_scope(&FragmentKey::SvelteBoundaryBody(b.id)).unwrap_or(scope);
            walk_fragment(ctx, &b.fragment, component, parsed, child_scope);
        }
        Node::AwaitBlock(block) => {
            transform_node_expr(ctx, block.id, parsed, scope);
            if let Some(ref p) = block.pending {
                let pending_scope = ctx.analysis.scoping.fragment_scope(&FragmentKey::AwaitPending(block.id)).unwrap_or(scope);
                walk_fragment(ctx, p, component, parsed, pending_scope);
            }
            if let Some(ref t) = block.then {
                let then_scope = ctx.analysis.scoping.node_scope(block.id).unwrap_or(scope);
                walk_fragment(ctx, t, component, parsed, then_scope);
            }
            if let Some(ref c) = block.catch {
                let catch_scope = ctx.analysis.scoping.await_catch_scope(block.id).unwrap_or(scope);
                walk_fragment(ctx, c, component, parsed, catch_scope);
            }
        }
        Node::DebugTag(_) | Node::Text(_) | Node::Comment(_) | Node::Error(_) => {}
    }
}

/// Transform a single expression stored in `parsed.exprs` at the given node ID.
fn transform_node_expr<'a>(
    ctx: &mut TransformCtx<'a, '_>,
    node_id: NodeId,
    parsed: &mut ParsedExprs<'a>,
    scope: ScopeId,
) {
    if let Some(expr) = parsed.exprs.get_mut(&node_id) {
        transform_expr(ctx, expr, scope);
    }
}

/// Transform attribute expressions.
fn transform_attrs<'a>(
    ctx: &mut TransformCtx<'a, '_>,
    attrs: &[Attribute],
    parsed: &mut ParsedExprs<'a>,
    scope: ScopeId,
) {
    for attr in attrs {
        let attr_id = attr.id();
        if let Some(expr) = parsed.attr_exprs.get_mut(&attr_id) {
            transform_expr(ctx, expr, scope);
        }

        // Transform concat dynamic parts (ConcatenationAttribute + StyleDirective::Concatenation)
        let concat_parts: Option<&[svelte_ast::ConcatPart]> = match attr {
            Attribute::ConcatenationAttribute(a) => Some(&a.parts),
            Attribute::StyleDirective(a) => match &a.value {
                svelte_ast::StyleDirectiveValue::Concatenation(parts) => Some(parts),
                _ => None,
            },
            _ => None,
        };
        if let Some(parts) = concat_parts {
            let dyn_count = parts.iter().filter(|p| matches!(p, svelte_ast::ConcatPart::Dynamic(_))).count();
            for dyn_idx in 0..dyn_count {
                let part_key = (attr_id, dyn_idx);
                if let Some(expr) = parsed.concat_part_exprs.get_mut(&part_key) {
                    transform_expr(ctx, expr, scope);
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Expression transformer (recursive)
// ---------------------------------------------------------------------------

/// Recursively transform an Expression in-place.
///
/// Arrow function parameters are registered as scoped bindings during analysis
/// (`register_arrow_scopes`), so `find_binding` naturally resolves them — no
/// separate shadow stack needed.
fn transform_expr<'a>(
    ctx: &mut TransformCtx<'a, '_>,
    expr: &mut Expression<'a>,
    scope: ScopeId,
) {
    match expr {
        Expression::Identifier(id) => {
            let name = id.name.as_str();

            // Store subscriptions: $X → $X() (thunk call).
            if ctx.analysis.scoping.is_store_ref(name) {
                *expr = rune_refs::make_thunk_call(ctx.alloc, name);
                return;
            }

            // Scope-first: all remaining classification via SymbolId
            let Some(sym_id) = ctx.analysis.scoping.find_binding(scope, name) else {
                return;
            };

            // Destructured const alias → $.get(tmp).prop
            if let Some(tag_id) = ctx.analysis.scoping.const_alias_tag(sym_id) {
                if let Some(tmp) = ctx.transform_data.const_tag_tmp_names.get(&tag_id) {
                    *expr = rune_refs::make_member_get(ctx.alloc, tmp, name);
                    return;
                }
            }

            let is_root = ctx.analysis.scoping.symbol_scope_id(sym_id)
                == ctx.analysis.scoping.root_scope_id();

            if !is_root {
                if ctx.analysis.scoping.is_arrow_param(sym_id) {
                    // Arrow function parameter — leave as-is
                } else if ctx.analysis.scoping.is_snippet_param(sym_id) {
                    *expr = rune_refs::make_thunk_call(ctx.alloc, name);
                } else {
                    *expr = rune_refs::make_rune_get(ctx.alloc, name);
                }
                return;
            }

            // Root scope classification
            if ctx.analysis.scoping.is_prop_source(sym_id) {
                *expr = rune_refs::make_thunk_call(ctx.alloc, name);
            } else if let Some(prop_name) = ctx.analysis.scoping.prop_non_source_name(sym_id) {
                let prop_name = prop_name.to_string();
                *expr = rune_refs::make_props_access(ctx.alloc, &prop_name);
            } else if let Some(kind) = ctx.analysis.scoping.rune_kind(sym_id) {
                let needs_get = ctx.analysis.scoping.is_mutated(sym_id) || kind.is_derived();
                if needs_get {
                    *expr = rune_refs::make_rune_get(ctx.alloc, name);
                }
            }
            return;
        }
        Expression::AssignmentExpression(assign) => {
            // First, transform the RHS
            transform_expr(ctx, &mut assign.right, scope);

            // Check if LHS is a rune identifier → $.set(name, rhs)
            if let AssignmentTarget::AssignmentTargetIdentifier(id) = &assign.left {
                let name = id.name.as_str();
                if let Some(sym_id) = ctx.analysis.scoping.find_binding(scope, name) {
                    if ctx.analysis.scoping.rune_kind(sym_id).is_some()
                        && ctx.analysis.scoping.is_mutated(sym_id)
                    {
                        let right = std::mem::replace(
                            &mut assign.right,
                            rune_refs::make_rune_get(ctx.alloc, ""),
                        );
                        *expr = rune_refs::make_rune_set(ctx.alloc, name, right, false);
                        return;
                    }
                }
            }
            return;
        }
        Expression::UpdateExpression(upd) => {
            if let SimpleAssignmentTarget::AssignmentTargetIdentifier(id) = &upd.argument {
                let name = id.name.as_str();
                if let Some(sym_id) = ctx.analysis.scoping.find_binding(scope, name) {
                    if ctx.analysis.scoping.rune_kind(sym_id).is_some()
                        && ctx.analysis.scoping.is_mutated(sym_id)
                    {
                        let is_increment =
                            upd.operator == oxc_syntax::operator::UpdateOperator::Increment;
                        *expr = rune_refs::make_rune_update(
                            ctx.alloc,
                            name,
                            upd.prefix,
                            is_increment,
                        );
                        return;
                    }
                }
            }
            return;
        }
        Expression::ArrowFunctionExpression(arrow) => {
            // Arrow params are registered in scoping — switch to arrow's scope
            let arrow_scope = ctx.analysis.scoping.arrow_scope(arrow.span.start)
                .unwrap_or(scope);
            for stmt in arrow.body.statements.iter_mut() {
                transform_stmt(ctx, stmt, arrow_scope);
            }
            return;
        }
        _ => {}
    }

    // Generic: walk child expressions
    walk_expr_children(ctx, expr, scope);
}

/// Walk into an expression's children and transform them.
fn walk_expr_children<'a>(
    ctx: &mut TransformCtx<'a, '_>,
    expr: &mut Expression<'a>,
    scope: ScopeId,
) {
    match expr {
        Expression::BinaryExpression(bin) => {
            transform_expr(ctx, &mut bin.left, scope);
            transform_expr(ctx, &mut bin.right, scope);
        }
        Expression::LogicalExpression(log) => {
            transform_expr(ctx, &mut log.left, scope);
            transform_expr(ctx, &mut log.right, scope);
        }
        Expression::UnaryExpression(un) => {
            transform_expr(ctx, &mut un.argument, scope);
        }
        Expression::ConditionalExpression(cond) => {
            transform_expr(ctx, &mut cond.test, scope);
            transform_expr(ctx, &mut cond.consequent, scope);
            transform_expr(ctx, &mut cond.alternate, scope);
        }
        Expression::CallExpression(call) => {
            transform_expr(ctx, &mut call.callee, scope);
            for arg in call.arguments.iter_mut() {
                if let Some(expr) = arg.as_expression_mut() {
                    transform_expr(ctx, expr, scope);
                }
            }
        }
        Expression::StaticMemberExpression(mem) => {
            transform_expr(ctx, &mut mem.object, scope);
        }
        Expression::ComputedMemberExpression(mem) => {
            transform_expr(ctx, &mut mem.object, scope);
            transform_expr(ctx, &mut mem.expression, scope);
        }
        Expression::TemplateLiteral(tl) => {
            for e in tl.expressions.iter_mut() {
                transform_expr(ctx, e, scope);
            }
        }
        Expression::ParenthesizedExpression(paren) => {
            transform_expr(ctx, &mut paren.expression, scope);
        }
        Expression::ArrayExpression(arr) => {
            for elem in arr.elements.iter_mut() {
                if let Some(expr) = elem.as_expression_mut() {
                    transform_expr(ctx, expr, scope);
                }
            }
        }
        Expression::ObjectExpression(obj) => {
            for prop in obj.properties.iter_mut() {
                match prop {
                    ObjectPropertyKind::ObjectProperty(p) => {
                        transform_expr(ctx, &mut p.value, scope);
                    }
                    ObjectPropertyKind::SpreadProperty(s) => {
                        transform_expr(ctx, &mut s.argument, scope);
                    }
                }
            }
        }
        Expression::SequenceExpression(seq) => {
            for e in seq.expressions.iter_mut() {
                transform_expr(ctx, e, scope);
            }
        }
        // Expressions already handled in transform_expr (Identifier, Assignment, Update, Arrow)
        // or leaf nodes that need no recursion:
        Expression::Identifier(_)
        | Expression::AssignmentExpression(_)
        | Expression::UpdateExpression(_)
        | Expression::ArrowFunctionExpression(_)
        | Expression::NumericLiteral(_)
        | Expression::StringLiteral(_)
        | Expression::BooleanLiteral(_)
        | Expression::NullLiteral(_)
        | Expression::FunctionExpression(_) => {}
        _ => {
            // Other expression types — no recursion needed for common Svelte patterns
        }
    }
}

/// Transform statements inside arrow function bodies.
fn transform_stmt<'a>(
    ctx: &mut TransformCtx<'a, '_>,
    stmt: &mut Statement<'a>,
    scope: ScopeId,
) {
    match stmt {
        Statement::ExpressionStatement(es) => {
            transform_expr(ctx, &mut es.expression, scope);
        }
        Statement::ReturnStatement(ret) => {
            if let Some(arg) = &mut ret.argument {
                transform_expr(ctx, arg, scope);
            }
        }
        Statement::BlockStatement(block) => {
            for s in block.body.iter_mut() {
                transform_stmt(ctx, s, scope);
            }
        }
        _ => {}
    }
}
