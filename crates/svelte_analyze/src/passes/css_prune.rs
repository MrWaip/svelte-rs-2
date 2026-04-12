use compact_str::CompactString;
use oxc_ast::ast::{Expression, LogicalOperator, PropertyKey};
use rustc_hash::{FxHashMap, FxHashSet};
use smallvec::SmallVec;
use svelte_ast::{
    Attribute, Component as SvelteComponent, ConcatPart, Fragment, FragmentKey, Node, NodeId,
};
use svelte_css::{
    AtRule, Combinator, CombinatorKind, ComplexSelector, CssNodeId, PseudoClassSelector,
    RelativeSelector, SelectorList, SimpleSelector, StyleRule, StyleSheet, Visit,
};
use svelte_diagnostics::{Diagnostic, DiagnosticKind};
use svelte_parser::ParserResult;
use svelte_span::GetSpan;

use super::css_prune_index::{
    CandidateKind, CandidateNode, CssPruneIndex, NodeExistsValue, SiblingCandidate,
};
use crate::scope::SymbolId;
use crate::types::data::{
    ElementFacts, ExpressionInfo, ExpressionKind, NamespaceKind, ParentKind, TemplateAnalysis,
    TemplateElementIndex,
};
use crate::types::node_table::NodeBitSet;
use crate::AnalysisData;

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

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Forward,
    Backward,
}

struct SelectorPlan {
    complex_id: CssNodeId,
    relatives: Vec<RelativeSelector>,
    is_all_global: bool,
    needs_full_scan: bool,
}

struct RuleContext<'a> {
    rule: *const StyleRule,
    parent_rules: &'a [*const StyleRule],
    _marker: std::marker::PhantomData<&'a StyleRule>,
}

impl<'a> RuleContext<'a> {
    fn new(rule: &'a StyleRule, parent_rules: &'a [*const StyleRule]) -> Self {
        Self {
            rule: rule as *const StyleRule,
            parent_rules,
            _marker: std::marker::PhantomData,
        }
    }

    fn rule(&self) -> &'a StyleRule {
        unsafe { &*self.rule }
    }

    fn parent_rule(&self) -> Option<&'a StyleRule> {
        self.parent_rules.last().map(|ptr| unsafe { &**ptr })
    }

    fn parent_rules_without_last(&self) -> &'a [*const StyleRule] {
        self.parent_rules
            .split_last()
            .map_or(&[], |(_, parents)| parents)
    }
}

enum AttributeChunk<'a> {
    Text(&'a str),
    Expr(&'a Expression<'a>),
}

pub(crate) fn prune_and_warn(
    component: &SvelteComponent,
    stylesheet: &StyleSheet,
    css_source: &str,
    css_offset: u32,
    emit_warnings: bool,
    parsed: &ParserResult<'_>,
    data: &mut AnalysisData,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let elements = &data.template.template_elements;
    let element_facts = &data.elements.facts;
    let index = build_css_prune_index(component, data);
    let template = &data.template;
    let mut used = FxHashSet::default();
    let scoped = &mut data.output.css.scoped_elements;
    let mut pruner = PruneVisitor {
        component,
        css_source,
        parsed,
        template,
        elements,
        element_facts,
        index,
        used: &mut used,
        scoped,
        in_global_block: false,
        rule_stack: Vec::new(),
    };
    pruner.visit_stylesheet(stylesheet);
    data.output.css.used_selectors = used;

    if emit_warnings {
        warn_unused(
            stylesheet,
            css_source,
            css_offset,
            &data.output.css.used_selectors,
            diagnostics,
        );
    }
}

struct PruneVisitor<'a, 'b, 'p> {
    component: &'a SvelteComponent,
    css_source: &'a str,
    parsed: &'p ParserResult<'p>,
    template: &'b TemplateAnalysis,
    elements: &'b TemplateElementIndex,
    element_facts: &'b ElementFacts,
    index: CssPruneIndex,
    used: &'b mut FxHashSet<svelte_css::CssNodeId>,
    scoped: &'b mut NodeBitSet,
    in_global_block: bool,
    rule_stack: Vec<*const StyleRule>,
}

impl Visit for PruneVisitor<'_, '_, '_> {
    fn visit_at_rule(&mut self, node: &AtRule) {
        if node.name == "keyframes" {
            return;
        }
        svelte_css::visit::walk_at_rule(self, node);
    }

    fn visit_style_rule(&mut self, node: &StyleRule) {
        let was_global = self.in_global_block;
        self.in_global_block = was_global || node.is_lone_global_block();

        let parent_len = self.rule_stack.len();
        self.rule_stack.push(node as *const StyleRule);
        let parent_rules = self.rule_stack[..parent_len].to_vec();
        let rule_ctx = RuleContext::new(node, &parent_rules);

        for complex in &node.prelude.children {
            let plan = build_rule_selector_plan(complex, &rule_ctx);

            if self.in_global_block || plan.is_all_global {
                self.used.insert(plan.complex_id);
                continue;
            }

            if plan.relatives.is_empty() {
                self.used.insert(plan.complex_id);
                continue;
            }

            for elem_id in candidate_elements(
                self.elements,
                plan.relatives.last().unwrap(),
                plan.needs_full_scan,
            ) {
                if apply_selector(
                    self,
                    &plan.relatives,
                    &rule_ctx,
                    elem_id,
                    Direction::Backward,
                    0,
                    plan.relatives.len(),
                ) {
                    self.used.insert(plan.complex_id);
                }
            }
        }

        svelte_css::visit::walk_style_rule(self, node);
        self.rule_stack.truncate(parent_len);
        self.in_global_block = was_global;
    }
}

fn build_css_prune_index(component: &SvelteComponent, data: &AnalysisData) -> CssPruneIndex {
    let mut index = CssPruneIndex::new(component.store.len());
    collect_css_prune_edges_in_fragment(&component.fragment, component, data, &mut index);
    index
}

fn collect_css_prune_edges_in_fragment(
    fragment: &Fragment,
    component: &SvelteComponent,
    data: &AnalysisData,
    edges: &mut CssPruneIndex,
) {
    for &id in &fragment.nodes {
        match component.store.get(id) {
            Node::Element(el) => {
                collect_css_prune_edges_in_fragment(&el.fragment, component, data, edges);
            }
            Node::ComponentNode(node) => {
                let snippets = component_possible_snippets(node, data);
                record_possible_snippets(node.id, &snippets, edges, true);
                collect_css_prune_edges_in_fragment(&node.fragment, component, data, edges);
            }
            Node::IfBlock(block) => {
                collect_css_prune_edges_in_fragment(&block.consequent, component, data, edges);
                if let Some(alt) = &block.alternate {
                    collect_css_prune_edges_in_fragment(alt, component, data, edges);
                }
            }
            Node::EachBlock(block) => {
                collect_css_prune_edges_in_fragment(&block.body, component, data, edges);
                if let Some(fallback) = &block.fallback {
                    collect_css_prune_edges_in_fragment(fallback, component, data, edges);
                }
            }
            Node::SnippetBlock(block) => {
                collect_css_prune_edges_in_fragment(&block.body, component, data, edges);
            }
            Node::KeyBlock(block) => {
                collect_css_prune_edges_in_fragment(&block.fragment, component, data, edges);
            }
            Node::SvelteHead(head) => {
                collect_css_prune_edges_in_fragment(&head.fragment, component, data, edges);
            }
            Node::SvelteElement(node) => {
                collect_css_prune_edges_in_fragment(&node.fragment, component, data, edges);
            }
            Node::SvelteBoundary(node) => {
                collect_css_prune_edges_in_fragment(&node.fragment, component, data, edges);
            }
            Node::AwaitBlock(block) => {
                if let Some(pending) = &block.pending {
                    collect_css_prune_edges_in_fragment(pending, component, data, edges);
                }
                if let Some(then) = &block.then {
                    collect_css_prune_edges_in_fragment(then, component, data, edges);
                }
                if let Some(catch) = &block.catch {
                    collect_css_prune_edges_in_fragment(catch, component, data, edges);
                }
            }
            Node::RenderTag(tag) => {
                let snippets = render_tag_possible_snippets(tag.id, data);
                record_possible_snippets(tag.id, &snippets, edges, false);
            }
            Node::Text(_)
            | Node::Comment(_)
            | Node::ExpressionTag(_)
            | Node::DebugTag(_)
            | Node::HtmlTag(_)
            | Node::ConstTag(_)
            | Node::SvelteWindow(_)
            | Node::SvelteDocument(_)
            | Node::SvelteBody(_)
            | Node::Error(_) => {}
        }
    }
}

