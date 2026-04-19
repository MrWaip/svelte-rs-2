use crate::reactivity_semantics::data::PropDefaultLowering;
use crate::types::script::RuneKind;
use oxc_ast::ast::{BindingPattern, CallExpression, Program, Statement};
use oxc_ast_visit::walk::walk_call_expression;
use oxc_ast_visit::Visit;
use oxc_syntax::node::NodeId as OxcNodeId;
use svelte_ast::{
    Attribute, Component, EachBlock, Element, Fragment, IfBlock, LetDirectiveLegacy, Node, NodeId,
};
use svelte_diagnostics::Diagnostic;
use svelte_span::Span;

use super::*;

fn assert_content_strategy_variant(data: &AnalysisData, key: FragmentKey, variant: &str) {
    let actual = data
        .template
        .fragments
        .content_types
        .get(&key)
        .unwrap_or_else(|| panic!("no content strategy for {:?}", key));
    let actual_variant = match actual {
        ContentStrategy::Empty => "Empty",
        ContentStrategy::Static(_) => "Static",
        ContentStrategy::SingleElement(_) => "SingleElement",
        ContentStrategy::SingleBlock(_) => "SingleBlock",
        ContentStrategy::DynamicText => "DynamicText",
        ContentStrategy::Mixed { .. } => "Mixed",
    };
    assert_eq!(
        actual_variant, variant,
        "expected {:?} to be {}",
        key, variant
    );
}

// -----------------------------------------------------------------------
// Finders — identify nodes by source text (span-based)
// -----------------------------------------------------------------------

fn find_expr_tag(fragment: &Fragment, component: &Component, target: &str) -> Option<NodeId> {
    let store = &component.store;
    for &id in &fragment.nodes {
        match store.get(id) {
            Node::ExpressionTag(t) if component.source_text(t.expression_span) == target => {
                return Some(t.id);
            }
            Node::Element(el) => {
                if let Some(id) = find_expr_tag(&el.fragment, component, target) {
                    return Some(id);
                }
            }
            Node::IfBlock(b) => {
                if let Some(id) = find_expr_tag(&b.consequent, component, target) {
                    return Some(id);
                }
                if let Some(alt) = &b.alternate {
                    if let Some(id) = find_expr_tag(alt, component, target) {
                        return Some(id);
                    }
                }
            }
            Node::EachBlock(b) => {
                if let Some(id) = find_expr_tag(&b.body, component, target) {
                    return Some(id);
                }
            }
            Node::SvelteElement(el) => {
                if let Some(id) = find_expr_tag(&el.fragment, component, target) {
                    return Some(id);
                }
            }
            _ => {}
        }
    }
    None
}

fn find_render_tag(fragment: &Fragment, component: &Component, target: &str) -> Option<NodeId> {
    let store = &component.store;
    for &id in &fragment.nodes {
        match store.get(id) {
            Node::RenderTag(tag) if component.source_text(tag.expression_span) == target => {
                return Some(tag.id);
            }
            Node::Element(el) => {
                if let Some(id) = find_render_tag(&el.fragment, component, target) {
                    return Some(id);
                }
            }
            Node::IfBlock(b) => {
                if let Some(id) = find_render_tag(&b.consequent, component, target) {
                    return Some(id);
                }
                if let Some(alt) = &b.alternate {
                    if let Some(id) = find_render_tag(alt, component, target) {
                        return Some(id);
                    }
                }
            }
            Node::EachBlock(b) => {
                if let Some(id) = find_render_tag(&b.body, component, target) {
                    return Some(id);
                }
            }
            Node::SvelteElement(el) => {
                if let Some(id) = find_render_tag(&el.fragment, component, target) {
                    return Some(id);
                }
            }
            Node::SnippetBlock(block) => {
                if let Some(id) = find_render_tag(&block.body, component, target) {
                    return Some(id);
                }
            }
            _ => {}
        }
    }
    None
}

fn find_element<'a>(
    fragment: &'a Fragment,
    component: &'a Component,
    tag_name: &str,
) -> Option<&'a Element> {
    let store = &component.store;
    for &id in &fragment.nodes {
        match store.get(id) {
            Node::Element(el) if el.name == tag_name => return Some(el),
            Node::ComponentNode(node) => {
                if let Some(found) = find_element(&node.fragment, component, tag_name) {
                    return Some(found);
                }
            }
            Node::SnippetBlock(block) => {
                if let Some(found) = find_element(&block.body, component, tag_name) {
                    return Some(found);
                }
            }
            Node::Element(el) => {
                if let Some(found) = find_element(&el.fragment, component, tag_name) {
                    return Some(found);
                }
            }
            Node::IfBlock(b) => {
                if let Some(found) = find_element(&b.consequent, component, tag_name) {
                    return Some(found);
                }
                if let Some(alt) = &b.alternate {
                    if let Some(found) = find_element(alt, component, tag_name) {
                        return Some(found);
                    }
                }
            }
            Node::EachBlock(b) => {
                if let Some(found) = find_element(&b.body, component, tag_name) {
                    return Some(found);
                }
            }
            Node::SvelteHead(head) => {
                if let Some(found) = find_element(&head.fragment, component, tag_name) {
                    return Some(found);
                }
            }
            Node::SvelteElement(el) => {
                if let Some(found) = find_element(&el.fragment, component, tag_name) {
                    return Some(found);
                }
            }
            _ => {}
        }
    }
    None
}

fn find_nth_element<'a>(
    fragment: &'a Fragment,
    component: &'a Component,
    tag_name: &str,
    target_index: usize,
) -> Option<&'a Element> {
    fn visit<'a>(
        fragment: &'a Fragment,
        component: &'a Component,
        tag_name: &str,
        target_index: usize,
        seen: &mut usize,
    ) -> Option<&'a Element> {
        let store = &component.store;
        for &id in &fragment.nodes {
            match store.get(id) {
                Node::Element(el) if el.name == tag_name => {
                    if *seen == target_index {
                        return Some(el);
                    }
                    *seen += 1;
                    if let Some(found) =
                        visit(&el.fragment, component, tag_name, target_index, seen)
                    {
                        return Some(found);
                    }
                }
                Node::ComponentNode(node) => {
                    if let Some(found) =
                        visit(&node.fragment, component, tag_name, target_index, seen)
                    {
                        return Some(found);
                    }
                }
                Node::SnippetBlock(block) => {
                    if let Some(found) = visit(&block.body, component, tag_name, target_index, seen)
                    {
                        return Some(found);
                    }
                }
                Node::Element(el) => {
                    if let Some(found) =
                        visit(&el.fragment, component, tag_name, target_index, seen)
                    {
                        return Some(found);
                    }
                }
                Node::IfBlock(b) => {
                    if let Some(found) =
                        visit(&b.consequent, component, tag_name, target_index, seen)
                    {
                        return Some(found);
                    }
                    if let Some(alt) = &b.alternate {
                        if let Some(found) = visit(alt, component, tag_name, target_index, seen) {
                            return Some(found);
                        }
                    }
                }
                Node::EachBlock(b) => {
                    if let Some(found) = visit(&b.body, component, tag_name, target_index, seen) {
                        return Some(found);
                    }
                }
                Node::SvelteHead(head) => {
                    if let Some(found) =
                        visit(&head.fragment, component, tag_name, target_index, seen)
                    {
                        return Some(found);
                    }
                }
                Node::SvelteElement(el) => {
                    if let Some(found) =
                        visit(&el.fragment, component, tag_name, target_index, seen)
                    {
                        return Some(found);
                    }
                }
                _ => {}
            }
        }
        None
    }

    let mut seen = 0;
    visit(fragment, component, tag_name, target_index, &mut seen)
}

fn find_component_node_id(
    fragment: &Fragment,
    component: &Component,
    name: &str,
) -> Option<NodeId> {
    let store = &component.store;
    for &id in &fragment.nodes {
        match store.get(id) {
            Node::ComponentNode(node) if node.name == name => return Some(node.id),
            Node::Element(el) => {
                if let Some(found) = find_component_node_id(&el.fragment, component, name) {
                    return Some(found);
                }
            }
            Node::IfBlock(b) => {
                if let Some(found) = find_component_node_id(&b.consequent, component, name) {
                    return Some(found);
                }
                if let Some(alt) = &b.alternate {
                    if let Some(found) = find_component_node_id(alt, component, name) {
                        return Some(found);
                    }
                }
            }
            Node::EachBlock(b) => {
                if let Some(found) = find_component_node_id(&b.body, component, name) {
                    return Some(found);
                }
            }
            _ => {}
        }
    }
    None
}

fn find_let_directive(attrs: &[Attribute]) -> Option<&LetDirectiveLegacy> {
    attrs.iter().find_map(|attr| match attr {
        Attribute::LetDirectiveLegacy(dir) => Some(dir),
        _ => None,
    })
}

fn find_html_tag_id(fragment: &Fragment, component: &Component, expr_text: &str) -> Option<NodeId> {
    let store = &component.store;
    for &id in &fragment.nodes {
        match store.get(id) {
            Node::HtmlTag(tag) if component.source_text(tag.expression_span) == expr_text => {
                return Some(tag.id);
            }
            Node::ComponentNode(node) => {
                if let Some(found) = find_html_tag_id(&node.fragment, component, expr_text) {
                    return Some(found);
                }
            }
            Node::Element(el) => {
                if let Some(found) = find_html_tag_id(&el.fragment, component, expr_text) {
                    return Some(found);
                }
            }
            Node::IfBlock(b) => {
                if let Some(found) = find_html_tag_id(&b.consequent, component, expr_text) {
                    return Some(found);
                }
                if let Some(alt) = &b.alternate {
                    if let Some(found) = find_html_tag_id(alt, component, expr_text) {
                        return Some(found);
                    }
                }
            }
            Node::EachBlock(b) => {
                if let Some(found) = find_html_tag_id(&b.body, component, expr_text) {
                    return Some(found);
                }
            }
            Node::SvelteHead(head) => {
                if let Some(found) = find_html_tag_id(&head.fragment, component, expr_text) {
                    return Some(found);
                }
            }
            Node::SvelteElement(el) => {
                if let Some(found) = find_html_tag_id(&el.fragment, component, expr_text) {
                    return Some(found);
                }
            }
            _ => {}
        }
    }
    None
}

fn find_svelte_head_id(fragment: &Fragment, component: &Component) -> Option<NodeId> {
    let store = &component.store;
    for &id in &fragment.nodes {
        match store.get(id) {
            Node::SvelteHead(head) => return Some(head.id),
            Node::ComponentNode(node) => {
                if let Some(found) = find_svelte_head_id(&node.fragment, component) {
                    return Some(found);
                }
            }
            Node::Element(el) => {
                if let Some(found) = find_svelte_head_id(&el.fragment, component) {
                    return Some(found);
                }
            }
            Node::IfBlock(b) => {
                if let Some(found) = find_svelte_head_id(&b.consequent, component) {
                    return Some(found);
                }
                if let Some(alt) = &b.alternate {
                    if let Some(found) = find_svelte_head_id(alt, component) {
                        return Some(found);
                    }
                }
            }
            Node::EachBlock(b) => {
                if let Some(found) = find_svelte_head_id(&b.body, component) {
                    return Some(found);
                }
            }
            Node::SvelteElement(el) => {
                if let Some(found) = find_svelte_head_id(&el.fragment, component) {
                    return Some(found);
                }
            }
            _ => {}
        }
    }
    None
}

fn find_bind_directive_id(
    fragment: &Fragment,
    component: &Component,
    tag_name: &str,
    bind_name: &str,
) -> Option<NodeId> {
    let store = &component.store;
    for &id in &fragment.nodes {
        match store.get(id) {
            Node::Element(el) => {
                if el.name == tag_name {
                    if let Some(dir_id) = el.attributes.iter().find_map(|attr| match attr {
                        svelte_ast::Attribute::BindDirective(dir) if dir.name == bind_name => {
                            Some(dir.id)
                        }
                        _ => None,
                    }) {
                        return Some(dir_id);
                    }
                }
                if let Some(found) =
                    find_bind_directive_id(&el.fragment, component, tag_name, bind_name)
                {
                    return Some(found);
                }
            }
            Node::ComponentNode(node) if node.name == tag_name => {
                if let Some(dir_id) = node.attributes.iter().find_map(|attr| match attr {
                    svelte_ast::Attribute::BindDirective(dir) if dir.name == bind_name => {
                        Some(dir.id)
                    }
                    _ => None,
                }) {
                    return Some(dir_id);
                }
                if let Some(found) =
                    find_bind_directive_id(&node.fragment, component, tag_name, bind_name)
                {
                    return Some(found);
                }
            }
            Node::IfBlock(b) => {
                if let Some(found) =
                    find_bind_directive_id(&b.consequent, component, tag_name, bind_name)
                {
                    return Some(found);
                }
                if let Some(alt) = &b.alternate {
                    if let Some(found) = find_bind_directive_id(alt, component, tag_name, bind_name)
                    {
                        return Some(found);
                    }
                }
            }
            Node::EachBlock(b) => {
                if let Some(found) = find_bind_directive_id(&b.body, component, tag_name, bind_name)
                {
                    return Some(found);
                }
            }
            Node::SvelteElement(el) if tag_name == "svelte:element" => {
                if let Some(dir_id) = el.attributes.iter().find_map(|attr| match attr {
                    svelte_ast::Attribute::BindDirective(dir) if dir.name == bind_name => {
                        Some(dir.id)
                    }
                    _ => None,
                }) {
                    return Some(dir_id);
                }
                if let Some(found) =
                    find_bind_directive_id(&el.fragment, component, tag_name, bind_name)
                {
                    return Some(found);
                }
            }
            Node::SvelteWindow(node) if tag_name == "svelte:window" => {
                if let Some(dir_id) = node.attributes.iter().find_map(|attr| match attr {
                    svelte_ast::Attribute::BindDirective(dir) if dir.name == bind_name => {
                        Some(dir.id)
                    }
                    _ => None,
                }) {
                    return Some(dir_id);
                }
            }
            Node::SvelteDocument(node) if tag_name == "svelte:document" => {
                if let Some(dir_id) = node.attributes.iter().find_map(|attr| match attr {
                    svelte_ast::Attribute::BindDirective(dir) if dir.name == bind_name => {
                        Some(dir.id)
                    }
                    _ => None,
                }) {
                    return Some(dir_id);
                }
            }
            Node::SvelteBody(node) if tag_name == "svelte:body" => {
                if let Some(dir_id) = node.attributes.iter().find_map(|attr| match attr {
                    svelte_ast::Attribute::BindDirective(dir) if dir.name == bind_name => {
                        Some(dir.id)
                    }
                    _ => None,
                }) {
                    return Some(dir_id);
                }
            }
            _ => {}
        }
    }
    None
}

fn find_attribute_id(
    fragment: &Fragment,
    component: &Component,
    tag_name: &str,
    attr_name: &str,
) -> Option<NodeId> {
    let store = &component.store;
    for &id in &fragment.nodes {
        match store.get(id) {
            Node::Element(el) => {
                if el.name == tag_name {
                    if let Some(attr_id) = el.attributes.iter().find_map(|attr| match attr {
                        Attribute::ExpressionAttribute(a) if a.name == attr_name => Some(a.id),
                        Attribute::ConcatenationAttribute(a) if a.name == attr_name => Some(a.id),
                        Attribute::BooleanAttribute(a) if a.name == attr_name => Some(a.id),
                        Attribute::StringAttribute(a) if a.name == attr_name => Some(a.id),
                        Attribute::BindDirective(a) if a.name == attr_name => Some(a.id),
                        Attribute::ClassDirective(a) if a.name == attr_name => Some(a.id),
                        Attribute::StyleDirective(a) if a.name == attr_name => Some(a.id),
                        _ => None,
                    }) {
                        return Some(attr_id);
                    }
                }
                if let Some(found) = find_attribute_id(&el.fragment, component, tag_name, attr_name)
                {
                    return Some(found);
                }
            }
            Node::IfBlock(b) => {
                if let Some(found) =
                    find_attribute_id(&b.consequent, component, tag_name, attr_name)
                {
                    return Some(found);
                }
                if let Some(alt) = &b.alternate {
                    if let Some(found) = find_attribute_id(alt, component, tag_name, attr_name) {
                        return Some(found);
                    }
                }
            }
            Node::EachBlock(b) => {
                if let Some(found) = find_attribute_id(&b.body, component, tag_name, attr_name) {
                    return Some(found);
                }
            }
            _ => {}
        }
    }
    None
}

fn find_if_block<'a>(
    fragment: &'a Fragment,
    component: &'a Component,
    test_text: &str,
) -> Option<&'a IfBlock> {
    let store = &component.store;
    for &id in &fragment.nodes {
        if let Node::IfBlock(b) = store.get(id) {
            if component.source_text(b.test_span) == test_text {
                return Some(b);
            }
        }
    }
    None
}

fn find_each_block<'a>(
    fragment: &'a Fragment,
    component: &'a Component,
    expr_text: &str,
) -> Option<&'a EachBlock> {
    let store = &component.store;
    for &id in &fragment.nodes {
        if let Node::EachBlock(b) = store.get(id) {
            if component.source_text(b.expression_span) == expr_text {
                return Some(b);
            }
        }
    }
    None
}

fn find_snippet_block<'a>(
    fragment: &'a Fragment,
    component: &'a Component,
    name: &str,
) -> Option<&'a svelte_ast::SnippetBlock> {
    let store = &component.store;
    for &id in &fragment.nodes {
        match store.get(id) {
            Node::SnippetBlock(block) if block.name(&component.source) == name => {
                return Some(block);
            }
            Node::ComponentNode(node) => {
                if let Some(block) = find_snippet_block(&node.fragment, component, name) {
                    return Some(block);
                }
            }
            Node::Element(el) => {
                if let Some(block) = find_snippet_block(&el.fragment, component, name) {
                    return Some(block);
                }
            }
            Node::IfBlock(b) => {
                if let Some(block) = find_snippet_block(&b.consequent, component, name) {
                    return Some(block);
                }
                if let Some(alt) = &b.alternate {
                    if let Some(block) = find_snippet_block(alt, component, name) {
                        return Some(block);
                    }
                }
            }
            Node::EachBlock(b) => {
                if let Some(block) = find_snippet_block(&b.body, component, name) {
                    return Some(block);
                }
            }
            _ => {}
        }
    }
    None
}

// -----------------------------------------------------------------------
// Assertion helpers
// -----------------------------------------------------------------------

fn analyze_source(source: &str) -> (Component, AnalysisData<'static>) {
    let alloc = Box::leak(Box::new(oxc_allocator::Allocator::default()));
    let (component, js_result, parse_diags) = svelte_parser::parse_with_js(alloc, source);
    assert!(
        parse_diags.is_empty(),
        "unexpected parse diagnostics: {parse_diags:?}"
    );
    let (data, _parsed, diags) = analyze(&component, js_result);
    assert!(diags.is_empty(), "unexpected diagnostics: {diags:?}");
    (component, data)
}

fn analyze_source_with_parsed(
    source: &'static str,
) -> (Component, AnalysisData<'static>, ParserResult<'static>) {
    let alloc = Box::leak(Box::new(oxc_allocator::Allocator::default()));
    let (component, js_result, parse_diags) = svelte_parser::parse_with_js(alloc, source);
    assert!(
        parse_diags.is_empty(),
        "unexpected parse diagnostics: {parse_diags:?}"
    );
    let (data, parsed, diags) = analyze(&component, js_result);
    assert!(diags.is_empty(), "unexpected diagnostics: {diags:?}");
    (component, data, parsed)
}

fn analyze_source_with_options(
    source: &str,
    options: AnalyzeOptions,
) -> (Component, AnalysisData<'static>) {
    let alloc = Box::leak(Box::new(oxc_allocator::Allocator::default()));
    let (component, js_result, parse_diags) = svelte_parser::parse_with_js(alloc, source);
    assert!(
        parse_diags.is_empty(),
        "unexpected parse diagnostics: {parse_diags:?}"
    );
    let (data, _parsed, diags) = analyze_with_options(&component, js_result, &options);
    assert!(diags.is_empty(), "unexpected diagnostics: {diags:?}");
    (component, data)
}

fn analyze_source_with_css(source: &str) -> (Component, AnalysisData<'static>) {
    let (component, data, css_pass_diags) = analyze_source_with_css_diags(source);
    assert!(
        css_pass_diags.is_empty(),
        "unexpected css analyze diagnostics: {css_pass_diags:?}"
    );
    (component, data)
}

fn analyze_source_with_css_diags(
    source: &str,
) -> (Component, AnalysisData<'static>, Vec<Diagnostic>) {
    let alloc = Box::leak(Box::new(oxc_allocator::Allocator::default()));
    let (component, js_result, parse_diags) = svelte_parser::parse_with_js(alloc, source);
    assert!(
        parse_diags.is_empty(),
        "unexpected parse diagnostics: {parse_diags:?}"
    );
    let (mut data, parsed, diags) = analyze(&component, js_result);
    assert!(diags.is_empty(), "unexpected diagnostics: {diags:?}");
    let Some((stylesheet, css_diags)) = svelte_parser::parse_css_block(&component) else {
        panic!("expected style block")
    };
    assert!(
        css_diags.is_empty(),
        "unexpected css diagnostics: {css_diags:?}"
    );
    let mut css_pass_diags = Vec::new();
    analyze_css_pass(
        &component,
        &stylesheet,
        &parsed,
        false,
        &mut data,
        &mut css_pass_diags,
    );
    (component, data, css_pass_diags)
}

fn analyze_source_with_diags(
    source: &str,
) -> (
    Component,
    AnalysisData<'static>,
    Vec<svelte_diagnostics::Diagnostic>,
) {
    let alloc = Box::leak(Box::new(oxc_allocator::Allocator::default()));
    let (component, js_result, parse_diags) = svelte_parser::parse_with_js(alloc, source);
    assert!(
        parse_diags.is_empty(),
        "unexpected parse diagnostics: {parse_diags:?}"
    );
    let (data, _parsed, diags) = analyze(&component, js_result);
    (component, data, diags)
}

fn assert_root_content_type(data: &AnalysisData, expected: ContentStrategy) {
    let actual = data
        .template
        .fragments
        .content_types
        .get(&FragmentKey::Root)
        .expect("no root content type");
    assert_eq!(*actual, expected);
}

fn assert_symbol(data: &AnalysisData, name: &str) {
    let root = data.scoping.root_scope_id();
    assert!(
        data.scoping.find_binding(root, name).is_some(),
        "expected symbol '{name}'",
    );
}

fn assert_dynamic_tag(data: &AnalysisData, component: &Component, expr_text: &str) {
    let id = find_expr_tag(&component.fragment, component, expr_text)
        .unwrap_or_else(|| panic!("no ExpressionTag with source '{expr_text}'"));
    assert!(
        data.dynamism.is_dynamic_node(id),
        "expected ExpressionTag '{expr_text}' to be dynamic"
    );
}

