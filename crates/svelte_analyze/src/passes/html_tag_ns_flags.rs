use svelte_ast::{is_mathml, is_svg, AstStore, Component, Namespace, Node, NodeId};

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

    walk(
        &component.fragment.nodes,
        &component.store,
        initial_svg,
        initial_mathml,
        data,
    );
}

fn walk(
    nodes: &[NodeId],
    store: &AstStore,
    in_svg_ns: bool,
    in_mathml: bool,
    data: &mut AnalysisData,
) {
    for &id in nodes {
        match store.get(id) {
            Node::Element(el) => {
                // SVG namespace propagates through all elements except
                // <foreignObject>. <text> does NOT reset namespace.
                let child_svg = el.name != "foreignObject" && (is_svg(&el.name) || in_svg_ns);
                // <annotation-xml> resets from MathML to HTML context.
                let child_mathml =
                    is_mathml(&el.name) || (in_mathml && el.name != "annotation-xml");
                walk(&el.fragment.nodes, store, child_svg, child_mathml, data);
            }
            Node::SlotElementLegacy(el) => {
                walk(&el.fragment.nodes, store, in_svg_ns, in_mathml, data);
            }
            Node::SvelteFragmentLegacy(el) => {
                walk(&el.fragment.nodes, store, in_svg_ns, in_mathml, data);
            }
            Node::ComponentNode(cn) => {
                walk(&cn.fragment.nodes, store, false, false, data);
            }
            Node::IfBlock(block) => {
                walk(&block.consequent.nodes, store, in_svg_ns, in_mathml, data);
                if let Some(alt) = &block.alternate {
                    walk(&alt.nodes, store, in_svg_ns, in_mathml, data);
                }
            }
            Node::EachBlock(block) => {
                walk(&block.body.nodes, store, in_svg_ns, in_mathml, data);
                if let Some(fb) = &block.fallback {
                    walk(&fb.nodes, store, in_svg_ns, in_mathml, data);
                }
            }
            Node::SnippetBlock(block) => {
                walk(&block.body.nodes, store, false, false, data);
            }
            Node::KeyBlock(block) => {
                walk(&block.fragment.nodes, store, in_svg_ns, in_mathml, data);
            }
            Node::SvelteHead(head) => {
                walk(&head.fragment.nodes, store, false, false, data);
            }
            Node::SvelteElement(el) => {
                walk(&el.fragment.nodes, store, in_svg_ns, in_mathml, data);
            }
            Node::SvelteBoundary(b) => {
                walk(&b.fragment.nodes, store, false, false, data);
            }
            Node::AwaitBlock(block) => {
                if let Some(ref p) = block.pending {
                    walk(&p.nodes, store, in_svg_ns, in_mathml, data);
                }
                if let Some(ref t) = block.then {
                    walk(&t.nodes, store, in_svg_ns, in_mathml, data);
                }
                if let Some(ref c) = block.catch {
                    walk(&c.nodes, store, in_svg_ns, in_mathml, data);
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