fn record_possible_snippets(
    site_id: NodeId,
    snippets: &[NodeId],
    index: &mut CssPruneIndex,
    is_component_site: bool,
) {
    if !snippets.is_empty() {
        if is_component_site {
            index.insert_component_possible_snippets(site_id, snippets.to_vec());
        } else {
            index.insert_render_possible_snippets(site_id, snippets.to_vec());
        }
    }
    for &snippet_id in snippets {
        let sites = index.snippet_sites_mut(snippet_id);
        if !sites.contains(&site_id) {
            sites.push(site_id);
        }
    }
}

fn render_tag_possible_snippets(id: NodeId, data: &AnalysisData) -> Vec<NodeId> {
    if let Some(sym_id) = data.blocks.render_tag_callee_sym.get(id).copied() {
        if let Some(snippet_id) = data.template.snippets.snippet_by_symbol(sym_id) {
            return vec![snippet_id];
        }
        if is_resolved_snippet_symbol(data, sym_id) {
            return Vec::new();
        }
    }

    data.template.snippets.local_snippets().to_vec()
}

fn component_possible_snippets(
    node: &svelte_ast::ComponentNode,
    data: &AnalysisData,
) -> Vec<NodeId> {
    let mut resolved = true;
    let mut snippets = Vec::new();

    for attr in &node.attributes {
        match attr {
            Attribute::SpreadAttribute(_) | Attribute::BindDirective(_) => {
                resolved = false;
            }
            Attribute::ExpressionAttribute(attr) => {
                if let Some(info) = data.attr_expression(attr.id) {
                    resolved &= collect_component_attr_snippets(data, info, &mut snippets);
                } else {
                    resolved = false;
                }
            }
            Attribute::Shorthand(attr) => {
                if let Some(info) = data.attr_expression(attr.id) {
                    resolved &= collect_component_attr_snippets(data, info, &mut snippets);
                } else {
                    resolved = false;
                }
            }
            Attribute::StringAttribute(_)
            | Attribute::BooleanAttribute(_)
            | Attribute::ConcatenationAttribute(_) => {}
            Attribute::ClassDirective(_)
            | Attribute::StyleDirective(_)
            | Attribute::OnDirectiveLegacy(_)
            | Attribute::UseDirective(_)
            | Attribute::TransitionDirective(_)
            | Attribute::AnimateDirective(_)
            | Attribute::AttachTag(_) => {}
        }
    }

    if resolved {
        for &snippet_id in data.template.snippets.component_snippets(node.id) {
            if !snippets.contains(&snippet_id) {
                snippets.push(snippet_id);
            }
        }
        return snippets;
    }

    data.template.snippets.local_snippets().to_vec()
}

fn collect_component_attr_snippets(
    data: &AnalysisData,
    info: &ExpressionInfo,
    snippets: &mut Vec<NodeId>,
) -> bool {
    match &info.kind {
        ExpressionKind::Identifier(name) => {
            let sym_id = info.ref_symbols.first().copied().or_else(|| {
                data.scoping
                    .find_binding(data.scoping.root_scope_id(), name.as_str())
            });
            let Some(sym_id) = sym_id else {
                return true;
            };
            if let Some(snippet_id) = data.template.snippets.snippet_by_symbol(sym_id) {
                if !snippets.contains(&snippet_id) {
                    snippets.push(snippet_id);
                }
                true
            } else {
                is_resolved_snippet_symbol(data, sym_id)
            }
        }
        ExpressionKind::Literal => true,
        _ => false,
    }
}

fn is_resolved_snippet_symbol(data: &AnalysisData, sym_id: SymbolId) -> bool {
    data.scoping.is_import(sym_id)
        || data.scoping.is_prop_source(sym_id)
        || data.scoping.is_rest_prop(sym_id)
        || data.template.snippets.snippet_by_symbol(sym_id).is_some()
}

fn build_rule_selector_plan(complex: &ComplexSelector, rule_ctx: &RuleContext<'_>) -> SelectorPlan {
    let mut relatives = build_truncated_relatives(complex);

    if rule_ctx.parent_rule().is_some()
        && !relatives.is_empty()
        && !contains_explicit_nesting(&relatives)
    {
        if relatives[0].combinator.is_none() {
            relatives[0].combinator = Some(synthetic_descendant_combinator());
        }
        relatives.insert(0, synthetic_nesting_relative());
    }

    SelectorPlan {
        complex_id: complex.id,
        needs_full_scan: relatives.last().is_some_and(requires_full_scan),
        is_all_global: is_all_global(&relatives, rule_ctx),
        relatives,
    }
}

fn build_truncated_relatives(complex: &ComplexSelector) -> Vec<RelativeSelector> {
    let end = complex
        .children
        .iter()
        .position(is_bare_global_relative)
        .unwrap_or_else(|| {
            complex
                .children
                .iter()
                .rposition(|rel| !is_global_relative(rel))
                .map_or(0, |idx| idx + 1)
        });

    let mut relatives = Vec::with_capacity(end);
    for child in &complex.children[..end] {
        let mut relative = child.clone();
        normalize_relative_selector(&mut relative);
        truncate_root_relative(&mut relative);
        relatives.push(relative);
    }
    relatives
}

fn truncate_root_relative(relative: &mut RelativeSelector) {
    let has_root = relative
        .selectors
        .iter()
        .any(|selector| matches!(selector, SimpleSelector::PseudoClass(pc) if pc.name == "root"));
    if !has_root || is_global_relative(relative) {
        return;
    }

    relative.selectors = relative
        .selectors
        .iter()
        .filter(|selector| matches!(selector, SimpleSelector::PseudoClass(pc) if pc.name == "has"))
        .cloned()
        .collect();
}

fn is_all_global(selectors: &[RelativeSelector], rule_ctx: &RuleContext<'_>) -> bool {
    selectors
        .iter()
        .all(|selector| is_global(selector, rule_ctx))
}

fn is_global_relative(rel: &RelativeSelector) -> bool {
    rel.selectors
        .first()
        .is_some_and(|selector| matches!(selector, SimpleSelector::Global { .. }))
        && rel.selectors.iter().all(|selector| match selector {
            SimpleSelector::Global { .. } => true,
            SimpleSelector::PseudoElement(_) => true,
            SimpleSelector::PseudoClass(pc) => is_unscoped_pseudo_class(pc, None),
            _ => false,
        })
}

fn is_bare_global_relative(rel: &RelativeSelector) -> bool {
    matches!(
        rel.selectors.first(),
        Some(SimpleSelector::Global { args: None, .. })
    )
}

fn is_outer_global(relative: &RelativeSelector) -> bool {
    relative
        .selectors
        .first()
        .is_some_and(|selector| matches!(selector, SimpleSelector::Global { .. }))
        && relative.selectors.iter().all(|selector| {
            matches!(
                selector,
                SimpleSelector::Global { .. }
                    | SimpleSelector::PseudoClass(_)
                    | SimpleSelector::PseudoElement(_)
            )
        })
}

fn is_global(selector: &RelativeSelector, rule_ctx: &RuleContext<'_>) -> bool {
    if is_global_relative(selector) {
        return true;
    }

    let mut explicitly_global = false;
    for simple in &selector.selectors {
        let mut selector_list: Option<&svelte_css::SelectorList> = None;
        let mut can_be_global = false;
        let mut nested_rule_ctx = None;

        match simple {
            SimpleSelector::PseudoClass(pc) => {
                if matches!(pc.name.as_str(), "is" | "where") {
                    selector_list = pc.args.as_ref().map(|args| &**args);
                } else {
                    can_be_global = is_unscoped_pseudo_class(pc, Some(rule_ctx));
                }
            }
            SimpleSelector::Nesting(_) => {
                let Some(parent_rule) = rule_ctx.parent_rule() else {
                    return false;
                };
                selector_list = Some(&parent_rule.prelude);
                nested_rule_ctx = Some(RuleContext::new(
                    parent_rule,
                    rule_ctx.parent_rules_without_last(),
                ));
            }
            _ => {}
        }

        let has_global_selectors = selector_list.is_some_and(|list| {
            let nested_ctx = nested_rule_ctx.as_ref().unwrap_or(rule_ctx);
            list.children.iter().any(|complex| {
                let relatives = build_truncated_relatives(complex);
                !relatives.is_empty()
                    && relatives
                        .iter()
                        .all(|relative| is_global(relative, nested_ctx))
            })
        });

        explicitly_global |= has_global_selectors;
        if !has_global_selectors && !can_be_global {
            return false;
        }
    }

    explicitly_global || selector.selectors.is_empty()
}

