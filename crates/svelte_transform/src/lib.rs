//! AST-to-AST transformation pass for Svelte expressions.
//!
//! Transforms rune references, prop accesses, and each-block variables
//! in pre-parsed Expression ASTs. Runs between analyze and codegen.

mod data;
pub mod rune_refs;

pub use data::TransformData;

use oxc_allocator::Allocator;
use oxc_ast::ast::*;
use oxc_ast_visit::walk_mut::{walk_arrow_function_expression, walk_expression};
use oxc_ast_visit::VisitMut;

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
// Expression transformer (VisitMut-based)
// ---------------------------------------------------------------------------

/// Transform an expression tree in-place using OXC's VisitMut.
///
/// Arrow function parameters are registered as scoped bindings during analysis
/// (`register_arrow_scopes`), so `find_binding` naturally resolves them — no
/// separate shadow stack needed.
fn transform_expr<'a>(
    ctx: &mut TransformCtx<'a, '_>,
    expr: &mut Expression<'a>,
    scope: ScopeId,
) {
    let mut visitor = ExprTransformer { ctx, scope };
    visitor.visit_expression(expr);
}

struct ExprTransformer<'a, 'b, 'c> {
    ctx: &'c mut TransformCtx<'a, 'b>,
    scope: ScopeId,
}

impl<'a> VisitMut<'a> for ExprTransformer<'a, '_, '_> {
    fn visit_expression(&mut self, it: &mut Expression<'a>) {
        // Identifier: leaf node, handle directly
        if let Expression::Identifier(id) = it {
            let name = id.name.as_str();

            if self.ctx.analysis.scoping.is_store_ref(name) {
                *it = rune_refs::make_thunk_call(self.ctx.alloc, name);
                return;
            }

            let Some(sym_id) = self.ctx.analysis.scoping.find_binding(self.scope, name) else {
                return;
            };

            if let Some(tag_id) = self.ctx.analysis.scoping.const_alias_tag(sym_id) {
                if let Some(tmp) = self.ctx.transform_data.const_tag_tmp_names.get(&tag_id) {
                    *it = rune_refs::make_member_get(self.ctx.alloc, tmp, name);
                    return;
                }
            }

            let is_root = self.ctx.analysis.scoping.symbol_scope_id(sym_id)
                == self.ctx.analysis.scoping.root_scope_id();

            if !is_root {
                if self.ctx.analysis.scoping.is_arrow_param(sym_id) {
                    // Arrow function parameter — leave as-is
                } else if self.ctx.analysis.scoping.is_snippet_param(sym_id) {
                    *it = rune_refs::make_thunk_call(self.ctx.alloc, name);
                } else {
                    *it = rune_refs::make_rune_get(self.ctx.alloc, name);
                }
                return;
            }

            if self.ctx.analysis.scoping.is_prop_source(sym_id) {
                *it = rune_refs::make_thunk_call(self.ctx.alloc, name);
            } else if let Some(prop_name) = self.ctx.analysis.scoping.prop_non_source_name(sym_id) {
                let prop_name = prop_name.to_string();
                *it = rune_refs::make_props_access(self.ctx.alloc, &prop_name);
            } else if let Some(kind) = self.ctx.analysis.scoping.rune_kind(sym_id) {
                let needs_get = self.ctx.analysis.scoping.is_mutated(sym_id) || kind.is_derived();
                if needs_get {
                    *it = rune_refs::make_rune_get(self.ctx.alloc, name);
                }
            }
            return;
        }

        // UpdateExpression: leaf-like, handle before walk
        if let Expression::UpdateExpression(upd) = it {
            if let SimpleAssignmentTarget::AssignmentTargetIdentifier(id) = &upd.argument {
                let name = id.name.as_str();
                if let Some(sym_id) = self.ctx.analysis.scoping.find_binding(self.scope, name) {
                    if self.ctx.analysis.scoping.rune_kind(sym_id).is_some()
                        && self.ctx.analysis.scoping.is_mutated(sym_id)
                    {
                        let is_increment =
                            upd.operator == oxc_syntax::operator::UpdateOperator::Increment;
                        *it = rune_refs::make_rune_update(
                            self.ctx.alloc,
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

        // Walk all children (covers every expression type automatically)
        walk_expression(self, it);

        // Post-walk: AssignmentExpression LHS rune → $.set()
        if let Expression::AssignmentExpression(assign) = it {
            if let AssignmentTarget::AssignmentTargetIdentifier(id) = &assign.left {
                let name = id.name.as_str();
                if let Some(sym_id) = self.ctx.analysis.scoping.find_binding(self.scope, name) {
                    if self.ctx.analysis.scoping.rune_kind(sym_id).is_some()
                        && self.ctx.analysis.scoping.is_mutated(sym_id)
                    {
                        let right = std::mem::replace(
                            &mut assign.right,
                            rune_refs::make_rune_get(self.ctx.alloc, ""),
                        );
                        *it = rune_refs::make_rune_set(self.ctx.alloc, name, right, false);
                    }
                }
            }
        }
    }

    fn visit_arrow_function_expression(&mut self, it: &mut ArrowFunctionExpression<'a>) {
        let parent_scope = self.scope;
        self.scope = self.ctx.analysis.scoping.arrow_scope(it.span.start)
            .unwrap_or(self.scope);
        walk_arrow_function_expression(self, it);
        self.scope = parent_scope;
    }
}
