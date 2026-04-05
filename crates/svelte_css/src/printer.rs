use crate::ast::*;

/// CSS printer — serializes a [`StyleSheet`] AST back to a CSS string.
///
/// Requires access to the original source text because the AST stores
/// [`Span`]s rather than owned strings.
pub struct Printer {
    output: String,
    indent: usize,
    minify: bool,
}

impl Printer {
    pub fn new() -> Self {
        Self {
            output: String::new(),
            indent: 0,
            minify: false,
        }
    }

    pub fn minified() -> Self {
        Self {
            output: String::new(),
            indent: 0,
            minify: true,
        }
    }

    /// Convenience: parse + print in one call.
    pub fn print(stylesheet: &StyleSheet, source: &str) -> String {
        let mut p = Printer::new();
        p.print_stylesheet(stylesheet, source);
        p.output
    }

    pub fn print_stylesheet(&mut self, node: &StyleSheet, source: &str) -> &str {
        let mut first = true;
        for child in &node.children {
            match child {
                StyleSheetChild::Comment(c) => {
                    if !first && !self.minify {
                        self.newline();
                    }
                    first = false;
                    self.write_indent();
                    self.push_span(c.span, source);
                    self.newline();
                }
                StyleSheetChild::Rule(rule) => {
                    if !first && !self.minify {
                        self.newline();
                    }
                    first = false;
                    self.print_rule(rule, source);
                }
            }
        }
        &self.output
    }

    fn print_rule(&mut self, rule: &Rule, source: &str) {
        match rule {
            Rule::Style(r) => self.print_style_rule(r, source),
            Rule::AtRule(r) => self.print_at_rule(r, source),
        }
    }

    fn print_style_rule(&mut self, rule: &StyleRule, source: &str) {
        self.write_indent();
        self.print_selector_list(&rule.prelude, source);
        self.push_str(if self.minify { "{" } else { " {\n" });
        self.indent += 1;
        self.print_block_children(&rule.block, source);
        self.indent -= 1;
        self.write_indent();
        self.push_str("}\n");
    }

    fn print_at_rule(&mut self, rule: &AtRule, source: &str) {
        self.write_indent();
        self.push_str("@");
        self.push_span(rule.name, source);

        let prelude_text = rule.prelude.source_text(source).trim();
        if !prelude_text.is_empty() {
            self.push_str(" ");
            self.push_str(prelude_text);
        }

        if let Some(block) = &rule.block {
            self.push_str(if self.minify { "{" } else { " {\n" });
            self.indent += 1;
            self.print_block_children(block, source);
            self.indent -= 1;
            self.write_indent();
            self.push_str("}\n");
        } else {
            self.push_str(";\n");
        }
    }

    fn print_block_children(&mut self, block: &Block, source: &str) {
        for child in &block.children {
            match child {
                BlockChild::Declaration(d) => self.print_declaration(d, source),
                BlockChild::Rule(r) => self.print_rule(r, source),
                BlockChild::Comment(c) => {
                    self.write_indent();
                    self.push_span(c.span, source);
                    self.newline();
                }
            }
        }
    }

    fn print_declaration(&mut self, decl: &Declaration, source: &str) {
        self.write_indent();
        self.push_span(decl.property, source);
        self.push_str(if self.minify { ":" } else { ": " });
        self.push_span(decl.value, source);
        self.push_str(";\n");
    }

    fn print_selector_list(&mut self, list: &SelectorList, source: &str) {
        for (i, complex) in list.children.iter().enumerate() {
            if i > 0 {
                self.push_str(if self.minify { "," } else { ", " });
            }
            self.print_complex_selector(complex, source);
        }
    }

    fn print_complex_selector(&mut self, sel: &ComplexSelector, source: &str) {
        for (i, rel) in sel.children.iter().enumerate() {
            if let Some(combinator) = &rel.combinator
                && (i > 0 || combinator.kind != CombinatorKind::Descendant)
            {
                match combinator.kind {
                    CombinatorKind::Descendant => self.push_str(" "),
                    CombinatorKind::Child => self.push_str(" > "),
                    CombinatorKind::NextSibling => self.push_str(" + "),
                    CombinatorKind::SubsequentSibling => self.push_str(" ~ "),
                    CombinatorKind::Column => self.push_str(" || "),
                }
            }
            for simple in &rel.selectors {
                self.print_simple_selector(simple, source);
            }
        }
    }

    fn print_simple_selector(&mut self, sel: &SimpleSelector, source: &str) {
        match sel {
            SimpleSelector::Type(span)
            | SimpleSelector::Id(span)
            | SimpleSelector::Class(span)
            | SimpleSelector::Nesting(span)
            | SimpleSelector::Nth(span)
            | SimpleSelector::Percentage(span) => {
                self.push_span(*span, source);
            }
            SimpleSelector::PseudoClass(pc) => {
                self.push_str(":");
                self.push_span(pc.name, source);
                if let Some(args) = &pc.args {
                    self.push_str("(");
                    self.print_selector_list(args, source);
                    self.push_str(")");
                }
            }
            SimpleSelector::PseudoElement(pe) => {
                self.push_str("::");
                self.push_span(pe.name, source);
            }
            SimpleSelector::Attribute(attr) => {
                self.push_str("[");
                self.push_span(attr.name, source);
                if let Some(matcher) = attr.matcher {
                    self.push_span(matcher, source);
                    if let Some(value) = attr.value {
                        self.push_str("\"");
                        self.push_span(value, source);
                        self.push_str("\"");
                    }
                }
                if let Some(flags) = attr.flags {
                    self.push_str(" ");
                    self.push_span(flags, source);
                }
                self.push_str("]");
            }
        }
    }

    // -- output helpers -----------------------------------------------------

    fn push_str(&mut self, s: &str) {
        self.output.push_str(s);
    }

    fn push_span(&mut self, span: svelte_span::Span, source: &str) {
        self.output.push_str(span.source_text(source));
    }

    fn write_indent(&mut self) {
        if !self.minify {
            for _ in 0..self.indent {
                self.output.push_str("  ");
            }
        }
    }

    fn newline(&mut self) {
        if !self.minify {
            self.output.push('\n');
        }
    }
}

impl Default for Printer {
    fn default() -> Self {
        Self::new()
    }
}
