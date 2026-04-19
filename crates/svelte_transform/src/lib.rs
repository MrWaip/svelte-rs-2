//! AST-to-AST transformation pass for Svelte components.
//!
//! Template- and script-side rewrites (rune references, prop accesses,
//! stores, etc.) share a single `ComponentTransformer` in
//! `transformer/`. The template entry point collects expression/statement
//! handles during a structural walk of the Svelte tree, then drives the
//! transformer over a reusable synthetic `Program` via `oxc_traverse`.

mod data;
pub mod rune_refs;
pub mod transformer;

pub use data::TransformData;

pub use transformer::{
    compute_line_col, sanitize_location, transform_script, IgnoreQuery, TransformScriptOutput,
};

use oxc_allocator::Allocator;

use svelte_analyze::scope::ScopeId;
use svelte_analyze::{AnalysisData, ExprHandle, IdentGen, ParserResult, StmtHandle};
use svelte_ast::{Attribute, Component, ConcatPart, Fragment, FragmentKey, Node};

/// Transform all parsed template expressions and statements in-place.
///
/// Rewrites include: rune references → `$.get/set/update`, prop sources →
/// thunk calls, prop non-sources → `$$props.name`, each-block context
/// variables → `$.get(name)`, snippet parameter defaults, destructured
/// const aliases → `$.get(tmp).prop`.
///
/// Must be called AFTER all analysis passes are complete.
pub fn transform_component<'a>(
    alloc: &'a Allocator,
    component: &Component,
    analysis: &AnalysisData<'a>,
    parsed: &mut ParserResult<'a>,
    ident_gen: &mut IdentGen,
    dev: bool,
) -> TransformData {
    let root_scope = analysis.scoping.root_scope_id();

    let mut ctx = TransformCtx {
        analysis,
        ident_gen,
        transform_data: TransformData::new(),
        expr_handles: Vec::new(),
        stmt_handles: Vec::new(),
        bind_expr_handles: Vec::new(),
    };

    // Pass 1: structural walk — records (handle) for every template
    // expression/statement and pre-populates const-tag tmp names needed by
    // the `ConstAliasRead` branch.
    walk_fragment(&mut ctx, &component.fragment, component, parsed, root_scope);

    let TransformCtx {
        transform_data,
        expr_handles,
        stmt_handles,
        bind_expr_handles,
        ..
    } = ctx;

    // Pass 2: unified ComponentTransformer (Template mode) runs over each
    // collected handle via a reusable synthetic Program.
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
    /// Pairs of (expression handle, owning template node). Owner is
    /// `Some` for expressions bound to a specific node (ExpressionTag,
    /// IfBlock test, EachBlock collection, RenderTag arg, etc.) and
    /// `None` for expressions that aren't owned by a single node
    /// (concat parts, directive names). The owner is used by the
    /// template-mode transformer for node-level predicates such as
    /// `is_ignored(node_id, "await_reactivity_loss")`.
    expr_handles: Vec<(ExprHandle, Option<svelte_ast::NodeId>)>,
    stmt_handles: Vec<(StmtHandle, Option<svelte_ast::NodeId>)>,
    /// Bind-directive expressions handled by the dedicated bind pass. The
    /// pass clones the original expression into a read-form getter and a
    /// write-form setter, transforms each through the full pipeline, and
    /// repacks them into a `SequenceExpression(getter, setter)` that the
    /// codegen unpacks directly into runtime bind calls.
    bind_expr_handles: Vec<(ExprHandle, svelte_ast::NodeId)>,
}

