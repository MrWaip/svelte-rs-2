//! Template side tables and symbol marks.
//!
//! Scopes and bindings are created by `build_component_semantics` via the
//! shared `svelte_component_semantics` storage. This pass only populates side
//! tables (each_blocks, const_tags) and applies symbol marks
//! (each_block_var, each_non_reactive, snippet_name).
//!
//! Only `$$item` (synthetic destructured-context binding) uses `add_binding`
//! directly because it has no JS AST owner.
//!
//! Marks that depend on bindings (find_binding) go in leave_* hooks —
//! by that time the component semantics pass has already created the bindings.

use oxc_ast::ast::{BindingPattern, Statement, VariableDeclarator};
use svelte_ast::{
    is_mathml, is_svg, is_void, Attribute, ComponentNode, ConstTag, EachBlock, Element, Namespace,
    Node, SnippetBlock, SvelteBody, SvelteBoundary, SvelteDocument, SvelteElement, SvelteWindow,
};

use crate::types::data::NamespaceKind;
use crate::walker::{TemplateVisitor, VisitContext};
use crate::ElementFactsEntry;

pub(crate) struct TemplateSideTablesVisitor<'c> {
    pub component: &'c svelte_ast::Component,
}

fn root_namespace(component: &svelte_ast::Component) -> NamespaceKind {
    match component
        .options
        .as_ref()
        .and_then(|options| options.namespace)
        .unwrap_or(Namespace::Html)
    {
        Namespace::Html => NamespaceKind::Html,
        Namespace::Svg => NamespaceKind::Svg,
        Namespace::Mathml => NamespaceKind::MathMl,
    }
}

fn inherited_namespace(
    component: &svelte_ast::Component,
    ctx: &VisitContext<'_, '_>,
    parent_element: Option<svelte_ast::NodeId>,
) -> NamespaceKind {
    parent_element
        .and_then(|id| ctx.data.namespace(id))
        .unwrap_or_else(|| root_namespace(component))
}

fn namespace_for_element(name: &str, inherited: NamespaceKind) -> NamespaceKind {
    if name == "foreignObject" && inherited == NamespaceKind::Svg {
        return NamespaceKind::ForeignObject;
    }
    if name == "annotation-xml" && inherited == NamespaceKind::MathMl {
        return NamespaceKind::AnnotationXml;
    }
    if is_svg(name) {
        return NamespaceKind::Svg;
    }
    if is_mathml(name) {
        return NamespaceKind::MathMl;
    }
    if inherited == NamespaceKind::Svg && matches!(name, "a" | "title") {
        return NamespaceKind::Svg;
    }
    NamespaceKind::Html
}

fn creation_namespace_for_element(name: &str, inherited: NamespaceKind) -> Namespace {
    namespace_for_element(name, inherited).creation_namespace()
}

fn static_xmlns_namespace(attrs: &[Attribute], source: &str) -> Option<NamespaceKind> {
    let xmlns = attrs.iter().find_map(|attr| match attr {
        Attribute::StringAttribute(attr) if attr.name == "xmlns" => {
            Some(attr.value_span.source_text(source))
        }
        _ => None,
    })?;

    match xmlns {
        "http://www.w3.org/2000/svg" => Some(NamespaceKind::Svg),
        "http://www.w3.org/1998/Math/MathML" => Some(NamespaceKind::MathMl),
        _ => None,
    }
}

pub(crate) fn collect_fragment_namespaces(
    component: &svelte_ast::Component,
    data: &mut crate::types::data::AnalysisData,
) {
    let root_ns = root_namespace(component).as_namespace();
    collect_fragment_namespaces_in(component.root, None, root_ns, &component.store, data);
}

