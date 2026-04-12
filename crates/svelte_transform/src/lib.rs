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
use svelte_analyze::RuneKind;
use svelte_analyze::{AnalysisData, ExprHandle, IdentGen, ParserResult};
use svelte_ast::{Attribute, Component, ConcatPart, Fragment, FragmentKey, Node};

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
    for &id in &fragment.nodes {
        let node = component.store.get(id);
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
            if let Some(handle) = parsed.expr_handle(tag.expression_span.start) {
                transform_expr_at(ctx, handle, parsed, scope);
            }
        }
        Node::Element(el) => {
            transform_attrs(ctx, &el.attributes, parsed, scope);
            walk_fragment(ctx, &el.fragment, component, parsed, scope);
        }
        Node::SlotElementLegacy(el) => {
            transform_attrs(ctx, &el.attributes, parsed, scope);
            walk_fragment(ctx, &el.fragment, component, parsed, scope);
        }
        Node::ComponentNode(cn) => {
            let component_has_slot_attr =
                attrs_static_slot_name(&cn.attributes, component.source.as_str()).is_some();
            let default_scope = if component_has_slot_attr {
                scope
            } else {
                ctx.analysis
                    .scoping
                    .fragment_scope(&FragmentKey::ComponentNode(cn.id))
                    .unwrap_or(scope)
            };

            for attr in &cn.attributes {
                match attr {
                    Attribute::LetDirectiveLegacy(_) => {
                        transform_attrs(ctx, std::slice::from_ref(attr), parsed, default_scope);
                    }
                    _ => transform_attrs(ctx, std::slice::from_ref(attr), parsed, scope),
                }
            }

            for &child_id in &cn.fragment.nodes {
                let child = component.store.get(child_id);
                let child_scope =
                    if node_static_slot_name(child, component.source.as_str()).is_some() {
                        ctx.analysis
                            .scoping
                            .fragment_scope(&FragmentKey::NamedSlot(cn.id, child_id))
                            .unwrap_or(scope)
                    } else {
                        default_scope
                    };
                walk_fragment(
                    ctx,
                    &svelte_ast::Fragment::new(vec![child_id]),
                    component,
                    parsed,
                    child_scope,
                );
            }
        }
        Node::IfBlock(block) => {
            if let Some(handle) = parsed.expr_handle(block.test_span.start) {
                transform_expr_at(ctx, handle, parsed, scope);
            }
            let cons_scope = ctx.analysis.if_consequent_scope(block.id, scope);
            walk_fragment(ctx, &block.consequent, component, parsed, cons_scope);
            if let Some(alt) = &block.alternate {
                let alt_scope = ctx.analysis.if_alternate_scope(block.id, scope);
                walk_fragment(ctx, alt, component, parsed, alt_scope);
            }
        }
        Node::EachBlock(block) => {
            if let Some(handle) = parsed.expr_handle(block.expression_span.start) {
                transform_expr_at(ctx, handle, parsed, scope);
            }

            // Each block body uses child scope (context + index vars)
            let body_scope = ctx.analysis.each_body_scope(block.id, scope);
            walk_fragment(ctx, &block.body, component, parsed, body_scope);

            // Fallback uses parent scope
            if let Some(fb) = &block.fallback {
                walk_fragment(ctx, fb, component, parsed, scope);
            }
        }
        Node::SnippetBlock(block) => {
            let snippet_scope = ctx.analysis.snippet_body_scope(block.id, scope);
            // Default expressions inside destructure params live on the
            // pre-parsed arrow stmt; without this, codegen's build_fallback_expr
            // would clone raw identifiers (e.g. `[counter]`) without $.get wrap.
            if let Some(handle) = ctx.analysis.snippet_stmt_handle(block.id) {
                if let Some(stmt) = parsed.stmt_mut(handle) {
                    transform_snippet_param_defaults(ctx, stmt, snippet_scope);
                }
            }
            walk_fragment(ctx, &block.body, component, parsed, snippet_scope);
        }
        Node::RenderTag(tag) => {
            if let Some(handle) = parsed.expr_handle(tag.expression_span.start) {
                transform_expr_at(ctx, handle, parsed, scope);
            }
        }
        Node::HtmlTag(tag) => {
            if let Some(handle) = parsed.expr_handle(tag.expression_span.start) {
                transform_expr_at(ctx, handle, parsed, scope);
            }
        }
        Node::ConstTag(tag) => {
            // Init expression lives in stmts (not exprs) — transform it in-place.
            if let Some(oxc_ast::ast::Statement::VariableDeclaration(decl)) = parsed
                .stmt_handle(tag.expression_span.start)
                .and_then(|handle| parsed.stmt_mut(handle))
            {
                if let Some(init) = decl.declarations.first_mut().and_then(|d| d.init.as_mut()) {
                    transform_expr(ctx, init, scope);
                }
            }

            let names = ctx
                .analysis
                .template
                .const_tags
                .names(tag.id)
                .cloned()
                .unwrap_or_default();
            if names.len() > 1 {
                let tmp = ctx.ident_gen.gen("computed_const");
                ctx.transform_data.const_tag_tmp_names.insert(tag.id, tmp);
            }
        }
        Node::KeyBlock(block) => {
            if let Some(handle) = parsed.expr_handle(block.expression_span.start) {
                transform_expr_at(ctx, handle, parsed, scope);
            }
            let child_scope = ctx.analysis.key_block_body_scope(block.id, scope);
            walk_fragment(ctx, &block.fragment, component, parsed, child_scope);
        }
        Node::SvelteHead(head) => {
            let child_scope = ctx.analysis.svelte_head_body_scope(head.id, scope);
            walk_fragment(ctx, &head.fragment, component, parsed, child_scope);
        }
        Node::SvelteFragmentLegacy(el) => {
            transform_attrs(ctx, &el.attributes, parsed, scope);
            walk_fragment(ctx, &el.fragment, component, parsed, scope);
        }
        Node::SvelteElement(el) => {
            if !el.static_tag {
                if let Some(handle) = parsed.expr_handle(el.tag_span.start) {
                    transform_expr_at(ctx, handle, parsed, scope);
                }
            }
            transform_attrs(ctx, &el.attributes, parsed, scope);
            let child_scope = ctx.analysis.svelte_element_body_scope(el.id, scope);
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
            let child_scope = ctx.analysis.svelte_boundary_body_scope(b.id, scope);
            walk_fragment(ctx, &b.fragment, component, parsed, child_scope);
        }
        Node::AwaitBlock(block) => {
            if let Some(handle) = parsed.expr_handle(block.expression_span.start) {
                transform_expr_at(ctx, handle, parsed, scope);
            }
            if let Some(ref p) = block.pending {
                let pending_scope = ctx.analysis.await_pending_scope(block.id, scope);
                walk_fragment(ctx, p, component, parsed, pending_scope);
            }
            if let Some(ref t) = block.then {
                let then_scope = ctx.analysis.await_then_scope(block.id, scope);
                walk_fragment(ctx, t, component, parsed, then_scope);
            }
            if let Some(ref c) = block.catch {
                let catch_scope = ctx.analysis.await_catch_scope(block.id, scope);
                walk_fragment(ctx, c, component, parsed, catch_scope);
            }
        }
        Node::DebugTag(tag) => {
            for span in &tag.identifiers {
                if let Some(handle) = parsed.expr_handle(span.start) {
                    transform_expr_at(ctx, handle, parsed, scope);
                }
            }
        }
        Node::Text(_) | Node::Comment(_) | Node::Error(_) => {}
    }
}