// ---------------------------------------------------------------------------
// Template tree walker: records handles + const-tag tmp names.
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
            record_expr(ctx, parsed, tag.expression_span.start, Some(tag.id));
        }
        Node::Element(el) => {
            walk_attrs(ctx, &el.attributes, parsed);
            walk_fragment(ctx, &el.fragment, component, parsed, scope);
        }
        Node::SlotElementLegacy(el) => {
            walk_attrs(ctx, &el.attributes, parsed);
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
                walk_attrs(ctx, std::slice::from_ref(attr), parsed);
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
            record_expr(ctx, parsed, block.test_span.start, Some(block.id));
            let cons_scope = ctx.analysis.if_consequent_scope(block.id, scope);
            walk_fragment(ctx, &block.consequent, component, parsed, cons_scope);
            if let Some(alt) = &block.alternate {
                let alt_scope = ctx.analysis.if_alternate_scope(block.id, scope);
                walk_fragment(ctx, alt, component, parsed, alt_scope);
            }
        }
        Node::EachBlock(block) => {
            record_expr(ctx, parsed, block.expression_span.start, Some(block.id));
            let body_scope = ctx.analysis.each_body_scope(block.id, scope);
            walk_fragment(ctx, &block.body, component, parsed, body_scope);
            if let Some(fb) = &block.fallback {
                walk_fragment(ctx, fb, component, parsed, scope);
            }
        }
        Node::SnippetBlock(block) => {
            let snippet_scope = ctx.analysis.snippet_body_scope(block.id, scope);
            let handle = ctx.analysis.snippet_stmt_handle(block.id);
            if let Some(handle) = handle {
                ctx.stmt_handles.push((handle, Some(block.id)));
            }
            walk_fragment(ctx, &block.body, component, parsed, snippet_scope);
        }
        Node::RenderTag(tag) => {
            record_expr(ctx, parsed, tag.expression_span.start, Some(tag.id));
        }
        Node::HtmlTag(tag) => {
            record_expr(ctx, parsed, tag.expression_span.start, Some(tag.id));
        }
        Node::ConstTag(tag) => {
            if let Some(handle) = parsed.stmt_handle(tag.expression_span.start) {
                ctx.stmt_handles.push((handle, Some(tag.id)));
            }

            if let svelte_analyze::BlockSemantics::ConstTag(sem) =
                ctx.analysis.block_semantics(tag.id)
            {
                if sem.bindings.len() > 1 {
                    let tmp = ctx.ident_gen.gen("computed_const");
                    ctx.transform_data.const_tag_tmp_names.insert(tag.id, tmp);
                }
            }
        }
        Node::KeyBlock(block) => {
            if let Some(handle) = parsed.expr_handle(block.expression_span.start) {
                ctx.expr_handles.push((handle, Some(block.id)));
            }
            let child_scope = ctx.analysis.key_block_body_scope(block.id, scope);
            walk_fragment(ctx, &block.fragment, component, parsed, child_scope);
        }
        Node::SvelteHead(head) => {
            let child_scope = ctx.analysis.svelte_head_body_scope(head.id, scope);
            walk_fragment(ctx, &head.fragment, component, parsed, child_scope);
        }
        Node::SvelteFragmentLegacy(el) => {
            walk_attrs(ctx, &el.attributes, parsed);
            walk_fragment(ctx, &el.fragment, component, parsed, scope);
        }
        Node::SvelteElement(el) => {
            if !el.static_tag {
                record_expr(ctx, parsed, el.tag_span.start, Some(el.id));
            }
            walk_attrs(ctx, &el.attributes, parsed);
            let child_scope = ctx.analysis.svelte_element_body_scope(el.id, scope);
            walk_fragment(ctx, &el.fragment, component, parsed, child_scope);
        }
        Node::SvelteWindow(w) => walk_attrs(ctx, &w.attributes, parsed),
        Node::SvelteDocument(d) => walk_attrs(ctx, &d.attributes, parsed),
        Node::SvelteBody(b) => walk_attrs(ctx, &b.attributes, parsed),
        Node::SvelteBoundary(b) => {
            walk_attrs(ctx, &b.attributes, parsed);
            let child_scope = ctx.analysis.svelte_boundary_body_scope(b.id, scope);
            walk_fragment(ctx, &b.fragment, component, parsed, child_scope);
        }
        Node::AwaitBlock(block) => {
            record_expr(ctx, parsed, block.expression_span.start, Some(block.id));
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
                    ctx.expr_handles.push((handle, Some(tag.id)));
                }
            }
        }
        Node::Text(_) | Node::Comment(_) | Node::Error(_) => {}
    }
}

