use rustc_hash::FxHashSet;
use smallvec::SmallVec;
use svelte_ast::{Attribute, Component as SvelteComponent, Node, NodeId};
use svelte_css::{
    AtRule, CombinatorKind, ComplexSelector, RelativeSelector, SimpleSelector, StyleRule,
    StyleSheet, Visit,
};
use svelte_diagnostics::{Diagnostic, DiagnosticKind};
use svelte_span::GetSpan;

use crate::types::data::{ElementFacts, NamespaceKind, TemplateElementIndex};
use crate::types::node_table::NodeBitSet;
use crate::AnalysisData;

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

/// Determine which CSS selectors match template elements.
/// Marks matched selectors in `css.used_selectors`, marks every element
/// reached by a non-global selector chain in `css.scoped_elements`, and
/// emits `css_unused_selector` warnings for unmatched selectors.
pub(crate) fn prune_and_warn(
    component: &SvelteComponent,
    stylesheet: &StyleSheet,
    css_source: &str,
    data: &mut AnalysisData,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let elements = &data.template.template_elements;
    let element_facts = &data.elements.facts;
    let mut used = FxHashSet::default();
    let scoped = &mut data.output.css.scoped_elements;
    let mut pruner = PruneVisitor {
        component,
        css_source,
        elements,
        element_facts,
        used: &mut used,
        scoped,
        in_global_block: false,
    };
    pruner.visit_stylesheet(stylesheet);
    data.output.css.used_selectors = used;

    warn_unused(
        stylesheet,
        css_source,
        &data.output.css.used_selectors,
        diagnostics,
    );
}

// ---------------------------------------------------------------------------
// Stylesheet pruning
// ---------------------------------------------------------------------------

struct PruneVisitor<'a, 'b> {
    component: &'a SvelteComponent,
    css_source: &'a str,
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
                    self.component,
                    self.css_source,
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
    if let Some(idx) = complex.children.iter().position(is_bare_global_relative) {
        return complex.children[..idx].iter().collect();
    }

    let i = complex
        .children
        .iter()
        .rposition(|rel| !is_global_relative(rel))
        .map_or(0, |pos| pos + 1);
    complex.children[..i].iter().collect()
}

