//! AST-to-AST transformation pass for Svelte expressions.
//!
//! Transforms rune references, prop accesses, and each-block variables
//! in pre-parsed Expression ASTs. Runs between analyze and codegen.

mod data;
pub mod rune_refs;

pub use data::TransformData;

use oxc_allocator::Allocator;
use oxc_ast::ast::*;
use oxc_ast_visit::walk_mut::{self, walk_arrow_function_expression, walk_expression};
use oxc_ast_visit::VisitMut;

use svelte_analyze::scope::ScopeId;
use svelte_analyze::{AnalysisData, FragmentKey, IdentGen, ParserResult};
use svelte_ast::{
    Attribute, Component, ConcatPart, Fragment, Node,
};
use svelte_analyze::RuneKind;


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
    parsed: &mut ParserResult<'a>,
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
    parsed: &mut ParserResult<'a>,
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
    parsed: &mut ParserResult<'a>,
    scope: ScopeId,
) {
    match node {
        Node::ExpressionTag(tag) => {
            transform_expr_at(ctx, tag.expression_span.start, parsed, scope);
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
            transform_expr_at(ctx, block.test_span.start, parsed, scope);
            let cons_scope = ctx.analysis.scoping.fragment_scope(&FragmentKey::IfConsequent(block.id)).unwrap_or(scope);
            walk_fragment(ctx, &block.consequent, component, parsed, cons_scope);
            if let Some(alt) = &block.alternate {
                let alt_scope = ctx.analysis.scoping.fragment_scope(&FragmentKey::IfAlternate(block.id)).unwrap_or(scope);
                walk_fragment(ctx, alt, component, parsed, alt_scope);
            }
        }
        Node::EachBlock(block) => {
            transform_expr_at(ctx, block.expression_span.start, parsed, scope);

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
            transform_expr_at(ctx, tag.expression_span.start, parsed, scope);
        }
        Node::HtmlTag(tag) => {
            transform_expr_at(ctx, tag.expression_span.start, parsed, scope);
        }
        Node::ConstTag(tag) => {
            // Init expression lives in stmts (not exprs) — transform it in-place.
            let offset = tag.expression_span.start;
            if let Some(oxc_ast::ast::Statement::VariableDeclaration(decl)) = parsed.stmts.get_mut(&offset) {
                if let Some(init) = decl.declarations.first_mut().and_then(|d| d.init.as_mut()) {
                    transform_expr(ctx, init, scope);
                }
            }

            let names = ctx.analysis.const_tags.names(tag.id).cloned().unwrap_or_default();
            if names.len() > 1 {
                let tmp = ctx.ident_gen.gen("computed_const");
                ctx.transform_data.const_tag_tmp_names.insert(tag.id, tmp);
            }
        }
        Node::KeyBlock(block) => {
            transform_expr_at(ctx, block.expression_span.start, parsed, scope);
            let child_scope = ctx.analysis.scoping.fragment_scope(&FragmentKey::KeyBlockBody(block.id)).unwrap_or(scope);
            walk_fragment(ctx, &block.fragment, component, parsed, child_scope);
        }
        Node::SvelteHead(head) => {
            let child_scope = ctx.analysis.scoping.fragment_scope(&FragmentKey::SvelteHeadBody(head.id)).unwrap_or(scope);
            walk_fragment(ctx, &head.fragment, component, parsed, child_scope);
        }
        Node::SvelteElement(el) => {
            if !el.static_tag {
                transform_expr_at(ctx, el.tag_span.start, parsed, scope);
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
            transform_expr_at(ctx, block.expression_span.start, parsed, scope);
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
        Node::DebugTag(tag) => {
            for span in &tag.identifiers {
                transform_expr_at(ctx, span.start, parsed, scope);
            }
        }
        Node::Text(_) | Node::Comment(_) | Node::Error(_) => {}
    }
}

/// Transform a single expression stored by offset.
fn transform_expr_at<'a>(
    ctx: &mut TransformCtx<'a, '_>,
    offset: u32,
    parsed: &mut ParserResult<'a>,
    scope: ScopeId,
) {
    if let Some(expr) = parsed.exprs.get_mut(&offset) {
        transform_expr(ctx, expr, scope);
    }
}

/// Transform attribute expressions.
fn transform_attrs<'a>(
    ctx: &mut TransformCtx<'a, '_>,
    attrs: &[Attribute],
    parsed: &mut ParserResult<'a>,
    scope: ScopeId,
) {
    for attr in attrs {
        // Transform the main attribute expression
        let expr_offset = get_attr_expr_offset(attr);
        if let Some(offset) = expr_offset {
            transform_expr_at(ctx, offset, parsed, scope);
        }

        // Transform directive name expressions (use:, transition:, animate:)
        let name_offset = get_directive_name_offset(attr);
        if let Some(offset) = name_offset {
            transform_expr_at(ctx, offset, parsed, scope);
        }

        // Transform concat dynamic parts
        let concat_parts: Option<&[ConcatPart]> = match attr {
            Attribute::ConcatenationAttribute(a) => Some(&a.parts),
            Attribute::StyleDirective(a) => match &a.value {
                svelte_ast::StyleDirectiveValue::Concatenation(parts) => Some(parts),
                _ => None,
            },
            _ => None,
        };
        if let Some(parts) = concat_parts {
            for part in parts {
                if let ConcatPart::Dynamic(span) = part {
                    transform_expr_at(ctx, span.start, parsed, scope);
                }
            }
        }
    }
}

