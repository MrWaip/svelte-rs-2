mod bind_semantics;
mod content_types;
mod markers;
mod data;
mod element_flags;
mod elseif;
mod hoistable;
pub mod ident_gen;
mod known_values;
mod lower;
mod needs_var;
mod parse_js;
mod props;
mod reactivity;
mod resolve_references;
pub mod scope;
mod store_subscriptions;
mod validate;
pub(crate) mod walker;

pub use data::{
    AnalysisData, AwaitBindingData, ClassDirectiveInfo, ComponentPropInfo, ComponentPropKind,
    LoweredTextPart, ConstTagData, ContentStrategy, DebugTagData, ElementFlags,
    FragmentData, FragmentItem, FragmentKey, LoweredFragment, ParsedExprs, PropAnalysis, PropsAnalysis, SnippetData,
};
pub use ident_gen::IdentGen;
pub use scope::ComponentScoping;

use oxc_allocator::Allocator;
use svelte_ast::Component;
use svelte_diagnostics::Diagnostic;

/// Run all analysis passes over a parsed component.
///
/// `alloc` is a shared OXC allocator for storing parsed Expression ASTs.
/// The caller owns the allocator; the returned `ParsedExprs<'a>` borrows from it.
///
/// Pass order:
/// 1. parse_js      — parse JS expressions + script block (stores ASTs in ParsedExprs)
/// 2. build_scoping — build unified scope tree (script + template)
/// 3. resolve_references — resolve template refs to SymbolId, register mutations
/// 4. store_subscriptions — detect $store subscriptions
/// 5. known_values  — evaluate const declarations with literal initializers
/// 6. props         — analyze $props() destructuring
/// 7. lower         — trim whitespace, group text+expressions
/// 8. composite walk — reactivity + elseif + element flags + hoistable snippets
/// 9. classify_and_mark_dynamic — content types + fragment dynamism (single HashMap pass)
/// 10. needs_var    — compute elements needing DOM variable
/// 11. validate     — semantic checks
pub fn analyze<'a>(
    alloc: &'a Allocator,
    component: &Component,
) -> (AnalysisData, ParsedExprs<'a>, Vec<Diagnostic>) {
    analyze_with_options(alloc, component, false)
}

/// Analyze with compile options that affect analysis behavior.
pub fn analyze_with_options<'a>(
    alloc: &'a Allocator,
    component: &Component,
    custom_element: bool,
) -> (AnalysisData, ParsedExprs<'a>, Vec<Diagnostic>) {
    let mut data = AnalysisData::new();
    data.custom_element = custom_element;
    let mut parsed = ParsedExprs::new();
    let mut diags = Vec::new();

    parse_js::parse_js(alloc, component, &mut data, &mut parsed, &mut diags);

    let scoping_built = scope::build_scoping(component, &mut data);
    parse_js::register_arrow_scopes(component, &mut data, &parsed);
    data.import_syms = data.scoping.collect_import_syms();
    resolve_references::resolve_references(component, &mut data, scoping_built);
    store_subscriptions::detect_store_subscriptions(&mut data);
    known_values::collect_known_values(component, &mut data);
    props::analyze_props(&mut data);
    resolve_render_tag_prop_sources(&mut data);
    resolve_render_tag_dynamic(&mut data);
    lower::lower(component, &mut data);

    // Single composite walk: reactivity + elseif + element flags + hoistable snippets
    {
        let root = data.scoping.root_scope_id();
        let script_syms: rustc_hash::FxHashSet<crate::scope::SymbolId> = data
            .script
            .as_ref()
            .map(|s| {
                s.declarations.iter()
                    .filter_map(|d| data.scoping.find_binding(root, &d.name))
                    .collect()
            })
            .unwrap_or_default();
        let top_level_snippet_ids: rustc_hash::FxHashSet<svelte_ast::NodeId> = component
            .fragment
            .nodes
            .iter()
            .filter_map(|n| {
                if let svelte_ast::Node::SnippetBlock(b) = n {
                    Some(b.id)
                } else {
                    None
                }
            })
            .collect();
        let mut visitor = (
            reactivity::ReactivityVisitor,
            elseif::ElseifVisitor,
            element_flags::ElementFlagsVisitor::new(&component.source),
            hoistable::HoistableSnippetsVisitor::new(script_syms, top_level_snippet_ids),
            bind_semantics::BindSemanticsVisitor::new(&component.source),
        );
        walker::walk_template(&component.fragment, &mut data, root, &mut visitor);
    }

    // Classify fragments + mark which have dynamic children (single pass over lowered_fragments)
    debug_assert!(
        !data.fragments.lowered.is_empty() || component.fragment.nodes.is_empty(),
        "classify_and_mark_dynamic requires lowered_fragments (from lower pass)"
    );
    content_types::classify_and_mark_dynamic(&mut data);
    // Separate walker pass: needs_var depends on content_types (computed above)
    debug_assert!(
        !data.fragments.content_types.is_empty() || component.fragment.nodes.is_empty(),
        "needs_var requires content_types (from classify_and_mark_dynamic)"
    );
    {
        let root = data.scoping.root_scope_id();
        let mut visitor = needs_var::NeedsVarVisitor;
        walker::walk_template(&component.fragment, &mut data, root, &mut visitor);
    }
    validate::validate(component, &data, &mut diags);

    (data, parsed, diags)
}

