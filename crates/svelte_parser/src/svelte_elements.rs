use svelte_ast::{
    AstStore, Attribute, Component, CssMode, CustomElementConfig, Element, Fragment, Namespace,
    Node, NodeId, SvelteBody, SvelteBoundary, SvelteDocument, SvelteHead, SvelteOptions,
    SvelteWindow,
};
use svelte_diagnostics::Diagnostic;
use svelte_span::Span;

use crate::{validate_custom_element_tag, Parser, TagError};

impl<'a> Parser<'a> {
    // -----------------------------------------------------------------------
    // <svelte:options> extraction
    // -----------------------------------------------------------------------

    pub(crate) fn extract_svelte_options(&mut self, component: &mut Component) {
        let options_idx = component
            .fragment
            .nodes
            .iter()
            .position(|&id| component.store.get(id).as_element().is_some_and(|el| el.name == "svelte:options"));

        let Some(idx) = options_idx else {
            return;
        };

        let node_id = component.fragment.nodes.remove(idx);
        let node = component.store.get(node_id);
        let el = node.as_element().unwrap();

        // Check for duplicate <svelte:options>
        let has_another = component
            .fragment
            .nodes
            .iter()
            .any(|&id| component.store.get(id).as_element().is_some_and(|e| e.name == "svelte:options"));
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
                    let expr_text = ea.expression_span.source_text(self.source).trim();
                    match expr_text {
                        "true" => {
                            self.process_svelte_option_bool(&ea.name, true, el.span, &mut options);
                        }
                        "false" => {
                            self.process_svelte_option_bool(
                                &ea.name, false, el.span, &mut options,
                            );
                        }
                        _ => {
                            // Could be an object expression for customElement
                            if ea.name == "customElement" {
                                self.process_custom_element_expression(
                                    ea.expression_span,
                                    el.span,
                                    &mut options,
                                );
                            } else {
                                self.recover(Diagnostic::svelte_options_invalid_attribute(
                                    el.span,
                                ));
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
        let ids: Vec<NodeId> = component.fragment.nodes.clone();
        for id in ids {
            let node = component.store.get(id);
            if let Some(el) = node.as_element() {
                if el.name == "svelte:head" {
                    let head = SvelteHead {
                        id: el.id,
                        span: el.span,
                        fragment: Fragment::new(el.fragment.nodes.clone()),
                    };
                    component.store.replace(id, Node::SvelteHead(head));
                }
            }
        }
    }

    /// Convert `<svelte:window>` Element nodes in the root fragment to SvelteWindow nodes.
    pub(crate) fn convert_svelte_window(component: &mut Component) {
        let ids: Vec<NodeId> = component.fragment.nodes.clone();
        for id in ids {
            let node = component.store.get(id);
            if let Some(el) = node.as_element() {
                if el.name == "svelte:window" {
                    let window = SvelteWindow {
                        id: el.id,
                        span: el.span,
                        attributes: el.attributes.clone(),
                        fragment: Fragment::new(el.fragment.nodes.clone()),
                    };
                    component.store.replace(id, Node::SvelteWindow(window));
                }
            }
        }
    }

    /// Convert `<svelte:document>` Element nodes in the root fragment to SvelteDocument nodes.
    pub(crate) fn convert_svelte_document(component: &mut Component) {
        let ids: Vec<NodeId> = component.fragment.nodes.clone();
        for id in ids {
            let node = component.store.get(id);
            if let Some(el) = node.as_element() {
                if el.name == "svelte:document" {
                    let doc = SvelteDocument {
                        id: el.id,
                        span: el.span,
                        attributes: el.attributes.clone(),
                        fragment: Fragment::new(el.fragment.nodes.clone()),
                    };
                    component.store.replace(id, Node::SvelteDocument(doc));
                }
            }
        }
    }

    /// Convert `<svelte:body>` Element nodes in the root fragment to SvelteBody nodes.
    pub(crate) fn convert_svelte_body(component: &mut Component) {
        let ids: Vec<NodeId> = component.fragment.nodes.clone();
        for id in ids {
            let node = component.store.get(id);
            if let Some(el) = node.as_element() {
                if el.name == "svelte:body" {
                    let body = SvelteBody {
                        id: el.id,
                        span: el.span,
                        attributes: el.attributes.clone(),
                        fragment: Fragment::new(el.fragment.nodes.clone()),
                    };
                    component.store.replace(id, Node::SvelteBody(body));
                }
            }
        }
    }

    // -----------------------------------------------------------------------
    // <svelte:element> conversion
    // -----------------------------------------------------------------------

    /// Convert `<svelte:element this={expr}>` Element nodes to SvelteElement nodes.
    /// Unlike svelte:head, these can appear anywhere in the tree, so we walk recursively.
    pub(crate) fn convert_svelte_element(store: &mut AstStore, fragment: &Fragment) {
        let ids: Vec<NodeId> = fragment.nodes.clone();
        for id in ids {
            let node = store.get(id);
            match node {
                Node::Element(el) if el.name == "svelte:element" => {
                    let mut attrs = el.attributes.clone();
                    let child_ids = el.fragment.nodes.clone();
                    let span = el.span;
                    let orig_id = el.id;
                    let (tag_span, static_tag) = Self::extract_this_attribute(&mut attrs);
                    let child_fragment = Fragment::new(child_ids);
                    Self::convert_svelte_element(store, &child_fragment);
                    let svelte_el = svelte_ast::SvelteElement {
                        id: orig_id,
                        span,
                        tag_span,
                        static_tag,
                        attributes: attrs,
                        fragment: child_fragment,
                    };
                    store.replace(id, Node::SvelteElement(svelte_el));
                }
                _ => {
                    // Collect child fragments to recurse into
                    let child_fragments = collect_child_fragment_ids(node);
                    for child_frag in child_fragments {
                        Self::convert_svelte_element(store, &child_frag);
                    }
                }
            }
        }
    }

    /// Convert `<svelte:boundary>` Element nodes to SvelteBoundary nodes.
    /// Recursive — boundary can appear anywhere in the template.
    pub(crate) fn convert_svelte_boundary(store: &mut AstStore, fragment: &Fragment) {
        let ids: Vec<NodeId> = fragment.nodes.clone();
        for id in ids {
            let node = store.get(id);
            match node {
                Node::Element(el) if el.name == "svelte:boundary" => {
                    let attrs = el.attributes.clone();
                    let child_ids = el.fragment.nodes.clone();
                    let span = el.span;
                    let orig_id = el.id;
                    let child_fragment = Fragment::new(child_ids);
                    Self::convert_svelte_boundary(store, &child_fragment);
                    let boundary = SvelteBoundary {
                        id: orig_id,
                        span,
                        attributes: attrs,
                        fragment: child_fragment,
                    };
                    store.replace(id, Node::SvelteBoundary(boundary));
                }
                _ => {
                    let child_fragments = collect_child_fragment_ids(node);
                    for child_frag in child_fragments {
                        Self::convert_svelte_boundary(store, &child_frag);
                    }
                }
            }
        }
    }
}

/// Collect child fragments from a node for recursive conversion.
/// Returns owned Fragment copies (cheap — just Vec<NodeId>).
fn collect_child_fragment_ids(node: &Node) -> Vec<Fragment> {
    match node {
        Node::Element(el) => vec![Fragment::new(el.fragment.nodes.clone())],
        Node::ComponentNode(cn) => vec![Fragment::new(cn.fragment.nodes.clone())],
        Node::IfBlock(block) => {
            let mut frags = vec![Fragment::new(block.consequent.nodes.clone())];
            if let Some(alt) = &block.alternate {
                frags.push(Fragment::new(alt.nodes.clone()));
            }
            frags
        }
        Node::EachBlock(block) => {
            let mut frags = vec![Fragment::new(block.body.nodes.clone())];
            if let Some(fb) = &block.fallback {
                frags.push(Fragment::new(fb.nodes.clone()));
            }
            frags
        }
        Node::SnippetBlock(block) => vec![Fragment::new(block.body.nodes.clone())],
        Node::KeyBlock(block) => vec![Fragment::new(block.fragment.nodes.clone())],
        Node::SvelteHead(head) => vec![Fragment::new(head.fragment.nodes.clone())],
        Node::SvelteElement(el) => vec![Fragment::new(el.fragment.nodes.clone())],
        Node::SvelteBoundary(b) => vec![Fragment::new(b.fragment.nodes.clone())],
        _ => vec![],
    }
}
