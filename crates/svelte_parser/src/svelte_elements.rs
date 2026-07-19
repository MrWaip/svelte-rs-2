use svelte_ast::{
    AstStore, Attribute, Component, CssMode, CustomElementConfig, Element, Namespace, Node, NodeId,
    SVELTE_BODY, SVELTE_BOUNDARY, SVELTE_DOCUMENT, SVELTE_ELEMENT, SVELTE_FRAGMENT, SVELTE_HEAD,
    SVELTE_OPTIONS, SVELTE_WINDOW, SlotElementLegacy, SvelteBody, SvelteBoundary, SvelteDocument,
    SvelteFragmentLegacy, SvelteHead, SvelteOptions, SvelteWindow,
};
use svelte_diagnostics::Diagnostic;
use svelte_span::Span;

use crate::{Parser, TagError, validate_custom_element_tag};

#[derive(Clone, Copy)]
enum SpecialRootElement {
    Head,
    Window,
    Document,
    Body,
}

fn is_svelte_options(store: &AstStore, id: NodeId) -> bool {
    store
        .get(id)
        .as_element()
        .is_some_and(|el| el.name == SVELTE_OPTIONS)
}

impl<'a> Parser<'a> {
    pub(crate) fn extract_svelte_options(&mut self, component: &mut Component) {
        let root_id = component.root;

        let (idx, has_another) = {
            let mut options = component
                .store
                .fragment(root_id)
                .nodes
                .iter()
                .enumerate()
                .filter(|(_, id)| is_svelte_options(&component.store, **id))
                .map(|(i, _)| i);
            let Some(idx) = options.next() else {
                return;
            };
            (idx, options.next().is_some())
        };

        let node_id = component.store.fragment_mut(root_id).nodes.remove(idx);
        let node = component.store.get(node_id);
        let Some(el) = node.as_element() else {
            return;
        };

        if has_another {
            self.recover(Diagnostic::svelte_options_duplicate(el.span));
        }

        let nodes = component.fragment_nodes(el.fragment);
        if let (Some(&first), Some(&last)) = (nodes.first(), nodes.last()) {
            let start = component.store.get(first).span().start;
            let end = component.store.get(last).span().end;
            self.recover(Diagnostic::error(
                svelte_diagnostics::DiagnosticKind::SvelteMetaInvalidContent {
                    name: SVELTE_OPTIONS.to_string(),
                },
                Span::new(start, end),
            ));
        }

        component.options = Some(self.read_svelte_options(el));
    }

    fn read_svelte_options(&mut self, el: &Element) -> SvelteOptions {
        let mut options = SvelteOptions {
            span: el.span,
            runes: None,
            namespace: None,
            css: None,
            custom_element: None,
            immutable: None,
            accessors: None,
            preserve_whitespace: None,
            attributes: el.attributes.clone(),
        };

        for attr in &el.attributes {
            match attr {
                Attribute::BooleanAttribute(ba) => {
                    self.process_svelte_option_bool(&ba.name, true, el.span, &mut options);
                }
                Attribute::StringAttribute(sa) => {
                    let value = sa.value(self.source).to_string();
                    self.process_svelte_option_string(&sa.name, &value, el.span, &mut options);
                }
                Attribute::ExpressionAttribute(ea) => {
                    let expr_text = ea.expression.span.source_text(self.source).trim();
                    match expr_text {
                        "true" => {
                            self.process_svelte_option_bool(&ea.name, true, el.span, &mut options);
                        }
                        "false" => {
                            self.process_svelte_option_bool(&ea.name, false, el.span, &mut options);
                        }
                        _ => match ea.name.as_str() {
                            "customElement" => {
                                self.process_custom_element_expression(
                                    ea.expression.span,
                                    el.span,
                                    &mut options,
                                );
                            }
                            "namespace" => {
                                self.recover(Diagnostic::svelte_options_invalid_attribute_value(
                                    el.span,
                                    r#""html", "mathml" or "svg""#.into(),
                                ));
                            }
                            _ => {
                                self.recover(Diagnostic::svelte_options_invalid_attribute(el.span));
                            }
                        },
                    }
                }
                _ => {
                    self.recover(Diagnostic::svelte_options_invalid_attribute(el.span));
                }
            }
        }

        options
    }