fn is_unscoped_pseudo_class(pc: &PseudoClassSelector, rule_ctx: Option<&RuleContext<'_>>) -> bool {
    if pc.name != "has" && pc.name != "is" && pc.name != "where" {
        if pc.name != "not" {
            return true;
        }

        let Some(args) = &pc.args else {
            return true;
        };
        if args
            .children
            .iter()
            .all(|complex| complex.children.len() == 1)
        {
            return true;
        }
    }

    let Some(args) = &pc.args else {
        return false;
    };
    let Some(rule_ctx) = rule_ctx else {
        return false;
    };

    args.children.iter().all(|complex| {
        let relatives = build_truncated_relatives(complex);
        relatives
            .iter()
            .all(|relative| is_global(relative, rule_ctx))
    })
}

fn contains_explicit_nesting(relatives: &[RelativeSelector]) -> bool {
    relatives
        .iter()
        .any(|relative| relative_has_explicit_nesting(relative))
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
    list.children
        .iter()
        .any(|complex| complex.children.iter().any(relative_has_explicit_nesting))
}

fn normalize_relative_selector(relative: &mut RelativeSelector) {
    for selector in relative.selectors.iter_mut() {
        normalize_simple_selector(selector);
    }
}

fn normalize_simple_selector(selector: &mut SimpleSelector) {
    match selector {
        SimpleSelector::Type { name, .. }
        | SimpleSelector::Id { name, .. }
        | SimpleSelector::Class { name, .. } => {
            *name = normalize_css_identifier(name.as_str());
        }
        SimpleSelector::PseudoClass(pc) => {
            pc.name = normalize_css_identifier(pc.name.as_str());
        }
        SimpleSelector::PseudoElement(pe) => {
            pe.name = normalize_css_identifier(pe.name.as_str());
        }
        SimpleSelector::Attribute(_) => {}
        SimpleSelector::Global { .. }
        | SimpleSelector::Nesting(_)
        | SimpleSelector::Nth(_)
        | SimpleSelector::Percentage(_) => {}
    }
}

fn normalize_css_identifier(value: &str) -> CompactString {
    if !value.contains('\\') {
        return CompactString::new(value);
    }

    let mut normalized = String::with_capacity(value.len());
    let mut chars = value.chars();
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            if let Some(next) = chars.next() {
                normalized.push(next);
            }
        } else {
            normalized.push(ch);
        }
    }
    CompactString::new(normalized)
}

fn synthetic_descendant_combinator() -> Combinator {
    Combinator {
        kind: CombinatorKind::Descendant,
        span: svelte_span::Span::new(0, 0),
    }
}

fn synthetic_nesting_relative() -> RelativeSelector {
    RelativeSelector {
        id: CssNodeId(0),
        span: svelte_span::Span::new(0, 0),
        combinator: None,
        selectors: vec![SimpleSelector::Nesting(svelte_span::Span::new(0, 0))].into(),
    }
}

fn apply_selector(
    pruner: &mut PruneVisitor<'_, '_, '_>,
    selectors: &[RelativeSelector],
    rule_ctx: &RuleContext<'_>,
    elem_id: NodeId,
    direction: Direction,
    from: usize,
    to: usize,
) -> bool {
    if from >= to {
        return false;
    }

    let selector_index = match direction {
        Direction::Forward => from,
        Direction::Backward => to - 1,
    };
    let relative_selector = &selectors[selector_index];
    let (rest_from, rest_to) = match direction {
        Direction::Forward => (from + 1, to),
        Direction::Backward => (from, to - 1),
    };

    let matched =
        relative_selector_matches(pruner, &relative_selector, rule_ctx, elem_id, direction)
            && apply_combinator(
                pruner,
                &relative_selector,
                selectors,
                rest_from,
                rest_to,
                rule_ctx,
                elem_id,
                direction,
            );

    if matched {
        if !is_outer_global(&relative_selector) {
            pruner.scoped.insert(elem_id);
        }
    }

    matched
}

fn apply_combinator(
    pruner: &mut PruneVisitor<'_, '_, '_>,
    relative_selector: &RelativeSelector,
    selectors: &[RelativeSelector],
    from: usize,
    to: usize,
    rule_ctx: &RuleContext<'_>,
    node_id: NodeId,
    direction: Direction,
) -> bool {
    let combinator = match direction {
        Direction::Forward => {
            if from < to {
                selectors[from].combinator.as_ref()
            } else {
                None
            }
        }
        Direction::Backward => relative_selector.combinator.as_ref(),
    };
    let Some(combinator) = combinator else {
        return true;
    };

    match combinator.kind {
        CombinatorKind::Descendant | CombinatorKind::Child => {
            let adjacent_only = combinator.kind == CombinatorKind::Child;
            let candidates = match direction {
                Direction::Forward => get_descendant_elements(pruner, node_id, adjacent_only),
                Direction::Backward => get_ancestor_elements(pruner, node_id, adjacent_only),
            };

            let mut matched = false;
            let no_candidates = candidates.is_empty();
            for candidate in &candidates {
                if apply_selector(pruner, selectors, rule_ctx, *candidate, direction, from, to) {
                    matched = true;
                }
            }

            matched
                || (matches!(direction, Direction::Backward)
                    && (combinator.kind != CombinatorKind::Child || no_candidates)
                    && every_is_global(selectors, from, to, rule_ctx))
        }
        CombinatorKind::NextSibling | CombinatorKind::SubsequentSibling => {
            let siblings = get_possible_element_siblings(
                pruner,
                node_id,
                direction,
                combinator.kind == CombinatorKind::NextSibling,
            );
            let mut matched = false;

            for (candidate, _) in siblings {
                match candidate.kind {
                    CandidateKind::RenderTag | CandidateKind::ComponentNode => {
                        if to > from
                            && to - from == 1
                            && matches!(direction, Direction::Backward)
                            && is_global(&selectors[from], rule_ctx)
                        {
                            matched = true;
                        }
                    }
                    CandidateKind::Element | CandidateKind::SvelteElement => {
                        if apply_selector(
                            pruner,
                            selectors,
                            rule_ctx,
                            candidate.id,
                            direction,
                            from,
                            to,
                        ) {
                            matched = true;
                        }
                    }
                }
            }

            matched
                || (matches!(direction, Direction::Backward)
                    && get_element_parent(pruner, node_id).is_none()
                    && every_is_global(selectors, from, to, rule_ctx))
        }
        CombinatorKind::Column => true,
    }
}

fn every_is_global(
    selectors: &[RelativeSelector],
    from: usize,
    to: usize,
    rule_ctx: &RuleContext<'_>,
) -> bool {
    selectors[from..to]
        .iter()
        .all(|selector| is_global(selector, rule_ctx))
}

