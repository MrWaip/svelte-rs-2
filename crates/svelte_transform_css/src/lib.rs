use compact_str::CompactString;
use svelte_css::{
    AtRule, Block, BlockChild, ComplexSelector, Declaration, RelativeSelector, Rule,
    SimpleSelector, StyleSheet, StyleSheetChild, VisitMut,
};
use svelte_span::Span;

/// Transform the stylesheet AST: scope selectors and serialize to CSS text.
///
/// `hash_class` is the scoping class name (e.g. `"svelte-1a7i8ec"`).
/// `keyframes` is the list of locally-scoped `@keyframes` names (collected by analyze).
/// `source` is the original CSS source text (needed by the printer which
/// resolves `Span`s back to source slices).
pub fn transform_css(
    hash_class: &str,
    keyframes: &[CompactString],
    mut stylesheet: StyleSheet,
    source: &str,
) -> String {
    let mut scoper = ScopeSelectors {
        hash_class: CompactString::new(hash_class),
        keyframes,
        source,
    };
    scoper.visit_stylesheet_mut(&mut stylesheet);
    svelte_css::Printer::print(&stylesheet, source)
}

struct ScopeSelectors<'a> {
    hash_class: CompactString,
    keyframes: &'a [CompactString],
    source: &'a str,
}

impl VisitMut for ScopeSelectors<'_> {
    fn visit_stylesheet_mut(&mut self, node: &mut StyleSheet) {
        let children = std::mem::take(&mut node.children);
        let mut new_children = Vec::with_capacity(children.len());
        for child in children {
            match child {
                StyleSheetChild::Rule(Rule::Style(sr)) if sr.is_lone_global_block() => {
                    // Hoist inner rules unscoped — do NOT visit them with the scoper
                    for block_child in sr.block.children {
                        match block_child {
                            BlockChild::Rule(rule) => {
                                new_children.push(StyleSheetChild::Rule(rule));
                            }
                            BlockChild::Comment(c) => {
                                new_children.push(StyleSheetChild::Comment(c));
                            }
                            _ => {}
                        }
                    }
                }
                StyleSheetChild::Rule(mut rule) => {
                    self.visit_rule_mut(&mut rule);
                    new_children.push(StyleSheetChild::Rule(rule));
                }
                other => new_children.push(other),
            }
        }
        node.children = new_children;
    }

    fn visit_block_mut(&mut self, node: &mut Block) {
        let children = std::mem::take(&mut node.children);
        let mut new_children = Vec::with_capacity(children.len());
        for child in children {
            match child {
                BlockChild::Rule(Rule::Style(sr)) if sr.is_lone_global_block() => {
                    // Hoist inner rules unscoped
                    for block_child in sr.block.children {
                        new_children.push(block_child);
                    }
                }
                BlockChild::Rule(mut rule) => {
                    self.visit_rule_mut(&mut rule);
                    new_children.push(BlockChild::Rule(rule));
                }
                BlockChild::Declaration(mut d) => {
                    self.visit_declaration_mut(&mut d);
                    new_children.push(BlockChild::Declaration(d));
                }
                other => new_children.push(other),
            }
        }
        node.children = new_children;
    }

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
                    // Recurse into pseudo-class args (e.g. :not(), :is(), :where(), :has())
                    // so that :global() inside them is unwrapped and inner selectors are scoped.
                    if let Some(last) = new_selectors.last_mut() {
                        self.visit_simple_selector_mut(last);
                    }
                }
            }
        }

        if has_non_global_scopable && !scope_inserted {
            // Insert scope class before trailing pseudo-class/pseudo-element selectors,
            // matching the reference compiler which walks backwards to find insertion point.
            let mut insert_pos = new_selectors.len();
            while insert_pos > 0 {
                match &new_selectors[insert_pos - 1] {
                    SimpleSelector::PseudoClass(_) | SimpleSelector::PseudoElement(_) => {
                        insert_pos -= 1;
                    }
                    _ => break,
                }
            }
            new_selectors.insert(insert_pos, self.scope_class());
        }

        node.selectors = new_selectors.into();
    }

    fn visit_at_rule_mut(&mut self, node: &mut AtRule) {
        if node.name == "keyframes" {
            let prelude = node.prelude.source_text(self.source).trim();
            if let Some(stripped) = prelude.strip_prefix("-global-") {
                // `-global-` escape: strip prefix, leave name unscoped
                node.prelude_override = Some(CompactString::new(stripped));
            } else if self.keyframes.iter().any(|k| k.as_str() == prelude) {
                // Local keyframe: prefix with component hash
                node.prelude_override =
                    Some(CompactString::new(&format!("{}-{}", self.hash_class, prelude)));
            }
            // Do NOT recurse into @keyframes body — keyframe selectors (from/to/%)
            // are not scoped.
            return;
        }
        svelte_css::visit::walk_at_rule_mut(self, node);
    }

    fn visit_simple_selector_mut(&mut self, node: &mut SimpleSelector) {
        // Recurse into :is(), :where(), :has(), :not() arguments so that
        // :global() inside them is unwrapped and non-global selectors are scoped.
        // Mirrors reference compiler PseudoClassSelector visitor.
        if let SimpleSelector::PseudoClass(pc) = node {
            match pc.name.as_str() {
                "is" | "where" | "has" | "not" => {
                    svelte_css::visit::walk_simple_selector_args_mut(self, node);
                }
                _ => {}
            }
        }
    }

    fn visit_declaration_mut(&mut self, node: &mut Declaration) {
        if self.keyframes.is_empty() {
            return;
        }
        let prop = node.property.source_text(self.source).trim();
        let prop_lower = prop.to_ascii_lowercase();
        let is_animation = prop_lower == "animation" || prop_lower == "animation-name";
        if !is_animation {
            return;
        }
        let value = node.value.source_text(self.source);
        let rewritten = rewrite_animation_value(value, &self.hash_class, self.keyframes);
        if let Some(rewritten) = rewritten {
            node.value_override = Some(rewritten);
        }
    }
}

