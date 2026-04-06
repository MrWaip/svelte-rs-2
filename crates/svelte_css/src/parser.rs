use compact_str::CompactString;
use smallvec::SmallVec;
use svelte_diagnostics::{Diagnostic, DiagnosticKind};
use svelte_span::Span;

use crate::ast::*;
use crate::scanner::{Scanner, TokenKind};

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
    scanner: Scanner<'src>,
    next_id: u32,
    diagnostics: Vec<Diagnostic>,
}

impl<'src> Parser<'src> {
    fn new(src: &'src str) -> Self {
        Self {
            scanner: Scanner::new(src),
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

    // -- ident helpers ------------------------------------------------------

    /// Expect an `Ident` token, emitting a diagnostic on failure.
    fn parse_ident(&mut self) -> Option<Span> {
        if self.scanner.at(TokenKind::Ident) {
            let tok = self.scanner.bump();
            Some(tok.span)
        } else {
            self.recover(
                DiagnosticKind::CssExpectedIdentifier,
                self.scanner.span_at(),
            );
            None
        }
    }

    /// Parse ident and return (span, name) pair.
    fn parse_ident_with_name(&mut self) -> Option<(Span, CompactString)> {
        let span = self.parse_ident()?;
        let name = CompactString::new(self.scanner.source_text(span));
        Some((span, name))
    }

    // -- whitespace & comments with AST nodes -------------------------------

    fn skip_whitespace_and_collect_comments<T>(
        &mut self,
        out: &mut Vec<T>,
        wrap: fn(Comment) -> T,
    ) {
        loop {
            match self.scanner.peek().kind {
                TokenKind::Whitespace | TokenKind::Cdo | TokenKind::Cdc => {
                    self.scanner.bump();
                }
                TokenKind::Comment => {
                    let tok = self.scanner.bump();
                    out.push(wrap(Comment { span: tok.span }));
                }
                _ => break,
            }
        }
    }

    // -- value reading ------------------------------------------------------

    /// Read a raw CSS value (e.g. declaration value or at-rule prelude).
    /// Consumes tokens until `;`, `{`, `}` at paren-depth 0.
    /// Trailing whitespace is trimmed from the returned span.
    fn read_value(&mut self) -> Span {
        let start = self.scanner.current_start();
        let mut last_non_ws_end = start;
        let mut paren_depth: u32 = 0;

        loop {
            let kind = self.scanner.peek().kind;
            match kind {
                TokenKind::Semicolon | TokenKind::LBrace | TokenKind::RBrace
                    if paren_depth == 0 =>
                {
                    return Span::new(start, last_non_ws_end);
                }
                TokenKind::Eof => return Span::new(start, last_non_ws_end),
                TokenKind::LParen => paren_depth += 1,
                TokenKind::RParen => paren_depth = paren_depth.saturating_sub(1),
                _ => {}
            }
            self.scanner.bump();
            if !matches!(kind, TokenKind::Whitespace | TokenKind::Comment) {
                last_non_ws_end = self.scanner.prev_end;
            }
        }
    }

    /// Read an attribute value (quoted or unquoted).
    fn read_attribute_value(&mut self) -> Option<Span> {
        if self.scanner.at(TokenKind::String) {
            let tok = self.scanner.bump();
            // Return inner span (without quotes).
            return Some(Span::new(tok.span.start + 1, tok.span.end - 1));
        }

        // Unquoted: consume tokens until `]` or whitespace.
        let start = self.scanner.current_start();
        loop {
            match self.scanner.peek().kind {
                TokenKind::RBracket | TokenKind::Whitespace | TokenKind::Eof => break,
                _ => {
                    self.scanner.bump();
                }
            }
        }
        let end = self.scanner.prev_end;

        if start == end {
            self.recover(
                DiagnosticKind::CssExpectedToken { token: "]".into() },
                self.scanner.span_from(start),
            );
            return None;
        }

        Some(Span::new(start, end))
    }

    // -- nth patterns -------------------------------------------------------

    fn try_parse_nth(&mut self) -> Option<Span> {
        let save = self.scanner.save();
        let start = self.scanner.current_start();

        // "even" or "odd"
        if self.scanner.at(TokenKind::Ident) {
            let text = self.scanner.current_raw();
            if text == "even" || text == "odd" {
                self.scanner.bump();
                if self.is_nth_terminator() {
                    return Some(self.scanner.span_from(start));
                }
                self.scanner.restore(save);
                return None;
            }
        }

        let mut has_n = false;

        // Optional leading sign (as a Delim token)
        if self.scanner.at_delim(b'+') || self.scanner.at_delim(b'-') {
            self.scanner.bump();
        }

        if self.scanner.at(TokenKind::Number) {
            self.scanner.bump();
            // Check if `n` follows immediately (the tokenizer may have
            // kept them separate when there is whitespace).
            if self.scanner.at(TokenKind::Ident) {
                let t = self.scanner.current_raw();
                if t == "n" || t == "N" || t == "-n" || t == "-N" {
                    has_n = true;
                    self.scanner.bump();
                }
            }
        } else if self.scanner.at(TokenKind::Dimension) {
            let text = self.scanner.current_raw();
            if text.contains('n') || text.contains('N') {
                has_n = true;
            }
            self.scanner.bump();
        } else if self.scanner.at(TokenKind::Ident) {
            let text = self.scanner.current_raw();
            if text.contains('n') || text.contains('N') {
                has_n = true;
                self.scanner.bump();
            }
        }

        // Nothing consumed
        if self.scanner.current_start() == start {
            return None;
        }

        // If we have 'n', optionally consume + B
        if has_n {
            let before_b = self.scanner.save();
            self.scanner.skip_whitespace();

            // Signed number as single token: "+1", "-1"
            if self.scanner.at(TokenKind::Number) {
                let text = self.scanner.current_raw();
                if text.starts_with('+') || text.starts_with('-') {
                    self.scanner.bump();
                } else {
                    self.scanner.restore(before_b);
                }
            }
            // Or sign + number as separate tokens
            else if self.scanner.at_delim(b'+') || self.scanner.at_delim(b'-') {
                self.scanner.bump();
                self.scanner.skip_whitespace();
                if self.scanner.at(TokenKind::Number) {
                    self.scanner.bump();
                }
            } else {
                self.scanner.restore(before_b);
            }
        }

        // Check for "of" keyword
        let saved = self.scanner.save();
        self.scanner.skip_whitespace();
        if self.scanner.at(TokenKind::Ident) && self.scanner.current_raw() == "of" {
            self.scanner.bump();
            // Consume one whitespace token after "of" so the Nth span
            // covers "2n+1 of " — the selector after it is separate.
            if self.scanner.at(TokenKind::Whitespace) {
                self.scanner.bump();
                return Some(self.scanner.span_from(start));
            }
        }
        self.scanner.restore(saved);

        if self.is_nth_terminator() {
            return Some(self.scanner.span_from(start));
        }

        self.scanner.restore(save);
        None
    }

    #[inline(always)]
    fn is_nth_terminator(&self) -> bool {
        matches!(
            self.scanner.peek().kind,
            TokenKind::RParen | TokenKind::Comma | TokenKind::Whitespace | TokenKind::Eof
        )
    }

    // -- percentage ---------------------------------------------------------

    fn try_parse_percentage(&mut self) -> Option<Span> {
        if self.scanner.at(TokenKind::Percentage) {
            return Some(self.scanner.bump().span);
        }
        None
    }

    // -- combinator ---------------------------------------------------------

    fn try_parse_combinator(&mut self) -> Option<Combinator> {
        let had_ws_start = self.scanner.current_start();
        let had_ws = self.scanner.eat(TokenKind::Whitespace);

        // ||
        if self.scanner.at_delim(b'|') && self.scanner.peek_at(1).kind == TokenKind::Delim(b'|') {
            let comb_start = self.scanner.current_start();
            self.scanner.bump();
            self.scanner.bump();
            return Some(Combinator {
                span: self.scanner.span_from(comb_start),
                kind: CombinatorKind::Column,
            });
        }

        // > + ~
        let kind = match self.scanner.peek().kind {
            TokenKind::Delim(b'>') => Some(CombinatorKind::Child),
            TokenKind::Delim(b'+') => Some(CombinatorKind::NextSibling),
            TokenKind::Delim(b'~') => Some(CombinatorKind::SubsequentSibling),
            _ => None,
        };
        if let Some(kind) = kind {
            let comb_start = self.scanner.current_start();
            self.scanner.bump();
            return Some(Combinator {
                span: self.scanner.span_from(comb_start),
                kind,
            });
        }

        // Descendant (whitespace-only) combinator
        if had_ws {
            return Some(Combinator {
                span: Span::new(had_ws_start, self.scanner.current_start()),
                kind: CombinatorKind::Descendant,
            });
        }

        None
    }

    // -- attribute selector helpers -----------------------------------------

    /// Parse a type selector: `ident` or `ident|ident` (namespace).
    fn parse_type_selector(&mut self, start: u32) -> Option<SimpleSelector> {
        let ident_start = self.scanner.current_start();
        self.parse_ident()?;
        let mut ident_end = self.scanner.prev_end;
        if self.scanner.eat_delim(b'|') {
            self.parse_ident()?;
            ident_end = self.scanner.prev_end;
        }
        Some(SimpleSelector::Type {
            span: self.scanner.span_from(start),
            name: CompactString::new(self.scanner.source_text(Span::new(ident_start, ident_end))),
        })
    }

    fn try_parse_attr_matcher(&mut self) -> Option<Span> {
        let start = self.scanner.current_start();

        match self.scanner.peek().kind {
            TokenKind::Delim(b'~' | b'^' | b'$' | b'*' | b'|') => {
                let save = self.scanner.save();
                self.scanner.bump();
                if self.scanner.eat_delim(b'=') {
                    return Some(self.scanner.span_from(start));
                }
                self.scanner.restore(save);
                None
            }
            TokenKind::Delim(b'=') => {
                self.scanner.bump();
                Some(self.scanner.span_from(start))
            }
            _ => None,
        }
    }

    fn try_parse_attr_flags(&mut self) -> Option<Span> {
        if self.scanner.at(TokenKind::Ident) {
            let tok = self.scanner.bump();
            Some(tok.span)
        } else {
            None
        }
    }

    // =======================================================================
    // Main parsing methods
    // =======================================================================

    fn parse_stylesheet(&mut self) -> StyleSheet {
        let start = self.scanner.current_start();
        let children = self.parse_stylesheet_body();
        StyleSheet {
            span: self.scanner.span_from(start),
            children,
        }
    }

    fn parse_stylesheet_body(&mut self) -> Vec<StyleSheetChild> {
        let mut children = Vec::new();

        loop {
            self.skip_whitespace_and_collect_comments(&mut children, StyleSheetChild::Comment);

            if self.scanner.at_end() {
                break;
            }

            let start = self.scanner.current_start();
            if self.scanner.at(TokenKind::AtKeyword) {
                if let Some(at) = self.parse_at_rule() {
                    children.push(StyleSheetChild::Rule(Rule::AtRule(at)));
                } else {
                    // Ensure forward progress after failed parse
                    if self.scanner.current_start() == start {
                        self.scanner.bump();
                    }
                    children.push(StyleSheetChild::Error(self.scanner.span_from(start)));
                }
            } else if let Some(rule) = self.parse_style_rule() {
                children.push(StyleSheetChild::Rule(Rule::Style(Box::new(rule))));
            } else {
                if self.scanner.current_start() == start {
                    self.scanner.bump();
                }
                children.push(StyleSheetChild::Error(self.scanner.span_from(start)));
            }
        }

        children
    }

    fn parse_at_rule(&mut self) -> Option<AtRule> {
        let start = self.scanner.current_start();

        // Consume the AtKeyword token and extract name (skip leading '@').
        let tok = self.scanner.bump();
        let name = CompactString::new(self.scanner.text_after(tok.span, 1));

        // Skip whitespace before prelude
        self.scanner.skip_whitespace();

        let prelude = self.read_value();

        let block = if self.scanner.at(TokenKind::LBrace) {
            Some(self.parse_block())
        } else if !self.scanner.eat(TokenKind::Semicolon) {
            self.recover(
                DiagnosticKind::CssExpectedToken { token: ";".into() },
                self.scanner.span_from(start),
            );
            self.scanner.skip_to_semicolon_or_block_end();
            None
        } else {
            None
        };

        Some(AtRule {
            span: self.scanner.span_from(start),
            name,
            prelude,
            prelude_override: None,
            block,
        })
    }

    fn parse_style_rule(&mut self) -> Option<StyleRule> {
        let id = self.alloc_id();
        let start = self.scanner.current_start();

        let prelude = match self.parse_selector_list(false) {
            Some(sel) => sel,
            None => {
                self.scanner.skip_rule();
                return None;
            }
        };

        if !self.scanner.at(TokenKind::LBrace) {
            self.recover(
                DiagnosticKind::CssExpectedToken { token: "{".into() },
                self.scanner.span_from(start),
            );
            self.scanner.skip_rule();
            return None;
        }

        let block = self.parse_block();

        Some(StyleRule {
            id,
            span: self.scanner.span_from(start),
            prelude,
            block,
        })
    }

    // -- selectors ----------------------------------------------------------

    fn parse_selector_list(&mut self, inside_pseudo: bool) -> Option<SelectorList> {
        let mut children = SmallVec::new();

        self.scanner.skip_whitespace_and_comments();
        let start = self.scanner.current_start();

        loop {
            if self.scanner.at_end() {
                self.recover(
                    DiagnosticKind::CssSelectorInvalid,
                    self.scanner.span_from(start),
                );
                return None;
            }

            match self.parse_complex_selector(inside_pseudo) {
                Some(sel) => children.push(sel),
                None => return None,
            }
            let end = self.scanner.prev_end;

            self.scanner.skip_whitespace_and_comments();

            let terminator = if inside_pseudo {
                TokenKind::RParen
            } else {
                TokenKind::LBrace
            };
            if self.scanner.at(terminator) {
                return Some(SelectorList {
                    span: Span::new(start, end),
                    children,
                });
            }

            if !self.scanner.eat(TokenKind::Comma) {
                self.recover(
                    DiagnosticKind::CssSelectorInvalid,
                    self.scanner.span_from(start),
                );
                return None;
            }
            self.scanner.skip_whitespace_and_comments();
        }
    }

    fn parse_complex_selector(&mut self, inside_pseudo: bool) -> Option<ComplexSelector> {
        let id = self.alloc_id();
        let list_start = self.scanner.current_start();
        let mut children: RelativeSelectorVec = SmallVec::new();

        let mut rel = self.new_relative_selector(None);

        loop {
            if self.scanner.at_end() {
                self.recover(
                    DiagnosticKind::CssSelectorInvalid,
                    self.scanner.span_from(list_start),
                );
                return None;
            }

            let start = self.scanner.current_start();

            match self.scanner.peek().kind {
                // &
                TokenKind::Delim(b'&') => {
                    let tok = self.scanner.bump();
                    rel.selectors.push(SimpleSelector::Nesting(tok.span));
                }
                // * (universal / namespace)
                TokenKind::Delim(b'*') => {
                    self.scanner.bump();
                    let mut name_end = self.scanner.prev_end;
                    if self.scanner.eat_delim(b'|') {
                        name_end = self.parse_ident()?.end;
                    }
                    let name_span = Span::new(start, name_end);
                    rel.selectors.push(SimpleSelector::Type {
                        span: self.scanner.span_from(start),
                        name: CompactString::new(self.scanner.source_text(name_span)),
                    });
                }
                // # → id selector (via Hash token)
                TokenKind::Hash => {
                    let tok = self.scanner.bump();
                    rel.selectors.push(SimpleSelector::Id {
                        span: tok.span,
                        name: CompactString::new(self.scanner.text_after(tok.span, 1)),
                    });
                }
                // . → class selector
                TokenKind::Delim(b'.') => {
                    self.scanner.bump();
                    let ident = self.parse_ident()?;
                    rel.selectors.push(SimpleSelector::Class {
                        span: self.scanner.span_from(start),
                        name: CompactString::new(self.scanner.source_text(ident)),
                    });
                }
                // : or :: → pseudo-class or pseudo-element
                TokenKind::Colon => {
                    self.scanner.bump();
                    // Check for ::
                    let is_element = self.scanner.eat(TokenKind::Colon);

                    let (_, name) = self.parse_ident_with_name()?;

                    let args = if self.scanner.eat(TokenKind::LParen) {
                        let sel_list = self.parse_selector_list(true)?;
                        if !self.scanner.eat(TokenKind::RParen) {
                            self.recover(
                                DiagnosticKind::CssExpectedToken { token: ")".into() },
                                self.scanner.span_from(start),
                            );
                            return None;
                        }
                        Some(Box::new(sel_list))
                    } else {
                        None
                    };

                    let span = self.scanner.span_from(start);

                    if is_element {
                        rel.selectors
                            .push(SimpleSelector::PseudoElement(PseudoElementSelector {
                                span,
                                name,
                                args,
                            }));
                    } else if name.as_str() == "global" {
                        rel.selectors.push(SimpleSelector::Global { span, args });
                    } else {
                        rel.selectors
                            .push(SimpleSelector::PseudoClass(PseudoClassSelector {
                                span,
                                name,
                                args,
                            }));
                    }
                }
                // [ → attribute selector
                TokenKind::LBracket => {
                    self.scanner.bump();
                    match self.parse_attribute_selector_inner(start) {
                        Some(attr) => rel.selectors.push(SimpleSelector::Attribute(attr)),
                        None => return None,
                    }
                }
                // Other selectors depending on context
                _ => {
                    if inside_pseudo {
                        if let Some(nth) = self.try_parse_nth() {
                            rel.selectors.push(SimpleSelector::Nth(nth));
                        } else if let Some(pct) = self.try_parse_percentage() {
                            rel.selectors.push(SimpleSelector::Percentage(pct));
                        } else if !self.scanner.is_combinator_start() {
                            rel.selectors.push(self.parse_type_selector(start)?);
                        }
                    } else if let Some(pct) = self.try_parse_percentage() {
                        rel.selectors.push(SimpleSelector::Percentage(pct));
                    } else if !self.scanner.is_combinator_start() {
                        rel.selectors.push(self.parse_type_selector(start)?);
                    }
                }
            }

            // Check for selector list terminator
            let index_start = self.scanner.current_start();
            let index_save = self.scanner.save();

            self.scanner.skip_whitespace_and_comments();

            let terminator = if inside_pseudo {
                TokenKind::RParen
            } else {
                TokenKind::LBrace
            };
            if self.scanner.at(TokenKind::Comma) || self.scanner.at(terminator) {
                self.scanner.restore(index_save);
                rel.span.end = index_start;
                children.push(rel);

                return Some(ComplexSelector {
                    id,
                    span: Span::new(list_start, index_start),
                    children,
                });
            }

            // Try combinator
            self.scanner.restore(index_save);
            if let Some(combinator) = self.try_parse_combinator() {
                if !rel.selectors.is_empty() {
                    rel.span.end = index_start;
                    children.push(rel);
                }

                rel = self.new_relative_selector(Some(combinator));

                self.scanner.skip_whitespace();

                if self.scanner.at(TokenKind::Comma) || self.scanner.at(terminator) {
                    self.recover(
                        DiagnosticKind::CssSelectorInvalid,
                        self.scanner.span_from(list_start),
                    );
                    return None;
                }
            } else if self.scanner.current_start() == start {
                // Nothing matched and pos didn't advance
                self.recover(
                    DiagnosticKind::CssSelectorInvalid,
                    self.scanner.span_from(list_start),
                );
                return None;
            }
        }
    }

    fn parse_attribute_selector_inner(&mut self, start: u32) -> Option<AttributeSelector> {
        self.scanner.skip_whitespace();
        let (_, attr_name) = self.parse_ident_with_name()?;
        self.scanner.skip_whitespace();

        let matcher = self.try_parse_attr_matcher();

        let value = if matcher.is_some() {
            self.scanner.skip_whitespace();
            self.read_attribute_value()
        } else {
            None
        };

        self.scanner.skip_whitespace();
        let flags = self.try_parse_attr_flags();
        self.scanner.skip_whitespace();

        if !self.scanner.eat(TokenKind::RBracket) {
            self.recover(
                DiagnosticKind::CssExpectedToken { token: "]".into() },
                self.scanner.span_from(start),
            );
            return None;
        }

        Some(AttributeSelector {
            span: self.scanner.span_from(start),
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
            .map_or(self.scanner.current_start(), |c| c.span.start);
        RelativeSelector {
            id,
            span: Span::new(start, 0),
            combinator,
            selectors: SmallVec::new(),
        }
    }

    // -- blocks & declarations ----------------------------------------------

    fn parse_block(&mut self) -> Block {
        let start = self.scanner.current_start();

        if !self.scanner.eat(TokenKind::LBrace) {
            self.recover(
                DiagnosticKind::CssExpectedToken { token: "{".into() },
                self.scanner.span_at(),
            );
            return Block {
                span: self.scanner.span_from(start),
                children: Vec::new(),
            };
        }

        let mut children = Vec::new();

        loop {
            self.skip_whitespace_and_collect_comments(&mut children, BlockChild::Comment);

            if self.scanner.eat(TokenKind::RBrace) {
                break;
            }
            if self.scanner.at_end() {
                self.recover(
                    DiagnosticKind::CssUnclosedBlock,
                    self.scanner.span_from(start),
                );
                break;
            }

            self.parse_block_item(&mut children);
        }

        Block {
            span: self.scanner.span_from(start),
            children,
        }
    }

    fn parse_block_item(&mut self, children: &mut Vec<BlockChild>) {
        let start = self.scanner.current_start();

        if self.scanner.at(TokenKind::AtKeyword) {
            if let Some(at) = self.parse_at_rule() {
                children.push(BlockChild::Rule(Rule::AtRule(at)));
            } else {
                if self.scanner.current_start() == start {
                    self.scanner.bump();
                }
                children.push(BlockChild::Error(self.scanner.span_from(start)));
            }
            return;
        }

        if self.scanner.block_item_is_rule() {
            if let Some(rule) = self.parse_style_rule() {
                children.push(BlockChild::Rule(Rule::Style(Box::new(rule))));
            } else {
                if self.scanner.current_start() == start {
                    self.scanner.bump();
                }
                children.push(BlockChild::Error(self.scanner.span_from(start)));
            }
        } else {
            match self.parse_declaration() {
                Some(decl) => children.push(BlockChild::Declaration(decl)),
                None => {
                    if self.scanner.current_start() == start {
                        self.scanner.bump();
                    }
                    children.push(BlockChild::Error(self.scanner.span_from(start)));
                }
            }
        }
    }

    fn parse_declaration(&mut self) -> Option<Declaration> {
        let start = self.scanner.current_start();

        // Property name: scan tokens until Whitespace, Colon, Semicolon, or braces.
        // Typically an Ident, but we accept any tokens for robustness.
        let prop_start = self.scanner.current_start();
        loop {
            match self.scanner.peek().kind {
                TokenKind::Whitespace
                | TokenKind::Colon
                | TokenKind::Semicolon
                | TokenKind::LBrace
                | TokenKind::RBrace
                | TokenKind::Eof => break,
                _ => {
                    self.scanner.bump();
                }
            }
        }
        let property = Span::new(prop_start, self.scanner.prev_end);

        if property.start == property.end {
            self.recover(DiagnosticKind::CssEmptyDeclaration, self.scanner.span_at());
            self.scanner.skip_to_semicolon_or_block_end();
            return None;
        }

        self.scanner.skip_whitespace();

        if !self.scanner.eat(TokenKind::Colon) {
            self.recover(
                DiagnosticKind::CssExpectedToken { token: ":".into() },
                self.scanner.span_from(start),
            );
            self.scanner.skip_to_semicolon_or_block_end();
            return None;
        }

        self.scanner.skip_whitespace();

        let value = self.read_value();

        if value.start == value.end {
            let prop_text = self.scanner.source_text(property);
            if !prop_text.starts_with("--") {
                self.recover(
                    DiagnosticKind::CssEmptyDeclaration,
                    self.scanner.span_from(start),
                );
                self.scanner.skip_to_semicolon_or_block_end();
                return None;
            }
        }

        let end = self.scanner.current_start();

        // Consume trailing semicolon if not at block end
        if !self.scanner.at(TokenKind::RBrace) && !self.scanner.eat(TokenKind::Semicolon) {
            self.recover(
                DiagnosticKind::CssExpectedToken { token: ";".into() },
                self.scanner.span_from(start),
            );
            self.scanner.skip_to_semicolon_or_block_end();
            return None;
        }

        Some(Declaration {
            span: Span::new(start, end),
            property,
            value,
            value_override: None,
        })
    }
}
