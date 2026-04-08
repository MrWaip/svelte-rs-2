use compact_str::CompactString;
use svelte_css::{
    AtRule, Block, BlockChild, CombinatorKind, ComplexSelector, RelativeSelector, SelectorList,
    SimpleSelector, StyleRule, StyleSheet, Visit,
};
use svelte_diagnostics::Diagnostic;
use svelte_diagnostics::DiagnosticKind;
use svelte_span::GetSpan;

use svelte_ast::Component as SvelteComponent;

use crate::css::css_component_hash;
use crate::types::data::{AnalysisData, CssAnalysis};
use crate::types::node_table::NodeBitSet;

/// Classify the CSS block: compute hash, mark scoped template elements, set inject flag.
/// Also validates CSS and emits diagnostics for invalid `:global()` usage.
///
/// Does NOT transform or serialize CSS — call `svelte_transform_css::transform_css` for that.
pub fn analyze_css_pass(
    component: &SvelteComponent,
    stylesheet: &StyleSheet,
    inject_styles: bool,
    data: &mut AnalysisData,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let Some(css_block) = &component.css else {
        return;
    };
    let css_text = component.source_text(css_block.content_span);
    let hash = css_component_hash(css_text);

    // Phase 1: collect locally-scoped @keyframes names.
    let keyframes = collect_keyframe_names(stylesheet, css_text);

    // Phase 2: validate CSS (:global usage, nesting selectors, etc.)
    let mut validator = CssValidator::new(diagnostics);
    validator.visit_stylesheet(stylesheet);

    let node_count = component.node_count();
    data.output.css = CssAnalysis {
        hash,
        scoped_elements: NodeBitSet::new(node_count),
        inject_styles,
        keyframes,
        used_selectors: rustc_hash::FxHashSet::default(),
    };

    // Phase 3: prune — backward-match every selector against the template, populate
    // `used_selectors` and mark every matched element in `scoped_elements`.
    super::css_prune::prune_and_warn(stylesheet, css_text, data, diagnostics);
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

/// True if the `SimpleSelector` is `:global` (block form, no args).
fn is_global_block_selector(sel: &SimpleSelector) -> bool {
    matches!(sel, SimpleSelector::Global { args: None, .. })
}

/// True if a `RelativeSelector` is `:global(...)` or `:global` and all other
/// selectors in the compound are unscoped pseudo-classes or pseudo-elements.
/// Matches the reference compiler's `is_global()` from `css/utils.js`.
fn is_global_relative_selector(rel: &RelativeSelector) -> bool {
    let Some(first) = rel.selectors.first() else {
        return false;
    };
    match first {
        SimpleSelector::Global { args, .. } => {
            if args.is_none() {
                // `:global` block selector — counts as global
                return true;
            }
            // `:global(...)` — only global if all siblings are unscoped pseudo-classes/pseudo-elements
            rel.selectors.iter().all(|s| match s {
                SimpleSelector::PseudoElement(_) => true,
                SimpleSelector::PseudoClass(_) => is_unscoped_pseudo_class(s),
                SimpleSelector::Global { .. } => true,
                _ => false,
            })
        }
        _ => false,
    }
}

/// True if the pseudo-class cannot be scoped (matches reference `is_unscoped_pseudo_class`).
fn is_unscoped_pseudo_class(sel: &SimpleSelector) -> bool {
    let SimpleSelector::PseudoClass(pc) = sel else {
        return false;
    };
    let name = pc.name.as_str();
    // These pseudo-classes make the selector scoped
    if name == "has" || name == "is" || name == "where" {
        // ...unless all their children are global
        return pc.args.as_ref().map_or(true, |args| {
            args.children
                .iter()
                .all(|c| c.children.iter().all(|r| is_global_relative_selector(r)))
        });
    }
    if name == "not" {
        // :not is special: unscoped unless it has multiple selectors in args
        // (e.g. :not(.x .y) should be scoped, :not(.x) is unscoped)
        return pc.args.as_ref().map_or(true, |args| {
            args.children.iter().all(|c| c.children.len() == 1)
                || args
                    .children
                    .iter()
                    .all(|c| c.children.iter().all(|r| is_global_relative_selector(r)))
        });
    }
    // All other pseudo-classes are unscoped
    true
}

// ---------------------------------------------------------------------------
// CSS validation — `:global()` diagnostics
// ---------------------------------------------------------------------------

/// Per-rule context tracked on the validator's stack.
struct RuleContext {
    /// Whether this rule is a lone `:global` block (single complex, single relative, single selector).
    is_lone_global_block: bool,
    /// Whether this rule has a parent rule (i.e. is nested).
    has_parent_rule: bool,
    /// Whether the parent rule is a lone top-level `:global` block
    /// (needed for `&` validation inside `:global { &.foo { ... } }`).
    parent_is_lone_global_block: bool,
    /// True if this rule's prelude is exactly `:global(&)`.
    is_lone_global_with_nesting_arg: bool,
}

struct CssValidator<'a> {
    diagnostics: &'a mut Vec<Diagnostic>,
    /// Stack of parent style rules (innermost last).
    rule_stack: Vec<RuleContext>,
    /// Whether we're currently inside a pseudo-class selector's args.
    in_pseudo_class: bool,
}

