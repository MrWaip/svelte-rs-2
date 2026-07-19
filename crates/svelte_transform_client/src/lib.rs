mod data;
pub mod rune_refs;
pub mod transformer;

pub use data::{RestExcludeKey, RestExcludes, TransformData};

pub use transformer::{IgnoreQuery, TransformScriptOutput, sanitize_location, transform_script};

use std::slice;

use oxc_syntax::node::NodeId as OxcNodeId;
use svelte_component_semantics::OxcNodeId as SemOxcNodeId;

use oxc_ast::ast::{Expression, Statement};
use svelte_analyze::scope::ScopeId;
use svelte_analyze::{
    AnalysisData, AttributeSemantics, BlockSemantics, EachItemKind, HtmlBindKind, IdentGen, JsAst,
};
use svelte_ast::{
    Attribute, Component, ConcatPart, EachBlock, ExprRef, FragmentId, LegacySlot, Node,
    NodeId as SvelteNodeId, StyleDirectiveValue,
};
use svelte_component_semantics::walk_bindings;

pub(crate) fn is_simple_expression(expr: &Expression<'_>) -> bool {
    match expr {
        Expression::NullLiteral(_)
        | Expression::BooleanLiteral(_)
        | Expression::NumericLiteral(_)
        | Expression::StringLiteral(_)
        | Expression::BigIntLiteral(_)
        | Expression::RegExpLiteral(_)
        | Expression::Identifier(_)
        | Expression::ArrowFunctionExpression(_)
        | Expression::FunctionExpression(_) => true,
        Expression::ParenthesizedExpression(inner) => is_simple_expression(&inner.expression),
        Expression::ConditionalExpression(cond) => {
            is_simple_expression(&cond.test)
                && is_simple_expression(&cond.consequent)
                && is_simple_expression(&cond.alternate)
        }
        Expression::BinaryExpression(bin) => {
            is_simple_expression(&bin.left) && is_simple_expression(&bin.right)
        }
        Expression::LogicalExpression(log) => {
            is_simple_expression(&log.left) && is_simple_expression(&log.right)
        }
        _ => false,
    }
}

pub fn transform_component<'a>(
    ctx: &mut svelte_types::CompileContext<'a, '_>,
    options: &svelte_types::TransformOptions,
) -> TransformData {
    let alloc = ctx.alloc;
    let component = ctx.component;
    let analysis = ctx.analysis;
    let parsed: &mut JsAst<'a> = ctx.js_arena;
    let ident_gen: &mut IdentGen = ctx.ident_gen;
    let line_index = ctx.line_index;
    let dev = options.dev;
    let root_scope = analysis.scoping.root_scope_id();

    let mut ctx = TransformCtx {
        analysis,
        ident_gen,
        transform_data: TransformData::new(),
        expr_handles: Vec::new(),
        stmt_handles: Vec::new(),
        bind_expr_handles: Vec::new(),
    };

    walk_fragment(&mut ctx, component.root, component, parsed, root_scope);

    let TransformCtx {
        transform_data,
        expr_handles,
        stmt_handles,
        bind_expr_handles,
        ident_gen,
        ..
    } = ctx;

    transformer::template_entry::run_template(
        alloc,
        analysis,
        &analysis.scoping,
        ident_gen,
        expr_handles,
        stmt_handles,
        bind_expr_handles,
        transform_data,
        parsed,
        component,
        line_index,
        dev,
        &options.filename,
    )
}

struct TransformCtx<'a, 'b> {
    analysis: &'b AnalysisData<'a>,
    ident_gen: &'b mut IdentGen,
    transform_data: TransformData,

    expr_handles: Vec<(OxcNodeId, Option<SvelteNodeId>)>,
    stmt_handles: Vec<(OxcNodeId, Option<SvelteNodeId>)>,

    bind_expr_handles: Vec<BindExprHandle>,
}

pub(crate) enum BindHandleKind {
    Element,
    Component { prop_name: String },
    This,
}

