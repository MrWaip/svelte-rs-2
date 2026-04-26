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

use oxc_syntax::node::NodeId as OxcNodeId;

use svelte_analyze::scope::ScopeId;
use svelte_analyze::{AnalysisData, IdentGen, JsAst};
use svelte_ast::{Attribute, Component, ConcatPart, Node, NodeId};

/// Transform all parsed template expressions and statements in-place.
///
/// Rewrites include: rune references → `$.get/set/update`, prop sources →
/// thunk calls, prop non-sources → `$$props.name`, each-block context
/// variables → `$.get(name)`, snippet parameter defaults, destructured
/// const aliases → `$.get(tmp).prop`.
///
/// Must be called AFTER all analysis passes are complete.
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

    // Pass 1: structural walk — records (handle) for every template
    // expression/statement and pre-populates const-tag tmp names needed by
    // the `ConstAliasRead` branch.
    walk_fragment(&mut ctx, component.root, component, parsed, root_scope);

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
    expr_handles: Vec<(OxcNodeId, Option<svelte_ast::NodeId>)>,
    stmt_handles: Vec<(OxcNodeId, Option<svelte_ast::NodeId>)>,
    /// Bind-directive expressions handled by the dedicated bind pass. The
    /// pass clones the original expression into a read-form getter and a
    /// write-form setter, transforms each through the full pipeline, and
    /// repacks them into a `SequenceExpression(getter, setter)` that the
    /// codegen unpacks directly into runtime bind calls.
    bind_expr_handles: Vec<(OxcNodeId, svelte_ast::NodeId)>,
}

// ---------------------------------------------------------------------------
// Template tree walker: records handles + const-tag tmp names.
// ---------------------------------------------------------------------------

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
        Node::ComponentNode(cn) => {
            let component_has_slot_attr =
                attrs_static_slot_name(&cn.attributes, component.source.as_str()).is_some();
            let default_scope = if component_has_slot_attr {
                scope
            } else {
                ctx.analysis
                    .scoping
                    .fragment_scope_by_id(cn.fragment)
                    .unwrap_or(scope)
            };

            for attr in &cn.attributes {
                walk_attrs(ctx, std::slice::from_ref(attr), parsed);
            }

            walk_fragment(ctx, cn.fragment, component, parsed, default_scope);

            let slot_pairs: Vec<(svelte_ast::FragmentId, NodeId)> = cn
                .legacy_slots
                .iter()
                .map(|s| (s.fragment, component.fragment_nodes(s.fragment)[0]))
                .collect();
            let cn_id = cn.id;
            for (slot_fid, wrapper_id) in slot_pairs {
                let slot_scope = ctx
                    .analysis
                    .scoping
                    .named_slot_scope(cn_id, wrapper_id)
                    .unwrap_or(scope);
                walk_fragment(ctx, slot_fid, component, parsed, slot_scope);
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

            // Destructured const-tags need a tmp binding for the derived
            // (`computed_const_N`). Resolve the pattern on demand from
            // the payload's `decl_node_id` — block_semantics does not
            // cache per-leaf data.
            if let svelte_analyze::BlockSemantics::ConstTag(sem) =
                ctx.analysis.block_semantics(tag.id)
            {
                if is_destructured_const_tag(ctx.analysis, sem.decl_node_id) {
                    let tmp = ctx.ident_gen.gen("computed_const");
                    ctx.transform_data.const_tag_tmp_names.insert(tag.id, tmp);
                }
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
            if let Some(tag_ref) = el.tag.as_ref() {
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

fn walk_attrs<'a>(ctx: &mut TransformCtx<'a, '_>, attrs: &[Attribute], parsed: &JsAst<'a>) {
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
            let bind_id = bind.expression.id();
            let is_user_sequence = parsed
                .expr(bind_id)
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
                // User-written `bind:prop={(g), (s)}`: each branch is a real
                // read/write arrow that needs the normal per-expression
                // rewrites. Pass through the generic handles pipeline.
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
                    // Concat parts keep the attribute as their owner so that
                    // node-level predicates (e.g. is_ignored) resolve on the
                    // enclosing attribute node.
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
        // BindDirective is handled by the bind-specific pass — see walk_attrs.
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

/// `true` if the `{@const}` backed by `decl_node_id` uses a destructure
/// pattern — resolved on demand from the OXC AST so block_semantics
/// does not have to cache per-tag binding lists.
fn is_destructured_const_tag(
    analysis: &svelte_analyze::AnalysisData<'_>,
    decl_node_id: svelte_component_semantics::OxcNodeId,
) -> bool {
    use oxc_ast::{ast::BindingPattern, AstKind};
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
    use svelte_analyze::{analyze, IdentGen};
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
                    if let Some(alt) = block.alternate {
                        if let Some(found) = find_snippet_block(alt, component, name) {
                            return Some(found);
                        }
                    }
                }
                Node::EachBlock(block) => {
                    if let Some(found) = find_snippet_block(block.body, component, name) {
                        return Some(found);
                    }
                    if let Some(fallback) = block.fallback {
                        if let Some(found) = find_snippet_block(fallback, component, name) {
                            return Some(found);
                        }
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
                    if let Some(alt) = block.alternate {
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
