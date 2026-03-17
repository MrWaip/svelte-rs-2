pub mod token;

use std::{iter::Peekable, str::Chars, vec};

pub use svelte_ast::is_void;

use token::{
    AnimateDirective, AttachTagToken, Attribute, AttributeIdentifierType, AttributeValue,
    BindDirective, ClassDirective, Concatenation, ConcatenationPart, ExpressionTag, HTMLAttribute,
    OnDirectiveLegacy, ScriptTag, StartEachTag, StartIfTag, StartKeyTag, StartTag,
    StyleDirective, Token, TokenType, TransitionDirective, UseDirective,
};

use svelte_diagnostics::Diagnostic;
use svelte_span::{Span, SPAN};

pub struct Scanner<'a> {
    source: &'a str,
    chars: Peekable<Chars<'a>>,
    tokens: Vec<Token>,
    diagnostics: Vec<Diagnostic>,
    start: usize,
    prev: usize,
    current: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Scanner<'a> {
        Scanner {
            source,
            tokens: vec![],
            diagnostics: vec![],
            chars: source.chars().peekable(),
            prev: 0,
            current: 0,
            start: 0,
        }
    }

    pub fn scan_tokens(&mut self) -> (Vec<Token>, Vec<Diagnostic>) {
        while !self.is_at_end() {
            self.start = self.current;
            if let Err(diagnostic) = self.scan_token() {
                self.diagnostics.push(diagnostic);
                self.sync_to_next_token();
            }
        }

        self.tokens.push(Token {
            token_type: TokenType::EOF,
            span: Span::new(self.start as u32, self.current as u32),
        });

        let tokens = std::mem::take(&mut self.tokens);
        let diagnostics = std::mem::take(&mut self.diagnostics);
        (tokens, diagnostics)
    }

    fn recover(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }

    /// Skip forward to the next synchronization point after a scan error.
    fn sync_to_next_token(&mut self) {
        while !self.is_at_end() {
            match self.peek() {
                Some('<') | Some('{') => break,
                _ => { self.advance(); }
            }
        }
    }

    fn scan_token(&mut self) -> Result<(), Diagnostic> {
        let char = self.advance();

        if char == '<' {
            let peeked = self.peek();

            match peeked {
                Some('/') => return self.end_tag(),
                Some('!') => return self.comment(),
                _ => return self.start_tag(),
            }
        }

        if char == '{' {
            return match self.peek() {
                Some('#') => self.start_template(),
                Some(':') => self.middle_template(),
                Some('/') => self.end_template(),
                Some('@') => self.at_template(),
                _ => self.interpolation(),
            };
        }

        self.text();

        Ok(())
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.tokens.push(Token {
            token_type,
            span: Span::new(self.start as u32, self.current as u32),
        });
    }

    fn advance(&mut self) -> char {
        let char = self.chars.next().unwrap();

        self.prev = self.current;
        self.current += char.len_utf8();

        char
    }

    fn is_at_end(&mut self) -> bool {
        self.chars.peek().is_none()
    }

    fn identifier(&mut self) -> &'a str {
        let start = self.current;

        while let Some(ch) = self.peek() {
            if ch.is_alphanumeric() || ch == '-' {
                self.advance();
            } else {
                break;
            }
        }

        self.slice_source(start, self.current)
    }

    fn slice_source(&self, start: usize, end: usize) -> &'a str {
        &self.source[start..end]
    }

    fn span(&self, start: usize, end: usize) -> Span {
        Span::new(start as u32, end as u32)
    }

    fn attribute_identifier(&mut self) -> Result<AttributeIdentifierType<'a>, Diagnostic> {
        let start = self.current;

        let mut is_directive = false;
        let mut colon_pos: usize = 0;

        while let Some(ch) = self.peek() {
            if ch == ':' {
                is_directive = true;
                colon_pos = self.current;
            }

            if ch.is_alphanumeric() || ch == '-' || ch == ':' {
                self.advance();
            } else {
                break;
            }
        }

        if is_directive {
            let name = self.slice_source(start, colon_pos);
            let value = self.slice_source(colon_pos + 1, self.current);
            let value_span = self.span(colon_pos + 1, self.current);

            if AttributeIdentifierType::is_class_directive(name) {
                AttributeIdentifierType::ClassDirective(value_span, value).as_ok()
            } else if AttributeIdentifierType::is_style_directive(name) {
                AttributeIdentifierType::StyleDirective(value_span, value).as_ok()
            } else if AttributeIdentifierType::is_bind_directive(name) {
                AttributeIdentifierType::BindDirective(value_span, value).as_ok()
            } else if AttributeIdentifierType::is_use_directive(name) {
                AttributeIdentifierType::UseDirective(value_span, value).as_ok()
            // LEGACY(svelte4): on:directive
            } else if AttributeIdentifierType::is_on_directive(name) {
                AttributeIdentifierType::OnDirectiveLegacy(value_span, value).as_ok()
            } else if AttributeIdentifierType::is_transition_directive(name) {
                AttributeIdentifierType::TransitionDirective(value_span, name).as_ok()
            } else if AttributeIdentifierType::is_animate_directive(name) {
                AttributeIdentifierType::AnimateDirective(value_span, value).as_ok()
            } else {
                Diagnostic::unknown_directive(Span::new(colon_pos as u32, self.current as u32)).as_err()
            }
        } else if start == self.current {
            AttributeIdentifierType::None.as_ok()
        } else {
            let full_span = self.span(start, self.current);
            AttributeIdentifierType::HTMLAttribute(full_span, self.slice_source(start, self.current)).as_ok()
        }
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.peek().is_some_and(|c| c != expected) {
            return false;
        }

        self.advance();

        true
    }

    fn peek(&mut self) -> Option<char> {
        if self.is_at_end() {
            return None;
        }

        self.chars.peek().copied()
    }

    fn collect_until_span<F>(&mut self, condition: F) -> Result<Span, Diagnostic>
    where
        F: Fn(char) -> bool,
    {
        let start = self.current;

        while !self.is_at_end() {
            if self.peek().is_some_and(&condition) {
                break;
            }

            self.advance();
        }

        if self.is_at_end() {
            self.recover(Diagnostic::unexpected_end_of_file(Span::new(
                start as u32,
                self.current as u32,
            )));
        }

        Ok(self.span(start, self.current))
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    // Tokens:

    fn start_tag(&mut self) -> Result<(), Diagnostic> {
        let name_start = self.current;
        let name = self.identifier();

        if name.is_empty() {
            return Err(Diagnostic::invalid_tag_name(self.span(name_start, self.current)));
        }

        // Handle `svelte:*` special element names (e.g., svelte:options, svelte:head)
        if name == "svelte" && self.peek() == Some(':') {
            self.advance(); // consume ':'
            self.identifier(); // consume the rest (options, head, window, etc.)
        }

        let name_span = self.span(name_start, self.current);

        let attributes = self.attributes()?;
        let self_closing = self.match_char('/') || is_void(name);

        if !self.match_char('>') {
            // Emit partial StartTag with recovery — parser-level will handle auto-close
            self.recover(Diagnostic::unterminated_start_tag(self.span(
                name_start,
                self.current,
            )));

            self.add_token(TokenType::StartTag(StartTag {
                attributes,
                name_span,
                self_closing,
            }));

            return Ok(());
        }

        if name == "script" {
            return self.script_tag(&attributes, name_span);
        }

        if name == "style" {
            return self.style_tag(name_span);
        }

        self.add_token(TokenType::StartTag(StartTag {
            attributes,
            name_span,
            self_closing,
        }));

        Ok(())
    }

    fn attributes(&mut self) -> Result<Vec<Attribute>, Diagnostic> {
        let mut attributes: Vec<Attribute> = vec![];

        loop {
            let peeked = self.peek();

            if peeked.is_none() {
                break;
            }

            let ch = peeked.unwrap();

            if ch == '/' || ch == '>' {
                break;
            }

            self.skip_whitespace();

            let peeked = self.peek();

            let attr_result = if peeked == Some('{') {
                if self.source[self.current..].starts_with("{@attach") {
                    self.attach_tag_attribute().map(Attribute::AttachTag)
                } else {
                    self.expression_tag().map(Attribute::ExpressionTag)
                }
            } else {
                let name = match self.attribute_identifier() {
                    Ok(name) => name,
                    Err(d) => {
                        self.recover(d);
                        break;
                    }
                };

                match name {
                    AttributeIdentifierType::HTMLAttribute(span, _) => self.html_attribute(span),
                    AttributeIdentifierType::ClassDirective(span, name) => {
                        self.class_directive(span, name)
                    }
                    AttributeIdentifierType::StyleDirective(span, _) => {
                        self.style_directive(span)
                    }
                    AttributeIdentifierType::BindDirective(span, name) => self.bind_directive(span, name),
                    AttributeIdentifierType::UseDirective(span, _) => self.use_directive(span),
                    // LEGACY(svelte4): on:directive
                    AttributeIdentifierType::OnDirectiveLegacy(span, _) => self.on_directive_legacy(span),
                    AttributeIdentifierType::TransitionDirective(span, prefix) => {
                        self.transition_directive(span, prefix)
                    }
                    AttributeIdentifierType::AnimateDirective(span, _) => {
                        self.animate_directive(span)
                    }
                    AttributeIdentifierType::None => break,
                }
            };

            match attr_result {
                Ok(attr) => attributes.push(attr),
                Err(d) => {
                    self.recover(d);
                    break;
                }
            }

            self.skip_whitespace();
        }

        Ok(attributes)
    }

    fn html_attribute(&mut self, name_span: Span) -> Result<Attribute, Diagnostic> {
        let mut value: AttributeValue = AttributeValue::Empty;

        if self.match_char('=') {
            value = self.attribute_value()?;
        }

        Ok(Attribute::HTMLAttribute(HTMLAttribute { name_span, value }))
    }

    fn class_directive(&mut self, name_span: Span, _name: &str) -> Result<Attribute, Diagnostic> {
        if self.match_char('=') {
            let res = self.expression_tag()?;

            return Ok(Attribute::ClassDirective(ClassDirective {
                expression_span: res.expression_span,
                name_span,
                shorthand: false,
            }));
        }

        Ok(Attribute::ClassDirective(ClassDirective {
            name_span,
            expression_span: name_span,
            shorthand: true,
        }))
    }

    fn style_directive(&mut self, name_span: Span) -> Result<Attribute, Diagnostic> {
        // Check for |important modifier
        let important = if self.match_char('|') {
            let start = self.current;
            while self.peek().is_some_and(|c| c.is_alphabetic()) {
                self.advance();
            }
            let modifier = self.slice_source(start, self.current);
            if modifier != "important" {
                self.recover(Diagnostic::unknown_directive(Span::new(
                    start as u32 - 1, // include the '|'
                    self.current as u32,
                )));
                false
            } else {
                true
            }
        } else {
            false
        };

        if self.match_char('=') {
            let value = self.attribute_value()?;

            return Ok(Attribute::StyleDirective(StyleDirective {
                value,
                name_span,
                shorthand: false,
                important,
            }));
        }

        Ok(Attribute::StyleDirective(StyleDirective {
            name_span,
            value: AttributeValue::Empty,
            shorthand: true,
            important,
        }))
    }

    fn bind_directive(&mut self, name_span: Span, _name: &str) -> Result<Attribute, Diagnostic> {
        if self.match_char('=') {
            let res = self.expression_tag()?;

            return Ok(Attribute::BindDirective(BindDirective {
                expression_span: res.expression_span,
                name_span,
                shorthand: false,
            }));
        }

        Ok(Attribute::BindDirective(BindDirective {
            name_span,
            expression_span: name_span,
            shorthand: true,
        }))
    }

    fn use_directive(&mut self, mut name_span: Span) -> Result<Attribute, Diagnostic> {
        // Consume dotted name segments: use:a.b.c
        while self.peek() == Some('.') {
            self.advance(); // consume '.'
            while self.peek().is_some_and(|c| c.is_alphanumeric() || c == '_') {
                self.advance();
            }
            name_span = Span::new(name_span.start, self.current as u32);
        }

        if self.match_char('=') {
            let res = self.expression_tag()?;

            return Ok(Attribute::UseDirective(UseDirective {
                expression_span: res.expression_span,
                name_span,
                shorthand: false,
            }));
        }

        Ok(Attribute::UseDirective(UseDirective {
            name_span,
            expression_span: name_span,
            shorthand: true,
        }))
    }

    /// LEGACY(svelte4): Parse `on:event|modifier1|modifier2={handler}` or `on:event` (bubble).
    fn on_directive_legacy(&mut self, name_span: Span) -> Result<Attribute, Diagnostic> {
        let mut modifiers = Vec::new();
        while self.match_char('|') {
            let start = self.current;
            while self.peek().is_some_and(|c| c.is_alphabetic() || c == '_') {
                self.advance();
            }
            modifiers.push(self.span(start, self.current));
        }

        if self.match_char('=') {
            let res = self.expression_tag()?;
            return Ok(Attribute::OnDirectiveLegacy(OnDirectiveLegacy {
                name_span,
                expression_span: res.expression_span,
                modifiers,
                has_expression: true,
            }));
        }

        // No expression — bubble event
        Ok(Attribute::OnDirectiveLegacy(OnDirectiveLegacy {
            name_span,
            expression_span: SPAN,
            modifiers,
            has_expression: false,
        }))
    }

    /// Parse `transition:name|modifier={expr}`, `in:name`, or `out:name`.
    fn transition_directive(&mut self, mut name_span: Span, prefix: &str) -> Result<Attribute, Diagnostic> {
        // Consume dotted name segments: transition:a.b.c
        while self.peek() == Some('.') {
            self.advance(); // consume '.'
            while self.peek().is_some_and(|c| c.is_alphanumeric() || c == '_') {
                self.advance();
            }
            name_span = Span::new(name_span.start, self.current as u32);
        }

        // Parse |modifier segments
        let mut modifiers = Vec::new();
        while self.match_char('|') {
            let start = self.current;
            while self.peek().is_some_and(|c| c.is_alphabetic() || c == '_') {
                self.advance();
            }
            modifiers.push(self.span(start, self.current));
        }

        if self.match_char('=') {
            let res = self.expression_tag()?;
            return Ok(Attribute::TransitionDirective(TransitionDirective {
                name_span,
                expression_span: res.expression_span,
                modifiers,
                has_expression: true,
                direction_prefix: prefix.to_string(),
            }));
        }

        Ok(Attribute::TransitionDirective(TransitionDirective {
            name_span,
            expression_span: SPAN,
            modifiers,
            has_expression: false,
            direction_prefix: prefix.to_string(),
        }))
    }

    /// Parse `animate:name={expr}` or `animate:name`.
    fn animate_directive(&mut self, mut name_span: Span) -> Result<Attribute, Diagnostic> {
        // Consume dotted name segments: animate:a.b.c
        while self.peek() == Some('.') {
            self.advance(); // consume '.'
            while self.peek().is_some_and(|c| c.is_alphanumeric() || c == '_') {
                self.advance();
            }
            name_span = Span::new(name_span.start, self.current as u32);
        }

        if self.match_char('=') {
            let res = self.expression_tag()?;
            return Ok(Attribute::AnimateDirective(AnimateDirective {
                name_span,
                expression_span: res.expression_span,
                has_expression: true,
            }));
        }

        Ok(Attribute::AnimateDirective(AnimateDirective {
            name_span,
            expression_span: SPAN,
            has_expression: false,
        }))
    }

    /// Parse `{@attach expr}` in the attribute position.
    fn attach_tag_attribute(&mut self) -> Result<AttachTagToken, Diagnostic> {
        debug_assert!(self.source[self.current..].starts_with("{@attach"));

        // Consume `{@attach`
        for _ in 0.."{@attach".len() {
            self.advance();
        }

        self.skip_whitespace();
        let expression_span = self.collect_js_expression()?;

        Ok(AttachTagToken { expression_span })
    }

    fn attribute_value(&mut self) -> Result<AttributeValue, Diagnostic> {
        let peeked = self.peek();

        if self.peek() == Some('{') {
            return self.expression_tag().map(AttributeValue::ExpressionTag);
        }

        if let Some(quote) = peeked.filter(|c| *c == '"' || *c == '\'') {
            return self.attribute_concatenation_or_string(quote);
        }

        let span = self.collect_until_span(|char| match char {
            '"' | '\'' | '>' | '<' | '`' => true,
            char => char.is_whitespace(),
        })?;

        Ok(AttributeValue::String(span))
    }

    fn expression_tag(&mut self) -> Result<ExpressionTag, Diagnostic> {
        debug_assert_eq!(self.peek(), Some('{'));

        let start = self.start;
        self.advance();

        let expression_span = self.collect_js_expression()?;

        Ok(ExpressionTag {
            expression_span,
            span: Span::new(start as u32, self.current as u32),
        })
    }

    fn attribute_concatenation_or_string(
        &mut self,
        quote: char,
    ) -> Result<AttributeValue, Diagnostic> {
        debug_assert_eq!(self.peek(), Some(quote));

        let mut has_expression = false;
        let start = self.current;
        let mut parts: Vec<ConcatenationPart> = vec![];

        // consume first quote
        self.advance();
        let mut current_pos: usize = self.current;

        while let Some(char) = self.peek() {
            if char == quote {
                break;
            }

            if char == '{' {
                has_expression = true;

                if current_pos < self.current {
                    parts.push(ConcatenationPart::String(self.span(current_pos, self.current)));
                }

                let expression_tag = self.expression_tag()?;

                parts.push(ConcatenationPart::Expression(expression_tag));
                current_pos = self.current;

                continue;
            }

            self.advance();
        }

        let last_span = self.span(current_pos, self.current);

        // consume last quote (or recover at EOF)
        if !self.is_at_end() {
            self.advance();
        } else {
            self.recover(Diagnostic::unexpected_end_of_file(Span::new(
                start as u32,
                self.current as u32,
            )));
        }

        if has_expression && current_pos < self.current - 1 {
            // There's trailing text after last expression (before closing quote)
            if last_span.start != last_span.end {
                parts.push(ConcatenationPart::String(last_span));
            }
        }

        if !has_expression && parts.is_empty() {
            return Ok(AttributeValue::String(last_span));
        }

        Ok(AttributeValue::Concatenation(Concatenation {
            span: self.span(start, self.current),
            parts,
        }))
    }

    fn end_tag(&mut self) -> Result<(), Diagnostic> {
        self.advance();

        let name_start = self.current;
        let name = self.identifier();

        if name.is_empty() {
            return Err(Diagnostic::invalid_tag_name(self.span(name_start, self.current)));
        }

        // Handle `svelte:*` special element names
        if name == "svelte" && self.peek() == Some(':') {
            self.advance(); // consume ':'
            self.identifier(); // consume the rest
        }

        let name_span = self.span(name_start, self.current);

        self.skip_whitespace();

        if !self.match_char('>') {
            self.recover(Diagnostic::unexpected_token(self.span(name_start, self.current)));
        }

        self.add_token(TokenType::EndTag(token::EndTag { name_span }));

        Ok(())
    }

    fn text(&mut self) {
        while self.peek() != Some('<') && self.peek() != Some('{') && !self.is_at_end() {
            self.advance();
        }

        self.add_token(TokenType::Text);
    }

    fn interpolation(&mut self) -> Result<(), Diagnostic> {
        let expression_span = self.collect_js_expression()?;

        self.add_token(TokenType::Interpolation(ExpressionTag {
            expression_span,
            span: Span::new(self.start as u32, self.current as u32),
        }));

        Ok(())
    }

    fn collect_js_expression(&mut self) -> Result<Span, Diagnostic> {
        let mut stack: Vec<bool> = vec![];
        let start = self.current;

        while !self.is_at_end() {
            let char = self.advance();

            if char == '\n' {
                continue;
            }

            if char == '\'' || char == '"' || char == '`' {
                self.skip_js_string(char)?;
                continue;
            }

            if char == '{' {
                stack.push(true);
                continue;
            }

            if char == '}' && stack.pop().is_none() {
                let raw = if self.current - start > 2 {
                    self.slice_source(start, self.prev)
                } else {
                    ""
                };

                let trim_start = raw.len() - raw.trim_start().len();
                let trim_end = raw.len() - raw.trim_end().len();
                let span_start = start + trim_start;
                let span_end = self.prev - trim_end;

                return Ok(Span::new(span_start as u32, span_end as u32));
            }
        }

        // EOF — return partial expression with recovery
        self.recover(Diagnostic::unexpected_end_of_file(Span::new(
            start as u32,
            self.current as u32,
        )));

        let raw = self.slice_source(start, self.current);
        let trim_start = raw.len() - raw.trim_start().len();
        let trim_end = raw.len() - raw.trim_end().len();
        let span_start = start + trim_start;
        let span_end = if trim_end <= self.current - start {
            self.current - trim_end
        } else {
            start
        };

        Ok(Span::new(span_start as u32, span_end as u32))
    }

    fn skip_js_string(&mut self, quote: char) -> Result<(), Diagnostic> {
        let start = self.current;
        while self.peek() != Some(quote) && !self.is_at_end() {
            if self.peek() == Some('\\') {
                self.advance(); // skip backslash
                if !self.is_at_end() {
                    self.advance(); // skip escaped char
                }
                continue;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.recover(Diagnostic::unexpected_end_of_file(Span::new(
                start as u32,
                self.current as u32,
            )));
            return Ok(());
        }

        self.advance();

        Ok(())
    }

    fn start_template(&mut self) -> Result<(), Diagnostic> {
        debug_assert_eq!(self.peek(), Some('#'));

        self.advance();

        let start = self.current;
        let keyword = self.identifier();

        if keyword.is_empty() {
            return Err(Diagnostic::unexpected_keyword(Span::new(
                self.start as u32,
                self.current as u32,
            )));
        }

        match keyword {
            "if" => {
                let expression_span = self.collect_js_expression()?;

                self.add_token(TokenType::StartIfTag(StartIfTag { expression_span }));

                Ok(())
            }
            "each" => self.start_each_tag(),
            "snippet" => self.start_snippet_tag(),
            "key" => {
                let expression_span = self.collect_js_expression()?;
                self.add_token(TokenType::StartKeyTag(StartKeyTag { expression_span }));
                Ok(())
            }
            _ => Err(Diagnostic::unexpected_keyword(Span::new(
                start as u32,
                self.current as u32,
            ))),
        }
    }

    fn end_template(&mut self) -> Result<(), Diagnostic> {
        debug_assert_eq!(self.peek(), Some('/'));

        self.advance();

        let start = self.current;
        let keyword = self.identifier();

        if keyword.is_empty() {
            return Err(Diagnostic::unexpected_keyword(Span::new(
                self.start as u32,
                self.current as u32,
            )));
        }

        match keyword {
            "if" => {
                self.skip_whitespace();

                if !self.match_char('}') {
                    self.recover(Diagnostic::unexpected_token(Span::new(start as u32, self.current as u32)));
                }

                self.add_token(TokenType::EndIfTag);

                Ok(())
            }
            "each" => {
                self.skip_whitespace();

                if !self.match_char('}') {
                    self.recover(Diagnostic::unexpected_token(Span::new(start as u32, self.current as u32)));
                }

                self.add_token(TokenType::EndEachTag);

                Ok(())
            }
            "snippet" => {
                self.skip_whitespace();

                if !self.match_char('}') {
                    self.recover(Diagnostic::unexpected_token(Span::new(start as u32, self.current as u32)));
                }

                self.add_token(TokenType::EndSnippetTag);

                Ok(())
            }
            "key" => {
                self.skip_whitespace();

                if !self.match_char('}') {
                    self.recover(Diagnostic::unexpected_token(Span::new(start as u32, self.current as u32)));
                }

                self.add_token(TokenType::EndKeyTag);

                Ok(())
            }
            _ => Err(Diagnostic::unexpected_keyword(Span::new(
                start as u32,
                self.current as u32,
            ))),
        }
    }

    fn middle_template(&mut self) -> Result<(), Diagnostic> {
        debug_assert_eq!(self.peek(), Some(':'));

        self.advance();

        let start = self.current;
        let keyword = self.identifier();

        if keyword.is_empty() {
            return Err(Diagnostic::unexpected_keyword(Span::new(
                start as u32,
                self.current as u32,
            )));
        }

        match keyword {
            "else" => {
                self.skip_whitespace();

                let start = self.current;
                let elseif = self.identifier();

                if !elseif.is_empty() {
                    if elseif != "if" {
                        return Err(Diagnostic::unexpected_keyword(Span::new(
                            start as u32,
                            self.current as u32,
                        )));
                    }

                    let expression_span = self.collect_js_expression()?;

                    self.add_token(TokenType::ElseTag(token::ElseTag {
                        elseif: true,
                        expression_span: Some(expression_span),
                    }));
                } else {
                    if !self.match_char('}') {
                        self.recover(Diagnostic::unexpected_token(Span::new(start as u32, self.current as u32)));
                    }

                    self.add_token(TokenType::ElseTag(token::ElseTag {
                        elseif: false,
                        expression_span: None,
                    }));
                }

                Ok(())
            }
            _ => Err(Diagnostic::unexpected_keyword(Span::new(
                start as u32,
                self.current as u32,
            ))),
        }
    }

    fn script_tag(&mut self, attributes: &[Attribute], _name_span: Span) -> Result<(), Diagnostic> {
        let start = self.current;
        let mut end = start;

        while !self.is_at_end() {
            let char = self.advance();

            if char != '<' {
                continue;
            }

            end = self.prev;

            if !self.match_char('/') {
                continue;
            }

            let identifier = self.identifier();

            if identifier == "script" {
                break;
            }
        }

        let is_typescript = attributes.iter().any(|item| match item {
            Attribute::HTMLAttribute(attr) => {
                let name = attr.name_span.source_text(self.source);
                name == "lang"
                    && matches!(attr.value, AttributeValue::String(span) if span.source_text(self.source) == "ts")
            }
            _ => false,
        });

        let is_module = attributes.iter().any(|item| match item {
            Attribute::HTMLAttribute(attr) => {
                let name = attr.name_span.source_text(self.source);
                (name == "module" && attr.value == AttributeValue::Empty)
                    || (name == "context"
                        && matches!(attr.value, AttributeValue::String(span) if span.source_text(self.source) == "module"))
            }
            _ => false,
        });

        if self.is_at_end() {
            self.recover(Diagnostic::unexpected_end_of_file(Span::new(
                start as u32,
                self.current as u32,
            )));

            self.add_token(TokenType::ScriptTag(ScriptTag {
                content_span: self.span(start, self.current),
                is_typescript,
                is_module,
            }));

            return Ok(());
        }

        self.skip_whitespace();

        if !self.match_char('>') {
            return Err(Diagnostic::unexpected_token(Span::new(start as u32, self.current as u32)));
        }

        self.add_token(TokenType::ScriptTag(ScriptTag {
            content_span: self.span(start, end),
            is_typescript,
            is_module,
        }));

        Ok(())
    }

    fn style_tag(&mut self, _name_span: Span) -> Result<(), Diagnostic> {
        let start = self.current;
        let mut end = start;

        while !self.is_at_end() {
            let char = self.advance();

            if char != '<' {
                continue;
            }

            end = self.prev;

            if !self.match_char('/') {
                continue;
            }

            let identifier = self.identifier();

            if identifier == "style" {
                break;
            }
        }

        if self.is_at_end() {
            self.recover(Diagnostic::unexpected_end_of_file(Span::new(
                start as u32,
                self.current as u32,
            )));

            self.add_token(TokenType::StyleTag(token::StyleTag {
                content_span: self.span(start, self.current),
            }));

            return Ok(());
        }

        self.skip_whitespace();

        if !self.match_char('>') {
            return Err(Diagnostic::unexpected_token(Span::new(start as u32, self.current as u32)));
        }

        self.add_token(TokenType::StyleTag(token::StyleTag {
            content_span: self.span(start, end),
        }));

        Ok(())
    }

    fn comment(&mut self) -> Result<(), Diagnostic> {
        let start = self.current;
        self.advance();

        if !self.match_char('-') {
            return Err(Diagnostic::unexpected_token(Span::new(start as u32, self.current as u32)));
        }

        if !self.match_char('-') {
            return Err(Diagnostic::unexpected_token(Span::new(start as u32, self.current as u32)));
        }

        while !self.is_at_end() {
            if self.match_char('-') && self.match_char('-') && self.peek() == Some('>') {
                break;
            }

            self.advance();
        }

        if self.is_at_end() {
            self.recover(Diagnostic::unexpected_end_of_file(Span::new(
                start as u32,
                self.current as u32,
            )));

            self.add_token(TokenType::Comment);
            return Ok(());
        }

        self.advance();

        self.add_token(TokenType::Comment);

        Ok(())
    }

    fn at_template(&mut self) -> Result<(), Diagnostic> {
        debug_assert_eq!(self.peek(), Some('@'));

        self.advance();

        let start = self.current;
        let keyword = self.identifier();

        if keyword.is_empty() {
            return Err(Diagnostic::unexpected_keyword(Span::new(
                self.start as u32,
                self.current as u32,
            )));
        }

        match keyword {
            "render" => {
                self.skip_whitespace();
                let expression_span = self.collect_js_expression()?;

                self.add_token(TokenType::RenderTag(token::RenderTagToken {
                    expression_span,
                }));

                Ok(())
            }
            "html" => {
                self.skip_whitespace();
                let expression_span = self.collect_js_expression()?;

                self.add_token(TokenType::HtmlTag(token::HtmlTagToken {
                    expression_span,
                }));

                Ok(())
            }
            "const" => {
                self.skip_whitespace();
                let declaration_span = self.collect_js_expression()?;

                self.add_token(TokenType::ConstTag(token::ConstTagToken {
                    declaration_span,
                }));

                Ok(())
            }
            _ => Err(Diagnostic::unexpected_keyword(Span::new(
                start as u32,
                self.current as u32,
            ))),
        }
    }

    fn start_snippet_tag(&mut self) -> Result<(), Diagnostic> {
        self.skip_whitespace();

        let name_start = self.current;
        let name = self.identifier();

        if name.is_empty() {
            return Err(Diagnostic::unexpected_token(Span::new(
                self.start as u32,
                self.current as u32,
            )));
        }

        let name_span = self.span(name_start, self.current);

        let params_span = if self.peek() == Some('(') {
            self.advance(); // consume '('
            let params_start = self.current;
            let mut depth: u32 = 1;

            while !self.is_at_end() && depth > 0 {
                let ch = self.advance();
                match ch {
                    '(' => depth += 1,
                    ')' => depth -= 1,
                    _ => {}
                }
            }

            if depth != 0 {
                return Err(Diagnostic::unexpected_end_of_file(Span::new(
                    params_start as u32,
                    self.current as u32,
                )));
            }

            let params_end = self.prev; // position of ')'
            let raw = self.slice_source(params_start, params_end);
            let value = raw.trim();
            let trim_start = raw.len() - raw.trim_start().len();
            let trim_end = raw.len() - raw.trim_end().len();
            let span_start = params_start + trim_start;
            let span_end = params_end - trim_end;

            if value.is_empty() {
                None
            } else {
                Some(Span::new(span_start as u32, span_end as u32))
            }
        } else {
            None
        };

        self.skip_whitespace();

        if !self.match_char('}') {
            self.recover(Diagnostic::unexpected_token(Span::new(
                self.start as u32,
                self.current as u32,
            )));
        }

        self.add_token(TokenType::StartSnippetTag(token::StartSnippetTag {
            name_span,
            params_span,
        }));

        Ok(())
    }

    fn start_each_tag(&mut self) -> Result<(), Diagnostic> {
        let mut collection_span = None;
        let mut item_span = None;

        self.skip_whitespace();

        let start_collection_pos = self.current;
        let mut end_collection_pos = start_collection_pos;

        while !self.is_at_end() {
            let peeked = self.peek();

            if !peeked.is_some_and(|c| c.is_ascii_whitespace()) {
                self.advance();
                continue;
            }

            end_collection_pos = self.current;

            self.skip_whitespace();

            let as_keyword = self.identifier();

            if as_keyword != "as" {
                continue;
            }

            collection_span = Some(self.span(start_collection_pos, end_collection_pos));

            self.skip_whitespace();

            item_span = Some(self.collect_each_context()?);

            break;
        }

        let Some(collection_span) = collection_span else {
            return Diagnostic::unexpected_token(Span::new(self.start as u32, self.current as u32)).as_err();
        };

        let Some(context_span) = item_span else {
            return Diagnostic::unexpected_token(Span::new(self.start as u32, self.current as u32)).as_err();
        };

        let last_char = self.slice_source(self.prev, self.prev + 1);

        // Parse optional index: `, i`
        let mut index_span = None;
        let mut key_span = None;

        if last_char == "," {
            self.skip_whitespace();
            let idx_start = self.current;
            let idx_name = self.identifier();
            if idx_name.is_empty() {
                return Diagnostic::unexpected_token(Span::new(self.current as u32, self.current as u32)).as_err();
            }
            index_span = Some(self.span(idx_start, idx_start + idx_name.len()));
            self.skip_whitespace();

            // After index, check for key `(expr)` or closing `}`
            if self.peek() == Some('(') {
                key_span = Some(self.collect_key_expression(false)?);
                self.skip_whitespace();
            }

            if !self.match_char('}') {
                return Diagnostic::unexpected_token(Span::new(self.current as u32, self.current as u32)).as_err();
            }
        } else if last_char == "(" {
            // Key expression directly after item (no index), `(` already consumed
            key_span = Some(self.collect_key_expression(true)?);
            self.skip_whitespace();

            if !self.match_char('}') {
                return Diagnostic::unexpected_token(Span::new(self.current as u32, self.current as u32)).as_err();
            }
        }
        // else: `}` — no index, no key

        self.add_token(TokenType::StartEachTag(StartEachTag {
            collection_span,
            context_span,
            index_span,
            key_span,
        }));

        Ok(())
    }

    /// Collect key expression in `{#each ... (key)}`.
    /// If `open_consumed` is true, `(` was already consumed by the caller.
    /// If false, `(` is expected at the current peek position and will be consumed.
    fn collect_key_expression(&mut self, open_consumed: bool) -> Result<Span, Diagnostic> {
        if !open_consumed {
            debug_assert_eq!(self.peek(), Some('('));
            self.advance(); // consume '('
        }

        let start = self.current;
        let mut depth: u32 = 1;

        while !self.is_at_end() && depth > 0 {
            let ch = self.advance();
            match ch {
                '\'' | '"' | '`' => self.skip_js_string(ch)?,
                '(' => depth += 1,
                ')' => depth -= 1,
                _ => {}
            }
        }

        if depth != 0 {
            return Err(Diagnostic::unexpected_end_of_file(Span::new(
                start as u32,
                self.current as u32,
            )));
        }

        let end = self.prev; // position of ')'
        let raw = self.slice_source(start, end);
        let trim_start = raw.len() - raw.trim_start().len();
        let trim_end = raw.len() - raw.trim_end().len();
        let span_start = start + trim_start;
        let span_end = end - trim_end;

        Ok(Span::new(span_start as u32, span_end as u32))
    }

    /// Collect item expression in each-block context.
    /// Stops on `,` or `}` at depth 0. Tracks `{}` and `[]` nesting for destructuring.
    fn collect_each_context(&mut self) -> Result<Span, Diagnostic> {
        let mut curly_depth: u32 = 0;
        let mut bracket_depth: u32 = 0;
        let start = self.current;

        while !self.is_at_end() {
            let char = self.advance();

            if char == '\'' || char == '"' || char == '`' {
                self.skip_js_string(char)?;
                continue;
            }

            match char {
                '{' => curly_depth += 1,
                '}' if curly_depth > 0 => curly_depth -= 1,
                '[' => bracket_depth += 1,
                ']' if bracket_depth > 0 => bracket_depth -= 1,
                // At depth 0: `,` means index follows, `(` means key follows, `}` means end of block
                ',' | '}' | '(' if curly_depth == 0 && bracket_depth == 0 => {
                    let raw = self.slice_source(start, self.prev);
                    let trim_start = raw.len() - raw.trim_start().len();
                    let trim_end = raw.len() - raw.trim_end().len();
                    let span_start = start + trim_start;
                    let span_end = self.prev - trim_end;

                    return Ok(Span::new(span_start as u32, span_end as u32));
                }
                _ => {}
            }
        }

        Err(Diagnostic::unexpected_end_of_file(Span::new(
            start as u32,
            self.current as u32,
        )))
    }
}

#[cfg(test)]
mod tests {
    use svelte_diagnostics::DiagnosticKind;

    use super::*;

    #[test]
    fn smoke() {
        let source = "<div>kek {name} hello</div>";
        let mut scanner = Scanner::new(source);

        let tokens = scanner.scan_tokens().0;

        assert!(matches!(tokens[0].token_type, TokenType::StartTag(_)));
        assert!(tokens[1].token_type == TokenType::Text);
        assert!(matches!(tokens[2].token_type, TokenType::Interpolation(_)));
        assert!(tokens[3].token_type == TokenType::Text);
        assert!(matches!(tokens[4].token_type, TokenType::EndTag(_)));
        assert!(tokens[5].token_type == TokenType::EOF);
    }

    #[test]
    fn interpolation_with_js_strings() {
        let mut scanner = Scanner::new("{ name + '}' + \"{}\" + `{\n}` }");

        let tokens = scanner.scan_tokens().0;

        assert!(matches!(tokens[0].token_type, TokenType::Interpolation(_)));
        assert!(tokens[1].token_type == TokenType::EOF);
    }

    #[test]
    fn interpolation_js_curly_braces_balance() {
        let mut scanner = Scanner::new("{ { field: 1} + (function(){return {}}) }");

        let tokens = scanner.scan_tokens().0;

        assert!(matches!(tokens[0].token_type, TokenType::Interpolation(_)));
        assert!(tokens[1].token_type == TokenType::EOF);
    }

    #[test]
    fn self_closed_start_tag() {
        let source = "<input/>";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().0;

        assert_start_tag(source, &tokens[0], "input", vec![], true);
        assert!(tokens[1].token_type == TokenType::EOF);
    }

    #[test]
    fn start_tag_attributes() {
        let source = "<div valid id=123 touched some=true disabled value=\"333\" class='never' >";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().0;

        assert_start_tag(
            source,
            &tokens[0],
            "div",
            vec![
                ("valid", ""),
                ("id", "123"),
                ("touched", ""),
                ("some", "true"),
                ("disabled", ""),
                ("value", "333"),
                ("class", "never"),
            ],
            false,
        );

        assert!(tokens[1].token_type == TokenType::EOF);
    }

    #[test]
    fn comment() {
        let source = "<!-- \nsome comment\n -->";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().0;

        assert!(tokens[0].token_type == TokenType::Comment);
        assert_eq!(tokens[0].span.source_text(source), "<!-- \nsome comment\n -->");
        assert!(tokens[1].token_type == TokenType::EOF);
    }

    #[test]
    fn each_block() {
        let source = "{#each [1,2,3] as { value, flag }}";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().0;

        assert_start_each_tag(source, &tokens[0], "[1,2,3]", "{ value, flag }");
        assert!(tokens[1].token_type == TokenType::EOF);
    }

    fn assert_start_tag(
        source: &str,
        token: &Token,
        expected_name: &str,
        expected_attributes: Vec<(&str, &str)>,
        expected_self_closing: bool,
    ) {
        let start_tag = match &token.token_type {
            TokenType::StartTag(t) => t,
            _ => panic!("Expected token.type = StartTag."),
        };

        assert_eq!(start_tag.name_span.source_text(source), expected_name);
        assert_eq!(start_tag.self_closing, expected_self_closing);
        assert_attributes(source, &start_tag.attributes, expected_attributes);
    }

    fn assert_attributes(
        source: &str,
        actual_attributes: &[Attribute],
        expected_attributes: Vec<(&str, &str)>,
    ) {
        assert_eq!(actual_attributes.len(), expected_attributes.len());

        for (index, (expected_name, expected_value)) in expected_attributes.iter().enumerate() {
            let attribute = &actual_attributes[index];

            let name = match attribute {
                Attribute::HTMLAttribute(value) => value.name_span.source_text(source),
                Attribute::ExpressionTag(_) => "$expression",
                Attribute::ClassDirective(_) => "$classDirective",
                Attribute::StyleDirective(sd) => sd.name_span.source_text(source),
                Attribute::BindDirective(_) => "$bindDirective",
                Attribute::UseDirective(ud) => ud.name_span.source_text(source),
                Attribute::OnDirectiveLegacy(od) => od.name_span.source_text(source),
                Attribute::TransitionDirective(td) => td.name_span.source_text(source),
                Attribute::AnimateDirective(ad) => ad.name_span.source_text(source),
                Attribute::AttachTag(_) => "$attachTag",
            };

            let value: String = match attribute {
                Attribute::HTMLAttribute(value) => match value.value {
                    AttributeValue::String(span) => span.source_text(source).to_string(),
                    AttributeValue::Empty => String::new(),
                    AttributeValue::ExpressionTag(ref et) => et.expression_span.source_text(source).to_string(),
                    AttributeValue::Concatenation(ref c) => {
                        c.parts.iter().map(|p| match p {
                            ConcatenationPart::String(span) => format!("({})", span.source_text(source)),
                            ConcatenationPart::Expression(et) => format!("({{{}}})", et.expression_span.source_text(source)),
                        }).collect()
                    }
                },
                Attribute::ExpressionTag(value) => value.expression_span.source_text(source).to_string(),
                Attribute::ClassDirective(cd) => cd.expression_span.source_text(source).to_string(),
                Attribute::StyleDirective(sd) => match sd.value {
                    AttributeValue::String(span) => span.source_text(source).to_string(),
                    AttributeValue::Empty => String::new(),
                    AttributeValue::ExpressionTag(ref et) => et.expression_span.source_text(source).to_string(),
                    AttributeValue::Concatenation(ref c) => {
                        c.parts.iter().map(|p| match p {
                            ConcatenationPart::String(span) => format!("({})", span.source_text(source)),
                            ConcatenationPart::Expression(et) => format!("({{{}}})", et.expression_span.source_text(source)),
                        }).collect()
                    }
                },
                Attribute::BindDirective(bd) => bd.expression_span.source_text(source).to_string(),
                Attribute::UseDirective(ud) => ud.expression_span.source_text(source).to_string(),
                Attribute::OnDirectiveLegacy(od) => od.expression_span.source_text(source).to_string(),
                Attribute::TransitionDirective(td) => td.expression_span.source_text(source).to_string(),
                Attribute::AnimateDirective(ad) => ad.expression_span.source_text(source).to_string(),
                Attribute::AttachTag(at) => at.expression_span.source_text(source).to_string(),
            };

            assert_eq!(name, *expected_name);
            assert_eq!(value, expected_value.to_string());
        }
    }

    fn assert_start_each_tag(source: &str, token: &Token, expected_collection: &str, expected_item: &str) {
        let tag = match &token.token_type {
            TokenType::StartEachTag(t) => t,
            _ => panic!("Expected token.type = StartEachTag"),
        };

        assert_eq!(tag.collection_span.source_text(source), expected_collection);
        assert_eq!(tag.context_span.source_text(source), expected_item);
        assert!(tag.index_span.is_none(), "expected no index");
    }

    fn assert_start_each_tag_with_index(
        source: &str,
        token: &Token,
        expected_collection: &str,
        expected_item: &str,
        expected_index: &str,
    ) {
        let tag = match &token.token_type {
            TokenType::StartEachTag(t) => t,
            _ => panic!("Expected token.type = StartEachTag"),
        };

        assert_eq!(tag.collection_span.source_text(source), expected_collection);
        assert_eq!(tag.context_span.source_text(source), expected_item);
        let index = tag.index_span.as_ref().expect("expected index");
        assert_eq!(index.source_text(source), expected_index);
    }

    #[test]
    fn each_block_with_index() {
        let source = "{#each items as item, i}";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().0;

        assert_start_each_tag_with_index(source, &tokens[0], "items", "item", "i");
        assert!(tokens[1].token_type == TokenType::EOF);
    }

    #[test]
    fn each_block_destructured_with_index() {
        let source = "{#each items as { value, flag }, idx}";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().0;

        assert_start_each_tag_with_index(source, &tokens[0], "items", "{ value, flag }", "idx");
        assert!(tokens[1].token_type == TokenType::EOF);
    }

    fn assert_has_diagnostic(diagnostics: &[Diagnostic], err_kind: DiagnosticKind) {
        assert!(
            diagnostics.iter().any(|d| d.kind == err_kind),
            "expected diagnostic {err_kind:?}, got: {diagnostics:?}"
        );
    }

    #[test]
    fn unterminated_start_tag() {
        let mut scanner = Scanner::new("<div disabled");
        let (_, diagnostics) = scanner.scan_tokens();
        assert_has_diagnostic(&diagnostics, DiagnosticKind::UnterminatedStartTag);
    }

    // --- Escape sequence tests (Bug #1) ---

    #[test]
    fn interpolation_escaped_double_quote() {
        let mut scanner = Scanner::new(r#"{ name.replace("\"", "'") }"#);
        let tokens = scanner.scan_tokens().0;
        assert!(matches!(tokens[0].token_type, TokenType::Interpolation(_)));
        assert!(tokens[1].token_type == TokenType::EOF);
    }

    #[test]
    fn interpolation_escaped_single_quote() {
        let mut scanner = Scanner::new(r"{ 'it\'s a test' }");
        let tokens = scanner.scan_tokens().0;
        assert!(matches!(tokens[0].token_type, TokenType::Interpolation(_)));
        assert!(tokens[1].token_type == TokenType::EOF);
    }

    #[test]
    fn interpolation_escaped_backtick() {
        let mut scanner = Scanner::new(r"{ `hello \` world` }");
        let tokens = scanner.scan_tokens().0;
        assert!(matches!(tokens[0].token_type, TokenType::Interpolation(_)));
        assert!(tokens[1].token_type == TokenType::EOF);
    }

    #[test]
    fn interpolation_escaped_backslash() {
        let mut scanner = Scanner::new(r#"{ "path\\to\\file" }"#);
        let tokens = scanner.scan_tokens().0;
        assert!(matches!(tokens[0].token_type, TokenType::Interpolation(_)));
        assert!(tokens[1].token_type == TokenType::EOF);
    }

    // --- Style tag tests (Bug #2) ---

    #[test]
    fn style_tag_basic() {
        let source = "<style>.foo { color: red; }</style>";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().0;
        assert!(matches!(tokens[0].token_type, TokenType::StyleTag(_)));
        if let TokenType::StyleTag(ref st) = tokens[0].token_type {
            assert_eq!(st.content_span.source_text(source), ".foo { color: red; }");
        }
        assert!(tokens[1].token_type == TokenType::EOF);
    }

    #[test]
    fn style_tag_with_angle_brackets() {
        let source = "<style>a > b { color: red; }</style>";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().0;
        assert!(matches!(tokens[0].token_type, TokenType::StyleTag(_)));
        if let TokenType::StyleTag(ref st) = tokens[0].token_type {
            assert_eq!(st.content_span.source_text(source), "a > b { color: red; }");
        }
    }

    // --- Each block key tests (Bug #3) ---

    fn assert_start_each_tag_with_key(
        source: &str,
        token: &Token,
        expected_collection: &str,
        expected_item: &str,
        expected_key: &str,
    ) {
        let tag = match &token.token_type {
            TokenType::StartEachTag(t) => t,
            _ => panic!("Expected token.type = StartEachTag"),
        };

        assert_eq!(tag.collection_span.source_text(source), expected_collection);
        assert_eq!(tag.context_span.source_text(source), expected_item);
        let key = tag.key_span.as_ref().expect("expected key");
        assert_eq!(key.source_text(source), expected_key);
    }

    #[test]
    fn each_block_with_key() {
        let source = "{#each items as item (item.id)}";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().0;
        assert_start_each_tag_with_key(source, &tokens[0], "items", "item", "item.id");
        assert!(tokens[1].token_type == TokenType::EOF);
    }

    #[test]
    fn each_block_with_index_and_key() {
        let source = "{#each items as item, i (item.id)}";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().0;

        let tag = match &tokens[0].token_type {
            TokenType::StartEachTag(t) => t,
            _ => panic!("Expected StartEachTag"),
        };
        assert_eq!(tag.collection_span.source_text(source), "items");
        assert_eq!(tag.context_span.source_text(source), "item");
        assert_eq!(tag.index_span.as_ref().unwrap().source_text(source), "i");
        assert_eq!(tag.key_span.as_ref().unwrap().source_text(source), "item.id");
    }

    #[test]
    fn each_block_destructured_with_key() {
        let source = "{#each items as {name} (name)}";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().0;
        assert_start_each_tag_with_key(source, &tokens[0], "items", "{name}", "name");
    }

    // --- Directive tests ---

    #[test]
    fn class_directive_with_expression() {
        let source = "<div class:active={isActive}>";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().0;
        assert_start_tag(
            source,
            &tokens[0],
            "div",
            vec![("$classDirective", "isActive")],
            false,
        );
    }

    #[test]
    fn class_directive_shorthand() {
        let source = "<div class:active>";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().0;
        assert_start_tag(
            source,
            &tokens[0],
            "div",
            vec![("$classDirective", "active")],
            false,
        );
    }

    #[test]
    fn bind_directive_with_expression() {
        let source = "<input bind:value={name}>";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().0;
        assert_start_tag(
            source,
            &tokens[0],
            "input",
            vec![("$bindDirective", "name")],
            true,
        );
    }

    #[test]
    fn bind_directive_shorthand() {
        let source = "<input bind:value>";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().0;
        assert_start_tag(
            source,
            &tokens[0],
            "input",
            vec![("$bindDirective", "value")],
            true,
        );
    }

    // --- Attribute concatenation tests ---

    #[test]
    fn attribute_concatenation() {
        let source = r#"<div title="hello {name} world">"#;
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().0;
        assert!(matches!(tokens[0].token_type, TokenType::StartTag(_)));
        if let TokenType::StartTag(ref st) = tokens[0].token_type {
            assert_eq!(st.name_span.source_text(source), "div");
            assert_eq!(st.attributes.len(), 1);
            if let Attribute::HTMLAttribute(ref attr) = st.attributes[0] {
                assert_eq!(attr.name_span.source_text(source), "title");
                assert!(matches!(attr.value, AttributeValue::Concatenation(_)));
            } else {
                panic!("Expected HTMLAttribute");
            }
        }
    }

    // --- Spread/shorthand attribute tests ---

    #[test]
    fn spread_attribute() {
        let source = "<div {...props}>";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().0;
        assert!(matches!(tokens[0].token_type, TokenType::StartTag(_)));
        if let TokenType::StartTag(ref st) = tokens[0].token_type {
            assert_eq!(st.attributes.len(), 1);
            if let Attribute::ExpressionTag(ref et) = st.attributes[0] {
                assert_eq!(et.expression_span.source_text(source), "...props");
            } else {
                panic!("Expected ExpressionTag for spread");
            }
        }
    }

    #[test]
    fn shorthand_attribute() {
        let source = "<div {value}>";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().0;
        assert!(matches!(tokens[0].token_type, TokenType::StartTag(_)));
        if let TokenType::StartTag(ref st) = tokens[0].token_type {
            assert_eq!(st.attributes.len(), 1);
            if let Attribute::ExpressionTag(ref et) = st.attributes[0] {
                assert_eq!(et.expression_span.source_text(source), "value");
            } else {
                panic!("Expected ExpressionTag for shorthand");
            }
        }
    }

    // --- Snippet/render token tests ---

    #[test]
    fn snippet_tag_tokens() {
        let source = "{#snippet foo(a, b)}content{/snippet}";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().0;
        assert!(matches!(tokens[0].token_type, TokenType::StartSnippetTag(_)));
        if let TokenType::StartSnippetTag(ref st) = tokens[0].token_type {
            assert_eq!(st.name_span.source_text(source), "foo");
            assert_eq!(st.params_span.as_ref().unwrap().source_text(source), "a, b");
        }
        assert!(tokens[1].token_type == TokenType::Text);
        assert!(tokens[2].token_type == TokenType::EndSnippetTag);
    }

    #[test]
    fn render_tag_tokens() {
        let source = "{@render foo(x, y)}";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().0;
        assert!(matches!(tokens[0].token_type, TokenType::RenderTag(_)));
        if let TokenType::RenderTag(ref rt) = tokens[0].token_type {
            assert_eq!(rt.expression_span.source_text(source), "foo(x, y)");
        }
    }

    #[test]
    fn html_tag_tokens() {
        let source = "{@html content}";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().0;
        assert!(matches!(tokens[0].token_type, TokenType::HtmlTag(_)));
        if let TokenType::HtmlTag(ref ht) = tokens[0].token_type {
            assert_eq!(ht.expression_span.source_text(source), "content");
        }
    }

    #[test]
    fn const_tag_tokens() {
        let source = "{@const doubled = item * 2}";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().0;
        assert!(matches!(tokens[0].token_type, TokenType::ConstTag(_)));
        if let TokenType::ConstTag(ref ct) = tokens[0].token_type {
            assert_eq!(ct.declaration_span.source_text(source), "doubled = item * 2");
        }
    }

    // --- Scanner error recovery tests ---

    #[test]
    fn recovery_unterminated_start_tag_bare() {
        let source = "<div";
        let mut scanner = Scanner::new(source);
        let (tokens, diagnostics) = scanner.scan_tokens();
        assert_start_tag(source, &tokens[0], "div", vec![], false);
        assert!(tokens[1].token_type == TokenType::EOF);
        assert_has_diagnostic(&diagnostics, DiagnosticKind::UnterminatedStartTag);
    }

    #[test]
    fn recovery_unterminated_start_tag_with_bool_attr() {
        let source = "<div class";
        let mut scanner = Scanner::new(source);
        let (tokens, diagnostics) = scanner.scan_tokens();
        assert_start_tag(source, &tokens[0], "div", vec![("class", "")], false);
        assert!(tokens[1].token_type == TokenType::EOF);
        assert_has_diagnostic(&diagnostics, DiagnosticKind::UnterminatedStartTag);
    }

    #[test]
    fn recovery_unterminated_start_tag_with_partial_attr() {
        let mut scanner = Scanner::new("<div class=");
        let (tokens, diagnostics) = scanner.scan_tokens();
        // StartTag emitted with partial attrs, EOF diagnostic
        assert!(matches!(tokens[0].token_type, TokenType::StartTag(_)));
        assert!(tokens.last().unwrap().token_type == TokenType::EOF);
        assert!(!diagnostics.is_empty());
    }

    #[test]
    fn recovery_unclosed_script_tag() {
        let source = "<script>code";
        let mut scanner = Scanner::new(source);
        let (tokens, diagnostics) = scanner.scan_tokens();
        assert!(matches!(tokens[0].token_type, TokenType::ScriptTag(_)));
        if let TokenType::ScriptTag(ref st) = tokens[0].token_type {
            assert_eq!(st.content_span.source_text(source), "code");
        }
        assert!(tokens.last().unwrap().token_type == TokenType::EOF);
        assert_has_diagnostic(&diagnostics, DiagnosticKind::UnexpectedEndOfFile);
    }

    #[test]
    fn recovery_unclosed_style_tag() {
        let source = "<style>.foo{}";
        let mut scanner = Scanner::new(source);
        let (tokens, diagnostics) = scanner.scan_tokens();
        assert!(matches!(tokens[0].token_type, TokenType::StyleTag(_)));
        if let TokenType::StyleTag(ref st) = tokens[0].token_type {
            assert_eq!(st.content_span.source_text(source), ".foo{}");
        }
        assert!(tokens.last().unwrap().token_type == TokenType::EOF);
        assert_has_diagnostic(&diagnostics, DiagnosticKind::UnexpectedEndOfFile);
    }

    #[test]
    fn recovery_unclosed_comment() {
        let mut scanner = Scanner::new("<!-- text");
        let (tokens, diagnostics) = scanner.scan_tokens();
        assert!(tokens[0].token_type == TokenType::Comment);
        assert!(tokens.last().unwrap().token_type == TokenType::EOF);
        assert_has_diagnostic(&diagnostics, DiagnosticKind::UnexpectedEndOfFile);
    }

    #[test]
    fn recovery_unclosed_interpolation() {
        let source = "{name";
        let mut scanner = Scanner::new(source);
        let (tokens, diagnostics) = scanner.scan_tokens();
        assert!(matches!(tokens[0].token_type, TokenType::Interpolation(_)));
        if let TokenType::Interpolation(ref et) = tokens[0].token_type {
            assert_eq!(et.expression_span.source_text(source), "name");
        }
        assert!(tokens.last().unwrap().token_type == TokenType::EOF);
        assert_has_diagnostic(&diagnostics, DiagnosticKind::UnexpectedEndOfFile);
    }

    #[test]
    fn recovery_unclosed_if_tag() {
        let source = "{#if cond";
        let mut scanner = Scanner::new(source);
        let (tokens, diagnostics) = scanner.scan_tokens();
        assert!(matches!(tokens[0].token_type, TokenType::StartIfTag(_)));
        if let TokenType::StartIfTag(ref st) = tokens[0].token_type {
            assert_eq!(st.expression_span.source_text(source), "cond");
        }
        assert!(tokens.last().unwrap().token_type == TokenType::EOF);
        assert_has_diagnostic(&diagnostics, DiagnosticKind::UnexpectedEndOfFile);
    }

    #[test]
    fn recovery_attribute_concatenation_eof() {
        let mut scanner = Scanner::new(r#"<div class="foo"#);
        let (tokens, diagnostics) = scanner.scan_tokens();
        assert!(matches!(tokens[0].token_type, TokenType::StartTag(_)));
        assert!(!diagnostics.is_empty());
    }

    #[test]
    fn recovery_start_tag_then_more_content() {
        // Partial tag followed by valid content — scanner should recover
        let mut scanner = Scanner::new("<div<p>hello</p>");
        let (tokens, diagnostics) = scanner.scan_tokens();
        // At least some tokens should be emitted
        assert!(tokens.len() > 1);
        assert!(tokens.last().unwrap().token_type == TokenType::EOF);
        assert!(!diagnostics.is_empty());
    }
}