/// Simplified analysis for standalone `.svelte.js`/`.svelte.ts` modules.
///
/// Only parses JS, builds scopes, and detects runes. No template, no props,
/// no fragment classification — modules are pure JS with rune transforms.
pub fn analyze_module(source: &str, is_ts: bool, dev: bool) -> (AnalysisData, Vec<Diagnostic>) {
    let _ = dev; // reserved for future dev-mode analysis (e.g. $inspect.trace labels)
    let mut data = AnalysisData::new();
    let mut diags = Vec::new();

    match svelte_js::analyze_script_with_scoping(source, 0, is_ts) {
        Ok((script_info, scoping)) => {
            data.scoping = scope::ComponentScoping::from_scoping(scoping);

            // Mark runes from script declarations
            for decl in &script_info.declarations {
                if let Some(rune_kind) = decl.is_rune {
                    let root = data.scoping.root_scope_id();
                    if let Some(sym_id) = data.scoping.find_binding(root, &decl.name) {
                        data.scoping.mark_rune(sym_id, rune_kind);
                    }
                }
            }
        }
        Err(errs) => diags.extend(errs),
    }

    (data, diags)
}

/// Resolve render tag argument identifiers to prop-source getter names.
/// Consumes `render_tag_arg_idents` (intermediate) and populates `render_tag_prop_sources`.
fn resolve_render_tag_prop_sources(data: &mut AnalysisData) {
    let root = data.scoping.root_scope_id();
    for (node_id, idents) in data.render_tag_arg_idents.drain() {
        let resolved = idents.into_iter().map(|opt_name| {
            opt_name.and_then(|name| {
                let sym = data.scoping.find_binding(root, &name)?;
                data.scoping.is_prop_source(sym).then_some(sym)
            })
        }).collect();
        data.render_tag_prop_sources.insert(node_id, resolved);
    }
}

