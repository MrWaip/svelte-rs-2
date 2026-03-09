pub mod token;

use std::{iter::Peekable, str::Chars, vec};
use token::{
    Attribute, AttributeIdentifierType, AttributeValue, BindDirective, ClassDirective,
    Concatenation, ConcatenationPart, ExpressionTag, HTMLAttribute, JsExpression, ScriptTag,
    StartEachTag, StartIfTag, StartTag, Token, TokenType,
};

use svelte_diagnostics::Diagnostic;
use span::{Span, SPAN};

pub struct Scanner<'a> {
    source: &'a str,
    chars: Peekable<Chars<'a>>,
    tokens: Vec<Token<'a>>,
    start: usize,
    prev: usize,
    current: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Scanner<'a> {
        Scanner {
            source,
            tokens: vec![],
            chars: source.chars().peekable(),
            prev: 0,
            current: 0,
            start: 0,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token<'a>>, Diagnostic> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        self.tokens.push(Token {
            token_type: TokenType::EOF,
            span: Span::new(self.start, self.current),
            lexeme: "",
        });

        let tokens = std::mem::take(&mut self.tokens);
        Ok(tokens)
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
            span: Span::new(self.start, self.current),
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
                Diagnostic::unknown_directive(Span::new(colon_pos, self.current)).as_err()
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
            return Err(Diagnostic::unexpected_end_of_file(Span::new(
                start,
                self.current,
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
            return Err(Diagnostic::invalid_tag_name(Span::new(start, self.current)));
        }

        let attributes = self.attributes()?;
        let self_closing = self.match_char('/');

        if !self.match_char('>') {
            return Err(Diagnostic::unterminated_start_tag(Span::new(
                start,
                self.current,
            )));
        }

        if name == "script" {
            return self.script_tag(attributes);
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

            let attr = if peeked == Some('{') {
                let expression_tag = self.expression_tag()?;
                Attribute::ExpressionTag(expression_tag)
            } else {
                let name = self.attribute_identifier()?;

                match name {
                    AttributeIdentifierType::HTMLAttribute(name) => self.html_attribute(name)?,
                    AttributeIdentifierType::ClassDirective(value) => {
                        self.class_directive(value)?
                    }
                    AttributeIdentifierType::BindDirective(value) => self.bind_directive(value)?,
                    AttributeIdentifierType::None => break,
                }
            };

            attributes.push(attr);

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
            span: Span::new(start, self.current),
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

        // consume last quote
        self.advance();

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
            return Err(Diagnostic::invalid_tag_name(Span::new(start, self.current)));
        }

        self.skip_whitespace();

        if !self.match_char('>') {
            return Err(Diagnostic::unexpected_token(Span::new(start, self.current)));
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
            span: Span::new(self.start, self.current),
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
                    span: Span::new(span_start, span_end),
                    value,
                });
            }
        }

        Err(Diagnostic::unexpected_end_of_file(Span::new(
            start,
            self.current,
        )))
    }

    fn skip_js_string(&mut self, quote: char) -> Result<(), Diagnostic> {
        let start = self.current;
        while self.peek() != Some(quote) && !self.is_at_end() {
            self.advance();
        }

        if self.is_at_end() {
            return Err(Diagnostic::unexpected_end_of_file(Span::new(
                start,
                self.current,
            )));
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
                self.start,
                self.current,
            )));
        }

        match keyword {
            "if" => {
                let expression = self.collect_js_expression()?;

                self.add_token(TokenType::StartIfTag(StartIfTag { expression }));

                Ok(())
            }
            "each" => self.start_each_tag(),
            _ => Err(Diagnostic::unexpected_keyword(Span::new(
                start,
                self.current,
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
                self.start,
                self.current,
            )));
        }

        match keyword {
            "if" => {
                self.skip_whitespace();

                if !self.match_char('}') {
                    return Err(Diagnostic::unexpected_token(Span::new(start, self.current)));
                }

                self.add_token(TokenType::EndIfTag);

                Ok(())
            }
            "each" => {
                self.skip_whitespace();

                if !self.match_char('}') {
                    return Err(Diagnostic::unexpected_token(Span::new(start, self.current)));
                }

                self.add_token(TokenType::EndEachTag);

                Ok(())
            }
            _ => Err(Diagnostic::unexpected_keyword(Span::new(
                start,
                self.current,
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
                start,
                self.current,
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
                            start,
                            self.current,
                        )));
                    }

                    let expression = self.collect_js_expression()?;

                    self.add_token(TokenType::ElseTag(token::ElseTag {
                        elseif: true,
                        expression: Some(expression),
                    }));
                } else {
                    if !self.match_char('}') {
                        return Err(Diagnostic::unexpected_token(Span::new(start, self.current)));
                    }

                    self.add_token(TokenType::ElseTag(token::ElseTag {
                        elseif: false,
                        expression: None,
                    }));
                }

                Ok(())
            }
            _ => Err(Diagnostic::unexpected_keyword(Span::new(
                start,
                self.current,
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
            return Err(Diagnostic::unexpected_end_of_file(Span::new(
                start,
                self.current,
            )));
        }

        self.skip_whitespace();

        if !self.match_char('>') {
            return Err(Diagnostic::unexpected_token(Span::new(start, self.current)));
        }

        self.add_token(TokenType::ScriptTag(ScriptTag {
            source: self.slice_source(start, end),
            attributes,
        }));

        Ok(())
    }

    fn comment(&mut self) -> Result<(), Diagnostic> {
        let start = self.current;
        self.advance();

        if !self.match_char('-') {
            return Err(Diagnostic::unexpected_token(Span::new(start, self.current)));
        }

        if !self.match_char('-') {
            return Err(Diagnostic::unexpected_token(Span::new(start, self.current)));
        }

        while !self.is_at_end() {
            if self.match_char('-') && self.match_char('-') && self.peek() == Some('>') {
                break;
            }

            self.advance();
        }

        if self.is_at_end() {
            return Err(Diagnostic::unexpected_end_of_file(Span::new(
                start,
                self.current,
            )));
        }

        self.advance();

        self.add_token(TokenType::Comment);

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

            item = self.collect_js_expression()?.into();

            break;
        }

        let Some(collection) = collection else {
            return Diagnostic::unexpected_token(Span::new(self.start, self.current)).as_err();
        };

        let Some(item) = item else {
            return Diagnostic::unexpected_token(Span::new(self.start, self.current)).as_err();
        };

        self.add_token(TokenType::StartEachTag(StartEachTag {
            collection: JsExpression {
                span: Span::new(start_collection_pos, end_collection_pos),
                value: collection,
            },
            item,
            key: None,
            index: None,
        }));

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use svelte_diagnostics::DiagnosticKind;

    use super::*;

    #[test]
    fn smoke() {
        let mut scanner = Scanner::new("<div>kek {name} hello</div>");

        let tokens = scanner.scan_tokens().unwrap();

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

        let tokens = scanner.scan_tokens().unwrap();

        assert!(matches!(tokens[0].token_type, TokenType::Interpolation(_)));
        assert!(tokens[1].token_type == TokenType::EOF);
    }

    #[test]
    fn interpolation_js_curly_braces_balance() {
        let mut scanner = Scanner::new("{ { field: 1} + (function(){return {}}) }");

        let tokens = scanner.scan_tokens().unwrap();

        assert!(matches!(tokens[0].token_type, TokenType::Interpolation(_)));
        assert!(tokens[1].token_type == TokenType::EOF);
    }

    #[test]
    fn self_closed_start_tag() {
        let mut scanner = Scanner::new("<input/>");

        let tokens = scanner.scan_tokens().unwrap();

        assert_start_tag(&tokens[0], "input", vec![], true);
        assert!(tokens[1].token_type == TokenType::EOF);
    }

    #[test]
    fn start_tag_attributes() {
        let mut scanner = Scanner::new(
            "<div valid id=123 touched some=true disabled value=\"333\" class='never' >",
        );

        let tokens = scanner.scan_tokens().unwrap();

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
        let tokens = scanner.scan_tokens().unwrap();

        assert!(tokens[0].token_type == TokenType::Comment);
        assert_eq!(tokens[0].lexeme, "<!-- \nsome comment\n -->");
        assert!(tokens[1].token_type == TokenType::EOF);
    }

    #[test]
    fn each_block() {
        let mut scanner = Scanner::new("{#each [1,2,3] as { value, flag }}");
        let tokens = scanner.scan_tokens().unwrap();

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
    }

    fn assert_error_result<T: Debug>(res: Result<T, Diagnostic>, err_kind: DiagnosticKind) {
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().kind, err_kind);
    }

    #[test]
    fn unterminated_start_tag() {
        let mut scanner = Scanner::new("<div disabled");
        let result = scanner.scan_tokens();
        assert_error_result(result, DiagnosticKind::UnterminatedStartTag);
    }
}
