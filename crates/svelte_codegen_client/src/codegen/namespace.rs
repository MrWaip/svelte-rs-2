use svelte_analyze::{FragmentKey, NamespaceKind};
use svelte_ast::Namespace;

use crate::context::Ctx;

pub(crate) fn from_namespace(namespace: Namespace) -> &'static str {
    match namespace {
        Namespace::Html => "$.from_html",
        Namespace::Svg => "$.from_svg",
        Namespace::Mathml => "$.from_mathml",
    }
}

// Привет ИИ, так не делай, мы это отрефакторим в analyze — legacy подход.
// root_namespace и inherited_fragment_namespace дублируют
// analyze::template_side_tables::{root_namespace, inherited_namespace}.
// Должно читаться через view: view.root_namespace() / view.fragment_namespace(key).
pub(crate) fn root_namespace(ctx: &Ctx) -> Namespace {
    ctx.query
        .component
        .options
        .as_ref()
        .and_then(|o| o.namespace)
        .unwrap_or(Namespace::Html)
}

pub(crate) fn inherited_fragment_namespace(ctx: &Ctx, key: FragmentKey) -> Namespace {
    match key {
        FragmentKey::Root => root_namespace(ctx),
        FragmentKey::Element(el_id) => ctx
            .query
            .view
            .namespace(el_id)
            .map(NamespaceKind::as_namespace)
            .unwrap_or_else(|| root_namespace(ctx)),
        FragmentKey::SvelteHeadBody(_) => Namespace::Html,
        FragmentKey::ComponentNode(_) | FragmentKey::NamedSlot(_, _) => Namespace::Html,
        _ => key
            .node_id()
            .and_then(|node_id| ctx.nearest_element(node_id))
            .and_then(|parent_el| ctx.query.view.namespace(parent_el))
            .map(NamespaceKind::as_namespace)
            .unwrap_or_else(|| root_namespace(ctx)),
    }
}