/// Scan an `animation` or `animation-name` value token by token, replacing
/// tokens that match a locally-scoped keyframe name with the hash-prefixed form.
///
/// Returns `Some(rewritten)` if any replacement was made, `None` otherwise.
fn rewrite_animation_value(
    value: &str,
    hash_class: &str,
    keyframes: &[CompactString],
) -> Option<String> {
    let mut result = String::with_capacity(value.len() + 32);
    let mut changed = false;
    let bytes = value.as_bytes();
    let len = bytes.len();
    let mut i = 0;

    while i < len {
        let b = bytes[i];
        // Accumulate non-name characters (whitespace, commas, semicolons)
        if b.is_ascii_whitespace() || b == b',' {
            result.push(b as char);
            i += 1;
            continue;
        }
        // Accumulate a CSS name token
        let start = i;
        while i < len && !bytes[i].is_ascii_whitespace() && bytes[i] != b',' && bytes[i] != b';' {
            i += 1;
        }
        let token = &value[start..i];
        if keyframes.iter().any(|k| k.as_str() == token) {
            result.push_str(hash_class);
            result.push('-');
            result.push_str(token);
            changed = true;
        } else {
            result.push_str(token);
        }
    }

    changed.then_some(result)
}

impl ScopeSelectors<'_> {
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
        let result = transform_css("svelte-abc123", &[], ss, source);
        assert!(result.contains("p.svelte-abc123"), "got: {result}");
    }

    #[test]
    fn global_not_scoped() {
        let source = ":global(.foo) { color: red; }";
        let (ss, _) = svelte_css::parse(source);
        let result = transform_css("svelte-abc123", &[], ss, source);
        assert!(!result.contains("svelte-abc123"), "global should not be scoped, got: {result}");
        assert!(result.contains(".foo"), "got: {result}");
    }

    #[test]
    fn mixed_global_and_local() {
        let source = "p:global(.active) { font-weight: bold; }";
        let (ss, _) = svelte_css::parse(source);
        let result = transform_css("svelte-abc123", &[], ss, source);
        assert!(result.contains("p.svelte-abc123.active"), "got: {result}");
    }

    #[test]
    fn global_block_not_scoped() {
        let source = ":global { p { color: red; } }";
        let (ss, _) = svelte_css::parse(source);
        let result = transform_css("svelte-abc123", &[], ss, source);
        assert!(
            !result.contains("svelte-abc123"),
            "global block inner rules should not be scoped, got: {result}"
        );
        assert!(result.contains("p"), "got: {result}");
        assert!(!result.contains(":global"), "global wrapper should be removed, got: {result}");
    }

    #[test]
    fn global_block_multiple_inner_rules() {
        let source = ":global { .foo { color: red; } .bar { font-size: 16px; } }";
        let (ss, _) = svelte_css::parse(source);
        let result = transform_css("svelte-abc123", &[], ss, source);
        assert!(!result.contains("svelte-abc123"), "got: {result}");
        assert!(result.contains(".foo"), "got: {result}");
        assert!(result.contains(".bar"), "got: {result}");
        assert!(!result.contains(":global"), "got: {result}");
    }

    #[test]
    fn global_block_mixed_with_local() {
        let source = ":global { p { color: red; } }\ndiv { font-size: 16px; }";
        let (ss, _) = svelte_css::parse(source);
        let result = transform_css("svelte-abc123", &[], ss, source);
        assert!(
            !result.contains("p.svelte-abc123"),
            "p should not be scoped, got: {result}"
        );
        assert!(
            result.contains("div.svelte-abc123"),
            "div should be scoped, got: {result}"
        );
    }

    #[test]
    fn global_block_in_media() {
        let source = "@media (min-width: 768px) { :global { p { color: red; } } }";
        let (ss, _) = svelte_css::parse(source);
        let result = transform_css("svelte-abc123", &[], ss, source);
        assert!(!result.contains("svelte-abc123"), "got: {result}");
        assert!(result.contains("p"), "got: {result}");
        assert!(!result.contains(":global"), "got: {result}");
    }

    #[test]
    fn global_block_nested_in_style_rule() {
        let source = ".foo { :global { .bar { color: red; } } }";
        let (ss, _) = svelte_css::parse(source);
        let result = transform_css("svelte-abc123", &[], ss, source);
        assert!(
            result.contains(".foo.svelte-abc123"),
            ".foo should be scoped, got: {result}"
        );
        assert!(
            !result.contains(".bar.svelte-abc123"),
            ".bar should not be scoped, got: {result}"
        );
    }

    #[test]
    fn scope_keyframes_basic() {
        let source = "@keyframes bounce { from { opacity: 0; } to { opacity: 1; } }";
        let keyframes = vec![CompactString::new("bounce")];
        let (ss, _) = svelte_css::parse(source);
        let result = transform_css("svelte-abc123", &keyframes, ss, source);
        assert!(
            result.contains("@keyframes svelte-abc123-bounce"),
            "keyframe name should be scoped, got: {result}"
        );
    }

    #[test]
    fn keyframes_global_escape() {
        let source = "@keyframes -global-slide { from { opacity: 0; } to { opacity: 1; } }";
        let (ss, _) = svelte_css::parse(source);
        let result = transform_css("svelte-abc123", &[], ss, source);
        assert!(
            result.contains("@keyframes slide"),
            "-global- prefix should be stripped, got: {result}"
        );
        assert!(
            !result.contains("-global-"),
            "-global- should not appear in output, got: {result}"
        );
        assert!(
            !result.contains("svelte-abc123-slide"),
            "global keyframe should not be scoped, got: {result}"
        );
    }

    #[test]
    fn animation_value_rewritten() {
        let source = ".foo { animation: bounce 2s ease; }";
        let keyframes = vec![CompactString::new("bounce")];
        let (ss, _) = svelte_css::parse(source);
        let result = transform_css("svelte-abc123", &keyframes, ss, source);
        assert!(
            result.contains("svelte-abc123-bounce 2s ease"),
            "animation value should reference scoped keyframe, got: {result}"
        );
    }

    #[test]
    fn animation_name_rewritten() {
        let source = ".foo { animation-name: bounce, other; }";
        let keyframes = vec![CompactString::new("bounce")];
        let (ss, _) = svelte_css::parse(source);
        let result = transform_css("svelte-abc123", &keyframes, ss, source);
        assert!(
            result.contains("svelte-abc123-bounce"),
            "matching keyframe should be rewritten, got: {result}"
        );
        assert!(
            result.contains(", other"),
            "non-matching name should be preserved, got: {result}"
        );
    }

    #[test]
    fn global_inside_not() {
        let source = "p:not(:global(.active)) { color: red; }";
        let (ss, _) = svelte_css::parse(source);
        let result = transform_css("svelte-abc123", &[], ss, source);
        assert!(
            result.contains("p.svelte-abc123:not(.active)"),
            ":global() inside :not() should be unwrapped, got: {result}"
        );
        assert!(
            !result.contains(":global"),
            ":global should be removed, got: {result}"
        );
    }

    #[test]
    fn global_inside_is() {
        let source = ":is(:global(.foo), .bar) { color: red; }";
        let (ss, _) = svelte_css::parse(source);
        let result = transform_css("svelte-abc123", &[], ss, source);
        assert!(
            result.contains(".foo"),
            ":global(.foo) should be unwrapped to .foo, got: {result}"
        );
        assert!(
            result.contains(".bar.svelte-abc123"),
            ".bar should be scoped, got: {result}"
        );
        assert!(
            !result.contains(":global"),
            ":global should be removed, got: {result}"
        );
    }

    #[test]
    fn global_inside_where() {
        let source = "p:where(:global(.x)) { color: red; }";
        let (ss, _) = svelte_css::parse(source);
        let result = transform_css("svelte-abc123", &[], ss, source);
        assert!(
            result.contains("p.svelte-abc123:where(.x)"),
            ":global() inside :where() should be unwrapped, got: {result}"
        );
    }

    #[test]
    fn scoped_inside_not() {
        let source = "p:not(.active) { color: red; }";
        let (ss, _) = svelte_css::parse(source);
        let result = transform_css("svelte-abc123", &[], ss, source);
        assert!(
            result.contains("p.svelte-abc123:not(.active.svelte-abc123)"),
            "selectors inside :not() should be scoped, got: {result}"
        );
    }

    #[test]
    fn keyframes_in_global_block_untouched() {
        let source = ":global { @keyframes slide { from { opacity: 0; } to { opacity: 1; } } }";
        let (ss, _) = svelte_css::parse(source);
        let result = transform_css("svelte-abc123", &[], ss, source);
        assert!(
            result.contains("@keyframes slide"),
            "keyframes in :global block should stay unscoped, got: {result}"
        );
        assert!(
            !result.contains("svelte-abc123-slide"),
            "should not be scoped, got: {result}"
        );
    }
}