fn collect_fragment_namespaces_in(
    fragment_id: svelte_ast::FragmentId,
    parent_element: Option<svelte_ast::NodeId>,
    root_ns: svelte_ast::Namespace,
    store: &svelte_ast::AstStore,
    data: &mut crate::types::data::AnalysisData,
) {
    let fragment_ns = fragment_namespace_for(fragment_id, parent_element, root_ns, store, data);
    data.template
        .fragment_namespaces
        .record(fragment_id, fragment_ns);

    let nodes = store.fragment_nodes(fragment_id).to_vec();
    for id in nodes {
        match store.get(id) {
            Node::Element(el) => {
                collect_fragment_namespaces_in(el.fragment, Some(el.id), root_ns, store, data)
            }
            Node::ComponentNode(cn) => {
                collect_fragment_namespaces_in(cn.fragment, parent_element, root_ns, store, data)
            }
            Node::IfBlock(block) => {
                collect_fragment_namespaces_in(
                    block.consequent,
                    parent_element,
                    root_ns,
                    store,
                    data,
                );
                if let Some(alt) = block.alternate {
                    collect_fragment_namespaces_in(alt, parent_element, root_ns, store, data);
                }
            }
            Node::EachBlock(block) => {
                collect_fragment_namespaces_in(block.body, parent_element, root_ns, store, data);
                if let Some(fb) = block.fallback {
                    collect_fragment_namespaces_in(fb, parent_element, root_ns, store, data);
                }
            }
            Node::SnippetBlock(block) => {
                collect_fragment_namespaces_in(block.body, parent_element, root_ns, store, data)
            }
            Node::KeyBlock(block) => {
                collect_fragment_namespaces_in(block.fragment, parent_element, root_ns, store, data)
            }
            Node::SvelteHead(head) => {
                collect_fragment_namespaces_in(head.fragment, None, root_ns, store, data)
            }
            Node::SvelteElement(el) => {
                collect_fragment_namespaces_in(el.fragment, Some(el.id), root_ns, store, data)
            }
            Node::SvelteBoundary(b) => {
                collect_fragment_namespaces_in(b.fragment, parent_element, root_ns, store, data)
            }
            Node::AwaitBlock(block) => {
                if let Some(p) = block.pending {
                    collect_fragment_namespaces_in(p, parent_element, root_ns, store, data);
                }
                if let Some(t) = block.then {
                    collect_fragment_namespaces_in(t, parent_element, root_ns, store, data);
                }
                if let Some(c) = block.catch {
                    collect_fragment_namespaces_in(c, parent_element, root_ns, store, data);
                }
            }
            _ => {}
        }
    }
}

fn fragment_namespace_for(
    fragment_id: svelte_ast::FragmentId,
    parent_element: Option<svelte_ast::NodeId>,
    root_ns: svelte_ast::Namespace,
    store: &svelte_ast::AstStore,
    data: &crate::types::data::AnalysisData,
) -> svelte_ast::Namespace {
    use svelte_ast::FragmentRole;
    let role = store.fragment(fragment_id).role;
    match role {
        FragmentRole::Root => root_ns,
        FragmentRole::SvelteHeadBody
        | FragmentRole::ComponentChildren
        | FragmentRole::NamedSlot => svelte_ast::Namespace::Html,
        _ => parent_element
            .and_then(|el_id| data.namespace(el_id))
            .map(NamespaceKind::as_namespace)
            .unwrap_or(root_ns),
    }
}

pub(crate) fn collect_fragment_facts(
    component: &svelte_ast::Component,
    data: &mut crate::types::data::AnalysisData,
) {
    collect_fragment_facts_in(
        component.root,
        &component.store,
        &component.source,
        &mut data.template.fragment_facts,
    );
}

pub(crate) fn collect_rich_content_facts(
    component: &svelte_ast::Component,
    data: &mut crate::types::data::AnalysisData,
) {
    collect_rich_content_facts_in(
        component.root,
        &component.store,
        &component.source,
        &mut data.template.rich_content_facts,
    );
}

