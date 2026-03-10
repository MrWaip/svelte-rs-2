mod content_types;
mod data;
mod elseif;
mod known_values;
mod lower;
mod mutations;
mod parse_js;
mod reactivity;
mod runes;
mod symbols;
mod validate;

pub use data::{
    AnalysisData, ConcatPart, ContentType, FragmentItem, FragmentKey, LoweredFragment,
    SymbolId, SymbolInfo,
};

use svelte_ast::Component;
use svelte_diagnostics::Diagnostic;

/// Run all analysis passes over a parsed component.
///
/// Pass order:
/// 1. parse_js    — parse JS expressions + script block
/// 2. symbols     — extract symbol declarations
/// 3. runes       — detect rune symbols
/// 4. mutations   — detect mutated runes (script assignments + bind directives)
/// 5. lower       — trim whitespace, group text+expressions
/// 6. reactivity  — mark dynamic nodes and attributes
/// 7. content_types — classify fragment content
/// 8. validate    — semantic checks
pub fn analyze(component: &Component) -> (AnalysisData, Vec<Diagnostic>) {
    let mut data = AnalysisData::new();
    let mut diags = Vec::new();

    parse_js::parse_js(component, &mut data, &mut diags);
    symbols::collect_symbols(&mut data);
    runes::detect_runes(&mut data);
    known_values::collect_known_values(component, &mut data);
    mutations::detect_mutations(component, &mut data);
    lower::lower(component, &mut data);
    reactivity::mark_reactivity(component, &mut data);
    content_types::classify_content(component, &mut data);
    elseif::detect_elseif(component, &mut data);
    validate::validate(component, &data, &mut diags);

    (data, diags)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use svelte_ast::{Component, Element, EachBlock, Fragment, IfBlock, Node, NodeId};
    use svelte_parser::Parser;

    use super::*;

    // -----------------------------------------------------------------------
    // Finders — identify nodes by source text (span-based)
    // -----------------------------------------------------------------------

    fn find_expr_tag(fragment: &Fragment, component: &Component, target: &str) -> Option<NodeId> {
        for node in &fragment.nodes {
            match node {
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
                _ => {}
            }
        }
        None
    }

    fn find_element<'a>(fragment: &'a Fragment, tag_name: &str) -> Option<&'a Element> {
        for node in &fragment.nodes {
            match node {
                Node::Element(el) if el.name == tag_name => return Some(el),
                Node::Element(el) => {
                    if let Some(found) = find_element(&el.fragment, tag_name) {
                        return Some(found);
                    }
                }
                Node::IfBlock(b) => {
                    if let Some(found) = find_element(&b.consequent, tag_name) {
                        return Some(found);
                    }
                    if let Some(alt) = &b.alternate {
                        if let Some(found) = find_element(alt, tag_name) {
                            return Some(found);
                        }
                    }
                }
                Node::EachBlock(b) => {
                    if let Some(found) = find_element(&b.body, tag_name) {
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
        component: &Component,
        test_text: &str,
    ) -> Option<&'a IfBlock> {
        for node in &fragment.nodes {
            if let Node::IfBlock(b) = node {
                if component.source_text(b.test_span) == test_text {
                    return Some(b);
                }
            }
        }
        None
    }

    fn find_each_block<'a>(
        fragment: &'a Fragment,
        component: &Component,
        expr_text: &str,
    ) -> Option<&'a EachBlock> {
        for node in &fragment.nodes {
            if let Node::EachBlock(b) = node {
                if component.source_text(b.expression_span) == expr_text {
                    return Some(b);
                }
            }
        }
        None
    }

    // -----------------------------------------------------------------------
    // Assertion helpers
    // -----------------------------------------------------------------------

    fn analyze_source(source: &str) -> (Component, AnalysisData) {
        let component = Parser::new(source).parse().expect("parse failed");
        let (data, diags) = analyze(&component);
        assert!(diags.is_empty(), "unexpected diagnostics: {diags:?}");
        (component, data)
    }

    fn assert_root_content_type(data: &AnalysisData, expected: ContentType) {
        let actual = data.content_types.get(&FragmentKey::Root).expect("no root content type");
        assert_eq!(*actual, expected);
    }

    fn assert_symbol(data: &AnalysisData, name: &str) {
        assert!(
            data.symbol_by_name.contains_key(name),
            "expected symbol '{name}' — got {:?}",
            data.symbols.iter().map(|s| &s.name).collect::<Vec<_>>(),
        );
    }

    fn assert_is_rune(data: &AnalysisData, name: &str) {
        let id = data
            .symbol_by_name
            .get(name)
            .unwrap_or_else(|| panic!("no symbol '{name}'"));
        assert!(data.runes.contains_key(id), "expected '{name}' to be a rune");
    }

    fn assert_dynamic_tag(data: &AnalysisData, component: &Component, expr_text: &str) {
        let id = find_expr_tag(&component.fragment, component, expr_text)
            .unwrap_or_else(|| panic!("no ExpressionTag with source '{expr_text}'"));
        assert!(data.dynamic_nodes.contains(&id), "expected ExpressionTag '{expr_text}' to be dynamic");
    }

    fn assert_not_dynamic_tag(data: &AnalysisData, component: &Component, expr_text: &str) {
        let id = find_expr_tag(&component.fragment, component, expr_text)
            .unwrap_or_else(|| panic!("no ExpressionTag with source '{expr_text}'"));
        assert!(!data.dynamic_nodes.contains(&id), "expected ExpressionTag '{expr_text}' to NOT be dynamic");
    }

    fn assert_dynamic_if_block(data: &AnalysisData, component: &Component, test_text: &str) {
        let block = find_if_block(&component.fragment, component, test_text)
            .unwrap_or_else(|| panic!("no IfBlock with test '{test_text}'"));
        assert!(data.dynamic_nodes.contains(&block.id), "expected IfBlock '{test_text}' to be dynamic");
    }

    fn assert_dynamic_each(data: &AnalysisData, component: &Component, expr_text: &str) {
        let block = find_each_block(&component.fragment, component, expr_text)
            .unwrap_or_else(|| panic!("no EachBlock with expr '{expr_text}'"));
        assert!(data.dynamic_nodes.contains(&block.id), "expected EachBlock '{expr_text}' to be dynamic");
    }

    /// Assert that attribute at `attr_index` of `<tag_name>` is in `dynamic_attrs`.
    fn assert_dynamic_attr(
        data: &AnalysisData,
        component: &Component,
        tag_name: &str,
        attr_index: usize,
    ) {
        let el = find_element(&component.fragment, tag_name)
            .unwrap_or_else(|| panic!("no element <{tag_name}>"));
        assert!(
            data.dynamic_attrs.contains(&(el.id, attr_index)),
            "expected attr[{attr_index}] of <{tag_name}> to be dynamic",
        );
    }

    fn assert_node_needs_ref(data: &AnalysisData, component: &Component, tag_name: &str) {
        let el = find_element(&component.fragment, tag_name)
            .unwrap_or_else(|| panic!("no element <{tag_name}>"));
        assert!(data.node_needs_ref.contains(&el.id), "expected <{tag_name}> to need a ref");
    }

    fn assert_element_content_type(
        data: &AnalysisData,
        component: &Component,
        tag_name: &str,
        expected: ContentType,
    ) {
        let el = find_element(&component.fragment, tag_name)
            .unwrap_or_else(|| panic!("no element <{tag_name}>"));
        let actual = data.content_types.get(&FragmentKey::Element(el.id))
            .unwrap_or_else(|| panic!("no content type for <{tag_name}>"));
        assert_eq!(*actual, expected);
    }

    fn assert_consequent_content_type(
        data: &AnalysisData,
        component: &Component,
        test_text: &str,
        expected: ContentType,
    ) {
        let block = find_if_block(&component.fragment, component, test_text)
            .unwrap_or_else(|| panic!("no IfBlock with test '{test_text}'"));
        let actual = data.content_types.get(&FragmentKey::IfConsequent(block.id))
            .unwrap_or_else(|| panic!("no consequent content type for IfBlock '{test_text}'"));
        assert_eq!(*actual, expected);
    }

    fn assert_alternate_content_type(
        data: &AnalysisData,
        component: &Component,
        test_text: &str,
        expected: ContentType,
    ) {
        let block = find_if_block(&component.fragment, component, test_text)
            .unwrap_or_else(|| panic!("no IfBlock with test '{test_text}'"));
        let actual = data.content_types.get(&FragmentKey::IfAlternate(block.id))
            .unwrap_or_else(|| panic!("no alternate content type for IfBlock '{test_text}'"));
        assert_eq!(*actual, expected);
    }

    fn assert_lowered_item_count(data: &AnalysisData, key: FragmentKey, expected_count: usize) {
        let lf = data
            .lowered_fragments
            .get(&key)
            .unwrap_or_else(|| panic!("no lowered fragment for {:?}", key));
        assert_eq!(lf.items.len(), expected_count, "expected {expected_count} items in lowered fragment");
    }

    fn assert_item_is_text_concat(data: &AnalysisData, key: FragmentKey, index: usize) {
        let lf = data.lowered_fragments.get(&key).expect("no lowered fragment");
        assert!(
            matches!(lf.items.get(index), Some(FragmentItem::TextConcat { .. })),
            "expected item[{index}] to be TextConcat",
        );
    }

    // -----------------------------------------------------------------------
    // Tests
    // -----------------------------------------------------------------------

    #[test]
    fn empty_component() {
        let (_c, data) = analyze_source("");
        assert_root_content_type(&data, ContentType::Empty);
    }

    #[test]
    fn static_text_content() {
        let (_c, data) = analyze_source("Hello world");
        assert_root_content_type(&data, ContentType::StaticText);
    }

    #[test]
    fn single_element() {
        let (_c, data) = analyze_source("<div></div>");
        assert_root_content_type(&data, ContentType::SingleElement);
    }

    #[test]
    fn mixed_content() {
        let (_c, data) = analyze_source("<div></div><span></span>");
        assert_root_content_type(&data, ContentType::Mixed);
    }

    #[test]
    fn rune_detection() {
        let (c, data) = analyze_source(r#"<script>let count = $state(0);</script><p>{count}</p>"#);
        assert_symbol(&data, "count");
        assert_is_rune(&data, "count");
        assert_dynamic_tag(&data, &c, "count");
    }

    #[test]
    fn dynamic_text_content() {
        let (_c, data) = analyze_source(r#"<script>let count = $state(0);</script>{count}"#);
        assert_root_content_type(&data, ContentType::DynamicText);
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
            r#"<script>let show = $state(true);</script>{#if show}<p>hi</p>{/if}"#,
        );
        assert_symbol(&data, "show");
        assert_is_rune(&data, "show");
        assert_dynamic_if_block(&data, &c, "show");
    }

    // — new tests —

    #[test]
    fn dynamic_attr_and_needs_ref() {
        let (c, data) = analyze_source(
            r#"<script>let count = $state(0);</script><div class={count}></div>"#,
        );
        assert_symbol(&data, "count");
        assert_is_rune(&data, "count");
        assert_dynamic_attr(&data, &c, "div", 0);
        assert_node_needs_ref(&data, &c, "div");
    }

    #[test]
    fn whitespace_only_text_trimmed_at_boundaries() {
        // Leading and trailing whitespace-only text should not appear as items.
        let (_c, data) = analyze_source("\n  <div></div>\n  ");
        assert_lowered_item_count(&data, FragmentKey::Root, 1);
        assert_root_content_type(&data, ContentType::SingleElement);
    }

    #[test]
    fn multiple_lowered_groups() {
        // ExprTag, then Element, then ExprTag → three separate items.
        let (_c, data) = analyze_source("{a} <div></div> {b}");
        assert_lowered_item_count(&data, FragmentKey::Root, 3);
        assert_item_is_text_concat(&data, FragmentKey::Root, 0);
        assert_item_is_text_concat(&data, FragmentKey::Root, 2);
        assert_root_content_type(&data, ContentType::Mixed);
    }

    #[test]
    fn nested_dynamic_tag_in_element() {
        let (c, data) = analyze_source(
            r#"<script>let count = $state(0);</script><div>{count}</div>"#,
        );
        assert_dynamic_tag(&data, &c, "count");
        assert_element_content_type(&data, &c, "div", ContentType::DynamicText);
    }

    #[test]
    fn each_block_dynamic() {
        let (c, data) = analyze_source(
            r#"<script>let items = $state([]);</script>{#each items as item}<p>{item}</p>{/each}"#,
        );
        assert_symbol(&data, "items");
        assert_is_rune(&data, "items");
        assert_dynamic_each(&data, &c, "items");
    }

    #[test]
    fn if_block_alternate_content_type() {
        let (c, data) = analyze_source("{#if x}Hello{:else}<span></span>{/if}");
        assert_consequent_content_type(&data, &c, "x", ContentType::StaticText);
        assert_alternate_content_type(&data, &c, "x", ContentType::SingleElement);
    }
}
