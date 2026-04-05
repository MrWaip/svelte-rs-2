use compact_str::CompactString;
use smallvec::SmallVec;
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
// CSS whitespace lookup (space, tab, newline, carriage return, form feed)
// ---------------------------------------------------------------------------

/// Lookup table: `true` for bytes that are CSS whitespace.
/// CSS defines whitespace as: U+0020 (space), U+0009 (tab), U+000A (LF),
/// U+000D (CR), U+000C (FF).
static CSS_WS: [bool; 256] = {
    let mut t = [false; 256];
    t[0x20] = true; // space
    t[0x09] = true; // tab
    t[0x0A] = true; // LF
    t[0x0D] = true; // CR
    t[0x0C] = true; // FF
    t
};

#[inline(always)]
fn is_css_ws(b: u8) -> bool {
    CSS_WS[b as usize]
}

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
    next_id: u32,
}

impl<'src> Parser<'src> {
    fn new(src: &'src str) -> Self {
        Self {
            src,
            bytes: src.as_bytes(),
            pos: 0,
            next_id: 0,
        }
    }

    #[inline(always)]
    fn alloc_id(&mut self) -> CssNodeId {
        let id = CssNodeId(self.next_id);
        self.next_id += 1;
        id
    }

    // -- helpers (hot path — all #[inline]) ---------------------------------

    #[inline(always)]
    fn at_end(&self) -> bool {
        self.pos >= self.bytes.len()
    }

