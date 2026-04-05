use lightningcss::error::PrinterError;
use lightningcss::printer::{Printer, PrinterOptions};
use lightningcss::selector::{Component, PseudoClass, Selector};
use lightningcss::stylesheet::StyleSheet;
use lightningcss::traits::ToCss;
use lightningcss::values::ident::Ident;
use lightningcss::{visit_types, visitor::{Visit, Visitor, VisitTypes}};

/// Transform the stylesheet AST: scope selectors and serialize to CSS text.
///
/// `hash_str` must be allocated into the same arena as the stylesheet so that
/// `Ident<'a>` components satisfy the lifetime bound.
///
/// Returns `None` when the printer fails.
pub fn transform_css<'a>(
    hash_str: &'a str,
    mut stylesheet: StyleSheet<'a, 'a>,
) -> Option<String> {
    let mut scoper = ScopeSelectors { hash_class: hash_str };
    if stylesheet.visit(&mut scoper).is_err() {
        return None;
    }

    // Serialize directly through Printer with no CSS module context so that
    // class names are not renamed (stylesheet.to_css() would enable renaming
    // because the stylesheet was parsed with css_modules enabled).
    let mut dest = String::new();
    let mut printer = Printer::new(&mut dest, PrinterOptions::default());
    stylesheet.rules.to_css(&mut printer).ok()?;
    printer.newline().ok()?;
    Some(dest)
}

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
        // iter_raw_match_order() returns components in right-to-left (match) order at the
        // compound level: rightmost compound first, leftmost last. Within a single compound,
        // components are in parse (left-to-right) order. Selector::from() expects left-to-right
        // input. We split on combinators, then process compounds from highest index (leftmost in
        // CSS) down to 0 (rightmost), which produces the correct left-to-right sequence for
        // Selector::from(). Components within each compound are already in parse order and need
        // no reversal.
        let components: Vec<Component<'i>> =
            selector.iter_raw_match_order().cloned().collect();

        let has_local_name = components.iter().any(|c| matches!(c, Component::LocalName(_)));
        let has_global = components.iter().any(|c| {
            matches!(c, Component::NonTSPseudoClass(PseudoClass::Global { .. }))
        });

        if !has_local_name && !has_global {
            return Ok(());
        }

        // Split components into compound selectors (separated by combinators).
        // The resulting compounds are in right-to-left order (match order).
        let compounds: Vec<&[Component<'i>]> = components
            .as_slice()
            .split(|c| c.is_combinator())
            .collect();

        // Combinators in right-to-left order: combinators[k] is between compounds[k] and compounds[k+1].
        let combinators: Vec<Component<'i>> = components
            .iter()
            .filter(|c| c.is_combinator())
            .cloned()
            .collect();

        let mut result: Vec<Component<'i>> = Vec::with_capacity(components.len() + 1);

        // Iterate compounds from last index (leftmost in CSS) to 0 (rightmost in CSS).
        // Between compound[i] and compound[i-1] the combinator is combinators[i-1].
        for i in (0..compounds.len()).rev() {
            for component in compounds[i] {
                match component {
                    Component::NonTSPseudoClass(PseudoClass::Global { selector }) => {
                        // Expand the inner selector's components inline, without a scope class —
                        // content of :global() is intentionally unscoped.
                        // iter_raw_match_order() returns components within a compound in parse
                        // (left-to-right) order, so no reversal is needed here.
                        result.extend(selector.iter_raw_match_order().cloned());
                    }
                    Component::LocalName(_) => {
                        result.push(component.clone());
                        result.push(Component::Class(Ident::from(self.hash_class)));
                    }
                    _ => result.push(component.clone()),
                }
            }
            if i > 0 {
                result.push(combinators[i - 1].clone());
            }
        }

        *selector = Selector::from(result);
        Ok(())
    }
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