/// Get the offset for an attribute's primary expression.
fn get_attr_expr_offset(attr: &Attribute) -> Option<u32> {
    match attr {
        Attribute::ExpressionAttribute(a) => Some(a.expression_span.start),
        Attribute::ClassDirective(a) => a.expression_span.map(|s| s.start),
        Attribute::StyleDirective(a) => match &a.value {
            svelte_ast::StyleDirectiveValue::Expression(span) => Some(span.start),
            _ => None,
        },
        Attribute::BindDirective(a) => a.expression_span.map(|s| s.start),
        Attribute::SpreadAttribute(a) => Some(a.expression_span.start),
        Attribute::Shorthand(a) => Some(a.expression_span.start),
        Attribute::UseDirective(a) => a.expression_span.map(|s| s.start),
        Attribute::OnDirectiveLegacy(a) => a.expression_span.map(|s| s.start),
        Attribute::TransitionDirective(a) => a.expression_span.map(|s| s.start),
        Attribute::AnimateDirective(a) => a.expression_span.map(|s| s.start),
        Attribute::AttachTag(a) => Some(a.expression_span.start),
        Attribute::StringAttribute(_) | Attribute::BooleanAttribute(_)
        | Attribute::ConcatenationAttribute(_) => None,
    }
}

/// Get the offset for a directive's name expression (use:, transition:, animate:).
fn get_directive_name_offset(attr: &Attribute) -> Option<u32> {
    match attr {
        Attribute::UseDirective(a) => Some(a.name.start),
        Attribute::TransitionDirective(a) => Some(a.name.start),
        Attribute::AnimateDirective(a) => Some(a.name.start),
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// Expression transformer (VisitMut-based)
// ---------------------------------------------------------------------------

/// Transform an expression tree in-place using OXC's VisitMut.
///
/// JS scopes (arrows, functions, for-loops, blocks, catch) are registered
/// during analysis, so `find_binding` naturally resolves local bindings — no
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
                if self.ctx.analysis.scoping.is_expr_local(sym_id)
                    || self.ctx.analysis.scoping.is_snippet_name(sym_id)
                    || self.ctx.analysis.scoping.is_each_non_reactive(sym_id)
                {
                    // Arrow function parameter, template snippet name, or
                    // each-block var with key_is_item optimization — leave as-is
                } else if self.ctx.analysis.scoping.is_getter(sym_id) {
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

        // --- Pre-walk: UpdateExpression ---
        if let Expression::UpdateExpression(upd) = it {
            let is_increment = upd.operator == oxc_syntax::operator::UpdateOperator::Increment;

            // Identifier target: rune or store update
            if let SimpleAssignmentTarget::AssignmentTargetIdentifier(id) = &upd.argument {
                let name = id.name.as_str();

                // Store update: $count++ → $.update_store(count, $count())
                if self.ctx.analysis.scoping.is_store_ref(name) {
                    let base_name = &name[1..];
                    *it = rune_refs::make_store_update(
                        self.ctx.alloc, base_name, name, upd.prefix, is_increment,
                    );
                    return;
                }

                // Rune update
                if let Some(sym_id) = self.ctx.analysis.scoping.find_binding(self.scope, name) {
                    if self.ctx.analysis.scoping.rune_kind(sym_id).is_some()
                        && self.ctx.analysis.scoping.is_mutated(sym_id)
                    {
                        *it = rune_refs::make_rune_update(
                            self.ctx.alloc, name, upd.prefix, is_increment,
                        );
                        return;
                    }
                }
            }

            // Deep store update: $store.count++ → $.store_mutate(store, ...)
            if let Some(root_name) = extract_member_root_store_simple(&upd.argument, &self.ctx.analysis.scoping) {
                let root_name = root_name.to_string();
                let base_name = root_name[1..].to_string();
                let alloc = self.ctx.alloc;
                walk_simple_target_member_objects(self, &mut upd.argument);
                rune_refs::replace_expr_root_in_simple_target(&mut upd.argument, rune_refs::make_untrack(alloc, &root_name));
                // Swap out the entire UpdateExpression to break the borrow on `upd`
                let placeholder = rune_refs::make_rune_get(alloc, "");
                let mutation = std::mem::replace(it, placeholder);
                let untracked = rune_refs::make_untrack(alloc, &root_name);
                *it = rune_refs::make_store_mutate(alloc, &base_name, mutation, untracked);
                return;
            }

            return;
        }

        // --- Pre-walk: deep store assignment ($store.field = val) ---
        if let Expression::AssignmentExpression(assign) = it {
            if let Some(root_name) = extract_member_root_store_assign(&assign.left, &self.ctx.analysis.scoping) {
                let root_name = root_name.to_string();
                let base_name = root_name[1..].to_string();
                let alloc = self.ctx.alloc;
                self.visit_expression(&mut assign.right);
                walk_target_member_objects(self, &mut assign.left);
                rune_refs::replace_expr_root_in_assign_target(&mut assign.left, rune_refs::make_untrack(alloc, &root_name));
                // Swap out the entire AssignmentExpression to break the borrow on `assign`
                let placeholder = rune_refs::make_rune_get(alloc, "");
                let mutation = std::mem::replace(it, placeholder);
                let untracked = rune_refs::make_untrack(alloc, &root_name);
                *it = rune_refs::make_store_mutate(alloc, &base_name, mutation, untracked);
                return;
            }
        }

        // CallExpression: $state.eager(val) → $.eager(() => val), $effect.pending() → $.eager($.pending)
        if let Expression::CallExpression(call) = it {
            if let Expression::StaticMemberExpression(member) = &call.callee {
                if let Expression::Identifier(obj) = &member.object {
                    match (obj.name.as_str(), member.property.name.as_str()) {
                        ("$state", "eager") => {
                            // Walk the argument first so inner rune refs are transformed
                            for arg in call.arguments.iter_mut() {
                                if let Some(expr) = arg.as_expression_mut() {
                                    self.visit_expression(expr);
                                }
                            }
                            // Replace: $state.eager(val) → $.eager(() => val)
                            if let Expression::CallExpression(call) = std::mem::replace(it, rune_refs::make_eager_pending(self.ctx.alloc)) {
                                let mut call = call.unbox();
                                if !call.arguments.is_empty() {
                                    let arg = call.arguments.remove(0).into_expression();
                                    *it = rune_refs::make_eager_thunk(self.ctx.alloc, arg);
                                }
                            }
                            return;
                        }
                        ("$effect", "pending") => {
                            *it = rune_refs::make_eager_pending(self.ctx.alloc);
                            return;
                        }
                        _ => {}
                    }
                }
            }
        }

        // Walk all children (covers every expression type automatically)
        walk_expression(self, it);

        // --- Post-walk: AssignmentExpression with identifier LHS ---
        if let Expression::AssignmentExpression(assign) = it {
            if let AssignmentTarget::AssignmentTargetIdentifier(id) = &assign.left {
                let name = id.name.as_str();
                let alloc = self.ctx.alloc;
                let operator = assign.operator;

                // Store assignment: $count = 5 → $.store_set(count, 5)
                // Store compound: $count += 1 → $.store_set(count, $count() + 1)
                if self.ctx.analysis.scoping.is_store_ref(name) {
                    let base_name = &name[1..];
                    let right = std::mem::replace(
                        &mut assign.right,
                        rune_refs::make_rune_get(alloc, ""),
                    );
                    let value = if operator.is_assign() {
                        right
                    } else {
                        let current = rune_refs::make_thunk_call(alloc, name);
                        rune_refs::build_compound_value(alloc, operator, current, right)
                    };
                    *it = rune_refs::make_store_set(alloc, base_name, value);
                    return;
                }

                // Rune assignment/compound
                if let Some(sym_id) = self.ctx.analysis.scoping.find_binding(self.scope, name) {
                    if let Some(kind) = self.ctx.analysis.scoping.rune_kind(sym_id) {
                        if self.ctx.analysis.scoping.is_mutated(sym_id) {
                            let right = std::mem::replace(
                                &mut assign.right,
                                rune_refs::make_rune_get(alloc, ""),
                            );
                            let value = if operator.is_assign() {
                                right
                            } else {
                                let left_read = rune_refs::make_rune_get(alloc, name);
                                rune_refs::build_compound_value(alloc, operator, left_read, right)
                            };
                            let needs_proxy = kind != RuneKind::StateRaw
                                && rune_refs::should_proxy(&value);
                            *it = rune_refs::make_rune_set(alloc, name, value, needs_proxy);
                        }
                    }
                }
            }
        }
    }

    fn visit_arrow_function_expression(&mut self, it: &mut ArrowFunctionExpression<'a>) {
        let parent_scope = self.scope;
        self.scope = it.scope_id.get().unwrap_or(self.scope);
        walk_arrow_function_expression(self, it);
        self.scope = parent_scope;
    }

    fn visit_function(&mut self, it: &mut Function<'a>, flags: oxc_syntax::scope::ScopeFlags) {
        let parent_scope = self.scope;
        self.scope = it.scope_id.get().unwrap_or(self.scope);
        walk_mut::walk_function(self, it, flags);
        self.scope = parent_scope;
    }
}

// ---------------------------------------------------------------------------
// Deep store mutation helpers
// ---------------------------------------------------------------------------

use svelte_analyze::scope::ComponentScoping;

/// Check if an AssignmentTarget is a member expression with a store ref root.
fn extract_member_root_store_assign<'b>(
    target: &'b AssignmentTarget<'_>,
    scoping: &ComponentScoping,
) -> Option<&'b str> {
    let name = rune_refs::find_expr_root_name(target.as_member_expression()?.object())?;
    scoping.is_store_ref(name).then_some(name)
}

