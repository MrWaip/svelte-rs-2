use lightningcss::error::PrinterError;
use lightningcss::rules::CssRule;
use lightningcss::selector::{Component, PseudoClass, Selector};
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
/// `alloc` is the same OXC allocator used for the stylesheet — the hash string is
/// allocated into it so it shares lifetime `'a` with the selector components.
pub fn analyze_css_pass<'a>(
    alloc: &'a oxc_allocator::Allocator,
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
    // The hash string is allocated into the same arena as the stylesheet source so
    // that injected Ident<'a> components have the correct lifetime — no Box::leak needed.
    let hash_str: &'a str = alloc.alloc_str(&hash);
    let mut scoper = ScopeSelectors { hash_class: hash_str };
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
                if has_global_component(selector) {
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

struct ScopeSelectors<'h> {
    /// The scoping class name, e.g. `"svelte-1a7i8ec"` (without the dot).
    /// Allocated into the stylesheet arena so `Ident<'h>` satisfies `'h: 'i`.
    hash_class: &'h str,
}

impl<'i, 'h: 'i> Visitor<'i> for ScopeSelectors<'h> {
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

        if has_global_component(selector) {
            return Ok(());
        }

        let mut result: Vec<Component<'i>> = Vec::with_capacity(components.len() + 1);
        for component in components {
            let is_local = matches!(component, Component::LocalName(_));
            result.push(component);
            if is_local {
                // Ident<'h> coerces to Ident<'i> because 'h: 'i and Ident is covariant.
                let class_ident: Ident<'h> = Ident::from(self.hash_class);
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
    selector.iter_raw_match_order().any(|c| match c {
        // CSS Modules :global() — only when ParserOptions::css_modules is set
        Component::NonTSPseudoClass(PseudoClass::Global { .. }) => true,
        // Bare :global or :global(...) parsed as unknown pseudo-class without CSS modules
        Component::NonTSPseudoClass(PseudoClass::Custom { name }) => name.as_ref() == "global",
        Component::NonTSPseudoClass(PseudoClass::CustomFunction { name, .. }) => {
            name.as_ref() == "global"
        }
        _ => false,
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
