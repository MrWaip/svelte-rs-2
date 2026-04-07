use crate::types::script::RuneKind;
use svelte_ast::{Attribute, Component, EachBlock, Element, Fragment, IfBlock, Node, NodeId};

use super::*;

mod a11y;

fn assert_content_strategy_variant(data: &AnalysisData, key: FragmentKey, variant: &str) {
    let actual = data
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

fn analyze_module_with_diags(source: &str) -> Vec<Diagnostic> {
    let alloc = oxc_allocator::Allocator::default();
    let (_data, diags) = analyze_module(&alloc, source, false, false);
    diags
}

fn assert_root_content_type(data: &AnalysisData, expected: ContentStrategy) {
    let actual = data
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
        data.dynamic_nodes.contains(&id),
        "expected ExpressionTag '{expr_text}' to be dynamic"
    );
}

fn assert_not_dynamic_tag(data: &AnalysisData, component: &Component, expr_text: &str) {
    let id = find_expr_tag(&component.fragment, component, expr_text)
        .unwrap_or_else(|| panic!("no ExpressionTag with source '{expr_text}'"));
    assert!(
        !data.dynamic_nodes.contains(&id),
        "expected ExpressionTag '{expr_text}' to NOT be dynamic"
    );
}

fn assert_dynamic_if_block(data: &AnalysisData, component: &Component, test_text: &str) {
    let block = find_if_block(&component.fragment, component, test_text)
        .unwrap_or_else(|| panic!("no IfBlock with test '{test_text}'"));
    assert!(
        data.dynamic_nodes.contains(&block.id),
        "expected IfBlock '{test_text}' to be dynamic"
    );
}

fn assert_dynamic_each(data: &AnalysisData, component: &Component, expr_text: &str) {
    let block = find_each_block(&component.fragment, component, expr_text)
        .unwrap_or_else(|| panic!("no EachBlock with expr '{expr_text}'"));
    assert!(
        data.dynamic_nodes.contains(&block.id),
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
        data.snippets.is_hoistable(block.id),
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
        data.element_flags.needs_ref(el.id),
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
        data.element_flags.needs_var(el.id),
        expected,
        "unexpected needs_var for <{tag_name}>",
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
        data.element_flags.has_dynamic_class_directives(el.id),
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
        .fragments
        .content_types
        .get(&FragmentKey::IfConsequent(block.id))
        .unwrap_or_else(|| panic!("no consequent content type for IfBlock '{test_text}'"));
    assert_eq!(*actual, expected);
}

fn assert_lowered_item_count(data: &AnalysisData, key: FragmentKey, expected_count: usize) {
    let lf = data
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
    let (data, _parsed, diags) = analyze(&component, js_result);
    // The assignment to the each-block var is invalid in runes mode — exactly one diagnostic.
    assert!(
        diags
            .iter()
            .all(|d| d.kind.code() == "each_item_invalid_assignment"),
        "expected only each_item_invalid_assignment diagnostics, got: {diags:?}"
    );
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

    assert_eq!(data.element_flags.class_attr_id(el.id), Some(class_attr_id));
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

    let class_candidates = data.template_elements_for_class("foo");
    assert!(class_candidates.contains(&div.id));
    assert!(class_candidates.contains(&span.id));
    assert!(class_candidates.contains(&p.id));

    let id_candidates = data.template_elements_for_id("root");
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
        data.template_element_previous_siblings(p.id).as_slice(),
        &[span.id, div.id]
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

    let parent_each_blocks = data
        .parent_each_blocks(bind_id)
        .unwrap_or_else(|| panic!("no parent each blocks for bind:group"));
    assert_eq!(parent_each_blocks, &vec![inner_each.id, outer_each.id]);
    assert!(data.contains_group_binding(inner_each.id));
    assert!(data.contains_group_binding(outer_each.id));
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
        data.bind_semantics.bind_group_value_attr(bind_id),
        Some(value_attr_id)
    );
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
    assert_eq!(
        data.bind_each_context(bind_id),
        Some(&vec!["index".to_string()])
    );
    assert!(data.each_body_uses_index(each.id));
}

#[test]
fn static_contenteditable_marks_bound_contenteditable() {
    let (component, data) = analyze_source(
        r#"<script>let html = $state('');</script><div contenteditable bind:innerHTML={html}></div>"#,
    );
    let el = find_element(&component.fragment, &component, "div").expect("no element <div>");
    assert!(data.element_flags.is_bound_contenteditable(el.id));
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
    let (data, _parsed, diags) = analyze(&component, js_result);
    assert_has_error(&diags, "attribute_contenteditable_dynamic");
    let el = find_element(&component.fragment, &component, "div").expect("no element <div>");
    assert!(!data.element_flags.is_bound_contenteditable(el.id));
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
        data.blocker_data.stmt_metas.len(),
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
        data.fragments
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
        data.debug_tags.by_fragment(&FragmentKey::Root),
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
        data.title_elements
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
        .fragments
        .lowered(&FragmentKey::ComponentNode(component_id))
        .unwrap_or_else(|| panic!("no lowered default fragment"));
    assert!(matches!(
        default_fragment.items.as_slice(),
        [FragmentItem::Element(id)] if *id == default_child.id
    ));

    let named_slots = data.snippets.component_named_slots(component_id);
    assert_eq!(named_slots.len(), 1);
    let (slot_el_id, slot_key) = named_slots[0];
    assert_eq!(slot_el_id, slotted_child.id);
    assert_eq!(
        slot_key,
        FragmentKey::NamedSlot(component_id, slotted_child.id)
    );

    let slot_fragment = data
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
    let plan = data.runtime_plan;

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
            runes: true,
            dev: true,
            component_name: "Self".to_string(),
            filename_basename: "Self.svelte".to_string(),
            warning_filter: None,
        },
    );
    let plan = data.runtime_plan;

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
    let plan = data.runtime_plan;

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
    let plan = data.runtime_plan;

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
fn runtime_plan_needs_context_without_exports_skips_pop_return() {
    let (_c, data) = analyze_source("<script>$effect(() => {});</script><p>ok</p>");
    let plan = data.runtime_plan;

    assert!(plan.needs_push);
    assert!(!plan.has_component_exports);
    assert!(!plan.has_exports);
    assert!(!plan.has_bindable);
    assert!(!plan.has_stores);
    assert!(!plan.has_ce_props);
    assert!(plan.needs_props_param);
    assert!(!plan.needs_pop_with_return);
}

// ---------------------------------------------------------------------------
// Validation diagnostics
// ---------------------------------------------------------------------------

fn analyze_with_diags(source: &str) -> Vec<svelte_diagnostics::Diagnostic> {
    let (_component, _data, diags) = analyze_source_with_diags(source);
    diags
}

fn analyze_with_options_diags(
    source: &str,
    options: AnalyzeOptions,
) -> Vec<svelte_diagnostics::Diagnostic> {
    let alloc = oxc_allocator::Allocator::default();
    let (component, js_result, parse_diags) = svelte_parser::parse_with_js(&alloc, source);
    assert!(
        parse_diags.is_empty(),
        "unexpected parse diagnostics: {parse_diags:?}"
    );
    let (_data, _parsed, diags) = analyze_with_options(&component, js_result, &options);
    diags
}

fn assert_has_error(diags: &[svelte_diagnostics::Diagnostic], code: &str) {
    assert!(
        diags.iter().any(|d| d.kind.code() == code),
        "expected error '{code}', got: {diags:?}"
    );
}

fn assert_no_error(diags: &[svelte_diagnostics::Diagnostic], code: &str) {
    assert!(
        !diags.iter().any(|d| d.kind.code() == code),
        "unexpected error '{code}': {diags:?}"
    );
}

fn assert_has_warning(diags: &[svelte_diagnostics::Diagnostic], code: &str) {
    assert!(
        diags
            .iter()
            .any(|d| d.kind.code() == code && d.severity == svelte_diagnostics::Severity::Warning),
        "expected warning '{code}', got: {diags:?}"
    );
}

fn assert_has_warning_kind(
    diags: &[svelte_diagnostics::Diagnostic],
    pred: impl Fn(&svelte_diagnostics::DiagnosticKind) -> bool,
) {
    assert!(
        diags
            .iter()
            .any(|d| d.severity == svelte_diagnostics::Severity::Warning && pred(&d.kind)),
        "expected matching warning, got: {diags:?}"
    );
}

fn assert_no_warning(diags: &[svelte_diagnostics::Diagnostic], code: &str) {
    assert!(
        !diags
            .iter()
            .any(|d| d.kind.code() == code && d.severity == svelte_diagnostics::Severity::Warning),
        "unexpected warning '{code}': {diags:?}"
    );
}