/// Transform a single expression stored by handle.
fn transform_expr_at<'a>(
    ctx: &mut TransformCtx<'a, '_>,
    handle: ExprHandle,
    parsed: &mut ParserResult<'a>,
    scope: ScopeId,
) {
    if let Some(expr) = parsed.expr_mut(handle) {
        transform_expr(ctx, expr, scope);
    }
}

/// Rewrite identifier reads inside snippet parameter defaults so reactive
/// `$state` bindings become `$.get(name)` — matches the reference compiler's
/// `context.visit(path.expression, child_state)` in SnippetBlock.js.
fn transform_snippet_param_defaults<'a>(
    ctx: &mut TransformCtx<'a, '_>,
    stmt: &mut Statement<'a>,
    scope: ScopeId,
) {
    let Statement::VariableDeclaration(decl) = stmt else {
        return;
    };
    let Some(declarator) = decl.declarations.first_mut() else {
        return;
    };
    let Some(Expression::ArrowFunctionExpression(arrow)) = declarator.init.as_mut() else {
        return;
    };

    let mut visitor = ExprTransformer { ctx, scope };
    for param in arrow.params.items.iter_mut() {
        // Default VisitMut walk descends BindingPattern → AssignmentPattern.right
        // as Expression and recurses through nested object/array patterns, so
        // every default expression in nested destructuring is reached.
        visitor.visit_binding_pattern(&mut param.pattern);
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
        let expr_handle = get_attr_expr_handle(parsed, attr);
        if let Some(handle) = expr_handle {
            transform_expr_at(ctx, handle, parsed, scope);
        }

        // Transform directive name expressions (use:, transition:, animate:)
        let name_handle = get_directive_name_handle(parsed, attr);
        if let Some(handle) = name_handle {
            transform_expr_at(ctx, handle, parsed, scope);
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
                if let ConcatPart::Dynamic { span, .. } = part {
                    if let Some(handle) = parsed.expr_handle(span.start) {
                        transform_expr_at(ctx, handle, parsed, scope);
                    }
                }
            }
        }
    }
}

