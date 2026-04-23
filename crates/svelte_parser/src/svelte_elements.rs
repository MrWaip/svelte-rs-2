use svelte_ast::{
    AstStore, Attribute, Component, CssMode, CustomElementConfig, Element, Namespace, Node, NodeId,
    SlotElementLegacy, SvelteBody, SvelteBoundary, SvelteDocument, SvelteFragmentLegacy,
    SvelteHead, SvelteOptions, SvelteWindow, SVELTE_BODY, SVELTE_BOUNDARY, SVELTE_DOCUMENT,
    SVELTE_ELEMENT, SVELTE_FRAGMENT, SVELTE_HEAD, SVELTE_OPTIONS, SVELTE_WINDOW,
};
use svelte_diagnostics::Diagnostic;
use svelte_span::Span;

use crate::{validate_custom_element_tag, Parser, TagError};

impl<'a> Parser<'a> {
    pub(crate) fn validate_root_only_special_elements(&mut self, component: &Component) {
        #[derive(Default)]
        struct Seen {
            window: bool,
            document: bool,
            body: bool,
        }

        let mut seen = Seen::default();
        let mut current_level = component.fragment.nodes.clone();
        let mut next_level = Vec::new();
        let mut at_root = true;

        while !current_level.is_empty() {
            next_level.clear();

            for &id in &current_level {
                let node = component.store.get(id);

                if let Node::Element(el) = node {
                    if matches!(
                        el.name.as_str(),
                        "svelte:window" | "svelte:document" | "svelte:body"
                    ) {
                        let already_seen = match el.name.as_str() {
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
                                el.span,
                            ));
                        } else {
                            match el.name.as_str() {
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
                                el.span,
                            ));
                        }
                    }
                }

                extend_child_node_ids(node, &mut next_level);
            }

            current_level.clear();
            std::mem::swap(&mut current_level, &mut next_level);
            at_root = false;
        }
    }

    // -----------------------------------------------------------------------
    // <svelte:options> extraction
    // -----------------------------------------------------------------------

    pub(crate) fn extract_svelte_options(&mut self, component: &mut Component) {
        let options_idx = component.fragment.nodes.iter().position(|&id| {
            component
                .store
                .get(id)
                .as_element()
                .is_some_and(|el| el.name == SVELTE_OPTIONS)
        });

        let Some(idx) = options_idx else {
            return;
        };

        let node_id = component.fragment.nodes.remove(idx);
        let node = component.store.get(node_id);
        let el = node
            .as_element()
            .expect("node was found via options_idx — must be an element");

        // Check for duplicate <svelte:options>
        let has_another = component.fragment.nodes.iter().any(|&id| {
            component
                .store
                .get(id)
                .as_element()
                .is_some_and(|e| e.name == SVELTE_OPTIONS)
        });
        if has_another {
            self.recover(Diagnostic::svelte_options_duplicate(el.span));
        }

        // Validate no children
        if !el.fragment.is_empty() {
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
                            // Could be an object expression for customElement
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
                    // Directives and other non-standard attributes are not allowed
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
            // LEGACY(svelte4): `tag` renamed to `customElement`
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
            // LEGACY(svelte4): `tag` renamed to `customElement`
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

        // `null` is backwards compat from Svelte 4 — just ignore
        if expr_text == "null" {
            return;
        }

        // Must be an object expression
        if !expr_text.starts_with('{') {
            self.recover(Diagnostic::svelte_options_invalid_attribute(el_span));
            return;
        }

        // Store the expression span; full object parsing deferred to analysis
        options.custom_element = Some(CustomElementConfig::Expression(expression_span));
    }

    // -----------------------------------------------------------------------
    // <svelte:head> conversion
    // -----------------------------------------------------------------------

    /// Convert `<svelte:head>` Element nodes in the root fragment to SvelteHead nodes.
    pub(crate) fn convert_svelte_head(component: &mut Component) {
        for i in 0..component.fragment.nodes.len() {
            let id = component.fragment.nodes[i];
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
            let mut fragment = el.fragment;
            fragment.role = svelte_ast::FragmentRole::SvelteHeadBody;
            component.store.replace(
                id,
                Node::SvelteHead(SvelteHead {
                    id: el.id,
                    span: el.span,
                    fragment,
                }),
            );
        }
    }

    /// Convert `<svelte:window>` Element nodes in the root fragment to SvelteWindow nodes.
    pub(crate) fn convert_svelte_window(component: &mut Component) {
        for i in 0..component.fragment.nodes.len() {
            let id = component.fragment.nodes[i];
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

    /// Convert `<svelte:document>` Element nodes in the root fragment to SvelteDocument nodes.
    pub(crate) fn convert_svelte_document(component: &mut Component) {
        for i in 0..component.fragment.nodes.len() {
            let id = component.fragment.nodes[i];
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

    /// Convert `<svelte:body>` Element nodes in the root fragment to SvelteBody nodes.
    pub(crate) fn convert_svelte_body(component: &mut Component) {
        for i in 0..component.fragment.nodes.len() {
            let id = component.fragment.nodes[i];
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

    // -----------------------------------------------------------------------
    // LEGACY(svelte4): <slot> conversion
    // -----------------------------------------------------------------------

    /// Convert legacy `<slot>` Element nodes to SlotElementLegacy nodes.
    /// Recursive because slot elements can appear anywhere in the template tree.
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
                Self::convert_slot_element_legacy(store, &el.fragment.nodes);
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
                extend_child_node_ids(store.get(id), &mut next_level);
            }
        }
        if !next_level.is_empty() {
            Self::convert_slot_element_legacy(store, &next_level);
        }
    }

    // -----------------------------------------------------------------------
    // LEGACY(svelte4): <svelte:fragment> conversion
    // -----------------------------------------------------------------------

    /// Convert legacy `<svelte:fragment>` Element nodes to SvelteFragmentLegacy nodes.
    /// Recursive because fragment wrappers can appear inside component children and blocks.
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
                Self::convert_svelte_fragment_legacy(store, &el.fragment.nodes);
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
                extend_child_node_ids(store.get(id), &mut next_level);
            }
        }
        if !next_level.is_empty() {
            Self::convert_svelte_fragment_legacy(store, &next_level);
        }
    }

    // -----------------------------------------------------------------------
    // <svelte:element> conversion
    // -----------------------------------------------------------------------

    /// Convert `<svelte:element this={expr}>` Element nodes to SvelteElement nodes.
    /// Unlike svelte:head, these can appear anywhere in the tree, so we walk recursively.
    pub(crate) fn convert_svelte_element(store: &mut AstStore, node_ids: &[NodeId]) {
        let mut next_level = Vec::new();
        for &id in node_ids {
            if store
                .get(id)
                .as_element()
                .is_some_and(|el| el.name == SVELTE_ELEMENT)
            {
                let Node::Element(mut el) = store.take(id) else {
                    unreachable!()
                };
                let (tag_span, static_tag) = Self::extract_this_attribute(&mut el.attributes);
                Self::convert_svelte_element(store, &el.fragment.nodes);
                let tag = if static_tag {
                    None
                } else {
                    Some(svelte_ast::ExprRef::new(tag_span))
                };
                store.replace(
                    id,
                    Node::SvelteElement(svelte_ast::SvelteElement {
                        id: el.id,
                        span: el.span,
                        tag_span,
                        tag,
                        static_tag,
                        attributes: el.attributes,
                        fragment: {
                            let mut f = el.fragment;
                            f.role = svelte_ast::FragmentRole::SvelteElementBody;
                            f
                        },
                    }),
                );
            } else {
                extend_child_node_ids(store.get(id), &mut next_level);
            }
        }
        if !next_level.is_empty() {
            Self::convert_svelte_element(store, &next_level);
        }
    }

    /// Convert `<svelte:boundary>` Element nodes to SvelteBoundary nodes.
    /// Recursive — boundary can appear anywhere in the template.
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
                Self::convert_svelte_boundary(store, &el.fragment.nodes);
                store.replace(
                    id,
                    Node::SvelteBoundary(SvelteBoundary {
                        id: el.id,
                        span: el.span,
                        attributes: el.attributes,
                        fragment: {
                            let mut f = el.fragment;
                            f.role = svelte_ast::FragmentRole::SvelteBoundaryBody;
                            f
                        },
                    }),
                );
            } else {
                extend_child_node_ids(store.get(id), &mut next_level);
            }
        }
        if !next_level.is_empty() {
            Self::convert_svelte_boundary(store, &next_level);
        }
    }
}

