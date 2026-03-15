pub mod token;

use std::{iter::Peekable, str::Chars, vec};
use token::{
    Attribute, AttributeIdentifierType, AttributeValue, BindDirective, ClassDirective,
    Concatenation, ConcatenationPart, ExpressionTag, HTMLAttribute, JsExpression, ScriptTag,
    StartEachTag, StartIfTag, StartKeyTag, StartTag, Token, TokenType,
};

use svelte_diagnostics::Diagnostic;
use svelte_span::{Span, SPAN};

pub struct Scanner<'a> {
    source: &'a str,
    chars: Peekable<Chars<'a>>,
    tokens: Vec<Token<'a>>,
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

    pub fn scan_tokens(&mut self) -> (Vec<Token<'a>>, Vec<Diagnostic>) {
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
            lexeme: "",
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

    fn add_token(&mut self, token_type: TokenType<'a>) {
        let text = self.slice_source(self.start, self.current);

        self.tokens.push(Token {
            token_type,
            lexeme: text,
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

            if AttributeIdentifierType::is_class_directive(name) {
                AttributeIdentifierType::ClassDirective(value).as_ok()
            } else if AttributeIdentifierType::is_bind_directive(name) {
                AttributeIdentifierType::BindDirective(value).as_ok()
            } else {
                Diagnostic::unknown_directive(Span::new(colon_pos as u32, self.current as u32)).as_err()
            }
        } else if start == self.current {
            AttributeIdentifierType::None.as_ok()
        } else {
            AttributeIdentifierType::HTMLAttribute(self.slice_source(start, self.current)).as_ok()
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

    fn collect_until<F>(&mut self, condition: F) -> Result<&'a str, Diagnostic>
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

        Ok(self.slice_source(start, self.current))
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
        let start = self.current;
        let name = self.identifier();

        if name.is_empty() {
            return Err(Diagnostic::invalid_tag_name(Span::new(start as u32, self.current as u32)));
        }

        let attributes = self.attributes()?;
        let self_closing = self.match_char('/');

        if !self.match_char('>') {
            // Emit partial StartTag with recovery — parser-level will handle auto-close
            self.recover(Diagnostic::unterminated_start_tag(Span::new(
                start as u32,
                self.current as u32,
            )));

            self.add_token(TokenType::StartTag(StartTag {
                attributes,
                name,
                self_closing,
            }));

            return Ok(());
        }

        if name == "script" {
            return self.script_tag(attributes);
        }

        if name == "style" {
            return self.style_tag(attributes);
        }

        self.add_token(TokenType::StartTag(StartTag {
            attributes,
            name,
            self_closing,
        }));

        Ok(())
    }

    fn attributes(&mut self) -> Result<Vec<Attribute<'a>>, Diagnostic> {
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
                self.expression_tag().map(Attribute::ExpressionTag)
            } else {
                let name = match self.attribute_identifier() {
                    Ok(name) => name,
                    Err(d) => {
                        self.recover(d);
                        break;
                    }
                };

                match name {
                    AttributeIdentifierType::HTMLAttribute(name) => self.html_attribute(name),
                    AttributeIdentifierType::ClassDirective(value) => {
                        self.class_directive(value)
                    }
                    AttributeIdentifierType::BindDirective(value) => self.bind_directive(value),
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

    fn html_attribute(&mut self, name: &'a str) -> Result<Attribute<'a>, Diagnostic> {
        let mut value: AttributeValue = AttributeValue::Empty;

        if self.match_char('=') {
            value = self.attribute_value()?;
        }

        Ok(Attribute::HTMLAttribute(HTMLAttribute { name, value }))
    }

    fn class_directive(&mut self, name: &'a str) -> Result<Attribute<'a>, Diagnostic> {
        if self.match_char('=') {
            let res = self.expression_tag()?;

            return Ok(Attribute::ClassDirective(ClassDirective {
                expression: res.expression,
                name,
                shorthand: false,
            }));
        }

        Ok(Attribute::ClassDirective(ClassDirective {
            name,
            expression: JsExpression {
                span: SPAN,
                value: name,
            },
            shorthand: true,
        }))
    }

    fn bind_directive(&mut self, name: &'a str) -> Result<Attribute<'a>, Diagnostic> {
        if self.match_char('=') {
            let res = self.expression_tag()?;

            return Ok(Attribute::BindDirective(BindDirective {
                expression: res.expression,
                name,
                shorthand: false,
            }));
        }

        Ok(Attribute::BindDirective(BindDirective {
            name,
            expression: JsExpression {
                span: SPAN,
                value: name,
            },
            shorthand: true,
        }))
    }

    fn attribute_value(&mut self) -> Result<AttributeValue<'a>, Diagnostic> {
        let peeked = self.peek();

        if self.peek() == Some('{') {
            return self.expression_tag().map(AttributeValue::ExpressionTag);
        }

        if let Some(quote) = peeked.filter(|c| *c == '"' || *c == '\'') {
            return self.attribute_concatenation_or_string(quote);
        }

        let value = self.collect_until(|char| match char {
            '"' | '\'' | '>' | '<' | '`' => true,
            char => char.is_whitespace(),
        })?;

        Ok(AttributeValue::String(value))
    }

    fn expression_tag(&mut self) -> Result<ExpressionTag<'a>, Diagnostic> {
        debug_assert_eq!(self.peek(), Some('{'));

        let start = self.start;
        self.advance();

        let expression = self.collect_js_expression()?;

        Ok(ExpressionTag {
            expression,
            span: Span::new(start as u32, self.current as u32),
        })
    }

    fn attribute_concatenation_or_string(
        &mut self,
        quote: char,
    ) -> Result<AttributeValue<'a>, Diagnostic> {
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
                let part = self.slice_source(current_pos, self.current);

                if !part.is_empty() {
                    parts.push(ConcatenationPart::String(part));
                }

                let expression_tag = self.expression_tag()?;

                parts.push(ConcatenationPart::Expression(expression_tag));
                current_pos = self.current;

                continue;
            }

            self.advance();
        }

        let last_part = self.slice_source(current_pos, self.current);

        // consume last quote (or recover at EOF)
        if !self.is_at_end() {
            self.advance();
        } else {
            self.recover(Diagnostic::unexpected_end_of_file(Span::new(
                start as u32,
                self.current as u32,
            )));
        }

        if has_expression && !last_part.is_empty() {
            parts.push(ConcatenationPart::String(last_part));
        }

        if !has_expression && parts.is_empty() {
            return Ok(AttributeValue::String(last_part));
        }

        Ok(AttributeValue::Concatenation(Concatenation {
            start,
            end: self.current,
            parts,
        }))
    }

    fn end_tag(&mut self) -> Result<(), Diagnostic> {
        self.advance();

        let start = self.current;
        let name = self.identifier();

        if name.is_empty() {
            return Err(Diagnostic::invalid_tag_name(Span::new(start as u32, self.current as u32)));
        }

        self.skip_whitespace();

        if !self.match_char('>') {
            self.recover(Diagnostic::unexpected_token(Span::new(start as u32, self.current as u32)));
        }

        self.add_token(TokenType::EndTag(token::EndTag { name }));

        Ok(())
    }

    fn text(&mut self) {
        while self.peek() != Some('<') && self.peek() != Some('{') && !self.is_at_end() {
            self.advance();
        }

        self.add_token(TokenType::Text);
    }

    fn interpolation(&mut self) -> Result<(), Diagnostic> {
        let expression = self.collect_js_expression()?;

        self.add_token(TokenType::Interpolation(ExpressionTag {
            expression,
            span: Span::new(self.start as u32, self.current as u32),
        }));

        Ok(())
    }

    fn collect_js_expression(&mut self) -> Result<JsExpression<'a>, Diagnostic> {
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

                let value = raw.trim();
                let trim_start = raw.len() - raw.trim_start().len();
                let trim_end = raw.len() - raw.trim_end().len();
                let span_start = start + trim_start;
                let span_end = self.prev - trim_end;

                return Ok(JsExpression {
                    span: Span::new(span_start as u32, span_end as u32),
                    value,
                });
            }
        }

        // EOF — return partial expression with recovery
        self.recover(Diagnostic::unexpected_end_of_file(Span::new(
            start as u32,
            self.current as u32,
        )));

        let raw = self.slice_source(start, self.current);
        let value = raw.trim();
        let trim_start = raw.len() - raw.trim_start().len();
        let trim_end = raw.len() - raw.trim_end().len();
        let span_start = start + trim_start;
        let span_end = if trim_end <= self.current - start {
            self.current - trim_end
        } else {
            start
        };

        Ok(JsExpression {
            span: Span::new(span_start as u32, span_end as u32),
            value,
        })
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
                let expression = self.collect_js_expression()?;

                self.add_token(TokenType::StartIfTag(StartIfTag { expression }));

                Ok(())
            }
            "each" => self.start_each_tag(),
            "snippet" => self.start_snippet_tag(),
            "key" => {
                let expression = self.collect_js_expression()?;
                self.add_token(TokenType::StartKeyTag(StartKeyTag { expression }));
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

                    let expression = self.collect_js_expression()?;

                    self.add_token(TokenType::ElseTag(token::ElseTag {
                        elseif: true,
                        expression: Some(expression),
                    }));
                } else {
                    if !self.match_char('}') {
                        self.recover(Diagnostic::unexpected_token(Span::new(start as u32, self.current as u32)));
                    }

                    self.add_token(TokenType::ElseTag(token::ElseTag {
                        elseif: false,
                        expression: None,
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

    fn script_tag(&mut self, attributes: Vec<Attribute<'a>>) -> Result<(), Diagnostic> {
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

        if self.is_at_end() {
            self.recover(Diagnostic::unexpected_end_of_file(Span::new(
                start as u32,
                self.current as u32,
            )));

            self.add_token(TokenType::ScriptTag(ScriptTag {
                source: self.slice_source(start, self.current),
                attributes,
            }));

            return Ok(());
        }

        self.skip_whitespace();

        if !self.match_char('>') {
            return Err(Diagnostic::unexpected_token(Span::new(start as u32, self.current as u32)));
        }

        self.add_token(TokenType::ScriptTag(ScriptTag {
            source: self.slice_source(start, end),
            attributes,
        }));

        Ok(())
    }

    fn style_tag(&mut self, attributes: Vec<Attribute<'a>>) -> Result<(), Diagnostic> {
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
                source: self.slice_source(start, self.current),
                attributes,
            }));

            return Ok(());
        }

        self.skip_whitespace();

        if !self.match_char('>') {
            return Err(Diagnostic::unexpected_token(Span::new(start as u32, self.current as u32)));
        }

        self.add_token(TokenType::StyleTag(token::StyleTag {
            source: self.slice_source(start, end),
            attributes,
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
                let expression = self.collect_js_expression()?;

                self.add_token(TokenType::RenderTag(token::RenderTagToken {
                    expression,
                }));

                Ok(())
            }
            "html" => {
                self.skip_whitespace();
                let expression = self.collect_js_expression()?;

                self.add_token(TokenType::HtmlTag(token::HtmlTagToken {
                    expression,
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

        let name = self.identifier();

        if name.is_empty() {
            return Err(Diagnostic::unexpected_token(Span::new(
                self.start as u32,
                self.current as u32,
            )));
        }

        let params = if self.peek() == Some('(') {
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
                Some(JsExpression {
                    span: Span::new(span_start as u32, span_end as u32),
                    value,
                })
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
            name,
            params,
        }));

        Ok(())
    }

    fn start_each_tag(&mut self) -> Result<(), Diagnostic> {
        let mut collection = None;
        let mut item = None;
        let mut end_collection_pos = 0;

        self.skip_whitespace();

        let start_collection_pos = self.current;

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

            collection = self.source[start_collection_pos..end_collection_pos].into();

            self.skip_whitespace();

            item = self.collect_each_context()?.into();

            break;
        }

        let Some(collection) = collection else {
            return Diagnostic::unexpected_token(Span::new(self.start as u32, self.current as u32)).as_err();
        };

        let Some(item) = item else {
            return Diagnostic::unexpected_token(Span::new(self.start as u32, self.current as u32)).as_err();
        };

        let last_char = self.slice_source(self.prev, self.prev + 1);

        // Parse optional index: `, i`
        let mut index = None;
        let mut key = None;

        if last_char == "," {
            self.skip_whitespace();
            let idx_start = self.current;
            let idx_name = self.identifier();
            if idx_name.is_empty() {
                return Diagnostic::unexpected_token(Span::new(self.current as u32, self.current as u32)).as_err();
            }
            index = Some(JsExpression {
                span: Span::new(idx_start as u32, (idx_start + idx_name.len()) as u32),
                value: idx_name,
            });
            self.skip_whitespace();

            // After index, check for key `(expr)` or closing `}`
            if self.peek() == Some('(') {
                key = Some(self.collect_key_expression(false)?);
                self.skip_whitespace();
            }

            if !self.match_char('}') {
                return Diagnostic::unexpected_token(Span::new(self.current as u32, self.current as u32)).as_err();
            }
        } else if last_char == "(" {
            // Key expression directly after item (no index), `(` already consumed
            key = Some(self.collect_key_expression(true)?);
            self.skip_whitespace();

            if !self.match_char('}') {
                return Diagnostic::unexpected_token(Span::new(self.current as u32, self.current as u32)).as_err();
            }
        }
        // else: `}` — no index, no key

        self.add_token(TokenType::StartEachTag(StartEachTag {
            collection: JsExpression {
                span: Span::new(start_collection_pos as u32, end_collection_pos as u32),
                value: collection,
            },
            item,
            key,
            index,
        }));

        Ok(())
    }

    /// Collect key expression in `{#each ... (key)}`.
    /// If `open_consumed` is true, `(` was already consumed by the caller.
    /// If false, `(` is expected at the current peek position and will be consumed.
    fn collect_key_expression(&mut self, open_consumed: bool) -> Result<JsExpression<'a>, Diagnostic> {
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
        let value = raw.trim();
        let trim_start = raw.len() - raw.trim_start().len();
        let trim_end = raw.len() - raw.trim_end().len();
        let span_start = start + trim_start;
        let span_end = end - trim_end;

        Ok(JsExpression {
            span: Span::new(span_start as u32, span_end as u32),
            value,
        })
    }

    /// Collect item expression in each-block context.
    /// Stops on `,` or `}` at depth 0. Tracks `{}` and `[]` nesting for destructuring.
    fn collect_each_context(&mut self) -> Result<JsExpression<'a>, Diagnostic> {
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
                    let value = raw.trim();
                    let trim_start = raw.len() - raw.trim_start().len();
                    let trim_end = raw.len() - raw.trim_end().len();
                    let span_start = start + trim_start;
                    let span_end = self.prev - trim_end;

                    return Ok(JsExpression {
                        span: Span::new(span_start as u32, span_end as u32),
                        value,
                    });
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
        let mut scanner = Scanner::new("<div>kek {name} hello</div>");

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
        let mut scanner = Scanner::new("<input/>");

        let tokens = scanner.scan_tokens().0;

        assert_start_tag(&tokens[0], "input", vec![], true);
        assert!(tokens[1].token_type == TokenType::EOF);
    }

    #[test]
    fn start_tag_attributes() {
        let mut scanner = Scanner::new(
            "<div valid id=123 touched some=true disabled value=\"333\" class='never' >",
        );

        let tokens = scanner.scan_tokens().0;

        assert_start_tag(
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
        let mut scanner = Scanner::new("<!-- \nsome comment\n -->");
        let tokens = scanner.scan_tokens().0;

        assert!(tokens[0].token_type == TokenType::Comment);
        assert_eq!(tokens[0].lexeme, "<!-- \nsome comment\n -->");
        assert!(tokens[1].token_type == TokenType::EOF);
    }

    #[test]
    fn each_block() {
        let mut scanner = Scanner::new("{#each [1,2,3] as { value, flag }}");
        let tokens = scanner.scan_tokens().0;

        assert_start_each_tag(&tokens[0], "[1,2,3]", "{ value, flag }");
        assert!(tokens[1].token_type == TokenType::EOF);
    }

    fn assert_start_tag(
        token: &Token,
        expected_name: &str,
        expected_attributes: Vec<(&str, &str)>,
        expected_self_closing: bool,
    ) {
        let start_tag = match &token.token_type {
            TokenType::StartTag(t) => t,
            _ => panic!("Expected token.type = StartTag."),
        };

        assert_eq!(start_tag.name, expected_name);
        assert_eq!(start_tag.self_closing, expected_self_closing);
        assert_attributes(&start_tag.attributes, expected_attributes);
    }

    fn assert_attributes(
        actual_attributes: &[Attribute],
        expected_attributes: Vec<(&str, &str)>,
    ) {
        assert_eq!(actual_attributes.len(), expected_attributes.len());

        for (index, (expected_name, expected_value)) in expected_attributes.iter().enumerate() {
            let attribute = &actual_attributes[index];

            let name = match attribute {
                Attribute::HTMLAttribute(value) => value.name,
                Attribute::ExpressionTag(_) => "$expression",
                Attribute::ClassDirective(_) => "$classDirective",
                Attribute::BindDirective(_) => "$bindDirective",
            };

            let value: AttributeValue = match attribute {
                Attribute::HTMLAttribute(value) => value.value.clone(),
                Attribute::ExpressionTag(value) => AttributeValue::String(value.expression.value),
                Attribute::ClassDirective(cd) => AttributeValue::String(cd.expression.value),
                Attribute::BindDirective(bd) => AttributeValue::String(bd.expression.value),
            };

            assert_eq!(name, *expected_name);
            assert_eq!(value.to_string(), expected_value.to_string());
        }
    }

    fn assert_start_each_tag(token: &Token, expected_collection: &str, expected_item: &str) {
        let tag = match &token.token_type {
            TokenType::StartEachTag(t) => t,
            _ => panic!("Expected token.type = StartEachTag"),
        };

        assert_eq!(tag.collection.value, expected_collection);
        assert_eq!(tag.item.value, expected_item);
        assert!(tag.index.is_none(), "expected no index");
    }

    fn assert_start_each_tag_with_index(
        token: &Token,
        expected_collection: &str,
        expected_item: &str,
        expected_index: &str,
    ) {
        let tag = match &token.token_type {
            TokenType::StartEachTag(t) => t,
            _ => panic!("Expected token.type = StartEachTag"),
        };

        assert_eq!(tag.collection.value, expected_collection);
        assert_eq!(tag.item.value, expected_item);
        let index = tag.index.as_ref().expect("expected index");
        assert_eq!(index.value, expected_index);
    }

    #[test]
    fn each_block_with_index() {
        let mut scanner = Scanner::new("{#each items as item, i}");
        let tokens = scanner.scan_tokens().0;

        assert_start_each_tag_with_index(&tokens[0], "items", "item", "i");
        assert!(tokens[1].token_type == TokenType::EOF);
    }

    #[test]
    fn each_block_destructured_with_index() {
        let mut scanner = Scanner::new("{#each items as { value, flag }, idx}");
        let tokens = scanner.scan_tokens().0;

        assert_start_each_tag_with_index(&tokens[0], "items", "{ value, flag }", "idx");
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
        let mut scanner = Scanner::new("<style>.foo { color: red; }</style>");
        let tokens = scanner.scan_tokens().0;
        assert!(matches!(tokens[0].token_type, TokenType::StyleTag(_)));
        if let TokenType::StyleTag(ref st) = tokens[0].token_type {
            assert_eq!(st.source, ".foo { color: red; }");
        }
        assert!(tokens[1].token_type == TokenType::EOF);
    }

    #[test]
    fn style_tag_with_angle_brackets() {
        let mut scanner = Scanner::new("<style>a > b { color: red; }</style>");
        let tokens = scanner.scan_tokens().0;
        assert!(matches!(tokens[0].token_type, TokenType::StyleTag(_)));
        if let TokenType::StyleTag(ref st) = tokens[0].token_type {
            assert_eq!(st.source, "a > b { color: red; }");
        }
    }

    // --- Each block key tests (Bug #3) ---

    fn assert_start_each_tag_with_key(
        token: &Token,
        expected_collection: &str,
        expected_item: &str,
        expected_key: &str,
    ) {
        let tag = match &token.token_type {
            TokenType::StartEachTag(t) => t,
            _ => panic!("Expected token.type = StartEachTag"),
        };

        assert_eq!(tag.collection.value, expected_collection);
        assert_eq!(tag.item.value, expected_item);
        let key = tag.key.as_ref().expect("expected key");
        assert_eq!(key.value, expected_key);
    }

    #[test]
    fn each_block_with_key() {
        let mut scanner = Scanner::new("{#each items as item (item.id)}");
        let tokens = scanner.scan_tokens().0;
        assert_start_each_tag_with_key(&tokens[0], "items", "item", "item.id");
        assert!(tokens[1].token_type == TokenType::EOF);
    }

    #[test]
    fn each_block_with_index_and_key() {
        let mut scanner = Scanner::new("{#each items as item, i (item.id)}");
        let tokens = scanner.scan_tokens().0;

        let tag = match &tokens[0].token_type {
            TokenType::StartEachTag(t) => t,
            _ => panic!("Expected StartEachTag"),
        };
        assert_eq!(tag.collection.value, "items");
        assert_eq!(tag.item.value, "item");
        assert_eq!(tag.index.as_ref().unwrap().value, "i");
        assert_eq!(tag.key.as_ref().unwrap().value, "item.id");
    }

    #[test]
    fn each_block_destructured_with_key() {
        let mut scanner = Scanner::new("{#each items as {name} (name)}");
        let tokens = scanner.scan_tokens().0;
        assert_start_each_tag_with_key(&tokens[0], "items", "{name}", "name");
    }

    // --- Directive tests ---

    #[test]
    fn class_directive_with_expression() {
        let mut scanner = Scanner::new("<div class:active={isActive}>");
        let tokens = scanner.scan_tokens().0;
        assert_start_tag(
            &tokens[0],
            "div",
            vec![("$classDirective", "isActive")],
            false,
        );
    }

    #[test]
    fn class_directive_shorthand() {
        let mut scanner = Scanner::new("<div class:active>");
        let tokens = scanner.scan_tokens().0;
        assert_start_tag(
            &tokens[0],
            "div",
            vec![("$classDirective", "active")],
            false,
        );
    }

    #[test]
    fn bind_directive_with_expression() {
        let mut scanner = Scanner::new("<input bind:value={name}>");
        let tokens = scanner.scan_tokens().0;
        assert_start_tag(
            &tokens[0],
            "input",
            vec![("$bindDirective", "name")],
            false,
        );
    }

    #[test]
    fn bind_directive_shorthand() {
        let mut scanner = Scanner::new("<input bind:value>");
        let tokens = scanner.scan_tokens().0;
        assert_start_tag(
            &tokens[0],
            "input",
            vec![("$bindDirective", "value")],
            false,
        );
    }

    // --- Attribute concatenation tests ---

    #[test]
    fn attribute_concatenation() {
        let mut scanner = Scanner::new(r#"<div title="hello {name} world">"#);
        let tokens = scanner.scan_tokens().0;
        assert!(matches!(tokens[0].token_type, TokenType::StartTag(_)));
        if let TokenType::StartTag(ref st) = tokens[0].token_type {
            assert_eq!(st.name, "div");
            assert_eq!(st.attributes.len(), 1);
            if let Attribute::HTMLAttribute(ref attr) = st.attributes[0] {
                assert_eq!(attr.name, "title");
                assert!(matches!(attr.value, AttributeValue::Concatenation(_)));
            } else {
                panic!("Expected HTMLAttribute");
            }
        }
    }

    // --- Spread/shorthand attribute tests ---

    #[test]
    fn spread_attribute() {
        let mut scanner = Scanner::new("<div {...props}>");
        let tokens = scanner.scan_tokens().0;
        assert!(matches!(tokens[0].token_type, TokenType::StartTag(_)));
        if let TokenType::StartTag(ref st) = tokens[0].token_type {
            assert_eq!(st.attributes.len(), 1);
            if let Attribute::ExpressionTag(ref et) = st.attributes[0] {
                assert_eq!(et.expression.value, "...props");
            } else {
                panic!("Expected ExpressionTag for spread");
            }
        }
    }

    #[test]
    fn shorthand_attribute() {
        let mut scanner = Scanner::new("<div {value}>");
        let tokens = scanner.scan_tokens().0;
        assert!(matches!(tokens[0].token_type, TokenType::StartTag(_)));
        if let TokenType::StartTag(ref st) = tokens[0].token_type {
            assert_eq!(st.attributes.len(), 1);
            if let Attribute::ExpressionTag(ref et) = st.attributes[0] {
                assert_eq!(et.expression.value, "value");
            } else {
                panic!("Expected ExpressionTag for shorthand");
            }
        }
    }

    // --- Snippet/render token tests ---

    #[test]
    fn snippet_tag_tokens() {
        let mut scanner = Scanner::new("{#snippet foo(a, b)}content{/snippet}");
        let tokens = scanner.scan_tokens().0;
        assert!(matches!(tokens[0].token_type, TokenType::StartSnippetTag(_)));
        if let TokenType::StartSnippetTag(ref st) = tokens[0].token_type {
            assert_eq!(st.name, "foo");
            assert_eq!(st.params.as_ref().unwrap().value, "a, b");
        }
        assert!(tokens[1].token_type == TokenType::Text);
        assert!(tokens[2].token_type == TokenType::EndSnippetTag);
    }

    #[test]
    fn render_tag_tokens() {
        let mut scanner = Scanner::new("{@render foo(x, y)}");
        let tokens = scanner.scan_tokens().0;
        assert!(matches!(tokens[0].token_type, TokenType::RenderTag(_)));
        if let TokenType::RenderTag(ref rt) = tokens[0].token_type {
            assert_eq!(rt.expression.value, "foo(x, y)");
        }
    }

    #[test]
    fn html_tag_tokens() {
        let mut scanner = Scanner::new("{@html content}");
        let tokens = scanner.scan_tokens().0;
        assert!(matches!(tokens[0].token_type, TokenType::HtmlTag(_)));
        if let TokenType::HtmlTag(ref ht) = tokens[0].token_type {
            assert_eq!(ht.expression.value, "content");
        }
    }

    // --- Scanner error recovery tests ---

    #[test]
    fn recovery_unterminated_start_tag_bare() {
        let mut scanner = Scanner::new("<div");
        let (tokens, diagnostics) = scanner.scan_tokens();
        assert_start_tag(&tokens[0], "div", vec![], false);
        assert!(tokens[1].token_type == TokenType::EOF);
        assert_has_diagnostic(&diagnostics, DiagnosticKind::UnterminatedStartTag);
    }

    #[test]
    fn recovery_unterminated_start_tag_with_bool_attr() {
        let mut scanner = Scanner::new("<div class");
        let (tokens, diagnostics) = scanner.scan_tokens();
        assert_start_tag(&tokens[0], "div", vec![("class", "")], false);
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
        let mut scanner = Scanner::new("<script>code");
        let (tokens, diagnostics) = scanner.scan_tokens();
        assert!(matches!(tokens[0].token_type, TokenType::ScriptTag(_)));
        if let TokenType::ScriptTag(ref st) = tokens[0].token_type {
            assert_eq!(st.source, "code");
        }
        assert!(tokens.last().unwrap().token_type == TokenType::EOF);
        assert_has_diagnostic(&diagnostics, DiagnosticKind::UnexpectedEndOfFile);
    }

    #[test]
    fn recovery_unclosed_style_tag() {
        let mut scanner = Scanner::new("<style>.foo{}");
        let (tokens, diagnostics) = scanner.scan_tokens();
        assert!(matches!(tokens[0].token_type, TokenType::StyleTag(_)));
        if let TokenType::StyleTag(ref st) = tokens[0].token_type {
            assert_eq!(st.source, ".foo{}");
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
        let mut scanner = Scanner::new("{name");
        let (tokens, diagnostics) = scanner.scan_tokens();
        assert!(matches!(tokens[0].token_type, TokenType::Interpolation(_)));
        if let TokenType::Interpolation(ref et) = tokens[0].token_type {
            assert_eq!(et.expression.value, "name");
        }
        assert!(tokens.last().unwrap().token_type == TokenType::EOF);
        assert_has_diagnostic(&diagnostics, DiagnosticKind::UnexpectedEndOfFile);
    }

    #[test]
    fn recovery_unclosed_if_tag() {
        let mut scanner = Scanner::new("{#if cond");
        let (tokens, diagnostics) = scanner.scan_tokens();
        assert!(matches!(tokens[0].token_type, TokenType::StartIfTag(_)));
        if let TokenType::StartIfTag(ref st) = tokens[0].token_type {
            assert_eq!(st.expression.value, "cond");
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