    fn process_svelte_option_bool(
        &mut self,
        name: &str,
        value: bool,
        span: Span,
        options: &mut SvelteOptions,
    ) {
        match name {
            "runes" => options.runes = Some(value),
            "immutable" => options.immutable = Some(value),
            "accessors" => options.accessors = Some(value),
            "preserveWhitespace" => options.preserve_whitespace = Some(value),
            "namespace" | "css" | "customElement" => {
                self.recover(Diagnostic::svelte_options_invalid_attribute_value(
                    span,
                    "a string value".into(),
                ));
            }
            "tag" => {
                self.recover(Diagnostic::svelte_options_deprecated_tag(span));
            }
            _ => {
                self.recover(Diagnostic::svelte_options_unknown_attribute(
                    span,
                    name.to_string(),
                ));
            }
        }
    }

    fn process_svelte_option_string(
        &mut self,
        name: &str,
        value: &str,
        span: Span,
        options: &mut SvelteOptions,
    ) {
        match name {
            "namespace" => match value {
                "html" => options.namespace = Some(Namespace::Html),
                "svg" | "http://www.w3.org/2000/svg" => options.namespace = Some(Namespace::Svg),
                "mathml" | "http://www.w3.org/1998/Math/MathML" => {
                    options.namespace = Some(Namespace::Mathml)
                }
                _ => {
                    self.recover(Diagnostic::svelte_options_invalid_attribute_value(
                        span,
                        r#""html", "mathml" or "svg""#.into(),
                    ));
                }
            },
            "css" => {
                if value == "injected" {
                    options.css = Some(CssMode::Injected);
                } else {
                    self.recover(Diagnostic::svelte_options_invalid_attribute_value(
                        span,
                        r#""injected""#.into(),
                    ));
                }
            }
            "customElement" => {
                if let Some(tag_err) = validate_custom_element_tag(value) {
                    match tag_err {
                        TagError::Invalid => {
                            self.recover(Diagnostic::svelte_options_invalid_tagname(span));
                        }
                        TagError::Reserved => {
                            self.recover(Diagnostic::svelte_options_reserved_tag_name(span));
                        }
                    }
                } else {
                    options.custom_element = Some(CustomElementConfig::Tag(value.to_string()));
                }
            }
            "runes" | "immutable" | "accessors" | "preserveWhitespace" => {
                self.recover(Diagnostic::svelte_options_invalid_attribute_value(
                    span,
                    "true or false".into(),
                ));
            }
            "tag" => {
                self.recover(Diagnostic::svelte_options_deprecated_tag(span));
            }
            _ => {
                self.recover(Diagnostic::svelte_options_unknown_attribute(
                    span,
                    name.to_string(),
                ));
            }
        }
    }

