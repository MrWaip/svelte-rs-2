use svelte_ast::{is_mathml, is_svg, AstStore, Component, Namespace, Node};

use crate::types::data::AnalysisData;

/// Record which `{@html ...}` tags sit inside an SVG or MathML subtree, so
/// codegen can wrap their innerHTML application accordingly.
pub(crate) fn collect(component: &Component, data: &mut AnalysisData) {
    let initial_svg = component
        .options
        .as_ref()
        .and_then(|o| o.namespace.as_ref())
        == Some(&Namespace::Svg);
    let initial_mathml = component
        .options
        .as_ref()
        .and_then(|o| o.namespace.as_ref())
        == Some(&Namespace::Mathml);

    walk_fragment(
        component.root,
        &component.store,
        initial_svg,
        initial_mathml,
        data,
    );
}

fn walk_fragment(
    fragment_id: svelte_ast::FragmentId,
    store: &AstStore,
    in_svg_ns: bool,
    in_mathml: bool,
    data: &mut AnalysisData,
) {
    let nodes = store.fragment_nodes(fragment_id).to_vec();
    for id in nodes {
        match store.get(id) {
            Node::Element(el) => {
                let child_svg = el.name != "foreignObject" && (is_svg(&el.name) || in_svg_ns);
                let child_mathml =
                    is_mathml(&el.name) || (in_mathml && el.name != "annotation-xml");
                walk_fragment(el.fragment, store, child_svg, child_mathml, data);
            }
            Node::SlotElementLegacy(el) => {
                walk_fragment(el.fragment, store, in_svg_ns, in_mathml, data);
            }
            Node::SvelteFragmentLegacy(el) => {
                walk_fragment(el.fragment, store, in_svg_ns, in_mathml, data);
            }
            Node::ComponentNode(cn) => {
                walk_fragment(cn.fragment, store, false, false, data);
            }
            Node::IfBlock(block) => {
                walk_fragment(block.consequent, store, in_svg_ns, in_mathml, data);
                if let Some(alt) = block.alternate {
                    walk_fragment(alt, store, in_svg_ns, in_mathml, data);
                }
            }
            Node::EachBlock(block) => {
                walk_fragment(block.body, store, in_svg_ns, in_mathml, data);
                if let Some(fb) = block.fallback {
                    walk_fragment(fb, store, in_svg_ns, in_mathml, data);
                }
            }
            Node::SnippetBlock(block) => {
                walk_fragment(block.body, store, false, false, data);
            }
            Node::KeyBlock(block) => {
                walk_fragment(block.fragment, store, in_svg_ns, in_mathml, data);
            }
            Node::SvelteHead(head) => {
                walk_fragment(head.fragment, store, false, false, data);
            }
            Node::SvelteElement(el) => {
                walk_fragment(el.fragment, store, in_svg_ns, in_mathml, data);
            }
            Node::SvelteBoundary(b) => {
                walk_fragment(b.fragment, store, false, false, data);
            }
            Node::AwaitBlock(block) => {
                if let Some(p) = block.pending {
                    walk_fragment(p, store, in_svg_ns, in_mathml, data);
                }
                if let Some(t) = block.then {
                    walk_fragment(t, store, in_svg_ns, in_mathml, data);
                }
                if let Some(c) = block.catch {
                    walk_fragment(c, store, in_svg_ns, in_mathml, data);
                }
            }
            Node::HtmlTag(tag) => {
                if in_svg_ns {
                    data.elements.html_tag_in_svg.insert(tag.id);
                } else if in_mathml {
                    data.elements.html_tag_in_mathml.insert(tag.id);
                }
            }
            Node::SvelteWindow(_)
            | Node::SvelteDocument(_)
            | Node::SvelteBody(_)
            | Node::Text(_)
            | Node::Comment(_)
            | Node::ExpressionTag(_)
            | Node::RenderTag(_)
            | Node::ConstTag(_)
            | Node::DebugTag(_)
            | Node::Error(_) => {}
        }
    }
}
