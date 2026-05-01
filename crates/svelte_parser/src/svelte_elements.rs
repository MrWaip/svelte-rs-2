use svelte_ast::{
    AstStore, Attribute, Component, CssMode, CustomElementConfig, Element, Namespace, Node, NodeId,
    SVELTE_BODY, SVELTE_BOUNDARY, SVELTE_DOCUMENT, SVELTE_ELEMENT, SVELTE_FRAGMENT, SVELTE_HEAD,
    SVELTE_OPTIONS, SVELTE_WINDOW, SlotElementLegacy, SvelteBody, SvelteBoundary, SvelteDocument,
    SvelteFragmentLegacy, SvelteHead, SvelteOptions, SvelteWindow,
};
use svelte_diagnostics::Diagnostic;
use svelte_span::Span;

use crate::{Parser, TagError, validate_custom_element_tag};

impl<'a> Parser<'a> {
    pub(crate) fn validate_root_only_special_elements(&mut self, component: &Component) {
        #[derive(Default)]
        struct Seen {
            head: bool,
            window: bool,
            document: bool,
            body: bool,
        }

        let mut seen = Seen::default();
        let mut current_level = component.root_fragment().nodes.clone();
        let mut next_level = Vec::new();
        let mut at_root = true;

        while !current_level.is_empty() {
            next_level.clear();

            for &id in &current_level {
                let node = component.store.get(id);

                if let Node::Element(el) = node
                    && matches!(
                        el.name.as_str(),
                        "svelte:head" | "svelte:window" | "svelte:document" | "svelte:body"
                    )
                {
                    let already_seen = match el.name.as_str() {
                        "svelte:head" => seen.head,
                        "svelte:window" => seen.window,
                        "svelte:document" => seen.document,
                        "svelte:body" => seen.body,
                        _ => unreachable!("only root-only special elements are tracked here"),
                    };

                    if already_seen {
                        self.recover(Diagnostic::error(
                            svelte_diagnostics::DiagnosticKind::SvelteMetaDuplicate {
                                name: el.name.clone(),
                            },
                            Span::new(el.span.start, el.span.start),
                        ));
                    } else {
                        match el.name.as_str() {
                            "svelte:head" => seen.head = true,
                            "svelte:window" => seen.window = true,
                            "svelte:document" => seen.document = true,
                            "svelte:body" => seen.body = true,
                            _ => {
                                unreachable!("only root-only special elements are tracked here")
                            }
                        }
                    }

                    if !at_root {
                        self.recover(Diagnostic::error(
                            svelte_diagnostics::DiagnosticKind::SvelteMetaInvalidPlacement {
                                name: el.name.clone(),
                            },
                            Span::new(el.span.start, el.span.start),
                        ));
                    }
                }

                extend_child_node_ids(&component.store, node, &mut next_level);
            }

            current_level.clear();
            std::mem::swap(&mut current_level, &mut next_level);
            at_root = false;
        }
    }