fn relative_selector_matches(
    pruner: &mut PruneVisitor<'_, '_, '_>,
    relative_selector: &RelativeSelector,
    rule_ctx: &RuleContext<'_>,
    elem_id: NodeId,
    direction: Direction,
) -> bool {
    let mut include_self = None;

    for selector in &relative_selector.selectors {
        match selector {
            SimpleSelector::PseudoClass(pc) if pc.name == "has" && pc.args.is_some() => {
                let include_self =
                    *include_self.get_or_insert_with(|| has_global_or_root_selector(rule_ctx));
                let mut matched = false;

                for complex in &pc.args.as_ref().unwrap().children {
                    let inner = build_truncated_relatives(complex);
                    if inner.is_empty() {
                        pruner.used.insert(complex.id);
                        matched = true;
                        continue;
                    }

                    if include_self {
                        let mut including_self = inner.clone();
                        if let Some(first) = including_self.first_mut() {
                            if first.combinator.is_some() {
                                *first = RelativeSelector {
                                    id: svelte_css::CssNodeId(0),
                                    combinator: None,
                                    selectors: first.selectors.clone(),
                                    span: first.span,
                                };
                            }
                        }
                        if apply_selector(
                            pruner,
                            &including_self,
                            rule_ctx,
                            elem_id,
                            Direction::Forward,
                            0,
                            including_self.len(),
                        ) {
                            pruner.used.insert(complex.id);
                            matched = true;
                        }
                    }

                    let mut excluding_self = Vec::with_capacity(inner.len() + 1);
                    excluding_self.push(RelativeSelector {
                        id: svelte_css::CssNodeId(0),
                        combinator: None,
                        selectors: vec![SimpleSelector::Type {
                            span: svelte_span::Span::new(0, 0),
                            name: "*".into(),
                        }]
                        .into(),
                        span: svelte_span::Span::new(0, 0),
                    });
                    let first = inner[0].clone();
                    let first_with_descendant = if first.combinator.is_some() {
                        first
                    } else {
                        RelativeSelector {
                            id: svelte_css::CssNodeId(0),
                            combinator: Some(svelte_css::Combinator {
                                kind: CombinatorKind::Descendant,
                                span: svelte_span::Span::new(0, 0),
                            }),
                            selectors: first.selectors.clone(),
                            span: first.span,
                        }
                    };
                    excluding_self.push(first_with_descendant);
                    excluding_self.extend_from_slice(&inner[1..]);

                    if apply_selector(
                        pruner,
                        &excluding_self,
                        rule_ctx,
                        elem_id,
                        Direction::Forward,
                        0,
                        excluding_self.len(),
                    ) {
                        pruner.used.insert(complex.id);
                        matched = true;
                    }
                }

                if !matched {
                    return false;
                }
            }
            SimpleSelector::Type { name, .. } => {
                let Some(tag_name) = pruner.elements.tag_name(elem_id) else {
                    return false;
                };
                if name.as_str() != "*" && !tag_name.eq_ignore_ascii_case(name.as_str()) {
                    return false;
                }
            }
            SimpleSelector::Class { name, .. } => {
                if !attribute_matches(pruner, elem_id, "class", Some(name.as_str()), "~=", false) {
                    return false;
                }
            }
            SimpleSelector::Id { name, .. } => {
                if !attribute_matches(pruner, elem_id, "id", Some(name.as_str()), "=", false) {
                    return false;
                }
            }
            SimpleSelector::Global {
                args: Some(args), ..
            } if relative_selector.selectors.len() == 1 => {
                if let Some(complex) = args.children.first() {
                    let inner = build_truncated_relatives(complex);
                    if !inner.is_empty()
                        && !apply_selector(
                            pruner,
                            &inner,
                            rule_ctx,
                            elem_id,
                            Direction::Backward,
                            0,
                            inner.len(),
                        )
                    {
                        return false;
                    }
                }
            }
            SimpleSelector::Global { args: Some(_), .. } => {}
            SimpleSelector::Global { args: None, .. } => return true,
            SimpleSelector::PseudoClass(pc) => match pc.name.as_str() {
                "host" | "root" => return false,
                "not" => {
                    if let Some(args) = &pc.args {
                        for complex in &args.children {
                            pruner.used.insert(complex.id);
                            if complex.children.len() > 1 {
                                for ancestor in std::iter::once(elem_id)
                                    .chain(get_ancestor_elements(pruner, elem_id, false))
                                {
                                    pruner.scoped.insert(ancestor);
                                }
                                return true;
                            }
                        }
                    }
                }
                "is" | "where" => {
                    if !pseudo_is_or_where_matches(pruner, pc, rule_ctx, elem_id) {
                        return false;
                    }
                }
                _ => {}
            },
            SimpleSelector::Attribute(attr) => {
                let whitelisted = matches!(
                    pruner.elements.tag_name(elem_id).map(|name| name.to_ascii_lowercase()),
                    Some(tag_name) if (tag_name == "details" || tag_name == "dialog")
                        && attr.name.eq_ignore_ascii_case("open")
                );
                if !whitelisted
                    && !attribute_matches(
                        pruner,
                        elem_id,
                        attr.name.as_str(),
                        attr.value
                            .map(|span| unquote(span.source_text(pruner.css_source))),
                        attr.matcher
                            .map(|span| span.source_text(pruner.css_source).trim())
                            .unwrap_or(""),
                        attribute_selector_case_insensitive(
                            pruner.element_facts,
                            elem_id,
                            attr.name.as_str(),
                            attr.flags
                                .map(|span| span.source_text(pruner.css_source).trim()),
                        ),
                    )
                {
                    return false;
                }
            }
            SimpleSelector::Nesting(_) => {
                let Some(parent_rule) = rule_ctx.parent_rule() else {
                    return false;
                };
                let parent_rule_ctx =
                    RuleContext::new(parent_rule, rule_ctx.parent_rules_without_last());
                let mut matched = false;
                for complex in &parent_rule.prelude.children {
                    let parent_plan = build_rule_selector_plan(complex, &parent_rule_ctx);
                    if apply_selector(
                        pruner,
                        &parent_plan.relatives,
                        &parent_rule_ctx,
                        elem_id,
                        direction,
                        0,
                        parent_plan.relatives.len(),
                    ) || parent_plan.is_all_global
                    {
                        pruner.used.insert(complex.id);
                        matched = true;
                    }
                }
                if !matched {
                    return false;
                }
            }
            SimpleSelector::PseudoElement(_)
            | SimpleSelector::Nth(_)
            | SimpleSelector::Percentage(_) => {}
        }
    }

    true
}

fn pseudo_is_or_where_matches(
    pruner: &mut PruneVisitor<'_, '_, '_>,
    pseudo: &PseudoClassSelector,
    rule_ctx: &RuleContext<'_>,
    elem_id: NodeId,
) -> bool {
    let Some(args) = &pseudo.args else {
        return true;
    };

    let mut matched = false;
    for complex in &args.children {
        let selectors = build_truncated_relatives(complex);
        if selectors.is_empty() {
            pruner.used.insert(complex.id);
            matched = true;
            continue;
        }

        if apply_selector(
            pruner,
            &selectors,
            rule_ctx,
            elem_id,
            Direction::Backward,
            0,
            selectors.len(),
        ) {
            pruner.used.insert(complex.id);
            matched = true;
            continue;
        }

        if complex.children.len() > 1 {
            pruner.used.insert(complex.id);
            matched = true;
            for ancestor in
                std::iter::once(elem_id).chain(get_ancestor_elements(pruner, elem_id, false))
            {
                pruner.scoped.insert(ancestor);
            }
        }
    }

    matched
}

fn has_global_or_root_selector(rule_ctx: &RuleContext<'_>) -> bool {
    if rule_ctx
        .parent_rules
        .iter()
        .copied()
        .chain(std::iter::once(rule_ctx.rule))
        .any(|rule_ptr| {
            let rule = unsafe { &*rule_ptr };
            let nested_ctx = RuleContext::new(rule, &[]);
            rule.prelude.children.iter().any(|complex| {
                let selectors = build_truncated_relatives(complex);
                selectors
                    .iter()
                    .any(|selector| is_global(selector, &nested_ctx))
            })
        })
    {
        return true;
    }

    let rule = rule_ctx.rule();
    rule.prelude.children.iter().any(|complex| {
        complex.children.iter().any(|relative| {
            relative.selectors.iter().any(|selector| match selector {
                SimpleSelector::PseudoClass(pc) => pc.name == "root",
                SimpleSelector::Global { args: Some(_), .. } => true,
                _ => false,
            })
        })
    })
}

fn get_ancestor_elements(
    pruner: &mut PruneVisitor<'_, '_, '_>,
    node_id: NodeId,
    adjacent_only: bool,
) -> Vec<NodeId> {
    if let Some(cached) = pruner.index.cached_ancestors(node_id, adjacent_only) {
        return cached.to_vec();
    }

    let mut ancestors = Vec::new();
    let mut seen = FxHashSet::default();
    collect_ancestor_elements(pruner, node_id, adjacent_only, &mut seen, &mut ancestors);
    pruner
        .index
        .store_ancestors(node_id, adjacent_only, ancestors.clone());
    ancestors
}

