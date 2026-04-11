use crate::ast::*;
use rustc_hash::FxHashSet;

/// CSS printer — serializes a [`StyleSheet`] AST back to a CSS string.
///
/// Requires access to the original source text because the AST stores
/// [`Span`]s rather than owned strings.
pub struct Printer<'a> {
    output: String,
    indent: usize,
    minify: bool,
    used_selectors: Option<&'a FxHashSet<CssNodeId>>,
    remove_unused: bool,
}

/// Pre-computed indentation strings to avoid per-indent allocation.
const INDENTS: [&str; 8] = [
    "",
    "  ",
    "    ",
    "      ",
    "        ",
    "          ",
    "            ",
    "              ",
];

impl Printer<'_> {
    pub fn new() -> Self {
        Self {
            output: String::new(),
            indent: 0,
            minify: false,
            used_selectors: None,
            remove_unused: false,
        }
    }

    pub fn minified() -> Self {
        Self {
            output: String::new(),
            indent: 0,
            minify: true,
            used_selectors: None,
            remove_unused: false,
        }
    }

    /// Convenience: print stylesheet to string.
    pub fn print(stylesheet: &StyleSheet, source: &str) -> String {
        // Pre-allocate ~same size as source (CSS output is similar length)
        let mut p = Printer {
            output: String::with_capacity(source.len()),
            indent: 0,
            minify: false,
            used_selectors: None,
            remove_unused: false,
        };
        p.print_stylesheet(stylesheet, source);
        p.output
    }

    pub fn print_with_usage(
        stylesheet: &StyleSheet,
        source: &str,
        used_selectors: &'_ FxHashSet<CssNodeId>,
        remove_unused: bool,
    ) -> String {
        let mut p = Printer {
            output: String::with_capacity(source.len()),
            indent: 0,
            minify: false,
            used_selectors: Some(used_selectors),
            remove_unused,
        };
        p.print_stylesheet(stylesheet, source);
        p.output
    }

    pub fn print_stylesheet(&mut self, node: &StyleSheet, source: &str) -> &str {
        if self.output.capacity() == 0 {
            self.output.reserve(128);
        }
        let mut first = true;
        for child in &node.children {
            match child {
                StyleSheetChild::Comment(c) => {
                    if !first && !self.minify {
                        self.output.push('\n');
                    }
                    first = false;
                    self.write_indent();
                    self.push_span(c.span, source);
                    if !self.minify {
                        self.output.push('\n');
                    }
                }
                StyleSheetChild::Rule(rule) => {
                    if !first && !self.minify {
                        self.output.push('\n');
                    }
                    if self.print_rule(rule, source) {
                        first = false;
                    }
                }
                StyleSheetChild::Error(_) => {}
            }
        }
        &self.output
    }

    fn print_rule(&mut self, rule: &Rule, source: &str) -> bool {
        match rule {
            Rule::Style(r) => self.print_style_rule(r.as_ref(), source),
            Rule::AtRule(r) => {
                self.print_at_rule(r, source);
                true
            }
        }
    }

    fn print_style_rule(&mut self, rule: &StyleRule, source: &str) -> bool {
        let rule_used = self.rule_is_used(rule);
        if self.remove_unused && !rule_used {
            return false;
        }

        self.write_indent();
        if rule_used {
            self.print_selector_list(&rule.prelude, source);
            if self.minify {
                self.output.push('{');
            } else {
                self.output.push_str(" {\n");
            }
            self.indent += 1;
            self.print_block_children(&rule.block, source);
            self.indent -= 1;
            self.write_indent();
            self.output.push_str("}\n");
            true
        } else {
            self.output.push_str("/* (unused) ");
            self.output.push_str(rule.span.source_text(source).trim());
            self.output.push_str("*/\n");
            true
        }
    }

    fn print_at_rule(&mut self, rule: &AtRule, source: &str) {
        self.write_indent();
        self.output.push('@');
        self.output.push_str(&rule.name);

        let prelude_text = match &rule.prelude_override {
            Some(ov) => ov.as_str(),
            None => rule.prelude.source_text(source).trim(),
        };
        if !prelude_text.is_empty() {
            self.output.push(' ');
            self.output.push_str(prelude_text);
        }

        if let Some(block) = &rule.block {
            let is_keyframes = rule.name == "keyframes"
                || rule
                    .name
                    .strip_prefix('-')
                    .and_then(|s| s.split_once('-'))
                    .is_some_and(|(_, rest)| rest == "keyframes");

            if is_keyframes {
                // Preserve original formatting for @keyframes body — the internal
                // structure (from/to/percentage stops) is never transformed.
                self.output.push(' ');
                self.push_span(block.span, source);
                self.output.push('\n');
            } else if self.minify {
                self.output.push('{');
                self.print_block_children(block, source);
                self.output.push('}');
            } else {
                self.output.push_str(" {\n");
                self.indent += 1;
                self.print_block_children(block, source);
                self.indent -= 1;
                self.write_indent();
                self.output.push_str("}\n");
            }
        } else {
            self.output.push_str(";\n");
        }
    }

    fn print_block_children(&mut self, block: &Block, source: &str) {
        for child in &block.children {
            match child {
                BlockChild::Declaration(d) => self.print_declaration(d, source),
                BlockChild::Rule(r) => {
                    self.print_rule(r, source);
                }
                BlockChild::Comment(c) => {
                    self.write_indent();
                    self.push_span(c.span, source);
                    if !self.minify {
                        self.output.push('\n');
                    }
                }
                BlockChild::Error(_) => {}
            }
        }
    }

    fn print_declaration(&mut self, decl: &Declaration, source: &str) {
        self.write_indent();
        self.push_span(decl.property, source);
        if self.minify {
            self.output.push(':');
        } else {
            self.output.push_str(": ");
        }
        match &decl.value_override {
            Some(ov) => self.output.push_str(ov),
            None => self.push_span(decl.value, source),
        }
        self.output.push_str(";\n");
    }

    fn print_selector_list(&mut self, list: &SelectorList, source: &str) {
        let mut groups: Vec<(bool, Vec<String>)> = Vec::new();

        for complex in &list.children {
            let used = self.selector_is_used(complex.id);
            if self.remove_unused && !used {
                continue;
            }
            let text = if used {
                self.render_complex_selector(complex, source)
            } else {
                complex.span.source_text(source).trim().to_string()
            };
            if let Some((group_used, entries)) = groups.last_mut()
                && *group_used == used
            {
                entries.push(text);
            } else {
                groups.push((used, vec![text]));
            }
        }

        for (group_idx, (used, entries)) in groups.iter().enumerate() {
            if group_idx > 0 {
                self.output.push(' ');
            }
            if !*used {
                self.output.push_str("/* (unused) ");
            }
            for (entry_idx, entry) in entries.iter().enumerate() {
                if entry_idx > 0 {
                    if self.minify {
                        self.output.push(',');
                    } else {
                        self.output.push_str(", ");
                    }
                }
                self.output.push_str(entry);
            }
            if !*used {
                self.output.push_str("*/");
            }
        }
    }

    fn render_complex_selector(&self, sel: &ComplexSelector, source: &str) -> String {
        let mut output = String::new();
        for (i, rel) in sel.children.iter().enumerate() {
            if let Some(combinator) = &rel.combinator
                && (i > 0 || combinator.kind != CombinatorKind::Descendant)
            {
                match combinator.kind {
                    CombinatorKind::Descendant => output.push(' '),
                    CombinatorKind::Child => output.push_str(" > "),
                    CombinatorKind::NextSibling => output.push_str(" + "),
                    CombinatorKind::SubsequentSibling => output.push_str(" ~ "),
                    CombinatorKind::Column => output.push_str(" || "),
                }
            }
            for simple in &rel.selectors {
                Self::render_simple_selector(&mut output, simple, source);
            }
        }
        output
    }

    fn render_simple_selector(output: &mut String, sel: &SimpleSelector, source: &str) {
        match sel {
            SimpleSelector::Type { name, .. } => {
                output.push_str(name);
            }
            SimpleSelector::Id { name, .. } => {
                output.push('#');
                output.push_str(name);
            }
            SimpleSelector::Class { name, .. } => {
                output.push('.');
                output.push_str(name);
            }
            SimpleSelector::Global { args, .. } => {
                output.push_str(":global");
                if let Some(args) = args {
                    output.push('(');
                    for (idx, complex) in args.children.iter().enumerate() {
                        if idx > 0 {
                            output.push_str(", ");
                        }
                        let mut nested = String::new();
                        for (rel_idx, rel) in complex.children.iter().enumerate() {
                            if let Some(combinator) = &rel.combinator
                                && (rel_idx > 0 || combinator.kind != CombinatorKind::Descendant)
                            {
                                match combinator.kind {
                                    CombinatorKind::Descendant => nested.push(' '),
                                    CombinatorKind::Child => nested.push_str(" > "),
                                    CombinatorKind::NextSibling => nested.push_str(" + "),
                                    CombinatorKind::SubsequentSibling => nested.push_str(" ~ "),
                                    CombinatorKind::Column => nested.push_str(" || "),
                                }
                            }
                            for simple in &rel.selectors {
                                Self::render_simple_selector(&mut nested, simple, source);
                            }
                        }
                        output.push_str(&nested);
                    }
                    output.push(')');
                }
            }
            SimpleSelector::Nesting(span)
            | SimpleSelector::Nth(span)
            | SimpleSelector::Percentage(span) => {
                output.push_str(span.source_text(source));
            }
            SimpleSelector::PseudoClass(pc) => {
                output.push(':');
                output.push_str(&pc.name);
                if let Some(args) = &pc.args {
                    output.push('(');
                    for (idx, complex) in args.children.iter().enumerate() {
                        if idx > 0 {
                            output.push_str(", ");
                        }
                        let mut nested = String::new();
                        for (rel_idx, rel) in complex.children.iter().enumerate() {
                            if let Some(combinator) = &rel.combinator
                                && (rel_idx > 0 || combinator.kind != CombinatorKind::Descendant)
                            {
                                match combinator.kind {
                                    CombinatorKind::Descendant => nested.push(' '),
                                    CombinatorKind::Child => nested.push_str(" > "),
                                    CombinatorKind::NextSibling => nested.push_str(" + "),
                                    CombinatorKind::SubsequentSibling => nested.push_str(" ~ "),
                                    CombinatorKind::Column => nested.push_str(" || "),
                                }
                            }
                            for simple in &rel.selectors {
                                Self::render_simple_selector(&mut nested, simple, source);
                            }
                        }
                        output.push_str(&nested);
                    }
                    output.push(')');
                }
            }
            SimpleSelector::PseudoElement(pe) => {
                output.push_str("::");
                output.push_str(&pe.name);
                if let Some(args) = &pe.args {
                    output.push('(');
                    for (idx, complex) in args.children.iter().enumerate() {
                        if idx > 0 {
                            output.push_str(", ");
                        }
                        let mut nested = String::new();
                        for (rel_idx, rel) in complex.children.iter().enumerate() {
                            if let Some(combinator) = &rel.combinator
                                && (rel_idx > 0 || combinator.kind != CombinatorKind::Descendant)
                            {
                                match combinator.kind {
                                    CombinatorKind::Descendant => nested.push(' '),
                                    CombinatorKind::Child => nested.push_str(" > "),
                                    CombinatorKind::NextSibling => nested.push_str(" + "),
                                    CombinatorKind::SubsequentSibling => nested.push_str(" ~ "),
                                    CombinatorKind::Column => nested.push_str(" || "),
                                }
                            }
                            for simple in &rel.selectors {
                                Self::render_simple_selector(&mut nested, simple, source);
                            }
                        }
                        output.push_str(&nested);
                    }
                    output.push(')');
                }
            }
            SimpleSelector::Attribute(attr) => {
                output.push('[');
                output.push_str(&attr.name);
                if let Some(matcher) = attr.matcher {
                    output.push_str(matcher.source_text(source));
                    if let Some(value) = attr.value {
                        output.push('"');
                        output.push_str(value.source_text(source));
                        output.push('"');
                    }
                }
                if let Some(flags) = attr.flags {
                    output.push(' ');
                    output.push_str(flags.source_text(source));
                }
                output.push(']');
            }
        }
    }

    fn selector_is_used(&self, id: CssNodeId) -> bool {
        self.used_selectors.is_none_or(|used| used.contains(&id))
    }

    fn rule_is_used(&self, rule: &StyleRule) -> bool {
        rule.is_lone_global_block()
            || rule
                .prelude
                .children
                .iter()
                .any(|sel| self.selector_is_used(sel.id))
    }

    // -- output helpers -----------------------------------------------------

    #[inline]
    fn push_span(&mut self, span: svelte_span::Span, source: &str) {
        self.output.push_str(span.source_text(source));
    }

    #[inline]
    fn write_indent(&mut self) {
        if !self.minify {
            if self.indent < INDENTS.len() {
                self.output.push_str(INDENTS[self.indent]);
            } else {
                for _ in 0..self.indent {
                    self.output.push_str("  ");
                }
            }
        }
    }
}

impl Default for Printer<'_> {
    fn default() -> Self {
        Self::new()
    }
}