fn collect_fragment_facts_in(
    fragment_id: svelte_ast::FragmentId,
    store: &svelte_ast::AstStore,
    source: &str,
    facts: &mut crate::types::data::FragmentFacts,
) {
    facts.record(
        fragment_id,
        crate::types::data::FragmentFactsEntry::from_fragment(
            store.fragment(fragment_id),
            store,
            source,
        ),
    );

    let nodes = store.fragment_nodes(fragment_id).to_vec();
    for id in nodes {
        match store.get(id) {
            Node::Element(el) => collect_fragment_facts_in(el.fragment, store, source, facts),
            Node::ComponentNode(cn) => {
                let cn_fragment = cn.fragment;
                let slot_frags: Vec<_> = cn.legacy_slots.iter().map(|s| s.fragment).collect();
                collect_fragment_facts_in(cn_fragment, store, source, facts);
                for fid in slot_frags {
                    collect_fragment_facts_in(fid, store, source, facts);
                }
            }
            Node::IfBlock(block) => {
                collect_fragment_facts_in(block.consequent, store, source, facts);
                if let Some(alt) = block.alternate {
                    collect_fragment_facts_in(alt, store, source, facts);
                }
            }
            Node::EachBlock(block) => {
                collect_fragment_facts_in(block.body, store, source, facts);
                if let Some(fallback) = block.fallback {
                    collect_fragment_facts_in(fallback, store, source, facts);
                }
            }
            Node::SnippetBlock(block) => {
                collect_fragment_facts_in(block.body, store, source, facts)
            }
            Node::KeyBlock(block) => {
                collect_fragment_facts_in(block.fragment, store, source, facts)
            }
            Node::SvelteHead(head) => {
                collect_fragment_facts_in(head.fragment, store, source, facts)
            }
            Node::SvelteElement(el) => collect_fragment_facts_in(el.fragment, store, source, facts),
            Node::SvelteBoundary(boundary) => {
                collect_fragment_facts_in(boundary.fragment, store, source, facts)
            }
            Node::AwaitBlock(block) => {
                if let Some(pending) = block.pending {
                    collect_fragment_facts_in(pending, store, source, facts);
                }
                if let Some(then) = block.then {
                    collect_fragment_facts_in(then, store, source, facts);
                }
                if let Some(catch) = block.catch {
                    collect_fragment_facts_in(catch, store, source, facts);
                }
            }
            _ => {}
        }
    }
}

fn collect_rich_content_facts_in(
    fragment_id: svelte_ast::FragmentId,
    store: &svelte_ast::AstStore,
    source: &str,
    facts: &mut crate::types::data::RichContentFacts,
) {
    let nodes = store.fragment_nodes(fragment_id).to_vec();
    for id in nodes {
        match store.get(id) {
            Node::Element(el) => collect_rich_content_facts_in(el.fragment, store, source, facts),
            Node::ComponentNode(cn) => {
                let cn_fragment = cn.fragment;
                let slot_frags: Vec<_> = cn.legacy_slots.iter().map(|s| s.fragment).collect();
                collect_rich_content_facts_in(cn_fragment, store, source, facts);
                for fid in slot_frags {
                    collect_rich_content_facts_in(fid, store, source, facts);
                }
            }
            Node::IfBlock(block) => {
                collect_rich_content_facts_in(block.consequent, store, source, facts);
                if let Some(alt) = block.alternate {
                    collect_rich_content_facts_in(alt, store, source, facts);
                }
            }
            Node::EachBlock(block) => {
                collect_rich_content_facts_in(block.body, store, source, facts);
                if let Some(fallback) = block.fallback {
                    collect_rich_content_facts_in(fallback, store, source, facts);
                }
            }
            Node::SnippetBlock(block) => {
                collect_rich_content_facts_in(block.body, store, source, facts)
            }
            Node::KeyBlock(block) => {
                collect_rich_content_facts_in(block.fragment, store, source, facts)
            }
            Node::SvelteHead(head) => {
                collect_rich_content_facts_in(head.fragment, store, source, facts)
            }
            Node::SvelteElement(el) => {
                collect_rich_content_facts_in(el.fragment, store, source, facts)
            }
            Node::SvelteBoundary(boundary) => {
                collect_rich_content_facts_in(boundary.fragment, store, source, facts)
            }
            Node::AwaitBlock(block) => {
                if let Some(pending) = block.pending {
                    collect_rich_content_facts_in(pending, store, source, facts);
                }
                if let Some(then) = block.then {
                    collect_rich_content_facts_in(then, store, source, facts);
                }
                if let Some(catch) = block.catch {
                    collect_rich_content_facts_in(catch, store, source, facts);
                }
            }
            _ => {}
        }
    }

    facts.record(
        fragment_id,
        crate::types::data::RichContentFactsEntry::new(
            fragment_has_rich_content(
                fragment_id,
                crate::types::data::RichContentParentKind::Select,
                store,
                source,
                facts,
            ),
            fragment_has_rich_content(
                fragment_id,
                crate::types::data::RichContentParentKind::Optgroup,
                store,
                source,
                facts,
            ),
            fragment_has_rich_content(
                fragment_id,
                crate::types::data::RichContentParentKind::Option,
                store,
                source,
                facts,
            ),
        ),
    );
}