    pub(crate) fn extract_svelte_options(&mut self, component: &mut Component) {
        let root_id = component.root;
        let options_idx = component
            .store
            .fragment(root_id)
            .nodes
            .iter()
            .position(|&id| {
                component
                    .store
                    .get(id)
                    .as_element()
                    .is_some_and(|el| el.name == SVELTE_OPTIONS)
            });

        let Some(idx) = options_idx else {
            return;
        };

        let node_id = component.store.fragment_mut(root_id).nodes.remove(idx);
        let node = component.store.get(node_id);
        let el = node
            .as_element()
            .expect("node was found via options_idx — must be an element");

        let has_another = component.fragment_nodes(root_id).iter().any(|&id| {
            component
                .store
                .get(id)
                .as_element()
                .is_some_and(|e| e.name == SVELTE_OPTIONS)
        });
        if has_another {
            self.recover(Diagnostic::svelte_options_duplicate(el.span));
        }

        let body_empty = component.fragment_nodes(el.fragment).is_empty();
        if !body_empty {
            self.recover(Diagnostic::svelte_options_no_children(el.span));
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
                    let value = sa.value_span.source_text(self.source).to_string();
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
                        _ => {
                            if ea.name == "customElement" {
                                self.process_custom_element_expression(
                                    ea.expression.span,
                                    el.span,
                                    &mut options,
                                );
                            } else {
                                self.recover(Diagnostic::svelte_options_invalid_attribute(el.span));
                            }
                        }
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
                            self.recover(Diagnostic::svelte_options_invalid_custom_element_tag(
                                span,
                            ));
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
            self.recover(Diagnostic::svelte_options_invalid_attribute(el_span));
            return;
        }
        options.custom_element = Some(CustomElementConfig::Expression(expression_span));
    }
    pub(crate) fn convert_svelte_head(component: &mut Component) {
        let root_id = component.root;
        let len = component.fragment_nodes(root_id).len();
        for i in 0..len {
            let id = component.fragment_nodes(root_id)[i];
            if component
                .store
                .get(id)
                .as_element()
                .is_none_or(|el| el.name != SVELTE_HEAD)
            {
                continue;
            }
            let Node::Element(el) = component.store.take(id) else {
                unreachable!()
            };
            component.store.fragment_mut(el.fragment).role =
                svelte_ast::FragmentRole::SvelteHeadBody;
            component.store.replace(
                id,
                Node::SvelteHead(SvelteHead {
                    id: el.id,
                    span: el.span,
                    attributes: el.attributes,
                    fragment: el.fragment,
                }),
            );
        }
    }
    pub(crate) fn convert_svelte_window(component: &mut Component) {
        let root_id = component.root;
        let len = component.fragment_nodes(root_id).len();
        for i in 0..len {
            let id = component.fragment_nodes(root_id)[i];
            if component
                .store
                .get(id)
                .as_element()
                .is_none_or(|el| el.name != SVELTE_WINDOW)
            {
                continue;
            }
            let Node::Element(el) = component.store.take(id) else {
                unreachable!()
            };
            component.store.replace(
                id,
                Node::SvelteWindow(SvelteWindow {
                    id: el.id,
                    span: el.span,
                    attributes: el.attributes,
                    fragment: el.fragment,
                }),
            );
        }
    }
    pub(crate) fn convert_svelte_document(component: &mut Component) {
        let root_id = component.root;
        let len = component.fragment_nodes(root_id).len();
        for i in 0..len {
            let id = component.fragment_nodes(root_id)[i];
            if component
                .store
                .get(id)
                .as_element()
                .is_none_or(|el| el.name != SVELTE_DOCUMENT)
            {
                continue;
            }
            let Node::Element(el) = component.store.take(id) else {
                unreachable!()
            };
            component.store.replace(
                id,
                Node::SvelteDocument(SvelteDocument {
                    id: el.id,
                    span: el.span,
                    attributes: el.attributes,
                    fragment: el.fragment,
                }),
            );
        }
    }
    pub(crate) fn convert_svelte_body(component: &mut Component) {
        let root_id = component.root;
        let len = component.fragment_nodes(root_id).len();
        for i in 0..len {
            let id = component.fragment_nodes(root_id)[i];
            if component
                .store
                .get(id)
                .as_element()
                .is_none_or(|el| el.name != SVELTE_BODY)
            {
                continue;
            }
            let Node::Element(el) = component.store.take(id) else {
                unreachable!()
            };
            component.store.replace(
                id,
                Node::SvelteBody(SvelteBody {
                    id: el.id,
                    span: el.span,
                    attributes: el.attributes,
                    fragment: el.fragment,
                }),
            );
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
fn collect_child_fragments(node: &Node, buf: &mut Vec<svelte_ast::FragmentId>) {
    match node {
        Node::Element(el) => buf.push(el.fragment),
        Node::SlotElementLegacy(el) => buf.push(el.fragment),
        Node::ComponentNode(cn) => {
            buf.push(cn.fragment);
            for slot in &cn.legacy_slots {
                buf.push(slot.fragment);
            }
        }
        Node::IfBlock(block) => {
            buf.push(block.consequent);
            if let Some(alt) = block.alternate {
                buf.push(alt);
            }
        }
        Node::EachBlock(block) => {
            buf.push(block.body);
            if let Some(fb) = block.fallback {
                buf.push(fb);
            }
        }
        Node::SnippetBlock(block) => buf.push(block.body),
        Node::KeyBlock(block) => buf.push(block.fragment),
        Node::SvelteWindow(window) => buf.push(window.fragment),
        Node::SvelteDocument(document) => buf.push(document.fragment),
        Node::SvelteBody(body) => buf.push(body.fragment),
        Node::SvelteHead(head) => buf.push(head.fragment),
        Node::SvelteFragmentLegacy(fragment) => buf.push(fragment.fragment),
        Node::SvelteElement(el) => buf.push(el.fragment),
        Node::SvelteComponentLegacy(el) => {
            buf.push(el.fragment);
            for slot in &el.legacy_slots {
                buf.push(slot.fragment);
            }
        }
        Node::SvelteBoundary(b) => buf.push(b.fragment),
        Node::AwaitBlock(block) => {
            if let Some(pending) = block.pending {
                buf.push(pending);
            }
            if let Some(then) = block.then {
                buf.push(then);
            }
            if let Some(catch) = block.catch {
                buf.push(catch);
            }
        }
        Node::Text(_)
        | Node::Comment(_)
        | Node::ExpressionTag(_)
        | Node::RenderTag(_)
        | Node::HtmlTag(_)
        | Node::ConstTag(_)
        | Node::DebugTag(_)
        | Node::Error(_) => {}
    }
}

fn extend_child_node_ids(store: &AstStore, node: &Node, buf: &mut Vec<NodeId>) {
    let mut frags = Vec::new();
    collect_child_fragments(node, &mut frags);
    for fid in frags {
        buf.extend_from_slice(&store.fragment(fid).nodes);
    }
}
