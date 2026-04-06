use compact_str::CompactString;
use svelte_css::{ComplexSelector, RelativeSelector, SimpleSelector, StyleSheet, VisitMut};
use svelte_span::Span;

/// Transform the stylesheet AST: scope selectors and serialize to CSS text.
///
/// `hash_class` is the scoping class name (e.g. `"svelte-1a7i8ec"`).
/// `source` is the original CSS source text (needed by the printer which
/// resolves `Span`s back to source slices).
pub fn transform_css(
    hash_class: &str,
    mut stylesheet: StyleSheet,
    source: &str,
) -> String {
    let mut scoper = ScopeSelectors {
        hash_class: CompactString::new(hash_class),
    };
    scoper.visit_stylesheet_mut(&mut stylesheet);
    svelte_css::Printer::print(&stylesheet, source)
}

struct ScopeSelectors {
    hash_class: CompactString,
}

impl VisitMut for ScopeSelectors {
    fn visit_complex_selector_mut(&mut self, node: &mut ComplexSelector) {
        if is_entirely_global(node) {
            unwrap_global(node);
            return;
        }

        svelte_css::visit::walk_complex_selector_mut(self, node);
    }

    fn visit_relative_selector_mut(&mut self, node: &mut RelativeSelector) {
        let mut new_selectors = Vec::with_capacity(node.selectors.len() + 1);
        let mut has_non_global_scopable = false;
        let mut scope_inserted = false;

        for sel in node.selectors.drain(..) {
            match sel {
                SimpleSelector::Global {
                    args: Some(args), ..
                } => {
                    // Insert scope class before global content if we have a scopable selector
                    // but haven't inserted the scope class yet.
                    if has_non_global_scopable && !scope_inserted {
                        new_selectors.push(self.scope_class());
                        scope_inserted = true;
                    }
                    // Expand :global(...) — inline the inner selectors unscoped
                    for complex in args.children {
                        for rel in complex.children {
                            new_selectors.extend(rel.selectors);
                        }
                    }
                }
                SimpleSelector::Global { args: None, .. } => {
                    // Bare `:global` without args — drop it (block form handled elsewhere)
                }
                _ => {
                    if is_scopable(&sel) {
                        has_non_global_scopable = true;
                    }
                    new_selectors.push(sel);
                }
            }
        }

        if has_non_global_scopable && !scope_inserted {
            new_selectors.push(self.scope_class());
        }

        node.selectors = new_selectors.into();
    }
}

impl ScopeSelectors {
    fn scope_class(&self) -> SimpleSelector {
        SimpleSelector::Class {
            span: Span::new(0, 0),
            name: self.hash_class.clone(),
        }
    }
}

/// Returns true if the entire complex selector is a single `:global(...)` call.
fn is_entirely_global(complex: &ComplexSelector) -> bool {
    complex.children.len() == 1
        && complex.children[0].selectors.len() == 1
        && matches!(
            &complex.children[0].selectors[0],
            SimpleSelector::Global { args: Some(_), .. }
        )
}

/// Unwrap a `:global(...)` complex selector — replace with its inner selectors.
fn unwrap_global(complex: &mut ComplexSelector) {
    let sel = complex.children[0].selectors.remove(0);
    if let SimpleSelector::Global {
        args: Some(args), ..
    } = sel
    {
        let mut new_children = svelte_css::RelativeSelectorVec::new();
        for inner_complex in args.children {
            new_children.extend(inner_complex.children);
        }
        complex.children = new_children;
    }
}

/// Check if a simple selector is something that can be scoped
/// (type, class, id, attribute, pseudo-element, nesting).
fn is_scopable(sel: &SimpleSelector) -> bool {
    matches!(
        sel,
        SimpleSelector::Type { .. }
            | SimpleSelector::Class { .. }
            | SimpleSelector::Id { .. }
            | SimpleSelector::Attribute(_)
            | SimpleSelector::PseudoElement(_)
            | SimpleSelector::Nesting(_)
    )
}

/// Compact formatted CSS into a single-line string matching the reference compiler's
/// injected-mode format: whitespace inside declaration blocks is stripped, whitespace
/// before `{` is preserved as a single space.
pub fn compact_css_for_injection(css: &str) -> String {
    let mut out = String::with_capacity(css.len());
    let chars: Vec<char> = css.chars().collect();
    let n = chars.len();
    let mut i = 0;

    while i < n {
        let ch = chars[i];
        if ch == '{' || ch == ':' {
            out.push(ch);
            i += 1;
            while i < n && chars[i].is_ascii_whitespace() {
                i += 1;
            }
        } else if ch.is_ascii_whitespace() {
            let mut j = i;
            while j < n && chars[j].is_ascii_whitespace() {
                j += 1;
            }
            if j < n && chars[j] == '}' {
                i = j;
            } else {
                out.push(' ');
                i = j;
            }
        } else {
            out.push(ch);
            i += 1;
        }
    }

    while out.ends_with(|c: char| c.is_ascii_whitespace()) {
        out.pop();
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scope_type_selector() {
        let source = "p { color: red; }";
        let (ss, _) = svelte_css::parse(source);
        let result = transform_css("svelte-abc123", ss, source);
        assert!(result.contains("p.svelte-abc123"), "got: {result}");
    }

    #[test]
    fn global_not_scoped() {
        let source = ":global(.foo) { color: red; }";
        let (ss, _) = svelte_css::parse(source);
        let result = transform_css("svelte-abc123", ss, source);
        assert!(!result.contains("svelte-abc123"), "global should not be scoped, got: {result}");
        assert!(result.contains(".foo"), "got: {result}");
    }

    #[test]
    fn mixed_global_and_local() {
        let source = "p:global(.active) { font-weight: bold; }";
        let (ss, _) = svelte_css::parse(source);
        let result = transform_css("svelte-abc123", ss, source);
        assert!(result.contains("p.svelte-abc123.active"), "got: {result}");
    }
}