pub(crate) struct BindExprHandle {
    pub bind_id: OxcNodeId,
    pub owner: SvelteNodeId,
    pub kind: BindHandleKind,
}

fn walk_fragment<'a>(
    ctx: &mut TransformCtx<'a, '_>,
    fragment_id: FragmentId,
    component: &Component,
    parsed: &mut JsAst<'a>,
    scope: ScopeId,
) {
    for &id in component.fragment_nodes(fragment_id) {
        let node = component.store.get(id);
        walk_node(ctx, node, component, parsed, scope);
    }
}

fn reserve_each_index_name(ctx: &mut TransformCtx<'_, '_>, block: &EachBlock) {
    let name = ctx.ident_gen.generate("$$index");
    ctx.transform_data
        .each_index_internal_names
        .insert(block.id, name);

    let BlockSemantics::Each(sem) = ctx.analysis.block_semantics(block.id) else {
        return;
    };
    let EachItemKind::Identifier(item_sym) = &sem.item else {
        return;
    };
    if !ctx
        .analysis
        .binding_semantics(*item_sym)
        .is_each_item_indexed_legacy()
    {
        return;
    }
    ctx.transform_data
        .each_block_by_item_legacy
        .insert(*item_sym, block.id);
    if block.index.is_some() {
        return;
    }
    ctx.transform_data
        .each_index_block_by_item
        .insert(*item_sym, block.id);
}

fn reserve_destructure_default_simple<'a>(
    ctx: &mut TransformCtx<'a, '_>,
    block: &EachBlock,
    parsed: &JsAst<'a>,
) {
    let Some(context) = block.context.as_ref() else {
        return;
    };
    let Some(Statement::VariableDeclaration(decl)) = parsed.stmt(context.id()) else {
        return;
    };
    let Some(declarator) = decl.declarations.first() else {
        return;
    };
    walk_bindings(&declarator.id, |v| {
        let mut flags: Vec<bool> = Vec::new();
        for step in v.path {
            if let Some(default) = step.default {
                flags.push(is_simple_expression(default));
            }
        }
        if !flags.is_empty() {
            ctx.transform_data
                .destructure_default_simple
                .insert(v.symbol, flags);
        }
    });
}

fn reserve_each_collection_name_legacy(
    ctx: &mut TransformCtx<'_, '_>,
    block: &EachBlock,
    body_scope: ScopeId,
) {
    let BlockSemantics::Each(sem) = ctx.analysis.block_semantics(block.id) else {
        return;
    };
    if !sem.shadows_outer {
        return;
    }
    let mut writeback_syms: Vec<svelte_component_semantics::SymbolId> = Vec::new();
    {
        let storage = ctx.analysis.scoping.semantics();
        let names: Vec<String> = storage
            .own_binding_names(body_scope)
            .map(|name| name.to_string())
            .collect();
        for name in &names {
            let Some(sym) = storage.get_binding(body_scope, name) else {
                continue;
            };
            let writes_back = ctx
                .analysis
                .each_item_indirect_sources(sym)
                .is_some_and(|sources| !sources.is_empty());
            if writes_back {
                writeback_syms.push(sym);
            }
        }
    }
    if writeback_syms.is_empty() {
        return;
    }
    let name = ctx.ident_gen.generate("$$array");
    ctx.transform_data
        .each_collection_internal_names_legacy
        .insert(block.id, name);
    for sym in writeback_syms {
        ctx.transform_data
            .each_collection_block_by_item_legacy
            .insert(sym, block.id);
    }
}