/// Determine which render tags have dynamic callees and which are getter-type.
/// Must run after `resolve_render_tag_prop_sources` (which runs after `props`).
fn resolve_render_tag_dynamic(data: &mut AnalysisData) {
    // Collect all render tag IDs from arg_has_call (populated for every render tag).
    let all_ids: Vec<svelte_ast::NodeId> = data.render_tag_arg_has_call.keys().copied().collect();

    for node_id in all_ids {
        let is_dynamic = match data.render_tag_callee_sym.get(&node_id) {
            Some(&sym_id) => {
                let normal = data.scoping.is_normal_binding(sym_id);
                if !normal && (data.scoping.is_prop_source(sym_id) || data.scoping.is_snippet_param(sym_id)) {
                    data.render_tag_callee_is_getter.insert(node_id);
                }
                !normal
            }
            // No sym: non-Identifier callee or unresolved binding — always dynamic.
            None => true,
        };
        if is_dynamic {
            data.render_tag_dynamic.insert(node_id);
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use svelte_ast::{Component, EachBlock, Element, Fragment, IfBlock, Node, NodeId};
    use svelte_parser::Parser;

    use super::*;

    fn assert_content_strategy_variant(data: &AnalysisData, key: FragmentKey, variant: &str) {
        let actual = data.fragments.content_types.get(&key)
            .unwrap_or_else(|| panic!("no content strategy for {:?}", key));
        let actual_variant = match actual {
            ContentStrategy::Empty => "Empty",
            ContentStrategy::Static(_) => "Static",
            ContentStrategy::SingleElement(_) => "SingleElement",
            ContentStrategy::SingleBlock(_) => "SingleBlock",
            ContentStrategy::DynamicText => "DynamicText",
            ContentStrategy::Mixed { .. } => "Mixed",
        };
        assert_eq!(actual_variant, variant, "expected {:?} to be {}", key, variant);
    }

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
        let alloc = oxc_allocator::Allocator::default();
        let (component, _) = Parser::new(source).parse();
        let (data, _parsed, diags) = analyze(&alloc, &component);
        assert!(diags.is_empty(), "unexpected diagnostics: {diags:?}");
        (component, data)
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

    fn assert_element_content_type(
        data: &AnalysisData,
        component: &Component,
        tag_name: &str,
        expected: ContentStrategy,
    ) {
        let el = find_element(&component.fragment, tag_name)
            .unwrap_or_else(|| panic!("no element <{tag_name}>"));
        let actual = data
            .fragments
            .content_types
            .get(&FragmentKey::Element(el.id))
            .unwrap_or_else(|| panic!("no content type for <{tag_name}>"));
        assert_eq!(*actual, expected);
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

    fn assert_alternate_content_type(
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
            .get(&FragmentKey::IfAlternate(block.id))
            .unwrap_or_else(|| panic!("no alternate content type for IfBlock '{test_text}'"));
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
        assert_root_content_type(&data, ContentStrategy::Mixed { has_elements: true, has_blocks: false, has_text: false });
    }

    #[test]
    fn rune_detection() {
        let (c, data) = analyze_source(r#"<script>let count = $state(0); count = 1;</script><p>{count}</p>"#);
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
        let (c, data) =
            analyze_source(r#"<script>let show = $state(true); show = false;</script>{#if show}<p>hi</p>{/if}"#);
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
        assert_root_content_type(&data, ContentStrategy::Mixed { has_elements: true, has_blocks: false, has_text: true });
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
        let each_scope = data.scoping.node_scope(each_block.id).unwrap();
        let each_sym = data.scoping.find_binding(each_scope, "item").unwrap();
        // The each-block's `item` is NOT a rune (it's a block variable)
        assert!(!data.scoping.is_rune(each_sym));
        // And it's a different symbol than the root one
        assert_ne!(root_sym, each_sym);
    }

    #[test]
    fn each_block_shadowing_does_not_mutate_rune() {
        // `count = 99` inside each targets the each-block variable, not the rune
        let (_c, data) = analyze_source(
            r#"<script>let count = $state(0); let items = $state([]);</script>{#each items as count}{count = 99}{/each}"#,
        );
        assert_is_rune(&data, "count");
        let root = data.scoping.root_scope_id();
        let count_sym = data.scoping.find_binding(root, "count").expect("symbol 'count' not found");
        assert!(
            !data.scoping.is_mutated(count_sym),
            "rune 'count' should NOT be mutated — the assignment targets the each-block variable"
        );
    }

    #[test]
    fn each_block_index_is_dynamic() {
        let (c, data) = analyze_source(
            r#"<script>let items = $state([]);</script>{#each items as item, i}<p>{i}</p>{/each}"#,
        );
        assert_dynamic_tag(&data, &c, "i");
    }
}
