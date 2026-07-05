use oxc_ast::ast::{BindingPattern, Expression, Statement, VariableDeclarator};
use svelte_ast::{
    Attribute, ComponentNode, ConstTag, EachBlock, Element, FragmentId, FragmentRole, HtmlTag,
    Namespace, Node, NodeId, SlotElementLegacy, SnippetBlock, SvelteBody, SvelteBoundary,
    SvelteComponentLegacy, SvelteDocument, SvelteElement, SvelteFragmentLegacy, SvelteWindow,
    is_mathml, is_svg, is_void,
};

use crate::ElementFactsEntry;
use crate::types::data::{
    AnalysisData, FragmentFacts, FragmentFactsEntry, NamespaceKind, RichContentFacts,
    RichContentFactsEntry, RichContentParentKind,
};
use crate::walker::{TemplateVisitor, VisitContext};

pub(crate) type FragmentBuckets = Vec<Option<Vec<NodeId>>>;

fn push_into_bucket(buckets: &mut FragmentBuckets, frag_id: FragmentId, node_id: NodeId) {
    let idx = frag_id.0 as usize;
    if buckets.len() <= idx {
        buckets.resize(idx + 1, None);
    }
    buckets[idx].get_or_insert_default().push(node_id);
}

pub(crate) struct TemplateSideTablesVisitor<'c> {
    pub component: &'c svelte_ast::Component,
    pub(crate) title_buckets: FragmentBuckets,
    pub(crate) expression_tag_buckets: FragmentBuckets,
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
    data: &mut AnalysisData,
) {
    let root_ns = root_namespace(component).as_namespace();
    collect_fragment_namespaces_in(component.root, None, root_ns, &component.store, data);
}

