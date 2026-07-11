use svelte_ast::{
    AstStore, Component, FragmentId, Namespace, Node, is_whitespace_removable_parent,
};

use crate::AnalysisData;

use super::data::{FragmentSemantics, FragmentSemanticsStore, FragmentWhitespace};

struct WsContext {
    preserve: bool,
    svg_text: bool,
    removable: bool,
}

pub(crate) fn build(component: &Component, data: &AnalysisData) -> FragmentSemanticsStore {
    let mut out = FragmentSemanticsStore::new();
    let store = &component.store;
    let root = WsContext {
        preserve: data.script.preserve_whitespace,
        svg_text: false,
        removable: fragment_is_svg(component.root, data)
            || fragment_children_are_svg(component.root, store, data),
    };
    walk(component.root, &root, store, data, &mut out);
    out
}

fn walk(
    fragment_id: FragmentId,
    ctx: &WsContext,
    store: &AstStore,
    data: &AnalysisData,
    out: &mut FragmentSemanticsStore,
) {
    let whitespace = if ctx.preserve {
        FragmentWhitespace::Preserve
    } else if ctx.removable {
        FragmentWhitespace::Remove
    } else {
        FragmentWhitespace::Collapse
    };
    out.record(fragment_id, FragmentSemantics { whitespace });

    for id in store.fragment_nodes(fragment_id).to_vec() {
        match store.get(id) {
            Node::Element(el) => {
                let name = el.name.as_str();
                let child_ns = fragment_is_svg(el.fragment, data);
                let preserve = ctx.preserve || matches!(name, "pre" | "textarea" | "script");
                let svg_text = if name == "foreignObject" {
                    false
                } else if name == "text" && child_ns {
                    true
                } else {
                    ctx.svg_text
                };
                let removable = if name == "foreignObject" {
                    false
                } else if child_ns {
                    name != "text" && !svg_text
                } else {
                    is_whitespace_removable_parent(name)
                };
                walk(
                    el.fragment,
                    &WsContext {
                        preserve,
                        svg_text,
                        removable,
                    },
                    store,
                    data,
                    out,
                );
            }
            Node::SvelteElement(el) => {
                let child_ns = fragment_is_svg(el.fragment, data);
                walk(
                    el.fragment,
                    &WsContext {
                        preserve: ctx.preserve,
                        svg_text: ctx.svg_text,
                        removable: child_ns && !ctx.svg_text,
                    },
                    store,
                    data,
                    out,
                );
            }
            Node::ComponentNode(_) | Node::SvelteComponentLegacy(_) | Node::SvelteSelf(_) => {
                if let Some(view) = store.get(id).as_component_like() {
                    walk_slot(view.fragment, ctx.preserve, store, data, out);
                    for slot in view.legacy_slots {
                        walk_slot(slot.fragment, ctx.preserve, store, data, out);
                    }
                }
            }
            Node::SvelteFragmentLegacy(el) => {
                walk_slot(el.fragment, ctx.preserve, store, data, out)
            }
            Node::SlotElementLegacy(el) => walk(el.fragment, ctx, store, data, out),
            Node::IfBlock(block) => {
                walk_block(block.consequent, ctx, store, data, out);
                if let Some(alt) = block.alternate {
                    walk_block(alt, ctx, store, data, out);
                }
            }
            Node::EachBlock(block) => {
                walk_block(block.body, ctx, store, data, out);
                if let Some(fb) = block.fallback {
                    walk_block(fb, ctx, store, data, out);
                }
            }
            Node::SnippetBlock(block) => walk_block(block.body, ctx, store, data, out),
            Node::KeyBlock(block) => walk_block(block.fragment, ctx, store, data, out),
            Node::AwaitBlock(block) => {
                if let Some(p) = block.pending {
                    walk_block(p, ctx, store, data, out);
                }
                if let Some(t) = block.then {
                    walk_block(t, ctx, store, data, out);
                }
                if let Some(c) = block.catch {
                    walk_block(c, ctx, store, data, out);
                }
            }
            Node::SvelteHead(head) => walk(
                head.fragment,
                &WsContext {
                    preserve: ctx.preserve,
                    svg_text: false,
                    removable: ctx.removable,
                },
                store,
                data,
                out,
            ),
            Node::SvelteBoundary(b) => walk(b.fragment, ctx, store, data, out),
            Node::Text(_)
            | Node::Comment(_)
            | Node::ExpressionTag(_)
            | Node::RenderTag(_)
            | Node::HtmlTag(_)
            | Node::ConstTag(_)
            | Node::DebugTag(_)
            | Node::SvelteWindow(_)
            | Node::SvelteDocument(_)
            | Node::SvelteBody(_)
            | Node::Error(_) => {}
        }
    }
}

fn walk_block(
    fragment_id: FragmentId,
    ctx: &WsContext,
    store: &AstStore,
    data: &AnalysisData,
    out: &mut FragmentSemanticsStore,
) {
    let in_svg =
        fragment_is_svg(fragment_id, data) || fragment_children_are_svg(fragment_id, store, data);
    let removable = ctx.removable || (!ctx.svg_text && in_svg);
    walk(
        fragment_id,
        &WsContext {
            preserve: ctx.preserve,
            svg_text: ctx.svg_text,
            removable,
        },
        store,
        data,
        out,
    );
}

fn walk_slot(
    fragment_id: FragmentId,
    preserve: bool,
    store: &AstStore,
    data: &AnalysisData,
    out: &mut FragmentSemanticsStore,
) {
    let ctx = WsContext {
        preserve,
        svg_text: false,
        removable: fragment_children_are_svg(fragment_id, store, data),
    };
    walk(fragment_id, &ctx, store, data, out);
}

fn fragment_is_svg(fragment_id: FragmentId, data: &AnalysisData) -> bool {
    match data.template.fragment_namespaces.get(fragment_id) {
        Some(Namespace::Svg) => true,
        Some(Namespace::Html) | Some(Namespace::Mathml) | None => false,
    }
}

fn fragment_children_are_svg(
    fragment_id: FragmentId,
    store: &AstStore,
    data: &AnalysisData,
) -> bool {
    let mut saw_svg = false;
    for id in store.fragment_nodes(fragment_id) {
        if !matches!(store.get(*id), Node::Element(_) | Node::SvelteElement(_)) {
            continue;
        }
        match data.creation_namespace(*id) {
            Some(Namespace::Svg) => saw_svg = true,
            Some(Namespace::Html) | Some(Namespace::Mathml) | None => return false,
        }
    }
    saw_svg
}