fn walk_node<'a>(
    ctx: &mut TransformCtx<'a, '_>,
    node: &Node,
    component: &Component,
    parsed: &mut JsAst<'a>,
    scope: ScopeId,
) {
    match node {
        Node::ExpressionTag(tag) => {
            record_expr(ctx, parsed, &tag.expression, Some(tag.id));
        }
        Node::Element(el) => {
            walk_attrs(ctx, &el.attributes, parsed);
            walk_fragment(ctx, el.fragment, component, parsed, scope);
        }
        Node::SlotElementLegacy(el) => {
            walk_attrs(ctx, &el.attributes, parsed);
            walk_fragment(ctx, el.fragment, component, parsed, scope);
        }
        Node::ComponentNode(_) | Node::SvelteComponentLegacy(_) | Node::SvelteSelf(_) => {
            if let Node::ComponentNode(cn) = node {
                record_expr(ctx, parsed, &cn.name, Some(cn.id));
            }
            if let Some(view) = node.as_component_like() {
                walk_component_like(
                    ctx,
                    view.attributes,
                    view.fragment,
                    view.legacy_slots,
                    component,
                    parsed,
                    scope,
                );
            }
        }
        Node::IfBlock(block) => {
            record_expr(ctx, parsed, &block.test, Some(block.id));
            let cons_scope = ctx
                .analysis
                .effective_fragment_scope(block.consequent, scope);
            walk_fragment(ctx, block.consequent, component, parsed, cons_scope);
            if let Some(alt) = block.alternate {
                let alt_scope = ctx.analysis.effective_fragment_scope(alt, scope);
                walk_fragment(ctx, alt, component, parsed, alt_scope);
            }
        }
        Node::EachBlock(block) => {
            record_expr(ctx, parsed, &block.expression, Some(block.id));
            if let Some(key) = block.key.as_ref() {
                ctx.expr_handles.push((key.id(), Some(block.id)));
            }
            if let Some(context) = block.context.as_ref() {
                ctx.stmt_handles.push((context.id(), Some(block.id)));
            }
            reserve_destructure_default_simple(ctx, block, parsed);
            let body_scope = ctx.analysis.effective_fragment_scope(block.body, scope);
            walk_fragment(ctx, block.body, component, parsed, body_scope);
            if let Some(fb) = block.fallback {
                walk_fragment(ctx, fb, component, parsed, scope);
            }
            reserve_each_index_name(ctx, block);
            reserve_each_collection_name_legacy(ctx, block, body_scope);
        }
        Node::SnippetBlock(block) => {
            let snippet_scope = ctx.analysis.effective_fragment_scope(block.body, scope);
            ctx.stmt_handles.push((block.decl.id(), Some(block.id)));
            walk_fragment(ctx, block.body, component, parsed, snippet_scope);
        }
        Node::RenderTag(tag) => {
            record_expr(ctx, parsed, &tag.expression, Some(tag.id));
        }
        Node::HtmlTag(tag) => {
            record_expr(ctx, parsed, &tag.expression, Some(tag.id));
        }
        Node::ConstTag(tag) => {
            ctx.stmt_handles.push((tag.decl.id(), Some(tag.id)));

            if let BlockSemantics::ConstTag(sem) = ctx.analysis.block_semantics(tag.id)
                && is_destructured_const_tag(ctx.analysis, sem.decl_node_id)
            {
                let tmp = ctx.ident_gen.generate("computed_const");
                ctx.transform_data.const_tag_tmp_names.insert(tag.id, tmp);
            }
        }
        Node::DeclarationTag(tag) => {
            ctx.stmt_handles.push((tag.declaration.id(), Some(tag.id)));
        }
        Node::KeyBlock(block) => {
            ctx.expr_handles
                .push((block.expression.id(), Some(block.id)));
            let child_scope = ctx.analysis.effective_fragment_scope(block.fragment, scope);
            walk_fragment(ctx, block.fragment, component, parsed, child_scope);
        }
        Node::SvelteHead(head) => {
            let child_scope = ctx.analysis.effective_fragment_scope(head.fragment, scope);
            walk_fragment(ctx, head.fragment, component, parsed, child_scope);
        }
        Node::SvelteFragmentLegacy(el) => {
            walk_attrs(ctx, &el.attributes, parsed);
            walk_fragment(ctx, el.fragment, component, parsed, scope);
        }
        Node::SvelteElement(el) => {
            if let Some(tag_ref) = el.this_expr() {
                record_expr(ctx, parsed, tag_ref, Some(el.id));
            }
            walk_attrs(ctx, &el.attributes, parsed);
            let child_scope = ctx.analysis.effective_fragment_scope(el.fragment, scope);
            walk_fragment(ctx, el.fragment, component, parsed, child_scope);
        }
        Node::SvelteWindow(w) => walk_attrs(ctx, &w.attributes, parsed),
        Node::SvelteDocument(d) => walk_attrs(ctx, &d.attributes, parsed),
        Node::SvelteBody(b) => walk_attrs(ctx, &b.attributes, parsed),
        Node::SvelteBoundary(b) => {
            walk_attrs(ctx, &b.attributes, parsed);
            let child_scope = ctx.analysis.effective_fragment_scope(b.fragment, scope);
            walk_fragment(ctx, b.fragment, component, parsed, child_scope);
        }
        Node::AwaitBlock(block) => {
            record_expr(ctx, parsed, &block.expression, Some(block.id));
            if let Some(value) = block.value.as_ref() {
                ctx.stmt_handles.push((value.id(), Some(block.id)));
            }
            if let Some(error) = block.error.as_ref() {
                ctx.stmt_handles.push((error.id(), Some(block.id)));
            }
            if let Some(p) = block.pending {
                let pending_scope = ctx.analysis.effective_fragment_scope(p, scope);
                walk_fragment(ctx, p, component, parsed, pending_scope);
            }
            if let Some(t) = block.then {
                let then_scope = ctx.analysis.effective_fragment_scope(t, scope);
                walk_fragment(ctx, t, component, parsed, then_scope);
            }
            if let Some(c) = block.catch {
                let catch_scope = ctx.analysis.effective_fragment_scope(c, scope);
                walk_fragment(ctx, c, component, parsed, catch_scope);
            }
        }
        Node::DebugTag(tag) => {
            for ident_ref in &tag.identifier_refs {
                ctx.expr_handles.push((ident_ref.id(), Some(tag.id)));
            }
        }
        Node::Text(_) | Node::Comment(_) | Node::Error(_) => {}
    }
}