impl<'a> CssValidator<'a> {
    fn new(diagnostics: &'a mut Vec<Diagnostic>) -> Self {
        Self {
            diagnostics,
            rule_stack: Vec::new(),
            in_pseudo_class: false,
        }
    }

    fn current_rule(&self) -> Option<&RuleContext> {
        self.rule_stack.last()
    }

    fn emit(&mut self, kind: DiagnosticKind, span: svelte_span::Span) {
        self.diagnostics.push(Diagnostic::error(kind, span));
    }

    /// Validate and traverse a style rule.
    /// Ported from the `Rule` visitor in `css-analyze.js`.
    fn validate_rule(&mut self, rule: &StyleRule) {
        let has_parent = !self.rule_stack.is_empty();
        let mut rule_is_global_block = false;
        let mut first_complex = true;

        for complex_selector in &rule.prelude.children {
            let mut is_global_block = false;

            for (selector_idx, child) in complex_selector.children.iter().enumerate() {
                let global_pos = child.selectors.iter().position(is_global_block_selector);

                if let Some(idx) = global_pos {
                    if idx == 0 {
                        if child.selectors.len() > 1 && selector_idx == 0 && !has_parent {
                            // `:global.foo { ... }` at top level — modifier not allowed
                            self.emit(
                                DiagnosticKind::CssGlobalBlockInvalidModifierStart,
                                child.selectors[1].span(),
                            );
                        } else {
                            is_global_block = true;

                            // `:global` can only follow descendant combinator
                            if let Some(ref comb) = child.combinator {
                                if comb.kind != CombinatorKind::Descendant {
                                    self.emit(
                                        DiagnosticKind::CssGlobalBlockInvalidCombinator {
                                            name: comb.kind.as_str().to_string(),
                                        },
                                        child.span,
                                    );
                                }
                            }

                            let is_lone_global = complex_selector.children.len() == 1
                                && complex_selector.children[0].selectors.len() == 1;

                            if is_lone_global && rule.prelude.children.len() > 1 {
                                self.emit(
                                    DiagnosticKind::CssGlobalBlockInvalidList,
                                    rule.prelude.span,
                                );
                            }

                            if is_lone_global
                                && rule.prelude.children.len() == 1
                                && has_declaration(&rule.block)
                            {
                                let decl_span = rule
                                    .block
                                    .children
                                    .iter()
                                    .find_map(|c| match c {
                                        BlockChild::Declaration(d) => Some(d.span),
                                        _ => None,
                                    })
                                    .unwrap();
                                self.emit(
                                    DiagnosticKind::CssGlobalBlockInvalidDeclaration,
                                    decl_span,
                                );
                            }
                        }
                    } else {
                        // `:global` not at position 0 — modifying an existing selector
                        self.emit(
                            DiagnosticKind::CssGlobalBlockInvalidModifier,
                            child.selectors[idx].span(),
                        );
                    }
                }
            }

            if rule_is_global_block && !is_global_block {
                self.emit(DiagnosticKind::CssGlobalBlockInvalidList, rule.prelude.span);
            }

            if first_complex {
                rule_is_global_block = is_global_block;
            }
            first_complex = false;
        }

        // Compute context for child rules
        let is_lone_global_block = rule_is_global_block
            && rule.prelude.children.len() == 1
            && rule.prelude.children[0].children.len() == 1
            && rule.prelude.children[0].children[0].selectors.len() == 1;

        let parent_is_lone_global_block = if has_parent {
            self.current_rule()
                .map_or(false, |r| r.is_lone_global_block && !r.has_parent_rule)
        } else {
            false
        };

        let is_lone_global_with_nesting_arg = rule.prelude.children.len() == 1
            && rule.prelude.children[0].children.len() == 1
            && rule.prelude.children[0].children[0].selectors.len() == 1
            && matches!(
                &rule.prelude.children[0].children[0].selectors[0],
                SimpleSelector::Global { args: Some(args), .. }
                if is_nesting_in_global_args(args)
            );

        self.rule_stack.push(RuleContext {
            is_lone_global_block,
            has_parent_rule: has_parent,
            parent_is_lone_global_block,
            is_lone_global_with_nesting_arg,
        });

        // Visit prelude then block
        self.visit_selector_list(&rule.prelude);
        self.visit_block(&rule.block);

        self.rule_stack.pop();
    }

