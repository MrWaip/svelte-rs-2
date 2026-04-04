use lightningcss::error::PrinterError;
use lightningcss::rules::CssRule;
use lightningcss::selector::{Component, Selector};
use lightningcss::stylesheet::{PrinterOptions, StyleSheet};
use lightningcss::values::ident::Ident;
use lightningcss::{visit_types, visitor::{Visit, Visitor, VisitTypes}};
use rustc_hash::FxHashSet;

use svelte_ast::{AstStore, Component as SvelteComponent, Fragment, Node};

use crate::css::css_component_hash;
use crate::types::data::{AnalysisData, CssAnalysis};
use crate::types::node_table::NodeBitSet;

/// Run the CSS analysis pass: compute hash, mark scoped elements, emit scoped CSS.
///
/// Called from `compile()` after `analyze_with_options` returns, so it receives
/// the already-created `AnalysisData` mutably.
pub fn analyze_css_pass<'a>(
    component: &SvelteComponent,
    mut stylesheet: StyleSheet<'a, 'a>,
    data: &mut AnalysisData,
) {
    let Some(css_block) = &component.css else {
        return;
    };
    let css_text = component.source_text(css_block.content_span);
    let hash = css_component_hash(css_text);

    // Phase 1: collect HTML tag names selected by CSS rules (read-only).
    let selected_tags = collect_type_selectors(&stylesheet);

    // Phase 2: transform stylesheet — append `.hash` to each scoped TypeSelector.
    let hash_static: &'static str = Box::leak(hash.clone().into_boxed_str());
    let mut scoper = ScopeSelectors { hash_class: hash_static };
    let css_output = if stylesheet.visit(&mut scoper).is_ok() {
        stylesheet
            .to_css(PrinterOptions::default())
            .ok()
            .map(|r| r.code)
    } else {
        None
    };

    // Phase 3: walk template, mark elements whose tag name is in selected_tags.
    let node_count = component.node_count();
    // node_count is u32 per the Component API
    let mut scoped = NodeBitSet::new(node_count);
    mark_scoped_elements(&component.fragment, &component.store, &selected_tags, &mut scoped);

    data.css = CssAnalysis {
        hash,
        scoped_elements: scoped,
        css_output,
    };
}

// ---------------------------------------------------------------------------
// Collect selected tag names (read-only, no visitor mutation)
// ---------------------------------------------------------------------------

fn collect_type_selectors(stylesheet: &StyleSheet<'_, '_>) -> FxHashSet<String> {
    let mut tags = FxHashSet::default();
    for rule in stylesheet.rules.0.iter() {
        if let CssRule::Style(style_rule) = rule {
            for selector in style_rule.selectors.0.iter() {
                // Skip selectors containing :global (handled in later slices).
                let has_global = has_global_component(selector);
                if has_global {
                    continue;
                }
                for component in selector.iter_raw_match_order() {
                    if let Component::LocalName(name) = component {
                        tags.insert(name.name.to_string());
                    }
                }
            }
        }
    }
    tags
}

// ---------------------------------------------------------------------------
// Scope selectors (mutating visitor)
// ---------------------------------------------------------------------------

struct ScopeSelectors {
    /// The scoping class name, e.g. `"svelte-1a7i8ec"` (without the dot).
    /// `'static` allows the ident to coerce to any `Ident<'i>` via lifetime covariance.
    hash_class: &'static str,
}

impl<'i> Visitor<'i> for ScopeSelectors {
    type Error = PrinterError;

    fn visit_types(&self) -> VisitTypes {
        visit_types!(SELECTORS)
    }

    fn visit_selector(&mut self, selector: &mut Selector<'i>) -> Result<(), Self::Error> {
        let components: Vec<Component<'i>> =
            selector.iter_raw_match_order().cloned().collect();

        let has_local_name = components.iter().any(|c| matches!(c, Component::LocalName(_)));
        if !has_local_name {
            return Ok(());
        }

        // Leave :global selectors untouched (later slice).
        if has_global_component(selector) {
            return Ok(());
        }

        let mut result: Vec<Component<'i>> = Vec::with_capacity(components.len() + 1);
        for component in components {
            let is_local = matches!(component, Component::LocalName(_));
            result.push(component);
            if is_local {
                // Ident<'static> coerces to Ident<'i> because `'static: 'i`.
                let class_ident: Ident<'static> = Ident::from(self.hash_class);
                result.push(Component::Class(class_ident));
            }
        }

        *selector = Selector::from(result);
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn has_global_component(selector: &Selector<'_>) -> bool {
    selector.iter_raw_match_order().any(|c| {
        // :global pseudo-class debug output contains "global"
        let dbg = format!("{c:?}");
        dbg.contains("global") || dbg.contains("Global")
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