fn record_expr<'a>(
    ctx: &mut TransformCtx<'a, '_>,
    parsed: &ParserResult<'a>,
    offset: u32,
    owner: Option<svelte_ast::NodeId>,
) {
    if let Some(handle) = parsed.expr_handle(offset) {
        ctx.expr_handles.push((handle, owner));
    }
}

fn walk_attrs<'a>(ctx: &mut TransformCtx<'a, '_>, attrs: &[Attribute], parsed: &ParserResult<'a>) {
    for attr in attrs {
        let owner = Some(attr.id());
        // Bind directives route into the bind-specific transform pass which
        // clones the expression and builds a `SequenceExpression(getter, setter)`.
        // Two user-authored forms already carry their own get/set shape and
        // must skip that pass, otherwise we'd nest their SequenceExpression
        // inside ours:
        //   - `bind:this={...}`: reference-only binding; getter/setter form is
        //     synthesized directly in codegen from the target identifier.
        //   - `bind:prop={(getter), (setter)}`: user supplied the pair as a
        //     SequenceExpression literal — run it through the generic read
        //     pass so each sub-expression gets normal rune/prop rewrites.
        if let Attribute::BindDirective(bind) = attr {
            let Some(handle) = parsed.expr_handle(bind.expression_span.start) else {
                continue;
            };
            let is_user_sequence = parsed
                .expr(handle)
                .is_some_and(|e| matches!(e, oxc_ast::ast::Expression::SequenceExpression(_)));
            if bind.name == "this" {
                // `bind:this` needs the untouched identifier/member — codegen
                // synthesizes the get/set pair directly from its
                // `ReferenceSemantics`. Running the expression through either
                // the generic read pass or the bind pass would strip that
                // identity (turning `foo` into `$.get(foo)` or worse).
                continue;
            }
            // Window/document bindings produce `$.set(x, $$value, true)` with
            // the silent flag to break reactivity feedback loops. The bind
            // pass only knows the two-arg shape, so keep those handles on the
            // original identifier and let codegen build the silent setter
            // locally.
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
            // Bindable prop sources lower to `$.bind_*(el, <prop_accessor>)`
            // — the runtime takes the accessor directly, there is no
            // `($$value) => foo = $$value` setter to synthesize. Skip the
            // bind pass so codegen can detect the prop shape via
            // `directive_root_reference_semantics` and emit the short form.
            let is_bindable_prop_source = parsed.expr(handle).is_some_and(|expr| {
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
                // User-written `bind:prop={(g), (s)}`: each branch is a real
                // read/write arrow that needs the normal per-expression
                // rewrites. Pass through the generic handles pipeline.
                ctx.expr_handles.push((handle, owner));
            } else {
                ctx.bind_expr_handles.push((handle, attr.id()));
            }
            continue;
        }
        if let Some(handle) = get_attr_expr_handle(parsed, attr) {
            ctx.expr_handles.push((handle, owner));
        }
        if let Some(handle) = get_directive_name_handle(parsed, attr) {
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
                if let ConcatPart::Dynamic { span, .. } = part {
                    if let Some(handle) = parsed.expr_handle(span.start) {
                        // Concat parts keep the attribute as their owner so that
                        // node-level predicates (e.g. is_ignored) resolve on the
                        // enclosing attribute node.
                        ctx.expr_handles.push((handle, owner));
                    }
                }
            }
        }
    }
}