fn record_expr<'a>(
    ctx: &mut TransformCtx<'a, '_>,
    _parsed: &JsAst<'a>,
    expr_ref: &ExprRef,
    owner: Option<SvelteNodeId>,
) {
    ctx.expr_handles.push((expr_ref.id(), owner));
}

#[allow(clippy::too_many_arguments)]
fn walk_component_like<'a>(
    ctx: &mut TransformCtx<'a, '_>,
    attributes: &[Attribute],
    cn_fragment: FragmentId,
    legacy_slots: &[LegacySlot],
    component: &Component,
    parsed: &mut JsAst<'a>,
    scope: ScopeId,
) {
    let component_has_slot_attr =
        attrs_static_slot_name(attributes, component.source.as_str()).is_some();
    let default_scope = if component_has_slot_attr {
        scope
    } else {
        ctx.analysis
            .scoping
            .fragment_scope_by_id(cn_fragment)
            .unwrap_or(scope)
    };

    for attr in attributes {
        walk_attrs(ctx, slice::from_ref(attr), parsed);
    }

    walk_fragment(ctx, cn_fragment, component, parsed, default_scope);

    let slot_frags: Vec<FragmentId> = legacy_slots.iter().map(|s| s.fragment).collect();
    for slot_fid in slot_frags {
        let slot_scope = ctx
            .analysis
            .scoping
            .fragment_scope_by_id(slot_fid)
            .unwrap_or(scope);
        walk_fragment(ctx, slot_fid, component, parsed, slot_scope);
    }
}