fn assert_no_errors(diags: &[svelte_diagnostics::Diagnostic]) {
    let errors: Vec<_> = diags
        .iter()
        .filter(|d| d.severity == svelte_diagnostics::Severity::Error)
        .collect();
    assert!(errors.is_empty(), "expected no errors, got: {errors:?}");
}

#[test]
fn validate_state_frozen_renamed() {
    let diags = analyze_with_diags(
        r#"<script>
let x = $state.frozen(1);
</script>"#,
    );
    assert_has_error(&diags, "rune_renamed");
}

#[test]
fn validate_state_is_removed() {
    let diags = analyze_with_diags(
        r#"<script>
let x = $state.is(a, b);
</script>"#,
    );
    assert_has_error(&diags, "rune_removed");
}

#[test]
#[ignore = "missing: <svelte:head> illegal attribute validation (analyze)"]
fn validate_svelte_head_illegal_attribute() {
    let diags = analyze_with_diags(r#"<svelte:head lang="en"></svelte:head>"#);
    assert_has_error(&diags, "svelte_head_illegal_attribute");
}

#[test]
#[ignore = "missing: <svelte:window>/<svelte:document> illegal attribute validation (analyze)"]
fn validate_svelte_window_and_document_illegal_attributes() {
    for source in [
        r#"<svelte:window class="x" />"#,
        r#"<svelte:window {...props} />"#,
        r#"<svelte:document class="x" />"#,
        r#"<svelte:document {...props} />"#,
    ] {
        let diags = analyze_with_diags(source);
        assert_has_error(&diags, "illegal_element_attribute");
    }
}

#[test]
#[ignore = "missing: <svelte:body> illegal attribute validation (analyze)"]
fn validate_svelte_body_illegal_attributes() {
    for source in [
        r#"<svelte:body class="x" />"#,
        r#"<svelte:body {...props} />"#,
    ] {
        let diags = analyze_with_diags(source);
        assert_has_error(&diags, "svelte_body_illegal_attribute");
    }
}

#[test]
#[ignore = "missing: special-element child validation (analyze)"]
fn validate_svelte_special_elements_invalid_content() {
    for name in ["window", "document", "body"] {
        let source = format!("<svelte:{name}>child</svelte:{name}>");
        let diags = analyze_with_diags(&source);
        assert_has_error(&diags, "svelte_meta_invalid_content");
    }
}

#[test]
#[ignore = "missing: <title> illegal attribute validation (analyze)"]
fn validate_title_illegal_attribute() {
    let diags = analyze_with_diags(r#"<svelte:head><title class="x">Hello</title></svelte:head>"#);
    assert_has_error(&diags, "title_illegal_attribute");
}

#[test]
#[ignore = "missing: <title> child content validation (analyze)"]
fn validate_title_invalid_content() {
    let diags =
        analyze_with_diags(r#"<svelte:head><title><span>Hello</span></title></svelte:head>"#);
    assert_has_error(&diags, "title_invalid_content");
}

#[test]
#[ignore = "missing: duplicate transition directive validation (analyze)"]
fn validate_transition_duplicate_transition() {
    let diags = analyze_with_diags(
        r#"<script>
	import { fade, fly } from 'svelte/transition';
</script>

<div transition:fade transition:fly></div>"#,
    );
    assert_has_error(&diags, "transition_duplicate");
}

#[test]
#[ignore = "missing: duplicate intro transition validation (analyze)"]
fn validate_transition_duplicate_in() {
    let diags = analyze_with_diags(
        r#"<script>
	import { fade, fly } from 'svelte/transition';
</script>

<div in:fade in:fly></div>"#,
    );
    assert_has_error(&diags, "transition_duplicate");
}

#[test]
#[ignore = "missing: duplicate outro transition validation (analyze)"]
fn validate_transition_duplicate_out() {
    let diags = analyze_with_diags(
        r#"<script>
	import { fade, fly } from 'svelte/transition';
</script>

<div out:fade out:fly></div>"#,
    );
    assert_has_error(&diags, "transition_duplicate");
}

#[test]
#[ignore = "missing: transition plus in conflict validation (analyze)"]
fn validate_transition_conflict_in() {
    let diags = analyze_with_diags(
        r#"<script>
	import { fade, fly } from 'svelte/transition';
</script>

<div transition:fade in:fly></div>"#,
    );
    assert_has_error(&diags, "transition_conflict");
}

#[test]
#[ignore = "missing: transition plus out conflict validation (analyze)"]
fn validate_transition_conflict_out() {
    let diags = analyze_with_diags(
        r#"<script>
	import { fade, fly } from 'svelte/transition';
</script>

<div transition:fade out:fly></div>"#,
    );
    assert_has_error(&diags, "transition_conflict");
}

#[test]
#[ignore = "missing: await expressions in use directive values (analyze)"]
fn validate_use_directive_illegal_await_expression() {
    let diags = analyze_with_diags(
        r#"<script>
	import { tooltip } from './actions.js';

	async function load() {
		return { text: 'hello' };
	}
</script>

<div use:tooltip={await load()}></div>"#,
    );
    assert_has_error(&diags, "illegal_await_expression");
}

#[test]
#[ignore = "missing: await expressions in transition directive values (analyze)"]
fn validate_transition_illegal_await_expression() {
    let diags = analyze_with_diags(
        r#"<script>
	import { fade } from 'svelte/transition';

	async function load() {
		return { duration: 100 };
	}
</script>

<div transition:fade={await load()}></div>"#,
    );
    assert_has_error(&diags, "illegal_await_expression");
}

#[test]
fn validate_state_invalid_placement_bare_expr() {
    let diags = analyze_with_diags(
        r#"<script>
$state(1);
</script>"#,
    );
    assert_has_error(&diags, "state_invalid_placement");
}

#[test]
fn validate_key_block_empty_warns() {
    let diags = analyze_with_diags(
        r#"<script>
let count = 1;
</script>

{#key count} {/key}"#,
    );
    assert_has_warning(&diags, "block_empty");
}

#[test]
fn validate_state_invalid_placement_fn_arg() {
    let diags = analyze_with_diags(
        r#"<script>
console.log($state(1));
</script>"#,
    );
    assert_has_error(&diags, "state_invalid_placement");
}

#[test]
fn validate_state_too_many_args() {
    let diags = analyze_with_diags(
        r#"<script>
let x = $state(1, 2);
</script>"#,
    );
    assert_has_error(&diags, "rune_invalid_arguments_length");
}

#[test]
fn validate_state_raw_too_many_args() {
    let diags = analyze_with_diags(
        r#"<script>
let x = $state.raw(1, 2);
</script>"#,
    );
    assert_has_error(&diags, "rune_invalid_arguments_length");
}

#[test]
fn validate_derived_wrong_arg_count() {
    let diags = analyze_with_diags(
        r#"<script>
let x = $derived();
</script>"#,
    );
    assert_has_error(&diags, "rune_invalid_arguments_length");
}

#[test]
fn validate_derived_by_wrong_arg_count() {
    let diags = analyze_with_diags(
        r#"<script>
let x = $derived.by(fn1, fn2);
</script>"#,
    );
    assert_has_error(&diags, "rune_invalid_arguments_length");
}

#[test]
fn validate_derived_rune_invalid_usage_in_non_runes_mode() {
    let diags = analyze_with_options_diags(
        r#"<script>
let count = 1;
let doubled = $derived(count * 2);
</script>"#,
        AnalyzeOptions {
            runes: false,
            ..AnalyzeOptions::default()
        },
    );
    assert_has_error(&diags, "rune_invalid_usage");
}

#[test]
fn validate_derived_destructured_rune_invalid_usage_in_non_runes_mode() {
    let diags = analyze_with_options_diags(
        r#"<script>
let source = { value: 1 };
let { value } = $derived(source);
</script>"#,
        AnalyzeOptions {
            runes: false,
            ..AnalyzeOptions::default()
        },
    );
    assert_has_error(&diags, "rune_invalid_usage");
}

#[test]
fn validate_derived_rune_allowed_in_runes_mode() {
    let diags = analyze_with_options_diags(
        r#"<script>
let count = $state(0);
let doubled = $derived(count * 2);
</script>"#,
        AnalyzeOptions {
            runes: true,
            ..AnalyzeOptions::default()
        },
    );
    assert!(
        !diags.iter().any(|d| d.kind.code() == "rune_invalid_usage"),
        "unexpected rune_invalid_usage: {diags:?}"
    );
}

#[test]
fn validate_derived_invalid_export() {
    let diags = analyze_with_diags(
        r#"<script>
const count = $state(0);
export const total = $derived(count * 2);
</script>"#,
    );
    assert_has_error(&diags, "derived_invalid_export");
}

#[test]
fn validate_derived_invalid_export_specifier() {
    let diags = analyze_with_diags(
        r#"<script>
const count = $state(0);
const total = $derived(count * 2);
export { total };
</script>"#,
    );
    assert_has_error(&diags, "derived_invalid_export");
}

#[test]
fn validate_derived_invalid_default_export() {
    let diags = analyze_with_diags(
        r#"<script>
const count = $state(0);
const total = $derived(count * 2);
export default total;
</script>"#,
    );
    assert_has_error(&diags, "derived_invalid_export");
}

#[test]
fn validate_state_referenced_locally_for_derived() {
    let diags = analyze_with_diags(
        r#"<script>
let count = $state(0);
let total = $derived(count * 2);
const snapshot = total;
</script>"#,
    );
    assert_has_warning(&diags, "state_referenced_locally");
}

#[test]
fn validate_state_referenced_locally_derived_type_is_derived_inside_state_arg() {
    let diags = analyze_with_diags(
        r#"<script>
let count = $state(0);
let total = $derived(count * 2);
let x = $state(total);
</script>"#,
    );
    let w = diags
        .iter()
        .find(|d| d.kind.code() == "state_referenced_locally")
        .expect("warning missing");
    assert!(
        w.kind.message().contains("derived"),
        "expected type_ == 'derived', got: {}",
        w.kind.message()
    );
}

#[test]
fn validate_state_referenced_locally_derived_no_warning_across_fn_boundary() {
    // Inside an arrow function the function_depth differs from declaration depth — no warning.
    let diags = analyze_with_diags(
        r#"<script>
let count = $state(0);
let total = $derived(count * 2);
let x = $state(() => total);
</script>"#,
    );
    assert!(
        diags
            .iter()
            .all(|d| d.kind.code() != "state_referenced_locally"),
        "unexpected warning: {diags:?}",
    );
}

#[test]
fn validate_state_referenced_locally_for_reassigned_state() {
    // Reassigned $state at the same function depth — should warn.
    let diags = analyze_with_diags(
        r#"<script>
let count = $state(0);
count = 1;
const snapshot = count;
</script>"#,
    );
    assert_has_warning(&diags, "state_referenced_locally");
}

#[test]
fn validate_state_referenced_locally_for_primitive_state() {
    // $state with primitive init and no reassignment: !is_proxy_init — should warn.
    let diags = analyze_with_diags(
        r#"<script>
let count = $state(42);
const snapshot = count;
</script>"#,
    );
    assert_has_warning(&diags, "state_referenced_locally");
}

#[test]
fn validate_state_referenced_locally_no_warning_for_proxy_state() {
    // $state({}) has is_proxy_init = true and is not reassigned — no warning.
    let diags = analyze_with_diags(
        r#"<script>
let obj = $state({ x: 1 });
obj.x = 2;
const ref_ = obj;
</script>"#,
    );
    assert!(
        diags
            .iter()
            .all(|d| d.kind.code() != "state_referenced_locally"),
        "unexpected warning for proxy state: {diags:?}",
    );
}

#[test]
fn validate_state_referenced_locally_for_state_raw() {
    // $state.raw is always warned at same function depth.
    let diags = analyze_with_diags(
        r#"<script>
let items = $state.raw([1, 2, 3]);
const snapshot = items;
</script>"#,
    );
    assert_has_warning(&diags, "state_referenced_locally");
}

#[test]
fn validate_state_referenced_locally_no_warning_across_fn_boundary_state() {
    // Inside a function the depth differs — no warning for $state either.
    let diags = analyze_with_diags(
        r#"<script>
let count = $state(0);
count = 1;
const fn_ = () => count;
</script>"#,
    );
    assert!(
        diags
            .iter()
            .all(|d| d.kind.code() != "state_referenced_locally"),
        "unexpected warning across fn boundary: {diags:?}",
    );
}

#[test]
fn validate_non_reactive_update_for_direct_template_read() {
    let diags = analyze_with_diags(
        r#"<script>
let count = 0;
count = 1;
</script>

{count}"#,
    );
    assert_has_warning(&diags, "non_reactive_update");
}

#[test]
fn validate_non_reactive_update_no_warning_across_function_boundary() {
    let diags = analyze_with_diags(
        r#"<script>
let count = 0;
count = 1;
</script>

<button onclick={() => count}></button>"#,
    );
    assert_no_warning(&diags, "non_reactive_update");
}

#[test]
fn validate_non_reactive_update_bind_this_no_warning_without_dynamic_block() {
    let diags = analyze_with_diags(
        r#"<script>
let node = null;
node = document.body;
</script>

<div bind:this={node}></div>"#,
    );
    assert_no_warning(&diags, "non_reactive_update");
}

#[test]
fn validate_non_reactive_update_bind_this_warns_inside_if_block() {
    let diags = analyze_with_diags(
        r#"<script>
let ok = true;
let node = null;
node = document.body;
</script>

{#if ok}
    <div bind:this={node}></div>
{/if}"#,
    );
    assert_has_warning(&diags, "non_reactive_update");
}

#[test]
fn validate_state_invalid_export_for_reassigned_state() {
    let diags = analyze_with_diags(
        r#"<script>
export let count = $state(0);
count = 1;
</script>"#,
    );
    assert_has_error(&diags, "state_invalid_export");
}

#[test]
fn validate_state_invalid_export_for_reassigned_state_raw() {
    let diags = analyze_with_diags(
        r#"<script>
export let items = $state.raw([]);
items = [];
</script>"#,
    );
    assert_has_error(&diags, "state_invalid_export");
}

#[test]
fn validate_state_invalid_export_no_error_without_reassignment() {
    // Property mutations do not count as reassignment.
    let diags = analyze_with_diags(
        r#"<script>
export let obj = $state({ x: 0 });
obj.x = 1;
</script>"#,
    );
    assert!(
        diags
            .iter()
            .all(|d| d.kind.code() != "state_invalid_export"),
        "unexpected error: {diags:?}",
    );
}

#[test]
fn validate_state_invalid_export_for_reassigned_state_export_specifier() {
    let diags = analyze_module_with_diags(
        r#"
let count = $state(0);
count = 1;
export { count };
"#,
    );
    assert_has_error(&diags, "state_invalid_export");
}

#[test]
fn validate_state_invalid_export_for_reassigned_state_default_export() {
    let diags = analyze_module_with_diags(
        r#"
let count = $state(0);
count = 1;
export default count;
"#,
    );
    assert_has_error(&diags, "state_invalid_export");
}

#[test]
fn validate_state_invalid_export_no_error_for_default_export_without_reassignment() {
    let diags = analyze_module_with_diags(
        r#"
let count = $state(0);
count.value = 1;
export default count;
"#,
    );
    assert!(
        diags
            .iter()
            .all(|d| d.kind.code() != "state_invalid_export"),
        "unexpected error: {diags:?}",
    );
}

#[test]
fn validate_effect_invalid_placement_fn_arg() {
    let diags = analyze_with_diags(
        r#"<script>
console.log($effect(() => {}));
</script>"#,
    );
    assert_has_error(&diags, "effect_invalid_placement");
}

#[test]
fn validate_effect_pre_invalid_placement_assignment() {
    let diags = analyze_with_diags(
        r#"<script>
let cleanup = $effect.pre(() => {});
</script>"#,
    );
    assert_has_error(&diags, "effect_invalid_placement");
}

#[test]
fn validate_effect_wrong_arg_count() {
    let diags = analyze_with_diags(
        r#"<script>
$effect();
</script>"#,
    );
    assert_has_error(&diags, "rune_invalid_arguments_length");
}

#[test]
fn validate_effect_pre_wrong_arg_count() {
    let diags = analyze_with_diags(
        r#"<script>
$effect.pre(a, b);
</script>"#,
    );
    assert_has_error(&diags, "rune_invalid_arguments_length");
}

#[test]
fn validate_effect_root_wrong_arg_count() {
    let diags = analyze_with_diags(
        r#"<script>
$effect.root();
</script>"#,
    );
    assert_has_error(&diags, "rune_invalid_arguments_length");
}

#[test]
fn validate_effect_tracking_with_argument() {
    let diags = analyze_with_diags(
        r#"<script>
$effect.tracking(someFn);
</script>"#,
    );
    assert_has_error(&diags, "rune_invalid_arguments");
}

#[test]
fn validate_inspect_requires_arguments() {
    let diags = analyze_with_diags(
        r#"<script>
$inspect();
</script>"#,
    );
    assert_has_error(&diags, "rune_invalid_arguments_length");
}

#[test]
fn validate_inspect_with_requires_callback() {
    let diags = analyze_with_diags(
        r#"<script>
$inspect(count).with();
</script>"#,
    );
    assert_has_error(&diags, "rune_invalid_arguments_length");
}

#[test]
fn validate_inspect_trace_wrong_arg_count() {
    let diags = analyze_with_diags(
        r#"<script>
function demo() {
    $inspect.trace('a', 'b');
}
</script>"#,
    );
    assert_has_error(&diags, "rune_invalid_arguments_length");
}

#[test]
fn validate_inspect_trace_invalid_placement() {
    let diags = analyze_with_diags(
        r#"<script>
function demo() {
    console.log('before');
    $inspect.trace();
}
</script>"#,
    );
    assert_has_error(&diags, "inspect_trace_invalid_placement");
}

#[test]
fn validate_inspect_trace_generator_invalid() {
    let diags = analyze_with_diags(
        r#"<script>
function* demo() {
    $inspect.trace();
}
</script>"#,
    );
    assert_has_error(&diags, "inspect_trace_generator");
}

#[test]
fn validate_state_valid_positions() {
    let diags = analyze_with_diags(
        r#"<script>
let x = $state(0);
let y = $state.raw('hello');

class Foo {
    count = $state(0);
    constructor() {
        this.name = $state('');
    }
}
</script>"#,
    );
    assert_no_errors(&diags);
}

#[test]
fn validate_state_constructor_private_field() {
    let diags = analyze_with_diags(
        r#"<script>
class Foo {
    #name;
    constructor() {
        this.#name = $state('');
    }
}
</script>"#,
    );
    assert_no_errors(&diags);
}

#[test]
fn validate_host_invalid_placement_without_custom_element() {
    let diags = analyze_with_diags(
        r#"<script>
let host = $host();
</script>"#,
    );
    assert_has_error(&diags, "host_invalid_placement");
}

#[test]
fn validate_host_invalid_arguments() {
    let diags = analyze_with_diags(
        r#"<script>
let host = $host(1);
</script>"#,
    );
    assert_has_error(&diags, "rune_invalid_arguments");
}

#[test]
fn validate_state_eager_no_args() {
    let diags = analyze_with_diags(
        r#"<script>
let x = $state.eager();
</script>"#,
    );
    assert_has_error(&diags, "rune_invalid_arguments_length");
}

#[test]
fn validate_state_eager_too_many_args() {
    let diags = analyze_with_diags(
        r#"<script>
let x = $state.eager(a, b);
</script>"#,
    );
    assert_has_error(&diags, "rune_invalid_arguments_length");
}

#[test]
fn validate_state_nested_class_in_constructor() {
    // Nested class constructor must not reset the outer constructor's flag
    let diags = analyze_with_diags(
        r#"<script>
class Outer {
    constructor() {
        const Inner = class {
            constructor() {
                this.a = $state(0);
            }
        };
        this.b = $state(1);
    }
}
</script>"#,
    );
    assert_no_errors(&diags);
}

#[test]
fn validate_bindable_invalid_location() {
    let diags = analyze_with_diags(
        r#"<script>
let value = $bindable();
</script>"#,
    );
    assert_has_error(&diags, "bindable_invalid_location");
}

#[test]
fn validate_bindable_invalid_location_inside_arrow() {
    let diags = analyze_with_diags(
        r#"<script>
let { x = () => $bindable() } = $props();
</script>"#,
    );
    assert_has_error(&diags, "bindable_invalid_location");
}

#[test]
fn validate_bindable_too_many_args() {
    let diags = analyze_with_diags(
        r#"<script>
let { value = $bindable(1, 2) } = $props();
</script>"#,
    );
    assert_has_error(&diags, "rune_invalid_arguments_length");
}

#[test]
fn validate_props_invalid_placement_inside_function() {
    let diags = analyze_with_diags(
        r#"<script>
function setup() {
    let { value } = $props();
}
</script>"#,
    );
    assert_has_error(&diags, "props_invalid_placement");
}

#[test]
fn validate_props_duplicate() {
    let diags = analyze_with_diags(
        r#"<script>
let { a } = $props();
let { b } = $props();
</script>"#,
    );
    assert_has_error(&diags, "props_duplicate");
}

#[test]
fn validate_props_and_props_id_coexist() {
    // $props() and $props.id() are allowed to coexist — only duplicate calls of
    // the SAME rune are errors (matching reference compiler behaviour).
    let diags = analyze_with_diags(
        r#"<script>
let { a } = $props();
const id = $props.id();
</script>"#,
    );
    assert!(
        !diags.iter().any(|d| d.kind.code() == "props_duplicate"),
        "unexpected props_duplicate"
    );
}

#[test]
fn validate_props_invalid_pattern_computed_key() {
    let diags = analyze_with_diags(
        r#"<script>
let { [key]: value } = $props();
</script>"#,
    );
    assert_has_error(&diags, "props_invalid_pattern");
}

#[test]
fn validate_props_id_invalid_placement_inside_function() {
    let diags = analyze_with_diags(
        r#"<script>
function setup() {
    const id = $props.id();
}
</script>"#,
    );
    assert_has_error(&diags, "props_id_invalid_placement");
}

#[test]
fn validate_const_tag_invalid_placement_root() {
    let diags = analyze_with_diags("{@const doubled = count * 2}");
    assert_has_error(&diags, "const_tag_invalid_placement");
}

#[test]
fn validate_const_tag_invalid_placement_inside_element() {
    // {@const} directly inside a regular element (without slot attr) is invalid.
    let diags = analyze_with_diags("<div>{@const x = 1}</div>");
    assert_has_error(&diags, "const_tag_invalid_placement");
}

#[test]
fn validate_const_tag_valid_placement_each() {
    let diags = analyze_with_diags("{#each items as item}{@const x = item.value}<p>{x}</p>{/each}");
    assert_no_error(&diags, "const_tag_invalid_placement");
}

#[test]
fn validate_const_tag_valid_placement_if() {
    let diags = analyze_with_diags("{#if cond}{@const x = 1}<p>{x}</p>{/if}");
    assert_no_error(&diags, "const_tag_invalid_placement");
}

#[test]
fn validate_const_tag_valid_placement_key() {
    let diags = analyze_with_diags("{#key val}{@const x = 1}<p>{x}</p>{/key}");
    assert_no_error(&diags, "const_tag_invalid_placement");
}

#[test]
fn validate_const_tag_invalid_expression() {
    // `{@const a = b, c = d}` has an unparenthesized sequence expression as init.
    // OXC parses `const a = b, c = d;` as two declarators, triggering the error.
    let diags = analyze_with_diags("{#each items as item}{@const a = item.x, b = item.y}{/each}");
    assert_has_error(&diags, "const_tag_invalid_expression");
}

#[test]
fn validate_const_tag_parenthesized_sequence_ok() {
    // `{@const a = (b, c)}` is valid — the sequence is inside parens.
    let diags = analyze_with_diags("{#each items as item}{@const a = (item.x, item.y)}{/each}");
    assert_no_error(&diags, "const_tag_invalid_expression");
}

#[test]
#[ignore = "each_key_without_as is unreachable from valid Svelte template syntax — \
            the JS expression parser always consumes (key) as a call expression"]
fn validate_each_key_without_as() {
    let diags = analyze_with_diags("{#each items (item.id)}<p />{/each}");
    assert_has_error(&diags, "each_key_without_as");
}

#[test]
fn validate_each_animation_missing_key() {
    let diags = analyze_with_diags(
        r#"<script>import { flip } from 'svelte/animate'; let items = [];</script>
{#each items as item}
    <div animate:flip>{item}</div>
{/each}"#,
    );
    assert_has_error(&diags, "animation_missing_key");
}

#[test]
fn validate_each_animation_invalid_placement() {
    let diags = analyze_with_diags(
        r#"<script>import { flip } from 'svelte/animate'; let items = [];</script>
{#each items as item (item.id)}
    <div animate:flip>{item}</div>
    <span>extra</span>
{/each}"#,
    );
    assert_has_error(&diags, "animation_invalid_placement");
}

#[test]
fn validate_each_item_invalid_assignment() {
    let diags = analyze_with_diags(
        r#"<script>let items = $state([1, 2, 3]);</script>
{#each items as item}
    {item = 1}
{/each}"#,
    );
    assert_has_error(&diags, "each_item_invalid_assignment");
}

#[test]
fn validate_each_item_invalid_assignment_bind_identifier() {
    let diags = analyze_with_diags(
        r#"<script>let items = $state([{ value: "a" }]);</script>
{#each items as item}
    <input bind:value={item}>
{/each}"#,
    );
    assert_has_error(&diags, "each_item_invalid_assignment");
}

#[test]
fn validate_each_item_bind_member_expression_no_invalid_assignment() {
    let diags = analyze_with_diags(
        r#"<script>let items = $state([{ value: "a" }]);</script>
{#each items as item}
    <input bind:value={item.value}>
{/each}"#,
    );
    assert!(
        diags
            .iter()
            .all(|d| d.kind.code() != "each_item_invalid_assignment"),
        "unexpected error: {diags:?}",
    );
}

#[test]
fn validate_each_item_invalid_assignment_array_destructure() {
    let diags = analyze_with_diags(
        r#"<script>let items = $state([1, 2, 3]);</script>
{#each items as item}
    {([item] = [1])}
{/each}"#,
    );
    assert_has_error(&diags, "each_item_invalid_assignment");
}

#[test]
fn validate_text_invalid_placement() {
    let diags = analyze_with_diags("<table>bad</table>");
    assert_has_error(&diags, "node_invalid_placement");
}

#[test]
fn validate_expression_tag_invalid_placement() {
    let diags = analyze_with_diags("<table>{value}</table>");
    assert_has_error(&diags, "node_invalid_placement");
}

#[test]
fn validate_text_bidirectional_control_warning() {
    let source = format!("<p>before {} after</p>", '\u{202E}');
    let diags = analyze_with_diags(&source);
    assert_has_warning(&diags, "bidirectional_control_characters");
}

#[test]
fn validate_text_bidirectional_control_warning_ignored() {
    let source = format!(
        "<!-- svelte-ignore bidirectional_control_characters --><p>before {} after</p>",
        '\u{202E}'
    );
    let diags = analyze_with_diags(&source);
    assert!(
        !diags
            .iter()
            .any(|d| d.kind.code() == "bidirectional_control_characters"),
        "expected bidi warning to be ignored, got: {diags:?}"
    );
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
        data.element_flags
            .needs_textarea_value_lowering(textarea.id)
    );

    assert!(data.fragment_has_expression_child(&FragmentKey::Element(option.id)));
    assert!(matches!(
        component.store.get(option_expr),
        Node::ExpressionTag(_)
    ));
    assert_eq!(
        data.element_flags.option_synthetic_value_expr(option.id),
        Some(option_expr)
    );
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
fn fragment_facts_single_child_supports_block_empty() {
    let (component, data, diags) = analyze_source_with_diags("{#if ok} {/if}");
    let block = find_if_block(&component.fragment, &component, "ok").expect("expected if block");

    assert_has_warning(&diags, "block_empty");

    let child_id = data
        .fragment_single_child(&FragmentKey::IfConsequent(block.id))
        .expect("expected single child");
    assert!(data.fragment_has_children(&FragmentKey::IfConsequent(block.id)));
    assert!(matches!(component.store.get(child_id), Node::Text(_)));
}

#[test]
fn fragment_facts_track_non_trivial_child_counts() {
    let (component, data, diags) = analyze_source_with_diags(
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

    assert_has_error(&diags, "snippet_conflict");
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

    assert!(data.element_flags.is_customizable_select(rich_select.id));
    assert!(!data.element_flags.is_customizable_select(plain_select.id));
    assert!(data.element_flags.is_customizable_select(optgroup.id));
    assert!(data.element_flags.is_customizable_select(option.id));
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
fn validate_snippet_parameter_assignment() {
    let diags = analyze_with_diags(
        r#"{#snippet view(value)}
    {value = 1}
{/snippet}"#,
    );
    assert_has_error(&diags, "snippet_parameter_assignment");
}

#[test]
fn validate_snippet_parameter_assignment_in_nested_target() {
    let diags = analyze_with_diags(
        r#"{#snippet view(value)}
    {({ current: value } = { current: 1 })}
{/snippet}"#,
    );
    assert_has_error(&diags, "snippet_parameter_assignment");
}

#[test]
fn validate_snippet_invalid_rest_parameter() {
    let diags = analyze_with_diags(
        r#"{#snippet view(...args)}
    <p>{args.length}</p>
{/snippet}"#,
    );
    assert_has_error(&diags, "snippet_invalid_rest_parameter");
}

#[test]
fn validate_snippet_shadowing_prop() {
    let diags = analyze_with_diags(
        r#"<Widget title="hello">
    {#snippet title()}
        <p>shadowed</p>
    {/snippet}
</Widget>"#,
    );
    assert_has_error(&diags, "snippet_shadowing_prop");
}

#[test]
fn validate_snippet_conflict() {
    let diags = analyze_with_diags(
        r#"<Widget>
    before
    {#snippet children()}
        <p>inside</p>
    {/snippet}
</Widget>"#,
    );
    assert_has_error(&diags, "snippet_conflict");
}

#[test]
fn validate_snippet_children_without_other_content_has_no_conflict() {
    let diags = analyze_with_diags(
        r#"<Widget>
    <!-- comment -->
    {#snippet children()}
        <p>inside</p>
    {/snippet}
    
</Widget>"#,
    );
    assert!(
        !diags.iter().any(|d| d.kind.code() == "snippet_conflict"),
        "unexpected snippet_conflict diagnostics: {diags:?}"
    );
}

#[test]
fn validate_snippet_invalid_export() {
    let diags = analyze_with_diags(
        r#"<script module>
    export { greeting };
</script>

<script>
    let message = 'hello';
</script>

{#snippet greeting(name)}
    <p>{message} {name}!</p>
{/snippet}"#,
    );
    assert_has_error(&diags, "snippet_invalid_export");
}

#[test]
fn validate_module_illegal_default_export() {
    let diags = analyze_with_diags(
        r#"<script module>
    const value = 1;
    export default value;
</script>"#,
    );
    assert_has_error(&diags, "module_illegal_default_export");
}

#[test]
fn validate_module_illegal_default_export_function() {
    let diags = analyze_with_diags(
        r#"<script module>
    export default function value() {
        return 1;
    }
</script>"#,
    );
    assert_has_error(&diags, "module_illegal_default_export");
}

#[test]
fn validate_module_illegal_default_export_specifier() {
    let diags = analyze_with_diags(
        r#"<script module>
    const value = 1;
    export { value as default };
</script>"#,
    );
    assert_has_error(&diags, "module_illegal_default_export");
}

#[test]
fn validate_snippet_invalid_export_no_false_positive() {
    // Exporting a non-snippet name from module script is not a snippet_invalid_export.
    let diags = analyze_with_diags(
        r#"<script module>
    export const PI = 3.14;
</script>"#,
    );
    assert!(
        !diags
            .iter()
            .any(|d| d.kind.code() == "snippet_invalid_export"),
        "should not fire snippet_invalid_export for a non-snippet export"
    );
}

#[test]
fn validate_snippet_invalid_export_module_bound_no_fire() {
    // If the exported name is declared in <script module>, it is NOT snippet_invalid_export
    // (matches reference compiler: only fires when the name has no module-scope binding).
    let diags = analyze_with_diags(
        r#"<script module>
    const greeting = () => "module fn";
    export { greeting };
</script>

{#snippet greeting()}
    <p>template snippet</p>
{/snippet}"#,
    );
    assert!(
        !diags
            .iter()
            .any(|d| d.kind.code() == "snippet_invalid_export"),
        "should not fire snippet_invalid_export when name is declared in module scope"
    );
}

#[test]
fn validate_each_item_invalid_assignment_nested_object_destructure() {
    let diags = analyze_with_diags(
        r#"<script>let items = $state([1, 2, 3]); let value = { nested: { current: 1 } };</script>
{#each items as item}
    {({ nested: { current: item } } = value)}
{/each}"#,
    );
    assert_has_error(&diags, "each_item_invalid_assignment");
}

#[test]
fn validate_bind_invalid_name() {
    let diags = analyze_with_diags(
        r#"<script>let value = $state('');</script>
<input bind:vale={value}>"#,
    );
    assert_has_error(&diags, "bind_invalid_name");
}

#[test]
fn validate_bind_invalid_expression() {
    let diags = analyze_with_diags(
        r#"<script>
    let value = $state('');
    let getter = () => value;
</script>
<input bind:value={getter()}>"#,
    );
    assert_has_error(&diags, "bind_invalid_expression");
}

#[test]
fn validate_bind_invalid_value() {
    // bind_invalid_value fires when binding to a non-writable rune (e.g. $derived)
    let diags = analyze_with_diags(
        r#"<script>let value = $derived('');</script>
<input bind:value={value}>"#,
    );
    assert_has_error(&diags, "bind_invalid_value");
}

#[test]
fn validate_bind_plain_let_is_valid() {
    // Plain let variables (non-rune) are bindable — the bind directive's setter writes to them.
    let diags = analyze_with_diags(
        r#"<script>let value = '';</script>
<input bind:value={value}>"#,
    );
    assert!(
        !diags.iter().any(|d| d.kind.code() == "bind_invalid_value"),
        "plain let should not fire bind_invalid_value"
    );
}

#[test]
fn validate_attribute_contenteditable_missing() {
    let diags = analyze_with_diags(
        r#"<script>let html = $state('');</script>
<div bind:innerHTML={html}></div>"#,
    );
    assert_has_error(&diags, "attribute_contenteditable_missing");
}

#[test]
fn validate_bind_invalid_target() {
    let diags = analyze_with_diags(
        r#"<script>let value = $state('');</script>
<div bind:value={value}></div>"#,
    );
    assert_has_error(&diags, "bind_invalid_target");
}

#[test]
fn validate_bind_invalid_name_with_special_element_candidates() {
    let diags = analyze_with_diags(
        r#"<script>let width = $state(0);</script>
<svelte:window bind:innerHTML={width} />"#,
    );
    assert_has_error(&diags, "bind_invalid_name");
}

#[test]
fn validate_bind_invalid_parens() {
    let diags = analyze_with_diags(
        r#"<script>
    let value = $state('');
</script>
<input bind:value={( () => value, (next) => value = next )}>"#,
    );
    assert_has_error(&diags, "bind_invalid_parens");
}

#[test]
fn validate_bind_getter_setter_without_parens() {
    let diags = analyze_with_diags(
        r#"<script>
    let value = $state('');
</script>
<input bind:value={
    () => value,
    (next) => value = next
}>"#,
    );
    assert_no_errors(&diags);
}

#[test]
fn validate_bind_group_invalid_expression() {
    let diags = analyze_with_diags(
        r#"<script>
    let selected = $state([]);
</script>
<input type="checkbox" bind:group={
    () => selected,
    (next) => selected = next
}>"#,
    );
    assert_has_error(&diags, "bind_group_invalid_expression");
}

#[test]
fn validate_bind_sequence_reports_all_relevant_errors() {
    let diags = analyze_with_diags(
        r#"<script>
    let selected = $state([]);
</script>
<input type="checkbox" bind:group={(() => selected, (next) => selected = next, selected)}>"#,
    );
    assert_has_error(&diags, "bind_group_invalid_expression");
    assert_has_error(&diags, "bind_invalid_parens");
    assert_has_error(&diags, "bind_invalid_expression");
}

#[test]
fn validate_bind_group_invalid_snippet_parameter() {
    let diags = analyze_with_diags(
        r#"{#snippet field(selected)}
    <input type="checkbox" bind:group={selected} />
{/snippet}"#,
    );
    assert_has_error(&diags, "bind_group_invalid_snippet_parameter");
}

#[test]
fn validate_attribute_contenteditable_dynamic() {
    let diags = analyze_with_diags(
        r#"<script>
    let html = $state('');
    let editable = true;
</script>
<div contenteditable={editable} bind:innerHTML={html}></div>"#,
    );
    assert_has_error(&diags, "attribute_contenteditable_dynamic");
}

#[test]
fn validate_attribute_invalid_type() {
    let diags = analyze_with_diags(
        r#"<script>
    let checked = $state(false);
    let input_type = 'checkbox';
</script>
<input type={input_type} bind:checked={checked}>"#,
    );
    assert_has_error(&diags, "attribute_invalid_type");
}

#[test]
fn validate_attribute_invalid_multiple() {
    let diags = analyze_with_diags(
        r#"<script>
    let value = $state('a');
    let multiple = true;
</script>
<select multiple={multiple} bind:value={value}></select>"#,
    );
    assert_has_error(&diags, "attribute_invalid_multiple");
}

#[test]
fn validate_bind_invalid_each_rest() {
    let diags = analyze_with_diags(
        r#"<script>
    let items = $state([{ selected: [] }]);
</script>
{#each items as { ...rest }}
    <input type="checkbox" bind:group={rest} value="x" />
{/each}"#,
    );
    assert_has_warning(&diags, "bind_invalid_each_rest");
}

#[test]
fn validate_bind_checked_radio_target() {
    let diags = analyze_with_diags(
        r#"<script>let checked = $state(false);</script>
<input type="radio" bind:checked={checked}>"#,
    );
    assert_has_error(&diags, "bind_invalid_target");
}

#[test]
fn validate_bind_files_wrong_input_type() {
    let diags = analyze_with_diags(
        r#"<script>let files = $state();</script>
<input type="text" bind:files={files}>"#,
    );
    assert_has_error(&diags, "bind_invalid_target");
}

#[test]
fn validate_bind_member_expression_no_error() {
    let diags = analyze_with_diags(
        r#"<script>
    let state = $state({ value: '' });
</script>
<input bind:value={state.value}>"#,
    );
    assert_no_errors(&diags);
}

#[test]
fn validate_bind_getter_setter_no_error() {
    let diags = analyze_with_diags(
        r#"<script>
    let value = $state('');
</script>
<input bind:value={() => value, (next) => value = next}>"#,
    );
    assert_no_errors(&diags);
}

#[test]
fn validate_store_invalid_scoped_subscription() {
    let diags = analyze_with_diags(
        r#"<script>
function foo() {
    let count = 0;
    console.log($count);
}
</script>"#,
    );
    assert_has_error(&diags, "store_invalid_scoped_subscription");
}

#[test]
fn validate_store_rune_conflict() {
    let diags = analyze_with_diags(
        r#"<script>
import { writable } from 'svelte/store';
let state = writable(0);
let x = $state(0);
</script>"#,
    );
    assert_has_warning(&diags, "store_rune_conflict");
}

#[test]
fn validate_store_invalid_subscription_in_module() {
    let diags = analyze_with_diags(
        r#"<script>
import { writable } from 'svelte/store';
const count = writable(0);
</script>
<script module>
console.log($count);
</script>"#,
    );
    assert_has_error(&diags, "store_invalid_subscription");
}

#[test]
fn validate_props_illegal_name_rest_member_access() {
    let diags = analyze_with_diags(
        r#"<script>
let { x, ...rest } = $props();
console.log(rest.$$slots);
</script>"#,
    );
    assert_has_error(&diags, "props_illegal_name");
}

#[test]
fn validate_props_illegal_name_identifier_pattern_member_access() {
    let diags = analyze_with_diags(
        r#"<script>
const props = $props();
console.log(props.$$props);
</script>"#,
    );
    assert_has_error(&diags, "props_illegal_name");
}

#[test]
fn validate_props_normal_member_access_no_error() {
    let diags = analyze_with_diags(
        r#"<script>
let { x, ...rest } = $props();
console.log(rest.normalProp);
</script>"#,
    );
    assert_no_errors(&diags);
}

#[test]
fn validate_custom_element_props_identifier_warns() {
    let diags = analyze_with_options_diags(
        r#"<svelte:options customElement={{ tag: 'x-foo' }} />
<script>
const props = $props();
</script>
<p>{props.x}</p>"#,
        AnalyzeOptions {
            custom_element: true,
            ..Default::default()
        },
    );
    assert_has_warning(&diags, "custom_element_props_identifier");
}

#[test]
fn validate_custom_element_props_rest_warns() {
    let diags = analyze_with_options_diags(
        r#"<svelte:options customElement={{ tag: 'x-foo' }} />
<script>
let { x, ...rest } = $props();
</script>
<p>{x}</p>"#,
        AnalyzeOptions {
            custom_element: true,
            ..Default::default()
        },
    );
    assert_has_warning(&diags, "custom_element_props_identifier");
}

#[test]
fn validate_custom_element_props_destructured_no_warn() {
    let diags = analyze_with_options_diags(
        r#"<svelte:options customElement={{ tag: 'x-foo' }} />
<script>
let { x, y } = $props();
</script>
<p>{x}{y}</p>"#,
        AnalyzeOptions {
            custom_element: true,
            ..Default::default()
        },
    );
    assert_no_warning(&diags, "custom_element_props_identifier");
}

#[test]
fn validate_custom_element_with_explicit_props_config_no_warn() {
    let diags = analyze_with_options_diags(
        r#"<svelte:options customElement={{ tag: 'x-foo', props: { x: { reflect: true, type: 'Number' } } }} />
<script>
const props = $props();
</script>
<p>{props.x}</p>"#,
        AnalyzeOptions {
            custom_element: true,
            ..Default::default()
        },
    );
    assert_no_warning(&diags, "custom_element_props_identifier");
}

#[test]
fn validate_options_accessors_warns_in_runes_mode() {
    let diags = analyze_with_options_diags(
        r#"<svelte:options accessors={true} />
<p>ok</p>"#,
        AnalyzeOptions {
            runes: true,
            ..Default::default()
        },
    );
    assert_has_warning(&diags, "options_deprecated_accessors");
}

#[test]
fn validate_options_accessors_no_warn_in_legacy_mode() {
    let diags = analyze_with_options_diags(
        r#"<svelte:options accessors={true} />
<p>ok</p>"#,
        AnalyzeOptions {
            runes: false,
            ..Default::default()
        },
    );
    assert_no_warning(&diags, "options_deprecated_accessors");
}

#[test]
fn validate_options_immutable_warns_in_runes_mode() {
    let diags = analyze_with_options_diags(
        r#"<svelte:options immutable={true} />
<p>ok</p>"#,
        AnalyzeOptions {
            runes: true,
            ..Default::default()
        },
    );
    assert_has_warning(&diags, "options_deprecated_immutable");
}

#[test]
fn validate_options_immutable_no_warn_in_legacy_mode() {
    let diags = analyze_with_options_diags(
        r#"<svelte:options immutable={true} />
<p>ok</p>"#,
        AnalyzeOptions {
            runes: false,
            ..Default::default()
        },
    );
    assert_no_warning(&diags, "options_deprecated_immutable");
}

#[test]
fn validate_options_custom_element_warns_without_compiler_flag() {
    let diags = analyze_with_options_diags(
        r#"<svelte:options customElement="x-foo" />
<p>ok</p>"#,
        AnalyzeOptions::default(),
    );
    assert_has_warning(&diags, "options_missing_custom_element");
}

#[test]
fn validate_options_custom_element_no_warn_with_compiler_flag() {
    let diags = analyze_with_options_diags(
        r#"<svelte:options customElement="x-foo" />
<p>ok</p>"#,
        AnalyzeOptions {
            custom_element: true,
            ..Default::default()
        },
    );
    assert_no_warning(&diags, "options_missing_custom_element");
}

#[test]
fn validate_perf_avoid_nested_class_no_warning_at_instance_top_level() {
    let diags = analyze_with_diags(
        r#"<script>
class Foo {}
</script>"#,
    );
    assert_no_warning(&diags, "perf_avoid_nested_class");
}

#[test]
fn validate_perf_avoid_nested_class_warns_in_instance_nested_function() {
    let diags = analyze_with_diags(
        r#"<script>
function outer() {
    class Foo {}
}
</script>"#,
    );
    assert_has_warning(&diags, "perf_avoid_nested_class");
}

#[test]
fn validate_perf_avoid_nested_class_no_warning_at_module_top_level() {
    let diags = analyze_with_diags(
        r#"<script module>
class Foo {}
</script>"#,
    );
    assert_no_warning(&diags, "perf_avoid_nested_class");
}

#[test]
fn validate_perf_avoid_nested_class_warns_in_module_nested_function() {
    let diags = analyze_with_diags(
        r#"<script module>
function outer() {
    class Foo {}
}
</script>"#,
    );
    assert_has_warning(&diags, "perf_avoid_nested_class");
}

#[test]
fn validate_perf_avoid_inline_class_warns_at_instance_top_level() {
    let diags = analyze_with_diags(
        r#"<script>
const value = new class {}();
</script>"#,
    );
    assert_has_warning(&diags, "perf_avoid_inline_class");
}

#[test]
fn validate_perf_avoid_inline_class_no_warning_at_module_top_level() {
    let diags = analyze_with_diags(
        r#"<script module>
const value = new class {}();
</script>"#,
    );
    assert_no_warning(&diags, "perf_avoid_inline_class");
}

#[test]
fn validate_perf_avoid_inline_class_warns_in_nested_function() {
    let diags = analyze_with_diags(
        r#"<script>
function outer() {
    const value = new class {}();
}
</script>"#,
    );
    assert_has_warning(&diags, "perf_avoid_inline_class");
}

// -----------------------------------------------------------------------
// Event directive diagnostics
// -----------------------------------------------------------------------

#[test]
fn on_directive_invalid_modifier() {
    let diags =
        analyze_with_diags("<script>function f(){}</script><div on:click|invalid={f}></div>");
    assert_has_error(&diags, "event_handler_invalid_modifier");
}

#[test]
fn on_directive_passive_nonpassive_conflict() {
    let diags = analyze_with_diags(
        "<script>function f(){}</script><div on:touchmove|passive|nonpassive={f}></div>",
    );
    assert_has_error(&diags, "event_handler_invalid_modifier_combination");
}

#[test]
fn on_directive_deprecated_in_runes_mode() {
    let diags = analyze_with_options_diags(
        "<script>function f(){}</script><div on:click={f}></div>",
        AnalyzeOptions {
            runes: true,
            ..Default::default()
        },
    );
    assert_has_warning(&diags, "event_directive_deprecated");
}

#[test]
fn on_directive_not_deprecated_in_non_runes_mode() {
    let diags = analyze_with_options_diags(
        "<script>function f(){}</script><div on:click={f}></div>",
        AnalyzeOptions {
            runes: false,
            ..Default::default()
        },
    );
    assert_no_warning(&diags, "event_directive_deprecated");
}

#[test]
fn on_directive_mixed_syntax() {
    let diags = analyze_with_diags(
        "<script>function f(){} function g(){}</script><div onclick={f} on:click={g}></div>",
    );
    assert_has_error(&diags, "mixed_event_handler_syntaxes");
}

#[test]
fn on_directive_mixed_syntax_svelte_element() {
    let diags = analyze_with_diags(
        r#"<script>let tag = $state("div"); function f(){} function g(){}</script><svelte:element this={tag} onclick={f} on:click={g}></svelte:element>"#,
    );
    assert_has_error(&diags, "mixed_event_handler_syntaxes");
}

#[test]
fn svelte_component_deprecated_warns_in_runes_mode() {
    let diags = analyze_with_options_diags(
        "<script>import A from './A.svelte'; let current = A;</script><svelte:component this={current}></svelte:component>",
        AnalyzeOptions {
            runes: true,
            ..Default::default()
        },
    );
    assert_has_warning(&diags, "svelte_component_deprecated");
}

#[test]
fn svelte_component_deprecated_no_warn_in_legacy_mode() {
    let diags = analyze_with_options_diags(
        "<script>import A from './A.svelte'; let current = A;</script><svelte:component this={current}></svelte:component>",
        AnalyzeOptions {
            runes: false,
            ..Default::default()
        },
    );
    assert_no_warning(&diags, "svelte_component_deprecated");
}

#[test]
fn svelte_self_deprecated_warns_with_default_self_import_hint() {
    let diags = analyze_with_options_diags(
        "<svelte:self></svelte:self>",
        AnalyzeOptions {
            runes: true,
            ..Default::default()
        },
    );
    assert_has_warning_kind(&diags, |kind| {
        matches!(
            kind,
            svelte_diagnostics::DiagnosticKind::SvelteSelfDeprecated { name, basename }
                if name == "Self" && basename == "Self.svelte"
        )
    });
}

#[test]
fn svelte_self_deprecated_warns_with_configured_self_import_hint() {
    let diags = analyze_with_options_diags(
        "<svelte:self></svelte:self>",
        AnalyzeOptions {
            runes: true,
            component_name: "Counter".to_string(),
            filename_basename: "Counter.svelte".to_string(),
            ..Default::default()
        },
    );
    assert_has_warning_kind(&diags, |kind| {
        matches!(
            kind,
            svelte_diagnostics::DiagnosticKind::SvelteSelfDeprecated { name, basename }
                if name == "Counter" && basename == "Counter.svelte"
        )
    });
}

#[test]
fn svelte_self_deprecated_no_warn_in_legacy_mode() {
    let diags = analyze_with_options_diags(
        "<svelte:self></svelte:self>",
        AnalyzeOptions {
            runes: false,
            ..Default::default()
        },
    );
    assert_no_warning(&diags, "svelte_self_deprecated");
}

#[test]
fn slot_attribute_invalid_placement_root() {
    let diags = analyze_with_diags(r#"<script></script><div slot="foo">content</div>"#);
    assert_has_error(&diags, "slot_attribute_invalid_placement");
}

#[test]
fn slot_attribute_invalid_placement_nested_inside_component() {
    let diags =
        analyze_with_diags(r#"<Comp><section><div slot="foo">content</div></section></Comp>"#);
    assert_has_error(&diags, "slot_attribute_invalid_placement");
}

#[test]
fn invalid_text_parent_uses_topology_ancestor_lookup() {
    let diags = analyze_with_diags(r#"<table><tbody><tr>text</tr></tbody></table>"#);
    assert_has_error(&diags, "node_invalid_placement");
}

// ---------------------------------------------------------------------------
// AwaitBlock diagnostics
// ---------------------------------------------------------------------------

#[test]
fn await_valid_then_catch_no_unexpected_character() {
    // Well-formed {#await}{:then val}{:catch e}{/await} must not emit block_unexpected_character.
    let diags = analyze_with_options_diags(
        r#"{#await p}{:then val}ok{:catch e}err{/await}"#,
        AnalyzeOptions {
            runes: true,
            ..AnalyzeOptions::default()
        },
    );
    assert!(
        !diags
            .iter()
            .any(|d| d.kind.code() == "block_unexpected_character"),
        "unexpected block_unexpected_character: {diags:?}"
    );
}

// ---------------------------------------------------------------------------
// DebugTag diagnostics
// ---------------------------------------------------------------------------

#[test]
fn debug_tag_valid_runes_no_unexpected_character() {
    // Well-formed {@debug x} in runes mode must not emit block_unexpected_character.
    let diags = analyze_with_options_diags(
        r#"{@debug x}"#,
        AnalyzeOptions {
            runes: true,
            ..AnalyzeOptions::default()
        },
    );
    assert!(
        !diags
            .iter()
            .any(|d| d.kind.code() == "block_unexpected_character"),
        "unexpected block_unexpected_character for valid debug tag: {diags:?}"
    );
}

#[test]
fn debug_tag_non_runes_skips_opening_tag_check() {
    // The validate_opening_tag check is runes-only; non-runes mode must not fire it.
    let diags = analyze_with_options_diags(
        r#"{@debug x}"#,
        AnalyzeOptions {
            runes: false,
            ..AnalyzeOptions::default()
        },
    );
    assert!(
        !diags
            .iter()
            .any(|d| d.kind.code() == "block_unexpected_character"),
        "unexpected block_unexpected_character in non-runes mode: {diags:?}"
    );
}

// ---------------------------------------------------------------------------
// $inspect / $inspect().with / $inspect.trace validation
// ---------------------------------------------------------------------------

#[test]
fn validate_inspect_zero_args() {
    let diags = analyze_with_diags(
        r#"<script>
$inspect();
</script>"#,
    );
    assert_has_error(&diags, "rune_invalid_arguments_length");
}

#[test]
fn validate_inspect_one_or_more_args_ok() {
    let diags = analyze_with_diags(
        r#"<script>
let a = $state(1);
$inspect(a);
</script>"#,
    );
    assert_no_errors(&diags);
}

#[test]
fn validate_inspect_with_wrong_arg_count_zero() {
    let diags = analyze_with_diags(
        r#"<script>
let a = $state(1);
$inspect(a).with();
</script>"#,
    );
    assert_has_error(&diags, "rune_invalid_arguments_length");
}

#[test]
fn validate_inspect_with_wrong_arg_count_two() {
    let diags = analyze_with_diags(
        r#"<script>
let a = $state(1);
$inspect(a).with(console.log, console.warn);
</script>"#,
    );
    assert_has_error(&diags, "rune_invalid_arguments_length");
}

#[test]
fn validate_inspect_with_one_arg_ok() {
    let diags = analyze_with_diags(
        r#"<script>
let a = $state(1);
$inspect(a).with(console.log);
</script>"#,
    );
    assert_no_errors(&diags);
}

#[test]
fn validate_inspect_trace_too_many_args() {
    let diags = analyze_with_diags(
        r#"<script>
function fn1() {
    $inspect.trace("a", "b");
}
</script>"#,
    );
    assert_has_error(&diags, "rune_invalid_arguments_length");
}

#[test]
fn validate_inspect_trace_zero_args_ok() {
    let diags = analyze_with_diags(
        r#"<script>
function fn1() {
    $inspect.trace();
}
</script>"#,
    );
    assert_no_errors(&diags);
}

#[test]
fn validate_inspect_trace_one_arg_ok() {
    let diags = analyze_with_diags(
        r#"<script>
function fn1() {
    $inspect.trace("label");
}
</script>"#,
    );
    assert_no_errors(&diags);
}

#[test]
fn validate_inspect_trace_invalid_placement_top_level() {
    let diags = analyze_with_diags(
        r#"<script>
$inspect.trace();
</script>"#,
    );
    assert_has_error(&diags, "inspect_trace_invalid_placement");
}

#[test]
fn validate_inspect_trace_invalid_placement_not_first_stmt() {
    let diags = analyze_with_diags(
        r#"<script>
function fn1() {
    let x = 1;
    $inspect.trace();
}
</script>"#,
    );
    assert_has_error(&diags, "inspect_trace_invalid_placement");
}

#[test]
fn validate_inspect_trace_valid_in_arrow() {
    let diags = analyze_with_diags(
        r#"<script>
const fn1 = () => {
    $inspect.trace();
};
</script>"#,
    );
    assert_no_errors(&diags);
}

#[test]
fn validate_inspect_trace_generator_rejected() {
    let diags = analyze_with_diags(
        r#"<script>
function* gen() {
    $inspect.trace();
}
</script>"#,
    );
    assert_has_error(&diags, "inspect_trace_generator");
}

// ---------------------------------------------------------------------------
// textarea_invalid_content
// ---------------------------------------------------------------------------

#[test]
fn textarea_invalid_content_fires() {
    let diags = analyze_with_diags(r#"<textarea value="x">content</textarea>"#);
    assert_has_error(&diags, "textarea_invalid_content");
}

#[test]
fn textarea_no_conflict_without_value_attr() {
    let diags = analyze_with_diags(r#"<textarea>content</textarea>"#);
    assert!(
        !diags
            .iter()
            .any(|d| d.kind.code() == "textarea_invalid_content")
    );
}

// ---------------------------------------------------------------------------
// slot_attribute_invalid
// ---------------------------------------------------------------------------

#[test]
fn slot_attribute_invalid_expression_value() {
    let diags = analyze_with_diags(
        r#"<script>let s = $state('foo');</script><Comp><div slot={s}></div></Comp>"#,
    );
    assert_has_error(&diags, "slot_attribute_invalid");
}

#[test]
fn slot_attribute_static_value_ok() {
    let diags = analyze_with_diags(r#"<Comp><div slot="header"></div></Comp>"#);
    assert!(
        !diags
            .iter()
            .any(|d| d.kind.code() == "slot_attribute_invalid")
    );
}

#[test]
fn const_tag_inside_slotted_element_is_allowed() {
    let diags =
        analyze_with_diags(r#"<Comp><div slot="header">{@const foo = 1}{foo}</div></Comp>"#);
    assert!(
        !diags
            .iter()
            .any(|d| d.kind.code() == "const_tag_invalid_placement")
    );
}

// ---------------------------------------------------------------------------
// attribute_quoted
// ---------------------------------------------------------------------------

#[test]
fn attribute_quoted_on_component() {
    let diags = analyze_with_diags(r#"<script>let x = $state('val');</script><Comp foo="{x}" />"#);
    assert_has_warning(&diags, "attribute_quoted");
}

#[test]
fn attribute_quoted_custom_element() {
    let diags =
        analyze_with_diags(r#"<script>let x = $state('val');</script><my-el foo="{x}"></my-el>"#);
    assert_has_warning(&diags, "attribute_quoted");
}

#[test]
fn attribute_quoted_regular_element_no_warn() {
    // Non-custom regular elements must not get attribute_quoted.
    let diags =
        analyze_with_diags(r#"<script>let x = $state('val');</script><div foo="{x}"></div>"#);
    assert!(!diags.iter().any(|d| d.kind.code() == "attribute_quoted"));
}

// ---------------------------------------------------------------------------
// attribute_global_event_reference
// ---------------------------------------------------------------------------

#[test]
fn attribute_global_event_reference_on_missing_binding() {
    let diags = analyze_with_diags(r#"<button onclick={onclick}></button>"#);
    assert_has_warning(&diags, "attribute_global_event_reference");
}

#[test]
fn attribute_global_event_reference_on_local_binding_no_warn() {
    let diags = analyze_with_diags(
        r#"<script>const onclick = () => {}</script><button onclick={onclick}></button>"#,
    );
    assert_no_warning(&diags, "attribute_global_event_reference");
}

// ---------------------------------------------------------------------------
// component_name_lowercase
// ---------------------------------------------------------------------------

#[test]
fn component_name_lowercase_on_unused_import() {
    let diags =
        analyze_with_diags(r#"<script>import widget from './widget.js';</script><widget />"#);
    assert_has_warning(&diags, "component_name_lowercase");
}

#[test]
fn component_name_lowercase_on_plain_html_element_no_warn() {
    let diags = analyze_with_diags(r#"<widget />"#);
    assert_no_warning(&diags, "component_name_lowercase");
}