fn get_attr_expr_handle(parsed: &ParserResult<'_>, attr: &Attribute) -> Option<ExprHandle> {
    let offset = match attr {
        Attribute::ExpressionAttribute(a) => Some(a.expression_span.start),
        Attribute::ClassDirective(a) => Some(a.expression_span.start),
        Attribute::StyleDirective(a) => match &a.value {
            svelte_ast::StyleDirectiveValue::Expression => Some(a.expression_span.start),
            _ => None,
        },
        // BindDirective is handled by the bind-specific pass — see walk_attrs.
        Attribute::BindDirective(_) => None,
        Attribute::LetDirectiveLegacy(a) => a.expression_span.map(|s| s.start),
        Attribute::SpreadAttribute(a) => Some(a.expression_span.start),
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

#[cfg(test)]
mod tests {
    use super::*;
    use oxc_allocator::Allocator;
    use oxc_ast::ast::Expression;
    use svelte_analyze::{analyze, IdentGen};
    use svelte_ast::{Component, Fragment, Node};

    fn find_snippet_block<'a>(
        fragment: &'a Fragment,
        component: &'a Component,
        name: &str,
    ) -> Option<&'a svelte_ast::SnippetBlock> {
        for &id in &fragment.nodes {
            match component.store.get(id) {
                Node::SnippetBlock(block) if block.name(component.source.as_str()) == name => {
                    return Some(block);
                }
                Node::IfBlock(block) => {
                    if let Some(found) = find_snippet_block(&block.consequent, component, name) {
                        return Some(found);
                    }
                    if let Some(alt) = &block.alternate {
                        if let Some(found) = find_snippet_block(alt, component, name) {
                            return Some(found);
                        }
                    }
                }
                Node::EachBlock(block) => {
                    if let Some(found) = find_snippet_block(&block.body, component, name) {
                        return Some(found);
                    }
                    if let Some(fallback) = &block.fallback {
                        if let Some(found) = find_snippet_block(fallback, component, name) {
                            return Some(found);
                        }
                    }
                }
                Node::SnippetBlock(block) => {
                    if let Some(found) = find_snippet_block(&block.body, component, name) {
                        return Some(found);
                    }
                }
                Node::Element(el) => {
                    if let Some(found) = find_snippet_block(&el.fragment, component, name) {
                        return Some(found);
                    }
                }
                Node::ComponentNode(node) => {
                    if let Some(found) = find_snippet_block(&node.fragment, component, name) {
                        return Some(found);
                    }
                }
                _ => {}
            }
        }
        None
    }

    fn find_expr_tag<'a>(
        fragment: &'a Fragment,
        component: &'a Component,
        needle: &str,
    ) -> Option<&'a svelte_ast::ExpressionTag> {
        for &id in &fragment.nodes {
            match component.store.get(id) {
                Node::ExpressionTag(tag)
                    if component.source_text(tag.expression_span).trim() == needle =>
                {
                    return Some(tag);
                }
                Node::Element(el) => {
                    if let Some(found) = find_expr_tag(&el.fragment, component, needle) {
                        return Some(found);
                    }
                }
                Node::IfBlock(block) => {
                    if let Some(found) = find_expr_tag(&block.consequent, component, needle) {
                        return Some(found);
                    }
                    if let Some(alt) = &block.alternate {
                        if let Some(found) = find_expr_tag(alt, component, needle) {
                            return Some(found);
                        }
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
        transform_component(
            &alloc,
            &component,
            &analysis,
            &mut parsed,
            &mut ident_gen,
            false,
        );

        let snippet = find_snippet_block(&component.fragment, &component, "withDefault")
            .unwrap_or_else(|| panic!("missing snippet"));
        let expr = find_expr_tag(&snippet.body, &component, "label")
            .unwrap_or_else(|| panic!("missing label expression"));
        let handle = parsed
            .expr_handle(expr.expression_span.start)
            .unwrap_or_else(|| panic!("missing expr handle"));
        let expr = parsed
            .expr(handle)
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
        transform_component(
            &alloc,
            &component,
            &analysis,
            &mut parsed,
            &mut ident_gen,
            false,
        );

        let snippet = find_snippet_block(&component.fragment, &component, "withDefault")
            .unwrap_or_else(|| panic!("missing snippet"));
        let expr = find_expr_tag(&snippet.body, &component, "label")
            .unwrap_or_else(|| panic!("missing label expression"));
        let handle = parsed
            .expr_handle(expr.expression_span.start)
            .unwrap_or_else(|| panic!("missing expr handle"));
        let expr = parsed
            .expr(handle)
            .unwrap_or_else(|| panic!("missing expr"));
        assert!(
            matches!(expr, Expression::CallExpression(_)),
            "unexpected transformed expr: {expr:?}"
        );
    }
}
