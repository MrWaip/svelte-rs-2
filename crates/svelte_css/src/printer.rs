use crate::ast::*;
use memchr::{memchr_iter, memrchr};
use rustc_hash::FxHashSet;
use svelte_sourcemap::{SourceMap, SourceMapBuilder};
use svelte_span::LineIndex;

struct MapState {
    builder: SourceMapBuilder,
    line: u32,
    column: u32,
    source_id: u32,
    line_index: LineIndex,
}

pub struct Printer<'a> {
    output: String,
    indent: usize,
    minify: bool,
    used_selectors: Option<&'a FxHashSet<CssNodeId>>,
    remove_unused: bool,
    map: Option<MapState>,
}
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
            map: None,
        }
    }

    pub fn minified() -> Self {
        Self {
            output: String::new(),
            indent: 0,
            minify: true,
            used_selectors: None,
            remove_unused: false,
            map: None,
        }
    }
    pub fn print(stylesheet: &StyleSheet, source: &str) -> String {
        let mut p = Printer {
            output: String::with_capacity(source.len()),
            indent: 0,
            minify: false,
            used_selectors: None,
            remove_unused: false,
            map: None,
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
            map: None,
        };
        p.print_stylesheet(stylesheet, source);
        p.output
    }
}

impl<'a> Printer<'a> {
    pub fn print_with_sourcemap(
        stylesheet: &StyleSheet,
        source: &str,
        filename: &str,
        used_selectors: Option<&'a FxHashSet<CssNodeId>>,
        remove_unused: bool,
    ) -> (String, SourceMap) {
        let mut builder = SourceMapBuilder::default();
        let source_id = builder.add_source_and_content(filename, source);
        let line_index = LineIndex::new(source);
        let mut p = Printer {
            output: String::with_capacity(source.len()),
            indent: 0,
            minify: false,
            used_selectors,
            remove_unused,
            map: Some(MapState {
                builder,
                line: 0,
                column: 0,
                source_id,
                line_index,
            }),
        };
        p.print_stylesheet(stylesheet, source);
        let map_state = p.map.take().expect("map state set above");
        (p.output, map_state.builder.into_sourcemap())
    }
}

