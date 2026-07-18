use std::mem;

use compact_str::CompactString;
use rustc_hash::FxHashSet;
use svelte_css::{
    AtRule, Block, BlockChild, Combinator, CombinatorKind, ComplexSelector, CssNodeId, Declaration,
    RelativeSelector, Rule, SelectorList, SimpleSelector, StyleRule, StyleSheet, StyleSheetChild,
    VisitMut,
    visit::{
        walk_at_rule_mut, walk_complex_selector_mut, walk_selector_list_mut,
        walk_simple_selector_args_mut,
    },
};
use svelte_sourcemap::SourceMap;
use svelte_span::Span;

pub fn transform_css(
    hash_class: &str,
    keyframes: &[CompactString],
    stylesheet: StyleSheet,
    source: &str,
) -> String {
    transform_css_with_usage(hash_class, keyframes, None, false, stylesheet, source)
}

pub fn transform_css_with_usage(
    hash_class: &str,
    keyframes: &[CompactString],
    used_selectors: Option<&FxHashSet<CssNodeId>>,
    remove_unused: bool,
    mut stylesheet: StyleSheet,
    source: &str,
) -> String {
    apply_scoping(hash_class, keyframes, &mut stylesheet, source);
    if let Some(used_selectors) = used_selectors {
        svelte_css::Printer::print_with_usage(&stylesheet, source, used_selectors, remove_unused)
    } else {
        svelte_css::Printer::print(&stylesheet, source)
    }
}

pub fn transform_css_with_sourcemap(
    hash_class: &str,
    keyframes: &[CompactString],
    used_selectors: Option<&FxHashSet<CssNodeId>>,
    remove_unused: bool,
    mut stylesheet: StyleSheet,
    source: &str,
    filename: &str,
) -> (String, SourceMap) {
    apply_scoping(hash_class, keyframes, &mut stylesheet, source);
    svelte_css::Printer::print_with_sourcemap(
        &stylesheet,
        source,
        filename,
        used_selectors,
        remove_unused,
    )
}

fn apply_scoping(
    hash_class: &str,
    keyframes: &[CompactString],
    stylesheet: &mut StyleSheet,
    source: &str,
) {
    let mut scoper = ScopeSelectors {
        hash_class: CompactString::new(hash_class),
        keyframes,
        source,
        after_bare_global: false,
        suppress_scoping_depth: 0,
        specificity_bumped: false,
        selector_list_depth: 0,
        ancestor_has_local: false,
    };
    scoper.visit_stylesheet_mut(stylesheet);
}

struct ScopeSelectors<'a> {
    hash_class: CompactString,
    keyframes: &'a [CompactString],
    source: &'a str,
    after_bare_global: bool,
    suppress_scoping_depth: usize,
    specificity_bumped: bool,
    selector_list_depth: usize,
    ancestor_has_local: bool,
}