fn collect_ancestor_elements(
    pruner: &mut PruneVisitor<'_, '_, '_>,
    node_id: NodeId,
    adjacent_only: bool,
    seen_snippets: &mut FxHashSet<NodeId>,
    out: &mut Vec<NodeId>,
) {
    let mut current = pruner.template.template_topology.parent(node_id);

    while let Some(parent) = current {
        if parent.kind == ParentKind::SnippetBlock {
            if seen_snippets.insert(parent.id) {
                let site_ids = pruner.index.snippet_sites(parent.id).to_vec();
                for site_id in site_ids {
                    collect_ancestor_elements(pruner, site_id, adjacent_only, seen_snippets, out);
                }
            }
            break;
        }

        if parent.kind.is_element() {
            out.push(parent.id);
            if adjacent_only {
                break;
            }
        }

        current = pruner.template.template_topology.parent(parent.id);
    }
}

fn get_descendant_elements(
    pruner: &mut PruneVisitor<'_, '_, '_>,
    node_id: NodeId,
    adjacent_only: bool,
) -> Vec<NodeId> {
    if let Some(cached) = pruner.index.cached_descendants(node_id, adjacent_only) {
        return cached.to_vec();
    }

    let mut descendants = Vec::new();
    let mut seen_snippets = FxHashSet::default();
    collect_descendants_from_node(
        pruner,
        node_id,
        adjacent_only,
        &mut seen_snippets,
        &mut descendants,
    );
    pruner
        .index
        .store_descendants(node_id, adjacent_only, descendants.clone());
    descendants
}

fn collect_descendants_from_node(
    pruner: &mut PruneVisitor<'_, '_, '_>,
    node_id: NodeId,
    adjacent_only: bool,
    seen_snippets: &mut FxHashSet<NodeId>,
    out: &mut Vec<NodeId>,
) {
    match pruner.component.store.get(node_id) {
        Node::Element(_) => collect_descendants_from_fragment(
            pruner,
            FragmentKey::Element(node_id),
            adjacent_only,
            seen_snippets,
            out,
        ),
        Node::SvelteElement(_) => collect_descendants_from_fragment(
            pruner,
            FragmentKey::SvelteElementBody(node_id),
            adjacent_only,
            seen_snippets,
            out,
        ),
        Node::ComponentNode(_) => collect_descendants_from_fragment(
            pruner,
            FragmentKey::ComponentNode(node_id),
            adjacent_only,
            seen_snippets,
            out,
        ),
        Node::SnippetBlock(_) => collect_descendants_from_fragment(
            pruner,
            FragmentKey::SnippetBody(node_id),
            adjacent_only,
            seen_snippets,
            out,
        ),
        Node::RenderTag(_) => {
            let snippet_ids = pruner.index.render_possible_snippets(node_id).to_vec();
            for snippet_id in snippet_ids {
                if seen_snippets.insert(snippet_id) {
                    collect_descendants_from_fragment(
                        pruner,
                        FragmentKey::SnippetBody(snippet_id),
                        adjacent_only,
                        seen_snippets,
                        out,
                    );
                }
            }
        }
        _ => {}
    }
}

fn collect_descendants_from_fragment(
    pruner: &mut PruneVisitor<'_, '_, '_>,
    key: FragmentKey,
    adjacent_only: bool,
    seen_snippets: &mut FxHashSet<NodeId>,
    out: &mut Vec<NodeId>,
) {
    let Some(lowered) = pruner.template.fragments.lowered(&key) else {
        return;
    };

    for item in &lowered.items {
        match item {
            crate::types::data::FragmentItem::Element(id) => {
                out.push(*id);
                if !adjacent_only {
                    collect_descendants_from_node(pruner, *id, false, seen_snippets, out);
                }
            }
            crate::types::data::FragmentItem::SvelteElement(id) => {
                out.push(*id);
                if !adjacent_only {
                    collect_descendants_from_node(pruner, *id, false, seen_snippets, out);
                }
            }
            crate::types::data::FragmentItem::RenderTag(id) => {
                collect_descendants_from_node(pruner, *id, adjacent_only, seen_snippets, out);
            }
            crate::types::data::FragmentItem::ComponentNode(id) => {
                collect_descendants_from_node(pruner, *id, adjacent_only, seen_snippets, out);
            }
            crate::types::data::FragmentItem::IfBlock(_)
            | crate::types::data::FragmentItem::EachBlock(_)
            | crate::types::data::FragmentItem::HtmlTag(_)
            | crate::types::data::FragmentItem::KeyBlock(_)
            | crate::types::data::FragmentItem::SvelteBoundary(_)
            | crate::types::data::FragmentItem::AwaitBlock(_)
            | crate::types::data::FragmentItem::TextConcat { .. } => {}
        }
    }
}

fn get_element_parent(pruner: &PruneVisitor<'_, '_, '_>, node_id: NodeId) -> Option<NodeId> {
    pruner
        .template
        .template_topology
        .ancestors(node_id)
        .find(|parent| parent.kind.is_element())
        .map(|parent| parent.id)
}

fn get_possible_element_siblings(
    pruner: &mut PruneVisitor<'_, '_, '_>,
    node_id: NodeId,
    direction: Direction,
    adjacent_only: bool,
) -> FxHashMap<CandidateNode, NodeExistsValue> {
    if let Some(cached) = pruner.index.cached_siblings(
        node_id,
        matches!(direction, Direction::Forward),
        adjacent_only,
    ) {
        return sibling_candidates_to_map(cached);
    }

    let mut result = FxHashMap::default();
    let mut seen_snippets = FxHashSet::default();
    collect_possible_siblings(
        pruner,
        node_id,
        direction,
        adjacent_only,
        &mut seen_snippets,
        &mut result,
    );
    pruner.index.store_siblings(
        node_id,
        matches!(direction, Direction::Forward),
        adjacent_only,
        map_to_sibling_candidates(&result),
    );
    result
}