impl Printer<'_> {
    pub fn print_stylesheet(&mut self, node: &StyleSheet, source: &str) -> &str {
        if self.output.capacity() == 0 {
            self.output.reserve(128);
        }
        let mut first = true;
        for child in &node.children {
            match child {
                StyleSheetChild::Comment(c) => {
                    if !first && !self.minify {
                        self.push_ch('\n');
                    }
                    first = false;
                    self.write_indent();
                    self.push_span(c.span, source);
                    if !self.minify {
                        self.push_ch('\n');
                    }
                }
                StyleSheetChild::Rule(rule) => {
                    if !first && !self.minify {
                        self.push_ch('\n');
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
            self.register_token(rule.span);
            self.print_selector_list(&rule.prelude, source);
            if self.minify {
                self.push_ch('{');
            } else {
                self.push_str(" {\n");
            }
            self.indent += 1;
            self.print_block_children(&rule.block, source);
            self.indent -= 1;
            self.write_indent();
            self.push_str("}");
            self.register_token_end(rule.span);
            self.push_ch('\n');
            true
        } else {
            self.push_str("/* (unused) ");
            self.push_str(rule.span.source_text(source).trim());
            self.push_str("*/\n");
            true
        }
    }

    fn print_at_rule(&mut self, rule: &AtRule, source: &str) {
        self.write_indent();
        self.register_token(rule.span);
        self.push_ch('@');
        self.push_str(&rule.name);

        let prelude_text = match &rule.prelude_override {
            Some(ov) => ov.as_str(),
            None => rule.prelude.source_text(source).trim(),
        };
        if !prelude_text.is_empty() {
            self.push_ch(' ');
            self.push_str(prelude_text);
        }

        if let Some(block) = &rule.block {
            let is_keyframes = rule.name == "keyframes"
                || rule
                    .name
                    .strip_prefix('-')
                    .and_then(|s| s.split_once('-'))
                    .is_some_and(|(_, rest)| rest == "keyframes");

            if is_keyframes {
                self.push_ch(' ');
                self.push_span(block.span, source);
                self.register_token_end(rule.span);
                self.push_ch('\n');
            } else if self.minify {
                self.push_ch('{');
                self.print_block_children(block, source);
                self.push_ch('}');
                self.register_token_end(rule.span);
            } else {
                self.push_str(" {\n");
                self.indent += 1;
                self.print_block_children(block, source);
                self.indent -= 1;
                self.write_indent();
                self.push_str("}");
                self.register_token_end(rule.span);
                self.push_ch('\n');
            }
        } else {
            self.push_str(";");
            self.register_token_end(rule.span);
            self.push_ch('\n');
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
                        self.push_ch('\n');
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
            self.push_ch(':');
        } else {
            self.push_str(": ");
        }
        match &decl.value_override {
            Some(ov) => self.push_str(ov),
            None => self.push_span(decl.value, source),
        }
        self.register_token_end(decl.value);
        self.push_str(";\n");
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
                self.push_ch(' ');
            }
            if !*used {
                self.push_str("/* (unused) ");
            }
            for (entry_idx, entry) in entries.iter().enumerate() {
                if entry_idx > 0 {
                    if self.minify {
                        self.push_ch(',');
                    } else {
                        self.push_str(", ");
                    }
                }
                self.push_str(entry);
            }
            if !*used {
                self.push_str("*/");
            }
        }
    }

    fn push_combinator(output: &mut String, combinator: &Combinator, is_first: bool) {
        match combinator.kind {
            CombinatorKind::Descendant => {
                if !is_first {
                    output.push(' ');
                }
            }
            CombinatorKind::Child => output.push_str(if is_first { "> " } else { " > " }),
            CombinatorKind::NextSibling => output.push_str(if is_first { "+ " } else { " + " }),
            CombinatorKind::SubsequentSibling => {
                output.push_str(if is_first { "~ " } else { " ~ " })
            }
            CombinatorKind::Column => output.push_str(if is_first { "|| " } else { " || " }),
        }
    }

    fn render_complex_selector(&self, sel: &ComplexSelector, source: &str) -> String {
        let mut output = String::new();
        for (i, rel) in sel.children.iter().enumerate() {
            if let Some(combinator) = &rel.combinator {
                Self::push_combinator(&mut output, combinator, i == 0);
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
                            if let Some(combinator) = &rel.combinator {
                                Self::push_combinator(&mut nested, combinator, rel_idx == 0);
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
                            if let Some(combinator) = &rel.combinator {
                                Self::push_combinator(&mut nested, combinator, rel_idx == 0);
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
                            if let Some(combinator) = &rel.combinator {
                                Self::push_combinator(&mut nested, combinator, rel_idx == 0);
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
                        if let Some(q) = attr.quote {
                            let quote = q as char;
                            output.push(quote);
                            output.push_str(value.source_text(source));
                            output.push(quote);
                        } else {
                            output.push_str(value.source_text(source));
                        }
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

    fn push_str(&mut self, s: &str) {
        if let Some(m) = self.map.as_mut() {
            advance_position(&mut m.line, &mut m.column, s);
        }
        self.output.push_str(s);
    }

    fn push_ch(&mut self, ch: char) {
        if let Some(m) = self.map.as_mut() {
            if ch == '\n' {
                m.line += 1;
                m.column = 0;
            } else {
                m.column += 1;
            }
        }
        self.output.push(ch);
    }

    fn push_span(&mut self, span: svelte_span::Span, source: &str) {
        self.register_token(span);
        let text = span.source_text(source);
        if let Some(m) = self.map.as_mut() {
            advance_position(&mut m.line, &mut m.column, text);
        }
        self.output.push_str(text);
    }

    fn register_token(&mut self, span: svelte_span::Span) {
        self.register_byte(span.start);
    }

    fn register_token_end(&mut self, span: svelte_span::Span) {
        self.register_byte(span.end);
    }

    fn register_byte(&mut self, byte: u32) {
        let Some(m) = self.map.as_mut() else { return };
        let (src_line, src_col) = m.line_index.line_col(byte);
        m.builder.add_token(
            m.line,
            m.column,
            src_line.saturating_sub(1) as u32,
            src_col as u32,
            Some(m.source_id),
            None,
        );
    }

    #[inline]
    fn write_indent(&mut self) {
        if !self.minify {
            if self.indent < INDENTS.len() {
                self.push_str(INDENTS[self.indent]);
            } else {
                for _ in 0..self.indent {
                    self.push_str("  ");
                }
            }
        }
    }
}

fn advance_position(line: &mut u32, column: &mut u32, text: &str) {
    let bytes = text.as_bytes();
    let newlines = memchr_iter(b'\n', bytes).count() as u32;
    if newlines == 0 {
        *column += bytes.len() as u32;
        return;
    }
    *line += newlines;
    let last_nl = memrchr(b'\n', bytes).expect("newlines > 0 above");
    *column = (bytes.len() - last_nl - 1) as u32;
}

impl Default for Printer<'_> {
    fn default() -> Self {
        Self::new()
    }
}