fn walk_attrs<'a>(ctx: &mut TransformCtx<'a, '_>, attrs: &[Attribute], parsed: &JsAst<'a>) {
    for attr in attrs {
        let owner = Some(attr.id());

        if let Attribute::BindDirective(bind) = attr {
            let bind_id = bind.expression.id();
            let is_user_sequence = parsed.expr(bind_id).is_some_and(|e| {
                matches!(e.get_inner_expression(), Expression::SequenceExpression(_))
            });
            if bind.name == "this" {
                if is_user_sequence {
                    ctx.expr_handles.push((bind_id, owner));
                    continue;
                }
                let route_this = match ctx.analysis.attributes.get(attr.id()) {
                    AttributeSemantics::ComponentBind(_) => true,
                    AttributeSemantics::ElementBind(b) => {
                        !matches!(b.kind, HtmlBindKind::StoreSubscribed { .. })
                    }
                    _ => false,
                };
                if route_this {
                    ctx.bind_expr_handles.push(BindExprHandle {
                        bind_id,
                        owner: attr.id(),
                        kind: BindHandleKind::This,
                    });
                }
                continue;
            }

            let is_window_or_document = matches!(
                ctx.analysis.attributes.get(attr.id()),
                AttributeSemantics::WindowBind(_) | AttributeSemantics::DocumentBind(_)
            );
            if is_window_or_document {
                continue;
            }

            let is_bindable_prop_source = parsed.expr(bind_id).is_some_and(|expr| {
                let mut current = expr.get_inner_expression();
                loop {
                    match current {
                        Expression::StaticMemberExpression(m) => {
                            current = m.object.get_inner_expression()
                        }
                        Expression::ComputedMemberExpression(m) => {
                            current = m.object.get_inner_expression()
                        }
                        Expression::Identifier(id) => {
                            let Some(ref_id) = id.reference_id.get() else {
                                return false;
                            };
                            return ctx
                                .analysis
                                .reference_semantics(ref_id)
                                .is_bindable_prop_access();
                        }
                        _ => return false,
                    }
                }
            });
            if is_bindable_prop_source {
                continue;
            }
            if is_user_sequence {
                ctx.expr_handles.push((bind_id, owner));
            } else {
                let kind = match ctx.analysis.attributes.get(attr.id()) {
                    AttributeSemantics::ComponentBind(_) => BindHandleKind::Component {
                        prop_name: bind.name.clone(),
                    },
                    AttributeSemantics::ElementBind(_) => BindHandleKind::Element,
                    _ => unreachable!(
                        "bind directive must classify as ElementBind/ComponentBind (window/document filtered above)"
                    ),
                };
                ctx.bind_expr_handles.push(BindExprHandle {
                    bind_id,
                    owner: attr.id(),
                    kind,
                });
            }
            continue;
        }
        if let Some(handle) = get_attr_expr_id(attr) {
            ctx.expr_handles.push((handle, owner));
        }
        if let Some(handle) = get_directive_name_id(attr) {
            ctx.expr_handles.push((handle, owner));
        }

        let concat_parts: Option<&[ConcatPart]> = match attr {
            Attribute::ConcatenationAttribute(a) => Some(&a.parts),
            Attribute::StyleDirective(a) => match &a.value {
                StyleDirectiveValue::Concatenation(parts) => Some(parts),
                _ => None,
            },
            _ => None,
        };
        if let Some(parts) = concat_parts {
            for part in parts {
                if let ConcatPart::Dynamic { expr, .. } = part {
                    ctx.expr_handles.push((expr.id(), owner));
                }
            }
        }
    }
}

