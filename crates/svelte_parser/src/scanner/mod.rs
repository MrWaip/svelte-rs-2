pub mod token;

use std::vec;

pub use svelte_ast::is_void;

use token::{
    AnimateDirective, AttachTagToken, Attribute, AttributeIdentifierType, AttributeValue,
    BindDirective, ClassDirective, Concatenation, ConcatenationPart, ExpressionTag, HTMLAttribute,
    LetDirectiveLegacy, OnDirectiveLegacy, ScriptTag, StartEachTag, StartIfTag, StartKeyTag,
    StartTag, StyleDirective, Token, TokenType, TransitionDirective, UseDirective,
};

use svelte_diagnostics::Diagnostic;
use svelte_span::{SPAN, Span};

pub struct Scanner<'a> {
    source: &'a str,
    bytes: &'a [u8],
    tokens: Vec<Token>,
    diagnostics: Vec<Diagnostic>,
    start: usize,
    prev: usize,
    current: usize,
    fragment_depth: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum JsExprState {
    ExpectOperand,
    ExpectOperator,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum JsScanTerminator {
    SvelteBrace,
    MatchingParen,
    TemplateExpression,
    EachContext,
    AwaitBinding,
}

struct JsScanResult {
    end: Option<usize>,
    await_clause: Option<&'static str>,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Scanner<'a> {
        Scanner {
            source,
            bytes: source.as_bytes(),
            tokens: vec![],
            diagnostics: vec![],
            prev: 0,
            current: 0,
            start: 0,
            fragment_depth: 0,
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

    fn sync_to_next_token(&mut self) {
        while !self.is_at_end() {
            match self.peek() {
                Some('<') | Some('{') => break,
                _ => {
                    self.advance();
                }
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
            self.skip_whitespace();
            return match self.peek() {
                Some('#') => self.start_template(),
                Some(':') => self.middle_template(),
                Some('@') => self.at_template(),
                Some('/') => self.end_template(),
                _ => self.interpolation(),
            };
        }

        self.text();

        Ok(())
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.push_token(
            token_type,
            Span::new(self.start as u32, self.current as u32),
        );
    }

    fn push_token(&mut self, token_type: TokenType, span: Span) {
        self.tokens.push(Token { token_type, span });
    }

    fn enter_fragment(&mut self) {
        self.fragment_depth += 1;
    }

    fn leave_fragment(&mut self) {
        self.fragment_depth = self.fragment_depth.saturating_sub(1);
    }

    fn advance(&mut self) -> char {
        self.prev = self.current;
        let b = self.bytes[self.current];
        if b < 0x80 {
            self.current += 1;
            b as char
        } else {
            let ch = self.source[self.current..]
                .chars()
                .next()
                .expect("source slice is non-empty — b was read above");
            self.current += ch.len_utf8();
            ch
        }
    }

    #[inline]
    fn is_at_end(&self) -> bool {
        self.current >= self.bytes.len()
    }

    #[inline]
    fn peek_byte(&self) -> Option<u8> {
        self.bytes.get(self.current).copied()
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

    fn is_js_identifier_start(ch: char) -> bool {
        ch.is_alphabetic() || matches!(ch, '_' | '$')
    }

    fn js_identifier_segment(&mut self) -> &'a str {
        let start = self.current;

        if !self.peek().is_some_and(Self::is_js_identifier_start) {
            return self.slice_source(start, start);
        }

        self.advance();
        while let Some(ch) = self.peek() {
            if ch.is_alphanumeric() || matches!(ch, '_' | '$') {
                self.advance();
            } else {
                break;
            }
        }

        self.slice_source(start, self.current)
    }

    fn try_component_tag_name(&mut self) -> Option<&'a str> {
        let start = self.current;
        let name = self.js_identifier_segment();

        if name.is_empty() {
            return None;
        }

        if name.starts_with(|ch: char| ch.is_uppercase()) {
            self.consume_dotted_tag_suffix();
            return Some(self.slice_source(start, self.current));
        }

        if self.peek() != Some('.') {
            self.current = start;
            return None;
        }

        let dotted_start = self.current;
        self.consume_dotted_tag_suffix();
        if self.current == dotted_start {
            self.current = start;
            return None;
        }

        Some(self.slice_source(start, self.current))
    }

    fn consume_dotted_tag_suffix(&mut self) {
        while self.peek() == Some('.') {
            let dot_pos = self.current;
            self.advance();

            let segment = self.scan_js_identifier();
            if segment.is_empty() {
                self.current = dot_pos;
                break;
            }
        }
    }

    fn slice_source(&self, start: usize, end: usize) -> &'a str {
        &self.source[start..end]
    }

    fn span(&self, start: usize, end: usize) -> Span {
        Span::new(start as u32, end as u32)
    }

    fn trimmed_span(&self, start: usize, end: usize) -> Span {
        let raw = self.slice_source(start, end);
        let trim_start = raw.len() - raw.trim_start().len();
        let trim_end = raw.len() - raw.trim_end().len();
        let span_start = start + trim_start;
        let span_end = end.saturating_sub(trim_end);
        Span::new(span_start as u32, span_end as u32)
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
            } else if AttributeIdentifierType::is_let_directive(name) {
                AttributeIdentifierType::LetDirectiveLegacy(value_span, value).as_ok()
            } else if AttributeIdentifierType::is_use_directive(name) {
                AttributeIdentifierType::UseDirective(value_span, value).as_ok()
            } else if AttributeIdentifierType::is_on_directive(name) {
                AttributeIdentifierType::OnDirectiveLegacy(value_span, value).as_ok()
            } else if AttributeIdentifierType::is_transition_directive(name) {
                AttributeIdentifierType::TransitionDirective(value_span, name).as_ok()
            } else if AttributeIdentifierType::is_animate_directive(name) {
                AttributeIdentifierType::AnimateDirective(value_span, value).as_ok()
            } else {
                let full_span = self.span(start, self.current);
                AttributeIdentifierType::HTMLAttribute(
                    full_span,
                    self.slice_source(start, self.current),
                )
                .as_ok()
            }
        } else if start == self.current {
            AttributeIdentifierType::None.as_ok()
        } else {
            let full_span = self.span(start, self.current);
            AttributeIdentifierType::HTMLAttribute(
                full_span,
                self.slice_source(start, self.current),
            )
            .as_ok()
        }
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.peek_byte().is_some_and(|b| b == expected as u8) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn peek(&self) -> Option<char> {
        if self.current >= self.bytes.len() {
            return None;
        }
        let b = self.bytes[self.current];
        if b < 0x80 {
            Some(b as char)
        } else {
            self.source[self.current..].chars().next()
        }
    }

    fn peek_next(&self) -> Option<char> {
        let current = self.peek()?;
        let next = self.current + current.len_utf8();
        if next >= self.bytes.len() {
            return None;
        }
        let b = self.bytes[next];
        if b < 0x80 {
            Some(b as char)
        } else {
            self.source[next..].chars().next()
        }
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

    fn start_tag(&mut self) -> Result<(), Diagnostic> {
        let name_start = self.current;
        let name = if let Some(name) = self.try_component_tag_name() {
            name
        } else {
            self.identifier()
        };

        if name.is_empty() {
            return Err(Diagnostic::invalid_tag_name(
                self.span(name_start, self.current),
            ));
        }

        self.consume_dotted_tag_suffix();

        if name == "svelte" && self.peek() == Some(':') {
            self.advance();
            self.identifier();
        }

        let name_span = self.span(name_start, self.current);

        let attributes = self.attributes()?;
        let self_closing = self.match_char('/') || is_void(name);

        if !self.match_char('>') {
            self.recover(Diagnostic::unterminated_start_tag(
                self.span(name_start, self.current),
            ));

            self.add_token(TokenType::StartTag(StartTag {
                attributes,
                name_span,
                self_closing,
            }));

            return Ok(());
        }

        if matches!(name, "script" | "style") && !self_closing {
            return if self.fragment_depth == 0 {
                if name == "script" {
                    self.script_tag(&attributes, name_span)
                } else {
                    self.style_tag(name_span)
                }
            } else {
                self.raw_text_element(name, name_span, attributes)
            };
        }

        self.add_token(TokenType::StartTag(StartTag {
            attributes,
            name_span,
            self_closing,
        }));
        if !self_closing {
            self.enter_fragment();
        }

        Ok(())
    }

    fn attributes(&mut self) -> Result<Vec<Attribute>, Diagnostic> {
        let mut attributes: Vec<Attribute> = vec![];

        loop {
            self.skip_whitespace();

            let peeked = self.peek();
            let Some(ch) = peeked else {
                break;
            };

            if ch == '/' || ch == '>' {
                break;
            }

            let attr_start = self.current;

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
                    AttributeIdentifierType::StyleDirective(span, _) => self.style_directive(span),
                    AttributeIdentifierType::BindDirective(span, name) => {
                        self.bind_directive(span, name)
                    }
                    AttributeIdentifierType::LetDirectiveLegacy(span, name) => {
                        self.let_directive_legacy(span, name)
                    }
                    AttributeIdentifierType::UseDirective(span, _) => self.use_directive(span),

                    AttributeIdentifierType::OnDirectiveLegacy(span, _) => {
                        self.on_directive_legacy(span)
                    }
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
                Ok(attr) => attributes.push(attr.with_span(self.span(attr_start, self.current))),
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

        Ok(Attribute::HTMLAttribute(HTMLAttribute {
            span: SPAN,
            name_span,
            value,
        }))
    }

    fn class_directive(&mut self, name_span: Span, _name: &str) -> Result<Attribute, Diagnostic> {
        if self.match_char('=') {
            let res = self.expression_tag()?;

            return Ok(Attribute::ClassDirective(ClassDirective {
                span: SPAN,
                expression_span: res.expression_span,
                name_span,
                shorthand: false,
            }));
        }

        Ok(Attribute::ClassDirective(ClassDirective {
            span: SPAN,
            name_span,
            expression_span: name_span,
            shorthand: true,
        }))
    }

    fn style_directive(&mut self, name_span: Span) -> Result<Attribute, Diagnostic> {
        let important = if self.match_char('|') {
            let start = self.current;
            while self.peek().is_some_and(|c| c.is_alphabetic()) {
                self.advance();
            }
            let modifier = self.slice_source(start, self.current);
            if modifier != "important" {
                self.recover(Diagnostic::unknown_directive(Span::new(
                    start as u32 - 1,
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
                span: SPAN,
                value,
                name_span,
                shorthand: false,
                important,
            }));
        }

        Ok(Attribute::StyleDirective(StyleDirective {
            span: SPAN,
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
                span: SPAN,
                expression_span: res.expression_span,
                name_span,
                shorthand: false,
            }));
        }

        Ok(Attribute::BindDirective(BindDirective {
            span: SPAN,
            name_span,
            expression_span: name_span,
            shorthand: true,
        }))
    }

    fn let_directive_legacy(
        &mut self,
        name_span: Span,
        _name: &str,
    ) -> Result<Attribute, Diagnostic> {
        if self.match_char('=') {
            let res = self.expression_tag()?;

            return Ok(Attribute::LetDirectiveLegacy(LetDirectiveLegacy {
                span: SPAN,
                name_span,
                expression_span: res.expression_span,
                has_expression: true,
            }));
        }

        Ok(Attribute::LetDirectiveLegacy(LetDirectiveLegacy {
            span: SPAN,
            name_span,
            expression_span: SPAN,
            has_expression: false,
        }))
    }

    fn use_directive(&mut self, mut name_span: Span) -> Result<Attribute, Diagnostic> {
        while self.peek() == Some('.') {
            self.advance();
            while self
                .peek()
                .is_some_and(|c| c.is_alphanumeric() || c == '_' || c == '-')
            {
                self.advance();
            }
            name_span = Span::new(name_span.start, self.current as u32);
        }

        if self.match_char('=') {
            let res = self.expression_tag()?;

            return Ok(Attribute::UseDirective(UseDirective {
                span: SPAN,
                expression_span: res.expression_span,
                name_span,
                shorthand: false,
            }));
        }

        Ok(Attribute::UseDirective(UseDirective {
            span: SPAN,
            name_span,
            expression_span: name_span,
            shorthand: true,
        }))
    }

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
                span: SPAN,
                name_span,
                expression_span: res.expression_span,
                modifiers,
                has_expression: true,
            }));
        }

        Ok(Attribute::OnDirectiveLegacy(OnDirectiveLegacy {
            span: SPAN,
            name_span,
            expression_span: SPAN,
            modifiers,
            has_expression: false,
        }))
    }

    fn transition_directive(
        &mut self,
        mut name_span: Span,
        prefix: &str,
    ) -> Result<Attribute, Diagnostic> {
        while self.peek() == Some('.') {
            self.advance();
            while self.peek().is_some_and(|c| c.is_alphanumeric() || c == '_') {
                self.advance();
            }
            name_span = Span::new(name_span.start, self.current as u32);
        }

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
                span: SPAN,
                name_span,
                expression_span: res.expression_span,
                modifiers,
                has_expression: true,
                direction_prefix: prefix.to_string(),
            }));
        }

        Ok(Attribute::TransitionDirective(TransitionDirective {
            span: SPAN,
            name_span,
            expression_span: SPAN,
            modifiers,
            has_expression: false,
            direction_prefix: prefix.to_string(),
        }))
    }

    fn animate_directive(&mut self, mut name_span: Span) -> Result<Attribute, Diagnostic> {
        while self.peek() == Some('.') {
            self.advance();
            while self.peek().is_some_and(|c| c.is_alphanumeric() || c == '_') {
                self.advance();
            }
            name_span = Span::new(name_span.start, self.current as u32);
        }

        if self.match_char('=') {
            let res = self.expression_tag()?;
            return Ok(Attribute::AnimateDirective(AnimateDirective {
                span: SPAN,
                name_span,
                expression_span: res.expression_span,
                has_expression: true,
            }));
        }

        Ok(Attribute::AnimateDirective(AnimateDirective {
            span: SPAN,
            name_span,
            expression_span: SPAN,
            has_expression: false,
        }))
    }

    fn attach_tag_attribute(&mut self) -> Result<AttachTagToken, Diagnostic> {
        debug_assert!(self.source[self.current..].starts_with("{@attach"));

        for _ in 0.."{@attach".len() {
            self.advance();
        }

        self.skip_whitespace();
        let expression_span = self.collect_js_expression()?;

        Ok(AttachTagToken {
            span: SPAN,
            expression_span,
        })
    }

    fn attribute_value(&mut self) -> Result<AttributeValue, Diagnostic> {
        let peeked = self.peek();

        if self.peek() == Some('{') {
            return self.expression_tag().map(AttributeValue::ExpressionTag);
        }

        if let Some(quote) = peeked.filter(|c| *c == '"' || *c == '\'') {
            return self.attribute_concatenation_or_string(quote);
        }

        self.unquoted_attribute_concatenation_or_string()
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

        self.advance();
        let mut current_pos: usize = self.current;

        while let Some(char) = self.peek() {
            if char == quote {
                break;
            }

            if char == '{' {
                has_expression = true;

                if current_pos < self.current {
                    parts.push(ConcatenationPart::String(
                        self.span(current_pos, self.current),
                    ));
                }

                let expression_tag = self.expression_tag()?;

                parts.push(ConcatenationPart::Expression(expression_tag));
                current_pos = self.current;

                continue;
            }

            self.advance();
        }

        let last_span = self.span(current_pos, self.current);

        if !self.is_at_end() {
            self.advance();
        } else {
            self.recover(Diagnostic::unexpected_end_of_file(Span::new(
                start as u32,
                self.current as u32,
            )));
        }

        if has_expression && current_pos < self.current - 1 && last_span.start != last_span.end {
            parts.push(ConcatenationPart::String(last_span));
        }

        if !has_expression && parts.is_empty() {
            return Ok(AttributeValue::String(last_span));
        }

        Ok(AttributeValue::Concatenation(Concatenation {
            span: self.span(start, self.current),
            parts,
        }))
    }

    fn unquoted_attribute_concatenation_or_string(&mut self) -> Result<AttributeValue, Diagnostic> {
        let start = self.current;
        let mut current_pos = self.current;
        let mut has_expression = false;
        let mut parts: Vec<ConcatenationPart> = vec![];

        while let Some(char) = self.peek() {
            if matches!(char, '"' | '\'' | '>' | '<' | '`') || char.is_whitespace() {
                break;
            }

            if char == '{' {
                has_expression = true;

                if current_pos < self.current {
                    parts.push(ConcatenationPart::String(
                        self.span(current_pos, self.current),
                    ));
                }

                let expression_tag = self.expression_tag()?;
                parts.push(ConcatenationPart::Expression(expression_tag));
                current_pos = self.current;
                continue;
            }

            self.advance();
        }

        let end = self.current;
        if !has_expression {
            return Ok(AttributeValue::String(self.span(start, end)));
        }

        if current_pos < end {
            parts.push(ConcatenationPart::String(self.span(current_pos, end)));
        }

        Ok(AttributeValue::Concatenation(Concatenation {
            span: self.span(start, end),
            parts,
        }))
    }

    fn end_tag(&mut self) -> Result<(), Diagnostic> {
        self.advance();

        let name_start = self.current;
        let name = if let Some(name) = self.try_component_tag_name() {
            name
        } else {
            self.identifier()
        };

        if name.is_empty() {
            return Err(Diagnostic::invalid_tag_name(
                self.span(name_start, self.current),
            ));
        }

        self.consume_dotted_tag_suffix();

        if name == "svelte" && self.peek() == Some(':') {
            self.advance();
            self.identifier();
        }

        let name_span = self.span(name_start, self.current);

        self.skip_whitespace();

        if !self.match_char('>') {
            self.recover(Diagnostic::unexpected_token(
                self.span(name_start, self.current),
            ));
        }

        self.add_token(TokenType::EndTag(token::EndTag { name_span }));
        self.leave_fragment();

        Ok(())
    }

    fn raw_text_element(
        &mut self,
        tag_name: &str,
        name_span: Span,
        attributes: Vec<Attribute>,
    ) -> Result<(), Diagnostic> {
        let open_tag_span = Span::new(self.start as u32, self.current as u32);
        self.push_token(
            TokenType::StartTag(StartTag {
                attributes,
                name_span,
                self_closing: false,
            }),
            open_tag_span,
        );
        self.enter_fragment();

        let content_start = self.current;
        let mut content_end = self.current;
        let mut end_tag_start = self.current;
        let mut end_name_span = SPAN;
        let mut found_end_tag = false;

        while !self.is_at_end() {
            let ch = self.advance();
            if ch != '<' {
                continue;
            }

            let candidate_start = self.prev;
            if !self.match_char('/') {
                continue;
            }

            let close_name_start = self.current;
            if self.identifier() != tag_name {
                continue;
            }

            content_end = candidate_start;
            end_tag_start = candidate_start;
            end_name_span = self.span(close_name_start, self.current);
            found_end_tag = true;
            break;
        }

        if found_end_tag {
            if content_start < content_end {
                self.push_token(TokenType::Text, self.span(content_start, content_end));
            }

            self.skip_whitespace();
            if !self.match_char('>') {
                return Err(Diagnostic::unexpected_token(Span::new(
                    end_tag_start as u32,
                    self.current as u32,
                )));
            }

            self.push_token(
                TokenType::EndTag(token::EndTag {
                    name_span: end_name_span,
                }),
                Span::new(end_tag_start as u32, self.current as u32),
            );
            self.leave_fragment();
            return Ok(());
        }

        self.recover(Diagnostic::unexpected_end_of_file(Span::new(
            content_start as u32,
            self.current as u32,
        )));
        if content_start < self.current {
            self.push_token(TokenType::Text, self.span(content_start, self.current));
        }

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
        let start = self.current;

        match self.scan_js_pattern(JsScanTerminator::SvelteBrace)?.end {
            Some(end) => Ok(self.trimmed_span(start, end)),
            None => {
                self.recover(Diagnostic::unexpected_end_of_file(Span::new(
                    start as u32,
                    self.current as u32,
                )));
                Ok(self.trimmed_span(start, self.current))
            }
        }
    }

    fn skip_js_string(&mut self, quote: char) -> Result<(), Diagnostic> {
        let start = self.current;
        while self.peek() != Some(quote) && !self.is_at_end() {
            if self.peek() == Some('\\') {
                self.advance();
                if !self.is_at_end() {
                    self.advance();
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

    fn skip_js_regex(&mut self) -> Result<(), Diagnostic> {
        let start = self.prev;
        let mut in_class = false;

        while !self.is_at_end() {
            let ch = self.advance();
            match ch {
                '\\' => {
                    if !self.is_at_end() {
                        self.advance();
                    }
                }
                '[' if !in_class => in_class = true,
                ']' if in_class => in_class = false,
                '/' if !in_class => {
                    while self.peek().is_some_and(|c| c.is_ascii_alphabetic()) {
                        self.advance();
                    }
                    return Ok(());
                }
                '\n' | '\r' => break,
                _ => {}
            }
        }

        self.recover(Diagnostic::unexpected_end_of_file(Span::new(
            start as u32,
            self.current as u32,
        )));
        Ok(())
    }

    fn skip_js_line_comment(&mut self) {
        while let Some(ch) = self.peek() {
            if ch == '\n' || ch == '\r' {
                break;
            }
            self.advance();
        }
    }

    fn skip_js_block_comment(&mut self) {
        let start = self.current.saturating_sub(2);

        while !self.is_at_end() {
            if self.peek() == Some('*') && self.peek_next() == Some('/') {
                self.advance();
                self.advance();
                return;
            }
            self.advance();
        }

        self.recover(Diagnostic::unexpected_end_of_file(Span::new(
            start as u32,
            self.current as u32,
        )));
    }

    fn skip_js_template(&mut self) -> Result<(), Diagnostic> {
        let start = self.current.saturating_sub(1);

        while !self.is_at_end() {
            match self.peek() {
                Some('`') => {
                    self.advance();
                    return Ok(());
                }
                Some('\\') => {
                    self.advance();
                    if !self.is_at_end() {
                        self.advance();
                    }
                }
                Some('$') if self.peek_next() == Some('{') => {
                    self.advance();
                    self.advance();
                    if self
                        .scan_js_pattern(JsScanTerminator::TemplateExpression)?
                        .end
                        .is_none()
                    {
                        self.recover(Diagnostic::unexpected_end_of_file(Span::new(
                            start as u32,
                            self.current as u32,
                        )));
                        return Ok(());
                    }
                }
                Some(_) => {
                    self.advance();
                }
                None => break,
            }
        }

        self.recover(Diagnostic::unexpected_end_of_file(Span::new(
            start as u32,
            self.current as u32,
        )));
        Ok(())
    }

    fn scan_js_identifier(&mut self) -> &'a str {
        let start = self.current;
        while self
            .peek()
            .is_some_and(|c| c.is_alphanumeric() || c == '_' || c == '$')
        {
            self.advance();
        }
        self.slice_source(start, self.current)
    }

    fn scan_js_number(&mut self) {
        while self.peek().is_some_and(|c| {
            c.is_ascii_alphanumeric() || matches!(c, '_' | '.' | 'x' | 'X' | 'o' | 'O' | 'b' | 'B')
        }) {
            self.advance();
        }
    }

    fn regex_allowed(state: JsExprState) -> bool {
        matches!(state, JsExprState::ExpectOperand)
    }

    fn keyword_expects_operand(keyword: &str) -> bool {
        matches!(
            keyword,
            "return"
                | "throw"
                | "case"
                | "delete"
                | "void"
                | "typeof"
                | "instanceof"
                | "in"
                | "of"
                | "new"
                | "yield"
                | "await"
        )
    }

    fn scan_js_pattern(
        &mut self,
        terminator: JsScanTerminator,
    ) -> Result<JsScanResult, Diagnostic> {
        let mut paren_depth = if matches!(terminator, JsScanTerminator::MatchingParen) {
            1
        } else {
            0
        };
        let mut bracket_depth = 0u32;
        let mut brace_depth = 0u32;
        let mut state = JsExprState::ExpectOperand;
        let mut result = JsScanResult {
            end: None,
            await_clause: None,
        };

        while !self.is_at_end() {
            let ch = self.peek().expect("loop condition guarantees not at end");
            match ch {
                '\'' | '"' => {
                    self.advance();
                    self.skip_js_string(ch)?;
                    state = JsExprState::ExpectOperator;
                }
                '`' => {
                    self.advance();
                    self.skip_js_template()?;
                    state = JsExprState::ExpectOperator;
                }
                '/' if self.peek_next() == Some('/') => {
                    self.advance();
                    self.advance();
                    self.skip_js_line_comment();
                }
                '/' if self.peek_next() == Some('*') => {
                    self.advance();
                    self.advance();
                    self.skip_js_block_comment();
                }
                '/' if Self::regex_allowed(state) => {
                    self.advance();
                    self.skip_js_regex()?;
                    state = JsExprState::ExpectOperator;
                }
                '(' => {
                    if matches!(terminator, JsScanTerminator::EachContext)
                        && paren_depth == 0
                        && bracket_depth == 0
                        && brace_depth == 0
                    {
                        self.advance();
                        result.end = Some(self.prev);
                        return Ok(result);
                    }

                    self.advance();
                    paren_depth += 1;
                    state = JsExprState::ExpectOperand;
                }
                ')' => {
                    self.advance();
                    if paren_depth > 0 {
                        paren_depth -= 1;
                        if paren_depth == 0 && matches!(terminator, JsScanTerminator::MatchingParen)
                        {
                            result.end = Some(self.prev);
                            return Ok(result);
                        }
                    }
                    state = JsExprState::ExpectOperator;
                }
                '[' => {
                    self.advance();
                    bracket_depth += 1;
                    state = JsExprState::ExpectOperand;
                }
                ']' => {
                    self.advance();
                    bracket_depth = bracket_depth.saturating_sub(1);
                    state = JsExprState::ExpectOperator;
                }
                '{' => {
                    self.advance();
                    brace_depth += 1;
                    state = JsExprState::ExpectOperand;
                }
                ',' if matches!(terminator, JsScanTerminator::EachContext)
                    && paren_depth == 0
                    && bracket_depth == 0
                    && brace_depth == 0 =>
                {
                    self.advance();
                    result.end = Some(self.prev);
                    return Ok(result);
                }
                '}' if paren_depth == 0 && bracket_depth == 0 && brace_depth == 0 => {
                    let end = self.current;
                    self.advance();
                    if matches!(
                        terminator,
                        JsScanTerminator::SvelteBrace | JsScanTerminator::TemplateExpression
                    ) {
                        result.end = Some(end);
                        return Ok(result);
                    }
                    if matches!(
                        terminator,
                        JsScanTerminator::EachContext | JsScanTerminator::AwaitBinding
                    ) {
                        result.end = Some(end);
                        return Ok(result);
                    }
                    state = JsExprState::ExpectOperator;
                }
                '}' => {
                    self.advance();
                    brace_depth = brace_depth.saturating_sub(1);
                    state = JsExprState::ExpectOperator;
                }
                c if c.is_whitespace() => {
                    self.advance();
                }
                c if c.is_ascii_digit() => {
                    self.scan_js_number();
                    state = JsExprState::ExpectOperator;
                }
                c if c.is_alphabetic() || c == '_' || c == '$' => {
                    let ident = self.scan_js_identifier();
                    state = if Self::keyword_expects_operand(ident) {
                        JsExprState::ExpectOperand
                    } else {
                        JsExprState::ExpectOperator
                    };
                }
                '+' | '-' => {
                    self.advance();
                    if self.peek() == Some(ch) {
                        self.advance();
                        state = JsExprState::ExpectOperator;
                    } else {
                        state = JsExprState::ExpectOperand;
                    }
                }
                '.' => {
                    self.advance();
                    state = JsExprState::ExpectOperand;
                }
                ',' | ':' | ';' | '?' | '=' | '!' | '~' | '*' | '%' | '^' | '&' | '|' | '<'
                | '>' => {
                    self.advance();
                    state = JsExprState::ExpectOperand;
                }
                _ => {
                    self.advance();
                    state = JsExprState::ExpectOperator;
                }
            }
        }

        Ok(result)
    }

    fn scan_await_expression(&mut self) -> Result<JsScanResult, Diagnostic> {
        let mut paren_depth = 0u32;
        let mut bracket_depth = 0u32;
        let mut brace_depth = 0u32;
        let mut state = JsExprState::ExpectOperand;
        let mut result = JsScanResult {
            end: None,
            await_clause: None,
        };

        while !self.is_at_end() {
            let ch = self.peek().expect("loop condition guarantees not at end");
            match ch {
                '\'' | '"' => {
                    self.advance();
                    self.skip_js_string(ch)?;
                    state = JsExprState::ExpectOperator;
                }
                '`' => {
                    self.advance();
                    self.skip_js_template()?;
                    state = JsExprState::ExpectOperator;
                }
                '/' if self.peek_next() == Some('/') => {
                    self.advance();
                    self.advance();
                    self.skip_js_line_comment();
                }
                '/' if self.peek_next() == Some('*') => {
                    self.advance();
                    self.advance();
                    self.skip_js_block_comment();
                }
                '/' if Self::regex_allowed(state) => {
                    self.advance();
                    self.skip_js_regex()?;
                    state = JsExprState::ExpectOperator;
                }
                '(' => {
                    self.advance();
                    paren_depth += 1;
                    state = JsExprState::ExpectOperand;
                }
                ')' => {
                    self.advance();
                    paren_depth = paren_depth.saturating_sub(1);
                    state = JsExprState::ExpectOperator;
                }
                '[' => {
                    self.advance();
                    bracket_depth += 1;
                    state = JsExprState::ExpectOperand;
                }
                ']' => {
                    self.advance();
                    bracket_depth = bracket_depth.saturating_sub(1);
                    state = JsExprState::ExpectOperator;
                }
                '{' => {
                    self.advance();
                    brace_depth += 1;
                    state = JsExprState::ExpectOperand;
                }
                '}' if paren_depth == 0 && bracket_depth == 0 && brace_depth == 0 => {
                    result.end = Some(self.current);
                    self.advance();
                    return Ok(result);
                }
                '}' => {
                    self.advance();
                    brace_depth = brace_depth.saturating_sub(1);
                    state = JsExprState::ExpectOperator;
                }
                c if c.is_whitespace()
                    && paren_depth == 0
                    && bracket_depth == 0
                    && brace_depth == 0 =>
                {
                    let ws_start = self.current;
                    self.skip_whitespace();
                    if self.is_at_end() {
                        break;
                    }

                    let ident = self.scan_js_identifier();
                    if !ident.is_empty()
                        && (ident == "then" || ident == "catch")
                        && self
                            .peek()
                            .is_some_and(|c| c.is_ascii_whitespace() || c == '}')
                    {
                        result.end = Some(ws_start);
                        result.await_clause = Some(if ident == "then" { "then" } else { "catch" });
                        return Ok(result);
                    }

                    if !ident.is_empty() {
                        state = if Self::keyword_expects_operand(ident) {
                            JsExprState::ExpectOperand
                        } else {
                            JsExprState::ExpectOperator
                        };
                        continue;
                    }
                }
                c if c.is_whitespace() => {
                    self.advance();
                }
                c if c.is_ascii_digit() => {
                    self.scan_js_number();
                    state = JsExprState::ExpectOperator;
                }
                c if c.is_alphabetic() || c == '_' || c == '$' => {
                    let ident = self.scan_js_identifier();
                    state = if Self::keyword_expects_operand(ident) {
                        JsExprState::ExpectOperand
                    } else {
                        JsExprState::ExpectOperator
                    };
                }
                '+' | '-' => {
                    self.advance();
                    if self.peek() == Some(ch) {
                        self.advance();
                        state = JsExprState::ExpectOperator;
                    } else {
                        state = JsExprState::ExpectOperand;
                    }
                }
                '.' => {
                    self.advance();
                    state = JsExprState::ExpectOperand;
                }
                ',' | ':' | ';' | '?' | '=' | '!' | '~' | '*' | '%' | '^' | '&' | '|' | '<'
                | '>' => {
                    self.advance();
                    state = JsExprState::ExpectOperand;
                }
                _ => {
                    self.advance();
                    state = JsExprState::ExpectOperator;
                }
            }
        }

        Ok(result)
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
                self.enter_fragment();

                Ok(())
            }
            "each" => self.start_each_tag(),
            "snippet" => self.start_snippet_tag(),
            "key" => {
                let expression_span = self.collect_js_expression()?;
                self.add_token(TokenType::StartKeyTag(StartKeyTag { expression_span }));
                self.enter_fragment();
                Ok(())
            }
            "await" => self.start_await_tag(),
            _ => Err(Diagnostic::unexpected_keyword(Span::new(
                start as u32,
                self.current as u32,
            ))),
        }
    }

    fn end_template(&mut self) -> Result<(), Diagnostic> {
        debug_assert_eq!(self.peek(), Some('/'));

        let saved_current = self.current;
        let saved_prev = self.prev;

        self.advance();

        let start = self.current;
        let keyword = self.identifier();

        match keyword {
            "if" => {
                self.skip_whitespace();

                if !self.match_char('}') {
                    self.recover(Diagnostic::unexpected_token(Span::new(
                        start as u32,
                        self.current as u32,
                    )));
                }

                self.add_token(TokenType::EndIfTag);
                self.leave_fragment();

                Ok(())
            }
            "each" => {
                self.skip_whitespace();

                if !self.match_char('}') {
                    self.recover(Diagnostic::unexpected_token(Span::new(
                        start as u32,
                        self.current as u32,
                    )));
                }

                self.add_token(TokenType::EndEachTag);
                self.leave_fragment();

                Ok(())
            }
            "snippet" => {
                self.skip_whitespace();

                if !self.match_char('}') {
                    self.recover(Diagnostic::unexpected_token(Span::new(
                        start as u32,
                        self.current as u32,
                    )));
                }

                self.add_token(TokenType::EndSnippetTag);
                self.leave_fragment();

                Ok(())
            }
            "key" => {
                self.skip_whitespace();

                if !self.match_char('}') {
                    self.recover(Diagnostic::unexpected_token(Span::new(
                        start as u32,
                        self.current as u32,
                    )));
                }

                self.add_token(TokenType::EndKeyTag);
                self.leave_fragment();

                Ok(())
            }
            "await" => {
                self.skip_whitespace();

                if !self.match_char('}') {
                    self.recover(Diagnostic::unexpected_token(Span::new(
                        start as u32,
                        self.current as u32,
                    )));
                }

                self.add_token(TokenType::EndAwaitTag);
                self.leave_fragment();

                Ok(())
            }
            _ => {
                self.current = saved_current;
                self.prev = saved_prev;
                self.interpolation()
            }
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
                        self.recover(Diagnostic::unexpected_token(Span::new(
                            start as u32,
                            self.current as u32,
                        )));
                    }

                    self.add_token(TokenType::ElseTag(token::ElseTag {
                        elseif: false,
                        expression_span: None,
                    }));
                }

                Ok(())
            }
            "then" | "catch" => {
                let clause = if keyword == "then" {
                    token::AwaitClause::Then
                } else {
                    token::AwaitClause::Catch
                };

                self.skip_whitespace();

                let binding_span = if self.peek() == Some('}') {
                    self.advance();
                    None
                } else {
                    Some(self.collect_await_binding()?)
                };

                self.add_token(TokenType::AwaitClauseTag(token::AwaitClauseTag {
                    clause,
                    binding_span,
                }));

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

        let context_deprecated = attributes.iter().any(|item| match item {
            Attribute::HTMLAttribute(attr) => {
                let name = attr.name_span.source_text(self.source);
                name == "context"
                    && matches!(attr.value, AttributeValue::String(span) if span.source_text(self.source) == "module")
            }
            _ => false,
        });

        let is_module = context_deprecated
            || attributes.iter().any(|item| match item {
                Attribute::HTMLAttribute(attr) => {
                    let name = attr.name_span.source_text(self.source);
                    name == "module" && attr.value == AttributeValue::Empty
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
                context_deprecated,
            }));

            return Ok(());
        }

        self.skip_whitespace();

        if !self.match_char('>') {
            return Err(Diagnostic::unexpected_token(Span::new(
                start as u32,
                self.current as u32,
            )));
        }

        self.add_token(TokenType::ScriptTag(ScriptTag {
            content_span: self.span(start, end),
            is_typescript,
            is_module,
            context_deprecated,
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
            return Err(Diagnostic::unexpected_token(Span::new(
                start as u32,
                self.current as u32,
            )));
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
            return Err(Diagnostic::unexpected_token(Span::new(
                start as u32,
                self.current as u32,
            )));
        }

        if !self.match_char('-') {
            return Err(Diagnostic::unexpected_token(Span::new(
                start as u32,
                self.current as u32,
            )));
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

                self.add_token(TokenType::HtmlTag(token::HtmlTagToken { expression_span }));

                Ok(())
            }
            "const" => {
                self.skip_whitespace();
                let expression_span = self.collect_js_expression()?;

                self.add_token(TokenType::ConstTag(token::ConstTagToken {
                    expression_span,
                }));

                Ok(())
            }
            "debug" => {
                self.skip_whitespace();

                let mut identifiers = Vec::new();

                if self.peek() != Some('}') {
                    loop {
                        self.skip_whitespace();
                        let id_start = self.current;
                        let name = self.identifier();
                        if name.is_empty() {
                            return Err(Diagnostic::unexpected_token(Span::new(
                                id_start as u32,
                                self.current as u32,
                            )));
                        }
                        if self.peek() == Some('.') || self.peek() == Some('(') {
                            return Err(Diagnostic::unexpected_token(Span::new(
                                id_start as u32,
                                self.current as u32 + 1,
                            )));
                        }
                        identifiers.push(self.span(id_start, self.current));
                        self.skip_whitespace();
                        if self.peek() == Some(',') {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                }

                if self.peek() == Some('}') {
                    self.advance();
                }

                self.add_token(TokenType::DebugTag(token::DebugTagToken { identifiers }));

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

        let expr_start = self.current;
        let name = self.identifier();

        if name.is_empty() {
            return Err(Diagnostic::unexpected_token(Span::new(
                self.start as u32,
                self.current as u32,
            )));
        }

        if self.peek() == Some('(') {
            self.advance();
            if self
                .scan_js_pattern(JsScanTerminator::MatchingParen)?
                .end
                .is_none()
            {
                return Err(Diagnostic::unexpected_end_of_file(Span::new(
                    expr_start as u32,
                    self.current as u32,
                )));
            }
        }

        let expression_span = self.span(expr_start, self.current);

        self.skip_whitespace();

        if !self.match_char('}') {
            self.recover(Diagnostic::unexpected_token(Span::new(
                self.start as u32,
                self.current as u32,
            )));
        }

        self.add_token(TokenType::StartSnippetTag(token::StartSnippetTag {
            expression_span,
        }));
        self.enter_fragment();

        Ok(())
    }

    fn start_each_tag(&mut self) -> Result<(), Diagnostic> {
        self.skip_whitespace();

        let start_collection_pos = self.current;
        let mut end_collection_pos;
        let mut depth: u32 = 0;
        let mut collection_span: Option<Span> = None;
        let mut item_span: Option<Span> = None;

        let mut last_comma_pos: Option<usize> = None;

        let mut no_as_index_span: Option<Span> = None;

        while !self.is_at_end() {
            let ch = self.peek().expect("loop condition guarantees not at end");
            match ch {
                '\'' | '"' | '`' => {
                    self.advance();
                    self.skip_js_string(ch)?;
                }
                '(' | '[' | '{' => {
                    depth += 1;
                    self.advance();
                }
                ')' | ']' => {
                    depth = depth.saturating_sub(1);
                    self.advance();
                }
                '}' if depth > 0 => {
                    depth -= 1;
                    self.advance();
                }
                '}' => {
                    let raw_end = self.current;
                    if let Some(comma_pos) = last_comma_pos {
                        let after_comma = self.slice_source(comma_pos + 1, raw_end);
                        let idx_trimmed = after_comma.trim();
                        if is_js_identifier(idx_trimmed) {
                            let raw_before = self.slice_source(start_collection_pos, comma_pos);
                            let col_end = start_collection_pos + raw_before.trim_end().len();
                            collection_span = Some(self.span(start_collection_pos, col_end));
                            let idx_leading = after_comma.len() - after_comma.trim_start().len();
                            let idx_start = comma_pos + 1 + idx_leading;
                            let idx_end =
                                raw_end - (after_comma.len() - after_comma.trim_end().len());
                            no_as_index_span = Some(self.span(idx_start, idx_end));
                            self.advance();
                            break;
                        }
                    }
                    let raw = self.slice_source(start_collection_pos, raw_end);
                    let trimmed_end = start_collection_pos + raw.trim_end().len();
                    collection_span = Some(self.span(start_collection_pos, trimmed_end));
                    self.advance();
                    break;
                }
                ',' if depth == 0 => {
                    last_comma_pos = Some(self.current);
                    self.advance();
                }
                c if c.is_ascii_whitespace() && depth == 0 => {
                    end_collection_pos = self.current;
                    self.skip_whitespace();
                    let keyword = self.identifier();
                    if keyword == "as" {
                        collection_span = Some(self.span(start_collection_pos, end_collection_pos));
                        self.skip_whitespace();
                        item_span = Some(self.collect_each_context()?);
                        break;
                    }
                }
                _ => {
                    self.advance();
                }
            }
        }

        let Some(collection_span) = collection_span else {
            return Diagnostic::unexpected_token(Span::new(self.start as u32, self.current as u32))
                .as_err();
        };

        let last_char = if item_span.is_some() {
            self.slice_source(self.prev, self.prev + 1)
        } else {
            ""
        };

        let mut index_span = no_as_index_span;
        let mut key_span = None;

        if last_char == "," {
            self.skip_whitespace();
            let idx_start = self.current;
            let idx_name = self.identifier();
            if idx_name.is_empty() {
                return Diagnostic::unexpected_token(Span::new(
                    self.current as u32,
                    self.current as u32,
                ))
                .as_err();
            }
            index_span = Some(self.span(idx_start, idx_start + idx_name.len()));
            self.skip_whitespace();

            if self.peek() == Some('(') {
                key_span = Some(self.collect_key_expression(false)?);
                self.skip_whitespace();
            }

            if !self.match_char('}') {
                return Diagnostic::unexpected_token(Span::new(
                    self.current as u32,
                    self.current as u32,
                ))
                .as_err();
            }
        } else if last_char == "(" {
            key_span = Some(self.collect_key_expression(true)?);
            self.skip_whitespace();

            if !self.match_char('}') {
                return Diagnostic::unexpected_token(Span::new(
                    self.current as u32,
                    self.current as u32,
                ))
                .as_err();
            }
        }

        self.add_token(TokenType::StartEachTag(StartEachTag {
            collection_span,
            context_span: item_span,
            index_span,
            key_span,
        }));
        self.enter_fragment();

        Ok(())
    }

    fn collect_key_expression(&mut self, open_consumed: bool) -> Result<Span, Diagnostic> {
        if !open_consumed {
            debug_assert_eq!(self.peek(), Some('('));
            self.advance();
        }

        let start = self.current;
        let Some(end) = self.scan_js_pattern(JsScanTerminator::MatchingParen)?.end else {
            return Err(Diagnostic::unexpected_end_of_file(Span::new(
                start as u32,
                self.current as u32,
            )));
        };

        Ok(self.trimmed_span(start, end))
    }

    fn collect_each_context(&mut self) -> Result<Span, Diagnostic> {
        let start = self.current;

        let Some(end) = self.scan_js_pattern(JsScanTerminator::EachContext)?.end else {
            return Err(Diagnostic::unexpected_end_of_file(Span::new(
                start as u32,
                self.current as u32,
            )));
        };

        Ok(self.trimmed_span(start, end))
    }

    fn start_await_tag(&mut self) -> Result<(), Diagnostic> {
        self.skip_whitespace();

        let expr_start = self.current;
        let scan = self.scan_await_expression()?;
        let expression_span = self.trimmed_span(expr_start, scan.end.unwrap_or(self.current));

        match scan.await_clause {
            Some("then") => {
                self.skip_whitespace();
                let binding_span = if self.peek() == Some('}') {
                    self.advance();
                    None
                } else {
                    Some(self.collect_await_binding()?)
                };
                self.add_token(TokenType::StartAwaitTag(token::StartAwaitTag {
                    expression_span,
                    value_span: binding_span,
                    error_span: None,
                    initial_clause: token::AwaitInitialClause::Then,
                }));
                self.enter_fragment();
            }
            Some("catch") => {
                self.skip_whitespace();
                let binding_span = if self.peek() == Some('}') {
                    self.advance();
                    None
                } else {
                    Some(self.collect_await_binding()?)
                };
                self.add_token(TokenType::StartAwaitTag(token::StartAwaitTag {
                    expression_span,
                    value_span: None,
                    error_span: binding_span,
                    initial_clause: token::AwaitInitialClause::Catch,
                }));
                self.enter_fragment();
            }
            _ => {
                self.add_token(TokenType::StartAwaitTag(token::StartAwaitTag {
                    expression_span,
                    value_span: None,
                    error_span: None,
                    initial_clause: token::AwaitInitialClause::Pending,
                }));
                self.enter_fragment();
            }
        }

        Ok(())
    }

    fn collect_await_binding(&mut self) -> Result<Span, Diagnostic> {
        let start = self.current;

        let Some(end) = self.scan_js_pattern(JsScanTerminator::AwaitBinding)?.end else {
            return Err(Diagnostic::unexpected_end_of_file(Span::new(
                start as u32,
                self.current as u32,
            )));
        };

        Ok(self.trimmed_span(start, end))
    }
}

fn is_js_identifier(s: &str) -> bool {
    let mut chars = s.chars();
    match chars.next() {
        None => false,
        Some(c) => {
            (c.is_alphabetic() || c == '_' || c == '$')
                && chars.all(|c| c.is_alphanumeric() || c == '_' || c == '$')
        }
    }
}

#[cfg(test)]
mod tests;
