use rustc_hash::FxHashSet;
use smallvec::SmallVec;
use svelte_css::{
    AtRule, CombinatorKind, ComplexSelector, RelativeSelector, SimpleSelector, StyleRule,
    StyleSheet, Visit,
};
use svelte_diagnostics::{Diagnostic, DiagnosticKind};
use svelte_span::GetSpan;

use svelte_ast::NodeId;

use crate::types::data::{ElementFacts, TemplateElementIndex};
use crate::AnalysisData;
use crate::types::node_table::NodeBitSet;

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

/// Determine which CSS selectors match template elements.
/// Marks matched selectors in `css.used_selectors`, marks every element
/// reached by a non-global selector chain in `css.scoped_elements`, and
/// emits `css_unused_selector` warnings for unmatched selectors.
pub(crate) fn prune_and_warn(
    stylesheet: &StyleSheet,
    css_source: &str,
    data: &mut AnalysisData,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let elements = &data.template_elements;
    let element_facts = &data.element_facts;
    let mut used = FxHashSet::default();
    let scoped = &mut data.css.scoped_elements;
    let mut pruner = PruneVisitor {
        elements,
        element_facts,
        used: &mut used,
        scoped,
        in_global_block: false,
    };
    pruner.visit_stylesheet(stylesheet);
    data.css.used_selectors = used;

    warn_unused(stylesheet, css_source, &data.css.used_selectors, diagnostics);
}

// ---------------------------------------------------------------------------
// Stylesheet pruning
// ---------------------------------------------------------------------------

struct PruneVisitor<'a, 'b> {
    elements: &'a TemplateElementIndex,
    element_facts: &'a ElementFacts,
    used: &'b mut FxHashSet<svelte_css::CssNodeId>,
    scoped: &'b mut NodeBitSet,
    in_global_block: bool,
}

impl Visit for PruneVisitor<'_, '_> {
    fn visit_at_rule(&mut self, node: &AtRule) {
        if node.name == "keyframes" {
            return;
        }
        svelte_css::visit::walk_at_rule(self, node);
    }

    fn visit_style_rule(&mut self, node: &StyleRule) {
        let was_global = self.in_global_block;
        self.in_global_block = was_global || node.is_lone_global_block();

        for complex in &node.prelude.children {
            if self.in_global_block || is_all_global(complex) {
                self.used.insert(complex.id);
                continue;
            }

            let selectors = truncate_trailing_globals(complex);
            if selectors.is_empty() {
                self.used.insert(complex.id);
                continue;
            }

            // Walk every candidate element matching the rightmost simple selector
            // and try the full chain. Mark every successful match as scoped — we
            // can't break early like the unused-selector pass because each
            // matching element needs the scope class.
            for elem_id in candidate_elements(self.elements, selectors.last().copied().unwrap()) {
                if apply_selector(
                    &selectors,
                    self.elements,
                    self.element_facts,
                    elem_id,
                    self.scoped,
                ) {
                    self.used.insert(complex.id);
                }
            }
        }

        svelte_css::visit::walk_style_rule(self, node);
        self.in_global_block = was_global;
    }
}

// ---------------------------------------------------------------------------
// Selector matching
// ---------------------------------------------------------------------------

/// Discard trailing `:global(...)` / global-like selectors — they don't affect scoping.
fn truncate_trailing_globals(complex: &ComplexSelector) -> Vec<&RelativeSelector> {
    let i = complex
        .children
        .iter()
        .rposition(|rel| !is_global_relative(rel))
        .map_or(0, |pos| pos + 1);
    complex.children[..i].iter().collect()
}

/// Check if every relative selector in a complex selector is global.
fn is_all_global(complex: &ComplexSelector) -> bool {
    complex.children.iter().all(is_global_relative)
}

/// Check if a relative selector is global (`:global(...)` or `:global` block).
fn is_global_relative(rel: &RelativeSelector) -> bool {
    rel.selectors.iter().all(|s| match s {
        SimpleSelector::Global { .. } => true,
        SimpleSelector::PseudoElement(_) => true,
        SimpleSelector::PseudoClass(pc) => is_unscoped_pseudo_class(pc),
        _ => false,
    }) && rel
        .selectors
        .iter()
        .any(|s| matches!(s, SimpleSelector::Global { .. }))
}