fn is_bare_global_relative(rel: &RelativeSelector) -> bool {
    rel.selectors
        .first()
        .is_some_and(|sel| matches!(sel, SimpleSelector::Global { args: None, .. }))
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
    component: &SvelteComponent,
    css_source: &str,
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

    if !relative_selector_matches(
        last_sel,
        component,
        css_source,
        elements,
        element_facts,
        elem_id,
    ) {
        return false;
    }

    if last_idx == 0 {
        // Only one selector and it matched
        scoped.insert(elem_id);
        return true;
    }

    // Need to match remaining selectors via combinators
    let rest = &selectors[..last_idx];
    if apply_combinator(
        rest,
        last_sel,
        component,
        css_source,
        elements,
        element_facts,
        elem_id,
        scoped,
    ) {
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
    component: &SvelteComponent,
    css_source: &str,
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
                if apply_selector(
                    remaining,
                    component,
                    css_source,
                    elements,
                    element_facts,
                    parent_id,
                    scoped,
                ) {
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
                if apply_selector(
                    remaining,
                    component,
                    css_source,
                    elements,
                    element_facts,
                    parent_id,
                    scoped,
                ) {
                    return true;
                }
            }
            // No parent or parent didn't match — check if all remaining are global
            all_global(remaining)
        }
        CombinatorKind::NextSibling => {
            if let Some(prev_id) = elements.previous_sibling(elem_id) {
                if apply_selector(
                    remaining,
                    component,
                    css_source,
                    elements,
                    element_facts,
                    prev_id,
                    scoped,
                ) {
                    return true;
                }
            }
            all_global(remaining)
        }
        CombinatorKind::SubsequentSibling => {
            for prev_id in elements.previous_siblings(elem_id) {
                if apply_selector(
                    remaining,
                    component,
                    css_source,
                    elements,
                    element_facts,
                    prev_id,
                    scoped,
                ) {
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
    component: &SvelteComponent,
    css_source: &str,
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
                        if relative_selector_matches(
                            last,
                            component,
                            css_source,
                            elements,
                            element_facts,
                            elem_id,
                        ) {
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
            SimpleSelector::Attribute(attr) => {
                if !element_matches_attribute_selector(
                    component,
                    css_source,
                    element_facts,
                    elem_id,
                    attr,
                ) {
                    return false;
                }
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

const HTML_CASE_INSENSITIVE_ATTRIBUTES: &[&str] = &[
    "accept-charset",
    "autocapitalize",
    "autocomplete",
    "behavior",
    "charset",
    "crossorigin",
    "decoding",
    "dir",
    "direction",
    "draggable",
    "enctype",
    "enterkeyhint",
    "fetchpriority",
    "formenctype",
    "formmethod",
    "formtarget",
    "hidden",
    "http-equiv",
    "inputmode",
    "kind",
    "loading",
    "method",
    "preload",
    "referrerpolicy",
    "rel",
    "rev",
    "role",
    "rules",
    "scope",
    "shape",
    "spellcheck",
    "target",
    "translate",
    "type",
    "valign",
    "wrap",
];

/// Check if an element has a class that matches `name`.
fn element_has_class(element_facts: &ElementFacts, elem_id: NodeId, name: &str) -> bool {
    element_facts.has_static_class(elem_id, name) || element_facts.may_match_class(elem_id)
}

fn element_has_id(element_facts: &ElementFacts, elem_id: NodeId, expected: &str) -> bool {
    element_facts.static_id(elem_id) == Some(expected) || element_facts.may_match_id(elem_id)
}

fn element_has_attribute_presence(
    component: &SvelteComponent,
    element_facts: &ElementFacts,
    elem_id: NodeId,
    attr_name: &str,
) -> bool {
    if element_facts.has_spread(elem_id) {
        return true;
    }

    let Some(attrs) = element_attributes(component, elem_id) else {
        return false;
    };

    attrs
        .iter()
        .any(|attr| attribute_name_matches_selector(component, attr, attr_name))
}

fn element_matches_attribute_selector(
    component: &SvelteComponent,
    css_source: &str,
    element_facts: &ElementFacts,
    elem_id: NodeId,
    selector: &svelte_css::AttributeSelector,
) -> bool {
    let attr_name = selector.name.as_str();
    if selector.matcher.is_none() && selector.value.is_none() {
        return element_has_attribute_presence(component, element_facts, elem_id, attr_name);
    }

    if element_facts.has_spread(elem_id) {
        return true;
    }

    let Some(attrs) = element_attributes(component, elem_id) else {
        return false;
    };

    let operator = selector
        .matcher
        .map(|span| span.source_text(css_source).trim())
        .unwrap_or("");
    let expected = selector
        .value
        .map(|span| span.source_text(css_source))
        .unwrap_or("");
    let flags = selector
        .flags
        .map(|span| span.source_text(css_source).trim());
    let case_insensitive =
        attribute_selector_case_insensitive(element_facts, elem_id, attr_name, flags);

    for attr in attrs {
        if !attribute_name_matches_selector(component, attr, attr_name) {
            continue;
        }
        match attr {
            Attribute::StringAttribute(attr) => {
                if test_attribute(
                    operator,
                    expected,
                    case_insensitive,
                    attr.value_span.source_text(&component.source),
                ) {
                    return true;
                }
            }
            Attribute::BooleanAttribute(_) => {}
            // Dynamic or directive-backed attrs stay conservative in this slice.
            Attribute::ExpressionAttribute(_)
            | Attribute::ConcatenationAttribute(_)
            | Attribute::Shorthand(_)
            | Attribute::BindDirective(_)
            | Attribute::ClassDirective(_)
            | Attribute::StyleDirective(_)
            | Attribute::OnDirectiveLegacy(_) => return true,
            Attribute::SpreadAttribute(_)
            | Attribute::UseDirective(_)
            | Attribute::TransitionDirective(_)
            | Attribute::AnimateDirective(_)
            | Attribute::AttachTag(_) => {}
        }
    }

    false
}

fn attribute_name_matches_selector(
    component: &SvelteComponent,
    attr: &Attribute,
    selector_name: &str,
) -> bool {
    match attr {
        Attribute::Shorthand(attr) => attr
            .expression_span
            .source_text(&component.source)
            .trim()
            .eq_ignore_ascii_case(selector_name),
        _ => attr
            .name()
            .is_some_and(|attr_name| attr_name.eq_ignore_ascii_case(selector_name)),
    }
}

fn element_attributes<'a>(
    component: &'a SvelteComponent,
    elem_id: NodeId,
) -> Option<&'a [Attribute]> {
    match component.store.get(elem_id) {
        Node::Element(element) => Some(&element.attributes),
        Node::SvelteElement(element) => Some(&element.attributes),
        _ => None,
    }
}

fn attribute_selector_case_insensitive(
    element_facts: &ElementFacts,
    elem_id: NodeId,
    attr_name: &str,
    flags: Option<&str>,
) -> bool {
    let has_case_sensitive_flag = flags.is_some_and(|flags| flags.contains('s'));
    flags.is_some_and(|flags| flags.contains('i'))
        || (!has_case_sensitive_flag
            && !matches!(
                element_facts.namespace(elem_id),
                Some(NamespaceKind::Svg | NamespaceKind::MathMl | NamespaceKind::AnnotationXml)
            )
            && HTML_CASE_INSENSITIVE_ATTRIBUTES
                .iter()
                .any(|candidate| candidate.eq_ignore_ascii_case(attr_name)))
}

fn test_attribute(operator: &str, expected: &str, case_insensitive: bool, value: &str) -> bool {
    let (expected, value) = if case_insensitive {
        (expected.to_lowercase(), value.to_lowercase())
    } else {
        (expected.to_owned(), value.to_owned())
    };

    match operator {
        "=" => value == expected,
        "~=" => value.split_whitespace().any(|part| part == expected),
        "|=" => {
            value == expected
                || value
                    .strip_prefix(expected.as_str())
                    .is_some_and(|rest| rest.starts_with('-'))
        }
        "^=" => value.starts_with(expected.as_str()),
        "$=" => value.ends_with(expected.as_str()),
        "*=" => value.contains(expected.as_str()),
        _ => false,
    }
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
