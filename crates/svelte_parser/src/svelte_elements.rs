use svelte_ast::{
    Attribute, Component, CssMode, CustomElementConfig, Element, Fragment, Namespace, Node,
    SvelteBody, SvelteBoundary, SvelteDocument, SvelteHead, SvelteOptions, SvelteWindow,
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
            .position(|n| n.as_element().is_some_and(|el| el.name == "svelte:options"));

        let Some(idx) = options_idx else {
            return;
        };

        let node = component.fragment.nodes.remove(idx);
        let Node::Element(el) = node else {
            unreachable!();
        };

        // Check for duplicate <svelte:options>
        let has_another = component
            .fragment
            .nodes
            .iter()
            .any(|n| n.as_element().is_some_and(|e| e.name == "svelte:options"));
        if has_another {
            self.recover(Diagnostic::svelte_options_duplicate(el.span));
        }

        // Validate no children
        if !el.fragment.is_empty() {
            self.recover(Diagnostic::svelte_options_no_children(el.span));
        }

        component.options = Some(self.read_svelte_options(&el));
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
        for node in &mut component.fragment.nodes {
            if let Node::Element(el) = node {
                if el.name == "svelte:head" {
                    let head = SvelteHead {
                        id: el.id,
                        span: el.span,
                        fragment: std::mem::replace(&mut el.fragment, Fragment::empty()),
                    };
                    *node = Node::SvelteHead(head);
                }
            }
        }
    }

    /// Convert `<svelte:window>` Element nodes in the root fragment to SvelteWindow nodes.
    pub(crate) fn convert_svelte_window(component: &mut Component) {
        for node in &mut component.fragment.nodes {
            if let Node::Element(el) = node {
                if el.name == "svelte:window" {
                    let window = SvelteWindow {
                        id: el.id,
                        span: el.span,
                        attributes: std::mem::take(&mut el.attributes),
                        fragment: std::mem::replace(&mut el.fragment, Fragment::empty()),
                    };
                    *node = Node::SvelteWindow(window);
                }
            }
        }
    }

    /// Convert `<svelte:document>` Element nodes in the root fragment to SvelteDocument nodes.
    pub(crate) fn convert_svelte_document(component: &mut Component) {
        for node in &mut component.fragment.nodes {
            if let Node::Element(el) = node {
                if el.name == "svelte:document" {
                    let doc = SvelteDocument {
                        id: el.id,
                        span: el.span,
                        attributes: std::mem::take(&mut el.attributes),
                        fragment: std::mem::replace(&mut el.fragment, Fragment::empty()),
                    };
                    *node = Node::SvelteDocument(doc);
                }
            }
        }
    }

    /// Convert `<svelte:body>` Element nodes in the root fragment to SvelteBody nodes.
    pub(crate) fn convert_svelte_body(component: &mut Component) {
        for node in &mut component.fragment.nodes {
            if let Node::Element(el) = node {
                if el.name == "svelte:body" {
                    let body = SvelteBody {
                        id: el.id,
                        span: el.span,
                        attributes: std::mem::take(&mut el.attributes),
                        fragment: std::mem::replace(&mut el.fragment, Fragment::empty()),
                    };
                    *node = Node::SvelteBody(body);
                }
            }
        }
    }

    // -----------------------------------------------------------------------
    // <svelte:element> conversion
    // -----------------------------------------------------------------------

    /// Convert `<svelte:element this={expr}>` Element nodes to SvelteElement nodes.
    /// Unlike svelte:head, these can appear anywhere in the tree, so we walk recursively.
    pub(crate) fn convert_svelte_element(fragment: &mut Fragment) {
        for node in &mut fragment.nodes {
            match node {
                Node::Element(el) if el.name == "svelte:element" => {
                    let (tag_span, static_tag) = Self::extract_this_attribute(&mut el.attributes);
                    let mut svelte_el = svelte_ast::SvelteElement {
                        id: el.id,
                        span: el.span,
                        tag_span,
                        static_tag,
                        attributes: std::mem::take(&mut el.attributes),
                        fragment: std::mem::replace(&mut el.fragment, Fragment::empty()),
                    };
                    Self::convert_svelte_element(&mut svelte_el.fragment);
                    *node = Node::SvelteElement(svelte_el);
                }
                Node::Element(el) => Self::convert_svelte_element(&mut el.fragment),
                Node::ComponentNode(cn) => Self::convert_svelte_element(&mut cn.fragment),
                Node::IfBlock(block) => {
                    Self::convert_svelte_element(&mut block.consequent);
                    if let Some(alt) = &mut block.alternate {
                        Self::convert_svelte_element(alt);
                    }
                }
                Node::EachBlock(block) => {
                    Self::convert_svelte_element(&mut block.body);
                    if let Some(fallback) = &mut block.fallback {
                        Self::convert_svelte_element(fallback);
                    }
                }
                Node::SnippetBlock(block) => Self::convert_svelte_element(&mut block.body),
                Node::KeyBlock(block) => Self::convert_svelte_element(&mut block.fragment),
                Node::SvelteHead(head) => Self::convert_svelte_element(&mut head.fragment),
                Node::SvelteElement(el) => Self::convert_svelte_element(&mut el.fragment),
                Node::SvelteBoundary(b) => Self::convert_svelte_element(&mut b.fragment),
                _ => {}
            }
        }
    }

    /// Convert `<svelte:boundary>` Element nodes to SvelteBoundary nodes.
    /// Recursive — boundary can appear anywhere in the template.
    pub(crate) fn convert_svelte_boundary(fragment: &mut Fragment) {
        for node in &mut fragment.nodes {
            match node {
                Node::Element(el) if el.name == "svelte:boundary" => {
                    let mut boundary = SvelteBoundary {
                        id: el.id,
                        span: el.span,
                        attributes: std::mem::take(&mut el.attributes),
                        fragment: std::mem::replace(&mut el.fragment, Fragment::empty()),
                    };
                    Self::convert_svelte_boundary(&mut boundary.fragment);
                    *node = Node::SvelteBoundary(boundary);
                }
                Node::Element(el) => Self::convert_svelte_boundary(&mut el.fragment),
                Node::ComponentNode(cn) => Self::convert_svelte_boundary(&mut cn.fragment),
                Node::IfBlock(block) => {
                    Self::convert_svelte_boundary(&mut block.consequent);
                    if let Some(alt) = &mut block.alternate {
                        Self::convert_svelte_boundary(alt);
                    }
                }
                Node::EachBlock(block) => {
                    Self::convert_svelte_boundary(&mut block.body);
                    if let Some(fallback) = &mut block.fallback {
                        Self::convert_svelte_boundary(fallback);
                    }
                }
                Node::SnippetBlock(block) => Self::convert_svelte_boundary(&mut block.body),
                Node::KeyBlock(block) => Self::convert_svelte_boundary(&mut block.fragment),
                Node::SvelteHead(head) => Self::convert_svelte_boundary(&mut head.fragment),
                Node::SvelteElement(el) => Self::convert_svelte_boundary(&mut el.fragment),
                Node::SvelteBoundary(b) => Self::convert_svelte_boundary(&mut b.fragment),
                _ => {}
            }
        }
    }
}
