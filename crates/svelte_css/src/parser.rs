use compact_str::CompactString;
use smallvec::SmallVec;
use svelte_diagnostics::{Diagnostic, DiagnosticKind};
use svelte_span::Span;

use crate::ast::*;

// ---------------------------------------------------------------------------
// CSS whitespace lookup (space, tab, newline, carriage return, form feed)
// ---------------------------------------------------------------------------

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

/// Parse CSS source into a StyleSheet AST.
/// Always returns an AST (best-effort on errors) plus accumulated diagnostics.
pub fn parse(source: &str) -> (StyleSheet, Vec<Diagnostic>) {
    let mut parser = Parser::new(source);
    let stylesheet = parser.parse_stylesheet();
    (stylesheet, parser.diagnostics)
}

// ---------------------------------------------------------------------------
// Parser
// ---------------------------------------------------------------------------

struct Parser<'src> {
    src: &'src str,
    bytes: &'src [u8],
    pos: usize,
    next_id: u32,
    diagnostics: Vec<Diagnostic>,
}

impl<'src> Parser<'src> {
    fn new(src: &'src str) -> Self {
        Self {
            src,
            bytes: src.as_bytes(),
            pos: 0,
            next_id: 0,
            diagnostics: Vec::new(),
        }
    }

    #[inline(always)]
    fn alloc_id(&mut self) -> CssNodeId {
        let id = CssNodeId(self.next_id);
        self.next_id += 1;
        id
    }

    // -- diagnostics --------------------------------------------------------

    #[cold]
    fn recover(&mut self, kind: DiagnosticKind, span: Span) {
        self.diagnostics.push(Diagnostic::error(kind, span));
    }

    // -- helpers (hot path) -------------------------------------------------

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

    #[inline(always)]
    fn span_from(&self, start: usize) -> Span {
        Span::new(start as u32, self.pos as u32)
    }

    #[inline(always)]
    fn span_at(&self) -> Span {
        let p = self.pos as u32;
        Span::new(p, p)
    }

    #[inline]
    fn compact_str(&self, start: usize, end: usize) -> CompactString {
        CompactString::new(&self.src[start..end])
    }

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

    // -- recovery helpers ---------------------------------------------------