/// Pseudo-classes that don't contribute to scoping.
fn is_unscoped_pseudo_class(pc: &svelte_css::PseudoClassSelector) -> bool {
    let name = pc.name.as_str();
    // :has, :is, :where can be scoped depending on their args
    if name == "has" || name == "is" || name == "where" || name == "not" {
        return false;
    }
    true
}

/// Try matching the selector chain against an element, working backward.
/// On a successful match, marks `elem_id` (and any ancestor element matched
/// through a combinator) in `scoped`.
fn apply_selector(
    selectors: &[&RelativeSelector],
    elements: &TemplateElementIndex,
    element_facts: &ElementFacts,
    elem_id: NodeId,
    scoped: &mut NodeBitSet,
) -> bool {
    if selectors.is_empty() {
        return false;
    }

    // Start from the rightmost selector (backward matching)
    let last_idx = selectors.len() - 1;
    let last_sel = selectors[last_idx];

    if !relative_selector_matches(last_sel, elements, element_facts, elem_id) {
        return false;
    }

    if last_idx == 0 {
        // Only one selector and it matched
        scoped.insert(elem_id);
        return true;
    }

    // Need to match remaining selectors via combinators
    let rest = &selectors[..last_idx];
    if apply_combinator(rest, last_sel, elements, element_facts, elem_id, scoped) {
        scoped.insert(elem_id);
        return true;
    }
    false
}

/// Apply the combinator from `current_sel` to traverse the template tree
/// and try matching the remaining selectors.
fn apply_combinator(
    remaining: &[&RelativeSelector],
    current_sel: &RelativeSelector,
    elements: &TemplateElementIndex,
    element_facts: &ElementFacts,
    elem_id: NodeId,
    scoped: &mut NodeBitSet,
) -> bool {
    let combinator = match &current_sel.combinator {
        Some(c) => c.kind,
        None => {
            // No combinator on the rightmost selector means implicit descendant
            // when there are remaining selectors
            CombinatorKind::Descendant
        }
    };

    match combinator {
        CombinatorKind::Descendant => {
            // Check all ancestors
            let mut current = elements.parent_element(elem_id);
            while let Some(parent_id) = current {
                if apply_selector(remaining, elements, element_facts, parent_id, scoped) {
                    return true;
                }
                current = elements.parent_element(parent_id);
            }
            // No ancestor matched — check if all remaining selectors are global
            all_global(remaining)
        }
        CombinatorKind::Child => {
            // Check direct parent only
            if let Some(parent_id) = elements.parent_element(elem_id) {
                if apply_selector(remaining, elements, element_facts, parent_id, scoped) {
                    return true;
                }
            }
            // No parent or parent didn't match — check if all remaining are global
            all_global(remaining)
        }
        CombinatorKind::NextSibling => {
            if let Some(prev_id) = elements.previous_sibling(elem_id) {
                if apply_selector(remaining, elements, element_facts, prev_id, scoped) {
                    return true;
                }
            }
            all_global(remaining)
        }
        CombinatorKind::SubsequentSibling => {
            for prev_id in elements.previous_siblings(elem_id) {
                if apply_selector(remaining, elements, element_facts, prev_id, scoped) {
                    return true;
                }
            }
            all_global(remaining)
        }
        CombinatorKind::Column => {
            // Column combinator — conservative match
            true
        }
    }
}

/// Check if all remaining selectors are global (no matching needed).
fn all_global(selectors: &[&RelativeSelector]) -> bool {
    selectors.iter().all(|s| is_global_relative(s))
}