fn assert_not_dynamic_tag(data: &AnalysisData, component: &Component, expr_text: &str) {
    let id = find_expr_tag(&component.fragment, component, expr_text)
        .unwrap_or_else(|| panic!("no ExpressionTag with source '{expr_text}'"));
    assert!(
        !data.dynamism.is_dynamic_node(id),
        "expected ExpressionTag '{expr_text}' to NOT be dynamic"
    );
}

fn assert_dynamic_component(data: &AnalysisData, component: &Component, name: &str) {
    let id = find_component_node_id(&component.fragment, component, name)
        .unwrap_or_else(|| panic!("no ComponentNode with name '{name}'"));
    assert!(
        data.dynamism.is_dynamic_component(id),
        "expected ComponentNode '{name}' to be dynamic"
    );
}

fn assert_component_ref_non_source_prop(
    data: &AnalysisData,
    component: &Component,
    name: &str,
    expected_prop_name: &str,
) {
    let id = find_component_node_id(&component.fragment, component, name)
        .unwrap_or_else(|| panic!("no ComponentNode with name '{name}'"));
    let sym_id = data
        .elements
        .flags
        .component_binding_sym(id)
        .unwrap_or_else(|| panic!("expected component binding symbol for '{name}'"));
    use crate::types::data::{DeclarationSemantics, PropDeclarationKind, PropDeclarationSemantics};
    let decl = data.declaration_semantics(data.scoping.symbol_declaration(sym_id));
    assert!(matches!(
        decl,
        DeclarationSemantics::Prop(PropDeclarationSemantics {
            kind: PropDeclarationKind::NonSource,
            ..
        })
    ));
    assert_eq!(data.binding_origin_key(sym_id), Some(expected_prop_name));
}

fn assert_const_tag_owner(data: &AnalysisData, name: &str) {
    let sym_id = data
        .scoping
        .find_binding_in_any_scope(name)
        .unwrap_or_else(|| panic!("no binding '{name}'"));
    let node_id = data.scoping.symbol_declaration(sym_id);
    assert!(
        matches!(
            data.declaration_semantics(node_id),
            crate::types::data::DeclarationSemantics::Const(
                crate::types::data::ConstDeclarationSemantics::ConstTag { .. }
            )
        ),
        "expected '{name}' to have a Const declaration semantic",
    );
}

fn symbol_declaration_semantics(data: &AnalysisData, name: &str) -> DeclarationSemantics {
    let sym_id = data
        .scoping
        .find_binding_in_any_scope(name)
        .unwrap_or_else(|| panic!("missing binding '{name}'"));
    data.declaration_root_for_symbol(sym_id)
        .map(|node_id| data.declaration_semantics(node_id))
        .filter(|semantics| !matches!(semantics, DeclarationSemantics::NonReactive))
        .unwrap_or_else(|| data.declaration_semantics(data.scoping.symbol_declaration(sym_id)))
}

fn script_reference_semantics(
    data: &AnalysisData,
    parsed: &ParserResult<'_>,
    name: &str,
    expected_read: bool,
    expected_write: bool,
    occurrence: usize,
) -> ReferenceSemantics {
    struct RefFinder<'n> {
        target: &'n str,
        ref_ids: Vec<oxc_semantic::ReferenceId>,
    }

    impl<'a> Visit<'a> for RefFinder<'_> {
        fn visit_identifier_reference(&mut self, ident: &oxc_ast::ast::IdentifierReference<'a>) {
            if ident.name != self.target {
                return;
            }
            let Some(ref_id) = ident.reference_id.get() else {
                return;
            };
            self.ref_ids.push(ref_id);
        }
    }

    let instance_program = parsed.program.as_ref().expect("expected instance program");
    let mut finder = RefFinder {
        target: name,
        ref_ids: Vec::new(),
    };
    finder.visit_program(instance_program);

    let matching: Vec<_> = finder
        .ref_ids
        .into_iter()
        .filter(|&ref_id| {
            let reference = data.scoping.get_reference(ref_id);
            reference.is_read() == expected_read && reference.is_write() == expected_write
        })
        .collect();

    let ref_id = *matching.get(occurrence).unwrap_or_else(|| {
        panic!(
            "missing reference '{name}' with flags read={expected_read} write={expected_write} occurrence={occurrence}"
        )
    });
    data.reference_semantics(ref_id)
}

fn assert_dynamic_if_block(data: &AnalysisData, component: &Component, test_text: &str) {
    let block = find_if_block(&component.fragment, component, test_text)
        .unwrap_or_else(|| panic!("no IfBlock with test '{test_text}'"));
    assert!(
        data.dynamism.is_dynamic_node(block.id),
        "expected IfBlock '{test_text}' to be dynamic"
    );
}

fn assert_dynamic_each(data: &AnalysisData, component: &Component, expr_text: &str) {
    let block = find_each_block(&component.fragment, component, expr_text)
        .unwrap_or_else(|| panic!("no EachBlock with expr '{expr_text}'"));
    assert!(
        data.dynamism.is_dynamic_node(block.id),
        "expected EachBlock '{expr_text}' to be dynamic"
    );
}

fn assert_snippet_hoistable(
    data: &AnalysisData,
    component: &Component,
    name: &str,
    expected: bool,
) {
    let block = find_snippet_block(&component.fragment, component, name)
        .unwrap_or_else(|| panic!("no SnippetBlock named '{name}'"));
    assert_eq!(
        data.template.snippets.is_hoistable(block.id),
        expected,
        "unexpected hoistability for snippet '{name}'",
    );
}

fn assert_snippet_param_refs_include(
    data: &AnalysisData,
    component: &Component,
    snippet_name: &str,
    binding_name: &str,
) {
    let block = find_snippet_block(&component.fragment, component, snippet_name)
        .unwrap_or_else(|| panic!("no SnippetBlock named '{snippet_name}'"));
    let root = data.scoping.root_scope_id();
    let sym = data
        .scoping
        .find_binding(root, binding_name)
        .unwrap_or_else(|| panic!("no binding '{binding_name}'"));
    assert!(
        data.snippet_param_ref_symbols(block.id).contains(&sym),
        "expected snippet '{snippet_name}' params to reference '{binding_name}'",
    );
}

fn assert_bind_target_symbol_name(
    data: &AnalysisData,
    component: &Component,
    tag_name: &str,
    bind_name: &str,
    expected_binding_name: &str,
) {
    let dir_id = find_bind_directive_id(&component.fragment, component, tag_name, bind_name)
        .unwrap_or_else(|| panic!("no bind:{bind_name} on <{tag_name}>"));
    let sym = data
        .bind_target_symbol(dir_id)
        .unwrap_or_else(|| panic!("no bind target symbol for bind:{bind_name} on <{tag_name}>"));
    assert_eq!(
        data.scoping.symbol_name(sym),
        expected_binding_name,
        "unexpected bind target symbol for bind:{bind_name} on <{tag_name}>",
    );
}

fn assert_diag_codes(diags: &[Diagnostic], expected: &[&str]) {
    let actual = diags
        .iter()
        .map(|diag| diag.kind.code())
        .collect::<Vec<_>>();
    assert_eq!(actual, expected, "unexpected diagnostics: {diags:?}");
}

fn assert_bind_target_semantics(
    data: &AnalysisData,
    component: &Component,
    tag_name: &str,
    bind_name: &str,
    expected_host: BindHostKind,
    expected_property: BindPropertyKind,
    expected_requires_mutable_target: bool,
) {
    let dir_id = find_bind_directive_id(&component.fragment, component, tag_name, bind_name)
        .unwrap_or_else(|| panic!("no bind:{bind_name} on <{tag_name}>"));
    let semantics = data
        .bind_target_semantics(dir_id)
        .unwrap_or_else(|| panic!("no bind target semantics for bind:{bind_name} on <{tag_name}>"));
    assert_eq!(
        semantics.host(),
        expected_host,
        "unexpected bind host for bind:{bind_name} on <{tag_name}>",
    );
    assert_eq!(
        semantics.property(),
        expected_property,
        "unexpected bind property for bind:{bind_name} on <{tag_name}>",
    );
    assert_eq!(
        semantics.requires_mutable_target(),
        expected_requires_mutable_target,
        "unexpected mutable-target requirement for bind:{bind_name} on <{tag_name}>",
    );
}

fn assert_shorthand_symbol_name(
    data: &AnalysisData,
    component: &Component,
    tag_name: &str,
    attr_name: &str,
    expected_binding_name: &str,
) {
    let el = find_element(&component.fragment, component, tag_name)
        .unwrap_or_else(|| panic!("no element <{tag_name}>"));
    let attr_id = el
        .attributes
        .iter()
        .find_map(|attr| match attr {
            svelte_ast::Attribute::ClassDirective(dir) if dir.name == attr_name => Some(dir.id),
            svelte_ast::Attribute::StyleDirective(dir) if dir.name == attr_name => Some(dir.id),
            _ => None,
        })
        .unwrap_or_else(|| panic!("no shorthand attr '{attr_name}' on <{tag_name}>"));
    let sym = data
        .shorthand_symbol(attr_id)
        .unwrap_or_else(|| panic!("no shorthand symbol for '{attr_name}' on <{tag_name}>"));
    assert_eq!(
        data.scoping.symbol_name(sym),
        expected_binding_name,
        "unexpected shorthand symbol for '{attr_name}' on <{tag_name}>",
    );
}

fn assert_element_content_type(
    data: &AnalysisData,
    component: &Component,
    tag_name: &str,
    expected: ContentStrategy,
) {
    let el = find_element(&component.fragment, component, tag_name)
        .unwrap_or_else(|| panic!("no element <{tag_name}>"));
    let actual = data
        .template
        .fragments
        .content_types
        .get(&FragmentKey::Element(el.id))
        .unwrap_or_else(|| panic!("no content type for <{tag_name}>"));
    assert_eq!(*actual, expected);
}

fn assert_element_needs_ref(
    data: &AnalysisData,
    component: &Component,
    tag_name: &str,
    expected: bool,
) {
    let el = find_element(&component.fragment, component, tag_name)
        .unwrap_or_else(|| panic!("no element <{tag_name}>"));
    assert_eq!(
        data.elements.flags.needs_ref(el.id),
        expected,
        "unexpected needs_ref for <{tag_name}>",
    );
}

fn assert_element_needs_var(
    data: &AnalysisData,
    component: &Component,
    tag_name: &str,
    expected: bool,
) {
    let el = find_element(&component.fragment, component, tag_name)
        .unwrap_or_else(|| panic!("no element <{tag_name}>"));
    assert_eq!(
        data.elements.flags.needs_var(el.id),
        expected,
        "unexpected needs_var for <{tag_name}>",
    );
}

fn assert_nth_element_needs_input_defaults(
    data: &AnalysisData,
    component: &Component,
    tag_name: &str,
    target_index: usize,
    expected: bool,
) {
    let el = find_nth_element(&component.fragment, component, tag_name, target_index)
        .unwrap_or_else(|| panic!("no element <{tag_name}> at index {target_index}"));
    assert_eq!(
        data.elements.flags.needs_input_defaults(el.id),
        expected,
        "unexpected needs_input_defaults for <{tag_name}> at index {target_index}",
    );
}

fn assert_has_dynamic_class_directives(
    data: &AnalysisData,
    component: &Component,
    tag_name: &str,
    expected: bool,
) {
    let el = find_element(&component.fragment, component, tag_name)
        .unwrap_or_else(|| panic!("no element <{tag_name}>"));
    assert_eq!(
        data.elements.flags.has_dynamic_class_directives(el.id),
        expected,
        "unexpected dynamic class-directive state for <{tag_name}>",
    );
}

fn assert_consequent_content_type(
    data: &AnalysisData,
    component: &Component,
    test_text: &str,
    expected: ContentStrategy,
) {
    let block = find_if_block(&component.fragment, component, test_text)
        .unwrap_or_else(|| panic!("no IfBlock with test '{test_text}'"));
    let actual = data
        .template
        .fragments
        .content_types
        .get(&FragmentKey::IfConsequent(block.id))
        .unwrap_or_else(|| panic!("no consequent content type for IfBlock '{test_text}'"));
    assert_eq!(*actual, expected);
}

fn assert_lowered_item_count(data: &AnalysisData, key: FragmentKey, expected_count: usize) {
    let lf = data
        .template
        .fragments
        .lowered
        .get(&key)
        .unwrap_or_else(|| panic!("no lowered fragment for {:?}", key));
    assert_eq!(
        lf.items.len(),
        expected_count,
        "expected {expected_count} items in lowered fragment"
    );
}

fn assert_item_is_text_concat(data: &AnalysisData, key: FragmentKey, index: usize) {
    let lf = data
        .template
        .fragments
        .lowered
        .get(&key)
        .expect("no lowered fragment");
    assert!(
        matches!(lf.items.get(index), Some(FragmentItem::TextConcat { .. })),
        "expected item[{index}] to be TextConcat",
    );
}

fn assert_rune_kind(data: &AnalysisData, name: &str, expected: RuneKind) {
    use crate::types::data::{
        DeclarationSemantics, DerivedDeclarationSemantics, DerivedKind, OptimizedRuneSemantics,
        StateDeclarationSemantics, StateKind,
    };
    let root = data.scoping.root_scope_id();
    let sym_id = data
        .scoping
        .find_binding(root, name)
        .unwrap_or_else(|| panic!("no symbol '{name}'"));
    let decl = data.declaration_semantics(data.scoping.symbol_declaration(sym_id));
    let actual = match decl {
        DeclarationSemantics::State(StateDeclarationSemantics {
            kind: StateKind::State,
            ..
        }) => RuneKind::State,
        DeclarationSemantics::State(StateDeclarationSemantics {
            kind: StateKind::StateRaw,
            ..
        }) => RuneKind::StateRaw,
        DeclarationSemantics::State(StateDeclarationSemantics {
            kind: StateKind::StateEager,
            ..
        }) => RuneKind::StateEager,
        DeclarationSemantics::Derived(DerivedDeclarationSemantics {
            kind: DerivedKind::Derived,
            ..
        }) => RuneKind::Derived,
        DeclarationSemantics::Derived(DerivedDeclarationSemantics {
            kind: DerivedKind::DerivedBy,
            ..
        }) => RuneKind::DerivedBy,
        DeclarationSemantics::OptimizedRune(OptimizedRuneSemantics {
            kind: StateKind::State,
            ..
        }) => RuneKind::State,
        DeclarationSemantics::OptimizedRune(OptimizedRuneSemantics {
            kind: StateKind::StateRaw,
            ..
        }) => RuneKind::StateRaw,
        DeclarationSemantics::OptimizedRune(OptimizedRuneSemantics {
            kind: StateKind::StateEager,
            ..
        }) => RuneKind::StateEager,
        other => panic!("'{name}' is not a rune: {other:?}"),
    };
    assert_eq!(actual, expected, "expected '{name}' to be {expected:?}");
}

fn assert_rune_is_mutated(data: &AnalysisData, name: &str) {
    let root = data.scoping.root_scope_id();
    let sym_id = data
        .scoping
        .find_binding(root, name)
        .unwrap_or_else(|| panic!("no symbol '{name}'"));
    assert!(
        data.scoping.is_mutated(sym_id),
        "expected rune '{name}' to be mutated"
    );
}

fn assert_rune_not_mutated(data: &AnalysisData, name: &str) {
    let root = data.scoping.root_scope_id();
    let sym_id = data
        .scoping
        .find_binding(root, name)
        .unwrap_or_else(|| panic!("no symbol '{name}'"));
    assert!(
        !data.scoping.is_mutated(sym_id),
        "expected rune '{name}' to NOT be mutated"
    );
}

fn find_call_node_id(program: &Program<'_>, source: &str, target: &str) -> Option<OxcNodeId> {
    struct Finder<'s> {
        source: &'s str,
        target: &'s str,
        found: Option<OxcNodeId>,
    }

    impl<'a> Visit<'a> for Finder<'_> {
        fn visit_call_expression(&mut self, call: &CallExpression<'a>) {
            if self.found.is_none() && call.span.source_text(self.source) == self.target {
                self.found = Some(call.node_id());
                return;
            }
            walk_call_expression(self, call);
        }
    }

    let mut finder = Finder {
        source,
        target,
        found: None,
    };
    finder.visit_program(program);
    finder.found
}

fn assert_expr_tag_has_call(data: &AnalysisData, component: &Component, expr_text: &str) {
    let id = find_expr_tag(&component.fragment, component, expr_text)
        .unwrap_or_else(|| panic!("no ExpressionTag with source '{expr_text}'"));
    let info = data
        .expression(id)
        .unwrap_or_else(|| panic!("no ExpressionInfo for '{expr_text}'"));
    assert!(
        info.has_call(),
        "expected ExpressionTag '{expr_text}' to have a call"
    );
}

fn assert_expr_tag_no_call(data: &AnalysisData, component: &Component, expr_text: &str) {
    let id = find_expr_tag(&component.fragment, component, expr_text)
        .unwrap_or_else(|| panic!("no ExpressionTag with source '{expr_text}'"));
    let info = data
        .expression(id)
        .unwrap_or_else(|| panic!("no ExpressionInfo for '{expr_text}'"));
    assert!(
        !info.has_call(),
        "expected ExpressionTag '{expr_text}' to NOT have a call"
    );
}

fn assert_expr_tag_has_store_ref(data: &AnalysisData, component: &Component, expr_text: &str) {
    let id = find_expr_tag(&component.fragment, component, expr_text)
        .unwrap_or_else(|| panic!("no ExpressionTag with source '{expr_text}'"));
    let info = data
        .expression(id)
        .unwrap_or_else(|| panic!("no ExpressionInfo for '{expr_text}'"));
    assert!(
        info.has_store_ref(),
        "expected ExpressionTag '{expr_text}' to have a store reference"
    );
}

fn assert_expr_tag_no_store_ref(data: &AnalysisData, component: &Component, expr_text: &str) {
    let id = find_expr_tag(&component.fragment, component, expr_text)
        .unwrap_or_else(|| panic!("no ExpressionTag with source '{expr_text}'"));
    let info = data
        .expression(id)
        .unwrap_or_else(|| panic!("no ExpressionInfo for '{expr_text}'"));
    assert!(
        !info.has_store_ref(),
        "expected ExpressionTag '{expr_text}' to NOT have a store reference"
    );
}

fn assert_node_expr_role(data: &AnalysisData, node_id: NodeId, expected: ExprRole, label: &str) {
    assert_eq!(
        data.expr_role(node_id),
        Some(expected),
        "expected {label} to have expr role {expected:?}"
    );
}

fn assert_expr_tag_role(
    data: &AnalysisData,
    component: &Component,
    expr_text: &str,
    expected: ExprRole,
) {
    let id = find_expr_tag(&component.fragment, component, expr_text)
        .unwrap_or_else(|| panic!("no ExpressionTag with source '{expr_text}'"));
    assert_node_expr_role(data, id, expected, &format!("ExpressionTag '{expr_text}'"));
}

fn assert_render_tag_role(
    data: &AnalysisData,
    component: &Component,
    expr_text: &str,
    expected: ExprRole,
) {
    let id = find_render_tag(&component.fragment, component, expr_text)
        .unwrap_or_else(|| panic!("no RenderTag with source '{expr_text}'"));
    assert_node_expr_role(data, id, expected, &format!("RenderTag '{expr_text}'"));
}

fn assert_attr_role(
    data: &AnalysisData,
    component: &Component,
    tag_name: &str,
    attr_name: &str,
    expected: ExprRole,
) {
    let id = find_attribute_id(&component.fragment, component, tag_name, attr_name)
        .unwrap_or_else(|| panic!("no attribute '{attr_name}' on <{tag_name}>"));
    assert_node_expr_role(data, id, expected, &format!("<{tag_name}>[{attr_name}]"));
}

fn assert_expr_tag_async_query(
    data: &AnalysisData,
    component: &Component,
    expr_text: &str,
    expected: bool,
) {
    let id = find_expr_tag(&component.fragment, component, expr_text)
        .unwrap_or_else(|| panic!("no ExpressionTag with source '{expr_text}'"));
    assert_eq!(
        data.expr_is_async(id),
        expected,
        "expected AnalysisData::expr_is_async for ExpressionTag '{expr_text}' to be {expected}"
    );
}

// -----------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------

#[test]
fn empty_component() {
    let (_c, data) = analyze_source("");
    assert_root_content_type(&data, ContentStrategy::Empty);
}

#[test]
fn static_text_content() {
    let (_c, data) = analyze_source("Hello world");
    assert_root_content_type(&data, ContentStrategy::Static("Hello world".to_string()));
}

#[test]
fn single_element() {
    let (_c, data) = analyze_source("<div></div>");
    assert_content_strategy_variant(&data, FragmentKey::Root, "SingleElement");
}

#[test]
fn mixed_content() {
    let (_c, data) = analyze_source("<div></div><span></span>");
    assert_root_content_type(
        &data,
        ContentStrategy::Mixed {
            has_elements: true,
            has_blocks: false,
            has_text: false,
        },
    );
}