fn collect_possible_siblings(
    pruner: &mut PruneVisitor<'_, '_, '_>,
    node_id: NodeId,
    direction: Direction,
    adjacent_only: bool,
    seen_snippets: &mut FxHashSet<NodeId>,
    out: &mut FxHashMap<CandidateNode, NodeExistsValue>,
) {
    let mut current_id = node_id;
    let mut current_fragment = pruner.template.template_topology.node_fragment(node_id);

    while let Some(fragment_key) = current_fragment {
        let Some(lowered) = pruner.template.fragments.lowered(&fragment_key) else {
            break;
        };
        let Some(current_index) = lowered
            .items
            .iter()
            .position(|item| fragment_item_matches_node_id(item, current_id))
        else {
            break;
        };

        let mut indices: Box<dyn Iterator<Item = usize>> = match direction {
            Direction::Forward => Box::new(current_index + 1..lowered.items.len()),
            Direction::Backward => Box::new((0..current_index).rev()),
        };

        while let Some(index) = indices.next() {
            match &lowered.items[index] {
                crate::types::data::FragmentItem::Element(id) => {
                    add_candidate(
                        out,
                        CandidateNode {
                            id: *id,
                            kind: CandidateKind::Element,
                        },
                        NodeExistsValue::Definitely,
                    );
                    if adjacent_only {
                        return;
                    }
                }
                crate::types::data::FragmentItem::SvelteElement(id) => {
                    add_candidate(
                        out,
                        CandidateNode {
                            id: *id,
                            kind: CandidateKind::SvelteElement,
                        },
                        NodeExistsValue::Probably,
                    );
                }
                crate::types::data::FragmentItem::IfBlock(id)
                | crate::types::data::FragmentItem::EachBlock(id)
                | crate::types::data::FragmentItem::KeyBlock(id)
                | crate::types::data::FragmentItem::AwaitBlock(id)
                | crate::types::data::FragmentItem::SvelteBoundary(id) => {
                    let nested = get_possible_nested_siblings(
                        pruner,
                        *id,
                        direction,
                        adjacent_only,
                        seen_snippets,
                    );
                    add_all_candidates(&nested, out);
                    if adjacent_only && has_definite_elements(&nested) {
                        return;
                    }
                }
                crate::types::data::FragmentItem::ComponentNode(id) => {
                    add_candidate(
                        out,
                        CandidateNode {
                            id: *id,
                            kind: CandidateKind::ComponentNode,
                        },
                        NodeExistsValue::Probably,
                    );
                    let nested = get_possible_nested_siblings(
                        pruner,
                        *id,
                        direction,
                        adjacent_only,
                        seen_snippets,
                    );
                    add_all_candidates(&nested, out);
                }
                crate::types::data::FragmentItem::RenderTag(id) => {
                    add_candidate(
                        out,
                        CandidateNode {
                            id: *id,
                            kind: CandidateKind::RenderTag,
                        },
                        NodeExistsValue::Probably,
                    );
                    let snippet_ids = pruner.index.render_possible_snippets(*id).to_vec();
                    for snippet_id in snippet_ids {
                        let nested = get_possible_nested_siblings(
                            pruner,
                            snippet_id,
                            direction,
                            adjacent_only,
                            seen_snippets,
                        );
                        add_all_candidates(&nested, out);
                    }
                }
                crate::types::data::FragmentItem::HtmlTag(_)
                | crate::types::data::FragmentItem::TextConcat { .. } => {}
            }
        }

        let Some(owner_id) = fragment_key.node_id() else {
            break;
        };
        current_id = owner_id;

        match pruner.component.store.get(owner_id) {
            Node::ComponentNode(_) => {
                current_fragment = pruner.template.template_topology.node_fragment(owner_id);
                continue;
            }
            Node::SnippetBlock(_) => {
                if seen_snippets.insert(owner_id) {
                    let site_ids = pruner.index.snippet_sites(owner_id).to_vec();
                    for site_id in &site_ids {
                        let siblings = get_possible_element_siblings(
                            pruner,
                            *site_id,
                            direction,
                            adjacent_only,
                        );
                        add_all_candidates(&siblings, out);
                        if adjacent_only && site_ids.len() == 1 && has_definite_elements(&siblings)
                        {
                            return;
                        }
                    }
                }
                break;
            }
            Node::EachBlock(_) if matches!(fragment_key, FragmentKey::EachBody(_)) => {
                let nested = get_possible_nested_siblings(
                    pruner,
                    owner_id,
                    direction,
                    adjacent_only,
                    seen_snippets,
                );
                add_all_candidates(&nested, out);
            }
            _ => {}
        }

        if !is_block_like(pruner.component.store.get(owner_id)) {
            break;
        }
        current_fragment = pruner.template.template_topology.node_fragment(owner_id);
    }
}

fn get_possible_nested_siblings(
    pruner: &mut PruneVisitor<'_, '_, '_>,
    node_id: NodeId,
    direction: Direction,
    adjacent_only: bool,
    seen_snippets: &mut FxHashSet<NodeId>,
) -> FxHashMap<CandidateNode, NodeExistsValue> {
    let mut fragments = Vec::new();
    let mut exhaustive = true;

    match pruner.component.store.get(node_id) {
        Node::EachBlock(_) => {
            fragments.push(FragmentKey::EachBody(node_id));
            fragments.push(FragmentKey::EachFallback(node_id));
        }
        Node::IfBlock(_) => {
            fragments.push(FragmentKey::IfConsequent(node_id));
            fragments.push(FragmentKey::IfAlternate(node_id));
        }
        Node::AwaitBlock(_) => {
            fragments.push(FragmentKey::AwaitPending(node_id));
            fragments.push(FragmentKey::AwaitThen(node_id));
            fragments.push(FragmentKey::AwaitCatch(node_id));
        }
        Node::KeyBlock(_) => {
            fragments.push(FragmentKey::KeyBlockBody(node_id));
        }
        Node::SnippetBlock(_) => {
            if !seen_snippets.insert(node_id) {
                return FxHashMap::default();
            }
            exhaustive = false;
            fragments.push(FragmentKey::SnippetBody(node_id));
        }
        Node::ComponentNode(_) => {
            fragments.push(FragmentKey::ComponentNode(node_id));
            let snippet_ids = pruner.index.component_possible_snippets(node_id).to_vec();
            for snippet_id in snippet_ids {
                fragments.push(FragmentKey::SnippetBody(snippet_id));
            }
        }
        Node::SvelteBoundary(_) => {
            fragments.push(FragmentKey::SvelteBoundaryBody(node_id));
        }
        _ => return FxHashMap::default(),
    }

    let mut result = FxHashMap::default();
    for fragment_key in fragments {
        if pruner.template.fragments.lowered(&fragment_key).is_none() {
            exhaustive = false;
            continue;
        }
        let map = loop_child(
            pruner,
            fragment_key,
            direction,
            adjacent_only,
            seen_snippets,
        );
        exhaustive &= has_definite_elements(&map);
        add_all_candidates(&map, &mut result);
    }

    if !exhaustive {
        for exist in result.values_mut() {
            *exist = NodeExistsValue::Probably;
        }
    }

    result
}

fn loop_child(
    pruner: &mut PruneVisitor<'_, '_, '_>,
    fragment_key: FragmentKey,
    direction: Direction,
    adjacent_only: bool,
    seen_snippets: &mut FxHashSet<NodeId>,
) -> FxHashMap<CandidateNode, NodeExistsValue> {
    let mut result = FxHashMap::default();
    let Some(lowered) = pruner.template.fragments.lowered(&fragment_key) else {
        return result;
    };

    let mut indices: Box<dyn Iterator<Item = usize>> = match direction {
        Direction::Forward => Box::new(0..lowered.items.len()),
        Direction::Backward => Box::new((0..lowered.items.len()).rev()),
    };

    while let Some(index) = indices.next() {
        match &lowered.items[index] {
            crate::types::data::FragmentItem::Element(id) => {
                add_candidate(
                    &mut result,
                    CandidateNode {
                        id: *id,
                        kind: CandidateKind::Element,
                    },
                    NodeExistsValue::Definitely,
                );
                if adjacent_only {
                    break;
                }
            }
            crate::types::data::FragmentItem::SvelteElement(id) => {
                add_candidate(
                    &mut result,
                    CandidateNode {
                        id: *id,
                        kind: CandidateKind::SvelteElement,
                    },
                    NodeExistsValue::Probably,
                );
            }
            crate::types::data::FragmentItem::RenderTag(id) => {
                let snippet_ids = pruner.index.render_possible_snippets(*id).to_vec();
                for snippet_id in snippet_ids {
                    let nested = get_possible_nested_siblings(
                        pruner,
                        snippet_id,
                        direction,
                        adjacent_only,
                        seen_snippets,
                    );
                    add_all_candidates(&nested, &mut result);
                }
            }
            crate::types::data::FragmentItem::IfBlock(id)
            | crate::types::data::FragmentItem::EachBlock(id)
            | crate::types::data::FragmentItem::KeyBlock(id)
            | crate::types::data::FragmentItem::AwaitBlock(id)
            | crate::types::data::FragmentItem::SvelteBoundary(id) => {
                let nested = get_possible_nested_siblings(
                    pruner,
                    *id,
                    direction,
                    adjacent_only,
                    seen_snippets,
                );
                add_all_candidates(&nested, &mut result);
                if adjacent_only && has_definite_elements(&nested) {
                    break;
                }
            }
            crate::types::data::FragmentItem::ComponentNode(_)
            | crate::types::data::FragmentItem::HtmlTag(_)
            | crate::types::data::FragmentItem::TextConcat { .. } => {}
        }
    }

    result
}

fn add_candidate(
    map: &mut FxHashMap<CandidateNode, NodeExistsValue>,
    candidate: CandidateNode,
    exist: NodeExistsValue,
) {
    map.entry(candidate)
        .and_modify(|current| {
            if exist > *current {
                *current = exist;
            }
        })
        .or_insert(exist);
}

fn add_all_candidates(
    from: &FxHashMap<CandidateNode, NodeExistsValue>,
    to: &mut FxHashMap<CandidateNode, NodeExistsValue>,
) {
    for (&candidate, &exist) in from {
        add_candidate(to, candidate, exist);
    }
}

fn map_to_sibling_candidates(
    map: &FxHashMap<CandidateNode, NodeExistsValue>,
) -> Vec<SiblingCandidate> {
    map.iter()
        .map(|(&node, &existence)| SiblingCandidate { node, existence })
        .collect()
}