/// Check if a relative selector matches a specific template element.
fn relative_selector_matches(
    selector: &RelativeSelector,
    elements: &TemplateElementIndex,
    element_facts: &ElementFacts,
    elem_id: NodeId,
) -> bool {
    for simple in &selector.selectors {
        match simple {
            SimpleSelector::Type { name, .. } => {
                let Some(tag_name) = elements.tag_name(elem_id) else {
                    return false;
                };
                if name.as_str() != "*" && !tag_name.eq_ignore_ascii_case(name.as_str()) {
                    return false;
                }
            }
            SimpleSelector::Class { name, .. } => {
                if !element_has_class(element_facts, elem_id, name.as_str()) {
                    return false;
                }
            }
            SimpleSelector::Id { name, .. } => {
                if !element_has_id(element_facts, elem_id, name.as_str()) {
                    return false;
                }
            }
            SimpleSelector::Global {
                args: Some(args), ..
            } => {
                // :global(...) — match inner selectors against the element
                for complex in &args.children {
                    let inner: Vec<&RelativeSelector> = complex.children.iter().collect();
                    // Check the last selector of the :global() args
                    if !inner.is_empty() {
                        let last = inner.last().unwrap();
                        if relative_selector_matches(last, elements, element_facts, elem_id) {
                            return true;
                        }
                    }
                }
                return false;
            }
            SimpleSelector::Global { args: None, .. } => {
                // `:global` block form — always matches
                return true;
            }
            SimpleSelector::PseudoClass(pc) => {
                // :host and :root never match component elements
                if pc.name.as_str() == "host" || pc.name.as_str() == "root" {
                    return false;
                }
                // Other pseudo-classes: conservative match (assume they could match)
            }
            SimpleSelector::Attribute(_) => {
                // Conservative: attribute selectors always match
                // (full attribute value matching is a future slice)
            }
            SimpleSelector::PseudoElement(_)
            | SimpleSelector::Nesting(_)
            | SimpleSelector::Nth(_)
            | SimpleSelector::Percentage(_) => {
                // Conservative match
            }
        }
    }

    true
}

// ---------------------------------------------------------------------------
// Attribute matching helpers
// ---------------------------------------------------------------------------

/// Check if an element has a class that matches `name`.
fn element_has_class(element_facts: &ElementFacts, elem_id: NodeId, name: &str) -> bool {
    element_facts.has_static_class(elem_id, name) || element_facts.may_match_class(elem_id)
}

fn element_has_id(element_facts: &ElementFacts, elem_id: NodeId, expected: &str) -> bool {
    element_facts.static_id(elem_id) == Some(expected) || element_facts.may_match_id(elem_id)
}

fn candidate_elements(
    elements: &TemplateElementIndex,
    selector: &RelativeSelector,
) -> SmallVec<[NodeId; 8]> {
    for simple in &selector.selectors {
        match simple {
            SimpleSelector::Id { name, .. } => {
                return elements.id_candidates(name.as_str()).collect()
            }
            SimpleSelector::Class { name, .. } => {
                return elements.class_candidates(name.as_str()).collect();
            }
            SimpleSelector::Type { name, .. } if name.as_str() != "*" => {
                let direct = elements.elements_with_tag(name.as_str());
                if !direct.is_empty() {
                    return SmallVec::from_slice(direct);
                }
                if name.as_str().bytes().any(|byte| byte.is_ascii_uppercase()) {
                    return SmallVec::from_slice(elements.all_elements());
                }
                return SmallVec::new();
            }
            _ => {}
        }
    }

    SmallVec::from_slice(elements.all_elements())
}

// ---------------------------------------------------------------------------
// Unused selector warnings
// ---------------------------------------------------------------------------

/// Walk the stylesheet and emit warnings for unused selectors.
fn warn_unused(
    stylesheet: &StyleSheet,
    css_source: &str,
    used: &FxHashSet<svelte_css::CssNodeId>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let mut warner = UnusedWarner {
        css_source,
        used,
        diagnostics,
        in_keyframes: false,
        in_global_block: false,
    };
    warner.visit_stylesheet(stylesheet);
}

struct UnusedWarner<'a> {
    css_source: &'a str,
    used: &'a FxHashSet<svelte_css::CssNodeId>,
    diagnostics: &'a mut Vec<Diagnostic>,
    in_keyframes: bool,
    in_global_block: bool,
}