fn fragment_has_rich_content(
    fragment_id: svelte_ast::FragmentId,
    parent: crate::types::data::RichContentParentKind,
    store: &svelte_ast::AstStore,
    source: &str,
    facts: &crate::types::data::RichContentFacts,
) -> bool {
    let nodes = store.fragment_nodes(fragment_id);
    for &id in nodes {
        match store.get(id) {
            Node::Comment(_)
            | Node::ConstTag(_)
            | Node::DebugTag(_)
            | Node::ExpressionTag(_)
            | Node::SnippetBlock(_) => {}
            Node::IfBlock(block) => {
                if facts.has_rich_content_by_id(block.consequent, parent)
                    || block
                        .alternate
                        .is_some_and(|alt| facts.has_rich_content_by_id(alt, parent))
                {
                    return true;
                }
            }
            Node::EachBlock(block) => {
                if facts.has_rich_content_by_id(block.body, parent)
                    || block
                        .fallback
                        .is_some_and(|fb| facts.has_rich_content_by_id(fb, parent))
                {
                    return true;
                }
            }
            Node::KeyBlock(block) => {
                if facts.has_rich_content_by_id(block.fragment, parent) {
                    return true;
                }
            }
            Node::AwaitBlock(block) => {
                if block
                    .pending
                    .is_some_and(|p| facts.has_rich_content_by_id(p, parent))
                    || block
                        .then
                        .is_some_and(|t| facts.has_rich_content_by_id(t, parent))
                    || block
                        .catch
                        .is_some_and(|c| facts.has_rich_content_by_id(c, parent))
                {
                    return true;
                }
            }
            Node::SvelteBoundary(boundary) => {
                if facts.has_rich_content_by_id(boundary.fragment, parent) {
                    return true;
                }
            }
            Node::Text(text) => {
                if matches!(
                    parent,
                    crate::types::data::RichContentParentKind::Select
                        | crate::types::data::RichContentParentKind::Optgroup
                ) && !text.raw_value(source).trim().is_empty()
                {
                    return true;
                }
            }
            Node::Element(child_el) => match parent {
                crate::types::data::RichContentParentKind::Select => {
                    if child_el.name != "option" && child_el.name != "optgroup" {
                        return true;
                    }
                }
                crate::types::data::RichContentParentKind::Optgroup => {
                    if child_el.name != "option" {
                        return true;
                    }
                }
                crate::types::data::RichContentParentKind::Option => return true,
            },
            _ => return true,
        }
    }

    false
}

/// Extract the first VariableDeclarator from a parsed statement.
fn declarator_from_stmt_local<'a>(stmt: &'a Statement<'a>) -> Option<&'a VariableDeclarator<'a>> {
    match stmt {
        Statement::VariableDeclaration(decl) => decl.declarations.first(),
        _ => None,
    }
}