/// Append all child node IDs from a node's fragments into the buffer.
/// Batches IDs across nodes to minimize allocations during recursive traversal.
fn extend_child_node_ids(node: &Node, buf: &mut Vec<NodeId>) {
    match node {
        Node::Element(el) => buf.extend_from_slice(&el.fragment.nodes),
        Node::SlotElementLegacy(el) => buf.extend_from_slice(&el.fragment.nodes),
        Node::ComponentNode(cn) => {
            buf.extend_from_slice(&cn.fragment.nodes);
            for slot in &cn.legacy_slots {
                buf.extend_from_slice(&slot.fragment.nodes);
            }
        }
        Node::IfBlock(block) => {
            buf.extend_from_slice(&block.consequent.nodes);
            if let Some(alt) = &block.alternate {
                buf.extend_from_slice(&alt.nodes);
            }
        }
        Node::EachBlock(block) => {
            buf.extend_from_slice(&block.body.nodes);
            if let Some(fb) = &block.fallback {
                buf.extend_from_slice(&fb.nodes);
            }
        }
        Node::SnippetBlock(block) => buf.extend_from_slice(&block.body.nodes),
        Node::KeyBlock(block) => buf.extend_from_slice(&block.fragment.nodes),
        Node::SvelteWindow(window) => buf.extend_from_slice(&window.fragment.nodes),
        Node::SvelteDocument(document) => buf.extend_from_slice(&document.fragment.nodes),
        Node::SvelteBody(body) => buf.extend_from_slice(&body.fragment.nodes),
        Node::SvelteHead(head) => buf.extend_from_slice(&head.fragment.nodes),
        Node::SvelteFragmentLegacy(fragment) => buf.extend_from_slice(&fragment.fragment.nodes),
        Node::SvelteElement(el) => buf.extend_from_slice(&el.fragment.nodes),
        Node::SvelteBoundary(b) => buf.extend_from_slice(&b.fragment.nodes),
        Node::AwaitBlock(block) => {
            if let Some(pending) = &block.pending {
                buf.extend_from_slice(&pending.nodes);
            }
            if let Some(then) = &block.then {
                buf.extend_from_slice(&then.nodes);
            }
            if let Some(catch) = &block.catch {
                buf.extend_from_slice(&catch.nodes);
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