/// Get the handle for an attribute's primary expression.
fn get_attr_expr_handle(parsed: &ParserResult<'_>, attr: &Attribute) -> Option<ExprHandle> {
    let offset = match attr {
        Attribute::ExpressionAttribute(a) => Some(a.expression_span.start),
        Attribute::ClassDirective(a) => a.expression_span.map(|s| s.start),
        Attribute::StyleDirective(a) => match &a.value {
            svelte_ast::StyleDirectiveValue::Expression(span) => Some(span.start),
            _ => None,
        },
        Attribute::BindDirective(a) => a.expression_span.map(|s| s.start),
        Attribute::LetDirectiveLegacy(a) => a.expression_span.map(|s| s.start),
        Attribute::SpreadAttribute(a) => Some(a.expression_span.start),
        Attribute::Shorthand(a) => Some(a.expression_span.start),
        Attribute::UseDirective(a) => a.expression_span.map(|s| s.start),
        Attribute::OnDirectiveLegacy(a) => a.expression_span.map(|s| s.start),
        Attribute::TransitionDirective(a) => a.expression_span.map(|s| s.start),
        Attribute::AnimateDirective(a) => a.expression_span.map(|s| s.start),
        Attribute::AttachTag(a) => Some(a.expression_span.start),
        Attribute::StringAttribute(_)
        | Attribute::BooleanAttribute(_)
        | Attribute::ConcatenationAttribute(_) => None,
    }?;
    parsed.expr_handle(offset)
}

/// Get the handle for a directive's name expression (use:, transition:, animate:).
fn get_directive_name_handle(parsed: &ParserResult<'_>, attr: &Attribute) -> Option<ExprHandle> {
    let offset = match attr {
        Attribute::UseDirective(a) => Some(a.name.start),
        Attribute::TransitionDirective(a) => Some(a.name.start),
        Attribute::AnimateDirective(a) => Some(a.name.start),
        _ => None,
    }?;
    parsed.expr_handle(offset)
}

fn attrs_static_slot_name<'a>(attrs: &'a [Attribute], source: &'a str) -> Option<&'a str> {
    attrs.iter().find_map(|attr| match attr {
        Attribute::StringAttribute(attr) if attr.name.as_str() == "slot" => {
            Some(&source[attr.value_span.start as usize..attr.value_span.end as usize])
        }
        _ => None,
    })
}

fn node_static_slot_name<'a>(node: &'a Node, source: &'a str) -> Option<&'a str> {
    match node {
        Node::Element(node) => attrs_static_slot_name(&node.attributes, source),
        Node::SvelteFragmentLegacy(node) => attrs_static_slot_name(&node.attributes, source),
        Node::ComponentNode(node) => attrs_static_slot_name(&node.attributes, source),
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
fn transform_expr<'a>(ctx: &mut TransformCtx<'a, '_>, expr: &mut Expression<'a>, scope: ScopeId) {
    let mut visitor = ExprTransformer { ctx, scope };
    visitor.visit_expression(expr);
}

struct ExprTransformer<'a, 'b, 'c> {
    ctx: &'c mut TransformCtx<'a, 'b>,
    scope: ScopeId,
}