/// Check if a SimpleAssignmentTarget is a member expression with a store ref root.
fn extract_member_root_store_simple<'b>(
    target: &'b SimpleAssignmentTarget<'_>,
    scoping: &ComponentScoping,
) -> Option<&'b str> {
    let name = rune_refs::find_expr_root_name(target.as_member_expression()?.object())?;
    scoping.is_store_ref(name).then_some(name)
}

/// Walk and transform computed member expression objects in an AssignmentTarget chain,
/// skipping the root identifier.
fn walk_target_member_objects<'a>(
    visitor: &mut ExprTransformer<'a, '_, '_>,
    target: &mut AssignmentTarget<'a>,
) {
    // Handle computed expression at the target level
    if let Some(MemberExpression::ComputedMemberExpression(m)) = target.as_member_expression_mut() {
        visitor.visit_expression(&mut m.expression);
    }
    if let Some(member) = target.as_member_expression_mut() {
        walk_expr_member_objects(visitor, member.object_mut());
    }
}

/// Walk and transform computed member expression objects in a SimpleAssignmentTarget chain.
fn walk_simple_target_member_objects<'a>(
    visitor: &mut ExprTransformer<'a, '_, '_>,
    target: &mut SimpleAssignmentTarget<'a>,
) {
    if let Some(MemberExpression::ComputedMemberExpression(m)) = target.as_member_expression_mut() {
        visitor.visit_expression(&mut m.expression);
    }
    if let Some(member) = target.as_member_expression_mut() {
        walk_expr_member_objects(visitor, member.object_mut());
    }
}

/// Walk member chain objects (Expression side), transforming computed
/// properties but NOT the root identifier.
fn walk_expr_member_objects<'a>(
    visitor: &mut ExprTransformer<'a, '_, '_>,
    expr: &mut Expression<'a>,
) {
    let mut current = expr;
    while let Some(member) = current.as_member_expression_mut() {
        if let MemberExpression::ComputedMemberExpression(m) = member {
            visitor.visit_expression(&mut m.expression);
        }
        current = member.object_mut();
    }
}

