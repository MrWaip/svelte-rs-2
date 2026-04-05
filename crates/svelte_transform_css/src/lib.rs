use lightningcss::error::PrinterError;
use lightningcss::selector::{Component, PseudoClass, Selector};
use lightningcss::stylesheet::{PrinterOptions, StyleSheet};
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
    stylesheet.to_css(PrinterOptions::default()).ok().map(|r| r.code)
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
                let class_ident: Ident<'h> = Ident::from(self.hash_class);
                result.push(Component::Class(class_ident));
            }
        }

        *selector = Selector::from(result);
        Ok(())
    }
}

fn has_global_component(selector: &Selector<'_>) -> bool {
    selector.iter_raw_match_order().any(|c| match c {
        Component::NonTSPseudoClass(PseudoClass::Global { .. }) => true,
        Component::NonTSPseudoClass(PseudoClass::Custom { name }) => name.as_ref() == "global",
        Component::NonTSPseudoClass(PseudoClass::CustomFunction { name, .. }) => {
            name.as_ref() == "global"
        }
        _ => false,
    })
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
