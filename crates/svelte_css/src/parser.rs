use svelte_span::Span;

use crate::ast::*;

// ---------------------------------------------------------------------------
// Error
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub struct ParseError {
    pub message: String,
    pub pos: usize,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CSS parse error at {}: {}", self.pos, self.message)
    }
}

impl std::error::Error for ParseError {}

type Result<T> = std::result::Result<T, ParseError>;

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

pub fn parse(source: &str) -> Result<StyleSheet> {
    let mut parser = Parser::new(source);
    parser.parse_stylesheet()
}

// ---------------------------------------------------------------------------
// Parser
// ---------------------------------------------------------------------------

struct Parser<'src> {
    src: &'src str,
    bytes: &'src [u8],
    pos: usize,
}

impl<'src> Parser<'src> {
    fn new(src: &'src str) -> Self {
        Self {
            src,
            bytes: src.as_bytes(),
            pos: 0,
        }
    }

    // -- helpers ------------------------------------------------------------

    fn at_end(&self) -> bool {
        self.pos >= self.bytes.len()
    }

    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.pos).copied()
    }

    fn peek_at(&self, offset: usize) -> Option<u8> {
        self.bytes.get(self.pos + offset).copied()
    }

    fn current_char(&self) -> Option<char> {
        self.src[self.pos..].chars().next()
    }

    fn eat(&mut self, ch: u8) -> bool {
        if self.peek() == Some(ch) {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    fn eat_str(&mut self, s: &str) -> bool {
        if self.src[self.pos..].starts_with(s) {
            self.pos += s.len();
            true
        } else {
            false
        }
    }

    fn expect(&mut self, ch: u8) -> Result<()> {
        if self.eat(ch) {
            Ok(())
        } else {
            Err(self.error(format!("expected '{}'", ch as char)))
        }
    }

    fn matches(&self, ch: u8) -> bool {
        self.peek() == Some(ch)
    }

    fn matches_str(&self, s: &str) -> bool {
        self.src[self.pos..].starts_with(s)
    }

    fn span_from(&self, start: usize) -> Span {
        Span::new(start as u32, self.pos as u32)
    }

    fn error(&self, message: String) -> ParseError {
        ParseError {
            message,
            pos: self.pos,
        }
    }

    // -- whitespace & comments ----------------------------------------------

    fn skip_whitespace(&mut self) {
        while self.pos < self.bytes.len() && self.bytes[self.pos].is_ascii_whitespace() {
            self.pos += 1;
        }
    }

    /// Skip whitespace and comments (discarding comments).
    /// Used inside selectors and combinators.
    fn skip_whitespace_and_comments(&mut self) {
        self.skip_whitespace();
        while self.matches_str("/*") || self.matches_str("<!--") {
            if self.eat_str("/*") {
                self.read_until_str("*/");
                self.eat_str("*/");
            }
            if self.eat_str("<!--") {
                self.read_until_str("-->");
                self.eat_str("-->");
            }
            self.skip_whitespace();
        }
    }

    /// Skip whitespace and collect `/* ... */` comments as AST nodes.
    /// Used in stylesheet body and block children.
    fn collect_comments(&mut self, out: &mut Vec<Comment>) {
        self.skip_whitespace();
        while self.matches_str("/*") || self.matches_str("<!--") {
            if self.matches_str("/*") {
                let start = self.pos;
                self.pos += 2; // skip /*
                self.read_until_str("*/");
                self.eat_str("*/");
                out.push(Comment {
                    span: self.span_from(start),
                });
            }
            if self.eat_str("<!--") {
                self.read_until_str("-->");
                self.eat_str("-->");
            }
            self.skip_whitespace();
        }
    }

    /// Advance until `needle` is found (or EOF). Does NOT consume `needle`.
    fn read_until_str(&mut self, needle: &str) {
        while self.pos < self.bytes.len() && !self.src[self.pos..].starts_with(needle) {
            self.pos += 1;
        }
    }

    // -- identifiers --------------------------------------------------------

    fn is_ident_char(ch: u8) -> bool {
        ch.is_ascii_alphanumeric() || ch == b'_' || ch == b'-'
    }

    fn parse_ident(&mut self) -> Result<Span> {
        let start = self.pos;

        // First char: must not start with digit (but leading hyphen is ok for
        // custom properties like --foo, and ident-start chars)
        if self.at_end() {
            return Err(self.error("expected identifier".into()));
        }

        while self.pos < self.bytes.len() {
            let b = self.bytes[self.pos];
            if b == b'\\' {
                // Escape sequence: skip \ + next char (simplified)
                self.pos += 1;
                if self.pos < self.bytes.len() {
                    // Check for unicode escape: \XXXXXX
                    if self.bytes[self.pos].is_ascii_hexdigit() {
                        let hex_start = self.pos;
                        while self.pos < self.bytes.len()
                            && self.pos - hex_start < 6
                            && self.bytes[self.pos].is_ascii_hexdigit()
                        {
                            self.pos += 1;
                        }
                        // Optional trailing whitespace after unicode escape
                        if self.pos < self.bytes.len()
                            && self.bytes[self.pos].is_ascii_whitespace()
                        {
                            self.pos += 1;
                        }
                    } else {
                        self.pos += 1;
                    }
                }
            } else if Self::is_ident_char(b) {
                self.pos += 1;
            } else if b >= 0x80 {
                // Non-ASCII: advance by full UTF-8 char
                if let Some(ch) = self.current_char() {
                    self.pos += ch.len_utf8();
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        if self.pos == start {
            return Err(self.error("expected identifier".into()));
        }

        Ok(self.span_from(start))
    }

    // -- value reading (raw text spans) -------------------------------------

    /// Read raw CSS value text until `;`, `{`, or `}` (outside strings/escapes).
    /// Returns the span of the value, trimmed.
    fn read_value(&mut self) -> Span {
        let start = self.pos;
        let mut escaped = false;
        let mut in_url = false;
        let mut quote: Option<u8> = None;
        let mut last_non_ws = self.pos;

        while self.pos < self.bytes.len() {
            let b = self.bytes[self.pos];

            if escaped {
                escaped = false;
                self.pos += 1;
                last_non_ws = self.pos;
                continue;
            }

            if b == b'\\' {
                escaped = true;
                self.pos += 1;
                continue;
            }

            if Some(b) == quote {
                quote = None;
                self.pos += 1;
                last_non_ws = self.pos;
                continue;
            }

            if b == b')' {
                in_url = false;
            }

            if quote.is_none() && (b == b'"' || b == b'\'') {
                quote = Some(b);
                self.pos += 1;
                last_non_ws = self.pos;
                continue;
            }

            // Detect url(
            if quote.is_none()
                && b == b'('
                && self.pos >= 3
                && &self.src[self.pos - 3..self.pos] == "url"
            {
                in_url = true;
            }

            if !in_url && quote.is_none() && (b == b';' || b == b'{' || b == b'}') {
                // Trim trailing whitespace
                return Span::new(start as u32, last_non_ws as u32);
            }

            self.pos += 1;
            if !b.is_ascii_whitespace() {
                last_non_ws = self.pos;
            }
        }

        Span::new(start as u32, last_non_ws as u32)
    }

    /// Read an attribute value — quoted or unquoted — inside `[...]`.
    fn read_attribute_value(&mut self) -> Result<Span> {
        let quote = if self.eat(b'"') {
            Some(b'"')
        } else if self.eat(b'\'') {
            Some(b'\'')
        } else {
            None
        };

        let start = self.pos;
        let mut escaped = false;

        while self.pos < self.bytes.len() {
            let b = self.bytes[self.pos];
            if escaped {
                escaped = false;
                self.pos += 1;
                continue;
            }
            if b == b'\\' {
                escaped = true;
                self.pos += 1;
                continue;
            }

            if let Some(q) = quote {
                if b == q {
                    let span = self.span_from(start);
                    self.pos += 1; // eat closing quote
                    return Ok(span);
                }
            } else if b.is_ascii_whitespace() || b == b']' {
                return Ok(self.span_from(start));
            }

            self.pos += 1;
        }

        Err(self.error("unexpected end of input in attribute value".into()))
    }

    // -- nth patterns -------------------------------------------------------

    /// Try to parse an An+B / nth pattern (even, odd, 2n+1, etc).
    /// Returns None if not matching; does not advance on failure.
    fn try_parse_nth(&mut self) -> Option<Span> {
        let start = self.pos;

        // Try: even | odd
        if self.eat_str("even") {
            if self.is_nth_terminator() {
                return Some(self.span_from(start));
            }
            self.pos = start;
            return None;
        }
        if self.eat_str("odd") {
            if self.is_nth_terminator() {
                return Some(self.span_from(start));
            }
            self.pos = start;
            return None;
        }

        // Try: [+|-]?\d*n?(\s*[+-]\s*\d+)?
        // Optional leading sign
        if self.matches(b'+') || self.matches(b'-') {
            self.pos += 1;
        }

        // Digits
        self.eat_digits();

        // Optional 'n'
        if self.eat(b'n') {
            self.skip_whitespace();
            // Optional [+-] digits
            if self.matches(b'+') || self.matches(b'-') {
                self.pos += 1;
                self.skip_whitespace();
                self.eat_digits();
            }
        }

        if self.pos == start {
            return None;
        }

        // Check for "of" keyword or terminator
        let saved = self.pos;
        self.skip_whitespace();
        if self.eat_str("of") && self.pos < self.bytes.len() && self.bytes[self.pos].is_ascii_whitespace() {
            // "of" keyword — include it
            // The span goes up to after "of" + space consumed by read_selector
            self.pos = saved;
            self.skip_whitespace();
            self.eat_str("of");
            // include trailing whitespace
            if self.pos < self.bytes.len() && self.bytes[self.pos].is_ascii_whitespace() {
                self.pos += 1;
            }
            return Some(self.span_from(start));
        }
        self.pos = saved;

        if self.is_nth_terminator() {
            return Some(self.span_from(start));
        }

        self.pos = start;
        None
    }

    fn is_nth_terminator(&self) -> bool {
        matches!(self.peek(), Some(b')' | b',') | None)
            || self.peek().is_some_and(|b| b.is_ascii_whitespace())
    }

    fn eat_digits(&mut self) {
        while self.pos < self.bytes.len() && self.bytes[self.pos].is_ascii_digit() {
            self.pos += 1;
        }
    }

    // -- percentage ---------------------------------------------------------

    fn try_parse_percentage(&mut self) -> Option<Span> {
        let start = self.pos;
        self.eat_digits();
        if self.pos > start && self.eat(b'.') {
            self.eat_digits();
        }
        if self.pos > start && self.eat(b'%') {
            return Some(self.span_from(start));
        }
        self.pos = start;
        None
    }

    // -- combinator ---------------------------------------------------------

    fn try_parse_combinator(&mut self) -> Option<Combinator> {
        let start = self.pos;
        self.skip_whitespace();

        let index = self.pos;

        // Explicit combinators: +, ~, >, ||
        if self.eat_str("||") {
            return Some(Combinator {
                span: Span::new(index as u32, self.pos as u32),
                kind: CombinatorKind::Column,
            });
        }
        if let Some(&b) = self.bytes.get(self.pos) {
            let kind = match b {
                b'>' => Some(CombinatorKind::Child),
                b'+' => Some(CombinatorKind::NextSibling),
                b'~' => Some(CombinatorKind::SubsequentSibling),
                _ => None,
            };
            if let Some(kind) = kind {
                self.pos += 1;
                return Some(Combinator {
                    span: Span::new(index as u32, self.pos as u32),
                    kind,
                });
            }
        }

        // Descendant combinator: whitespace only (position moved from start)
        if self.pos != start {
            return Some(Combinator {
                span: Span::new(start as u32, self.pos as u32),
                kind: CombinatorKind::Descendant,
            });
        }

        None
    }

    // =======================================================================
    // Parsing methods
    // =======================================================================

    fn parse_stylesheet(&mut self) -> Result<StyleSheet> {
        let start = self.pos;
        let children = self.parse_stylesheet_body()?;
        Ok(StyleSheet {
            span: self.span_from(start),
            children,
        })
    }

    fn parse_stylesheet_body(&mut self) -> Result<Vec<StyleSheetChild>> {
        let mut children = Vec::new();

        loop {
            // Collect comments at this level
            let mut comments = Vec::new();
            self.collect_comments(&mut comments);
            children.extend(comments.into_iter().map(StyleSheetChild::Comment));

            if self.at_end() {
                break;
            }

            let rule = if self.matches(b'@') {
                Rule::AtRule(self.parse_at_rule()?)
            } else {
                Rule::Style(self.parse_style_rule()?)
            };
            children.push(StyleSheetChild::Rule(rule));
        }

        Ok(children)
    }

    fn parse_at_rule(&mut self) -> Result<AtRule> {
        let start = self.pos;
        self.expect(b'@')?;

        let name = self.parse_ident()?;
        let prelude = self.read_value();

        let block = if self.matches(b'{') {
            Some(self.parse_block()?)
        } else {
            self.expect(b';')?;
            None
        };

        Ok(AtRule {
            span: self.span_from(start),
            name,
            prelude,
            block,
        })
    }

    fn parse_style_rule(&mut self) -> Result<StyleRule> {
        let start = self.pos;
        let prelude = self.parse_selector_list(false)?;
        let block = self.parse_block()?;

        Ok(StyleRule {
            span: self.span_from(start),
            prelude,
            block,
            metadata: RuleMetadata::default(),
        })
    }

    // -- selectors ----------------------------------------------------------

    fn parse_selector_list(&mut self, inside_pseudo: bool) -> Result<SelectorList> {
        let mut children = Vec::new();

        self.skip_whitespace_and_comments();
        let start = self.pos;

        loop {
            if self.at_end() {
                return Err(self.error("unexpected end of input".into()));
            }

            children.push(self.parse_complex_selector(inside_pseudo)?);
            let end = self.pos;

            self.skip_whitespace_and_comments();

            let terminator = if inside_pseudo { b')' } else { b'{' };
            if self.matches(terminator) {
                return Ok(SelectorList {
                    span: Span::new(start as u32, end as u32),
                    children,
                });
            }

            self.expect(b',')?;
            self.skip_whitespace_and_comments();
        }
    }

    fn parse_complex_selector(&mut self, inside_pseudo: bool) -> Result<ComplexSelector> {
        let list_start = self.pos;
        let mut children = Vec::new();

        let mut rel = RelativeSelector {
            span: Span::new(self.pos as u32, 0),
            combinator: None,
            selectors: Vec::new(),
        };

        loop {
            if self.at_end() {
                return Err(self.error("unexpected end of input".into()));
            }

            let start = self.pos;

            if self.eat(b'&') {
                rel.selectors
                    .push(SimpleSelector::Nesting(self.span_from(start)));
            } else if self.eat(b'*') {
                // Possible namespace: *|ident
                if self.eat(b'|') {
                    self.parse_ident()?;
                }
                rel.selectors
                    .push(SimpleSelector::Type(self.span_from(start)));
            } else if self.eat(b'#') {
                self.parse_ident()?;
                rel.selectors
                    .push(SimpleSelector::Id(self.span_from(start)));
            } else if self.eat(b'.') {
                self.parse_ident()?;
                rel.selectors
                    .push(SimpleSelector::Class(self.span_from(start)));
            } else if self.eat_str("::") {
                self.parse_ident()?;
                let pseudo_span = self.span_from(start);
                // Read inner selectors of pseudo-element (discard)
                if self.eat(b'(') {
                    self.parse_selector_list(true)?;
                    self.expect(b')')?;
                }
                let name_span = Span::new(start as u32 + 2, pseudo_span.end);
                rel.selectors
                    .push(SimpleSelector::PseudoElement(PseudoElementSelector {
                        span: self.span_from(start),
                        name: name_span,
                    }));
            } else if self.eat(b':') {
                let name = self.parse_ident()?;

                let args = if self.eat(b'(') {
                    let sel_list = self.parse_selector_list(true)?;
                    self.expect(b')')?;
                    Some(sel_list)
                } else {
                    None
                };

                rel.selectors
                    .push(SimpleSelector::PseudoClass(PseudoClassSelector {
                        span: self.span_from(start),
                        name,
                        args,
                    }));
            } else if self.eat(b'[') {
                self.skip_whitespace();
                let name = self.parse_ident()?;
                self.skip_whitespace();

                let matcher = self.try_parse_attr_matcher();

                let value = if matcher.is_some() {
                    self.skip_whitespace();
                    Some(self.read_attribute_value()?)
                } else {
                    None
                };

                self.skip_whitespace();
                let flags = self.try_parse_attr_flags();
                self.skip_whitespace();
                self.expect(b']')?;

                rel.selectors
                    .push(SimpleSelector::Attribute(AttributeSelector {
                        span: self.span_from(start),
                        name,
                        matcher,
                        value,
                        flags,
                    }));
            } else if inside_pseudo {
                // Try nth pattern before combinator (to avoid + collision)
                if let Some(nth) = self.try_parse_nth() {
                    rel.selectors.push(SimpleSelector::Nth(nth));
                } else if let Some(pct) = self.try_parse_percentage() {
                    rel.selectors.push(SimpleSelector::Percentage(pct));
                } else if !self.is_combinator_start() {
                    // Type selector
                    let mut _name = self.parse_ident()?;
                    if self.eat(b'|') {
                        _name = self.parse_ident()?;
                    }
                    rel.selectors
                        .push(SimpleSelector::Type(self.span_from(start)));
                }
            } else if let Some(pct) = self.try_parse_percentage() {
                rel.selectors.push(SimpleSelector::Percentage(pct));
            } else if !self.is_combinator_start() {
                // Type selector
                let mut _name = self.parse_ident()?;
                if self.eat(b'|') {
                    _name = self.parse_ident()?;
                }
                rel.selectors
                    .push(SimpleSelector::Type(self.span_from(start)));
            }

            // Check for selector list terminator
            let index = self.pos;
            self.skip_whitespace_and_comments();

            let terminator = if inside_pseudo { b')' } else { b'{' };
            if self.matches(b',') || self.matches(terminator) {
                // Rewind — let the selector_list handler see the terminator
                self.pos = index;
                rel.span.end = index as u32;
                children.push(rel);

                return Ok(ComplexSelector {
                    span: Span::new(list_start as u32, index as u32),
                    children,
                });
            }

            // Try combinator
            self.pos = index;
            if let Some(combinator) = self.try_parse_combinator() {
                if !rel.selectors.is_empty() {
                    rel.span.end = index as u32;
                    children.push(rel);
                }

                rel = RelativeSelector {
                    span: Span::new(combinator.span.start, 0),
                    combinator: Some(combinator),
                    selectors: Vec::new(),
                };

                self.skip_whitespace();

                // After combinator, check we're not at a terminator
                if self.matches(b',') || self.matches(terminator) {
                    return Err(self.error("invalid selector".into()));
                }
            }
        }
    }

    fn is_combinator_start(&self) -> bool {
        matches!(self.peek(), Some(b'+' | b'~' | b'>'))
            || (self.peek() == Some(b'|') && self.peek_at(1) == Some(b'|'))
    }

    fn try_parse_attr_matcher(&mut self) -> Option<Span> {
        let start = self.pos;

        // Optional prefix: ~, ^, $, *, |
        match self.peek() {
            Some(b'~' | b'^' | b'$' | b'*' | b'|') => {
                self.pos += 1;
                if self.eat(b'=') {
                    return Some(self.span_from(start));
                }
                // '|' without '=' is namespace, not matcher — rewind
                self.pos = start;
            }
            Some(b'=') => {
                self.pos += 1;
                return Some(self.span_from(start));
            }
            _ => {}
        }

        None
    }

    fn try_parse_attr_flags(&mut self) -> Option<Span> {
        let start = self.pos;
        while self.pos < self.bytes.len() && self.bytes[self.pos].is_ascii_alphabetic() {
            self.pos += 1;
        }
        if self.pos > start {
            Some(self.span_from(start))
        } else {
            None
        }
    }

    // -- blocks & declarations ----------------------------------------------

    fn parse_block(&mut self) -> Result<Block> {
        let start = self.pos;
        self.expect(b'{')?;

        let mut children = Vec::new();

        loop {
            let mut comments = Vec::new();
            self.collect_comments(&mut comments);
            children.extend(comments.into_iter().map(BlockChild::Comment));

            if self.matches(b'}') {
                break;
            }
            if self.at_end() {
                return Err(self.error("unexpected end of input — unclosed block".into()));
            }

            children.push(self.parse_block_item()?);
        }

        self.expect(b'}')?;

        Ok(Block {
            span: self.span_from(start),
            children,
        })
    }

    fn parse_block_item(&mut self) -> Result<BlockChild> {
        if self.matches(b'@') {
            return Ok(BlockChild::Rule(Rule::AtRule(self.parse_at_rule()?)));
        }

        // Lookahead: read_value, then check if next is `{` (rule) or not (declaration)
        let saved = self.pos;
        self.read_value();
        let is_rule = self.matches(b'{');
        self.pos = saved;

        if is_rule {
            Ok(BlockChild::Rule(Rule::Style(self.parse_style_rule()?)))
        } else {
            Ok(BlockChild::Declaration(self.parse_declaration()?))
        }
    }

    fn parse_declaration(&mut self) -> Result<Declaration> {
        let start = self.pos;

        // Read property name (until whitespace or colon)
        let prop_start = self.pos;
        while self.pos < self.bytes.len() {
            let b = self.bytes[self.pos];
            if b.is_ascii_whitespace() || b == b':' {
                break;
            }
            self.pos += 1;
        }
        let property = self.span_from(prop_start);

        self.skip_whitespace();
        self.expect(b':')?;
        self.skip_whitespace();

        let value = self.read_value();

        // Check for empty declaration value (allowed for custom properties)
        if value.start == value.end {
            let prop_text = property.source_text(self.src);
            if !prop_text.starts_with("--") {
                return Err(ParseError {
                    message: "empty declaration value".into(),
                    pos: start,
                });
            }
        }

        let end = self.pos;

        if !self.matches(b'}') {
            self.expect(b';')?;
        }

        Ok(Declaration {
            span: Span::new(start as u32, end as u32),
            property,
            value,
        })
    }
}
