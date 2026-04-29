use compact_str::CompactString;
use svelte_css::{
    AtRule, Block, BlockChild, CombinatorKind, ComplexSelector, RelativeSelector, SelectorList,
    SimpleSelector, StyleRule, StyleSheet, Visit,
};
use svelte_diagnostics::Diagnostic;
use svelte_diagnostics::DiagnosticKind;
use svelte_span::GetSpan;

use svelte_ast::Component as SvelteComponent;
use svelte_parser::JsAst;

use crate::css::css_component_hash;
use crate::types::data::{AnalysisData, CssAnalysis};
use crate::types::node_table::NodeBitSet;

pub fn analyze_css_pass(
    component: &SvelteComponent,
    stylesheet: &StyleSheet,
    parsed: &JsAst<'_>,
    inject_styles: bool,
    data: &mut AnalysisData,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let Some(css_block) = &component.css else {
        return;
    };
    let css_text = component.source_text(css_block.content_span);
    let hash = css_component_hash(css_text);

    let keyframes = collect_keyframe_names(stylesheet, css_text);

    let css_diag_start = diagnostics.len();
    let mut validator = CssValidator::new(css_block.content_span.start, diagnostics);
    validator.visit_stylesheet(stylesheet);
    let has_css_errors = diagnostics[css_diag_start..]
        .iter()
        .any(|diag| diag.severity == svelte_diagnostics::Severity::Error);

    let node_count = component.node_count();
    data.output.css = CssAnalysis {
        hash,
        scoped_elements: NodeBitSet::new(node_count),
        inject_styles,
        keyframes,
        used_selectors: rustc_hash::FxHashSet::default(),
    };

    super::css_prune::prune_and_warn(
        component,
        stylesheet,
        css_text,
        css_block.content_span.start,
        !has_css_errors,
        parsed,
        data,
        diagnostics,
    );
}

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

        svelte_css::visit::walk_at_rule(self, node);
    }
}

fn is_global_block_selector(sel: &SimpleSelector) -> bool {
    matches!(sel, SimpleSelector::Global { args: None, .. })
}