fn collect_fragment_namespaces_in(
    fragment_id: svelte_ast::FragmentId,
    parent_element: Option<svelte_ast::NodeId>,
    root_ns: svelte_ast::Namespace,
    store: &svelte_ast::AstStore,
    data: &mut AnalysisData,
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
            Node::ComponentNode(_) | Node::SvelteComponentLegacy(_) | Node::SvelteSelf(_) => {
                if let Some(view) = store.get(id).as_component_like() {
                    collect_fragment_namespaces_in(
                        view.fragment,
                        parent_element,
                        root_ns,
                        store,
                        data,
                    );
                    for slot in view.legacy_slots {
                        collect_fragment_namespaces_in(slot.fragment, None, root_ns, store, data);
                    }
                }
            }
            Node::SvelteFragmentLegacy(el) => {
                collect_fragment_namespaces_in(el.fragment, None, root_ns, store, data)
            }
            Node::SlotElementLegacy(el) => {
                collect_fragment_namespaces_in(el.fragment, parent_element, root_ns, store, data)
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

pub(crate) fn promote_anchor_namespaces(
    component: &svelte_ast::Component,
    data: &mut AnalysisData,
) {
    promote_anchor_namespaces_in(
        component.root,
        false,
        &component.store,
        &component.source,
        data,
    );
}

fn promote_anchor_namespaces_in(
    fragment_id: svelte_ast::FragmentId,
    has_element_ancestor: bool,
    store: &svelte_ast::AstStore,
    source: &str,
    data: &mut AnalysisData,
) {
    for id in store.fragment_nodes(fragment_id).to_vec() {
        match store.get(id) {
            Node::Element(el) => {
                promote_anchor_namespaces_in(el.fragment, true, store, source, data);
                if el.name == "a"
                    && !has_element_ancestor
                    && data.namespace(id) != Some(NamespaceKind::Svg)
                    && anchor_has_svg_child(el.fragment, store, data)
                {
                    let facts = ElementFactsEntry::build(
                        &el.attributes,
                        source,
                        NamespaceKind::Svg,
                        Namespace::Svg,
                        is_void(&el.name),
                        el.name.contains('-'),
                        el.name == "input",
                    );
                    data.elements.facts.record_entry(id, facts);
                }
            }
            Node::SvelteElement(el) => {
                promote_anchor_namespaces_in(el.fragment, true, store, source, data)
            }
            Node::SvelteHead(head) => {
                promote_anchor_namespaces_in(head.fragment, false, store, source, data)
            }
            Node::ComponentNode(cn) => {
                promote_anchor_namespaces_in(cn.fragment, has_element_ancestor, store, source, data)
            }
            Node::SvelteComponentLegacy(cn) => {
                promote_anchor_namespaces_in(cn.fragment, has_element_ancestor, store, source, data)
            }
            Node::SvelteSelf(cn) => {
                promote_anchor_namespaces_in(cn.fragment, has_element_ancestor, store, source, data)
            }
            Node::SvelteFragmentLegacy(el) => {
                promote_anchor_namespaces_in(el.fragment, has_element_ancestor, store, source, data)
            }
            Node::SlotElementLegacy(el) => {
                promote_anchor_namespaces_in(el.fragment, has_element_ancestor, store, source, data)
            }
            Node::SvelteBoundary(b) => {
                promote_anchor_namespaces_in(b.fragment, has_element_ancestor, store, source, data)
            }
            Node::SnippetBlock(block) => {
                promote_anchor_namespaces_in(block.body, has_element_ancestor, store, source, data)
            }
            Node::KeyBlock(block) => promote_anchor_namespaces_in(
                block.fragment,
                has_element_ancestor,
                store,
                source,
                data,
            ),
            Node::IfBlock(block) => {
                promote_anchor_namespaces_in(
                    block.consequent,
                    has_element_ancestor,
                    store,
                    source,
                    data,
                );
                if let Some(alt) = block.alternate {
                    promote_anchor_namespaces_in(alt, has_element_ancestor, store, source, data);
                }
            }
            Node::EachBlock(block) => {
                promote_anchor_namespaces_in(block.body, has_element_ancestor, store, source, data);
                if let Some(fb) = block.fallback {
                    promote_anchor_namespaces_in(fb, has_element_ancestor, store, source, data);
                }
            }
            Node::AwaitBlock(block) => {
                if let Some(p) = block.pending {
                    promote_anchor_namespaces_in(p, has_element_ancestor, store, source, data);
                }
                if let Some(t) = block.then {
                    promote_anchor_namespaces_in(t, has_element_ancestor, store, source, data);
                }
                if let Some(c) = block.catch {
                    promote_anchor_namespaces_in(c, has_element_ancestor, store, source, data);
                }
            }
            _ => {}
        }
    }
}

fn anchor_has_svg_child(
    fragment_id: svelte_ast::FragmentId,
    store: &svelte_ast::AstStore,
    data: &AnalysisData,
) -> bool {
    store.fragment_nodes(fragment_id).iter().any(|&child_id| {
        matches!(store.get(child_id), Node::Element(child) if child.name != "svg")
            && data.namespace(child_id) == Some(NamespaceKind::Svg)
    })
}

fn fragment_namespace_for(
    fragment_id: svelte_ast::FragmentId,
    parent_element: Option<svelte_ast::NodeId>,
    root_ns: svelte_ast::Namespace,
    store: &svelte_ast::AstStore,
    data: &AnalysisData,
) -> svelte_ast::Namespace {
    use svelte_ast::FragmentRole;
    let role = store.fragment(fragment_id).role;
    let inherited = || {
        parent_element
            .and_then(|el_id| data.namespace(el_id))
            .map(NamespaceKind::as_namespace)
            .unwrap_or(root_ns)
    };
    match role {
        FragmentRole::Root => {
            infer_namespace_from_children(fragment_id, store, data).unwrap_or(root_ns)
        }
        FragmentRole::SvelteHeadBody => svelte_ast::Namespace::Html,
        FragmentRole::ComponentChildren | FragmentRole::NamedSlot => {
            infer_namespace_from_children(fragment_id, store, data)
                .unwrap_or(svelte_ast::Namespace::Html)
        }
        FragmentRole::SvelteElementBody => inherited(),
        FragmentRole::Element => {
            let owner_is_element = store
                .fragment(fragment_id)
                .owner
                .map(|oid| matches!(store.get(oid), Node::Element(_) | Node::SvelteElement(_)))
                .unwrap_or(false);
            if owner_is_element {
                inherited()
            } else {
                infer_namespace_from_children(fragment_id, store, data).unwrap_or_else(inherited)
            }
        }
        _ => infer_namespace_from_children(fragment_id, store, data).unwrap_or_else(inherited),
    }
}

fn infer_namespace_from_children(
    fragment_id: svelte_ast::FragmentId,
    store: &svelte_ast::AstStore,
    data: &AnalysisData,
) -> Option<svelte_ast::Namespace> {
    let mut acc: Option<svelte_ast::Namespace> = None;
    visit_fragment_for_namespace(fragment_id, store, data, &mut acc);
    acc
}

fn visit_fragment_for_namespace(
    fragment_id: svelte_ast::FragmentId,
    store: &svelte_ast::AstStore,
    data: &AnalysisData,
    acc: &mut Option<svelte_ast::Namespace>,
) -> bool {
    for &id in store.fragment_nodes(fragment_id) {
        if !visit_node_for_namespace(id, store, data, acc) {
            return false;
        }
    }
    true
}

fn visit_node_for_namespace(
    id: svelte_ast::NodeId,
    store: &svelte_ast::AstStore,
    data: &AnalysisData,
    acc: &mut Option<svelte_ast::Namespace>,
) -> bool {
    match store.get(id) {
        Node::Element(_) | Node::SvelteElement(_) => {
            let Some(ns) = data
                .creation_namespace(id)
                .or_else(|| data.namespace(id).map(NamespaceKind::as_namespace))
            else {
                return true;
            };
            *acc = match *acc {
                None => Some(ns),
                Some(prev) if prev == ns => Some(prev),
                Some(_) => {
                    *acc = Some(svelte_ast::Namespace::Html);
                    return false;
                }
            };
            true
        }
        Node::IfBlock(block) => {
            if !visit_fragment_for_namespace(block.consequent, store, data, acc) {
                return false;
            }
            if let Some(alt) = block.alternate {
                return visit_fragment_for_namespace(alt, store, data, acc);
            }
            true
        }
        Node::EachBlock(block) => {
            if !visit_fragment_for_namespace(block.body, store, data, acc) {
                return false;
            }
            if let Some(fb) = block.fallback {
                return visit_fragment_for_namespace(fb, store, data, acc);
            }
            true
        }
        Node::AwaitBlock(block) => {
            if let Some(p) = block.pending
                && !visit_fragment_for_namespace(p, store, data, acc)
            {
                return false;
            }
            if let Some(t) = block.then
                && !visit_fragment_for_namespace(t, store, data, acc)
            {
                return false;
            }
            if let Some(c) = block.catch {
                return visit_fragment_for_namespace(c, store, data, acc);
            }
            true
        }
        Node::KeyBlock(block) => visit_fragment_for_namespace(block.fragment, store, data, acc),
        Node::SvelteBoundary(b) => visit_fragment_for_namespace(b.fragment, store, data, acc),
        Node::Text(_)
        | Node::SlotElementLegacy(_)
        | Node::ComponentNode(_)
        | Node::Comment(_)
        | Node::ExpressionTag(_)
        | Node::SnippetBlock(_)
        | Node::RenderTag(_)
        | Node::HtmlTag(_)
        | Node::ConstTag(_)
        | Node::DebugTag(_)
        | Node::SvelteHead(_)
        | Node::SvelteFragmentLegacy(_)
        | Node::SvelteComponentLegacy(_)
        | Node::SvelteWindow(_)
        | Node::SvelteDocument(_)
        | Node::SvelteBody(_)
        | Node::SvelteSelf(_)
        | Node::Error(_) => true,
    }
}

pub(crate) fn collect_fragment_facts(component: &svelte_ast::Component, data: &mut AnalysisData) {
    collect_fragment_facts_in(
        component.root,
        &component.store,
        &component.source,
        &mut data.template.fragment_facts,
    );
}

pub(crate) fn collect_rich_content_facts(
    component: &svelte_ast::Component,
    data: &mut AnalysisData,
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
    facts: &mut FragmentFacts,
) {
    facts.record(
        fragment_id,
        FragmentFactsEntry::from_fragment(store.fragment(fragment_id), store, source),
    );

    let nodes = store.fragment_nodes(fragment_id).to_vec();
    for id in nodes {
        match store.get(id) {
            Node::Element(el) => collect_fragment_facts_in(el.fragment, store, source, facts),
            Node::ComponentNode(_) | Node::SvelteComponentLegacy(_) => {
                if let Some(view) = store.get(id).as_component_like() {
                    let cn_fragment = view.fragment;
                    let slot_frags: Vec<_> = view.legacy_slots.iter().map(|s| s.fragment).collect();
                    collect_fragment_facts_in(cn_fragment, store, source, facts);
                    for fid in slot_frags {
                        collect_fragment_facts_in(fid, store, source, facts);
                    }
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
    facts: &mut RichContentFacts,
) {
    let nodes = store.fragment_nodes(fragment_id).to_vec();
    for id in nodes {
        match store.get(id) {
            Node::Element(el) => collect_rich_content_facts_in(el.fragment, store, source, facts),
            Node::ComponentNode(_) | Node::SvelteComponentLegacy(_) => {
                if let Some(view) = store.get(id).as_component_like() {
                    let cn_fragment = view.fragment;
                    let slot_frags: Vec<_> = view.legacy_slots.iter().map(|s| s.fragment).collect();
                    collect_rich_content_facts_in(cn_fragment, store, source, facts);
                    for fid in slot_frags {
                        collect_rich_content_facts_in(fid, store, source, facts);
                    }
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
        RichContentFactsEntry::new(
            fragment_has_rich_content(
                fragment_id,
                RichContentParentKind::Select,
                store,
                source,
                facts,
            ),
            fragment_has_rich_content(
                fragment_id,
                RichContentParentKind::Optgroup,
                store,
                source,
                facts,
            ),
            fragment_has_rich_content(
                fragment_id,
                RichContentParentKind::Option,
                store,
                source,
                facts,
            ),
        ),
    );
}

fn fragment_has_rich_content(
    fragment_id: svelte_ast::FragmentId,
    parent: RichContentParentKind,
    store: &svelte_ast::AstStore,
    source: &str,
    facts: &RichContentFacts,
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
                    RichContentParentKind::Select | RichContentParentKind::Optgroup
                ) && !text.raw_value(source).trim().is_empty()
                {
                    return true;
                }
            }
            Node::Element(child_el) => match parent {
                RichContentParentKind::Select => {
                    if child_el.name != "option" && child_el.name != "optgroup" {
                        return true;
                    }
                }
                RichContentParentKind::Optgroup => {
                    if child_el.name != "option" {
                        return true;
                    }
                }
                RichContentParentKind::Option => return true,
            },
            _ => return true,
        }
    }

    false
}
fn declarator_from_stmt_local<'a>(stmt: &'a Statement<'a>) -> Option<&'a VariableDeclarator<'a>> {
    match stmt {
        Statement::VariableDeclaration(decl) => decl.declarations.first(),
        _ => None,
    }
}

fn record_component_snippets(
    cn_id: NodeId,
    cn_fragment: FragmentId,
    ctx: &mut VisitContext<'_, '_>,
) {
    let snippets: Vec<NodeId> = ctx
        .store
        .fragment_nodes(cn_fragment)
        .iter()
        .filter_map(|&nid| {
            if let Node::SnippetBlock(s) = ctx.store.get(nid) {
                Some(s.id)
            } else {
                None
            }
        })
        .collect();
    if !snippets.is_empty() {
        ctx.data
            .template
            .snippets
            .component_snippets
            .insert(cn_id, snippets);
    }
}

fn record_legacy_slot_wrappers(
    legacy_slots: &[svelte_ast::LegacySlot],
    ctx: &mut VisitContext<'_, '_>,
) {
    for slot in legacy_slots {
        let nodes = ctx.store.fragment_nodes(slot.fragment);
        let Some(&wrapper_id) = nodes.first() else {
            continue;
        };
        if matches!(ctx.store.get(wrapper_id), Node::SvelteFragmentLegacy(_)) {
            ctx.data
                .elements
                .flags
                .svelte_fragment_slots
                .insert(wrapper_id);
        }
    }
}

fn record_custom_element_slot_name(data: &mut AnalysisData, attrs: &[Attribute], source: &str) {
    if !data.output.is_custom_element_target {
        return;
    }
    let slot_name = legacy_slot_name(attrs, source);
    if data
        .output
        .custom_element_slot_names
        .iter()
        .any(|existing| existing == slot_name)
    {
        return;
    }
    data.output
        .custom_element_slot_names
        .push(slot_name.to_string());
}

fn legacy_slot_name<'a>(attrs: &'a [Attribute], source: &'a str) -> &'a str {
    for attr in attrs {
        if let Attribute::StringAttribute(attr) = attr
            && attr.name == "name"
        {
            return attr.value_span.source_text(source);
        }
    }
    "default"
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
        push_into_bucket(
            &mut self.expression_tag_buckets,
            ctx.current_fragment_id(),
            tag.id,
        );
    }

    fn visit_render_tag(&mut self, tag: &svelte_ast::RenderTag, ctx: &mut VisitContext<'_, '_>) {
        ctx.data
            .template
            .template_topology
            .record_node_parent(tag.id, ctx.parent());
    }

    fn visit_html_tag(&mut self, tag: &HtmlTag, ctx: &mut VisitContext<'_, '_>) {
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
            let _ctx_sym = ctx
                .data
                .scoping
                .add_synthetic_binding(child_scope, "$$item");
        }
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

        if let Some(idx_ref) = block.index.as_ref() {
            let idx_name = ctx
                .parsed()
                .and_then(|p| p.stmt(idx_ref.id()))
                .and_then(declarator_from_stmt_local)
                .and_then(|d| d.id.get_binding_identifier())
                .map(|ident| ident.name.as_str());
            if let Some(idx_name) = idx_name
                && let Some(idx_sym) = ctx.data.scoping.find_binding(child_scope, idx_name)
            {
                let key_is_index = block
                    .key
                    .as_ref()
                    .and_then(|r| ctx.parsed().and_then(|p| p.expr(r.id())))
                    .is_some_and(|expr| {
                        matches!(expr.get_inner_expression(), Expression::Identifier(ident) if ident.name.as_str() == idx_name)
                    });
                if block.key.is_none() || key_is_index {
                    ctx.data.scoping.mark_each_index_non_dynamic(idx_sym);
                }
            }
        }
    }

    fn leave_snippet_block(&mut self, block: &SnippetBlock, ctx: &mut VisitContext<'_, '_>) {
        ctx.data.template.snippets.local_snippets.push(block.id);
        let name = block.name(&self.component.source);
        if let Some(name_sym) = ctx.data.scoping.find_binding(ctx.scope, name) {
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
            el.name == "input",
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
        let frag_id = ctx.current_fragment_id();
        if el.name == "title" && ctx.store.fragment(frag_id).role == FragmentRole::SvelteHeadBody {
            push_into_bucket(&mut self.title_buckets, frag_id, el.id);
        }
    }

    fn visit_slot_element_legacy(
        &mut self,
        el: &SlotElementLegacy,
        ctx: &mut VisitContext<'_, '_>,
    ) {
        ctx.data
            .template
            .template_topology
            .record_node_parent(el.id, ctx.parent());
        record_custom_element_slot_name(ctx.data, &el.attributes, ctx.source);
    }

    fn visit_svelte_fragment_legacy(
        &mut self,
        el: &SvelteFragmentLegacy,
        ctx: &mut VisitContext<'_, '_>,
    ) {
        ctx.data
            .template
            .template_topology
            .record_node_parent(el.id, ctx.parent());
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
                false,
            ),
        );
        record_component_snippets(cn.id, cn.fragment, ctx);
        record_legacy_slot_wrappers(&cn.legacy_slots, ctx);
    }

    fn visit_svelte_component_legacy(
        &mut self,
        cn: &SvelteComponentLegacy,
        ctx: &mut VisitContext<'_, '_>,
    ) {
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
                false,
            ),
        );
        record_component_snippets(cn.id, cn.fragment, ctx);
        record_legacy_slot_wrappers(&cn.legacy_slots, ctx);
    }

    fn visit_svelte_self(&mut self, cn: &svelte_ast::SvelteSelf, ctx: &mut VisitContext<'_, '_>) {
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
                false,
            ),
        );
        record_component_snippets(cn.id, cn.fragment, ctx);
        record_legacy_slot_wrappers(&cn.legacy_slots, ctx);
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
                false,
            ),
        );
        let facts = ctx
            .data
            .elements
            .facts
            .entry(el.id)
            .expect("svelte:element facts recorded before template element index");
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