impl Visit for UnusedWarner<'_> {
    fn visit_at_rule(&mut self, node: &AtRule) {
        if node.name == "keyframes" {
            self.in_keyframes = true;
            svelte_css::visit::walk_at_rule(self, node);
            self.in_keyframes = false;
        } else {
            svelte_css::visit::walk_at_rule(self, node);
        }
    }

    fn visit_style_rule(&mut self, node: &StyleRule) {
        let was_global = self.in_global_block;
        if node.is_lone_global_block() {
            self.in_global_block = true;
        }
        svelte_css::visit::walk_style_rule(self, node);
        self.in_global_block = was_global;
    }

    fn visit_complex_selector(&mut self, node: &ComplexSelector) {
        if self.in_keyframes || self.in_global_block {
            return;
        }
        if !self.used.contains(&node.id) {
            let span = node.span();
            let selector_text = &self.css_source
                [span.start as usize..(span.end as usize).min(self.css_source.len())];
            self.diagnostics.push(Diagnostic::warning(
                DiagnosticKind::CssUnusedSelector {
                    name: selector_text.to_string(),
                },
                span,
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CssAnalysis;
    use svelte_diagnostics::DiagnosticKind;

    /// Parse a Svelte source with CSS, run the prune pass, and return diagnostic kinds.
    fn prune_diagnostics(source: &str) -> Vec<DiagnosticKind> {
        let alloc = oxc_allocator::Allocator::default();
        let (component, _js_result, parse_diags) = svelte_parser::parse_with_js(&alloc, source);
        assert!(
            parse_diags.is_empty(),
            "unexpected parse diagnostics: {parse_diags:?}"
        );
        let Some((stylesheet, css_diags)) = svelte_parser::parse_css_block(&component) else {
            panic!("no <style> block found in source");
        };
        assert!(
            css_diags.is_empty(),
            "unexpected CSS parse diagnostics: {css_diags:?}"
        );
        let css_block = component.css.as_ref().unwrap();
        let css_text = component.source_text(css_block.content_span);
        let (mut data, _parsed, analyze_diags) = crate::analyze(&component, _js_result);
        assert!(
            analyze_diags.is_empty(),
            "unexpected analyze diagnostics: {analyze_diags:?}"
        );
        data.css = CssAnalysis {
            hash: String::new(),
            scoped_elements: crate::types::node_table::NodeBitSet::new(component.node_count()),
            inject_styles: false,
            keyframes: Vec::new(),
            used_selectors: FxHashSet::default(),
        };
        let mut diagnostics = Vec::new();
        prune_and_warn(&stylesheet, css_text, &mut data, &mut diagnostics);
        diagnostics.into_iter().map(|d| d.kind).collect()
    }

    fn assert_unused(source: &str, expected_selector: &str) {
        let diags = prune_diagnostics(source);
        assert!(
            diags.iter().any(|k| matches!(k, DiagnosticKind::CssUnusedSelector { name } if name == expected_selector)),
            "Expected unused selector \"{expected_selector}\"\nGot: {diags:?}"
        );
    }

    fn assert_all_used(source: &str) {
        let diags = prune_diagnostics(source);
        assert!(
            diags.is_empty(),
            "Expected no unused selectors, got: {diags:?}"
        );
    }

    // Type selector matching

    #[test]
    fn type_selector_matches_element() {
        assert_all_used("<p>hello</p>\n<style>p { color: red; }</style>");
    }

    #[test]
    fn type_selector_no_match() {
        assert_unused("<p>hello</p>\n<style>h1 { font-size: 2em; }</style>", "h1");
    }

    #[test]
    fn type_selector_case_insensitive_css() {
        // CSS type selectors match case-insensitively against element names
        assert_all_used("<div>hello</div>\n<style>DIV { color: red; }</style>");
    }

    #[test]
    fn universal_selector_always_matches() {
        assert_all_used("<p>hello</p>\n<style>* { margin: 0; }</style>");
    }

    #[test]
    fn multiple_selectors_mixed() {
        let source = "<p>hello</p>\n<style>p { color: red; } h1 { font-size: 2em; }</style>";
        let diags = prune_diagnostics(source);
        assert_eq!(diags.len(), 1);
        assert!(matches!(&diags[0], DiagnosticKind::CssUnusedSelector { name } if name == "h1"));
    }

    // Class selector matching

    #[test]
    fn class_selector_static_match() {
        assert_all_used("<p class=\"foo bar\">hello</p>\n<style>.foo { color: red; }</style>");
    }

    #[test]
    fn class_selector_no_match() {
        assert_unused(
            "<p class=\"foo\">hello</p>\n<style>.bar { color: red; }</style>",
            ".bar",
        );
    }

    #[test]
    fn class_directive_match() {
        assert_all_used(
            "<script>let active = true;</script>\n<p class:active>hello</p>\n<style>.active { color: red; }</style>",
        );
    }

    // ID selector matching

    #[test]
    fn id_selector_match() {
        assert_all_used("<p id=\"main\">hello</p>\n<style>#main { color: red; }</style>");
    }

    #[test]
    fn id_selector_no_match() {
        assert_unused(
            "<p id=\"main\">hello</p>\n<style>#header { color: red; }</style>",
            "#header",
        );
    }

    // Global selectors

    #[test]
    fn global_functional_always_used() {
        assert_all_used("<p>hello</p>\n<style>:global(.external) { color: red; }</style>");
    }

    #[test]
    fn global_block_always_used() {
        assert_all_used("<p>hello</p>\n<style>:global { .external { color: red; } }</style>");
    }

    // Descendant combinator

    #[test]
    fn descendant_combinator_match() {
        assert_all_used("<div><p>hello</p></div>\n<style>div p { color: red; }</style>");
    }

    #[test]
    fn descendant_combinator_deep_match() {
        assert_all_used(
            "<div><span><p>hello</p></span></div>\n<style>div p { color: red; }</style>",
        );
    }

    #[test]
    fn descendant_combinator_no_match() {
        assert_unused(
            "<div><p>hello</p></div>\n<style>span p { color: red; }</style>",
            "span p",
        );
    }

    // Child combinator

    #[test]
    fn child_combinator_direct_match() {
        assert_all_used("<div><p>hello</p></div>\n<style>div > p { color: red; }</style>");
    }

    #[test]
    fn child_combinator_indirect_no_match() {
        assert_unused(
            "<div><span><p>hello</p></span></div>\n<style>div > p { color: red; }</style>",
            "div > p",
        );
    }

    #[test]
    fn adjacent_sibling_combinator_match() {
        assert_all_used("<div></div><p>hello</p>\n<style>div + p { color: red; }</style>");
    }

    #[test]
    fn adjacent_sibling_combinator_no_match() {
        assert_unused(
            "<div></div><span></span><p>hello</p>\n<style>div + p { color: red; }</style>",
            "div + p",
        );
    }

    #[test]
    fn general_sibling_combinator_match() {
        assert_all_used(
            "<div></div><span></span><p>hello</p>\n<style>div ~ p { color: red; }</style>",
        );
    }

    #[test]
    fn general_sibling_combinator_no_match() {
        assert_unused(
            "<section></section><span></span><p>hello</p>\n<style>div ~ p { color: red; }</style>",
            "div ~ p",
        );
    }

    // No template elements

    #[test]
    fn no_elements_all_unused() {
        assert_unused("{\"just text\"}\n<style>p { color: red; }</style>", "p");
    }

    // @keyframes are skipped

    #[test]
    fn keyframes_not_warned() {
        assert_all_used(
            "<p>hello</p>\n<style>p { color: red; } @keyframes fade { from { opacity: 1; } to { opacity: 0; } }</style>",
        );
    }

    // Inside @media

    #[test]
    fn media_query_unused_selector() {
        assert_unused(
            "<p>hello</p>\n<style>@media (max-width: 600px) { h1 { font-size: 1em; } }</style>",
            "h1",
        );
    }

    #[test]
    fn media_query_used_selector() {
        assert_all_used(
            "<p>hello</p>\n<style>@media (max-width: 600px) { p { font-size: 1em; } }</style>",
        );
    }

    // Spread attribute (conservative)

    #[test]
    fn spread_attribute_conservative_class_match() {
        assert_all_used(
            "<script>let props = {};</script>\n<p {...props}>hello</p>\n<style>.anything { color: red; }</style>",
        );
    }

    // Nested elements

    #[test]
    fn nested_element_match() {
        assert_all_used("<ul><li>item</li></ul>\n<style>li { color: red; }</style>");
    }

    #[test]
    fn deeply_nested_descendant() {
        assert_all_used(
            "<div><section><article><p>deep</p></article></section></div>\n<style>div p { color: red; }</style>",
        );
    }
}