fn sibling_candidates_to_map(
    candidates: &[SiblingCandidate],
) -> FxHashMap<CandidateNode, NodeExistsValue> {
    let mut map = FxHashMap::default();
    for candidate in candidates {
        add_candidate(&mut map, candidate.node, candidate.existence);
    }
    map
}

fn fragment_item_matches_node_id(item: &crate::types::data::FragmentItem, node_id: NodeId) -> bool {
    match item {
        crate::types::data::FragmentItem::Element(id)
        | crate::types::data::FragmentItem::ComponentNode(id)
        | crate::types::data::FragmentItem::IfBlock(id)
        | crate::types::data::FragmentItem::EachBlock(id)
        | crate::types::data::FragmentItem::RenderTag(id)
        | crate::types::data::FragmentItem::HtmlTag(id)
        | crate::types::data::FragmentItem::KeyBlock(id)
        | crate::types::data::FragmentItem::SvelteElement(id)
        | crate::types::data::FragmentItem::SvelteBoundary(id)
        | crate::types::data::FragmentItem::AwaitBlock(id) => *id == node_id,
        crate::types::data::FragmentItem::TextConcat { .. } => false,
    }
}

fn has_definite_elements(map: &FxHashMap<CandidateNode, NodeExistsValue>) -> bool {
    map.values()
        .any(|exist| matches!(exist, NodeExistsValue::Definitely))
}

fn is_block_like(node: &Node) -> bool {
    matches!(
        node,
        Node::IfBlock(_)
            | Node::EachBlock(_)
            | Node::AwaitBlock(_)
            | Node::KeyBlock(_)
            | Node::SvelteBoundary(_)
    )
}