impl TemplateVisitor for TemplateSideTablesVisitor<'_> {
    fn visit_text(&mut self, text: &svelte_ast::Text, ctx: &mut VisitContext<'_, '_>) {
        ctx.data
            .template
            .template_topology
            .record_node_parent(text.id, ctx.parent());
    }

    fn visit_expression_tag(
        &mut self,
        tag: &svelte_ast::ExpressionTag,
        ctx: &mut VisitContext<'_, '_>,
    ) {
        ctx.data
            .template
            .template_topology
            .record_node_parent(tag.id, ctx.parent());
    }

    fn visit_render_tag(&mut self, tag: &svelte_ast::RenderTag, ctx: &mut VisitContext<'_, '_>) {
        ctx.data
            .template
            .template_topology
            .record_node_parent(tag.id, ctx.parent());
    }

    fn visit_html_tag(&mut self, tag: &svelte_ast::HtmlTag, ctx: &mut VisitContext<'_, '_>) {
        ctx.data
            .template
            .template_topology
            .record_node_parent(tag.id, ctx.parent());
    }

    fn visit_each_block(&mut self, block: &EachBlock, ctx: &mut VisitContext<'_, '_>) {
        let parsed = ctx.parsed;
        ctx.data
            .template
            .template_topology
            .record_node_parent(block.id, ctx.parent());
        let is_destructured = block
            .context
            .as_ref()
            .and_then(|r| parsed.and_then(|p| p.stmt(r.id())))
            .and_then(declarator_from_stmt_local)
            .is_some_and(|d| !matches!(&d.id, BindingPattern::BindingIdentifier(_)));

        if is_destructured {
            let child_scope = ctx
                .data
                .scoping
                .fragment_scope_by_id(block.body)
                .expect("EachBody scope must exist");
            // $$item is synthetic — no OXC AST node for it
            let _ctx_sym = ctx
                .data
                .scoping
                .add_synthetic_binding(child_scope, "$$item");
            ctx.data.blocks.each_context.mark_destructured(block.id);
        }

        // Index SymbolId is populated in leave_each_block (after dispatch_stmt creates bindings)
    }

    fn visit_if_block(&mut self, block: &svelte_ast::IfBlock, ctx: &mut VisitContext<'_, '_>) {
        ctx.data
            .template
            .template_topology
            .record_node_parent(block.id, ctx.parent());
    }

    fn leave_each_block(&mut self, block: &EachBlock, ctx: &mut VisitContext<'_, '_>) {
        let child_scope = ctx
            .data
            .scoping
            .fragment_scope_by_id(block.body)
            .expect("EachBody scope must exist");

        // Reactivity marking (each_rest / getter / each_non_reactive for context
        // and index) is owned by `reactivity_semantics/builder_v2/contextual.rs`.
        // This pass only records non-reactivity structural indices.

        if let Some(idx_ref) = block.index.as_ref() {
            let idx_name = ctx
                .parsed()
                .and_then(|p| p.stmt(idx_ref.id()))
                .and_then(declarator_from_stmt_local)
                .and_then(|d| d.id.get_binding_identifier())
                .map(|ident| ident.name.as_str());
            if let Some(idx_name) = idx_name {
                if let Some(idx_sym) = ctx.data.scoping.find_binding(child_scope, idx_name) {
                    ctx.data
                        .blocks
                        .each_context
                        .record_index_sym(block.id, idx_sym);
                    // `EACH_INDEX_NON_DYNAMIC` bit stays on `ComponentScoping`
                    // until the dynamism classifier moves out of the scoping layer.
                    if block.key.is_none() {
                        ctx.data.scoping.mark_each_index_non_dynamic(idx_sym);
                    }
                }
            }
        }
    }

    fn leave_snippet_block(&mut self, block: &SnippetBlock, ctx: &mut VisitContext<'_, '_>) {
        ctx.data.template.snippets.local_snippets.push(block.id);
        let name = block.name(&self.component.source);
        if let Some(name_sym) = ctx.data.scoping.find_binding(ctx.scope, name) {
            // `mark_snippet_name` + snippet-param classification are owned by
            // `reactivity_semantics/builder_v2/contextual.rs`. Keep only the
            // template-side name → block id index here.
            ctx.data
                .template
                .snippets
                .snippet_name_symbols
                .insert(name_sym, block.id);
        }
    }

    fn visit_const_tag(&mut self, tag: &ConstTag, ctx: &mut VisitContext<'_, '_>) {
        ctx.data
            .template
            .template_topology
            .record_node_parent(tag.id, ctx.parent());
        // `{@const}` binding names used to be snapshotted here into
        // `ConstTagData::names`; reactivity / codegen now derive leaves
        // from the pre-parsed statement on demand (via block_semantics
        // or a local pattern walk), so no per-tag side-table write is
        // required anymore.
    }

    fn visit_element(&mut self, el: &Element, ctx: &mut VisitContext<'_, '_>) {
        ctx.data
            .template
            .template_topology
            .record_node_parent(el.id, ctx.parent());
        let parent_element = ctx.nearest_element();
        let inherited = inherited_namespace(self.component, ctx, parent_element);
        let facts = ElementFactsEntry::build(
            &el.attributes,
            ctx.source,
            namespace_for_element(&el.name, inherited),
            creation_namespace_for_element(&el.name, inherited),
            is_void(&el.name),
            el.name.contains('-'),
        );
        ctx.data.elements.facts.record_entry(el.id, facts);
        let facts = ctx
            .data
            .elements
            .facts
            .entry(el.id)
            .expect("element facts recorded before template element index");
        ctx.data
            .template
            .template_elements
            .record(el.id, &el.name, facts, parent_element);
    }

    fn visit_component_node(&mut self, cn: &ComponentNode, ctx: &mut VisitContext<'_, '_>) {
        ctx.data
            .template
            .template_topology
            .record_node_parent(cn.id, ctx.parent());
        ctx.data.record_element_facts(
            cn.id,
            ElementFactsEntry::build(
                &cn.attributes,
                ctx.source,
                inherited_namespace(self.component, ctx, ctx.nearest_element()),
                inherited_namespace(self.component, ctx, ctx.nearest_element()).as_namespace(),
                false,
                false,
            ),
        );
    }

    fn visit_svelte_element(&mut self, el: &SvelteElement, ctx: &mut VisitContext<'_, '_>) {
        ctx.data
            .template
            .template_topology
            .record_node_parent(el.id, ctx.parent());
        let parent_element = ctx.nearest_element();
        let namespace = static_xmlns_namespace(&el.attributes, ctx.source)
            .unwrap_or_else(|| inherited_namespace(self.component, ctx, parent_element));
        ctx.data.elements.facts.record_entry(
            el.id,
            ElementFactsEntry::build(
                &el.attributes,
                ctx.source,
                namespace,
                namespace.as_namespace(),
                false,
                false,
            ),
        );
        let facts = ctx
            .data
            .elements
            .facts
            .entry(el.id)
            .expect("svelte:element facts recorded before template element index");
        // Register in TemplateElementIndex with a wildcard tag so the CSS prune pass
        // can match class/ID selectors against it. Type selectors won't match "*".
        ctx.data
            .template
            .template_elements
            .record(el.id, "*", facts, parent_element);
    }

    fn visit_svelte_window(&mut self, el: &SvelteWindow, ctx: &mut VisitContext<'_, '_>) {
        ctx.data
            .template
            .template_topology
            .record_node_parent(el.id, ctx.parent());
        ctx.data.record_element_facts(
            el.id,
            ElementFactsEntry::build(
                &el.attributes,
                ctx.source,
                NamespaceKind::Html,
                Namespace::Html,
                false,
                false,
            ),
        );
    }

    fn visit_svelte_document(&mut self, el: &SvelteDocument, ctx: &mut VisitContext<'_, '_>) {
        ctx.data
            .template
            .template_topology
            .record_node_parent(el.id, ctx.parent());
        ctx.data.record_element_facts(
            el.id,
            ElementFactsEntry::build(
                &el.attributes,
                ctx.source,
                NamespaceKind::Html,
                Namespace::Html,
                false,
                false,
            ),
        );
    }

    fn visit_svelte_body(&mut self, el: &SvelteBody, ctx: &mut VisitContext<'_, '_>) {
        ctx.data
            .template
            .template_topology
            .record_node_parent(el.id, ctx.parent());
        ctx.data.record_element_facts(
            el.id,
            ElementFactsEntry::build(
                &el.attributes,
                ctx.source,
                NamespaceKind::Html,
                Namespace::Html,
                false,
                false,
            ),
        );
    }

    fn visit_svelte_boundary(&mut self, el: &SvelteBoundary, ctx: &mut VisitContext<'_, '_>) {
        ctx.data
            .template
            .template_topology
            .record_node_parent(el.id, ctx.parent());
        ctx.data.record_element_facts(
            el.id,
            ElementFactsEntry::build(
                &el.attributes,
                ctx.source,
                NamespaceKind::Html,
                Namespace::Html,
                false,
                false,
            ),
        );
    }

    fn visit_snippet_block(&mut self, block: &SnippetBlock, ctx: &mut VisitContext<'_, '_>) {
        ctx.data
            .template
            .template_topology
            .record_node_parent(block.id, ctx.parent());
    }

    fn visit_key_block(&mut self, block: &svelte_ast::KeyBlock, ctx: &mut VisitContext<'_, '_>) {
        ctx.data
            .template
            .template_topology
            .record_node_parent(block.id, ctx.parent());
    }

    fn visit_await_block(
        &mut self,
        block: &svelte_ast::AwaitBlock,
        ctx: &mut VisitContext<'_, '_>,
    ) {
        ctx.data
            .template
            .template_topology
            .record_node_parent(block.id, ctx.parent());
    }

    fn visit_attribute(&mut self, attr: &Attribute, ctx: &mut VisitContext<'_, '_>) {
        ctx.data
            .template
            .template_topology
            .record_node_parent(attr.id(), ctx.parent());
    }

    fn visit_expression(
        &mut self,
        node_id: svelte_ast::NodeId,
        _span: svelte_span::Span,
        ctx: &mut VisitContext<'_, '_>,
    ) {
        ctx.data
            .template
            .template_topology
            .record_expr_parent(node_id, ctx.parent());
    }
}

// Reactivity marker visitors (`SnippetParamMarker`, `DestructuredGetterMarker`,
// `EachRestMarker`) are owned by
// `crates/svelte_analyze/src/reactivity_semantics/builder_v2/contextual.rs`.