    /// Skip to next unquoted `}` or end of input, consuming the `}`.
    /// Used to recover from errors inside a block.
    fn skip_to_block_end(&mut self) {
        let mut depth: u32 = 0;
        let mut quote: u8 = 0;
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
            if b == quote {
                quote = 0;
                self.pos += 1;
                continue;
            }
            if quote == 0 && (b == b'"' || b == b'\'') {
                quote = b;
                self.pos += 1;
                continue;
            }
            if quote == 0 {
                match b {
                    b'{' => depth += 1,
                    b'}' => {
                        if depth == 0 {
                            self.pos += 1;
                            return;
                        }
                        depth -= 1;
                    }
                    _ => {}
                }
            }
            self.advance_char();
        }
    }

    /// Skip to next unquoted `;` or `}` (without consuming `}`), or end of input.
    /// Used to recover from bad declarations.
    fn skip_to_semicolon_or_block_end(&mut self) {
        let mut quote: u8 = 0;
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
            if b == quote {
                quote = 0;
                self.pos += 1;
                continue;
            }
            if quote == 0 && (b == b'"' || b == b'\'') {
                quote = b;
                self.pos += 1;
                continue;
            }
            if quote == 0 {
                match b {
                    b';' => {
                        self.pos += 1;
                        return;
                    }
                    b'}' => return, // don't consume — caller handles block close
                    _ => {}
                }
            }
            self.advance_char();
        }
    }

    /// Skip an entire rule: selector part + `{ ... }`.
    /// Used when selector parsing fails.
    fn skip_rule(&mut self) {
        let mut quote: u8 = 0;
        let mut escaped = false;

        // Phase 1: skip to `{`
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
            if b == quote {
                quote = 0;
                self.pos += 1;
                continue;
            }
            if quote == 0 && (b == b'"' || b == b'\'') {
                quote = b;
                self.pos += 1;
                continue;
            }
            if quote == 0 {
                match b {
                    b'{' => {
                        self.pos += 1;
                        // Phase 2: skip matched block
                        self.skip_to_block_end();
                        return;
                    }
                    b';' => {
                        self.pos += 1;
                        return;
                    }
                    b'}' => return,
                    _ => {}
                }
            }
            self.advance_char();
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
        while self.pos + 1 < self.bytes.len() {
            if self.bytes[self.pos] == b'*' && self.bytes[self.pos + 1] == b'/' {
                self.pos += 2;
                return;
            }
            self.pos += 1;
        }
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

    /// Parse a CSS identifier. Returns `None` on failure (emits diagnostic).
    fn parse_ident(&mut self) -> Option<Span> {
        let start = self.pos;

        if self.at_end() {
            self.recover(DiagnosticKind::CssExpectedIdentifier, self.span_at());
            return None;
        }

        while self.pos < self.bytes.len() {
            let b = self.bytes[self.pos];
            if b == b'\\' {
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
            self.recover(DiagnosticKind::CssExpectedIdentifier, self.span_at());
            return None;
        }

        Some(self.span_from(start))
    }

    /// Parse identifier and return (span, name) pair. Returns `None` on failure.
    #[inline]
    fn parse_ident_with_name(&mut self) -> Option<(Span, CompactString)> {
        let span = self.parse_ident()?;
        let name = self.compact_str(span.start as usize, span.end as usize);
        Some((span, name))
    }

    // -- value reading (raw text spans) -------------------------------------

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
                if self.bytes[p - 1] == b'l'
                    && self.bytes[p - 2] == b'r'
                    && self.bytes[p - 3] == b'u'
                    && (p < 4 || !Self::is_ident_char(self.bytes[p - 4]))
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

    fn read_attribute_value(&mut self) -> Option<Span> {
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
                    return Some(span);
                }
            } else if is_css_ws(b) || b == b']' {
                return Some(self.span_from(start));
            }

            self.advance_char();
        }

        self.recover(
            DiagnosticKind::CssSelectorInvalid,
            self.span_from(start),
        );
        None
    }

    // -- block item lookahead -----------------------------------------------

    /// Lookahead: is the current block item a nested rule (`{`) or a declaration (`;`/`}`)?
    /// Scans to first unquoted, non-paren-nested `{`, `}`, or `;`.
    /// Does NOT use `:` — pseudo-classes like `a:hover {}` contain `:` but are rules.
    fn block_item_is_rule(&self) -> bool {
        let mut i = self.pos;
        let mut escaped = false;
        let mut quote: u8 = 0;
        let mut paren_depth: u32 = 0;
        let len = self.bytes.len();

        while i < len {
            let b = self.bytes[i];

            if escaped {
                escaped = false;
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
                    b'(' => paren_depth += 1,
                    b')' => paren_depth = paren_depth.saturating_sub(1),
                    b'{' if paren_depth == 0 => return true,
                    b';' | b'}' if paren_depth == 0 => return false,
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

    fn parse_stylesheet(&mut self) -> StyleSheet {
        let start = self.pos;
        let children = self.parse_stylesheet_body();
        StyleSheet {
            span: self.span_from(start),
            children,
        }
    }

    fn parse_stylesheet_body(&mut self) -> Vec<StyleSheetChild> {
        let mut children = Vec::new();

        loop {
            self.skip_whitespace_and_collect_comments(&mut children, StyleSheetChild::Comment);

            if self.at_end() {
                break;
            }

            let start = self.pos;
            if self.matches(b'@') {
                if let Some(at) = self.parse_at_rule() {
                    children.push(StyleSheetChild::Rule(Rule::AtRule(at)));
                } else {
                    // Ensure forward progress after failed parse
                    if self.pos == start {
                        self.advance_char();
                    }
                    children.push(StyleSheetChild::Error(self.span_from(start)));
                }
            } else if let Some(rule) = self.parse_style_rule() {
                children.push(StyleSheetChild::Rule(Rule::Style(Box::new(rule))));
            } else {
                if self.pos == start {
                    self.advance_char();
                }
                children.push(StyleSheetChild::Error(self.span_from(start)));
            }
        }

        children
    }

    fn parse_at_rule(&mut self) -> Option<AtRule> {
        let start = self.pos;
        self.pos += 1; // consume '@'

        let (_, name) = self.parse_ident_with_name().or_else(|| {
            self.skip_to_semicolon_or_block_end();
            None
        })?;

        let prelude = self.read_value();

        let block = if self.matches(b'{') {
            Some(self.parse_block())
        } else if !self.eat(b';') {
            // Missing semicolon — recover
            self.recover(DiagnosticKind::CssSelectorInvalid, self.span_from(start));
            self.skip_to_semicolon_or_block_end();
            None
        } else {
            None
        };

        Some(AtRule {
            span: self.span_from(start),
            name,
            prelude,
            block,
        })
    }

    fn parse_style_rule(&mut self) -> Option<StyleRule> {
        let id = self.alloc_id();
        let start = self.pos;

        let prelude = match self.parse_selector_list(false) {
            Some(sel) => sel,
            None => {
                // Selector failed — skip entire rule
                self.skip_rule();
                return None;
            }
        };

        if !self.matches(b'{') {
            self.recover(DiagnosticKind::CssSelectorInvalid, self.span_from(start));
            self.skip_rule();
            return None;
        }

        let block = self.parse_block();

        Some(StyleRule {
            id,
            span: self.span_from(start),
            prelude,
            block,
        })
    }

    // -- selectors ----------------------------------------------------------

    fn parse_selector_list(&mut self, inside_pseudo: bool) -> Option<SelectorList> {
        let mut children = SmallVec::new();

        self.skip_whitespace_and_comments();
        let start = self.pos;

        loop {
            if self.at_end() {
                self.recover(
                    DiagnosticKind::CssSelectorInvalid,
                    self.span_from(start),
                );
                return None;
            }

            match self.parse_complex_selector(inside_pseudo) {
                Some(sel) => children.push(sel),
                None => return None,
            }
            let end = self.pos;

            self.skip_whitespace_and_comments();

            let terminator = if inside_pseudo { b')' } else { b'{' };
            if self.matches(terminator) {
                return Some(SelectorList {
                    span: Span::new(start as u32, end as u32),
                    children,
                });
            }

            if !self.eat(b',') {
                self.recover(
                    DiagnosticKind::CssSelectorInvalid,
                    self.span_from(start),
                );
                return None;
            }
            self.skip_whitespace_and_comments();
        }
    }

    fn parse_complex_selector(&mut self, inside_pseudo: bool) -> Option<ComplexSelector> {
        let id = self.alloc_id();
        let list_start = self.pos;
        let mut children: RelativeSelectorVec = SmallVec::new();

        let mut rel = self.new_relative_selector(None);

        loop {
            if self.at_end() {
                self.recover(
                    DiagnosticKind::CssSelectorInvalid,
                    self.span_from(list_start),
                );
                return None;
            }

            let start = self.pos;

            if self.eat(b'&') {
                rel.selectors
                    .push(SimpleSelector::Nesting(self.span_from(start)));
            } else if self.eat(b'*') {
                let mut name_end = self.pos;
                if self.eat(b'|') {
                    match self.parse_ident() {
                        Some(ident) => name_end = ident.end as usize,
                        None => return None,
                    }
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
                    if !self.eat(b')') {
                        self.recover(
                            DiagnosticKind::CssSelectorInvalid,
                            self.span_from(start),
                        );
                        return None;
                    }
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
                    if !self.eat(b')') {
                        self.recover(
                            DiagnosticKind::CssSelectorInvalid,
                            self.span_from(start),
                        );
                        return None;
                    }
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
                match self.parse_attribute_selector_inner(start) {
                    Some(attr) => rel.selectors.push(SimpleSelector::Attribute(attr)),
                    None => return None,
                }
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

                return Some(ComplexSelector {
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
                    self.recover(
                        DiagnosticKind::CssSelectorInvalid,
                        self.span_from(list_start),
                    );
                    return None;
                }
            } else if self.pos == start {
                // Nothing matched and pos didn't advance — bail to prevent infinite loop
                self.recover(
                    DiagnosticKind::CssSelectorInvalid,
                    self.span_from(list_start),
                );
                return None;
            }
        }
    }

    /// Parse attribute selector contents after `[` has been consumed.
    fn parse_attribute_selector_inner(&mut self, start: usize) -> Option<AttributeSelector> {
        self.skip_whitespace();
        let (_, attr_name) = self.parse_ident_with_name()?;
        self.skip_whitespace();

        let matcher = self.try_parse_attr_matcher();

        let value = if matcher.is_some() {
            self.skip_whitespace();
            self.read_attribute_value()
        } else {
            None
        };

        self.skip_whitespace();
        let flags = self.try_parse_attr_flags();
        self.skip_whitespace();

        if !self.eat(b']') {
            self.recover(
                DiagnosticKind::CssSelectorInvalid,
                self.span_from(start),
            );
            return None;
        }

        Some(AttributeSelector {
            span: self.span_from(start),
            name: attr_name,
            matcher,
            value,
            flags,
        })
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

    fn parse_block(&mut self) -> Block {
        let start = self.pos;

        if !self.eat(b'{') {
            self.recover(DiagnosticKind::CssSelectorInvalid, self.span_at());
            return Block {
                span: self.span_from(start),
                children: Vec::new(),
            };
        }

        let mut children = Vec::new();

        loop {
            self.skip_whitespace_and_collect_comments(&mut children, BlockChild::Comment);

            if self.eat(b'}') {
                break;
            }
            if self.at_end() {
                self.recover(
                    DiagnosticKind::CssSelectorInvalid,
                    self.span_from(start),
                );
                break;
            }

            self.parse_block_item(&mut children);
        }

        Block {
            span: self.span_from(start),
            children,
        }
    }

    fn parse_block_item(&mut self, children: &mut Vec<BlockChild>) {
        let start = self.pos;

        if self.matches(b'@') {
            if let Some(at) = self.parse_at_rule() {
                children.push(BlockChild::Rule(Rule::AtRule(at)));
            } else {
                if self.pos == start {
                    self.advance_char();
                }
                children.push(BlockChild::Error(self.span_from(start)));
            }
            return;
        }

        if self.block_item_is_rule() {
            if let Some(rule) = self.parse_style_rule() {
                children.push(BlockChild::Rule(Rule::Style(Box::new(rule))));
            } else {
                if self.pos == start {
                    self.advance_char();
                }
                children.push(BlockChild::Error(self.span_from(start)));
            }
        } else {
            match self.parse_declaration() {
                Some(decl) => children.push(BlockChild::Declaration(decl)),
                None => {
                    if self.pos == start {
                        self.advance_char();
                    }
                    children.push(BlockChild::Error(self.span_from(start)));
                }
            }
        }
    }

    fn parse_declaration(&mut self) -> Option<Declaration> {
        let start = self.pos;

        let prop_start = self.pos;
        while self.pos < self.bytes.len() {
            let b = self.bytes[self.pos];
            if is_css_ws(b) || b == b':' || b == b';' || b == b'{' || b == b'}' {
                break;
            }
            self.pos += 1;
        }
        let property = self.span_from(prop_start);

        if property.start == property.end {
            self.recover(DiagnosticKind::CssEmptyDeclaration, self.span_at());
            self.skip_to_semicolon_or_block_end();
            return None;
        }

        self.skip_whitespace();

        if !self.eat(b':') {
            self.recover(DiagnosticKind::CssSelectorInvalid, self.span_from(start));
            self.skip_to_semicolon_or_block_end();
            return None;
        }

        self.skip_whitespace();

        let value = self.read_value();

        if value.start == value.end {
            let prop_text = property.source_text(self.src);
            if !prop_text.starts_with("--") {
                self.recover(DiagnosticKind::CssEmptyDeclaration, self.span_from(start));
                self.skip_to_semicolon_or_block_end();
                return None;
            }
        }

        let end = self.pos;

        // Consume trailing semicolon if not at block end
        if !self.matches(b'}')
            && !self.eat(b';')
        {
            self.recover(DiagnosticKind::CssSelectorInvalid, self.span_from(start));
            self.skip_to_semicolon_or_block_end();
            return None;
        }

        Some(Declaration {
            span: Span::new(start as u32, end as u32),
            property,
            value,
        })
    }
}
