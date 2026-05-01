mod data;
pub mod rune_refs;
pub mod transformer;

pub use data::TransformData;

pub use transformer::{
    IgnoreQuery, TransformScriptOutput, compute_line_col, sanitize_location, transform_script,
};

use oxc_syntax::node::NodeId as OxcNodeId;

use svelte_analyze::scope::ScopeId;
use svelte_analyze::{AnalysisData, IdentGen, JsAst};
use svelte_ast::{Attribute, Component, ConcatPart, Node};

pub fn transform_component<'a>(
    ctx: &mut svelte_types::CompileContext<'a, '_>,
    options: &svelte_types::TransformOptions,
) -> TransformData {
    let alloc = ctx.alloc;
    let component = ctx.component;
    let analysis = ctx.analysis;
    let parsed: &mut JsAst<'a> = ctx.js_arena;
    let ident_gen: &mut IdentGen = ctx.ident_gen;
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
        ..
    } = ctx;

    transformer::template_entry::run_template(
        alloc,
        analysis,
        &analysis.scoping,
        expr_handles,
        stmt_handles,
        bind_expr_handles,
        transform_data,
        parsed,
        dev,
    )
}

struct TransformCtx<'a, 'b> {
    analysis: &'b AnalysisData<'a>,
    ident_gen: &'b mut IdentGen,
    transform_data: TransformData,

    expr_handles: Vec<(OxcNodeId, Option<svelte_ast::NodeId>)>,
    stmt_handles: Vec<(OxcNodeId, Option<svelte_ast::NodeId>)>,

    bind_expr_handles: Vec<(OxcNodeId, svelte_ast::NodeId)>,
}

fn walk_fragment<'a>(
    ctx: &mut TransformCtx<'a, '_>,
    fragment_id: svelte_ast::FragmentId,
    component: &Component,
    parsed: &mut JsAst<'a>,
    scope: ScopeId,
) {
    let nodes = component.fragment_nodes(fragment_id).to_vec();
    for id in nodes {
        let node = component.store.get(id);
        walk_node(ctx, node, component, parsed, scope);
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
        Node::ComponentNode(_) | Node::SvelteComponentLegacy(_) => {
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
            let body_scope = ctx.analysis.effective_fragment_scope(block.body, scope);
            walk_fragment(ctx, block.body, component, parsed, body_scope);
            if let Some(fb) = block.fallback {
                walk_fragment(ctx, fb, component, parsed, scope);
            }
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

            if let svelte_analyze::BlockSemantics::ConstTag(sem) =
                ctx.analysis.block_semantics(tag.id)
                && is_destructured_const_tag(ctx.analysis, sem.decl_node_id)
            {
                let tmp = ctx.ident_gen.generate("computed_const");
                ctx.transform_data.const_tag_tmp_names.insert(tag.id, tmp);
            }
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
    expr_ref: &svelte_ast::ExprRef,
    owner: Option<svelte_ast::NodeId>,
) {
    ctx.expr_handles.push((expr_ref.id(), owner));
}

#[allow(clippy::too_many_arguments)]
fn walk_component_like<'a>(
    ctx: &mut TransformCtx<'a, '_>,
    attributes: &[Attribute],
    cn_fragment: svelte_ast::FragmentId,
    legacy_slots: &[svelte_ast::LegacySlot],
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
        walk_attrs(ctx, std::slice::from_ref(attr), parsed);
    }

    walk_fragment(ctx, cn_fragment, component, parsed, default_scope);

    let slot_frags: Vec<svelte_ast::FragmentId> = legacy_slots.iter().map(|s| s.fragment).collect();
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
            let is_user_sequence = parsed
                .expr(bind_id)
                .is_some_and(|e| matches!(e, oxc_ast::ast::Expression::SequenceExpression(_)));
            if bind.name == "this" {
                continue;
            }

            let is_window_or_document =
                ctx.analysis
                    .bind_target_semantics(attr.id())
                    .is_some_and(|sem| {
                        matches!(
                            sem.property(),
                            svelte_analyze::BindPropertyKind::Window(_)
                                | svelte_analyze::BindPropertyKind::Document(_)
                        )
                    });
            if is_window_or_document {
                continue;
            }

            let is_bindable_prop_source = parsed.expr(bind_id).is_some_and(|expr| {
                let mut current = expr;
                loop {
                    match current {
                        oxc_ast::ast::Expression::StaticMemberExpression(m) => current = &m.object,
                        oxc_ast::ast::Expression::ComputedMemberExpression(m) => {
                            current = &m.object
                        }
                        oxc_ast::ast::Expression::Identifier(id) => {
                            let Some(ref_id) = id.reference_id.get() else {
                                return false;
                            };
                            return matches!(
                                ctx.analysis.reference_semantics(ref_id),
                                svelte_analyze::ReferenceSemantics::PropRead(
                                    svelte_analyze::PropReferenceSemantics::Source {
                                        bindable: true,
                                        ..
                                    }
                                ) | svelte_analyze::ReferenceSemantics::PropMutation {
                                    bindable: true,
                                    ..
                                }
                            );
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
                ctx.bind_expr_handles.push((bind_id, attr.id()));
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
                svelte_ast::StyleDirectiveValue::Concatenation(parts) => Some(parts),
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
            svelte_ast::StyleDirectiveValue::Expression => Some(a.expression.id()),
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
            Some(&source[attr.value_span.start as usize..attr.value_span.end as usize])
        }
        _ => None,
    })
}

fn is_destructured_const_tag(
    analysis: &svelte_analyze::AnalysisData<'_>,
    decl_node_id: svelte_component_semantics::OxcNodeId,
) -> bool {
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
    use svelte_ast::{Component, Node};

    fn find_snippet_block<'a>(
        fragment_id: svelte_ast::FragmentId,
        component: &'a Component,
        name: &str,
    ) -> Option<&'a svelte_ast::SnippetBlock> {
        let nodes = component.fragment_nodes(fragment_id).to_vec();
        for id in nodes {
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
        fragment_id: svelte_ast::FragmentId,
        component: &'a Component,
        needle: &str,
    ) -> Option<&'a svelte_ast::ExpressionTag> {
        let nodes = component.fragment_nodes(fragment_id).to_vec();
        for id in nodes {
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
        let mut ctx = svelte_types::CompileContext {
            alloc: &alloc,
            component: &component,
            analysis: &analysis,
            js_arena: &mut parsed,
            ident_gen: &mut ident_gen,
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
        let mut ctx = svelte_types::CompileContext {
            alloc: &alloc,
            component: &component,
            analysis: &analysis,
            js_arena: &mut parsed,
            ident_gen: &mut ident_gen,
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
