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

    /// Convenience: print stylesheet to string.
    pub fn print(stylesheet: &StyleSheet, source: &str) -> String {
        // Pre-allocate ~same size as source (CSS output is similar length)
        let mut p = Printer {
            output: String::with_capacity(source.len()),
            indent: 0,
            minify: false,
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
                    first = false;
                    self.print_rule(rule, source);
                }
                StyleSheetChild::Error(_) => {}
            }
        }
        &self.output
    }

    fn print_rule(&mut self, rule: &Rule, source: &str) {
        match rule {
            Rule::Style(r) => self.print_style_rule(r.as_ref(), source),
            Rule::AtRule(r) => self.print_at_rule(r, source),
        }
    }

    fn print_style_rule(&mut self, rule: &StyleRule, source: &str) {
        self.write_indent();
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
    }

    fn print_at_rule(&mut self, rule: &AtRule, source: &str) {
        self.write_indent();
        self.output.push('@');
        self.output.push_str(&rule.name);

        let prelude_text = rule.prelude.source_text(source).trim();
        if !prelude_text.is_empty() {
            self.output.push(' ');
            self.output.push_str(prelude_text);
        }

        if let Some(block) = &rule.block {
            if self.minify {
                self.output.push('{');
            } else {
                self.output.push_str(" {\n");
            }
            self.indent += 1;
            self.print_block_children(block, source);
            self.indent -= 1;
            self.write_indent();
            self.output.push_str("}\n");
        } else {
            self.output.push_str(";\n");
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
        self.push_span(decl.value, source);
        self.output.push_str(";\n");
    }

    fn print_selector_list(&mut self, list: &SelectorList, source: &str) {
        for (i, complex) in list.children.iter().enumerate() {
            if i > 0 {
                if self.minify {
                    self.output.push(',');
                } else {
                    self.output.push_str(", ");
                }
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
                    CombinatorKind::Descendant => self.output.push(' '),
                    CombinatorKind::Child => self.output.push_str(" > "),
                    CombinatorKind::NextSibling => self.output.push_str(" + "),
                    CombinatorKind::SubsequentSibling => self.output.push_str(" ~ "),
                    CombinatorKind::Column => self.output.push_str(" || "),
                }
            }
            for simple in &rel.selectors {
                self.print_simple_selector(simple, source);
            }
        }
    }

    fn print_simple_selector(&mut self, sel: &SimpleSelector, source: &str) {
        match sel {
            SimpleSelector::Type { span, .. }
            | SimpleSelector::Id { span, .. }
            | SimpleSelector::Class { span, .. }
            | SimpleSelector::Nesting(span)
            | SimpleSelector::Nth(span)
            | SimpleSelector::Percentage(span) => {
                self.push_span(*span, source);
            }
            SimpleSelector::PseudoClass(pc) => {
                self.output.push(':');
                self.output.push_str(&pc.name);
                if let Some(args) = &pc.args {
                    self.output.push('(');
                    self.print_selector_list(args.as_ref(), source);
                    self.output.push(')');
                }
            }
            SimpleSelector::PseudoElement(pe) => {
                self.output.push_str("::");
                self.output.push_str(&pe.name);
                if let Some(args) = &pe.args {
                    self.output.push('(');
                    self.print_selector_list(args.as_ref(), source);
                    self.output.push(')');
                }
            }
            SimpleSelector::Attribute(attr) => {
                self.output.push('[');
                self.output.push_str(&attr.name);
                if let Some(matcher) = attr.matcher {
                    self.push_span(matcher, source);
                    if let Some(value) = attr.value {
                        self.output.push('"');
                        self.push_span(value, source);
                        self.output.push('"');
                    }
                }
                if let Some(flags) = attr.flags {
                    self.output.push(' ');
                    self.push_span(flags, source);
                }
                self.output.push(']');
            }
        }
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

impl Default for Printer {
    fn default() -> Self {
        Self::new()
    }
}