#[test]
fn dynamic_text_content() {
    let (_c, data) = analyze_source(r#"<script>let count = $state(0); count++;</script>{count}"#);
    assert_root_content_type(&data, ContentStrategy::DynamicText);
}

#[test]
fn dynamic_expr_inside_svelte_element() {
    let (c, data) = analyze_source(
        r#"<script>let { name } = $props();</script><svelte:element this="div">{name}</svelte:element>"#,
    );
    assert_dynamic_tag(&data, &c, "name");
}

#[test]
fn no_rune_no_dynamic() {
    let (c, data) = analyze_source(r#"<script>let count = 0;</script>{count}"#);
    assert_symbol(&data, "count");
    assert_not_dynamic_tag(&data, &c, "count");
}

#[test]
fn lowered_fragment_groups_text_and_expr() {
    let (_c, data) = analyze_source(r#"<script>let x = $state(1);</script>Hello {x} world"#);
    assert_lowered_item_count(&data, FragmentKey::Root, 1);
    assert_item_is_text_concat(&data, FragmentKey::Root, 0);
}

#[test]
fn if_block_test_is_dynamic() {
    let (c, data) = analyze_source(
        r#"<script>let show = $state(true); show = false;</script>{#if show}<p>hi</p>{/if}"#,
    );
    assert_symbol(&data, "show");
    assert_dynamic_if_block(&data, &c, "show");
}

#[test]
fn component_rune_bindings_are_dynamic() {
    let (component, data) = analyze_source(
        r#"<svelte:options runes={true} />
<script>
    import Widget from './Widget.svelte';
    const Derived_1 = $derived(Widget);
</script>

<Derived_1 />

{#if true}
    {@const Const_0 = Widget}
    <Const_0 />
{/if}"#,
    );

    assert_dynamic_component(&data, &component, "Derived_1");
    assert_dynamic_component(&data, &component, "Const_0");
}

#[test]
fn component_prop_binding_uses_props_access_ref() {
    let (component, data) = analyze_source(
        r#"<svelte:options runes={true} />
<script>
    let { Widget } = $props();
</script>

<Widget />"#,
    );

    assert_dynamic_component(&data, &component, "Widget");
    assert_component_ref_non_source_prop(&data, &component, "Widget", "Widget");
}

// — new tests —

#[test]
fn whitespace_only_text_trimmed_at_boundaries() {
    // Leading and trailing whitespace-only text should not appear as items.
    let (_c, data) = analyze_source("\n  <div></div>\n  ");
    assert_lowered_item_count(&data, FragmentKey::Root, 1);
    assert_content_strategy_variant(&data, FragmentKey::Root, "SingleElement");
}

#[test]
fn multiple_lowered_groups() {
    // ExprTag, then Element, then ExprTag → three separate items.
    let (_c, data) = analyze_source("{a} <div></div> {b}");
    assert_lowered_item_count(&data, FragmentKey::Root, 3);
    assert_item_is_text_concat(&data, FragmentKey::Root, 0);
    assert_item_is_text_concat(&data, FragmentKey::Root, 2);
    assert_root_content_type(
        &data,
        ContentStrategy::Mixed {
            has_elements: true,
            has_blocks: false,
            has_text: true,
        },
    );
}

#[test]
fn nested_dynamic_tag_in_element() {
    let (c, data) =
        analyze_source(r#"<script>let count = $state(0); count++;</script><div>{count}</div>"#);
    assert_dynamic_tag(&data, &c, "count");
    assert_element_content_type(&data, &c, "div", ContentStrategy::DynamicText);
}

#[test]
fn each_block_dynamic() {
    let (c, data) = analyze_source(
        r#"<script>let items = $state([]); items = [1];</script>{#each items as item}<p>{item}</p>{/each}"#,
    );
    assert_symbol(&data, "items");
    assert_dynamic_each(&data, &c, "items");
}

#[test]
fn if_block_alternate_content_type() {
    let (c, data) = analyze_source("{#if x}Hello{:else}<span></span>{/if}");
    assert_consequent_content_type(&data, &c, "x", ContentStrategy::Static("Hello".to_string()));
    let if_id = find_if_block(&c.fragment, &c, "x")
        .expect("test invariant")
        .id;
    assert_content_strategy_variant(&data, FragmentKey::IfAlternate(if_id), "SingleElement");
}

#[test]
fn each_block_context_is_dynamic() {
    // {item} inside each block should be dynamic (it's a block-scoped variable)
    let (c, data) = analyze_source(
        r#"<script>let items = $state([]);</script>{#each items as item}<p>{item}</p>{/each}"#,
    );
    assert_dynamic_tag(&data, &c, "item");
}

#[test]
fn each_block_shadowing() {
    // `item` in script is a rune; `item` inside {#each} should shadow it
    // (resolved as each-block variable, not as script rune).
    let (c, data) = analyze_source(
        r#"<script>let item = $state(0); item = 1; let items = $state([]);</script>{#each items as item}<p>{item}</p>{/each}"#,
    );
    assert_dynamic_tag(&data, &c, "item");

    let root = data.scoping.root_scope_id();
    let root_sym = data
        .scoping
        .find_binding(root, "item")
        .expect("test invariant");
    assert!(matches!(
        data.declaration_semantics(data.scoping.symbol_declaration(root_sym)),
        crate::types::data::DeclarationSemantics::State(_)
    ));

    let each_block = find_each_block(&c.fragment, &c, "items").expect("test invariant");
    let each_scope = data
        .scoping
        .fragment_scope(&FragmentKey::EachBody(each_block.id))
        .expect("test invariant");
    let each_sym = data
        .scoping
        .find_binding(each_scope, "item")
        .expect("test invariant");
    assert!(matches!(
        data.declaration_semantics(data.scoping.symbol_declaration(each_sym)),
        crate::types::data::DeclarationSemantics::Contextual(_)
    ));
    assert_ne!(root_sym, each_sym);
}

#[test]
fn each_block_shadowing_does_not_mutate_rune() {
    // `count = 99` inside each targets the each-block variable (shadowing the rune), not the rune.
    // In runes mode this also triggers each_item_invalid_assignment — that's expected.
    let alloc = oxc_allocator::Allocator::default();
    let source = r#"<script>let count = $state(0); let items = $state([]);</script>{#each items as count}{count = 99}{/each}"#;
    let (component, js_result, parse_diags) = svelte_parser::parse_with_js(&alloc, source);
    assert!(parse_diags.is_empty());
    let (data, _parsed, _diags) = analyze(&component, js_result);
    // The ROOT-scoped rune `count` must NOT be mutated — shadowing works correctly.
    let root = data.scoping.root_scope_id();
    let count_sym = data
        .scoping
        .find_binding(root, "count")
        .expect("symbol 'count' not found");
    assert!(
        !data.scoping.is_mutated(count_sym),
        "rune 'count' should NOT be mutated — the assignment targets the each-block variable"
    );
}

#[test]
fn each_block_index_is_not_dynamic_unkeyed() {
    // In an unkeyed each block, the index is a plain iteration counter — not reactive, not dynamic.
    // Codegen uses direct assignment (div.textContent = i) rather than $.template_effect.
    let (c, data) = analyze_source(
        r#"<script>let items = $state([]);</script>{#each items as item, i}<p>{i}</p>{/each}"#,
    );
    assert_not_dynamic_tag(&data, &c, "i");
}

#[test]
fn element_facts_capture_static_and_runtime_attrs() {
    let (component, data) = analyze_source(
        r#"<script>let value = $state("x"); let spread = {};</script><div id="a" class={value} {...spread}></div>"#,
    );
    let el = find_element(&component.fragment, &component, "div").expect("no element <div>");
    let facts = data
        .element_facts(el.id)
        .unwrap_or_else(|| panic!("no element facts for <div>"));
    let idx = facts.attr_index();

    assert!(idx.has("id"));
    assert!(idx.has("class"));
    assert!(facts.has_runtime_attrs());
    assert!(facts.has_spread());
}

#[test]
fn template_topology_tracks_attr_and_expr_to_nearest_element() {
    let (component, data) = analyze_source(
        r#"<script>let value = "x"; let el;</script><div class={value} bind:this={el}></div>"#,
    );
    let el = find_element(&component.fragment, &component, "div").expect("no element <div>");
    let class_attr_id = find_attribute_id(&component.fragment, &component, "div", "class")
        .expect("no class attr on <div>");
    let bind_id = find_bind_directive_id(&component.fragment, &component, "div", "this")
        .expect("no bind:this on <div>");

    let class_parent = data
        .expr_parent(class_attr_id)
        .unwrap_or_else(|| panic!("no expr parent for class attr"));
    assert_eq!(class_parent.id, class_attr_id);
    assert_eq!(data.nearest_element(class_attr_id), Some(el.id));
    assert_eq!(data.nearest_element_for_expr(class_attr_id), Some(el.id));
    assert_eq!(data.nearest_element(bind_id), Some(el.id));
}

#[test]
fn concatenated_class_attr_is_registered_for_set_class() {
    let (component, data) = analyze_source(
        r#"<script>let value = $state("x");</script><div class="static {value}"></div>"#,
    );
    let el = find_element(&component.fragment, &component, "div").expect("no element <div>");
    let class_attr_id = find_attribute_id(&component.fragment, &component, "div", "class")
        .expect("no class attr on <div>");

    assert_eq!(
        data.elements.flags.class_attr_id(el.id),
        Some(class_attr_id)
    );
}

#[test]
fn template_element_index_tracks_css_candidates() {
    let (component, data) = analyze_source(
        r#"<script>let dynamic = "bar"; let props = {};</script>
<div class="foo bar" id="root"></div>
<span class={dynamic}></span>
<p {...props}></p>"#,
    );
    let div = find_element(&component.fragment, &component, "div").expect("no element <div>");
    let span = find_element(&component.fragment, &component, "span").expect("no element <span>");
    let p = find_element(&component.fragment, &component, "p").expect("no element <p>");

    assert_eq!(data.template_element_tag_name(div.id), Some("div"));
    assert_eq!(data.template_element_static_id(div.id), Some("root"));
    assert!(data.template_element_has_static_class(div.id, "foo"));
    assert!(data.template_element_has_static_class(div.id, "bar"));

    let class_candidates: Vec<_> = data.template_elements_for_class("foo").collect();
    assert!(class_candidates.contains(&div.id));
    assert!(class_candidates.contains(&span.id));
    assert!(class_candidates.contains(&p.id));

    let id_candidates: Vec<_> = data.template_elements_for_id("root").collect();
    assert!(id_candidates.contains(&div.id));
    assert!(!id_candidates.contains(&span.id));
    assert!(id_candidates.contains(&p.id));
}

#[test]
fn attribute_presence_selector_scopes_only_matching_elements() {
    let (component, data) = analyze_source_with_css(
        r#"<style>[data-role] { color: red; }</style>
<div data-role="banner"></div>
<span></span>"#,
    );
    let div = find_element(&component.fragment, &component, "div").expect("no element <div>");
    let span = find_element(&component.fragment, &component, "span").expect("no element <span>");

    assert!(data.is_css_scoped(div.id));
    assert!(!data.is_css_scoped(span.id));
}

#[test]
fn attribute_value_selectors_match_static_attributes() {
    let (component, data) = analyze_source_with_css(
        r#"<style>
[data-state="on"] { color: red; }
[data-state="on" i] { color: blue; }
[data-state="on" s] { color: green; }
[type="BUTTON"] { color: purple; }
</style>
<div data-state="on"></div>
<div data-state="On"></div>
<div data-state="off"></div>
<button type="button" aria-label="run"></button>"#,
    );

    let exact = find_nth_element(&component.fragment, &component, "div", 0).expect("exact div");
    let insensitive =
        find_nth_element(&component.fragment, &component, "div", 1).expect("insensitive div");
    let no_match = find_nth_element(&component.fragment, &component, "div", 2).expect("off div");
    let button = find_element(&component.fragment, &component, "button").expect("button");

    assert!(data.is_css_scoped(exact.id));
    assert!(data.is_css_scoped(insensitive.id));
    assert!(!data.is_css_scoped(no_match.id));
    assert!(data.is_css_scoped(button.id));
}

#[test]
fn attribute_matcher_operators_match_static_text_values() {
    let (component, data) = analyze_source_with_css(
        r#"<style>
[data-tags~="active"] { color: red; }
[data-lang|="en"] { color: blue; }
[data-url^="https://"] { color: green; }
[data-url$=".com"] { color: orange; }
[data-url*="example"] { color: purple; }
</style>
<div data-tags="card active"></div>
<div data-lang="en-US"></div>
<div data-url="https://example.com"></div>
<span data-tags="inactive"></span>
<div data-lang="bengali"></div>
<div data-url="http://sample.org"></div>"#,
    );

    let class_match =
        find_nth_element(&component.fragment, &component, "div", 0).expect("class div");
    let lang_match = find_nth_element(&component.fragment, &component, "div", 1).expect("lang div");
    let href_match = find_nth_element(&component.fragment, &component, "div", 2).expect("href div");
    let class_no_match = find_element(&component.fragment, &component, "span").expect("span");
    let lang_no_match =
        find_nth_element(&component.fragment, &component, "div", 3).expect("second lang div");
    let href_no_match =
        find_nth_element(&component.fragment, &component, "div", 4).expect("second href div");

    assert!(data.is_css_scoped(class_match.id));
    assert!(data.is_css_scoped(lang_match.id));
    assert!(data.is_css_scoped(href_match.id));
    assert!(!data.is_css_scoped(class_no_match.id));
    assert!(!data.is_css_scoped(lang_no_match.id));
    assert!(!data.is_css_scoped(href_no_match.id));
}

#[test]
fn attribute_selector_names_are_casefolded_for_static_matching() {
    let (component, data) = analyze_source_with_css(
        r#"<style>
[DATA-ROLE] { color: red; }
[TYPE="BUTTON"] { color: blue; }
[viewbox] { border: 1px solid green; }
</style>
<div DATA-role="banner"></div>
<button TYPE="button" aria-label="run"></button>
<svg viewBox="0 0 10 10"></svg>"#,
    );

    let div = find_element(&component.fragment, &component, "div").expect("div");
    let button = find_element(&component.fragment, &component, "button").expect("button");
    let svg = find_element(&component.fragment, &component, "svg").expect("svg");

    assert!(data.is_css_scoped(div.id));
    assert!(data.is_css_scoped(button.id));
    assert!(data.is_css_scoped(svg.id));
}

#[test]
fn pseudo_is_and_where_standalone_branches_scope_matching_elements() {
    let (component, data) = analyze_source_with_css(
        r#"<style>
:is(.foo) { color: red; }
:where(.bar) { color: blue; }
</style>
<p class="foo"></p>
<span class="bar"></span>
<h1 class="baz">title</h1>"#,
    );

    let p = find_element(&component.fragment, &component, "p").expect("p");
    let span = find_element(&component.fragment, &component, "span").expect("span");
    let h1 = find_element(&component.fragment, &component, "h1").expect("h1");

    assert!(data.is_css_scoped(p.id));
    assert!(data.is_css_scoped(span.id));
    assert!(!data.is_css_scoped(h1.id));
}

#[test]
fn pseudo_compound_is_and_where_branches_stay_conservative() {
    let (component, data, css_diags) = analyze_source_with_css_diags(
        r#"<style>
:is(.foo).bar { color: red; }
:where(.baz)[data-nope] { color: blue; }
</style>
<p class="foo">foo</p>
<span class="baz">baz</span>"#,
    );

    let p = find_element(&component.fragment, &component, "p").expect("p");
    let span = find_element(&component.fragment, &component, "span").expect("span");

    assert_eq!(css_diags.len(), 2);
    assert!(data.is_css_scoped(p.id));
    assert!(data.is_css_scoped(span.id));
}

#[test]
fn pseudo_where_complex_branches_stay_conservative() {
    let (component, data) = analyze_source_with_css(
        r#"<style>
:where(section p) { color: red; }
</style>
<p>hello</p>"#,
    );

    let p = find_element(&component.fragment, &component, "p").expect("p");
    assert!(data.is_css_scoped(p.id));
}

#[test]
fn pseudo_has_scopes_matching_ancestor_and_descendant() {
    let (component, data) = analyze_source_with_css(
        r#"<style>
section:has(.hit) { color: red; }
</style>
<section><p class="hit">yes</p></section>
<section><p>no</p></section>"#,
    );

    let section_hit =
        find_nth_element(&component.fragment, &component, "section", 0).expect("hit section");
    let section_miss =
        find_nth_element(&component.fragment, &component, "section", 1).expect("miss section");
    let p_hit = find_nth_element(&component.fragment, &component, "p", 0).expect("hit p");
    let p_miss = find_nth_element(&component.fragment, &component, "p", 1).expect("miss p");

    assert!(data.is_css_scoped(section_hit.id));
    assert!(data.is_css_scoped(p_hit.id));
    assert!(!data.is_css_scoped(section_miss.id));
    assert!(!data.is_css_scoped(p_miss.id));
}

#[test]
fn render_tag_sibling_matching_crosses_snippet_boundary() {
    let (component, data) = analyze_source_with_css(
        r#"<style>
.before + .after { color: red; }
</style>
{#snippet before()}<span class="before">before</span>{/snippet}
{@render before()}
<div class="after">after</div>
<div>other</div>"#,
    );

    let before = find_element(&component.fragment, &component, "span").expect("before span");
    let after = find_nth_element(&component.fragment, &component, "div", 0).expect("after div");
    let other = find_nth_element(&component.fragment, &component, "div", 1).expect("other div");

    assert!(data.is_css_scoped(before.id));
    assert!(data.is_css_scoped(after.id));
    assert!(!data.is_css_scoped(other.id));
}

#[test]
fn component_snippet_descendant_matching_crosses_component_boundary() {
    let (component, data) = analyze_source_with_css(
        r#"<style>
.host .inside { color: red; }
</style>
<div class="host">
    <Widget>
        {#snippet children()}
            <span class="inside">inside</span>
        {/snippet}
    </Widget>
</div>
<span class="inside">outside</span>"#,
    );

    let host = find_element(&component.fragment, &component, "div").expect("host");
    let inside = find_nth_element(&component.fragment, &component, "span", 0).expect("inside span");
    let outside =
        find_nth_element(&component.fragment, &component, "span", 1).expect("outside span");

    assert!(data.is_css_scoped(host.id));
    assert!(data.is_css_scoped(inside.id));
    assert!(!data.is_css_scoped(outside.id));
}

#[test]
fn nesting_selector_scopes_parent_chain() {
    let (component, data) = analyze_source_with_css(
        r#"<style>
.card {
    & .title { color: red; }
}
</style>
<div class="card"><h2 class="title">inside</h2></div>
<h2 class="title">outside</h2>"#,
    );

    let card = find_element(&component.fragment, &component, "div").expect("card");
    let inside = find_nth_element(&component.fragment, &component, "h2", 0).expect("inside title");
    let outside =
        find_nth_element(&component.fragment, &component, "h2", 1).expect("outside title");

    assert!(data.is_css_scoped(card.id));
    assert!(data.is_css_scoped(inside.id));
    assert!(!data.is_css_scoped(outside.id));
}

#[test]
fn implicit_nesting_selector_scopes_parent_chain() {
    let (component, data) = analyze_source_with_css(
        r#"<style>
.card {
    .title { color: red; }
}
</style>
<div class="card"><h2 class="title">inside</h2></div>
<h2 class="title">outside</h2>"#,
    );

    let card = find_element(&component.fragment, &component, "div").expect("card");
    let inside = find_nth_element(&component.fragment, &component, "h2", 0).expect("inside title");
    let outside =
        find_nth_element(&component.fragment, &component, "h2", 1).expect("outside title");

    assert!(data.is_css_scoped(card.id));
    assert!(data.is_css_scoped(inside.id));
    assert!(!data.is_css_scoped(outside.id));
}

#[test]
fn root_has_scopes_local_selector_branch() {
    let (component, data, css_diags) = analyze_source_with_css_diags(
        r#"<style>
:root:has(.hit) { color: red; }
</style>
<div class="hit">inside</div>
<div>outside</div>"#,
    );

    let hit = find_nth_element(&component.fragment, &component, "div", 0).expect("hit div");
    let outside = find_nth_element(&component.fragment, &component, "div", 1).expect("outside div");

    assert!(
        css_diags.is_empty(),
        "unexpected css diagnostics: {css_diags:?}"
    );
    assert!(data.is_css_scoped(hit.id));
    assert!(!data.is_css_scoped(outside.id));
}

#[test]
fn escaped_selectors_match_static_template_names() {
    let (component, data, css_diags) = analyze_source_with_css_diags(
        r#"<style>
.foo\:bar { color: red; }
#hero\:id { color: blue; }
</style>
<div class="foo:bar"></div>
<div id="hero:id"></div>
<div class="miss"></div>"#,
    );

    let class_match =
        find_nth_element(&component.fragment, &component, "div", 0).expect("class match");
    let id_match = find_nth_element(&component.fragment, &component, "div", 1).expect("id match");
    let miss = find_nth_element(&component.fragment, &component, "div", 2).expect("miss");

    assert!(
        css_diags.is_empty(),
        "unexpected css diagnostics: {css_diags:?}"
    );
    assert!(data.is_css_scoped(class_match.id));
    assert!(data.is_css_scoped(id_match.id));
    assert!(!data.is_css_scoped(miss.id));
}

#[test]
fn dynamic_attribute_values_expand_known_expression_branches() {
    let (component, data) = analyze_source_with_css(
        r#"<style>
[data-state="on"] { color: red; }
</style>
<div data-state={flag ? "on" : "off"}></div>
<span data-state={flag ? "idle" : "off"}></span>"#,
    );

    let div = find_element(&component.fragment, &component, "div").expect("div");
    let span = find_element(&component.fragment, &component, "span").expect("span");

    assert!(data.is_css_scoped(div.id));
    assert!(!data.is_css_scoped(span.id));
}

#[test]
fn bare_global_tail_only_scopes_local_prefix() {
    let source = r#"
<style>
    .a :global .b .c { color: red; }
    section :global strong { font-weight: bold; }
    :global .page .title { margin: 0; }
</style>

<div class="a"><div class="b"><div class="c">compound</div></div></div>
<section><strong>bare global</strong></section>
<h1 class="title">title</h1>
"#;

    let alloc = oxc_allocator::Allocator::default();
    let (component, js_result, parse_diags) = svelte_parser::parse_with_js(&alloc, source);
    assert!(
        parse_diags.is_empty(),
        "unexpected parse diagnostics: {parse_diags:?}"
    );
    let (mut data, parsed, diags) = analyze(&component, js_result);
    assert!(diags.is_empty(), "unexpected diagnostics: {diags:?}");
    let Some((stylesheet, css_diags)) = svelte_parser::parse_css_block(&component) else {
        panic!("expected style block")
    };
    assert!(
        css_diags.is_empty(),
        "unexpected css diagnostics: {css_diags:?}"
    );
    let mut css_pass_diags = Vec::new();
    analyze_css_pass(
        &component,
        &stylesheet,
        &parsed,
        false,
        &mut data,
        &mut css_pass_diags,
    );
    assert!(
        css_pass_diags.is_empty(),
        "unexpected css analyze diagnostics: {css_pass_diags:?}"
    );

    let div = find_nth_element(&component.fragment, &component, "div", 0).expect("root div");
    let section = find_element(&component.fragment, &component, "section").expect("section");
    let strong = find_element(&component.fragment, &component, "strong").expect("strong");
    let h1 = find_element(&component.fragment, &component, "h1").expect("h1");

    assert!(
        data.is_css_scoped(div.id),
        "local prefix element should be scoped"
    );
    assert!(
        data.is_css_scoped(section.id),
        "local selector before bare :global should stay scoped"
    );
    assert!(
        !data.is_css_scoped(strong.id),
        "selector tail after bare :global must stay unscoped"
    );
    assert!(
        !data.is_css_scoped(h1.id),
        "leading bare :global selector should not scope any component element"
    );
}

#[test]
fn template_element_index_preserves_parent_element_across_blocks() {
    let (component, data) = analyze_source(
        r#"<div>{#if ok}<section>{#each items as item}<span>{item}</span>{/each}</section>{/if}</div>"#,
    );
    let div = find_element(&component.fragment, &component, "div").expect("no element <div>");
    let section =
        find_element(&component.fragment, &component, "section").expect("no element <section>");
    let span = find_element(&component.fragment, &component, "span").expect("no element <span>");

    assert_eq!(data.template_element_parent(section.id), Some(div.id));
    assert_eq!(data.template_element_parent(span.id), Some(section.id));
}

#[test]
fn template_element_index_tracks_element_siblings() {
    let (component, data) = analyze_source(
        r#"<div></div>{value}<span></span><p></p>
<section>{#if ok}<em></em>{/if}<strong></strong></section>"#,
    );
    let div = find_element(&component.fragment, &component, "div").expect("no element <div>");
    let span = find_element(&component.fragment, &component, "span").expect("no element <span>");
    let p = find_element(&component.fragment, &component, "p").expect("no element <p>");
    let em = find_element(&component.fragment, &component, "em").expect("no element <em>");
    let strong =
        find_element(&component.fragment, &component, "strong").expect("no element <strong>");

    assert_eq!(data.template_element_previous_sibling(div.id), None);
    assert_eq!(data.template_element_next_sibling(div.id), Some(span.id));
    assert_eq!(
        data.template_element_previous_sibling(span.id),
        Some(div.id)
    );
    assert_eq!(data.template_element_next_sibling(span.id), Some(p.id));
    assert_eq!(
        data.template_element_previous_siblings(p.id)
            .collect::<Vec<_>>(),
        vec![span.id, div.id]
    );
    assert_eq!(
        data.template_element_previous_sibling(strong.id),
        Some(em.id)
    );
}

#[test]
fn bind_group_tracks_matching_ancestor_each_blocks_via_query_layer() {
    // `groups` is `$state({})` because `bind:group={groups[...][...]}` mutates
    // it through the member chain — keeping it plain would now (correctly)
    // trigger the non-reactive-update warning.
    let (component, data) = analyze_source(
        r#"<script>
    let items = $state([{ options: [{ id: 1 }] }]);
    let groups = $state({});
</script>
{#each items as item}
    {#each item.options as option}
        <input type="checkbox" bind:group={groups[item.id][option.id]} value={option.id} />
    {/each}
{/each}"#,
    );
    let outer_each = find_each_block(&component.fragment, &component, "items")
        .expect("no outer each block for items");
    let inner_each = find_each_block(&outer_each.body, &component, "item.options")
        .expect("no inner each block for item.options");
    let bind_id = find_bind_directive_id(&component.fragment, &component, "input", "group")
        .expect("no bind:group on input");

    let parent_each_blocks = data.parent_each_blocks(bind_id);
    assert_eq!(
        parent_each_blocks.as_slice(),
        &[inner_each.id, outer_each.id]
    );
    assert!(data.contains_group_binding(inner_each.id));
    assert!(data.contains_group_binding(outer_each.id));
}

#[test]
fn bind_group_without_each_references_does_not_mark_enclosing_each_blocks() {
    let (component, data) = analyze_source(
        r#"<script>
    let groups = [["a", "b"]];
    let selected = $state([]);
</script>
{#each groups as group}
    {#each group as item}
        <input type="checkbox" bind:group={selected} value={item} />
    {/each}
{/each}"#,
    );
    let outer_each = find_each_block(&component.fragment, &component, "groups")
        .expect("no outer each block for groups");
    let inner_each = find_each_block(&outer_each.body, &component, "group")
        .expect("no inner each block for group");
    let bind_id = find_bind_directive_id(&component.fragment, &component, "input", "group")
        .expect("no bind:group on input");

    assert!(data.parent_each_blocks(bind_id).is_empty());
    assert!(!data.contains_group_binding(inner_each.id));
    assert!(!data.contains_group_binding(outer_each.id));
}

#[test]
fn bind_group_marks_only_ancestor_each_blocks_referenced_by_expression() {
    let (component, data) = analyze_source(
        r#"<script>
    let groups = [["a", "b"]];
    let selected = $state({});
</script>
{#each groups as group}
    {#each group as item}
        <input type="checkbox" bind:group={selected[group]} value={item} />
    {/each}
{/each}"#,
    );
    let outer_each = find_each_block(&component.fragment, &component, "groups")
        .expect("no outer each block for groups");
    let inner_each = find_each_block(&outer_each.body, &component, "group")
        .expect("no inner each block for group");
    let bind_id = find_bind_directive_id(&component.fragment, &component, "input", "group")
        .expect("no bind:group on input");

    assert_eq!(
        data.parent_each_blocks(bind_id).as_slice(),
        &[outer_each.id]
    );
    assert!(data.contains_group_binding(outer_each.id));
    assert!(!data.contains_group_binding(inner_each.id));
}

#[test]
fn bind_group_records_expression_value_attr_only() {
    let (component, data) = analyze_source(
        r#"<script>
    let selected = $state([]);
    let value = $state("x");
</script>
<input type="checkbox" bind:group={selected} value={value} />
<input type="checkbox" bind:group={selected} value="static" />"#,
    );
    let bind_id = find_bind_directive_id(&component.fragment, &component, "input", "group")
        .expect("no bind:group on first input");
    let value_attr_id = find_attribute_id(&component.fragment, &component, "input", "value")
        .expect("no expression value attr on first input");

    assert_eq!(
        data.template.bind_semantics.bind_group_value_attr(bind_id),
        Some(value_attr_id)
    );
}

#[test]
fn bind_checked_marks_bindable_props_source_nodes() {
    let (component, data) = analyze_source(
        r#"<script lang="ts">
	type Props = {
		checked?: boolean;
		disabled?: boolean;
	};

	let { checked = $bindable(false), disabled = false }: Props = $props();
</script>

<input type="checkbox" bind:checked {disabled} />"#,
    );

    assert_bind_target_symbol_name(&data, &component, "input", "checked", "checked");
}

#[test]
fn each_context_index_tracks_index_symbol_and_bind_this_context() {
    // `node` is `$state([])` so bind:this={node[index]} member mutation
    // doesn't trigger the non-reactive-update warning. Before the shorthand
    // rework `node` was never flagged mutated so a plain `let node` used to
    // slip past this validator silently — that was a bug in analyze.
    let (component, data) = analyze_source(
        r#"<script>
    let items = [{ id: 1 }];
    let node = $state([]);
</script>
{#each items as item, index (item.id)}
    <div bind:this={node[index]}>{item.id}</div>
{/each}"#,
    );
    let each = find_each_block(&component.fragment, &component, "items").expect("no each block");
    let bind_id = find_bind_directive_id(&component.fragment, &component, "div", "this")
        .expect("no bind:this on div");

    let index_sym = data
        .each_index_sym(each.id)
        .unwrap_or_else(|| panic!("missing each index symbol"));
    assert_eq!(data.each_block_for_index_sym(index_sym), Some(each.id));
    assert_eq!(data.each_context_name(each.id), "item");
    let bind_context = data
        .bind_each_context(bind_id)
        .unwrap_or_else(|| panic!("missing bind:this each context"));
    let bind_context_names: Vec<_> = bind_context
        .iter()
        .map(|&sym| data.scoping.symbol_name(sym).to_string())
        .collect();
    assert_eq!(bind_context_names, vec!["index".to_string()]);
    assert!(data.each_body_uses_index(each.id));
}

#[test]
fn static_contenteditable_marks_bound_contenteditable() {
    let (component, data) = analyze_source(
        r#"<script>let html = $state('');</script><div contenteditable bind:innerHTML={html}></div>"#,
    );
    let el = find_element(&component.fragment, &component, "div").expect("no element <div>");
    assert!(data.elements.flags.is_bound_contenteditable(el.id));
}

#[test]
fn dynamic_contenteditable_does_not_mark_bound_contenteditable() {
    let source = r#"<script>let html = $state(''); let editable = true;</script><div contenteditable={editable} bind:innerHTML={html}></div>"#;
    let alloc = oxc_allocator::Allocator::default();
    let (component, js_result, parse_diags) = svelte_parser::parse_with_js(&alloc, source);
    assert!(
        parse_diags.is_empty(),
        "unexpected parse diagnostics: {parse_diags:?}"
    );
    let (data, _parsed, _diags) = analyze(&component, js_result);
    let el = find_element(&component.fragment, &component, "div").expect("no element <div>");
    assert!(!data.elements.flags.is_bound_contenteditable(el.id));
}

#[test]
fn dynamic_bind_marks_needs_ref_via_topology_queries() {
    let (component, data) = analyze_source(r#"<script>let el;</script><div bind:this={el}></div>"#);
    assert_element_needs_ref(&data, &component, "div", true);
}

#[test]
fn dynamic_class_directive_marks_element_state() {
    let (component, data) = analyze_source(
        r#"<script>let foo = $state(true); foo = false;</script><div class:active={foo}></div>"#,
    );
    assert_has_dynamic_class_directives(&data, &component, "div", true);
    assert_element_needs_ref(&data, &component, "div", true);
}

#[test]
fn runtime_attrs_force_needs_var_via_element_facts() {
    let (component, data) =
        analyze_source(r#"<script>let value = $state("x");</script><div class={value}></div>"#);
    assert_element_needs_var(&data, &component, "div", true);
}

#[test]
fn static_attrs_do_not_force_needs_var() {
    let (component, data) = analyze_source(r#"<div class="a" title="b"></div>"#);
    assert_element_needs_var(&data, &component, "div", false);
}

// ---------------------------------------------------------------------------
// Rune kind tests
// ---------------------------------------------------------------------------

#[test]
fn rune_kind_state() {
    let (_c, data) = analyze_source(r#"<script>let count = $state(0);</script>"#);
    assert_rune_kind(&data, "count", RuneKind::State);
}

#[test]
fn rune_kind_state_raw() {
    let (_c, data) = analyze_source(r#"<script>let data = $state.raw({});</script>"#);
    assert_rune_kind(&data, "data", RuneKind::StateRaw);
}

#[test]
fn rune_kind_derived() {
    let (_c, data) = analyze_source(
        r#"<script>let count = $state(0); let doubled = $derived(count * 2);</script>"#,
    );
    assert_rune_kind(&data, "count", RuneKind::State);
    assert_rune_kind(&data, "doubled", RuneKind::Derived);
}

#[test]
fn rune_kind_derived_by() {
    let (_c, data) = analyze_source(r#"<script>let val = $derived.by(() => 42);</script>"#);
    assert_rune_kind(&data, "val", RuneKind::DerivedBy);
}

#[test]
fn rune_kind_state_eager() {
    let (_c, data) = analyze_source(r#"<script>let count = $state.eager(0);</script>"#);
    assert_rune_kind(&data, "count", RuneKind::StateEager);
}

#[test]
fn module_rune_kinds_and_cross_script_refs() {
    let (component, data) = analyze_source(
        r#"<script module>
    let shared = $state(0);
    let doubled = $derived(shared * 2);
</script>

<script>
    function increment() {
        shared++;
    }
</script>

<button>{doubled}</button>"#,
    );

    assert_rune_kind(&data, "shared", RuneKind::State);
    assert_rune_kind(&data, "doubled", RuneKind::Derived);
    assert_rune_is_mutated(&data, "shared");

    let expr_id = find_expr_tag(&component.fragment, &component, "doubled")
        .expect("expected template expression for doubled");
    let root = data.scoping.root_scope_id();
    let doubled_sym = data
        .scoping
        .find_binding(root, "doubled")
        .expect("expected doubled binding");
    let expr = data.expression(expr_id).expect("expected expression info");
    assert!(
        expr.ref_symbols().contains(&doubled_sym),
        "template expression should resolve to the module-scoped doubled binding"
    );
}

#[test]
fn script_rune_calls_keep_module_and_instance_programs_distinct() {
    let source = r#"<script module>
    const teardown = $effect.root(() => {});
</script>
<script>
    let count = $state(0);
</script>"#;
    let (component, data, parsed) = analyze_source_with_parsed(source);

    let module_source = component.source_text(
        parsed
            .module_script_content_span
            .unwrap_or_else(|| panic!("missing module script span")),
    );
    let instance_source = component.source_text(
        parsed
            .script_content_span
            .unwrap_or_else(|| panic!("missing instance script span")),
    );
    let module_call = find_call_node_id(
        parsed
            .module_program
            .as_ref()
            .unwrap_or_else(|| panic!("missing module program")),
        module_source,
        "$effect.root(() => {})",
    )
    .unwrap_or_else(|| panic!("missing module rune call"));
    let instance_call = find_call_node_id(
        parsed
            .program
            .as_ref()
            .unwrap_or_else(|| panic!("missing instance program")),
        instance_source,
        "$state(0)",
    )
    .unwrap_or_else(|| panic!("missing instance rune call"));

    let remapped_instance =
        OxcNodeId::from_usize(instance_call.index() + data.script.instance_node_id_offset as usize);
    let remapped_module =
        OxcNodeId::from_usize(module_call.index() + data.script.module_node_id_offset as usize);

    assert_eq!(
        data.script_rune_calls().kind(remapped_module),
        Some(RuneKind::EffectRoot)
    );
    assert_eq!(
        data.script_rune_calls().kind(remapped_instance),
        Some(RuneKind::State)
    );
}

#[test]
fn script_rune_calls_survive_template_node_id_activity() {
    let source = r#"<script module>
    const teardown = $effect.root(() => {});
</script>
<script>
    let count = $state(0);
    let html = "<b>x</b>";
</script>
{@html html}"#;
    let (component, data, parsed) = analyze_source_with_parsed(source);

    let module_source =
        component.source_text(parsed.module_script_content_span.expect("test invariant"));
    let instance_source =
        component.source_text(parsed.script_content_span.expect("test invariant"));
    let module_call = find_call_node_id(
        parsed.module_program.as_ref().expect("test invariant"),
        module_source,
        "$effect.root(() => {})",
    )
    .expect("missing module rune call");
    let instance_call = find_call_node_id(
        parsed.program.as_ref().expect("test invariant"),
        instance_source,
        "$state(0)",
    )
    .expect("missing instance rune call");

    let remapped_instance =
        OxcNodeId::from_usize(instance_call.index() + data.script.instance_node_id_offset as usize);
    let remapped_module =
        OxcNodeId::from_usize(module_call.index() + data.script.module_node_id_offset as usize);

    assert_eq!(
        data.script_rune_calls().kind(remapped_module),
        Some(RuneKind::EffectRoot)
    );
    assert_eq!(
        data.script_rune_calls().kind(remapped_instance),
        Some(RuneKind::State)
    );
}

#[test]
fn module_imports_are_visible_from_instance_scope() {
    use oxc_ast_visit::Visit;

    let alloc = oxc_allocator::Allocator::default();
    let source = r#"<script module>
    import { onMount as shared } from 'svelte';
</script>

<script>
    const alias = shared;
</script>

<button>{shared}</button>"#;
    let (component, js_result, parse_diags) = svelte_parser::parse_with_js(&alloc, source);
    assert!(parse_diags.is_empty());

    let (data, parsed, diags) = analyze(&component, js_result);
    assert!(diags.is_empty(), "unexpected diagnostics: {diags:?}");

    let root = data.scoping.root_scope_id();
    let shared_sym = data
        .scoping
        .find_binding(root, "shared")
        .expect("expected module import binding");
    assert!(
        data.scoping.is_import(shared_sym),
        "expected cross-script binding to preserve import flags"
    );

    struct RefFinder<'n> {
        target: &'n str,
        ref_id: Option<oxc_semantic::ReferenceId>,
    }

    impl<'a> Visit<'a> for RefFinder<'_> {
        fn visit_identifier_reference(&mut self, ident: &oxc_ast::ast::IdentifierReference<'a>) {
            if ident.name == self.target {
                self.ref_id = ident.reference_id.get();
            }
        }
    }

    let instance_program = parsed.program.as_ref().expect("expected instance program");
    let mut finder = RefFinder {
        target: "shared",
        ref_id: None,
    };
    finder.visit_program(instance_program);
    let ref_id = finder
        .ref_id
        .expect("expected instance reference to shared");
    assert_eq!(
        data.scoping.get_reference(ref_id).symbol_id(),
        Some(shared_sym),
        "instance-script reference should resolve through the module parent scope"
    );

    let expr_id = find_expr_tag(&component.fragment, &component, "shared")
        .expect("expected template expression for shared");
    let expr = data.expression(expr_id).expect("expected expression info");
    assert!(
        expr.ref_symbols().contains(&shared_sym),
        "template expression should resolve to the module import binding"
    );
}

#[test]
fn module_exported_render_tag_callee_stays_direct_with_snippets() {
    let source = r#"<script module>
    export const KIND = "v1";
    export function label(name) {
        return `${KIND}:${name}`;
    }
</script>

<script>
    let { title } = $props();
</script>

{#snippet row(text)}
    <span>{text}</span>
{/snippet}

{@render row(label(title))}"#;
    let (component, data) = analyze_source(source);

    let render_id = find_render_tag(&component.fragment, &component, "row(label(title))")
        .expect("expected render tag");
    let plan = data
        .render_tag_plan(render_id)
        .expect("expected render tag plan");
    assert_eq!(
        plan.callee_mode,
        RenderTagCalleeMode::Direct,
        "snippet render call should stay direct when the callee is a normal snippet binding"
    );
}

// ---------------------------------------------------------------------------
// Rune mutation tests
// ---------------------------------------------------------------------------

#[test]
fn rune_mutated_by_assignment() {
    let (_c, data) = analyze_source(r#"<script>let count = $state(0); count = 1;</script>"#);
    assert_rune_is_mutated(&data, "count");
}

#[test]
fn rune_mutated_by_update() {
    let (_c, data) = analyze_source(r#"<script>let count = $state(0); count++;</script>"#);
    assert_rune_is_mutated(&data, "count");
}

#[test]
fn rune_mutated_by_compound_assignment() {
    let (_c, data) = analyze_source(r#"<script>let count = $state(0); count += 1;</script>"#);
    assert_rune_is_mutated(&data, "count");
}

#[test]
fn rune_not_mutated_when_only_read() {
    let (_c, data) = analyze_source(r#"<script>let count = $state(0);</script>{count}"#);
    assert_rune_not_mutated(&data, "count");
}

#[test]
fn derived_is_never_mutated() {
    let (_c, data) = analyze_source(
        r#"<script>let count = $state(0); let doubled = $derived(count * 2);</script>{doubled}"#,
    );
    assert_rune_not_mutated(&data, "doubled");
}

// ---------------------------------------------------------------------------
// Template expression has_call tests
// ---------------------------------------------------------------------------

#[test]
fn expr_tag_with_function_call() {
    let (c, data) = analyze_source(r#"<script>function fmt(x) { return x; }</script>{fmt(1)}"#);
    assert_expr_tag_has_call(&data, &c, "fmt(1)");
}

#[test]
fn expr_tag_without_call() {
    let (c, data) = analyze_source(r#"<script>let count = $state(0); count++;</script>{count}"#);
    assert_expr_tag_no_call(&data, &c, "count");
}

// ---------------------------------------------------------------------------
// Store reference detection tests
// ---------------------------------------------------------------------------

#[test]
fn store_ref_detected_in_template() {
    let (c, data) = analyze_source(r#"<script>import { count } from './store';</script>{$count}"#);
    assert_expr_tag_has_store_ref(&data, &c, "$count");
}

#[test]
fn no_store_ref_for_regular_var() {
    let (c, data) = analyze_source(r#"<script>let count = $state(0); count++;</script>{count}"#);
    assert_expr_tag_no_store_ref(&data, &c, "count");
}

// ---------------------------------------------------------------------------
// ExprRole tests
// ---------------------------------------------------------------------------

#[test]
fn expr_role_static_literal_expression() {
    let (c, data) = analyze_source("{1}");
    assert_expr_tag_role(&data, &c, "1", ExprRole::Static);
}

#[test]
fn expr_role_dynamic_pure_for_state_expression() {
    let (c, data) = analyze_source(r#"<script>let count = $state(0); count++;</script>{count}"#);
    assert_expr_tag_role(&data, &c, "count", ExprRole::DynamicPure);
}

#[test]
fn expr_role_dynamic_with_context_for_import_call_expression() {
    let (c, data) = analyze_source(r#"<script>import api from './api';</script>{api.run()}"#);
    assert_expr_tag_role(&data, &c, "api.run()", ExprRole::DynamicWithContext);
    assert!(
        data.output.needs_context,
        "dynamic-with-context expression should require component context"
    );
}

#[test]
fn expr_role_async_for_inline_await_expression() {
    let (c, data) = analyze_source_with_options(
        r#"<script>let promise = Promise.resolve(1);</script><p>{await promise}</p>"#,
        AnalyzeOptions {
            experimental_async: true,
            ..AnalyzeOptions::default()
        },
    );
    assert_expr_tag_role(&data, &c, "await promise", ExprRole::Async);
    assert_expr_tag_async_query(&data, &c, "await promise", true);
}

#[test]
fn expr_role_render_tag_for_render_call_site() {
    let (c, data) = analyze_source(
        r#"{#snippet row(text)}
    <span>{text}</span>
{/snippet}

{@render row('x')}"#,
    );
    assert_render_tag_role(&data, &c, "row('x')", ExprRole::RenderTag);
}

#[test]
fn expr_role_accessor_works_for_attribute_expressions() {
    let (c, data) = analyze_source(
        r#"<script>let count = $state(0); count++;</script><div title={count}></div>"#,
    );
    assert_attr_role(&data, &c, "div", "title", ExprRole::DynamicPure);
}

#[test]
fn expr_role_dynamic_pure_does_not_set_needs_context() {
    let (c, data) = analyze_source(r#"<script>let count = $state(0); count++;</script>{count}"#);
    assert_expr_tag_role(&data, &c, "count", ExprRole::DynamicPure);
    assert!(
        !data.output.needs_context,
        "dynamic-pure expression should not require component context by role alone"
    );
    assert_expr_tag_async_query(&data, &c, "count", false);
}

#[test]
fn prop_source_expression_stays_dynamic() {
    let (component, data) = analyze_source(
        r#"<svelte:options runes={true} />
<script>
    let { value = 1 } = $props();
</script>

{value.toString()}"#,
    );

    assert_dynamic_tag(&data, &component, "value.toString()");
}

#[test]
fn bindable_prop_source_expression_stays_dynamic() {
    let (component, data) = analyze_source(
        r#"<svelte:options runes={true} />
<script>
    let { value = $bindable(1) } = $props();
</script>

{value.toString()}"#,
    );

    assert_dynamic_tag(&data, &component, "value.toString()");
}

#[test]
fn non_source_props_expression_still_stays_dynamic() {
    let (component, data) = analyze_source(
        r#"<svelte:options runes={true} />
<script>
    let { value } = $props();
</script>

{value.toString()}"#,
    );

    assert_dynamic_tag(&data, &component, "value.toString()");
}

#[test]
fn const_tag_bindings_record_const_tag_owner() {
    let (_component, data) = analyze_source(
        r#"<script>
    let shared = 1;
</script>

{#if true}
    {@const alias = shared}
    <div>{alias}</div>
{/if}"#,
    );

    assert_const_tag_owner(&data, "alias");
}

// ---------------------------------------------------------------------------
// extract_expression_info tests
// ---------------------------------------------------------------------------

mod expression_info_tests {
    use crate::passes::js_analyze::analyze_expression;
    use crate::types::data::{ExpressionInfo, ExpressionKind};
    use compact_str::CompactString;
    use oxc_allocator::Allocator;
    use oxc_parser::Parser as OxcParser;
    use oxc_span::SourceType;
    use svelte_diagnostics::Diagnostic;
    use svelte_span::Span;

    fn compact(s: &str) -> CompactString {
        CompactString::from(s)
    }

    fn parse_and_analyze(source: &str, offset: u32) -> Result<ExpressionInfo, Diagnostic> {
        let allocator = Allocator::default();
        let parser = OxcParser::new(&allocator, source, SourceType::default());
        let expr = parser.parse_expression().map_err(|_| {
            Diagnostic::invalid_expression(Span::new(offset, offset + source.len() as u32))
        })?;
        let info = analyze_expression(&expr);
        Ok(info)
    }

    #[test]
    fn analyze_simple_identifier() {
        let info = parse_and_analyze("count", 0).expect("test invariant");
        assert_eq!(info.kind(), &ExpressionKind::Identifier(compact("count")));
        assert_eq!(info.identifier_name(), Some("count"));
        assert!(info.is_identifier());
        assert!(info.is_identifier_or_member_expression());
        // ref_symbols populated later by resolve_references (not during analyze_expression)
        assert!(!info.has_store_ref());
    }

    #[test]
    fn analyze_binary_expression() {
        let info = parse_and_analyze("count + 1", 0).expect("test invariant");
        assert_eq!(info.identifier_name(), None);
        assert!(!info.has_store_ref());
        assert!(!info.has_call());
        assert!(!info.needs_memoized_value());
        assert!(!info.needs_legacy_coarse_wrap());
    }

    #[test]
    fn analyze_call_expression() {
        let info = parse_and_analyze("foo(a, b)", 0).expect("test invariant");
        assert!(matches!(info.kind(), ExpressionKind::CallExpression { .. }));
        assert!(info.has_call());
        assert!(info.has_side_effects());
        assert!(info.has_context_sensitive_shape());
        assert!(info.needs_memoized_value());
        assert!(info.needs_legacy_coarse_wrap());
    }

    #[test]
    fn analyze_assignment() {
        let info = parse_and_analyze("count = 10", 0).expect("test invariant");
        assert_eq!(info.kind(), &ExpressionKind::Assignment);
        assert!(info.has_side_effects());
        assert!(info.needs_legacy_coarse_wrap());
    }

    #[test]
    fn analyze_store_ref() {
        let info = parse_and_analyze("$count + 1", 0).expect("test invariant");
        assert!(info.has_store_ref());
    }
}

// -----------------------------------------------------------------------
// Blocker tracking (experimental.async) — assert helpers
// -----------------------------------------------------------------------

fn assert_has_async(data: &AnalysisData) {
    assert!(
        data.blocker_data().has_async(),
        "expected blocker_data.has_async to be true"
    );
}

fn assert_no_async(data: &AnalysisData) {
    assert!(
        !data.blocker_data().has_async(),
        "expected blocker_data.has_async to be false"
    );
}

fn assert_first_await_index(data: &AnalysisData, expected: usize) {
    assert_eq!(
        data.blocker_data().first_await_index(),
        Some(expected),
        "first_await_index mismatch"
    );
}

fn assert_symbol_blocker(data: &AnalysisData, name: &str, expected_index: u32) {
    let root = data.scoping.root_scope_id();
    let sym = data
        .scoping
        .find_binding(root, name)
        .unwrap_or_else(|| panic!("no symbol '{name}'"));
    let actual = data
        .blocker_data()
        .symbol_blocker(sym)
        .unwrap_or_else(|| panic!("symbol '{name}' has no blocker"));
    assert_eq!(actual, expected_index, "blocker index for '{name}'");
}

fn assert_no_symbol_blocker(data: &AnalysisData, name: &str) {
    let root = data.scoping.root_scope_id();
    let sym = data
        .scoping
        .find_binding(root, name)
        .unwrap_or_else(|| panic!("no symbol '{name}'"));
    assert!(
        data.blocker_data().symbol_blocker(sym).is_none(),
        "expected symbol '{name}' to have no blocker"
    );
}

fn assert_stmt_meta_count(data: &AnalysisData, expected: usize) {
    assert_eq!(
        data.blocker_data().stmt_metas.len(),
        expected,
        "stmt_metas count mismatch"
    );
}

fn assert_stmt_meta_has_await(data: &AnalysisData, stmt_index: usize, expected: bool) {
    let meta = data
        .blocker_data()
        .stmt_meta(stmt_index)
        .unwrap_or_else(|| panic!("no stmt_meta at index {stmt_index}"));
    assert_eq!(
        meta.has_await(),
        expected,
        "stmt_meta[{stmt_index}].has_await"
    );
}

fn assert_stmt_meta_hoist_names(data: &AnalysisData, stmt_index: usize, expected: &[&str]) {
    let meta = data
        .blocker_data()
        .stmt_meta(stmt_index)
        .unwrap_or_else(|| panic!("no stmt_meta at index {stmt_index}"));
    let actual: Vec<&str> = meta.hoist_names().iter().map(|s| s.as_str()).collect();
    assert_eq!(actual, expected, "stmt_meta[{stmt_index}].hoist_names");
}

fn assert_expr_tag_has_blockers(data: &AnalysisData, component: &Component, expr_text: &str) {
    let id = find_expr_tag(&component.fragment, component, expr_text)
        .unwrap_or_else(|| panic!("no ExpressionTag with source '{expr_text}'"));
    assert!(
        data.expr_has_blockers(id),
        "expected ExpressionTag '{expr_text}' to have blockers"
    );
}

fn assert_expr_tag_no_blockers(data: &AnalysisData, component: &Component, expr_text: &str) {
    let id = find_expr_tag(&component.fragment, component, expr_text)
        .unwrap_or_else(|| panic!("no ExpressionTag with source '{expr_text}'"));
    assert!(
        !data.expr_has_blockers(id),
        "expected ExpressionTag '{expr_text}' to have no blockers"
    );
}

// -----------------------------------------------------------------------
// Blocker tracking tests
// -----------------------------------------------------------------------

#[test]
fn blocker_no_await_no_async() {
    let (_c, data) = analyze_source(r#"<script>let x = 1;</script><p>{x}</p>"#);
    assert_no_async(&data);
}

#[test]
fn blocker_single_await() {
    let (_c, data) =
        analyze_source(r#"<script>let data = await fetch('/api');</script><p>{data}</p>"#);
    assert_has_async(&data);
    assert_first_await_index(&data, 0);
    assert_symbol_blocker(&data, "data", 0);
}

#[test]
fn blocker_sync_before_await() {
    let (c, data) = analyze_source(
        r#"<script>
let x = 1;
let data = await fetch('/api');
</script>
<p>{x}</p><p>{data}</p>"#,
    );
    assert_has_async(&data);
    // x is statement 0 (sync), data is statement 1 (first await)
    assert_first_await_index(&data, 1);
    assert_no_symbol_blocker(&data, "x");
    assert_symbol_blocker(&data, "data", 0);
    assert_expr_tag_no_blockers(&data, &c, "x");
    assert_expr_tag_has_blockers(&data, &c, "data");
}

#[test]
fn blocker_multiple_await_statements() {
    let (_c, data) = analyze_source(
        r#"<script>
let a = await fetch('/a');
let b = await fetch('/b');
</script>
<p>{a}{b}</p>"#,
    );
    assert_has_async(&data);
    assert_first_await_index(&data, 0);
    assert_symbol_blocker(&data, "a", 0);
    assert_symbol_blocker(&data, "b", 1);
}

#[test]
fn blocker_fragment_indices_are_sorted_and_unique() {
    let (component, data) = analyze_source(
        r#"<script>
let a = await fetch('/a');
let b = await fetch('/b');
</script>
<p>{b}{a}{b}</p>"#,
    );
    let paragraph = find_element(&component.fragment, &component, "p")
        .unwrap_or_else(|| panic!("no <p> element"));
    assert_eq!(
        data.template
            .fragments
            .fragment_blockers(&FragmentKey::Element(paragraph.id)),
        &[0, 1]
    );
}

#[test]
fn debug_tag_ids_collected_for_fragment() {
    let (component, data) =
        analyze_source(r#"<script>let a = 1; let b = 2;</script>{@debug a}{@debug b}"#);
    let expected: Vec<NodeId> = component
        .fragment
        .nodes
        .iter()
        .filter_map(|&id| match component.store.get(id) {
            Node::DebugTag(tag) => Some(tag.id),
            _ => None,
        })
        .collect();
    assert_eq!(
        data.template.debug_tags.by_fragment(&FragmentKey::Root),
        Some(&expected)
    );
}

#[test]
fn title_elements_collected_for_svelte_head_fragment() {
    let (component, data) = analyze_source(
        r#"<svelte:head><title>Hello</title><meta name="x" content="y" /></svelte:head>"#,
    );
    let head_id = find_svelte_head_id(&component.fragment, &component)
        .unwrap_or_else(|| panic!("no <svelte:head>"));
    let title = find_element(&component.fragment, &component, "title")
        .unwrap_or_else(|| panic!("no <title>"));
    assert_eq!(
        data.template
            .title_elements
            .by_fragment(&FragmentKey::SvelteHeadBody(head_id)),
        Some(&vec![title.id])
    );
}

#[test]
fn html_tag_namespace_flags_preserved() {
    let (component, data) = analyze_source(
        r#"<script>let svgContent = ''; let mathContent = '';</script><svg>{@html svgContent}</svg><math>{@html mathContent}</math>"#,
    );
    let svg_tag = find_html_tag_id(&component.fragment, &component, "svgContent")
        .unwrap_or_else(|| panic!("no svg html tag"));
    let math_tag = find_html_tag_id(&component.fragment, &component, "mathContent")
        .unwrap_or_else(|| panic!("no mathml html tag"));
    assert!(data.html_tag_in_svg(svg_tag));
    assert!(!data.html_tag_in_mathml(svg_tag));
    assert!(data.html_tag_in_mathml(math_tag));
    assert!(!data.html_tag_in_svg(math_tag));
}

#[test]
fn annotation_xml_resets_child_namespace_to_html() {
    let (component, data) = analyze_source(
        r#"<svelte:options namespace="mathml" />
<annotation-xml><div></div></annotation-xml>"#,
    );
    let annotation = find_element(&component.fragment, &component, "annotation-xml")
        .unwrap_or_else(|| panic!("missing <annotation-xml>"));
    let div = find_element(&component.fragment, &component, "div")
        .unwrap_or_else(|| panic!("missing <div>"));

    assert_eq!(
        data.namespace(annotation.id),
        Some(NamespaceKind::AnnotationXml)
    );
    assert_eq!(
        data.namespace(div.id).map(NamespaceKind::as_namespace),
        Some(svelte_ast::Namespace::Html)
    );
}

#[test]
fn component_children_lowering_preserves_default_and_named_slot_fragments() {
    let (component, data) =
        analyze_source(r#"<Comp><p>default</p><p slot="footer">footer</p></Comp>"#);
    let component_id = find_component_node_id(&component.fragment, &component, "Comp")
        .unwrap_or_else(|| panic!("no <Comp>"));
    let default_child = find_nth_element(&component.fragment, &component, "p", 0)
        .unwrap_or_else(|| panic!("no default <p>"));
    let slotted_child = find_nth_element(&component.fragment, &component, "p", 1)
        .unwrap_or_else(|| panic!("no slotted <p>"));

    let default_fragment = data
        .template
        .fragments
        .lowered(&FragmentKey::ComponentNode(component_id))
        .unwrap_or_else(|| panic!("no lowered default fragment"));
    assert!(matches!(
        default_fragment.items.as_slice(),
        [FragmentItem::Element(id)] if *id == default_child.id
    ));

    let named_slots = data.template.snippets.component_named_slots(component_id);
    assert_eq!(named_slots.len(), 1);
    let (slot_el_id, slot_key) = named_slots[0];
    assert_eq!(slot_el_id, slotted_child.id);
    assert_eq!(
        slot_key,
        FragmentKey::NamedSlot(component_id, slotted_child.id)
    );

    let slot_fragment = data
        .template
        .fragments
        .lowered(&slot_key)
        .unwrap_or_else(|| panic!("no lowered named slot fragment"));
    assert!(matches!(
        slot_fragment.items.as_slice(),
        [FragmentItem::Element(id)] if *id == slotted_child.id
    ));
}

#[test]
fn slot_element_legacy_root_fragment_uses_dedicated_lowered_item() {
    let (component, data) = analyze_source_with_options(
        r#"<slot><p>fallback</p></slot>"#,
        AnalyzeOptions {
            runes: false,
            ..AnalyzeOptions::default()
        },
    );
    let slot_id = match component.store.get(component.fragment.nodes[0]) {
        Node::SlotElementLegacy(el) => el.id,
        _ => panic!("expected SlotElementLegacy at root"),
    };

    let root_fragment = data
        .template
        .fragments
        .lowered(&FragmentKey::Root)
        .unwrap_or_else(|| panic!("no lowered root fragment"));
    assert!(matches!(
        root_fragment.items.as_slice(),
        [FragmentItem::SlotElementLegacy(id)] if *id == slot_id
    ));
}

#[test]
fn legacy_slots_template_reads_require_sanitized_slots_binding() {
    let (_component, data) = analyze_source_with_options(
        r#"{#if $$slots.description}<slot name="description" />{/if}"#,
        AnalyzeOptions {
            runes: false,
            ..AnalyzeOptions::default()
        },
    );

    assert!(data.output.needs_sanitized_legacy_slots);
}

#[test]
fn legacy_slot_elements_do_not_require_sanitized_slots_binding() {
    let (_component, data) = analyze_source_with_options(
        r#"<slot name="description" />"#,
        AnalyzeOptions {
            runes: false,
            ..AnalyzeOptions::default()
        },
    );

    assert!(!data.output.needs_sanitized_legacy_slots);
}

#[test]
fn legacy_slots_script_reads_require_sanitized_slots_binding() {
    let (_component, data) = analyze_source_with_options(
        r#"<script>const has_description = !!$$slots.description;</script>"#,
        AnalyzeOptions {
            runes: false,
            ..AnalyzeOptions::default()
        },
    );

    assert!(data.output.needs_sanitized_legacy_slots);
}

#[test]
fn custom_element_slots_collect_ordered_slot_names() {
    let (_component, data) = analyze_source_with_options(
        r#"<svelte:options customElement="my-layout" />
<header><slot name="actions" /></header>
<main><slot /></main>"#,
        AnalyzeOptions::default(),
    );

    let slot_names: Vec<_> = data
        .custom_element_slot_names()
        .iter()
        .map(String::as_str)
        .collect();
    assert_eq!(slot_names, vec!["actions", "default"]);
}

#[test]
fn component_named_slot_mapping_uses_svelte_fragment_legacy_wrapper_id() {
    let (component, data) = analyze_source_with_options(
        r#"<Comp><svelte:fragment slot="footer"><p>footer</p></svelte:fragment></Comp>"#,
        AnalyzeOptions {
            runes: false,
            ..AnalyzeOptions::default()
        },
    );
    let component_id = find_component_node_id(&component.fragment, &component, "Comp")
        .unwrap_or_else(|| panic!("no <Comp>"));
    let component_node = component
        .fragment
        .nodes
        .iter()
        .find_map(|id| match component.store.get(*id) {
            Node::ComponentNode(cn) if cn.id == component_id => Some(cn),
            _ => None,
        })
        .unwrap_or_else(|| panic!("missing component node"));
    let (wrapper_id, wrapped_child_id) = match component.store.get(component_node.fragment.nodes[0])
    {
        Node::SvelteFragmentLegacy(el) => {
            let child_id = match component.store.get(el.fragment.nodes[0]) {
                Node::Element(child) => child.id,
                _ => panic!("expected wrapped child element inside SvelteFragmentLegacy"),
            };
            (el.id, child_id)
        }
        _ => panic!("expected SvelteFragmentLegacy as first component child"),
    };

    let named_slots = data.template.snippets.component_named_slots(component_id);
    assert_eq!(named_slots.len(), 1);
    let (slot_el_id, slot_key) = named_slots[0];
    assert_eq!(slot_el_id, wrapper_id);
    assert!(data
        .elements
        .flags
        .svelte_fragment_slots
        .contains(&wrapper_id));
    assert_eq!(slot_key, FragmentKey::NamedSlot(component_id, wrapper_id));

    let slot_fragment = data
        .template
        .fragments
        .lowered(&slot_key)
        .unwrap_or_else(|| panic!("no lowered named slot fragment"));
    assert!(matches!(
        slot_fragment.items.as_slice(),
        [FragmentItem::Element(id)] if *id == wrapped_child_id
    ));
}

#[test]
fn component_child_slot_attribute_lowers_child_component_into_named_slot() {
    let (component, data) = analyze_source_with_options(
        r#"<Outer><Inner slot="footer" /></Outer>"#,
        AnalyzeOptions {
            runes: false,
            ..AnalyzeOptions::default()
        },
    );
    let outer_id = find_component_node_id(&component.fragment, &component, "Outer")
        .unwrap_or_else(|| panic!("no <Outer>"));
    let outer_node = match component.store.get(outer_id) {
        Node::ComponentNode(node) => node,
        _ => panic!("<Outer> did not lower to ComponentNode"),
    };
    let inner_id = match outer_node.fragment.nodes.as_slice() {
        [child_id] => match component.store.get(*child_id) {
            Node::ComponentNode(node) => node.id,
            _ => panic!("expected direct child <Inner> component"),
        },
        _ => panic!("expected exactly one direct child for <Outer>"),
    };

    let default_fragment = data
        .template
        .fragments
        .lowered(&FragmentKey::ComponentNode(outer_id))
        .unwrap_or_else(|| panic!("no lowered default fragment"));
    assert!(default_fragment.items.is_empty());

    let named_slots = data.template.snippets.component_named_slots(outer_id);
    assert_eq!(named_slots.len(), 1);
    let (slot_el_id, slot_key) = named_slots[0];
    assert_eq!(slot_el_id, inner_id);
    assert_eq!(slot_key, FragmentKey::NamedSlot(outer_id, inner_id));

    let slot_fragment = data
        .template
        .fragments
        .lowered(&slot_key)
        .unwrap_or_else(|| panic!("no lowered named slot fragment"));
    assert!(matches!(
        slot_fragment.items.as_slice(),
        [FragmentItem::ComponentNode(id)] if *id == inner_id
    ));
}

#[test]
fn component_default_slot_bindings_do_not_leak_into_named_slot_scope() {
    let (component, data) = analyze_source_with_options(
        r#"<Nested let:count><p>{count}</p><p slot="bar">{count}</p></Nested>"#,
        AnalyzeOptions {
            runes: false,
            ..AnalyzeOptions::default()
        },
    );
    let nested_id = find_component_node_id(&component.fragment, &component, "Nested")
        .unwrap_or_else(|| panic!("no <Nested>"));
    let default_scope = data
        .scoping
        .fragment_scope(&FragmentKey::ComponentNode(nested_id))
        .unwrap_or_else(|| panic!("no default slot scope"));
    let default_count = data
        .scoping
        .get_binding(default_scope, "count")
        .unwrap_or_else(|| panic!("no default-slot binding for count"));

    let default_p = find_nth_element(&component.fragment, &component, "p", 0)
        .unwrap_or_else(|| panic!("missing default-slot <p>"));
    let named_p = find_nth_element(&component.fragment, &component, "p", 1)
        .unwrap_or_else(|| panic!("missing named-slot <p>"));
    let named_scope = data
        .scoping
        .fragment_scope(&FragmentKey::NamedSlot(nested_id, named_p.id))
        .unwrap_or_else(|| panic!("no named slot scope"));

    assert!(
        data.scoping.get_binding(named_scope, "count").is_none(),
        "default-slot binding must not resolve in named slot scope"
    );

    let default_expr = match component.store.get(default_p.fragment.nodes[0]) {
        Node::ExpressionTag(tag) => data
            .expression(tag.id)
            .unwrap_or_else(|| panic!("missing expression info for default slot")),
        _ => panic!("expected default slot expression tag"),
    };
    assert!(default_expr.ref_symbols().contains(&default_count));

    let named_expr = match component.store.get(named_p.fragment.nodes[0]) {
        Node::ExpressionTag(tag) => data
            .expression(tag.id)
            .unwrap_or_else(|| panic!("missing expression info for named slot")),
        _ => panic!("expected named slot expression tag"),
    };
    assert!(
        !named_expr.ref_symbols().contains(&default_count),
        "named-slot expression must not resolve default-slot binding"
    );
}

#[test]
fn legacy_slot_let_alias_uses_statement_backed_binding() {
    let (component, data, parsed) =
        analyze_source_with_parsed(r#"<List let:item={processed}><p>{processed}</p></List>"#);
    let list_id = find_component_node_id(&component.fragment, &component, "List")
        .unwrap_or_else(|| panic!("no <List>"));
    let list_node = match component.store.get(list_id) {
        Node::ComponentNode(node) => node,
        _ => panic!("<List> did not lower to ComponentNode"),
    };
    let let_dir = find_let_directive(&list_node.attributes)
        .unwrap_or_else(|| panic!("missing default-slot let directive"));

    let stmt_handle = data
        .let_directive_stmt_handle(let_dir.id)
        .unwrap_or_else(|| panic!("missing statement handle for let directive"));
    let stmt = parsed
        .stmt(stmt_handle)
        .unwrap_or_else(|| panic!("missing parsed statement for let directive"));
    match stmt {
        Statement::VariableDeclaration(decl) => match &decl.declarations[0].id {
            BindingPattern::BindingIdentifier(id) => assert_eq!(id.name.as_str(), "processed"),
            _ => panic!("alias let directive must parse as identifier binding"),
        },
        _ => panic!("let directive statement must be a variable declaration"),
    }

    let default_scope = data
        .scoping
        .fragment_scope(&FragmentKey::ComponentNode(list_id))
        .unwrap_or_else(|| panic!("missing default slot scope"));
    assert!(
        data.scoping.get_binding(default_scope, "item").is_none(),
        "alias let directive must not declare the slot prop name"
    );
    data.scoping
        .get_binding(default_scope, "processed")
        .unwrap_or_else(|| panic!("alias let directive must declare processed"));
}

#[test]
fn legacy_slot_let_destructure_registers_carrier_binding() {
    let (component, data, parsed) = analyze_source_with_parsed(
        r#"<List><svelte:fragment slot="item" let:item={{ text }}><p>{text}</p></svelte:fragment></List>"#,
    );
    let list_id = find_component_node_id(&component.fragment, &component, "List")
        .unwrap_or_else(|| panic!("no <List>"));
    let list_node = match component.store.get(list_id) {
        Node::ComponentNode(node) => node,
        _ => panic!("<List> did not lower to ComponentNode"),
    };
    let wrapper_id = match list_node.fragment.nodes.as_slice() {
        [child_id] => match component.store.get(*child_id) {
            Node::SvelteFragmentLegacy(node) => node.id,
            _ => panic!("expected slotted <svelte:fragment> child"),
        },
        _ => panic!("expected exactly one slotted child"),
    };
    let wrapper = match component.store.get(wrapper_id) {
        Node::SvelteFragmentLegacy(node) => node,
        _ => panic!("wrapper id did not resolve to <svelte:fragment>"),
    };
    let let_dir = find_let_directive(&wrapper.attributes)
        .unwrap_or_else(|| panic!("missing named-slot let directive"));

    let stmt_handle = data
        .let_directive_stmt_handle(let_dir.id)
        .unwrap_or_else(|| panic!("missing statement handle for destructured let directive"));
    let stmt = parsed
        .stmt(stmt_handle)
        .unwrap_or_else(|| panic!("missing parsed statement for destructured let directive"));
    match stmt {
        Statement::VariableDeclaration(decl) => {
            assert!(
                matches!(decl.declarations[0].id, BindingPattern::ObjectPattern(_)),
                "destructured let directive must parse as object binding pattern"
            );
        }
        _ => panic!("let directive statement must be a variable declaration"),
    }

    let named_scope = data
        .scoping
        .fragment_scope(&FragmentKey::NamedSlot(list_id, wrapper_id))
        .unwrap_or_else(|| panic!("missing named slot scope"));
    data.scoping
        .get_binding(named_scope, "text")
        .unwrap_or_else(|| panic!("missing destructured slot binding"));
}

#[test]
fn reactivity_semantics_declaration_semantics_cover_state_and_props() {
    let (_component, data) = analyze_source(
        r#"<svelte:options runes={true} />
<script>
    import { writable } from 'svelte/store';
    var count = $state({});
    count = {};
    let total = $derived(count.value);
    let { foo = 1, bar = $bindable(2), baz, ...rest } = $props();
    const store = writable(0);
    $store;
</script>"#,
    );

    assert!(matches!(
        symbol_declaration_semantics(&data, "count"),
        DeclarationSemantics::State(StateDeclarationSemantics {
            kind: StateKind::State,
            proxied: true,
            var_declared: true,
            ..
        })
    ));
    assert!(matches!(
        symbol_declaration_semantics(&data, "total"),
        DeclarationSemantics::Derived(DerivedDeclarationSemantics {
            kind: DerivedKind::Derived,
            lowering: DerivedLowering::Sync,
            ..
        })
    ));
    assert_eq!(
        symbol_declaration_semantics(&data, "foo"),
        DeclarationSemantics::Prop(PropDeclarationSemantics {
            lowering_mode: PropLoweringMode::Standard,
            kind: PropDeclarationKind::Source {
                bindable: false,
                updated: false,
                default_lowering: PropDefaultLowering::Eager,
                default_needs_proxy: false,
            },
        })
    );
    assert_eq!(
        symbol_declaration_semantics(&data, "bar"),
        DeclarationSemantics::Prop(PropDeclarationSemantics {
            lowering_mode: PropLoweringMode::Standard,
            kind: PropDeclarationKind::Source {
                bindable: true,
                updated: false,
                default_lowering: PropDefaultLowering::Eager,
                default_needs_proxy: false,
            },
        })
    );
    assert_eq!(
        symbol_declaration_semantics(&data, "baz"),
        DeclarationSemantics::Prop(PropDeclarationSemantics {
            lowering_mode: PropLoweringMode::Standard,
            kind: PropDeclarationKind::NonSource,
        })
    );
    assert_eq!(
        symbol_declaration_semantics(&data, "rest"),
        DeclarationSemantics::Prop(PropDeclarationSemantics {
            lowering_mode: PropLoweringMode::Standard,
            kind: PropDeclarationKind::Rest,
        })
    );
    let store_sym = data
        .scoping
        .find_binding_in_any_scope("store")
        .expect("missing binding 'store'");
    assert_eq!(
        symbol_declaration_semantics(&data, "store"),
        DeclarationSemantics::Store(StoreDeclarationSemantics {
            base_symbol: store_sym,
        })
    );
}

#[test]
fn reactivity_semantics_prop_declaration_semantics_include_updated() {
    let (_component, data) = analyze_source_with_options(
        r#"<svelte:options runes={true} />
<script>
    let { count } = $props();
    count = 1;
</script>"#,
        AnalyzeOptions {
            runes: true,
            ..Default::default()
        },
    );

    assert_eq!(
        symbol_declaration_semantics(&data, "count"),
        DeclarationSemantics::Prop(PropDeclarationSemantics {
            lowering_mode: PropLoweringMode::Standard,
            kind: PropDeclarationKind::Source {
                bindable: false,
                updated: true,
                default_lowering: PropDefaultLowering::None,
                default_needs_proxy: false,
            },
        })
    );
}

#[test]
fn reactivity_semantics_prop_declaration_semantics_include_default_proxy() {
    let (_component, data) = analyze_source_with_options(
        r#"<svelte:options runes={true} />
<script>
    let { value = $bindable({ answer: 42 }) } = $props();
</script>"#,
        AnalyzeOptions {
            runes: true,
            ..Default::default()
        },
    );

    assert_eq!(
        symbol_declaration_semantics(&data, "value"),
        DeclarationSemantics::Prop(PropDeclarationSemantics {
            lowering_mode: PropLoweringMode::Standard,
            kind: PropDeclarationKind::Source {
                bindable: true,
                updated: false,
                default_lowering: PropDefaultLowering::Lazy,
                default_needs_proxy: true,
            },
        })
    );
}

#[test]
fn reactivity_semantics_declaration_semantics_distinguish_derived_lowering() {
    let (_component, data) = analyze_source_with_options(
        r#"<svelte:options runes={true} />
<script>
    let sync_total = $derived(1 + 1);
    let async_total = $derived(await load());
    let mapped = $derived.by(() => 1 + 1);
</script>"#,
        AnalyzeOptions {
            experimental_async: true,
            ..AnalyzeOptions::default()
        },
    );

    assert!(matches!(
        symbol_declaration_semantics(&data, "sync_total"),
        DeclarationSemantics::Derived(DerivedDeclarationSemantics {
            kind: DerivedKind::Derived,
            lowering: DerivedLowering::Sync,
            ..
        })
    ));
    assert!(matches!(
        symbol_declaration_semantics(&data, "async_total"),
        DeclarationSemantics::Derived(DerivedDeclarationSemantics {
            kind: DerivedKind::Derived,
            lowering: DerivedLowering::Async,
            ..
        })
    ));
    assert!(matches!(
        symbol_declaration_semantics(&data, "mapped"),
        DeclarationSemantics::Derived(DerivedDeclarationSemantics {
            kind: DerivedKind::DerivedBy,
            lowering: DerivedLowering::Sync,
            ..
        })
    ));
}

#[test]
fn reactivity_semantics_v2_marks_destructured_state_bindings_as_proxied() {
    let (_component, data) = analyze_source(
        r#"<svelte:options runes={true} />
<script>
    let { left, nested: { right }, ...rest } = $state({ left: 1, nested: { right: 2 } });
    let [first, second = 2, ...tail] = $state([1, 2, 3]);
    left = 1;
    tail = [];
</script>"#,
    );

    let state_decl = |name: &str| -> StateDeclarationSemantics {
        match symbol_declaration_semantics(&data, name) {
            DeclarationSemantics::State(s) => s,
            other => panic!("expected State(...) for '{name}', got {other:?}"),
        }
    };

    let object_decl = state_decl("left");
    assert_eq!(object_decl.kind, StateKind::State);
    assert!(object_decl.proxied);
    assert!(!object_decl.var_declared);
    assert_eq!(
        object_decl.binding_semantics.as_slice(),
        &[
            StateBindingSemantics::StateSignal { proxied: true },
            StateBindingSemantics::NonReactive { proxied: true },
            StateBindingSemantics::NonReactive { proxied: true },
        ],
        "object-destructure binding_semantics mismatch"
    );

    for name in ["right", "rest"] {
        assert_eq!(
            state_decl(name).binding_semantics,
            object_decl.binding_semantics,
            "expected '{name}' to share the object-destructure declaration root"
        );
    }

    let array_decl = state_decl("tail");
    assert_eq!(array_decl.kind, StateKind::State);
    assert!(array_decl.proxied);
    assert_eq!(
        array_decl.binding_semantics.as_slice(),
        &[
            StateBindingSemantics::NonReactive { proxied: true },
            StateBindingSemantics::NonReactive { proxied: true },
            StateBindingSemantics::StateSignal { proxied: true },
        ],
        "array-destructure binding_semantics mismatch"
    );

    for name in ["first", "second"] {
        assert_eq!(
            state_decl(name).binding_semantics,
            array_decl.binding_semantics,
            "expected '{name}' to share the array-destructure declaration root"
        );
    }
}

#[test]
fn reactivity_semantics_v2_reference_semantics_cover_first_cluster() {
    let source = r#"<svelte:options runes={true} />
<script>
    import { writable } from 'svelte/store';

    var count = $state(0);
    let total = $derived(count * 2);
    const store = writable(0);
    let { foo = 1, bar = $bindable(2), baz = $bindable(), ...rest } = $props();
    let local = 1;

    count;
    count = 1;
    count += 1;
    total;
    total = 1;
    $store;
    $store = 1;
    foo;
    baz;
    bar = 1;
    rest;
    local = 2;
</script>"#;
    let alloc = Box::leak(Box::new(oxc_allocator::Allocator::default()));
    let (component, js_result, parse_diags) = svelte_parser::parse_with_js(alloc, source);
    assert!(
        parse_diags.is_empty(),
        "unexpected parse diagnostics: {parse_diags:?}"
    );
    let options = AnalyzeOptions {
        warning_filter: Some(Box::new(|_| false)),
        ..AnalyzeOptions::default()
    };
    let (data, parsed, diags) = analyze_with_options(&component, js_result, &options);
    assert!(diags.is_empty(), "unexpected diagnostics: {diags:?}");

    assert_eq!(
        script_reference_semantics(&data, &parsed, "count", true, false, 0),
        ReferenceSemantics::SignalRead {
            kind: SignalReferenceKind::State(StateKind::State),
            safe: true,
        }
    );
    assert_eq!(
        script_reference_semantics(&data, &parsed, "count", false, true, 0),
        ReferenceSemantics::SignalWrite {
            kind: StateKind::State,
        }
    );
    assert_eq!(
        script_reference_semantics(&data, &parsed, "count", true, true, 0),
        ReferenceSemantics::SignalUpdate {
            kind: StateKind::State,
            safe: true,
        }
    );
    assert_eq!(
        script_reference_semantics(&data, &parsed, "total", true, false, 0),
        ReferenceSemantics::SignalRead {
            kind: SignalReferenceKind::Derived(DerivedKind::Derived),
            safe: false,
        }
    );
    assert_eq!(
        script_reference_semantics(&data, &parsed, "total", false, true, 0),
        ReferenceSemantics::IllegalWrite
    );
    assert!(matches!(
        script_reference_semantics(&data, &parsed, "foo", true, false, 0),
        ReferenceSemantics::PropRead(PropReferenceSemantics::Source {
            bindable: false,
            lowering_mode: PropLoweringMode::Standard,
            ..
        })
    ));
    assert!(matches!(
        script_reference_semantics(&data, &parsed, "baz", true, false, 0),
        ReferenceSemantics::PropRead(PropReferenceSemantics::NonSource { .. })
    ));
    assert!(matches!(
        script_reference_semantics(&data, &parsed, "bar", false, true, 0),
        ReferenceSemantics::PropMutation { bindable: true, .. }
    ));
    assert_eq!(
        script_reference_semantics(&data, &parsed, "rest", true, false, 0),
        ReferenceSemantics::NonReactive
    );
    assert_eq!(
        script_reference_semantics(&data, &parsed, "local", false, true, 0),
        ReferenceSemantics::NonReactive
    );
}

#[test]
fn reactivity_semantics_v2_state_references_distinguish_plain_and_mutated_reads() {
    let source = r#"<svelte:options runes={true} />
<script>
    let plain = $state(0);
    let { left, nested: { right } } = $state({ left: 1, nested: { right: 2 } });
    var safe = $state(0);

    plain;
    left = 1;
    left;
    right += 1;
    safe = 1;
    safe;
</script>"#;
    let alloc = Box::leak(Box::new(oxc_allocator::Allocator::default()));
    let (component, js_result, parse_diags) = svelte_parser::parse_with_js(alloc, source);
    assert!(
        parse_diags.is_empty(),
        "unexpected parse diagnostics: {parse_diags:?}"
    );
    let options = AnalyzeOptions {
        warning_filter: Some(Box::new(|_| false)),
        ..AnalyzeOptions::default()
    };
    let (data, parsed, diags) = analyze_with_options(&component, js_result, &options);
    assert!(diags.is_empty(), "unexpected diagnostics: {diags:?}");

    assert_eq!(
        script_reference_semantics(&data, &parsed, "plain", true, false, 0),
        ReferenceSemantics::NonReactive
    );
    assert_eq!(
        script_reference_semantics(&data, &parsed, "left", false, true, 0),
        ReferenceSemantics::SignalWrite {
            kind: StateKind::State,
        }
    );
    assert_eq!(
        script_reference_semantics(&data, &parsed, "left", true, false, 0),
        ReferenceSemantics::SignalRead {
            kind: SignalReferenceKind::State(StateKind::State),
            safe: false,
        }
    );
    assert_eq!(
        script_reference_semantics(&data, &parsed, "right", true, true, 0),
        ReferenceSemantics::SignalUpdate {
            kind: StateKind::State,
            safe: false,
        }
    );
    assert_eq!(
        script_reference_semantics(&data, &parsed, "safe", true, false, 0),
        ReferenceSemantics::SignalRead {
            kind: SignalReferenceKind::State(StateKind::State),
            safe: true,
        }
    );
}

#[test]
fn reactivity_semantics_v2_state_raw_distinguishes_plain_and_mutated_bindings() {
    let source = r#"<svelte:options runes={true} />
<script>
    let plain = $state.raw(0);
    var safe = $state.raw(0);

    safe = 1;
    plain;
    safe;
    safe += 1;
</script>"#;
    let alloc = Box::leak(Box::new(oxc_allocator::Allocator::default()));
    let (component, js_result, parse_diags) = svelte_parser::parse_with_js(alloc, source);
    assert!(
        parse_diags.is_empty(),
        "unexpected parse diagnostics: {parse_diags:?}"
    );
    let options = AnalyzeOptions {
        warning_filter: Some(Box::new(|_| false)),
        ..AnalyzeOptions::default()
    };
    let (data, parsed, diags) = analyze_with_options(&component, js_result, &options);
    assert!(diags.is_empty(), "unexpected diagnostics: {diags:?}");

    assert!(matches!(
        symbol_declaration_semantics(&data, "plain"),
        DeclarationSemantics::OptimizedRune(crate::OptimizedRuneSemantics {
            kind: StateKind::StateRaw,
            ..
        })
    ));
    assert!(matches!(
        symbol_declaration_semantics(&data, "safe"),
        DeclarationSemantics::State(StateDeclarationSemantics {
            kind: StateKind::StateRaw,
            proxied: false,
            var_declared: true,
            ..
        })
    ));
    assert_eq!(
        script_reference_semantics(&data, &parsed, "plain", true, false, 0),
        ReferenceSemantics::NonReactive
    );
    assert_eq!(
        script_reference_semantics(&data, &parsed, "safe", true, false, 0),
        ReferenceSemantics::SignalRead {
            kind: SignalReferenceKind::State(StateKind::StateRaw),
            safe: true,
        }
    );
    assert_eq!(
        script_reference_semantics(&data, &parsed, "safe", true, true, 0),
        ReferenceSemantics::SignalUpdate {
            kind: StateKind::StateRaw,
            safe: true,
        }
    );
}

#[test]
fn reactivity_semantics_collects_contextual_binding_owners() {
    let (_component, data) = analyze_source(
        r#"<script>
    let items = [{ text: 'a' }];
    let promise = Promise.resolve({ value: 1 });
</script>

{#snippet row(entry, { fallback = 1 })}
    <p>{entry} {fallback}</p>
{/snippet}

<List let:item={{ text }} let:item2={processed}>
    <p>{text} {processed}</p>
</List>

{#each items as item, index}
    <span>{item.text} {index}</span>
{/each}

{#await promise then result}
    <p>{result.value}</p>
{:catch error}
    <p>{error.message}</p>
{/await}"#,
    );

    assert_eq!(
        symbol_declaration_semantics(&data, "text"),
        DeclarationSemantics::Contextual(ContextualDeclarationSemantics::LetDirective)
    );
    assert_eq!(
        symbol_declaration_semantics(&data, "item"),
        DeclarationSemantics::Contextual(ContextualDeclarationSemantics::EachItem(
            crate::EachItemStrategy::Signal
        ))
    );
    assert_eq!(
        symbol_declaration_semantics(&data, "index"),
        DeclarationSemantics::Contextual(ContextualDeclarationSemantics::EachIndex(
            crate::EachIndexStrategy::Direct
        ))
    );
    assert_eq!(
        symbol_declaration_semantics(&data, "entry"),
        DeclarationSemantics::Contextual(ContextualDeclarationSemantics::SnippetParam(
            crate::SnippetParamStrategy::Accessor
        ))
    );
    assert_eq!(
        symbol_declaration_semantics(&data, "result"),
        DeclarationSemantics::Contextual(ContextualDeclarationSemantics::AwaitValue)
    );
    assert_eq!(
        symbol_declaration_semantics(&data, "error"),
        DeclarationSemantics::Contextual(ContextualDeclarationSemantics::AwaitError)
    );
}

#[test]
fn blocker_function_decl_no_blocker() {
    let (_c, data) = analyze_source(
        r#"<script>
let data = await fetch('/api');
function helper() { return data; }
</script>
<p>{data}</p>"#,
    );
    assert_has_async(&data);
    assert_symbol_blocker(&data, "data", 0);
    // Function declarations get no blocker — stmt_meta for function has empty hoist_names
    assert_stmt_meta_count(&data, 2); // data decl + function decl
}

#[test]
fn blocker_function_valued_init_stays_sync() {
    let (_c, data) = analyze_source(
        r#"<script>
let data = await fetch('/api');
let fn1 = () => data;
</script>
<p>{data}</p>"#,
    );
    assert_has_async(&data);
    assert_symbol_blocker(&data, "data", 0);
    // fn1 has arrow function init — no blocker assigned
    assert_no_symbol_blocker(&data, "fn1");
}

#[test]
fn blocker_stmt_meta_hoist_names() {
    let (_c, data) = analyze_source(
        r#"<script>
let data = await fetch('/api');
let y = data.length;
</script>
<p>{data}</p>"#,
    );
    assert_has_async(&data);
    assert_first_await_index(&data, 0);
    // stmt 0: `let data = await fetch(...)` — has_await=true, hoist_names=["data"]
    assert_stmt_meta_has_await(&data, 0, true);
    assert_stmt_meta_hoist_names(&data, 0, &["data"]);
    // stmt 1: `let y = data.length` — has_await=false, hoist_names=["y"]
    assert_stmt_meta_has_await(&data, 1, false);
    assert_stmt_meta_hoist_names(&data, 1, &["y"]);
}

#[test]
fn blocker_expressions_marked_dynamic() {
    let (c, data) = analyze_source(
        r#"<script>
let x = 1;
let data = await fetch('/api');
</script>
<p>{data}</p>"#,
    );
    // Expressions referencing blocked symbols are marked dynamic
    assert_dynamic_tag(&data, &c, "data");
}

#[test]
fn blocker_needs_memoization_with_await() {
    let (c, data) = analyze_source(
        r#"<script>
let data = await fetch('/api');
</script>
{#if await check(data)}yes{/if}"#,
    );
    let block =
        find_if_block(&c.fragment, &c, "await check(data)").unwrap_or_else(|| panic!("no IfBlock"));
    assert!(
        data.needs_expr_memoization(block.id),
        "expression with await + ref_symbols should need memoization"
    );
}

#[test]
fn blocker_import_skipped_in_indexing() {
    let (_c, data) = analyze_source(
        r#"<script>
import { foo } from './foo';
let data = await fetch('/api');
</script>
<p>{data}</p>"#,
    );
    assert_has_async(&data);
    // Import is skipped — data is still non-import statement index 0
    assert_first_await_index(&data, 0);
    assert_symbol_blocker(&data, "data", 0);
}

#[test]
fn runtime_plan_basic_component_is_minimal() {
    let (_c, data) = analyze_source("<script>let count = 1;</script><p>{count}</p>");
    let plan = data.output.runtime_plan;

    assert!(!plan.needs_push);
    assert!(!plan.has_component_exports);
    assert!(!plan.has_exports);
    assert!(!plan.has_bindable);
    assert!(!plan.has_stores);
    assert!(!plan.has_ce_props);
    assert!(!plan.needs_props_param);
    assert!(!plan.needs_pop_with_return);
}

#[test]
fn runtime_plan_dev_custom_element_uses_exports_and_props() {
    let source = r#"
<svelte:options customElement={{ tag: 'x-foo' }} />
<script>
    let { answer = 42, extra } = $props();
    export function reset() {}
</script>
<p>{answer}{extra}</p>
"#;
    let (_c, data) = analyze_source_with_options(
        source,
        AnalyzeOptions {
            custom_element: true,
            experimental_async: false,
            runes: true,
            accessors: false,
            immutable: false,
            preserve_whitespace: false,
            dev: true,
            component_name: "Self".to_string(),
            filename_basename: "Self.svelte".to_string(),
            warning_filter: None,
        },
    );
    let plan = data.output.runtime_plan;

    assert!(plan.needs_push);
    assert!(plan.has_component_exports);
    assert!(plan.has_exports);
    assert!(!plan.has_bindable);
    assert!(plan.has_ce_props);
    assert!(plan.needs_props_param);
    assert!(plan.needs_pop_with_return);
}

#[test]
fn runtime_plan_bindable_props_require_push_without_component_exports() {
    let (_c, data) =
        analyze_source("<script>let { value = $bindable() } = $props();</script><p>{value}</p>");
    let plan = data.output.runtime_plan;

    assert!(plan.needs_push);
    assert!(!plan.has_component_exports);
    assert!(!plan.has_exports);
    assert!(plan.has_bindable);
    assert!(!plan.has_stores);
    assert!(!plan.has_ce_props);
    assert!(plan.needs_props_param);
    assert!(!plan.needs_pop_with_return);
}

#[test]
fn runtime_plan_store_subscriptions_do_not_force_push() {
    let (_c, data) =
        analyze_source("<script>import { count } from './stores';</script><p>{$count}</p>");
    let plan = data.output.runtime_plan;

    assert!(!plan.needs_push);
    assert!(!plan.has_component_exports);
    assert!(!plan.has_exports);
    assert!(!plan.has_bindable);
    assert!(plan.has_stores);
    assert!(!plan.has_ce_props);
    assert!(!plan.needs_props_param);
    assert!(!plan.needs_pop_with_return);
}

#[test]
fn legacy_export_let_becomes_props_when_runes_disabled() {
    let (_c, data) = analyze_source_with_options(
        "<script>export let count = 1;</script><p>{count}</p>",
        AnalyzeOptions {
            runes: false,
            ..AnalyzeOptions::default()
        },
    );

    let props = data
        .script
        .props
        .as_ref()
        .unwrap_or_else(|| panic!("expected props analysis in legacy mode"));
    assert_eq!(props.props.len(), 1);
    assert_eq!(props.props[0].local_name.as_str(), "count");
    assert_eq!(props.props[0].default_text.as_deref(), Some("1"));
    assert!(
        data.script.exports.is_empty(),
        "legacy export let should not remain a component export"
    );

    let plan = data.output.runtime_plan;
    assert!(!plan.needs_push);
    assert!(plan.needs_props_param);
    assert!(!plan.has_exports);
}

#[test]
fn runtime_plan_accessors_require_push_and_exports() {
    let (_c, data) = analyze_source_with_options(
        "<script>export let count = 1;</script><p>{count}</p>",
        AnalyzeOptions {
            runes: false,
            accessors: true,
            ..AnalyzeOptions::default()
        },
    );
    let plan = data.output.runtime_plan;

    assert!(plan.needs_push);
    assert!(plan.has_component_exports);
    assert!(plan.needs_props_param);
    assert!(data.script.accessors);
}

#[test]
fn runtime_plan_immutable_legacy_requires_push() {
    let (_c, data) = analyze_source_with_options(
        "<script>export let items = [1, 2];</script><p>{items.length}</p>",
        AnalyzeOptions {
            runes: false,
            immutable: true,
            ..AnalyzeOptions::default()
        },
    );
    let plan = data.output.runtime_plan;

    assert!(plan.needs_push);
    assert!(!plan.has_component_exports);
    assert!(plan.needs_props_param);
    assert!(data.script.immutable);
}

#[test]
fn runtime_plan_needs_context_without_exports_skips_pop_return() {
    let (_c, data) = analyze_source("<script>$effect(() => {});</script><p>ok</p>");
    let plan = data.output.runtime_plan;

    assert!(plan.needs_push);
    assert!(!plan.has_component_exports);
    assert!(!plan.has_exports);
    assert!(!plan.has_bindable);
    assert!(!plan.has_stores);
    assert!(!plan.has_ce_props);
    assert!(plan.needs_props_param);
    assert!(!plan.needs_pop_with_return);
}

#[test]
fn fragment_facts_capture_single_expression_queries() {
    let (component, data) = analyze_source(
        r#"<script>let value = $state('x');</script>
<textarea>{value}</textarea>
<option>{value}</option>"#,
    );

    let textarea = find_element(&component.fragment, &component, "textarea")
        .expect("expected textarea element");
    let option = find_element(&component.fragment, &component, "option").expect("expected option");
    let textarea_expr = data
        .fragment_single_expression_child(&FragmentKey::Element(textarea.id))
        .expect("expected textarea expression child");
    let option_expr = data
        .fragment_single_expression_child(&FragmentKey::Element(option.id))
        .expect("expected option expression child");

    assert!(data.fragment_has_expression_child(&FragmentKey::Element(textarea.id)));
    assert!(matches!(
        component.store.get(textarea_expr),
        Node::ExpressionTag(_)
    ));
    assert!(data
        .elements
        .flags
        .needs_textarea_value_lowering(textarea.id));

    assert!(data.fragment_has_expression_child(&FragmentKey::Element(option.id)));
    assert!(matches!(
        component.store.get(option_expr),
        Node::ExpressionTag(_)
    ));
    assert_eq!(
        data.elements.flags.option_synthetic_value_expr(option.id),
        Some(option_expr)
    );
}

#[test]
fn input_dynamic_special_attrs_mark_input_defaults() {
    let (component, data) = analyze_source(
        r#"<script>
let value = $state('');
let checked = $state(false);
let disabled = false;
</script>
<input {value} />
<input value={value} />
<input checked={checked} />
<input {disabled} />"#,
    );

    assert_nth_element_needs_input_defaults(&data, &component, "input", 0, true);
    assert_nth_element_needs_input_defaults(&data, &component, "input", 1, true);
    assert_nth_element_needs_input_defaults(&data, &component, "input", 2, true);
    assert_nth_element_needs_input_defaults(&data, &component, "input", 3, false);
}

#[test]
fn fragment_facts_track_each_body_child_shape_and_animate() {
    let (component, data) = analyze_source(
        r#"<script>import { flip } from 'svelte/animate'; let items = [{ id: 1, value: 'x' }];</script>
{#each items as item (item.id)}
    <!-- comment -->
    <div animate:flip>{item.value}</div>
{/each}"#,
    );

    let each = find_each_block(&component.fragment, &component, "items").expect("expected each");
    let div = find_element(&component.fragment, &component, "div").expect("expected div");

    assert_eq!(
        data.fragment_single_non_trivial_child(&FragmentKey::EachBody(each.id)),
        Some(div.id)
    );
    assert!(data.fragment_has_direct_animate_child(&FragmentKey::EachBody(each.id)));
    assert!(data.each_has_animate(each.id));
}

#[test]
fn fragment_facts_track_svelte_element_animate_in_each_body() {
    let (component, data) = analyze_source(
        r#"<script>import { flip } from 'svelte/animate'; let tag = 'div'; let items = [{ id: 1, value: 'x' }];</script>
{#each items as item (item.id)}
    <svelte:element this={tag} animate:flip>{item.value}</svelte:element>
{/each}"#,
    );

    let each = find_each_block(&component.fragment, &component, "items").expect("expected each");

    assert!(data.fragment_has_direct_animate_child(&FragmentKey::EachBody(each.id)));
    assert!(data.each_has_animate(each.id));
}

#[test]
fn fragment_facts_single_child_supports_block_empty() {
    let (component, data, _diags) = analyze_source_with_diags("{#if ok} {/if}");
    let block = find_if_block(&component.fragment, &component, "ok").expect("expected if block");

    let child_id = data
        .fragment_single_child(&FragmentKey::IfConsequent(block.id))
        .expect("expected single child");
    assert!(data.fragment_has_children(&FragmentKey::IfConsequent(block.id)));
    assert!(matches!(component.store.get(child_id), Node::Text(_)));
}

#[test]
fn fragment_facts_track_non_trivial_child_counts() {
    let (component, data, _diags) = analyze_source_with_diags(
        r#"<Widget>
    <!-- comment -->
    {#snippet children()}
        <p>inside</p>
    {/snippet}
    
    <span>after</span>
</Widget>"#,
    );
    let widget_id = find_component_node_id(&component.fragment, &component, "Widget")
        .expect("expected component node");
    let children_snippet =
        find_snippet_block(&component.fragment, &component, "children").expect("expected snippet");
    let span = find_element(&component.fragment, &component, "span").expect("expected span");
    let key = FragmentKey::ComponentNode(widget_id);
    assert_eq!(data.fragment_child_count(&key), 7);
    assert!(data.fragment_has_non_trivial_children(&key));
    assert_eq!(data.fragment_non_trivial_child_count(&key), 2);
    assert_eq!(data.fragment_single_non_trivial_child(&key), None);
    assert!(matches!(
        component.store.get(children_snippet.id),
        Node::SnippetBlock(_)
    ));
    assert!(matches!(component.store.get(span.id), Node::Element(_)));
}

#[test]
fn rich_content_facts_drive_customizable_select_detection() {
    let (component, data) = analyze_source(
        r#"<script>let ok = true;</script>
<select>{#if ok}<div></div>{/if}</select>
<select>{#if ok}<option>ok</option>{/if}</select>
<optgroup>{#if ok} text {/if}</optgroup>
<option>{#if ok}<span></span>{/if}</option>"#,
    );

    let rich_select =
        find_nth_element(&component.fragment, &component, "select", 0).expect("expected select");
    let plain_select =
        find_nth_element(&component.fragment, &component, "select", 1).expect("expected select");
    let optgroup =
        find_element(&component.fragment, &component, "optgroup").expect("expected optgroup");
    let option =
        find_nth_element(&component.fragment, &component, "option", 1).expect("expected option");

    assert!(data.fragment_has_rich_content(
        &FragmentKey::Element(rich_select.id),
        RichContentParentKind::Select
    ));
    assert!(!data.fragment_has_rich_content(
        &FragmentKey::Element(plain_select.id),
        RichContentParentKind::Select
    ));
    assert!(data.fragment_has_rich_content(
        &FragmentKey::Element(optgroup.id),
        RichContentParentKind::Optgroup
    ));
    assert!(data.fragment_has_rich_content(
        &FragmentKey::Element(option.id),
        RichContentParentKind::Option
    ));

    assert!(data.elements.flags.is_customizable_select(rich_select.id));
    assert!(!data.elements.flags.is_customizable_select(plain_select.id));
    assert!(data.elements.flags.is_customizable_select(optgroup.id));
    assert!(data.elements.flags.is_customizable_select(option.id));
}

#[test]
fn inspect_does_not_trigger_state_referenced_locally_for_rune_args() {
    let cases = [
        (
            "<script>let a = $state(1); $inspect(a);</script>",
            Vec::<&str>::new(),
        ),
        (
            "<script>let a = $state(1); $inspect(a).with(console.log);</script>",
            Vec::<&str>::new(),
        ),
        (
            "<script>let a = $state(1); $inspect(a).with();</script>",
            vec!["rune_invalid_arguments_length"],
        ),
        (
            "<script>let a = $state(1); $inspect(a).with(console.log, console.warn);</script>",
            vec!["rune_invalid_arguments_length"],
        ),
        (
            "<script>let count = $state(3); let doubled = $derived(count * 2); $inspect(doubled);</script>",
            Vec::<&str>::new(),
        ),
    ];

    for (source, expected_codes) in cases {
        let (_, _, diags) = analyze_source_with_diags(source);
        let actual_codes = diags
            .iter()
            .map(|diag| diag.kind.code())
            .collect::<Vec<_>>();
        assert_eq!(
            actual_codes, expected_codes,
            "unexpected diagnostics for source: {source}"
        );
    }
}

#[test]
fn shorthand_anchor_href_is_indexed_for_a11y_presence_checks() {
    let source = r#"
<script lang="ts">
    import type { Snippet } from 'svelte';

    type Props = {
        href?: string;
        target?: string;
        rel?: string;
        underline?: boolean;
        children?: Snippet;
    };

    let { href = '#', target = '_self', rel = '', underline = true, children }: Props = $props();
</script>

<a {href} {target} {rel} class="ui-link" style:text-decoration={underline ? 'underline' : 'none'}>
    {@render children?.()}
</a>
"#;

    let (component, data, diags) = analyze_source_with_diags(source);
    let anchor = find_element(&component.fragment, &component, "a").expect("expected anchor");

    assert!(
        data.has_attribute(anchor.id, "href"),
        "expected shorthand href to be indexed as a named attribute"
    );
    // `{href}` parses as ExpressionAttribute with shorthand: true.
    assert!(matches!(
        data.attribute(anchor.id, &anchor.attributes, "href"),
        Some(Attribute::ExpressionAttribute(a)) if a.shorthand
    ));
    assert!(
        diags
            .iter()
            .all(|diag| diag.kind.code() != "a11y_missing_attribute"),
        "unexpected diagnostics: {diags:?}"
    );
}

#[test]
fn button_role_presentation_with_text_warns_for_interactive_to_noninteractive_role() {
    let source = r#"<button role="presentation">Close</button>"#;

    let (_, _, diags) = analyze_source_with_diags(source);
    let actual_codes = diags
        .iter()
        .map(|diag| diag.kind.code())
        .collect::<Vec<_>>();

    assert_eq!(
        actual_codes,
        vec!["a11y_no_interactive_element_to_noninteractive_role"],
        "unexpected diagnostics: {diags:?}"
    );
}

#[test]
fn footer_role_button_warns_for_noninteractive_to_interactive_role() {
    let source = r#"<footer role="button">Footer</footer>"#;

    let (_, _, diags) = analyze_source_with_diags(source);
    let actual_codes = diags
        .iter()
        .map(|diag| diag.kind.code())
        .collect::<Vec<_>>();

    assert_eq!(
        actual_codes,
        vec!["a11y_no_noninteractive_element_to_interactive_role"],
        "unexpected diagnostics: {diags:?}"
    );
}

#[test]
fn li_role_menuitem_uses_reference_exception_without_warning() {
    let source = r#"<li role="menuitem">Item</li>"#;

    let (_, _, diags) = analyze_source_with_diags(source);
    let actual_codes = diags
        .iter()
        .map(|diag| diag.kind.code())
        .collect::<Vec<_>>();

    assert!(actual_codes.is_empty(), "unexpected diagnostics: {diags:?}");
}

#[test]
fn static_element_interactions_warning_respects_svelte_ignore() {
    let source = r#"
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
    onmouseenter={() => {}}
    onmouseleave={() => {}}
>
    Hover me
</div>
"#;

    let (_, _, diags) = analyze_source_with_diags(source);
    let actual_codes = diags
        .iter()
        .map(|diag| diag.kind.code())
        .collect::<Vec<_>>();

    assert!(
        actual_codes
            .iter()
            .all(|code| *code != "a11y_no_static_element_interactions"),
        "unexpected diagnostics: {diags:?}"
    );
}

#[test]
fn icon_button_warns_for_missing_explicit_label() {
    let source = r#"<button><svg aria-hidden="true"></svg></button>"#;

    let (_, _, diags) = analyze_source_with_diags(source);
    let actual_codes = diags
        .iter()
        .map(|diag| diag.kind.code())
        .collect::<Vec<_>>();

    assert_eq!(
        actual_codes,
        vec!["a11y_consider_explicit_label"],
        "unexpected diagnostics: {diags:?}"
    );
}

#[test]
fn button_with_text_does_not_warn_for_missing_explicit_label() {
    let source = r#"<button>Submit</button>"#;

    let (_, _, diags) = analyze_source_with_diags(source);
    let actual_codes = diags
        .iter()
        .map(|diag| diag.kind.code())
        .collect::<Vec<_>>();

    assert!(
        actual_codes
            .iter()
            .all(|code| *code != "a11y_consider_explicit_label"),
        "unexpected diagnostics: {diags:?}"
    );
}

#[test]
fn anchor_hash_href_warns_for_invalid_attribute() {
    let source = r##"<a href="#">jump</a>"##;

    let (_, _, diags) = analyze_source_with_diags(source);
    let actual_codes = diags
        .iter()
        .map(|diag| diag.kind.code())
        .collect::<Vec<_>>();

    assert_eq!(
        actual_codes,
        vec!["a11y_invalid_attribute"],
        "unexpected diagnostics: {diags:?}"
    );
}

#[test]
fn anchor_with_valid_href_and_text_has_no_a11y_content_warning() {
    let source = r#"<a href="/home">Home</a>"#;

    let (_, _, diags) = analyze_source_with_diags(source);
    let actual_codes = diags
        .iter()
        .map(|diag| diag.kind.code())
        .collect::<Vec<_>>();

    assert!(
        actual_codes.iter().all(|code| !matches!(
            *code,
            "a11y_invalid_attribute" | "a11y_consider_explicit_label"
        )),
        "unexpected diagnostics: {diags:?}"
    );
}

#[test]
fn label_without_for_or_control_warns_for_missing_associated_control() {
    let source = r#"<label>Username</label>"#;

    let (_, _, diags) = analyze_source_with_diags(source);
    let actual_codes = diags
        .iter()
        .map(|diag| diag.kind.code())
        .collect::<Vec<_>>();

    assert_eq!(
        actual_codes,
        vec!["a11y_label_has_associated_control"],
        "unexpected diagnostics: {diags:?}"
    );
}

#[test]
fn label_with_nested_input_does_not_warn_for_missing_associated_control() {
    let source = r#"<label><input /></label>"#;

    let (_, _, diags) = analyze_source_with_diags(source);
    let actual_codes = diags
        .iter()
        .map(|diag| diag.kind.code())
        .collect::<Vec<_>>();

    assert!(
        actual_codes
            .iter()
            .all(|code| *code != "a11y_label_has_associated_control"),
        "unexpected diagnostics: {diags:?}"
    );
}

#[test]
fn label_with_for_attribute_does_not_warn_for_missing_associated_control() {
    let source = r#"<label for="name">Username</label>"#;

    let (_, _, diags) = analyze_source_with_diags(source);
    let actual_codes = diags
        .iter()
        .map(|diag| diag.kind.code())
        .collect::<Vec<_>>();

    assert!(
        actual_codes
            .iter()
            .all(|code| *code != "a11y_label_has_associated_control"),
        "unexpected diagnostics: {diags:?}"
    );
}

#[test]
fn empty_h1_warns_for_missing_content() {
    let source = r#"<h1></h1>"#;

    let (_, _, diags) = analyze_source_with_diags(source);
    let actual_codes = diags
        .iter()
        .map(|diag| diag.kind.code())
        .collect::<Vec<_>>();

    assert_eq!(
        actual_codes,
        vec!["a11y_missing_content"],
        "unexpected diagnostics: {diags:?}"
    );
}

#[test]
fn h1_with_text_does_not_warn_for_missing_content() {
    let source = r#"<h1>Title</h1>"#;

    let (_, _, diags) = analyze_source_with_diags(source);
    let actual_codes = diags
        .iter()
        .map(|diag| diag.kind.code())
        .collect::<Vec<_>>();

    assert!(
        actual_codes
            .iter()
            .all(|code| *code != "a11y_missing_content"),
        "unexpected diagnostics: {diags:?}"
    );
}

#[test]
fn video_with_caption_track_does_not_warn_for_missing_captions() {
    let source = r#"<video src="movie.mp4"><track kind="captions" /></video>"#;

    let (_, _, diags) = analyze_source_with_diags(source);
    let actual_codes = diags
        .iter()
        .map(|diag| diag.kind.code())
        .collect::<Vec<_>>();

    assert!(
        actual_codes
            .iter()
            .all(|code| *code != "a11y_media_has_caption"),
        "unexpected diagnostics: {diags:?}"
    );
}

#[test]
fn figure_with_edge_figcaption_does_not_warn_for_figcaption_index() {
    let source = r#"<figure><figcaption>Caption</figcaption><img alt="" /></figure>"#;

    let (_, _, diags) = analyze_source_with_diags(source);
    let actual_codes = diags
        .iter()
        .map(|diag| diag.kind.code())
        .collect::<Vec<_>>();

    assert!(
        actual_codes
            .iter()
            .all(|code| !matches!(*code, "a11y_figcaption_parent" | "a11y_figcaption_index")),
        "unexpected diagnostics: {diags:?}"
    );
}

#[test]
fn invalid_input_autocomplete_warns() {
    let source = r#"<input type="text" autocomplete="totally-wrong" />"#;

    let (_, _, diags) = analyze_source_with_diags(source);
    let actual_codes = diags
        .iter()
        .map(|diag| diag.kind.code())
        .collect::<Vec<_>>();

    assert_eq!(
        actual_codes,
        vec!["a11y_autocomplete_valid"],
        "unexpected diagnostics: {diags:?}"
    );
}

#[test]
fn valid_input_autocomplete_does_not_warn() {
    let source = r#"<input type="text" autocomplete="name" />"#;

    let (_, _, diags) = analyze_source_with_diags(source);
    let actual_codes = diags
        .iter()
        .map(|diag| diag.kind.code())
        .collect::<Vec<_>>();

    assert!(
        actual_codes
            .iter()
            .all(|code| *code != "a11y_autocomplete_valid"),
        "unexpected diagnostics: {diags:?}"
    );
}

#[test]
fn redundant_img_alt_warns() {
    let source = r#"<img alt="image of a cat" />"#;

    let (_, _, diags) = analyze_source_with_diags(source);
    let actual_codes = diags
        .iter()
        .map(|diag| diag.kind.code())
        .collect::<Vec<_>>();

    assert_eq!(
        actual_codes,
        vec!["a11y_img_redundant_alt"],
        "unexpected diagnostics: {diags:?}"
    );
}

#[test]
fn th_scope_does_not_warn_for_misplaced_scope() {
    let source = r#"<th scope="col">value</th>"#;

    let (_, _, diags) = analyze_source_with_diags(source);
    let actual_codes = diags
        .iter()
        .map(|diag| diag.kind.code())
        .collect::<Vec<_>>();

    assert!(
        actual_codes
            .iter()
            .all(|code| *code != "a11y_misplaced_scope"),
        "unexpected diagnostics: {diags:?}"
    );
}

#[test]
fn snippet_hoistability_taints_computed_key_script_reference() {
    let (component, data) = analyze_source(
        r#"<script>
function key() {
    return "label";
}
</script>

{#snippet view({ [key()]: value })}
    <p>{value}</p>
{/snippet}"#,
    );

    assert_snippet_hoistable(&data, &component, "view", false);
}

#[test]
fn snippet_param_ref_symbols_capture_script_refs() {
    let (component, data) = analyze_source(
        r#"<script>
function key() {
    return "label";
}
let fallback = () => "ok";
</script>

{#snippet view({ [key()]: value = fallback() })}
    <p>{value}</p>
{/snippet}"#,
    );

    assert_snippet_param_refs_include(&data, &component, "view", "key");
    assert_snippet_param_refs_include(&data, &component, "view", "fallback");
}

#[test]
fn bind_target_symbol_covers_shorthand_and_identifier_bindings() {
    let (component, data) = analyze_source(
        r#"<script>
let value = $state('');
let checked = $state(false);
</script>

<input bind:value />
<input type="checkbox" bind:checked={checked} />"#,
    );

    assert_bind_target_symbol_name(&data, &component, "input", "value", "value");
    assert_bind_target_symbol_name(&data, &component, "input", "checked", "checked");
}

#[test]
fn bind_validator_keeps_plain_let_targets_valid() {
    let (_component, _data, diags) = analyze_source_with_diags(
        r#"<script>let value = '';</script>
<input bind:value={value}>"#,
    );

    assert_diag_codes(&diags, &["non_reactive_update"]);
}

#[test]
fn bind_group_validator_uses_unified_snippet_param_source_kind() {
    let (_component, _data, diags) = analyze_source_with_diags(
        r#"{#snippet group(selected)}
    <input type="checkbox" bind:group={selected} />
{/snippet}"#,
    );

    assert_diag_codes(
        &diags,
        &[
            "bind_group_invalid_snippet_parameter",
            "snippet_parameter_assignment",
        ],
    );
}

#[test]
fn bind_target_semantics_classifies_common_bind_families() {
    let (component, data) = analyze_source(
        r#"<script>
let value = $state('');
let checked = $state(false);
let node;
let width = $state(0);
let active = $state(null);
</script>

<input bind:value />
<input type="checkbox" bind:checked={checked} />
<div bind:this={node}></div>
<svelte:window bind:innerWidth={width} />
<svelte:document bind:activeElement={active} />"#,
    );

    assert_bind_target_semantics(
        &data,
        &component,
        "input",
        "value",
        BindHostKind::Element,
        BindPropertyKind::Value,
        true,
    );
    assert_bind_target_semantics(
        &data,
        &component,
        "input",
        "checked",
        BindHostKind::Element,
        BindPropertyKind::Checked,
        true,
    );
    assert_bind_target_semantics(
        &data,
        &component,
        "div",
        "this",
        BindHostKind::Element,
        BindPropertyKind::This,
        false,
    );
    assert_bind_target_semantics(
        &data,
        &component,
        "svelte:window",
        "innerWidth",
        BindHostKind::Window,
        BindPropertyKind::Window(WindowBindKind::InnerWidth),
        true,
    );
    assert_bind_target_semantics(
        &data,
        &component,
        "svelte:document",
        "activeElement",
        BindHostKind::Document,
        BindPropertyKind::Document(DocumentBindKind::ActiveElement),
        true,
    );
}

#[test]
fn bind_target_semantics_classifies_extended_bind_families() {
    let (component, data) = analyze_source(
        r#"<script>
let html = $state('');
let width = $state(0);
let rect = $state(null);
let played = $state(null);
let natural = $state(0);
let focused = $state(false);
let online = $state(false);
</script>

<div contenteditable bind:innerHTML={html}></div>
<div bind:clientWidth={width}></div>
<div bind:contentRect={rect}></div>
<audio bind:played={played}></audio>
<img alt="" bind:naturalWidth={natural} />
<input bind:focused={focused} />
<svelte:window bind:online={online} />"#,
    );

    assert_bind_target_semantics(
        &data,
        &component,
        "div",
        "innerHTML",
        BindHostKind::Element,
        BindPropertyKind::ContentEditable(ContentEditableKind::InnerHtml),
        true,
    );
    assert_bind_target_semantics(
        &data,
        &component,
        "div",
        "clientWidth",
        BindHostKind::Element,
        BindPropertyKind::ElementSize(ElementSizeKind::ClientWidth),
        true,
    );
    assert_bind_target_semantics(
        &data,
        &component,
        "div",
        "contentRect",
        BindHostKind::Element,
        BindPropertyKind::ResizeObserver(ResizeObserverKind::ContentRect),
        true,
    );
    assert_bind_target_semantics(
        &data,
        &component,
        "audio",
        "played",
        BindHostKind::Element,
        BindPropertyKind::Media(MediaBindKind::Played),
        true,
    );
    assert_bind_target_semantics(
        &data,
        &component,
        "img",
        "naturalWidth",
        BindHostKind::Element,
        BindPropertyKind::ImageNaturalSize(ImageNaturalSizeKind::NaturalWidth),
        true,
    );
    assert_bind_target_semantics(
        &data,
        &component,
        "input",
        "focused",
        BindHostKind::Element,
        BindPropertyKind::Focused,
        true,
    );
    assert_bind_target_semantics(
        &data,
        &component,
        "svelte:window",
        "online",
        BindHostKind::Window,
        BindPropertyKind::Window(WindowBindKind::Online),
        true,
    );
}

#[test]
fn bind_target_semantics_classifies_component_bind_families() {
    let (component, data) = analyze_source(
        r#"<script>
import Widget from './Widget.svelte';
let value = $state('');
let instance;
</script>

<Widget bind:value={value} bind:this={instance} />"#,
    );

    assert_bind_target_semantics(
        &data,
        &component,
        "Widget",
        "value",
        BindHostKind::Component,
        BindPropertyKind::ComponentProp,
        true,
    );
    assert_bind_target_semantics(
        &data,
        &component,
        "Widget",
        "this",
        BindHostKind::Component,
        BindPropertyKind::This,
        false,
    );
}

#[test]
fn shorthand_symbol_returns_symbol_for_class_and_style_shorthand() {
    let (component, data) = analyze_source(
        r#"<script>
let active = $state(false);
let color = $state('red');
</script>

<div class:active style:color></div>"#,
    );

    assert_shorthand_symbol_name(&data, &component, "div", "active", "active");
    assert_shorthand_symbol_name(&data, &component, "div", "color", "color");
}

#[test]
fn snippet_hoistability_taints_default_initializer_script_reference() {
    let (component, data) = analyze_source(
        r#"<script>
let fallback = () => "ok";
</script>

{#snippet view({ value = fallback() })}
    <p>{value}</p>
{/snippet}"#,
    );

    assert_snippet_hoistable(&data, &component, "view", false);
}

#[test]
fn snippet_hoistability_ignores_nested_destructure_without_script_refs() {
    let (component, data) = analyze_source(
        r#"{#snippet view({ outer: { value = "ok" } })}
    <p>{value}</p>
{/snippet}"#,
    );

    assert_snippet_hoistable(&data, &component, "view", true);
}

#[test]
fn snippet_hoistability_uses_symbols_not_names() {
    let (component, data) = analyze_source(
        r#"<script>
let fallback = "script";
</script>

{#snippet view({ value = (() => {
    let fallback = () => "ok";
    return fallback();
})() })}
    <p>{value}</p>
{/snippet}"#,
    );

    assert_snippet_hoistable(&data, &component, "view", true);
}

#[test]
fn options_preserve_whitespace_keeps_raw_text_nodes() {
    let (component, data) = analyze_source_with_options(
        "<div>\n\thello\n\t<span>world</span>\n\t!\n</div>",
        AnalyzeOptions {
            preserve_whitespace: true,
            ..AnalyzeOptions::default()
        },
    );

    let div = find_element(&component.fragment, &component, "div")
        .unwrap_or_else(|| panic!("expected div element"));
    let lowered = data
        .template
        .fragments
        .lowered
        .get(&FragmentKey::Element(div.id))
        .unwrap_or_else(|| panic!("expected lowered fragment"));
    let text = lowered
        .items
        .iter()
        .find_map(|item| match item {
            FragmentItem::TextConcat { parts, .. } => Some(parts),
            _ => None,
        })
        .unwrap_or_else(|| panic!("expected preserved text concat"));
    let rendered = text
        .iter()
        .filter_map(|part| part.text_value(&component.source))
        .collect::<String>();

    assert!(rendered.contains("\n\thello\n\t"));
    assert!(data.script.preserve_whitespace);
}

#[test]
fn special_element_invalid_content_uses_child_bounds() {
    let (_component, _data, diags) =
        analyze_source_with_diags("<svelte:window>child</svelte:window>");

    assert_eq!(diags.len(), 1, "unexpected diagnostics: {diags:?}");
    assert_eq!(diags[0].kind.code(), "svelte_meta_invalid_content");
    assert_eq!(diags[0].span, Span::new(15, 20));
}

#[test]
fn special_elements_allow_document_attach_and_body_use() {
    let source = r#"<script>
function track(node) { return () => {}; }
function act(node) {}
</script>
<svelte:document {@attach track} />
<svelte:body use:act />"#;
    let (_component, _data, diags) = analyze_source_with_diags(source);

    assert!(diags.is_empty(), "unexpected diagnostics: {diags:?}");
}

#[test]
fn special_element_illegal_attribute_uses_full_attr_span() {
    let (_component, _data, diags) = analyze_source_with_diags(r#"<svelte:document class="x" />"#);

    assert_eq!(diags.len(), 1, "unexpected diagnostics: {diags:?}");
    assert_eq!(diags[0].kind.code(), "illegal_element_attribute");
    assert_eq!(diags[0].span, Span::new(17, 26));
}

#[test]
fn rune_self_declaration_does_not_warn_store_rune_conflict() {
    let source = r#"<script>
let state = $state("");
</script>"#;

    let (_, _, diags) = analyze_source_with_diags(source);

    assert!(
        diags
            .iter()
            .all(|diag| diag.kind.code() != "store_rune_conflict"),
        "unexpected diagnostics: {diags:?}"
    );
}

#[test]
fn local_store_binding_still_warns_store_rune_conflict() {
    let source = r#"<script>
import { writable } from 'svelte/store';
let state = writable(0);
let x = $state(0);
</script>"#;

    let (_, _, diags) = analyze_source_with_diags(source);
    let actual_codes = diags
        .iter()
        .map(|diag| diag.kind.code())
        .collect::<Vec<_>>();

    assert_eq!(
        actual_codes,
        vec!["store_rune_conflict"],
        "unexpected diagnostics: {diags:?}"
    );
}

// -----------------------------------------------------------------------
// Event directive diagnostics
// -----------------------------------------------------------------------

// ---------------------------------------------------------------------------
// attribute_quoted
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// attribute_global_event_reference
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// component_name_lowercase
// ---------------------------------------------------------------------------

#[test]
fn analyze_module_reports_store_invalid_subscription_module() {
    let alloc = oxc_allocator::Allocator::default();
    let source = r#"
        import { writable } from 'svelte/store';
        const count = writable(0);
        console.log($count);
    "#;

    let (_data, _parsed, diags) = analyze_module(&alloc, source, false, false);
    let store_diags = diags
        .iter()
        .filter(|diag| diag.kind.code() == "store_invalid_subscription_module")
        .count();

    assert_eq!(store_diags, 1, "unexpected diagnostics: {diags:?}");
}

#[test]
fn analyze_module_ignores_nested_only_store_like_bindings() {
    let alloc = oxc_allocator::Allocator::default();
    let source = r#"
        function demo() {
            const count = 0;
            console.log($count);
        }
    "#;

    let (_data, _parsed, diags) = analyze_module(&alloc, source, false, false);

    assert!(
        !diags
            .iter()
            .any(|diag| diag.kind.code() == "store_invalid_subscription_module"),
        "unexpected diagnostics: {diags:?}"
    );
}

// -----------------------------------------------------------------------
// Target-site ReferenceSemantics queries cover what `target_site_semantics`
// used to answer — see `reference_semantics` via `directive_root_ref_id`
// (codegen) or via `attr_root_symbol` + `symbol_facts`. Tests for the
// individual variants live next to their consumers.
// -----------------------------------------------------------------------

// -----------------------------------------------------------------------
// PropSourceMemberMutationRoot — operation-level answer for `foo.x = val`
// where `foo` is a `$props()` source binding.
// -----------------------------------------------------------------------

#[test]
fn prop_source_member_mutation_root_in_script_assignment() {
    let (_component, data, parsed) = analyze_source_with_parsed(
        r#"<script>
    let { obj = $bindable({ x: 0 }) } = $props();
    function inc() { obj.x = 1; }
</script>"#,
    );

    let semantics = script_reference_semantics(&data, &parsed, "obj", true, false, 0);
    match semantics {
        ReferenceSemantics::PropSourceMemberMutationRoot { bindable, .. } => {
            assert!(bindable, "bindable flag");
        }
        other => panic!("expected PropSourceMemberMutationRoot, got {other:?}"),
    }
}

#[test]
fn prop_source_member_mutation_root_in_update_expression() {
    let (_component, data, parsed) = analyze_source_with_parsed(
        r#"<script>
    let { obj = $bindable({ n: 0 }) } = $props();
    function bump() { obj.n++; }
</script>"#,
    );

    let semantics = script_reference_semantics(&data, &parsed, "obj", true, false, 0);
    match semantics {
        ReferenceSemantics::PropSourceMemberMutationRoot { .. } => {}
        other => panic!("expected PropSourceMemberMutationRoot on ++, got {other:?}"),
    }
}

/// Look up the `ReferenceSemantics` for the first resolved read-reference of a
/// named binding. Used by contextual-read tests where the binding only lives
/// in template scope (no instance script).
fn first_read_reference_semantics(data: &AnalysisData, name: &str) -> ReferenceSemantics {
    let sym = data
        .scoping
        .find_binding_in_any_scope(name)
        .unwrap_or_else(|| panic!("binding {name} not found in any scope"));
    let ref_id = data
        .scoping
        .get_resolved_reference_ids(sym)
        .iter()
        .copied()
        .find(|&ref_id| data.scoping.get_reference(ref_id).is_read())
        .unwrap_or_else(|| panic!("no read reference for {name}"));
    data.reference_semantics(ref_id)
}

#[test]
fn each_item_read_is_classified_as_contextual_signal_read() {
    let (_c, data) = analyze_source(
        r#"<script>let items = $state([1, 2, 3]);</script>
{#each items as item}<p>{item}</p>{/each}"#,
    );

    match first_read_reference_semantics(&data, "item") {
        ReferenceSemantics::ContextualRead(ContextualReadSemantics {
            kind:
                ContextualReadKind::EachItem {
                    accessor: false,
                    signal: true,
                },
            ..
        }) => {}
        other => panic!("expected EachItem signal read, got {other:?}"),
    }
}

#[test]
fn snippet_param_read_is_classified_as_contextual_accessor_call() {
    // Non-default snippet params are marked as getters → emitted as `item()`.
    let (_c, data) = analyze_source(r#"{#snippet row(item)}<p>{item}</p>{/snippet}"#);

    match first_read_reference_semantics(&data, "item") {
        ReferenceSemantics::ContextualRead(ContextualReadSemantics {
            kind:
                ContextualReadKind::SnippetParam {
                    accessor: true,
                    signal: false,
                },
            ..
        }) => {}
        other => panic!("expected SnippetParam accessor read, got {other:?}"),
    }
}

#[test]
fn snippet_default_leaf_is_classified_as_contextual_signal_read() {
    // Destructure leaves under `= default` are NOT getters → `$.get(label)`.
    let (_c, data) =
        analyze_source(r#"{#snippet withDefault({ label = "x" })}<p>{label}</p>{/snippet}"#);

    match first_read_reference_semantics(&data, "label") {
        ReferenceSemantics::ContextualRead(ContextualReadSemantics {
            kind:
                ContextualReadKind::SnippetParam {
                    accessor: false,
                    signal: true,
                },
            ..
        }) => {}
        other => panic!("expected SnippetParam signal read, got {other:?}"),
    }
}

#[test]
fn slot_let_destructure_leaf_is_classified_as_carrier_member_read() {
    let (_c, data) = analyze_source(r#"<Widget let:item={{ text }}>{text}</Widget>"#);

    let carrier_expected = data
        .scoping
        .find_binding_in_any_scope("item")
        .expect("carrier symbol");
    match first_read_reference_semantics(&data, "text") {
        ReferenceSemantics::CarrierMemberRead(CarrierMemberReadSemantics {
            carrier_symbol,
            ..
        }) if carrier_symbol == carrier_expected => {}
        other => panic!("expected CarrierMemberRead with carrier=item, got {other:?}"),
    }
}

#[test]
fn derived_reactivity_flag_false_when_deps_are_inert() {
    // `count` is declared as `$state(0)` but never mutated — so it lowers as
    // an `OptimizedRune` and is not reactive at runtime. A `$derived` whose
    // only dep is `count` therefore also never changes, and its reads in the
    // template must not trigger a `$.template_effect` wrap.
    let (_c, data) = analyze_source(
        r#"<script>
  let count = $state(0);
  let doubled = $derived(count * 2);
</script>
<p>{doubled}</p>"#,
    );

    let doubled_sym = data
        .scoping
        .find_binding_in_any_scope("doubled")
        .expect("doubled binding");
    let decl = data.declaration_semantics(data.scoping.symbol_declaration(doubled_sym));
    match decl {
        DeclarationSemantics::Derived(d) => assert!(
            !d.reactive,
            "expected doubled.reactive=false when deps are inert"
        ),
        other => panic!("expected Derived decl for doubled, got {other:?}"),
    }
}

#[test]
fn derived_reactivity_flag_true_when_dep_is_mutated_state() {
    let (_c, data) = analyze_source(
        r#"<script>
  let count = $state(0);
  let doubled = $derived(count * 2);
  count = 5;
</script>
<p>{doubled}</p>"#,
    );

    let doubled_sym = data
        .scoping
        .find_binding_in_any_scope("doubled")
        .expect("doubled binding");
    let decl = data.declaration_semantics(data.scoping.symbol_declaration(doubled_sym));
    match decl {
        DeclarationSemantics::Derived(d) => assert!(
            d.reactive,
            "expected doubled.reactive=true when dep is mutated $state"
        ),
        other => panic!("expected Derived decl for doubled, got {other:?}"),
    }
}

// -------------------------------------------------------------------------
// block_semantics — {#each} (subsession B)
// -------------------------------------------------------------------------

mod block_semantics_each_tests {
    use super::analyze_source;
    use crate::block_semantics::{
        BlockSemantics, EachFlavor, EachIndexKind, EachItemKind, EachKeyKind,
    };
    use svelte_ast::Node;

    fn first_each_semantics<'a>(
        data: &'a crate::AnalysisData<'_>,
        component: &'a svelte_ast::Component,
    ) -> &'a crate::block_semantics::EachBlockSemantics {
        let block_id = component
            .fragment
            .nodes
            .iter()
            .find_map(|&id| match component.store.get(id) {
                Node::EachBlock(b) => Some(b.id),
                _ => None,
            })
            .expect("component must have a top-level {#each}");
        match data.block_semantics(block_id) {
            BlockSemantics::Each(sem) => sem,
            other => panic!("expected BlockSemantics::Each, got {other:?}"),
        }
    }

    #[test]
    fn each_plain() {
        let (component, data) =
            analyze_source(r#"{#each items as item}<span>{item}</span>{/each}"#);
        let sem = first_each_semantics(&data, &component);
        assert!(matches!(sem.item, EachItemKind::Identifier(_)));
        assert_eq!(sem.index, EachIndexKind::Absent);
        assert_eq!(sem.key, EachKeyKind::Unkeyed);
        assert_eq!(sem.flavor, EachFlavor::Regular);
    }

    #[test]
    fn each_indexed_body_uses_index() {
        let (component, data) =
            analyze_source(r#"{#each items as item, i}<span>{i}:{item}</span>{/each}"#);
        let sem = first_each_semantics(&data, &component);
        assert!(matches!(sem.item, EachItemKind::Identifier(_)));
        match sem.index {
            EachIndexKind::Declared {
                used_in_body,
                used_in_key,
                ..
            } => {
                assert!(used_in_body, "body uses `i`");
                assert!(!used_in_key, "no key expression");
            }
            _ => panic!("expected Declared"),
        }
    }

    #[test]
    fn each_indexed_body_unused() {
        let (component, data) =
            analyze_source(r#"{#each items as item, i}<span>{item}</span>{/each}"#);
        let sem = first_each_semantics(&data, &component);
        match sem.index {
            EachIndexKind::Declared {
                used_in_body,
                used_in_key,
                ..
            } => {
                assert!(!used_in_body, "body does not reference `i`");
                assert!(!used_in_key);
            }
            _ => panic!("expected Declared"),
        }
    }

    #[test]
    fn each_keyed_uses_index_in_key() {
        let (component, data) =
            analyze_source(r#"{#each items as item, i (i)}<span>{item}</span>{/each}"#);
        let sem = first_each_semantics(&data, &component);
        match sem.index {
            EachIndexKind::Declared {
                used_in_body,
                used_in_key,
                ..
            } => {
                assert!(used_in_key, "key expression uses `i`");
                assert!(!used_in_body, "body does not reference `i`");
            }
            _ => panic!("expected Declared"),
        }
    }

    #[test]
    fn each_keyed_by_item() {
        let (component, data) =
            analyze_source(r#"{#each items as item (item)}<span>{item}</span>{/each}"#);
        let sem = first_each_semantics(&data, &component);
        assert_eq!(sem.key, EachKeyKind::KeyedByItem);
    }

    #[test]
    fn each_keyed_by_expr() {
        let (component, data) =
            analyze_source(r#"{#each items as item (item.id)}<span>{item.id}</span>{/each}"#);
        let sem = first_each_semantics(&data, &component);
        assert!(matches!(sem.key, EachKeyKind::KeyedByExpr(_)));
    }

    #[test]
    fn each_destructured() {
        let (component, data) =
            analyze_source(r#"{#each pairs as { a, b }}<span>{a}:{b}</span>{/each}"#);
        let sem = first_each_semantics(&data, &component);
        assert!(matches!(sem.item, EachItemKind::Pattern(_)));
    }

    #[test]
    fn each_no_binding() {
        let (component, data) = analyze_source(r#"{#each items}<span>x</span>{/each}"#);
        let block_id = component
            .fragment
            .nodes
            .iter()
            .find_map(|&id| match component.store.get(id) {
                Node::EachBlock(b) => Some(b.id),
                _ => None,
            })
            .expect("test invariant");
        let BlockSemantics::Each(sem) = data.block_semantics(block_id) else {
            panic!("expected Each variant")
        };
        assert_eq!(sem.item, EachItemKind::NoBinding);
        assert_eq!(sem.index, EachIndexKind::Absent);
        assert_eq!(sem.key, EachKeyKind::Unkeyed);
    }

    #[test]
    fn each_with_bind_group_referencing_item_member_is_bind_group_flavor() {
        // The group expression references `item.picks` — a member of an
        // each-owned symbol — so this each frame must be flagged BindGroup.
        let (component, data) = analyze_source(
            r#"{#each items as item}
  <input type="checkbox" bind:group={item.picks} value={item.name}>
{/each}"#,
        );
        let sem = first_each_semantics(&data, &component);
        assert_eq!(sem.flavor, EachFlavor::BindGroup);
    }

    #[test]
    fn each_with_bind_group_on_outer_symbol_is_regular() {
        // `selected` is declared outside of every enclosing each, so
        // the bind:group does not belong to THIS each frame (Svelte's
        // scope-qualified rule).
        let (component, data) = analyze_source(
            r#"<script>let selected = $state([]);</script>
{#each items as item}
  <input type="checkbox" bind:group={selected} value={item}>
{/each}"#,
        );
        let sem = first_each_semantics(&data, &component);
        assert_eq!(sem.flavor, EachFlavor::Regular);
    }

    #[test]
    fn each_without_bind_group_is_regular_flavor() {
        let (component, data) = analyze_source(
            r#"{#each items as item}<input type="checkbox" bind:checked={item.on}>{/each}"#,
        );
        let sem = first_each_semantics(&data, &component);
        assert_eq!(sem.flavor, EachFlavor::Regular);
    }

    #[test]
    fn each_has_animate_when_direct_child_has_animate_directive() {
        use crate::block_semantics::EachFlags;
        let (component, data) = analyze_source(
            r#"{#each items as item (item.id)}<span animate:flip>{item}</span>{/each}"#,
        );
        let sem = first_each_semantics(&data, &component);
        assert!(sem.each_flags.contains(EachFlags::ANIMATED));
    }

    #[test]
    fn each_no_animate_without_directive() {
        use crate::block_semantics::EachFlags;
        let (component, data) =
            analyze_source(r#"{#each items as item}<span>{item}</span>{/each}"#);
        let sem = first_each_semantics(&data, &component);
        assert!(!sem.each_flags.contains(EachFlags::ANIMATED));
    }

    #[test]
    fn each_shadows_outer_when_body_rebinds_outer_name() {
        let (component, data) = analyze_source(
            r#"<script>let item = $state(null);</script>
{#each items as item}<span>{item}</span>{/each}"#,
        );
        let sem = first_each_semantics(&data, &component);
        assert!(sem.shadows_outer);
    }

    #[test]
    fn each_does_not_shadow_when_names_differ() {
        let (component, data) = analyze_source(
            r#"<script>let outer = $state(null);</script>
{#each items as item}<span>{item}</span>{/each}"#,
        );
        let sem = first_each_semantics(&data, &component);
        assert!(!sem.shadows_outer);
    }
}