impl<'a> VisitMut<'a> for ExprTransformer<'a, '_, '_> {
    fn visit_expression(&mut self, it: &mut Expression<'a>) {
        // Rest prop member access: props.label → $$props.label
        // Resolved via reference_id (set by TemplateSemanticVisitor during analysis)
        if let Expression::StaticMemberExpression(member) = it {
            if let Expression::Identifier(id) = &member.object {
                let sym_id = id
                    .reference_id
                    .get()
                    .and_then(|ref_id| self.ctx.analysis.scoping.get_reference(ref_id).symbol_id());
                if let Some(sym_id) = sym_id {
                    if self.ctx.analysis.scoping.is_rest_prop(sym_id)
                        && !self
                            .ctx
                            .analysis
                            .scoping
                            .is_rest_prop_excluded(&member.property.name)
                    {
                        let Expression::StaticMemberExpression(member) = it else {
                            unreachable!()
                        };
                        let ast = oxc_ast::AstBuilder::new(self.ctx.alloc);
                        member.object =
                            ast.expression_identifier(oxc_span::SPAN, ast.atom("$$props"));
                        walk_expression(self, it);
                        return;
                    }
                }
            }
        }

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

            // Module-scope symbols (from <script module>) must be treated the same
            // as root-scope symbols — not wrapped in $.get().
            let is_root = self
                .ctx
                .analysis
                .scoping
                .is_component_top_level_symbol(sym_id);

            match self.ctx.analysis.scoping.template_binding_read_kind(sym_id) {
                svelte_analyze::TemplateBindingReadKind::Identifier => {
                    if !is_root {
                        // Arrow function parameter, template snippet name, or
                        // each-block var with key_is_item optimization — leave as-is
                    }
                }
                svelte_analyze::TemplateBindingReadKind::ThunkCall => {
                    *it = rune_refs::make_thunk_call(self.ctx.alloc, name);
                }
                svelte_analyze::TemplateBindingReadKind::RuneGet => {
                    *it = rune_refs::make_rune_get(self.ctx.alloc, name);
                }
                svelte_analyze::TemplateBindingReadKind::RuneSafeGet => {
                    *it = rune_refs::make_rune_safe_get(self.ctx.alloc, name);
                }
                svelte_analyze::TemplateBindingReadKind::SlotLetCarrierMember => {
                    let carrier_sym_id = self
                        .ctx
                        .analysis
                        .scoping
                        .slot_let_binding_carrier(sym_id)
                        .unwrap_or_else(|| {
                            panic!("slot let binding {:?} missing carrier symbol", sym_id)
                        });
                    let carrier_name = self.ctx.analysis.scoping.symbol_name(carrier_sym_id);
                    *it = rune_refs::make_member_get(self.ctx.alloc, carrier_name, name);
                }
                svelte_analyze::TemplateBindingReadKind::PropsAccess => {
                    let prop_name = self
                        .ctx
                        .analysis
                        .scoping
                        .prop_non_source_name(sym_id)
                        .unwrap_or_else(|| panic!("PropsAccess read kind missing prop name"));
                    *it = rune_refs::make_props_access(self.ctx.alloc, prop_name);
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
                        self.ctx.alloc,
                        base_name,
                        name,
                        upd.prefix,
                        is_increment,
                    );
                    return;
                }

                // Rune update
                if let Some(sym_id) = self.ctx.analysis.scoping.find_binding(self.scope, name) {
                    if self.ctx.analysis.scoping.rune_kind(sym_id).is_some()
                        && self.ctx.analysis.scoping.is_mutated(sym_id)
                    {
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

            // Deep store update: $store.count++ → $.store_mutate(store, ...)
            if let Some(root_name) =
                extract_member_root_store_simple(&upd.argument, &self.ctx.analysis.scoping)
            {
                let root_name = root_name.to_string();
                let base_name = root_name[1..].to_string();
                let alloc = self.ctx.alloc;
                walk_simple_target_member_objects(self, &mut upd.argument);
                rune_refs::replace_expr_root_in_simple_target(
                    &mut upd.argument,
                    rune_refs::make_untrack(alloc, &root_name),
                );
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
            if let Some(root_name) =
                extract_member_root_store_assign(&assign.left, &self.ctx.analysis.scoping)
            {
                let root_name = root_name.to_string();
                let base_name = root_name[1..].to_string();
                let alloc = self.ctx.alloc;
                self.visit_expression(&mut assign.right);
                walk_target_member_objects(self, &mut assign.left);
                rune_refs::replace_expr_root_in_assign_target(
                    &mut assign.left,
                    rune_refs::make_untrack(alloc, &root_name),
                );
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
                            if let Expression::CallExpression(call) =
                                std::mem::replace(it, rune_refs::make_eager_pending(self.ctx.alloc))
                            {
                                let mut call = call.unbox();
                                if !call.arguments.is_empty() {
                                    let arg = call.arguments.remove(0).into_expression();
                                    *it = rune_refs::make_eager_thunk(self.ctx.alloc, arg);
                                }
                            }
                            return;
                        }
                        ("$state", "snapshot") => {
                            // Walk arguments for nested rune refs
                            for arg in call.arguments.iter_mut() {
                                if let Some(expr) = arg.as_expression_mut() {
                                    self.visit_expression(expr);
                                }
                            }
                            // Replace callee: $state.snapshot → $.snapshot
                            let Expression::CallExpression(call) = it else {
                                unreachable!()
                            };
                            let ast = oxc_ast::AstBuilder::new(self.ctx.alloc);
                            call.callee = rune_refs::make_dollar_member(&ast, "snapshot");
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
                    let right =
                        std::mem::replace(&mut assign.right, rune_refs::make_rune_get(alloc, ""));
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
                            let needs_proxy =
                                kind != RuneKind::StateRaw && rune_refs::should_proxy(&value);
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
