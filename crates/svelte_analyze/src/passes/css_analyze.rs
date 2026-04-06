use compact_str::CompactString;
use rustc_hash::FxHashSet;
use svelte_css::{
    AtRule, ComplexSelector, RelativeSelector, SimpleSelector, StyleRule, StyleSheet, Visit,
};

use svelte_ast::{AstStore, Component as SvelteComponent, Fragment, Node};

use crate::css::css_component_hash;
use crate::types::data::{AnalysisData, CssAnalysis};
use crate::types::node_table::NodeBitSet;

/// Classify the CSS block: compute hash, mark scoped template elements, set inject flag.
///
/// Does NOT transform or serialize CSS — call `svelte_transform_css::transform_css` for that.
pub fn analyze_css_pass(
    component: &SvelteComponent,
    stylesheet: &StyleSheet,
    inject_styles: bool,
    data: &mut AnalysisData,
) {
    let Some(css_block) = &component.css else {
        return;
    };
    let css_text = component.source_text(css_block.content_span);
    let hash = css_component_hash(css_text);

    // Phase 1: collect HTML tag names selected by CSS rules (read-only).
    let selected_tags = collect_type_selectors(stylesheet);

    // Phase 1b: collect locally-scoped @keyframes names.
    let keyframes = collect_keyframe_names(stylesheet, css_text);

    // Phase 2: walk template, mark elements whose tag name is in selected_tags.
    let node_count = component.node_count();
    let mut scoped = NodeBitSet::new(node_count);
    mark_scoped_elements(&component.fragment, &component.store, &selected_tags, &mut scoped);

    data.css = CssAnalysis {
        hash,
        scoped_elements: scoped,
        inject_styles,
        keyframes,
    };
}

// ---------------------------------------------------------------------------
// Collect selected tag names (read-only, no visitor mutation)
// ---------------------------------------------------------------------------

fn collect_type_selectors(stylesheet: &StyleSheet) -> FxHashSet<String> {
    let mut collector = TypeSelectorCollector {
        tags: FxHashSet::default(),
    };
    collector.visit_stylesheet(stylesheet);
    collector.tags
}

struct TypeSelectorCollector {
    tags: FxHashSet<String>,
}

impl Visit for TypeSelectorCollector {
    fn visit_style_rule(&mut self, node: &StyleRule) {
        if node.is_lone_global_block() {
            return;
        }
        svelte_css::visit::walk_style_rule(self, node);
    }

    fn visit_complex_selector(&mut self, node: &ComplexSelector) {
        if has_global_selector(node) {
            return;
        }
        svelte_css::visit::walk_complex_selector(self, node);
    }

    fn visit_relative_selector(&mut self, node: &RelativeSelector) {
        for sel in &node.selectors {
            if let SimpleSelector::Type { name, .. } = sel {
                self.tags.insert(name.to_string());
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Collect @keyframes names (read-only)
// ---------------------------------------------------------------------------

fn collect_keyframe_names(stylesheet: &StyleSheet, source: &str) -> Vec<CompactString> {
    let mut collector = KeyframeCollector {
        names: Vec::new(),
        source,
    };
    collector.visit_stylesheet(stylesheet);
    collector.names
}

struct KeyframeCollector<'a> {
    names: Vec<CompactString>,
    source: &'a str,
}

impl Visit for KeyframeCollector<'_> {
    fn visit_style_rule(&mut self, node: &StyleRule) {
        // Skip `:global { ... }` blocks — their keyframes are unscoped.
        if node.is_lone_global_block() {
            return;
        }
        svelte_css::visit::walk_style_rule(self, node);
    }

    fn visit_at_rule(&mut self, node: &AtRule) {
        if node.name == "keyframes" {
            let prelude = node.prelude.source_text(self.source).trim();
            if !prelude.starts_with("-global-") {
                self.names.push(CompactString::new(prelude));
            }
        }
        // Still recurse into the block for nested @keyframes (unlikely but correct)
        svelte_css::visit::walk_at_rule(self, node);
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn has_global_selector(complex: &ComplexSelector) -> bool {
    complex.children.iter().any(|rel| {
        rel.selectors
            .iter()
            .any(|s| matches!(s, SimpleSelector::Global { .. }))
    })
}

// ---------------------------------------------------------------------------
// Mark scoped elements in the template
// ---------------------------------------------------------------------------

fn mark_scoped_elements(
    fragment: &Fragment,
    store: &AstStore,
    selected_tags: &FxHashSet<String>,
    scoped: &mut NodeBitSet,
) {
    for &id in fragment.nodes.iter() {
        match store.get(id) {
            Node::Element(el) => {
                if selected_tags.contains(&el.name) {
                    scoped.insert(el.id);
                }
                mark_scoped_elements(&el.fragment, store, selected_tags, scoped);
            }
            Node::IfBlock(b) => {
                mark_scoped_elements(&b.consequent, store, selected_tags, scoped);
                if let Some(alt) = &b.alternate {
                    mark_scoped_elements(alt, store, selected_tags, scoped);
                }
            }
            Node::EachBlock(b) => {
                mark_scoped_elements(&b.body, store, selected_tags, scoped);
                if let Some(fb) = &b.fallback {
                    mark_scoped_elements(fb, store, selected_tags, scoped);
                }
            }
            Node::SnippetBlock(b) => {
                mark_scoped_elements(&b.body, store, selected_tags, scoped);
            }
            Node::KeyBlock(b) => {
                mark_scoped_elements(&b.fragment, store, selected_tags, scoped);
            }
            Node::AwaitBlock(b) => {
                if let Some(f) = &b.pending {
                    mark_scoped_elements(f, store, selected_tags, scoped);
                }
                if let Some(f) = &b.then {
                    mark_scoped_elements(f, store, selected_tags, scoped);
                }
                if let Some(f) = &b.catch {
                    mark_scoped_elements(f, store, selected_tags, scoped);
                }
            }
            _ => {}
        }
    }
}