    fn process_custom_element_expression(
        &mut self,
        expression_span: Span,
        el_span: Span,
        options: &mut SvelteOptions,
    ) {
        let expr_text = expression_span.source_text(self.source).trim();
        if expr_text == "null" {
            return;
        }
        if !expr_text.starts_with('{') {
            self.recover(Diagnostic::svelte_options_invalid_custom_element_tag(
                el_span,
            ));
            return;
        }
        options.custom_element = Some(CustomElementConfig::Expression(expression_span));
    }
    pub(crate) fn convert_special_root_elements(component: &mut Component) {
        let root_id = component.root;
        let len = component.fragment_nodes(root_id).len();
        for i in 0..len {
            let id = component.fragment_nodes(root_id)[i];
            let kind = match component.store.get(id).as_element() {
                Some(el) if el.name == SVELTE_HEAD => SpecialRootElement::Head,
                Some(el) if el.name == SVELTE_WINDOW => SpecialRootElement::Window,
                Some(el) if el.name == SVELTE_DOCUMENT => SpecialRootElement::Document,
                Some(el) if el.name == SVELTE_BODY => SpecialRootElement::Body,
                _ => continue,
            };
            let Node::Element(el) = component.store.take(id) else {
                unreachable!()
            };
            let node = match kind {
                SpecialRootElement::Head => {
                    component.store.fragment_mut(el.fragment).role =
                        svelte_ast::FragmentRole::SvelteHeadBody;
                    Node::SvelteHead(SvelteHead {
                        id: el.id,
                        span: el.span,
                        attributes: el.attributes,
                        fragment: el.fragment,
                    })
                }
                SpecialRootElement::Window => Node::SvelteWindow(SvelteWindow {
                    id: el.id,
                    span: el.span,
                    attributes: el.attributes,
                    fragment: el.fragment,
                }),
                SpecialRootElement::Document => Node::SvelteDocument(SvelteDocument {
                    id: el.id,
                    span: el.span,
                    attributes: el.attributes,
                    fragment: el.fragment,
                }),
                SpecialRootElement::Body => Node::SvelteBody(SvelteBody {
                    id: el.id,
                    span: el.span,
                    attributes: el.attributes,
                    fragment: el.fragment,
                }),
            };
            component.store.replace(id, node);
        }
    }
    pub(crate) fn convert_slot_element_legacy(store: &mut AstStore, node_ids: &[NodeId]) {
        let mut next_level = Vec::new();
        for &id in node_ids {
            if store
                .get(id)
                .as_element()
                .is_some_and(|el| el.name == "slot")
            {
                let Node::Element(el) = store.take(id) else {
                    unreachable!()
                };
                let inner_nodes = store.fragment_nodes(el.fragment).to_vec();
                Self::convert_slot_element_legacy(store, &inner_nodes);
                store.replace(
                    id,
                    Node::SlotElementLegacy(SlotElementLegacy {
                        id: el.id,
                        span: el.span,
                        attributes: el.attributes,
                        fragment: el.fragment,
                    }),
                );
            } else {
                let node = store.get(id);
                extend_child_node_ids(store, node, &mut next_level);
            }
        }
        if !next_level.is_empty() {
            Self::convert_slot_element_legacy(store, &next_level);
        }
    }
    pub(crate) fn convert_svelte_fragment_legacy(store: &mut AstStore, node_ids: &[NodeId]) {
        let mut next_level = Vec::new();
        for &id in node_ids {
            if store
                .get(id)
                .as_element()
                .is_some_and(|el| el.name == SVELTE_FRAGMENT)
            {
                let Node::Element(el) = store.take(id) else {
                    unreachable!()
                };
                let inner_nodes = store.fragment_nodes(el.fragment).to_vec();
                Self::convert_svelte_fragment_legacy(store, &inner_nodes);
                store.replace(
                    id,
                    Node::SvelteFragmentLegacy(SvelteFragmentLegacy {
                        id: el.id,
                        span: el.span,
                        attributes: el.attributes,
                        fragment: el.fragment,
                    }),
                );
            } else {
                let node = store.get(id);
                extend_child_node_ids(store, node, &mut next_level);
            }
        }
        if !next_level.is_empty() {
            Self::convert_svelte_fragment_legacy(store, &next_level);
        }
    }
    pub(crate) fn convert_svelte_element(
        store: &mut AstStore,
        diagnostics: &mut Vec<Diagnostic>,
        node_ids: &[NodeId],
    ) {
        let mut next_level = Vec::new();
        for &id in node_ids {
            if store
                .get(id)
                .as_element()
                .is_some_and(|el| el.name == SVELTE_ELEMENT)
            {
                let el = match store.take(id) {
                    Node::Element(el) => el,
                    other => {
                        diagnostics.push(Diagnostic::error(
                            svelte_diagnostics::DiagnosticKind::InternalError(
                                "convert_svelte_element: expected Element node".into(),
                            ),
                            other.span(),
                        ));
                        store.replace(id, other);
                        continue;
                    }
                };
                let (tag_span, static_tag) = Self::classify_this_attribute(&el.attributes);
                let inner_nodes = store.fragment_nodes(el.fragment).to_vec();
                Self::convert_svelte_element(store, diagnostics, &inner_nodes);
                store.fragment_mut(el.fragment).role = svelte_ast::FragmentRole::SvelteElementBody;
                store.replace(
                    id,
                    Node::SvelteElement(svelte_ast::SvelteElement {
                        id: el.id,
                        span: el.span,
                        tag_span,
                        static_tag,
                        attributes: el.attributes,
                        fragment: el.fragment,
                    }),
                );
            } else {
                let node = store.get(id);
                extend_child_node_ids(store, node, &mut next_level);
            }
        }
        if !next_level.is_empty() {
            Self::convert_svelte_element(store, diagnostics, &next_level);
        }
    }
    pub(crate) fn convert_svelte_boundary(store: &mut AstStore, node_ids: &[NodeId]) {
        let mut next_level = Vec::new();
        for &id in node_ids {
            if store
                .get(id)
                .as_element()
                .is_some_and(|el| el.name == SVELTE_BOUNDARY)
            {
                let Node::Element(el) = store.take(id) else {
                    unreachable!()
                };
                let inner_nodes = store.fragment_nodes(el.fragment).to_vec();
                Self::convert_svelte_boundary(store, &inner_nodes);
                store.fragment_mut(el.fragment).role = svelte_ast::FragmentRole::SvelteBoundaryBody;
                store.replace(
                    id,
                    Node::SvelteBoundary(SvelteBoundary {
                        id: el.id,
                        span: el.span,
                        attributes: el.attributes,
                        fragment: el.fragment,
                    }),
                );
            } else {
                let node = store.get(id);
                extend_child_node_ids(store, node, &mut next_level);
            }
        }
        if !next_level.is_empty() {
            Self::convert_svelte_boundary(store, &next_level);
        }
    }
}
pub(crate) fn for_each_child_fragment(node: &Node, mut f: impl FnMut(svelte_ast::FragmentId)) {
    match node {
        Node::Element(el) => f(el.fragment),
        Node::SlotElementLegacy(el) => f(el.fragment),
        Node::ComponentNode(cn) => {
            f(cn.fragment);
            for slot in &cn.legacy_slots {
                f(slot.fragment);
            }
        }
        Node::IfBlock(block) => {
            f(block.consequent);
            if let Some(alt) = block.alternate {
                f(alt);
            }
        }
        Node::EachBlock(block) => {
            f(block.body);
            if let Some(fb) = block.fallback {
                f(fb);
            }
        }
        Node::SnippetBlock(block) => f(block.body),
        Node::KeyBlock(block) => f(block.fragment),
        Node::SvelteWindow(window) => f(window.fragment),
        Node::SvelteDocument(document) => f(document.fragment),
        Node::SvelteBody(body) => f(body.fragment),
        Node::SvelteHead(head) => f(head.fragment),
        Node::SvelteFragmentLegacy(fragment) => f(fragment.fragment),
        Node::SvelteElement(el) => f(el.fragment),
        Node::SvelteComponentLegacy(el) => {
            f(el.fragment);
            for slot in &el.legacy_slots {
                f(slot.fragment);
            }
        }
        Node::SvelteSelf(el) => {
            f(el.fragment);
            for slot in &el.legacy_slots {
                f(slot.fragment);
            }
        }
        Node::SvelteBoundary(b) => f(b.fragment),
        Node::AwaitBlock(block) => {
            if let Some(pending) = block.pending {
                f(pending);
            }
            if let Some(then) = block.then {
                f(then);
            }
            if let Some(catch) = block.catch {
                f(catch);
            }
        }
        Node::Text(_)
        | Node::Comment(_)
        | Node::ExpressionTag(_)
        | Node::RenderTag(_)
        | Node::HtmlTag(_)
        | Node::ConstTag(_)
        | Node::DeclarationTag(_)
        | Node::DebugTag(_)
        | Node::Error(_) => {}
    }
}

fn extend_child_node_ids(store: &AstStore, node: &Node, buf: &mut Vec<NodeId>) {
    for_each_child_fragment(node, |fid| {
        buf.extend_from_slice(&store.fragment(fid).nodes);
    });
}