    /// Validate a complex selector for `:global(...)` placement issues.
    /// Ported from the `ComplexSelector` visitor in `css-analyze.js`.
    fn validate_complex_selector(&mut self, node: &ComplexSelector) {
        // Check `:global` block inside pseudo-class
        if let Some(global_rel) = node
            .children
            .iter()
            .find(|r| is_global_relative_selector(r))
        {
            if self.in_pseudo_class && is_global_block_selector_in_rel(global_rel) {
                self.emit(
                    DiagnosticKind::CssGlobalBlockInvalidPlacement,
                    global_rel.selectors[0].span(),
                );
            }

            // `:global(...)` not in middle unless all following are also global
            let idx = node
                .children
                .iter()
                .position(|r| is_global_relative_selector(r))
                .unwrap();
            if let SimpleSelector::Global {
                args: Some(_),
                span,
                ..
            } = &global_rel.selectors[0]
            {
                if idx != 0 && idx != node.children.len() - 1 {
                    // Emit once per non-global child after the global one
                    for r in &node.children[idx + 1..] {
                        if !is_global_relative_selector(r) {
                            self.emit(DiagnosticKind::CssGlobalInvalidPlacement, *span);
                        }
                    }
                }
            }
        }

        // Validate `:global(...)` / `:global` args in each relative selector
        for rel in &node.children {
            for (i, sel) in rel.selectors.iter().enumerate() {
                let (args, span) = match sel {
                    SimpleSelector::Global { args, span, .. } => (args, span),
                    _ => continue,
                };

                if let Some(args) = args {
                    // `:global(element)` must be at position 0 in compound
                    if let Some(first_type_sel) = args
                        .children
                        .first()
                        .and_then(|c| c.children.first())
                        .and_then(|r| r.selectors.first())
                    {
                        if matches!(first_type_sel, SimpleSelector::Type { .. }) && i != 0 {
                            self.emit(DiagnosticKind::CssGlobalInvalidSelectorList, *span);
                        }
                    }

                    // `:global(...)` must contain exactly one selector in compound context
                    if args.children.len() > 1
                        && (node.children.len() > 1 || rel.selectors.len() > 1)
                    {
                        self.emit(DiagnosticKind::CssGlobalInvalidSelector, *span);
                    }
                }

                // `:global(...)` or bare `:global` must not be followed by a type selector
                if let Some(next) = rel.selectors.get(i + 1) {
                    if matches!(next, SimpleSelector::Type { .. }) {
                        self.emit(DiagnosticKind::CssTypeSelectorInvalidPlacement, next.span());
                    }
                }
            }
        }
    }

    /// Validate nesting selector (`&`) placement.
    /// Ported from the `NestingSelector` visitor in `css-analyze.js`.
    fn validate_nesting_selector(&mut self, span: svelte_span::Span) {
        let has_parent = self.current_rule().map_or(false, |r| r.has_parent_rule);

        if !has_parent {
            // `&` outside nested rule — only valid inside lone `:global(&)`
            let valid = self
                .current_rule()
                .map_or(false, |r| r.is_lone_global_with_nesting_arg);
            if !valid {
                self.emit(DiagnosticKind::CssNestingSelectorInvalidPlacement, span);
            }
        } else {
            // `:global { &.foo { ... } }` — `&` inside lone top-level global block is invalid
            if self
                .current_rule()
                .map_or(false, |r| r.parent_is_lone_global_block)
            {
                self.emit(DiagnosticKind::CssGlobalBlockInvalidModifierStart, span);
            }
        }
    }
}

/// True if the args of a `:global(...)` contain a single nesting selector `&`.
fn is_nesting_in_global_args(args: &SelectorList) -> bool {
    args.children
        .first()
        .and_then(|c| c.children.first())
        .map_or(false, |r| {
            r.selectors.len() == 1 && matches!(r.selectors[0], SimpleSelector::Nesting(_))
        })
}