fn get_attr_expr_id(attr: &Attribute) -> Option<OxcNodeId> {
    match attr {
        Attribute::ExpressionAttribute(a) => Some(a.expression.id()),
        Attribute::ClassDirective(a) => Some(a.expression.id()),
        Attribute::StyleDirective(a) => match &a.value {
            StyleDirectiveValue::Expression => Some(a.expression.id()),
            _ => None,
        },

        Attribute::BindDirective(_) => None,
        Attribute::LetDirectiveLegacy(_) => None,
        Attribute::SpreadAttribute(a) => Some(a.expression.id()),
        Attribute::UseDirective(a) => a.expression.as_ref().map(|r| r.id()),
        Attribute::OnDirectiveLegacy(a) => a.expression.as_ref().map(|r| r.id()),
        Attribute::TransitionDirective(a) => a.expression.as_ref().map(|r| r.id()),
        Attribute::AnimateDirective(a) => a.expression.as_ref().map(|r| r.id()),
        Attribute::AttachTag(a) => Some(a.expression.id()),
        Attribute::StringAttribute(_)
        | Attribute::BooleanAttribute(_)
        | Attribute::ConcatenationAttribute(_) => None,
    }
}

fn get_directive_name_id(attr: &Attribute) -> Option<OxcNodeId> {
    match attr {
        Attribute::UseDirective(a) => Some(a.name_ref.id()),
        Attribute::TransitionDirective(a) => Some(a.name_ref.id()),
        Attribute::AnimateDirective(a) => Some(a.name_ref.id()),
        _ => None,
    }
}

fn attrs_static_slot_name<'a>(attrs: &'a [Attribute], source: &'a str) -> Option<&'a str> {
    attrs.iter().find_map(|attr| match attr {
        Attribute::StringAttribute(attr) if attr.name.as_str() == "slot" => {
            Some(attr.value(source))
        }
        _ => None,
    })
}