    #[inline(always)]
    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.pos).copied()
    }

    #[inline(always)]
    fn peek_at(&self, offset: usize) -> Option<u8> {
        self.bytes.get(self.pos + offset).copied()
    }

    #[inline(always)]
    fn matches(&self, ch: u8) -> bool {
        self.peek() == Some(ch)
    }

    #[inline(always)]
    fn eat(&mut self, ch: u8) -> bool {
        if self.pos < self.bytes.len() && self.bytes[self.pos] == ch {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    #[inline(always)]
    fn eat2(&mut self, a: u8, b: u8) -> bool {
        if self.pos + 1 < self.bytes.len()
            && self.bytes[self.pos] == a
            && self.bytes[self.pos + 1] == b
        {
            self.pos += 2;
            true
        } else {
            false
        }
    }

    #[inline]
    fn matches_str(&self, s: &str) -> bool {
        self.bytes
            .get(self.pos..self.pos + s.len())
            .is_some_and(|slice| slice == s.as_bytes())
    }

    #[inline]
    fn eat_str(&mut self, s: &str) -> bool {
        if self.matches_str(s) {
            self.pos += s.len();
            true
        } else {
            false
        }
    }

    #[inline]
    fn expect(&mut self, ch: u8) -> Result<()> {
        if self.eat(ch) {
            Ok(())
        } else {
            Err(self.error(format!("expected '{}'", ch as char)))
        }
    }

    #[inline(always)]
    fn span_from(&self, start: usize) -> Span {
        Span::new(start as u32, self.pos as u32)
    }

    /// Build CompactString directly from byte range — no intermediate &str slice.
    #[inline]
    fn compact_str(&self, start: usize, end: usize) -> CompactString {
        // Safety: `self.src` is valid UTF-8, and `start..end` are always on
        // character boundaries (ensured by `advance_char` for non-ASCII).
        CompactString::new(&self.src[start..end])
    }

    #[cold]
    fn error(&self, message: String) -> ParseError {
        ParseError {
            message,
            pos: self.pos,
        }
    }

    /// Advance `pos` by one UTF-8 character. Caller must ensure `!at_end()`.
    #[inline(always)]
    fn advance_char(&mut self) {
        let b = self.bytes[self.pos];
        if b < 0x80 {
            self.pos += 1;
        } else if b < 0xE0 {
            self.pos += 2;
        } else if b < 0xF0 {
            self.pos += 3;
        } else {
            self.pos += 4;
        }
    }

    // -- whitespace & comments ----------------------------------------------

    #[inline]
    fn skip_whitespace(&mut self) {
        while self.pos < self.bytes.len() && is_css_ws(self.bytes[self.pos]) {
            self.pos += 1;
        }
    }

    fn skip_whitespace_and_comments(&mut self) {
        loop {
            self.skip_whitespace();
            if self.eat2(b'/', b'*') {
                self.scan_to_comment_end();
            } else if self.eat_str("<!--") {
                self.scan_to_html_comment_end();
            } else {
                break;
            }
        }
    }

    fn skip_whitespace_and_collect_comments<T>(
        &mut self,
        out: &mut Vec<T>,
        wrap: fn(Comment) -> T,
    ) {
        loop {
            self.skip_whitespace();
            if self.pos + 1 < self.bytes.len()
                && self.bytes[self.pos] == b'/'
                && self.bytes[self.pos + 1] == b'*'
            {
                let start = self.pos;
                self.pos += 2;
                self.scan_to_comment_end();
                out.push(wrap(Comment {
                    span: self.span_from(start),
                }));
            } else if self.eat_str("<!--") {
                self.scan_to_html_comment_end();
            } else {
                break;
            }
        }
    }

    #[inline]
    fn scan_to_comment_end(&mut self) {
        // All delimiters are ASCII — `*` and `/` never appear as
        // continuation bytes in valid UTF-8, so byte-level scan is safe.
        while self.pos + 1 < self.bytes.len() {
            if self.bytes[self.pos] == b'*' && self.bytes[self.pos + 1] == b'/' {
                self.pos += 2;
                return;
            }
            self.pos += 1;
        }
        // Consume trailing byte if we ran out of pairs
        if self.pos < self.bytes.len() {
            self.pos += 1;
        }
    }

    #[inline]
    fn scan_to_html_comment_end(&mut self) {
        while self.pos + 2 < self.bytes.len() {
            if self.bytes[self.pos] == b'-'
                && self.bytes[self.pos + 1] == b'-'
                && self.bytes[self.pos + 2] == b'>'
            {
                self.pos += 3;
                return;
            }
            self.pos += 1;
        }
        self.pos = self.bytes.len();
    }

    // -- identifiers --------------------------------------------------------

    #[inline(always)]
    fn is_ident_char(ch: u8) -> bool {
        ch.is_ascii_alphanumeric() || ch == b'_' || ch == b'-'
    }

    fn parse_ident(&mut self) -> Result<Span> {
        let start = self.pos;

        if self.at_end() {
            return Err(self.error("expected identifier".into()));
        }

        while self.pos < self.bytes.len() {
            let b = self.bytes[self.pos];
            if b == b'\\' {
                // CSS escape sequence
                self.pos += 1;
                if self.pos < self.bytes.len() {
                    if self.bytes[self.pos].is_ascii_hexdigit() {
                        let hex_start = self.pos;
                        while self.pos < self.bytes.len()
                            && self.pos - hex_start < 6
                            && self.bytes[self.pos].is_ascii_hexdigit()
                        {
                            self.pos += 1;
                        }
                        // Optional single trailing whitespace after hex escape
                        if self.pos < self.bytes.len() && is_css_ws(self.bytes[self.pos]) {
                            self.pos += 1;
                        }
                    } else {
                        self.advance_char();
                    }
                }
            } else if Self::is_ident_char(b) {
                self.pos += 1;
            } else if b >= 0x80 {
                self.advance_char();
            } else {
                break;
            }
        }

        if self.pos == start {
            return Err(self.error("expected identifier".into()));
        }

        Ok(self.span_from(start))
    }

    /// Parse identifier and return (span, name) pair.
    #[inline]
    fn parse_ident_with_name(&mut self) -> Result<(Span, CompactString)> {
        let span = self.parse_ident()?;
        let name = self.compact_str(span.start as usize, span.end as usize);
        Ok((span, name))
    }

    // -- value reading (raw text spans) -------------------------------------

    /// Scan a CSS value, respecting quotes, escapes, and url() context.
    /// Stops at unquoted `;`, `{`, or `}`. Returns span trimmed of trailing whitespace.
    fn read_value(&mut self) -> Span {
        let start = self.pos;
        let mut escaped = false;
        let mut in_url = false;
        let mut quote: u8 = 0;
        let mut last_non_ws = self.pos;

        while self.pos < self.bytes.len() {
            let b = self.bytes[self.pos];

            if escaped {
                escaped = false;
                self.advance_char();
                last_non_ws = self.pos;
                continue;
            }

            if b == b'\\' {
                escaped = true;
                self.pos += 1;
                continue;
            }

            if b == quote {
                quote = 0;
                self.pos += 1;
                last_non_ws = self.pos;
                continue;
            }

            if b == b')' {
                in_url = false;
            }

            if quote == 0 && (b == b'"' || b == b'\'') {
                quote = b;
                self.pos += 1;
                last_non_ws = self.pos;
                continue;
            }

            if quote == 0 && b == b'(' && self.pos >= 3 {
                let p = self.pos;
                if self.bytes[p - 3] == b'u'
                    && self.bytes[p - 2] == b'r'
                    && self.bytes[p - 1] == b'l'
                {
                    in_url = true;
                }
            }

            if !in_url && quote == 0 && (b == b';' || b == b'{' || b == b'}') {
                return Span::new(start as u32, last_non_ws as u32);
            }

            self.advance_char();
            if !is_css_ws(b) {
                last_non_ws = self.pos;
            }
        }

        Span::new(start as u32, last_non_ws as u32)
    }

    fn read_attribute_value(&mut self) -> Result<Span> {
        let quote = if self.eat(b'"') {
            b'"'
        } else if self.eat(b'\'') {
            b'\''
        } else {
            0
        };

        let start = self.pos;
        let mut escaped = false;

        while self.pos < self.bytes.len() {
            let b = self.bytes[self.pos];
            if escaped {
                escaped = false;
                self.advance_char();
                continue;
            }
            if b == b'\\' {
                escaped = true;
                self.pos += 1;
                continue;
            }

            if quote != 0 {
                if b == quote {
                    let span = self.span_from(start);
                    self.pos += 1;
                    return Ok(span);
                }
            } else if is_css_ws(b) || b == b']' {
                return Ok(self.span_from(start));
            }

            self.advance_char();
        }

        Err(self.error("unexpected end of input in attribute value".into()))
    }

    // -- block item lookahead -----------------------------------------------

    /// Single-pass lookahead: scan to the first unquoted `:` or `{` to
    /// determine whether a block item is a declaration (`:`) or nested rule (`{`).
    /// Does NOT advance `self.pos`.
    fn block_item_is_rule(&self) -> bool {
        let mut i = self.pos;
        let mut escaped = false;
        let mut quote: u8 = 0;
        let len = self.bytes.len();

        while i < len {
            let b = self.bytes[i];

            if escaped {
                escaped = false;
                // advance past escaped char (UTF-8 aware)
                if b < 0x80 {
                    i += 1;
                } else if b < 0xE0 {
                    i += 2;
                } else if b < 0xF0 {
                    i += 3;
                } else {
                    i += 4;
                }
                continue;
            }

            if b == b'\\' {
                escaped = true;
                i += 1;
                continue;
            }

            if b == quote {
                quote = 0;
                i += 1;
                continue;
            }

            if quote == 0 && (b == b'"' || b == b'\'') {
                quote = b;
                i += 1;
                continue;
            }

            if quote == 0 {
                match b {
                    b'{' => return true,
                    b':' => return false,
                    b';' | b'}' => return false,
                    _ => {}
                }
            }

            if b < 0x80 {
                i += 1;
            } else if b < 0xE0 {
                i += 2;
            } else if b < 0xF0 {
                i += 3;
            } else {
                i += 4;
            }
        }

        false
    }

    // -- nth patterns -------------------------------------------------------

    fn try_parse_nth(&mut self) -> Option<Span> {
        let start = self.pos;

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

        if self.matches(b'+') || self.matches(b'-') {
            self.pos += 1;
        }

        self.eat_digits();

        if self.eat(b'n') {
            self.skip_whitespace();
            if self.matches(b'+') || self.matches(b'-') {
                self.pos += 1;
                self.skip_whitespace();
                self.eat_digits();
            }
        }

        if self.pos == start {
            return None;
        }

        let saved = self.pos;
        self.skip_whitespace();
        if self.eat_str("of")
            && self.pos < self.bytes.len()
            && is_css_ws(self.bytes[self.pos])
        {
            self.pos = saved;
            self.skip_whitespace();
            self.eat_str("of");
            if self.pos < self.bytes.len() && is_css_ws(self.bytes[self.pos]) {
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

    #[inline(always)]
    fn is_nth_terminator(&self) -> bool {
        match self.peek() {
            None | Some(b')' | b',') => true,
            Some(b) => is_css_ws(b),
        }
    }

    #[inline]
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

        if self.eat2(b'|', b'|') {
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
            self.skip_whitespace_and_collect_comments(&mut children, StyleSheetChild::Comment);

            if self.at_end() {
                break;
            }

            let rule = if self.matches(b'@') {
                Rule::AtRule(self.parse_at_rule()?)
            } else {
                Rule::Style(Box::new(self.parse_style_rule()?))
            };
            children.push(StyleSheetChild::Rule(rule));
        }

        Ok(children)
    }

    fn parse_at_rule(&mut self) -> Result<AtRule> {
        let start = self.pos;
        self.expect(b'@')?;

        let (_, name) = self.parse_ident_with_name()?;
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
        let id = self.alloc_id();
        let start = self.pos;
        let prelude = self.parse_selector_list(false)?;
        let block = self.parse_block()?;

        Ok(StyleRule {
            id,
            span: self.span_from(start),
            prelude,
            block,
        })
    }

    // -- selectors ----------------------------------------------------------

    fn parse_selector_list(&mut self, inside_pseudo: bool) -> Result<SelectorList> {
        let mut children = SmallVec::new();

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
        let id = self.alloc_id();
        let list_start = self.pos;
        let mut children: RelativeSelectorVec = SmallVec::new();

        let mut rel = self.new_relative_selector(None);

        loop {
            if self.at_end() {
                return Err(self.error("unexpected end of input".into()));
            }

            let start = self.pos;

            if self.eat(b'&') {
                rel.selectors
                    .push(SimpleSelector::Nesting(self.span_from(start)));
            } else if self.eat(b'*') {
                let mut name_end = self.pos;
                if self.eat(b'|') {
                    let ident = self.parse_ident()?;
                    name_end = ident.end as usize;
                }
                let full_span = self.span_from(start);
                rel.selectors.push(SimpleSelector::Type {
                    span: full_span,
                    name: self.compact_str(start, name_end),
                });
            } else if self.eat(b'#') {
                let ident = self.parse_ident()?;
                rel.selectors.push(SimpleSelector::Id {
                    span: self.span_from(start),
                    name: self.compact_str(ident.start as usize, ident.end as usize),
                });
            } else if self.eat(b'.') {
                let ident = self.parse_ident()?;
                rel.selectors.push(SimpleSelector::Class {
                    span: self.span_from(start),
                    name: self.compact_str(ident.start as usize, ident.end as usize),
                });
            } else if self.eat2(b':', b':') {
                let (_, name) = self.parse_ident_with_name()?;
                if self.eat(b'(') {
                    self.parse_selector_list(true)?;
                    self.expect(b')')?;
                }
                rel.selectors
                    .push(SimpleSelector::PseudoElement(PseudoElementSelector {
                        span: self.span_from(start),
                        name,
                    }));
            } else if self.eat(b':') {
                let (_, name) = self.parse_ident_with_name()?;

                let args = if self.eat(b'(') {
                    let sel_list = self.parse_selector_list(true)?;
                    self.expect(b')')?;
                    Some(Box::new(sel_list))
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
                let (_, attr_name) = self.parse_ident_with_name()?;
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
                        name: attr_name,
                        matcher,
                        value,
                        flags,
                    }));
            } else if inside_pseudo {
                if let Some(nth) = self.try_parse_nth() {
                    rel.selectors.push(SimpleSelector::Nth(nth));
                } else if let Some(pct) = self.try_parse_percentage() {
                    rel.selectors.push(SimpleSelector::Percentage(pct));
                } else if !self.is_combinator_start() {
                    let ident_start = self.pos;
                    self.parse_ident()?;
                    let mut ident_end = self.pos;
                    if self.eat(b'|') {
                        self.parse_ident()?;
                        ident_end = self.pos;
                    }
                    rel.selectors.push(SimpleSelector::Type {
                        span: self.span_from(start),
                        name: self.compact_str(ident_start, ident_end),
                    });
                }
            } else if let Some(pct) = self.try_parse_percentage() {
                rel.selectors.push(SimpleSelector::Percentage(pct));
            } else if !self.is_combinator_start() {
                let ident_start = self.pos;
                self.parse_ident()?;
                let mut ident_end = self.pos;
                if self.eat(b'|') {
                    self.parse_ident()?;
                    ident_end = self.pos;
                }
                rel.selectors.push(SimpleSelector::Type {
                    span: self.span_from(start),
                    name: self.compact_str(ident_start, ident_end),
                });
            }

            // Check for selector list terminator
            let index = self.pos;
            self.skip_whitespace_and_comments();

            let terminator = if inside_pseudo { b')' } else { b'{' };
            if self.matches(b',') || self.matches(terminator) {
                self.pos = index;
                rel.span.end = index as u32;
                children.push(rel);

                return Ok(ComplexSelector {
                    id,
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

                rel = self.new_relative_selector(Some(combinator));

                self.skip_whitespace();

                if self.matches(b',') || self.matches(terminator) {
                    return Err(self.error("invalid selector".into()));
                }
            }
        }
    }

    fn new_relative_selector(&mut self, combinator: Option<Combinator>) -> RelativeSelector {
        let id = self.alloc_id();
        let start = combinator
            .as_ref()
            .map_or(self.pos as u32, |c| c.span.start);
        RelativeSelector {
            id,
            span: Span::new(start, 0),
            combinator,
            selectors: SmallVec::new(),
        }
    }

    #[inline(always)]
    fn is_combinator_start(&self) -> bool {
        match self.peek() {
            Some(b'+' | b'~' | b'>') => true,
            Some(b'|') => self.peek_at(1) == Some(b'|'),
            _ => false,
        }
    }

    fn try_parse_attr_matcher(&mut self) -> Option<Span> {
        let start = self.pos;

        match self.peek() {
            Some(b'~' | b'^' | b'$' | b'*' | b'|') => {
                self.pos += 1;
                if self.eat(b'=') {
                    return Some(self.span_from(start));
                }
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
            self.skip_whitespace_and_collect_comments(&mut children, BlockChild::Comment);

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

        // Single-pass lookahead: scan to first unquoted `:` or `{`
        // without advancing parser position.
        if self.block_item_is_rule() {
            Ok(BlockChild::Rule(Rule::Style(Box::new(
                self.parse_style_rule()?,
            ))))
        } else {
            Ok(BlockChild::Declaration(self.parse_declaration()?))
        }
    }

    fn parse_declaration(&mut self) -> Result<Declaration> {
        let start = self.pos;

        // Property name: scan ASCII bytes until whitespace or `:`
        let prop_start = self.pos;
        while self.pos < self.bytes.len() {
            let b = self.bytes[self.pos];
            if is_css_ws(b) || b == b':' {
                break;
            }
            // Property names are ASCII (including `--custom-prop`)
            self.pos += 1;
        }
        let property = self.span_from(prop_start);

        self.skip_whitespace();
        self.expect(b':')?;
        self.skip_whitespace();

        let value = self.read_value();

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