fn is_global_relative_selector(rel: &RelativeSelector) -> bool {
    let Some(first) = rel.selectors.first() else {
        return false;
    };
    match first {
        SimpleSelector::Global { args, .. } => {
            if args.is_none() {
                return true;
            }

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

fn is_unscoped_pseudo_class(sel: &SimpleSelector) -> bool {
    let SimpleSelector::PseudoClass(pc) = sel else {
        return false;
    };
    let name = pc.name.as_str();

    if name == "has" || name == "is" || name == "where" {
        return pc.args.as_ref().is_none_or(|args| {
            args.children
                .iter()
                .all(|c| c.children.iter().all(is_global_relative_selector))
        });
    }
    if name == "not" {
        return pc.args.as_ref().is_none_or(|args| {
            args.children.iter().all(|c| c.children.len() == 1)
                || args
                    .children
                    .iter()
                    .all(|c| c.children.iter().all(is_global_relative_selector))
        });
    }

    true
}

struct RuleContext {
    is_lone_global_block: bool,

    has_parent_rule: bool,

    parent_is_lone_global_block: bool,

    is_lone_global_with_nesting_arg: bool,
}

struct CssValidator<'a> {
    css_offset: u32,
    diagnostics: &'a mut Vec<Diagnostic>,

    rule_stack: Vec<RuleContext>,

    in_pseudo_class: bool,
}

impl<'a> CssValidator<'a> {
    fn new(css_offset: u32, diagnostics: &'a mut Vec<Diagnostic>) -> Self {
        Self {
            css_offset,
            diagnostics,
            rule_stack: Vec::new(),
            in_pseudo_class: false,
        }
    }

    fn current_rule(&self) -> Option<&RuleContext> {
        self.rule_stack.last()
    }

    fn emit(&mut self, kind: DiagnosticKind, span: svelte_span::Span) {
        let span = svelte_span::Span::new(self.css_offset + span.start, self.css_offset + span.end);
        self.diagnostics.push(Diagnostic::error(kind, span));
    }

    fn validate_rule(&mut self, rule: &StyleRule) {
        let has_parent = !self.rule_stack.is_empty();
        let mut rule_is_global_block = false;
        let mut first_complex = true;
        let mut emitted_global_block_invalid_list = false;

        for complex_selector in &rule.prelude.children {
            let mut is_global_block = false;

            for (selector_idx, child) in complex_selector.children.iter().enumerate() {
                let global_pos = child.selectors.iter().position(is_global_block_selector);

                if let Some(idx) = global_pos {
                    if idx == 0 {
                        if child.selectors.len() > 1 && selector_idx == 0 && !has_parent {
                            self.emit(
                                DiagnosticKind::CssGlobalBlockInvalidModifierStart,
                                child.selectors[1].span(),
                            );
                        } else {
                            is_global_block = true;

                            if let Some(ref comb) = child.combinator
                                && comb.kind != CombinatorKind::Descendant
                            {
                                self.emit(
                                    DiagnosticKind::CssGlobalBlockInvalidCombinator {
                                        name: comb.kind.as_str().to_string(),
                                    },
                                    child.span,
                                );
                            }

                            let is_lone_global = complex_selector.children.len() == 1
                                && complex_selector.children[0].selectors.len() == 1;

                            if is_lone_global && rule.prelude.children.len() > 1 {
                                self.emit(
                                    DiagnosticKind::CssGlobalBlockInvalidList,
                                    rule.prelude.span,
                                );
                                emitted_global_block_invalid_list = true;
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
                                    .expect("has_declaration returned true");
                                self.emit(
                                    DiagnosticKind::CssGlobalBlockInvalidDeclaration,
                                    decl_span,
                                );
                            }
                        }
                    } else {
                        self.emit(
                            DiagnosticKind::CssGlobalBlockInvalidModifier,
                            child.selectors[idx].span(),
                        );
                    }
                }
            }

            if rule_is_global_block && !is_global_block && !emitted_global_block_invalid_list {
                self.emit(DiagnosticKind::CssGlobalBlockInvalidList, rule.prelude.span);
                emitted_global_block_invalid_list = true;
            }

            if first_complex {
                rule_is_global_block = is_global_block;
            }
            first_complex = false;
        }

        let is_lone_global_block = rule_is_global_block
            && rule.prelude.children.len() == 1
            && rule.prelude.children[0].children.len() == 1
            && rule.prelude.children[0].children[0].selectors.len() == 1;

        let parent_is_lone_global_block = if has_parent {
            self.current_rule()
                .is_some_and(|r| r.is_lone_global_block && !r.has_parent_rule)
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

        self.visit_selector_list(&rule.prelude);
        self.visit_block(&rule.block);

        self.rule_stack.pop();
    }

    fn validate_complex_selector(&mut self, node: &ComplexSelector) {
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

            let idx = node
                .children
                .iter()
                .position(is_global_relative_selector)
                .expect("caller guarantees selector contains a :global(...) child");
            if let SimpleSelector::Global {
                args: Some(_),
                span,
                ..
            } = &global_rel.selectors[0]
                && idx != 0
                && idx != node.children.len() - 1
                && node.children[idx + 1..]
                    .iter()
                    .any(|r| !is_global_relative_selector(r))
            {
                self.emit(DiagnosticKind::CssGlobalInvalidPlacement, *span);
            }
        }

        for rel in &node.children {
            for (i, sel) in rel.selectors.iter().enumerate() {
                let (args, span) = match sel {
                    SimpleSelector::Global { args, span, .. } => (args, span),
                    _ => continue,
                };

                if let Some(args) = args {
                    if let Some(first_type_sel) = args
                        .children
                        .first()
                        .and_then(|c| c.children.first())
                        .and_then(|r| r.selectors.first())
                        && matches!(first_type_sel, SimpleSelector::Type { .. })
                        && i != 0
                    {
                        self.emit(DiagnosticKind::CssGlobalInvalidSelectorList, *span);
                    }

                    if args.children.len() > 1
                        && (node.children.len() > 1 || rel.selectors.len() > 1)
                    {
                        self.emit(DiagnosticKind::CssGlobalInvalidSelector, *span);
                    }
                }

                if let Some(next) = rel.selectors.get(i + 1)
                    && matches!(next, SimpleSelector::Type { .. })
                {
                    self.emit(DiagnosticKind::CssTypeSelectorInvalidPlacement, next.span());
                }
            }
        }
    }

    fn validate_nesting_selector(&mut self, span: svelte_span::Span) {
        let has_parent = self.current_rule().is_some_and(|r| r.has_parent_rule);

        if !has_parent {
            let valid = self
                .current_rule()
                .is_some_and(|r| r.is_lone_global_with_nesting_arg);
            if !valid {
                self.emit(DiagnosticKind::CssNestingSelectorInvalidPlacement, span);
            }
        } else {
            if self
                .current_rule()
                .is_some_and(|r| r.parent_is_lone_global_block)
            {
                self.emit(DiagnosticKind::CssGlobalBlockInvalidModifierStart, span);
            }
        }
    }
}

fn is_nesting_in_global_args(args: &SelectorList) -> bool {
    args.children
        .first()
        .and_then(|c| c.children.first())
        .is_some_and(|r| {
            r.selectors.len() == 1 && matches!(r.selectors[0], SimpleSelector::Nesting(_))
        })
}

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
        for (i, child) in node.children.iter().enumerate() {
            if i == 0
                && !self.in_pseudo_class
                && !self.current_rule().is_some_and(|r| r.has_parent_rule)
                && let Some(combinator) = child.combinator.as_ref()
            {
                self.emit(DiagnosticKind::CssSelectorInvalid, combinator.span);
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