fn is_destructured_const_tag(analysis: &AnalysisData<'_>, decl_node_id: SemOxcNodeId) -> bool {
    use oxc_ast::{AstKind, ast::BindingPattern};
    let Some(AstKind::VariableDeclaration(decl)) = analysis.scoping.js_kind(decl_node_id) else {
        return false;
    };
    let Some(declarator) = decl.declarations.first() else {
        return false;
    };
    !matches!(declarator.id, BindingPattern::BindingIdentifier(_))
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxc_allocator::Allocator;
    use oxc_ast::ast::Expression;
    use svelte_analyze::{IdentGen, analyze};
    use svelte_ast::{Component, ExpressionTag, Node, SnippetBlock};

    fn find_snippet_block<'a>(
        fragment_id: FragmentId,
        component: &'a Component,
        name: &str,
    ) -> Option<&'a SnippetBlock> {
        for &id in component.fragment_nodes(fragment_id) {
            match component.store.get(id) {
                Node::SnippetBlock(block) if block.name(component.source.as_str()) == name => {
                    return Some(block);
                }
                Node::IfBlock(block) => {
                    if let Some(found) = find_snippet_block(block.consequent, component, name) {
                        return Some(found);
                    }
                    if let Some(alt) = block.alternate
                        && let Some(found) = find_snippet_block(alt, component, name)
                    {
                        return Some(found);
                    }
                }
                Node::EachBlock(block) => {
                    if let Some(found) = find_snippet_block(block.body, component, name) {
                        return Some(found);
                    }
                    if let Some(fallback) = block.fallback
                        && let Some(found) = find_snippet_block(fallback, component, name)
                    {
                        return Some(found);
                    }
                }
                Node::SnippetBlock(block) => {
                    if let Some(found) = find_snippet_block(block.body, component, name) {
                        return Some(found);
                    }
                }
                Node::Element(el) => {
                    if let Some(found) = find_snippet_block(el.fragment, component, name) {
                        return Some(found);
                    }
                }
                Node::ComponentNode(node) => {
                    if let Some(found) = find_snippet_block(node.fragment, component, name) {
                        return Some(found);
                    }
                }
                _ => {}
            }
        }
        None
    }

    fn find_expr_tag<'a>(
        fragment_id: FragmentId,
        component: &'a Component,
        needle: &str,
    ) -> Option<&'a ExpressionTag> {
        for &id in component.fragment_nodes(fragment_id) {
            match component.store.get(id) {
                Node::ExpressionTag(tag)
                    if component.source_text(tag.expression.span).trim() == needle =>
                {
                    return Some(tag);
                }
                Node::Element(el) => {
                    if let Some(found) = find_expr_tag(el.fragment, component, needle) {
                        return Some(found);
                    }
                }
                Node::IfBlock(block) => {
                    if let Some(found) = find_expr_tag(block.consequent, component, needle) {
                        return Some(found);
                    }
                    if let Some(alt) = block.alternate
                        && let Some(found) = find_expr_tag(alt, component, needle)
                    {
                        return Some(found);
                    }
                }
                _ => {}
            }
        }
        None
    }

    #[test]
    fn snippet_default_label_expr_uses_signal_get_after_transform() {
        let source = r#"{#snippet withDefault({ label = "default" })}
    <span>{label}</span>
{/snippet}"#;
        let alloc = Allocator::default();
        let (component, js_result, parse_diags) = svelte_parser::parse_with_js(&alloc, source);
        assert!(
            parse_diags.is_empty(),
            "unexpected parse diags: {parse_diags:?}"
        );
        let (analysis, mut parsed, diags) = analyze(&component, js_result);
        assert!(diags.is_empty(), "unexpected analyze diags: {diags:?}");

        let mut ident_gen = IdentGen::new();
        let line_index = svelte_span::LineIndex::new(component.source.as_str());
        let mut ctx = svelte_types::CompileContext {
            alloc: &alloc,
            component: &component,
            analysis: &analysis,
            js_arena: &mut parsed,
            ident_gen: &mut ident_gen,
            line_index: &line_index,
        };
        transform_component(&mut ctx, &svelte_types::TransformOptions::default());

        let snippet = find_snippet_block(component.root, &component, "withDefault")
            .unwrap_or_else(|| panic!("missing snippet"));
        let expr_tag = find_expr_tag(snippet.body, &component, "label")
            .unwrap_or_else(|| panic!("missing label expression"));
        let expr = parsed
            .expr(expr_tag.expression.id())
            .unwrap_or_else(|| panic!("missing expr"));
        assert!(
            matches!(expr, Expression::CallExpression(_)),
            "unexpected transformed expr: {expr:?}"
        );
    }

    #[test]
    fn snippet_default_label_expr_uses_signal_get_after_transform_with_script_context() {
        let source = r#"<script>
    let data = $state({ name: "world", age: 25 });
</script>

{#snippet greeting({ name, age })}
    <p>{name} is {age}</p>
{/snippet}

{#snippet withDefault({ label = "default" })}
    <span>{label}</span>
{/snippet}

{@render greeting(data)}
{@render withDefault({})}"#;
        let alloc = Allocator::default();
        let (component, js_result, parse_diags) = svelte_parser::parse_with_js(&alloc, source);
        assert!(
            parse_diags.is_empty(),
            "unexpected parse diags: {parse_diags:?}"
        );
        let (analysis, mut parsed, diags) = analyze(&component, js_result);
        assert!(diags.is_empty(), "unexpected analyze diags: {diags:?}");

        let mut ident_gen = IdentGen::new();
        let line_index = svelte_span::LineIndex::new(component.source.as_str());
        let mut ctx = svelte_types::CompileContext {
            alloc: &alloc,
            component: &component,
            analysis: &analysis,
            js_arena: &mut parsed,
            ident_gen: &mut ident_gen,
            line_index: &line_index,
        };
        transform_component(&mut ctx, &svelte_types::TransformOptions::default());

        let snippet = find_snippet_block(component.root, &component, "withDefault")
            .unwrap_or_else(|| panic!("missing snippet"));
        let expr_tag = find_expr_tag(snippet.body, &component, "label")
            .unwrap_or_else(|| panic!("missing label expression"));
        let expr = parsed
            .expr(expr_tag.expression.id())
            .unwrap_or_else(|| panic!("missing expr"));
        assert!(
            matches!(expr, Expression::CallExpression(_)),
            "unexpected transformed expr: {expr:?}"
        );
    }
}