impl VisitMut for ScopeSelectors<'_> {
    fn visit_stylesheet_mut(&mut self, node: &mut StyleSheet) {
        let children = mem::take(&mut node.children);
        let mut new_children = Vec::with_capacity(children.len());
        for child in children {
            match child {
                StyleSheetChild::Rule(Rule::Style(sr)) if sr.is_lone_global_block() => {
                    self.suppress_scoping_depth += 1;
                    for block_child in sr.block.children {
                        match block_child {
                            BlockChild::Rule(mut rule) => {
                                self.visit_rule_mut(&mut rule);
                                new_children.push(StyleSheetChild::Rule(rule));
                            }
                            BlockChild::Comment(c) => {
                                new_children.push(StyleSheetChild::Comment(c));
                            }
                            _ => {}
                        }
                    }
                    self.suppress_scoping_depth -= 1;
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
        let children = mem::take(&mut node.children);
        let mut new_children = Vec::with_capacity(children.len());
        for child in children {
            match child {
                BlockChild::Rule(Rule::Style(sr)) if sr.is_lone_global_block() => {
                    self.suppress_scoping_depth += 1;
                    for block_child in sr.block.children {
                        match block_child {
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
                    self.suppress_scoping_depth -= 1;
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

    fn visit_style_rule_mut(&mut self, node: &mut StyleRule) {
        let was_ancestor_has_local = self.ancestor_has_local;
        let this_rule_has_local = node.prelude.children.iter().any(complex_has_local_selector);
        let is_global_block = node.is_global_block();
        self.visit_selector_list_mut(&mut node.prelude);
        self.ancestor_has_local = was_ancestor_has_local || this_rule_has_local;
        if is_global_block {
            self.suppress_scoping_depth += 1;
        }
        self.visit_block_mut(&mut node.block);
        if is_global_block {
            self.suppress_scoping_depth -= 1;
        }
        self.ancestor_has_local = was_ancestor_has_local;
    }

    fn visit_complex_selector_mut(&mut self, node: &mut ComplexSelector) {
        let was_after_bare_global = self.after_bare_global;
        let was_specificity_bumped = self.specificity_bumped;
        let has_implicit_nesting = self.ancestor_has_local && !complex_has_explicit_nesting(node);
        self.after_bare_global = false;
        if has_implicit_nesting {
            self.specificity_bumped = true;
        }

        if is_entirely_global(node) {
            unwrap_global(node);
            self.after_bare_global = was_after_bare_global;
            self.specificity_bumped = was_specificity_bumped;
            return;
        }

        merge_bare_global_compound(node);
        expand_inline_global_with_combinators(node);

        walk_complex_selector_mut(self, node);
        let children = mem::take(&mut node.children);
        node.children.extend(
            children
                .into_iter()
                .enumerate()
                .filter_map(|(idx, rel)| (!rel.selectors.is_empty() || idx == 0).then_some(rel)),
        );
        self.after_bare_global = was_after_bare_global;
        self.specificity_bumped = was_specificity_bumped;
    }

    fn visit_selector_list_mut(&mut self, node: &mut SelectorList) {
        let was_specificity_bumped = self.specificity_bumped;
        let should_reset = self.selector_list_depth == 0;
        self.selector_list_depth += 1;
        if should_reset {
            self.specificity_bumped = self.ancestor_has_local;
        }
        walk_selector_list_mut(self, node);
        self.selector_list_depth -= 1;
        self.specificity_bumped = was_specificity_bumped;
    }

    fn visit_relative_selector_mut(&mut self, node: &mut RelativeSelector) {
        let suppress_scoping =
            self.suppress_scoping_depth > 0 || compound_has_unscoped_pseudo(node);
        let leading_pseudo_scope = !suppress_scoping
            && !self.after_bare_global
            && compound_is_pseudo_only_scopable(&node.selectors);
        let mut new_selectors = Vec::with_capacity(node.selectors.len() + 1);
        let mut has_local_scopable = false;
        let mut unscoped_tail = self.after_bare_global;
        let mut has_nesting_selector = false;
        let mut scope_contributor = leading_pseudo_scope;
        let mut global_unwrapped = false;
        let mut last_local_end: Option<usize> = None;

        for sel in node.selectors.drain(..) {
            match sel {
                SimpleSelector::Global {
                    args: Some(args), ..
                } => {
                    global_unwrapped = true;
                    for complex in args.children {
                        for rel in complex.children {
                            new_selectors.extend(rel.selectors);
                        }
                    }
                }
                SimpleSelector::Global { args: None, .. } => {
                    unscoped_tail = true;
                    self.after_bare_global = true;
                }
                _ => {
                    let is_nesting = matches!(sel, SimpleSelector::Nesting(_));
                    if is_nesting {
                        has_nesting_selector = true;
                    }
                    let this_scopable = !suppress_scoping && !unscoped_tail && is_scopable(&sel);
                    if this_scopable {
                        has_local_scopable = true;
                    }
                    new_selectors.push(sel);
                    if this_scopable {
                        last_local_end = Some(new_selectors.len());
                    }

                    if let Some(last) = new_selectors.last_mut() {
                        let was_specificity_bumped = self.specificity_bumped;
                        if scope_contributor {
                            self.specificity_bumped = true;
                        }
                        if unscoped_tail {
                            self.suppress_scoping_depth += 1;
                        }
                        self.visit_simple_selector_mut(last);
                        if unscoped_tail {
                            self.suppress_scoping_depth -= 1;
                        }
                        self.specificity_bumped = was_specificity_bumped;
                    }

                    let this_contributes = if is_nesting {
                        self.ancestor_has_local
                    } else {
                        true
                    };
                    scope_contributor |= this_contributes;
                }
            }
        }

        if !has_local_scopable
            && !suppress_scoping
            && !unscoped_tail
            && !has_nesting_selector
            && !global_unwrapped
            && compound_is_pseudo_only_scopable(&new_selectors)
        {
            has_local_scopable = true;
        }

        if has_local_scopable && !has_nesting_selector {
            let insert_pos =
                last_local_end.unwrap_or_else(|| position_before_trailing_pseudos(&new_selectors));
            let modifier = self.scope_modifier();
            let universal_pos = insert_pos.checked_sub(1).filter(|&pos| {
                matches!(&new_selectors[pos], SimpleSelector::Type { name, .. } if name == "*")
            });
            match universal_pos {
                Some(pos) => new_selectors[pos] = modifier,
                None => new_selectors.insert(insert_pos, modifier),
            }
        }

        if !suppress_scoping && has_nesting_selector && self.ancestor_has_local {
            self.specificity_bumped = true;
        }

        node.selectors = new_selectors.into();
    }

    fn visit_at_rule_mut(&mut self, node: &mut AtRule) {
        if strip_vendor_prefix(&node.name) == "keyframes" {
            let prelude = node.prelude.source_text(self.source).trim();
            if let Some(stripped) = prelude.strip_prefix("-global-") {
                node.prelude_override = Some(CompactString::new(stripped));
            } else if self.keyframes.iter().any(|k| k.as_str() == prelude) {
                node.prelude_override = Some(CompactString::new(format!(
                    "{}-{}",
                    self.hash_class, prelude
                )));
            }

            return;
        }
        walk_at_rule_mut(self, node);
    }

    fn visit_simple_selector_mut(&mut self, node: &mut SimpleSelector) {
        if let SimpleSelector::PseudoClass(pc) = node {
            match pc.name.as_str() {
                "is" | "where" | "has" | "not" => {
                    if pc.name == "not"
                        && pc
                            .args
                            .as_ref()
                            .is_some_and(|args| not_args_stay_unscoped(args))
                    {
                        self.suppress_scoping_depth += 1;
                        walk_simple_selector_args_mut(self, node);
                        self.suppress_scoping_depth -= 1;
                        return;
                    }
                    walk_simple_selector_args_mut(self, node);
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
        let unprefixed = strip_vendor_prefix(&prop_lower);
        let is_animation = unprefixed == "animation" || unprefixed == "animation-name";
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

fn strip_vendor_prefix(name: &str) -> &str {
    for prefix in ["-webkit-", "-moz-", "-o-", "-ms-"] {
        if let Some(rest) = name.strip_prefix(prefix) {
            return rest;
        }
    }
    name
}

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

        if b.is_ascii_whitespace() || b == b',' {
            result.push(b as char);
            i += 1;
            continue;
        }

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
    fn scope_modifier(&mut self) -> SimpleSelector {
        let first_bump = !self.specificity_bumped;
        self.specificity_bumped = true;

        if first_bump {
            return SimpleSelector::Class {
                span: Span::new(0, 0),
                name: self.hash_class.clone(),
            };
        }

        SimpleSelector::PseudoClass(svelte_css::PseudoClassSelector {
            span: Span::new(0, 0),
            name: CompactString::new("where"),
            args: Some(Box::new(SelectorList {
                span: Span::new(0, 0),
                children: vec![ComplexSelector {
                    id: CssNodeId(0),
                    span: Span::new(0, 0),
                    children: vec![RelativeSelector {
                        id: CssNodeId(0),
                        span: Span::new(0, 0),
                        combinator: None,
                        selectors: vec![SimpleSelector::Class {
                            span: Span::new(0, 0),
                            name: self.hash_class.clone(),
                        }]
                        .into(),
                    }]
                    .into(),
                }]
                .into(),
            })),
        })
    }
}

fn complex_has_explicit_nesting(complex: &ComplexSelector) -> bool {
    complex.children.iter().any(relative_has_explicit_nesting)
}

fn relative_has_explicit_nesting(relative: &RelativeSelector) -> bool {
    relative.selectors.iter().any(simple_has_explicit_nesting)
}

fn simple_has_explicit_nesting(selector: &SimpleSelector) -> bool {
    match selector {
        SimpleSelector::Nesting(_) => true,
        SimpleSelector::PseudoClass(pc) => pc
            .args
            .as_ref()
            .is_some_and(|args| selector_list_has_explicit_nesting(args)),
        SimpleSelector::Global {
            args: Some(args), ..
        } => selector_list_has_explicit_nesting(args),
        _ => false,
    }
}

fn selector_list_has_explicit_nesting(list: &SelectorList) -> bool {
    list.children.iter().any(complex_has_explicit_nesting)
}

fn not_args_stay_unscoped(args: &SelectorList) -> bool {
    args.children
        .iter()
        .all(|complex| complex.children.len() == 1)
}

fn compound_has_unscoped_pseudo(node: &RelativeSelector) -> bool {
    node.selectors.iter().any(|selector| {
        matches!(selector, SimpleSelector::PseudoClass(pc) if pc.name == "root" || pc.name == "host")
    })
}

fn position_before_trailing_pseudos(selectors: &[SimpleSelector]) -> usize {
    let mut pos = selectors.len();
    while pos > 0 {
        match &selectors[pos - 1] {
            SimpleSelector::PseudoClass(_) | SimpleSelector::PseudoElement(_) => pos -= 1,
            _ => break,
        }
    }
    pos
}

fn compound_is_pseudo_only_scopable(selectors: &[SimpleSelector]) -> bool {
    !selectors.is_empty()
        && selectors.iter().all(|selector| {
            matches!(
                selector,
                SimpleSelector::PseudoClass(_) | SimpleSelector::PseudoElement(_)
            )
        })
        && pseudo_only_compound_is_scopable(selectors)
}

fn is_entirely_global(complex: &ComplexSelector) -> bool {
    complex.children.len() == 1
        && complex.children[0].selectors.len() == 1
        && matches!(
            &complex.children[0].selectors[0],
            SimpleSelector::Global { args: Some(_), .. }
        )
}

fn merge_bare_global_compound(node: &mut ComplexSelector) {
    if !node.children.iter().any(starts_with_bare_global_compound) {
        return;
    }

    let children = mem::take(&mut node.children);
    let mut out = svelte_css::RelativeSelectorVec::new();
    for mut rel in children {
        if !starts_with_bare_global_compound(&rel) {
            out.push(rel);
            continue;
        }

        let is_descendant = matches!(
            rel.combinator.map(|combinator| combinator.kind),
            Some(CombinatorKind::Descendant)
        );
        if is_descendant && let Some(prev) = out.last_mut() {
            prev.selectors.append(&mut rel.selectors);
            continue;
        }

        if rel.combinator.is_none() {
            rel.selectors
                .insert(0, SimpleSelector::Nesting(Span::new(0, 0)));
        }
        out.push(rel);
    }
    node.children = out;
}

fn starts_with_bare_global_compound(rel: &RelativeSelector) -> bool {
    rel.selectors.len() > 1
        && matches!(
            rel.selectors.first(),
            Some(SimpleSelector::Global { args: None, .. })
        )
}

fn expand_inline_global_with_combinators(node: &mut ComplexSelector) {
    let needs_expand = node.children.iter().any(is_expandable_global);
    if !needs_expand {
        return;
    }

    let mut out = svelte_css::RelativeSelectorVec::new();
    for mut rel in mem::take(&mut node.children) {
        if !is_expandable_global(&rel) {
            out.push(rel);
            continue;
        }
        let outer_combinator = rel.combinator;
        let SimpleSelector::Global {
            span: global_span,
            args: Some(mut args),
        } = rel.selectors.remove(0)
        else {
            unreachable!()
        };
        let mut trailing = mem::take(&mut rel.selectors);
        let args_span = args.span;
        let inner_complex = args.children.remove(0);
        let inner_len = inner_complex.children.len();
        for (i, inner_rel) in inner_complex.children.into_iter().enumerate() {
            let combinator = if i == 0 {
                promoted_first_combinator(outer_combinator, inner_rel.combinator, inner_len)
            } else {
                inner_rel.combinator
            };
            let inner_span = inner_rel.span;
            let inner_id = inner_rel.id;
            let mut inner_children = svelte_css::RelativeSelectorVec::new();
            inner_children.push(RelativeSelector {
                id: inner_id,
                span: inner_span,
                combinator: None,
                selectors: inner_rel.selectors,
            });
            let mut list_children = svelte_css::SelectorVec::new();
            list_children.push(ComplexSelector {
                id: CssNodeId::DUMMY,
                span: inner_span,
                children: inner_children,
            });
            let wrapped_global = SimpleSelector::Global {
                span: global_span,
                args: Some(Box::new(SelectorList {
                    span: args_span,
                    children: list_children,
                })),
            };
            let mut new_selectors = svelte_css::SimpleSelectorVec::new();
            new_selectors.push(wrapped_global);
            if i == inner_len - 1 {
                new_selectors.append(&mut trailing);
            }
            out.push(RelativeSelector {
                id: inner_id,
                span: inner_span,
                combinator,
                selectors: new_selectors,
            });
        }
    }
    node.children = out;
}

fn is_expandable_global(rel: &RelativeSelector) -> bool {
    let Some(SimpleSelector::Global {
        args: Some(args), ..
    }) = rel.selectors.first()
    else {
        return false;
    };
    if args.children.len() != 1 {
        return false;
    }
    let inner = &args.children[0];
    if inner.children.len() > 1 {
        return true;
    }
    if inner.children.len() == 1 {
        return matches!(
            inner.children[0].combinator,
            Some(Combinator { kind, .. }) if kind != CombinatorKind::Descendant
        );
    }
    false
}

fn promoted_first_combinator(
    outer: Option<Combinator>,
    inner: Option<Combinator>,
    inner_len: usize,
) -> Option<Combinator> {
    if inner_len == 1 {
        match (outer, inner) {
            (None, Some(c)) => Some(c),
            (Some(o), Some(i)) if o.kind == CombinatorKind::Descendant => Some(i),
            _ => outer,
        }
    } else {
        outer
    }
}

fn unwrap_global(complex: &mut ComplexSelector) {
    let outer_combinator = complex.children[0].combinator;
    let sel = complex.children[0].selectors.remove(0);
    if let SimpleSelector::Global {
        args: Some(args), ..
    } = sel
    {
        let mut new_children = svelte_css::RelativeSelectorVec::new();
        for inner_complex in args.children {
            new_children.extend(inner_complex.children);
        }
        if let Some(first) = new_children.first_mut()
            && first.combinator.is_none()
        {
            first.combinator = outer_combinator;
        }
        complex.children = new_children;
    }
}

fn pseudo_only_compound_is_scopable(selectors: &[SimpleSelector]) -> bool {
    match &selectors[0] {
        SimpleSelector::PseudoElement(pe) => {
            !svelte_css::is_view_transition_pseudo_element(pe.name.as_str())
        }
        SimpleSelector::PseudoClass(first) => {
            let name = first.name.as_str();
            if matches!(name, "root" | "host") {
                return false;
            }
            if selectors.len() == 1 && matches!(name, "is" | "where") {
                return false;
            }
            true
        }
        _ => true,
    }
}

fn complex_has_local_selector(complex: &ComplexSelector) -> bool {
    complex
        .children
        .iter()
        .any(|rel| rel.selectors.iter().any(is_scopable))
}

fn is_scopable(sel: &SimpleSelector) -> bool {
    matches!(
        sel,
        SimpleSelector::Type { .. }
            | SimpleSelector::Class { .. }
            | SimpleSelector::Id { .. }
            | SimpleSelector::Attribute(_)
    )
}

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
            if out.ends_with('}') || (j < n && chars[j] == '}') {
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
    use rustc_hash::FxHashSet;
    use svelte_css::{CssNodeId, StyleSheetChild};

    fn top_level_selector_ids(stylesheet: &StyleSheet) -> Vec<CssNodeId> {
        let mut ids = Vec::new();
        for child in &stylesheet.children {
            if let StyleSheetChild::Rule(Rule::Style(rule)) = child {
                ids.extend(rule.prelude.children.iter().map(|sel| sel.id));
            }
        }
        ids
    }

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
        assert!(
            !result.contains("svelte-abc123"),
            "global should not be scoped, got: {result}"
        );
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
        assert!(
            !result.contains(":global"),
            "global wrapper should be removed, got: {result}"
        );
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
    fn bare_global_unscopes_selector_tail() {
        let source = ".a :global .b .c { color: red; }";
        let (ss, _) = svelte_css::parse(source);
        let result = transform_css("svelte-abc123", &[], ss, source);
        assert!(
            result.contains(".a.svelte-abc123 .b .c"),
            "local prefix should stay scoped while the tail stays global, got: {result}"
        );
        assert!(
            !result.contains(".b.svelte-abc123") && !result.contains(".c.svelte-abc123"),
            "selectors after bare :global must stay unscoped, got: {result}"
        );
    }

    #[test]
    fn bare_global_at_start_keeps_selector_unscoped() {
        let source = ":global .page .title { margin: 0; }";
        let (ss, _) = svelte_css::parse(source);
        let result = transform_css("svelte-abc123", &[], ss, source);
        assert!(
            !result.contains("svelte-abc123"),
            "leading bare :global should keep the selector fully unscoped, got: {result}"
        );
        assert!(result.contains(".page .title"), "got: {result}");
    }

    #[test]
    fn scoped_inside_not() {
        let source = "p:not(.active) { color: red; }";
        let (ss, _) = svelte_css::parse(source);
        let result = transform_css("svelte-abc123", &[], ss, source);
        assert!(
            result.contains("p.svelte-abc123:not(.active)"),
            "simple selectors inside :not() should stay unscoped, got: {result}"
        );
    }

    #[test]
    fn complex_selector_inside_not_is_scoped() {
        let source = "section:not(.active .inner) { color: blue; }";
        let (ss, _) = svelte_css::parse(source);
        let result = transform_css("svelte-abc123", &[], ss, source);
        assert!(
            result.contains(
                "section.svelte-abc123:not(.active:where(.svelte-abc123) .inner:where(.svelte-abc123))"
            ),
            "complex selectors inside :not() should be scoped after the outer bump, got: {result}"
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

    #[test]
    fn external_mode_drops_unused_rules_and_selectors() {
        let source = ".used { color: red; }\n.unused { color: blue; }\n.used, .unused-mixed { border: 1px solid; }";
        let (ss, _) = svelte_css::parse(source);
        let selector_ids = top_level_selector_ids(&ss);
        let used = FxHashSet::from_iter([selector_ids[0], selector_ids[2]]);

        let result = transform_css_with_usage("svelte-abc123", &[], Some(&used), true, ss, source);

        assert!(result.contains(".used.svelte-abc123"), "got: {result}");
        assert!(
            !result.contains(".unused"),
            "fully unused rule should be removed, got: {result}"
        );
        assert!(
            !result.contains("unused-mixed"),
            "unused selector in a mixed list should be removed, got: {result}"
        );
    }

    #[test]
    fn injected_mode_removes_unused_rules_and_selectors_before_compaction() {
        let source = ".used { color: red; }\n.unused { color: blue; }\n.used, .unused-mixed { border: 1px solid; }";
        let (ss, _) = svelte_css::parse(source);
        let selector_ids = top_level_selector_ids(&ss);
        let used = FxHashSet::from_iter([selector_ids[0], selector_ids[2]]);

        let result = transform_css_with_usage("svelte-abc123", &[], Some(&used), true, ss, source);

        assert!(result.contains(".used.svelte-abc123"), "got: {result}");
        assert!(
            !result.contains(".unused") && !result.contains("unused-mixed"),
            "unused CSS should be removed in injected mode, got: {result}"
        );
    }
}