fn candidate_elements(
    elements: &TemplateElementIndex,
    selector: &RelativeSelector,
    needs_full_scan: bool,
) -> SmallVec<[NodeId; 8]> {
    if needs_full_scan {
        return SmallVec::from_slice(elements.all_elements());
    }

    for simple in &selector.selectors {
        match simple {
            SimpleSelector::Id { name, .. } => {
                return elements.id_candidates(name.as_str()).collect();
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

fn requires_full_scan(selector: &RelativeSelector) -> bool {
    selector.selectors.iter().any(|simple| {
        matches!(simple, SimpleSelector::Nesting(_))
            || matches!(simple, SimpleSelector::PseudoClass(pc)
                if pc.args.is_some() && matches!(pc.name.as_str(), "is" | "where" | "has" | "not"))
    })
}

fn attribute_matches(
    pruner: &PruneVisitor<'_, '_, '_>,
    elem_id: NodeId,
    name: &str,
    expected_value: Option<&str>,
    operator: &str,
    case_insensitive: bool,
) -> bool {
    let name_lower = name.to_ascii_lowercase();
    let Some(attrs) = element_attributes(pruner.component, elem_id) else {
        return false;
    };

    for attr in attrs {
        if matches!(attr, Attribute::SpreadAttribute(_)) {
            return true;
        }
        if matches!(attr, Attribute::BindDirective(bind) if bind.name.eq_ignore_ascii_case(name)) {
            return true;
        }
        if matches!(attr, Attribute::StyleDirective(_) if name_lower == "style") {
            return true;
        }
        if let Attribute::ClassDirective(class_directive) = attr {
            if name_lower == "class" {
                if operator == "~=" {
                    if expected_value.is_some_and(|value| class_directive.name == value) {
                        return true;
                    }
                } else {
                    return true;
                }
            }
        }

        if !attribute_name_matches_selector(pruner.component, attr, name) {
            continue;
        }

        match attr {
            Attribute::BooleanAttribute(_) => {
                return expected_value.is_none();
            }
            Attribute::StringAttribute(attr) => {
                if expected_value.is_none() {
                    return true;
                }
                let matches = test_attribute(
                    operator,
                    expected_value.unwrap(),
                    case_insensitive,
                    attr.value_span.source_text(&pruner.component.source),
                );
                if !matches && (name_lower == "class" || name_lower == "style") {
                    continue;
                }
                return matches;
            }
            Attribute::ExpressionAttribute(attr) => {
                if expected_value.is_none() {
                    return true;
                }
                let Some(chunks) = attribute_chunks_for_expression_attr(pruner, attr.id) else {
                    return true;
                };
                if match_attribute_chunks(
                    operator,
                    expected_value.unwrap(),
                    case_insensitive,
                    &chunks,
                ) {
                    return true;
                }
                if name_lower == "class" || name_lower == "style" {
                    continue;
                }
                return false;
            }
            Attribute::ConcatenationAttribute(attr) => {
                if expected_value.is_none() {
                    return true;
                }
                let chunks = attribute_chunks_for_concat(pruner, &attr.parts);
                if match_attribute_chunks(
                    operator,
                    expected_value.unwrap(),
                    case_insensitive,
                    &chunks,
                ) {
                    return true;
                }
                if name_lower == "class" || name_lower == "style" {
                    continue;
                }
                return false;
            }
            Attribute::Shorthand(attr) => {
                if expected_value.is_none() {
                    return true;
                }
                let Some(chunks) = attribute_chunks_for_expression_attr(pruner, attr.id) else {
                    return true;
                };
                return match_attribute_chunks(
                    operator,
                    expected_value.unwrap(),
                    case_insensitive,
                    &chunks,
                );
            }
            Attribute::BindDirective(_)
            | Attribute::ClassDirective(_)
            | Attribute::StyleDirective(_)
            | Attribute::SpreadAttribute(_)
            | Attribute::UseDirective(_)
            | Attribute::OnDirectiveLegacy(_)
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

fn attribute_chunks_for_expression_attr<'a>(
    pruner: &'a PruneVisitor<'_, '_, 'a>,
    attr_id: NodeId,
) -> Option<Vec<AttributeChunk<'a>>> {
    let handle = pruner
        .template
        .template_semantics
        .attr_expr_handles
        .get(attr_id)
        .copied()?;
    let expr = pruner.parsed.expr(handle)?;
    Some(vec![AttributeChunk::Expr(expr)])
}

fn attribute_chunks_for_concat<'a>(
    pruner: &'a PruneVisitor<'_, '_, 'a>,
    parts: &'a [ConcatPart],
) -> Vec<AttributeChunk<'a>> {
    let mut chunks = Vec::with_capacity(parts.len());
    for part in parts {
        match part {
            ConcatPart::Static(text) => chunks.push(AttributeChunk::Text(text)),
            ConcatPart::Dynamic { id, .. } => {
                if let Some(handle) = pruner
                    .template
                    .template_semantics
                    .attr_expr_handles
                    .get(*id)
                    .copied()
                {
                    if let Some(expr) = pruner.parsed.expr(handle) {
                        chunks.push(AttributeChunk::Expr(expr));
                    }
                }
            }
        }
    }
    chunks
}

fn match_attribute_chunks(
    operator: &str,
    expected: &str,
    case_insensitive: bool,
    chunks: &[AttributeChunk<'_>],
) -> bool {
    let Some(values) = attribute_possible_values(chunks, operator == "~=") else {
        return true;
    };
    values
        .iter()
        .any(|value| test_attribute(operator, expected, case_insensitive, value))
}

fn attribute_possible_values(chunks: &[AttributeChunk<'_>], is_class: bool) -> Option<Vec<String>> {
    let mut possible_values = FxHashSet::default();
    let mut prev_values: Vec<String> = Vec::new();

    for chunk in chunks {
        let current_values = match chunk {
            AttributeChunk::Text(text) => Some(vec![(*text).to_string()]),
            AttributeChunk::Expr(expr) => get_possible_values(expr, is_class),
        }?;

        if !prev_values.is_empty() {
            let mut start_with_space = Vec::new();
            let mut remaining = Vec::new();
            for value in current_values {
                if starts_with_whitespace(&value) {
                    start_with_space.push(value);
                } else {
                    remaining.push(value);
                }
            }

            if !remaining.is_empty() {
                if !start_with_space.is_empty() {
                    for prev in &prev_values {
                        possible_values.insert(prev.clone());
                    }
                }

                let mut combined = Vec::new();
                for prev in &prev_values {
                    for value in &remaining {
                        combined.push(format!("{prev}{value}"));
                    }
                }
                prev_values = combined;

                for value in start_with_space {
                    if ends_with_whitespace(&value) {
                        possible_values.insert(value);
                    } else {
                        prev_values.push(value);
                    }
                }
                continue;
            }

            for prev in &prev_values {
                possible_values.insert(prev.clone());
            }
            prev_values.clear();
        }

        let current_values = match chunk {
            AttributeChunk::Text(text) => vec![(*text).to_string()],
            AttributeChunk::Expr(expr) => get_possible_values(expr, is_class)?,
        };
        let had_non_terminal = current_values
            .iter()
            .any(|value| !ends_with_whitespace(value));
        for value in current_values {
            if ends_with_whitespace(&value) {
                possible_values.insert(value);
            } else {
                prev_values.push(value);
            }
        }
        if had_non_terminal && prev_values.len() > 20 {
            return None;
        }
    }

    for prev in prev_values {
        possible_values.insert(prev);
    }

    Some(possible_values.into_iter().collect())
}

fn get_possible_values(expr: &Expression<'_>, is_class: bool) -> Option<Vec<String>> {
    let mut values = FxHashSet::default();
    if !gather_possible_values(expr, is_class, &mut values, false) {
        return None;
    }
    Some(values.into_iter().collect())
}

fn gather_possible_values(
    expr: &Expression<'_>,
    is_class: bool,
    values: &mut FxHashSet<String>,
    is_nested: bool,
) -> bool {
    match expr {
        Expression::StringLiteral(lit) => {
            values.insert(lit.value.to_string());
            true
        }
        Expression::NumericLiteral(lit) => {
            values.insert(lit.value.to_string());
            true
        }
        Expression::BooleanLiteral(lit) => {
            values.insert(lit.value.to_string());
            true
        }
        Expression::NullLiteral(_) => {
            values.insert("null".to_string());
            true
        }
        Expression::BigIntLiteral(lit) => {
            values.insert(
                lit.raw
                    .as_ref()
                    .map_or_else(|| "0".to_string(), ToString::to_string),
            );
            true
        }
        Expression::ConditionalExpression(expr) => {
            gather_possible_values(&expr.consequent, is_class, values, is_nested)
                && gather_possible_values(&expr.alternate, is_class, values, is_nested)
        }
        Expression::LogicalExpression(expr) => match expr.operator {
            LogicalOperator::And => {
                let mut left = FxHashSet::default();
                if !gather_possible_values(&expr.left, is_class, &mut left, is_nested) {
                    if !is_class || !is_nested {
                        values.insert(String::new());
                        values.insert("false".to_string());
                        values.insert("NaN".to_string());
                        values.insert("0".to_string());
                    }
                } else {
                    for value in left {
                        if is_falsy_string_value(&value) && (!is_class || !is_nested) {
                            values.insert(value);
                        }
                    }
                }
                gather_possible_values(&expr.right, is_class, values, is_nested)
            }
            _ => {
                gather_possible_values(&expr.left, is_class, values, is_nested)
                    && gather_possible_values(&expr.right, is_class, values, is_nested)
            }
        },
        Expression::ArrayExpression(array) if is_class => {
            for element in &array.elements {
                if let Some(expr) = element.as_expression() {
                    if !gather_possible_values(expr, is_class, values, true) {
                        return false;
                    }
                }
            }
            true
        }
        Expression::ObjectExpression(object) if is_class => {
            for property in &object.properties {
                let oxc_ast::ast::ObjectPropertyKind::ObjectProperty(property) = property else {
                    return false;
                };
                if property.computed {
                    return false;
                }
                match &property.key {
                    PropertyKey::Identifier(ident) => {
                        values.insert(ident.name.to_string());
                    }
                    PropertyKey::StringLiteral(lit) => {
                        values.insert(lit.value.to_string());
                    }
                    PropertyKey::NumericLiteral(lit) => {
                        values.insert(lit.value.to_string());
                    }
                    _ => return false,
                }
            }
            true
        }
        _ => false,
    }
}

fn is_falsy_string_value(value: &str) -> bool {
    value.is_empty() || value == "false" || value == "NaN" || value == "0"
}

fn starts_with_whitespace(value: &str) -> bool {
    value.chars().next().is_some_and(char::is_whitespace)
}

fn ends_with_whitespace(value: &str) -> bool {
    value.chars().next_back().is_some_and(char::is_whitespace)
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
        "|=" => format!("{value}-").starts_with(&format!("{expected}-")),
        "^=" => value.starts_with(expected.as_str()),
        "$=" => value.ends_with(expected.as_str()),
        "*=" => value.contains(expected.as_str()),
        _ => false,
    }
}

fn unquote(value: &str) -> &str {
    if value.len() >= 2 {
        let bytes = value.as_bytes();
        if (bytes[0] == b'"' && bytes[value.len() - 1] == b'"')
            || (bytes[0] == b'\'' && bytes[value.len() - 1] == b'\'')
        {
            return &value[1..value.len() - 1];
        }
    }
    value
}

fn warn_unused(
    stylesheet: &StyleSheet,
    css_source: &str,
    css_offset: u32,
    used: &FxHashSet<svelte_css::CssNodeId>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let mut warner = UnusedWarner {
        css_source,
        css_offset,
        used,
        diagnostics,
        in_keyframes: false,
        in_global_block: false,
        complex_used_stack: Vec::new(),
        pseudo_parent_used_stack: Vec::new(),
    };
    warner.visit_stylesheet(stylesheet);
}

struct UnusedWarner<'a> {
    css_source: &'a str,
    css_offset: u32,
    used: &'a FxHashSet<svelte_css::CssNodeId>,
    diagnostics: &'a mut Vec<Diagnostic>,
    in_keyframes: bool,
    in_global_block: bool,
    complex_used_stack: Vec<bool>,
    pseudo_parent_used_stack: Vec<bool>,
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
        if is_global_block_rule(node) {
            self.in_global_block = was_global || starts_with_global_block(node);
            self.visit_selector_list(&node.prelude);
            self.in_global_block = was_global;
            return;
        }
        svelte_css::visit::walk_style_rule(self, node);
        self.in_global_block = was_global;
    }

    fn visit_complex_selector(&mut self, node: &ComplexSelector) {
        if self.in_keyframes || self.in_global_block {
            return;
        }
        let is_used = self.used.contains(&node.id);
        let should_warn = !is_used
            && self
                .pseudo_parent_used_stack
                .last()
                .copied()
                .unwrap_or(true);

        self.complex_used_stack.push(is_used);
        if should_warn {
            let local_span = node.span();
            let selector_text = &self.css_source
                [local_span.start as usize..(local_span.end as usize).min(self.css_source.len())];
            let span = svelte_span::Span::new(
                self.css_offset + local_span.start,
                self.css_offset + local_span.end,
            );
            self.diagnostics.push(Diagnostic::warning(
                DiagnosticKind::CssUnusedSelector {
                    name: selector_text.to_string(),
                },
                span,
            ));
        }
        svelte_css::visit::walk_complex_selector(self, node);
        self.complex_used_stack.pop();
    }

    fn visit_simple_selector(&mut self, node: &SimpleSelector) {
        let SimpleSelector::PseudoClass(pc) = node else {
            return;
        };
        if !matches!(pc.name.as_str(), "is" | "where" | "has" | "not") {
            return;
        }
        let Some(args) = &pc.args else {
            return;
        };

        self.pseudo_parent_used_stack
            .push(self.complex_used_stack.last().copied().unwrap_or(false));
        self.visit_selector_list(args);
        self.pseudo_parent_used_stack.pop();
    }
}

fn is_global_block_rule(rule: &StyleRule) -> bool {
    rule.prelude.children.iter().any(|complex| {
        complex.children.iter().any(|relative| {
            relative.selectors.first().is_some_and(|selector| {
                matches!(selector, SimpleSelector::Global { args: None, .. })
            })
        })
    })
}

fn starts_with_global_block(rule: &StyleRule) -> bool {
    rule.prelude
        .children
        .first()
        .and_then(|complex| complex.children.first())
        .and_then(|relative| relative.selectors.first())
        .is_some_and(|selector| matches!(selector, SimpleSelector::Global { args: None, .. }))
}