/// True if a `RelativeSelector` starts with `:global` (block form, no args).
fn is_global_block_selector_in_rel(rel: &RelativeSelector) -> bool {
    rel.selectors.first().is_some_and(is_global_block_selector)
}

fn has_declaration(block: &Block) -> bool {
    block
        .children
        .iter()
        .any(|c| matches!(c, BlockChild::Declaration(_)))
}

impl Visit for CssValidator<'_> {
    fn visit_style_rule(&mut self, node: &StyleRule) {
        self.validate_rule(node);
    }

    fn visit_complex_selector(&mut self, node: &ComplexSelector) {
        // Validate relative selectors (leading combinator), walk into them, then validate complex
        for (i, child) in node.children.iter().enumerate() {
            if i == 0
                && child.combinator.is_some()
                && !self.in_pseudo_class
                && !self.current_rule().map_or(false, |r| r.has_parent_rule)
            {
                self.emit(
                    DiagnosticKind::CssSelectorInvalid,
                    child.combinator.as_ref().unwrap().span,
                );
            }
            self.visit_relative_selector(child);
        }
        self.validate_complex_selector(node);
    }

    fn visit_relative_selector(&mut self, node: &RelativeSelector) {
        for sel in &node.selectors {
            self.visit_simple_selector(sel);
        }
    }

    fn visit_simple_selector(&mut self, node: &SimpleSelector) {
        match node {
            SimpleSelector::Nesting(span) => {
                self.validate_nesting_selector(*span);
            }
            SimpleSelector::PseudoClass(pc) => {
                if let Some(args) = &pc.args {
                    let prev = self.in_pseudo_class;
                    self.in_pseudo_class = true;
                    self.visit_selector_list(args);
                    self.in_pseudo_class = prev;
                }
            }
            SimpleSelector::Global {
                args: Some(args), ..
            } => {
                let prev = self.in_pseudo_class;
                self.in_pseudo_class = true;
                self.visit_selector_list(args);
                self.in_pseudo_class = prev;
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use svelte_diagnostics::DiagnosticKind;

    /// Parse CSS and run the validator, returning collected diagnostics.
    fn validate_css(css: &str) -> Vec<DiagnosticKind> {
        let (stylesheet, _parse_diags) = svelte_css::parse(css);
        let mut diagnostics = Vec::new();
        let mut validator = CssValidator::new(&mut diagnostics);
        validator.visit_stylesheet(&stylesheet);
        diagnostics.into_iter().map(|d| d.kind).collect()
    }

    fn assert_diagnostic(css: &str, expected: DiagnosticKind) {
        let kinds = validate_css(css);
        assert!(
            kinds.contains(&expected),
            "Expected diagnostic {expected:?} for CSS:\n  {css}\nGot: {kinds:?}"
        );
    }

    fn assert_no_diagnostics(css: &str) {
        let kinds = validate_css(css);
        assert!(
            kinds.is_empty(),
            "Expected no diagnostics for CSS:\n  {css}\nGot: {kinds:?}"
        );
    }

    #[test]
    fn css_global_block_invalid_placement() {
        // `:global` (block form, no args) inside a pseudo-class
        assert_diagnostic(
            ":is(:global) { color: red; }",
            DiagnosticKind::CssGlobalBlockInvalidPlacement,
        );
    }

    #[test]
    fn css_global_invalid_placement() {
        // `:global(...)` in the middle of a selector sequence
        assert_diagnostic(
            ".a :global(.b) .c { color: red; }",
            DiagnosticKind::CssGlobalInvalidPlacement,
        );
    }

    #[test]
    fn css_global_invalid_placement_multiple_non_global_after() {
        // Reference emits once per non-global child after the global
        let kinds = validate_css(".a :global(.b) .c .d { color: red; }");
        let count = kinds
            .iter()
            .filter(|k| matches!(k, DiagnosticKind::CssGlobalInvalidPlacement))
            .count();
        assert_eq!(count, 2, "expected 2 emissions for .c and .d, got {count}");
    }

    #[test]
    fn css_global_invalid_placement_end_ok() {
        // `:global(...)` at the end is fine
        assert_no_diagnostics(".a :global(.b) { color: red; }");
    }

    #[test]
    fn css_global_invalid_placement_start_ok() {
        // `:global(...)` at the start is fine
        assert_no_diagnostics(":global(.a) .b { color: red; }");
    }

    #[test]
    fn css_global_invalid_selector_list() {
        // `:global(element)` not at position 0 in compound
        assert_diagnostic(
            ".a:global(div) { color: red; }",
            DiagnosticKind::CssGlobalInvalidSelectorList,
        );
    }

    #[test]
    fn css_type_selector_invalid_placement() {
        // `:global(.class)` followed by type selector
        assert_diagnostic(
            ":global(.a)div { color: red; }",
            DiagnosticKind::CssTypeSelectorInvalidPlacement,
        );
    }

    #[test]
    fn css_global_invalid_selector() {
        // `:global(...)` with multiple selectors in compound context
        assert_diagnostic(
            ".a:global(.b, .c) { color: red; }",
            DiagnosticKind::CssGlobalInvalidSelector,
        );
    }

    #[test]
    fn css_global_block_invalid_modifier_start() {
        // `:global` block modified at start of top-level rule
        assert_diagnostic(
            ":global.foo { color: red; }",
            DiagnosticKind::CssGlobalBlockInvalidModifierStart,
        );
    }

    #[test]
    fn css_global_block_invalid_combinator() {
        // `:global` block follows non-space combinator
        assert_diagnostic(
            "div > :global { p { color: red; } }",
            DiagnosticKind::CssGlobalBlockInvalidCombinator {
                name: ">".to_string(),
            },
        );
    }

    #[test]
    fn css_global_block_invalid_list() {
        // `:global` in selector list mixed with non-global entries
        assert_diagnostic(
            ":global, .foo { color: red; }",
            DiagnosticKind::CssGlobalBlockInvalidList,
        );
    }

    #[test]
    fn css_global_block_invalid_declaration() {
        // Lone `:global { ... }` block contains declarations
        assert_diagnostic(
            ":global { color: red; }",
            DiagnosticKind::CssGlobalBlockInvalidDeclaration,
        );
    }

    #[test]
    fn css_global_block_invalid_modifier() {
        // `:global` block selector not at position 0
        assert_diagnostic(
            ".foo:global { color: red; }",
            DiagnosticKind::CssGlobalBlockInvalidModifier,
        );
    }

    #[test]
    fn css_nesting_selector_invalid_placement() {
        // `&` used outside nested rule
        assert_diagnostic(
            "& { color: red; }",
            DiagnosticKind::CssNestingSelectorInvalidPlacement,
        );
    }

    #[test]
    fn css_nesting_selector_valid_in_global() {
        // `&` inside `:global(&)` is valid
        assert_no_diagnostics(":global(&) { color: red; }");
    }

    #[test]
    fn css_selector_invalid() {
        // Leading combinator on first selector
        assert_diagnostic("> .foo { color: red; }", DiagnosticKind::CssSelectorInvalid);
    }

    #[test]
    fn css_global_block_with_nested_rules_ok() {
        // Lone `:global { ... }` with nested rules (not declarations) is valid
        assert_no_diagnostics(":global { p { color: red; } }");
    }

    #[test]
    fn css_global_block_descendant_ok() {
        // `:global` following a descendant combinator (space) is valid
        assert_no_diagnostics("div :global { p { color: red; } }");
    }

    #[test]
    fn css_global_nesting_modifier_start_in_global_block() {
        // `&.foo` inside lone `:global { ... }` is invalid
        assert_diagnostic(
            ":global { &.foo { color: red; } }",
            DiagnosticKind::CssGlobalBlockInvalidModifierStart,
        );
    }

    #[test]
    fn css_global_block_invalid_list_mixed() {
        // One complex selector is global block, the other isn't
        assert_diagnostic(
            ":global .x, .y { color: red; }",
            DiagnosticKind::CssGlobalBlockInvalidList,
        );
    }

    #[test]
    fn css_nesting_in_compound_global_block_ok() {
        // `&` inside `:global .foo { ... }` (compound, not lone) should NOT
        // trigger CssGlobalBlockInvalidModifierStart — only lone `:global { &.foo }` does.
        assert_no_diagnostics(":global .foo { &.bar { color: red; } }");
    }

    #[test]
    fn valid_scoped_css_no_diagnostics() {
        assert_no_diagnostics("p { color: red; }");
        assert_no_diagnostics(".foo { color: red; }");
        assert_no_diagnostics(":global(.foo) { color: red; }");
        assert_no_diagnostics("p :global(.foo) { color: red; }");
    }
}
