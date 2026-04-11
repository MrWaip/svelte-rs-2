use crate::TemplateBindingReadKind;
use crate::types::script::RuneKind;
use oxc_ast::ast::{CallExpression, Program};
use oxc_ast_visit::Visit;
use oxc_ast_visit::walk::walk_call_expression;
use oxc_syntax::node::NodeId as OxcNodeId;
use svelte_ast::{Attribute, Component, EachBlock, Element, Fragment, IfBlock, Node, NodeId};
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

fn analyze_source(source: &str) -> (Component, AnalysisData) {
    let alloc = oxc_allocator::Allocator::default();
    let (component, js_result, parse_diags) = svelte_parser::parse_with_js(&alloc, source);
    assert!(
        parse_diags.is_empty(),
        "unexpected parse diagnostics: {parse_diags:?}"
    );
    let (data, _parsed, diags) = analyze(&component, js_result);
    assert!(diags.is_empty(), "unexpected diagnostics: {diags:?}");
    (component, data)
}

fn analyze_source_with_parsed(source: &str) -> (Component, AnalysisData, ParserResult<'_>) {
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

fn analyze_source_with_options(source: &str, options: AnalyzeOptions) -> (Component, AnalysisData) {
    let alloc = oxc_allocator::Allocator::default();
    let (component, js_result, parse_diags) = svelte_parser::parse_with_js(&alloc, source);
    assert!(
        parse_diags.is_empty(),
        "unexpected parse diagnostics: {parse_diags:?}"
    );
    let (data, _parsed, diags) = analyze_with_options(&component, js_result, &options);
    assert!(diags.is_empty(), "unexpected diagnostics: {diags:?}");
    (component, data)
}

fn analyze_source_with_diags(
    source: &str,
) -> (Component, AnalysisData, Vec<svelte_diagnostics::Diagnostic>) {
    let alloc = oxc_allocator::Allocator::default();
    let (component, js_result, parse_diags) = svelte_parser::parse_with_js(&alloc, source);
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

fn assert_is_rune(data: &AnalysisData, name: &str) {
    let root = data.scoping.root_scope_id();
    let sym_id = data
        .scoping
        .find_binding(root, name)
        .unwrap_or_else(|| panic!("no symbol '{name}'"));
    assert!(
        data.scoping.is_rune(sym_id),
        "expected '{name}' to be a rune"
    );
}

fn assert_dynamic_tag(data: &AnalysisData, component: &Component, expr_text: &str) {
    let id = find_expr_tag(&component.fragment, component, expr_text)
        .unwrap_or_else(|| panic!("no ExpressionTag with source '{expr_text}'"));
    assert!(
        data.output.dynamic_nodes.contains(&id),
        "expected ExpressionTag '{expr_text}' to be dynamic"
    );
}

fn assert_not_dynamic_tag(data: &AnalysisData, component: &Component, expr_text: &str) {
    let id = find_expr_tag(&component.fragment, component, expr_text)
        .unwrap_or_else(|| panic!("no ExpressionTag with source '{expr_text}'"));
    assert!(
        !data.output.dynamic_nodes.contains(&id),
        "expected ExpressionTag '{expr_text}' to NOT be dynamic"
    );
}

fn assert_dynamic_component(data: &AnalysisData, component: &Component, name: &str) {
    let id = find_component_node_id(&component.fragment, component, name)
        .unwrap_or_else(|| panic!("no ComponentNode with name '{name}'"));
    assert!(
        data.elements.flags.is_dynamic_component(id),
        "expected ComponentNode '{name}' to be dynamic"
    );
}

fn assert_component_ref_kind(
    data: &AnalysisData,
    component: &Component,
    name: &str,
    expected: TemplateBindingReadKind,
) {
    let id = find_component_node_id(&component.fragment, component, name)
        .unwrap_or_else(|| panic!("no ComponentNode with name '{name}'"));
    let sym_id = data
        .elements
        .flags
        .component_binding_sym(id)
        .unwrap_or_else(|| panic!("expected component binding symbol for '{name}'"));
    let actual = data.scoping.template_binding_read_kind(sym_id);
    assert_eq!(actual, expected);
}

fn assert_dynamic_if_block(data: &AnalysisData, component: &Component, test_text: &str) {
    let block = find_if_block(&component.fragment, component, test_text)
        .unwrap_or_else(|| panic!("no IfBlock with test '{test_text}'"));
    assert!(
        data.output.dynamic_nodes.contains(&block.id),
        "expected IfBlock '{test_text}' to be dynamic"
    );
}

fn assert_dynamic_each(data: &AnalysisData, component: &Component, expr_text: &str) {
    let block = find_each_block(&component.fragment, component, expr_text)
        .unwrap_or_else(|| panic!("no EachBlock with expr '{expr_text}'"));
    assert!(
        data.output.dynamic_nodes.contains(&block.id),
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

fn assert_bind_is_prop_source(
    data: &AnalysisData,
    component: &Component,
    tag_name: &str,
    bind_name: &str,
    expected: bool,
) {
    let dir_id = find_bind_directive_id(&component.fragment, component, tag_name, bind_name)
        .unwrap_or_else(|| panic!("no bind:{bind_name} on <{tag_name}>"));
    assert_eq!(
        data.template.bind_semantics.is_prop_source(dir_id),
        expected,
        "unexpected prop-source classification for bind:{bind_name} on <{tag_name}>",
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
    let root = data.scoping.root_scope_id();
    let sym_id = data
        .scoping
        .find_binding(root, name)
        .unwrap_or_else(|| panic!("no symbol '{name}'"));
    let actual = data
        .scoping
        .rune_kind(sym_id)
        .unwrap_or_else(|| panic!("'{name}' is not a rune"));
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
        info.has_call,
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
        !info.has_call,
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
        info.has_store_ref,
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
        !info.has_store_ref,
        "expected ExpressionTag '{expr_text}' to NOT have a store reference"
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
fn rune_detection() {
    let (c, data) =
        analyze_source(r#"<script>let count = $state(0); count = 1;</script><p>{count}</p>"#);
    assert_symbol(&data, "count");
    assert_is_rune(&data, "count");
    assert_dynamic_tag(&data, &c, "count");
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
    assert_is_rune(&data, "show");
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
    assert_component_ref_kind(
        &data,
        &component,
        "Derived_1",
        TemplateBindingReadKind::RuneGet,
    );
    assert_component_ref_kind(
        &data,
        &component,
        "Const_0",
        TemplateBindingReadKind::RuneGet,
    );
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
    assert_component_ref_kind(
        &data,
        &component,
        "Widget",
        TemplateBindingReadKind::PropsAccess,
    );
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
    assert_is_rune(&data, "items");
    assert_dynamic_each(&data, &c, "items");
}

#[test]
fn if_block_alternate_content_type() {
    let (c, data) = analyze_source("{#if x}Hello{:else}<span></span>{/if}");
    assert_consequent_content_type(&data, &c, "x", ContentStrategy::Static("Hello".to_string()));
    let if_id = find_if_block(&c.fragment, &c, "x").unwrap().id;
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
    // (resolved as each-block variable, not as script rune)
    let (c, data) = analyze_source(
        r#"<script>let item = $state(0); let items = $state([]);</script>{#each items as item}<p>{item}</p>{/each}"#,
    );
    assert_is_rune(&data, "item");
    // The {item} inside each is still dynamic (each-block var), but it
    // resolves to the each-block binding, not the script rune.
    assert_dynamic_tag(&data, &c, "item");

    // Verify the scope tree correctly models the shadowing:
    // root scope has `item` as a rune, each-block scope has `item` as a block var
    let root = data.scoping.root_scope_id();
    let root_sym = data.scoping.find_binding(root, "item").unwrap();
    assert!(data.scoping.is_rune(root_sym));

    // Find the each block's scope
    let each_block = find_each_block(&c.fragment, &c, "items").unwrap();
    let each_scope = data
        .scoping
        .fragment_scope(&FragmentKey::EachBody(each_block.id))
        .unwrap();
    let each_sym = data.scoping.find_binding(each_scope, "item").unwrap();
    // The each-block's `item` is NOT a rune (it's a block variable)
    assert!(!data.scoping.is_rune(each_sym));
    // And it's a different symbol than the root one
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
    assert_is_rune(&data, "count");
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
    let (component, data) = analyze_source(
        r#"<script>
    let items = $state([{ options: [{ id: 1 }] }]);
    let groups = {};
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
    assert_bind_is_prop_source(&data, &component, "input", "checked", true);
}

#[test]
fn each_context_index_tracks_index_symbol_and_bind_this_context() {
    let (component, data) = analyze_source(
        r#"<script>
    let items = [{ id: 1 }];
    let node;
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
        expr.ref_symbols.contains(&doubled_sym),
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

    let module_source = component.source_text(parsed.module_script_content_span.unwrap());
    let instance_source = component.source_text(parsed.script_content_span.unwrap());
    let module_call = find_call_node_id(
        parsed.module_program.as_ref().unwrap(),
        module_source,
        "$effect.root(() => {})",
    )
    .expect("missing module rune call");
    let instance_call = find_call_node_id(
        parsed.program.as_ref().unwrap(),
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
        expr.ref_symbols.contains(&shared_sym),
        "template expression should resolve to the module import binding"
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
        let info = parse_and_analyze("count", 0).unwrap();
        assert_eq!(info.kind, ExpressionKind::Identifier(compact("count")));
        // ref_symbols populated later by resolve_references (not during analyze_expression)
        assert!(!info.has_store_ref);
    }

    #[test]
    fn analyze_binary_expression() {
        let info = parse_and_analyze("count + 1", 0).unwrap();
        assert!(!info.has_store_ref);
        assert!(!info.has_call);
    }

    #[test]
    fn analyze_call_expression() {
        let info = parse_and_analyze("foo(a, b)", 0).unwrap();
        assert!(matches!(info.kind, ExpressionKind::CallExpression { .. }));
        assert!(info.has_call);
        assert!(info.has_side_effects);
    }

    #[test]
    fn analyze_assignment() {
        let info = parse_and_analyze("count = 10", 0).unwrap();
        assert_eq!(info.kind, ExpressionKind::Assignment);
        assert!(info.has_side_effects);
    }

    #[test]
    fn analyze_store_ref() {
        let info = parse_and_analyze("$count + 1", 0).unwrap();
        assert!(info.has_store_ref);
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
    assert!(props.props[0].is_prop_source);
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
    assert!(
        data.elements
            .flags
            .needs_textarea_value_lowering(textarea.id)
    );

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
    assert!(matches!(
        data.attribute(anchor.id, &anchor.attributes, "href"),
        Some(Attribute::Shorthand(_))
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
        actual_codes.iter().all(|code| !matches!(
            *code,
            "a11y_figcaption_parent" | "a11y_figcaption_index"
        )),
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
