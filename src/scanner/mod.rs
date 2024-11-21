mod error;

use error::ScannerError;
use std::{
    fmt::{self},
    mem, vec,
};

#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    Text,
    StartTag(StartTag),
    EndTag,
    Interpolation,
    EOF,
}

#[derive(Debug, PartialEq, Eq)]
pub struct StartTag {
    pub attributes: Vec<Attribute>,
    pub name: String,
    pub self_closing: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub struct HTMLAttribute {
    pub name: String,
    pub value: AttributeValue,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Attribute {
    HTMLAttribute(HTMLAttribute),
    ExpressionTag(ExpressionTag),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AttributeValue {
    String(String),
    ExpressionTag(ExpressionTag),
    Concatenation(Concatenation),
    Empty,
}

pub struct JsExpression {
    pub start: usize,
    pub end: usize,
    pub expression: String,
}

impl fmt::Display for AttributeValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AttributeValue::String(value) => write!(f, "{}", value),
            AttributeValue::ExpressionTag(value) => write!(f, "{}", value.expression),
            AttributeValue::Empty => write!(f, ""),
            AttributeValue::Concatenation(concatenation) => {
                let mut result = String::new();

                concatenation.parts.iter().for_each(|value| match value {
                    ConcatenationPart::String(v) => {
                        let part = format!("({})", v).to_string();
                        result.push_str(&part);
                    }
                    ConcatenationPart::Expression(expression_tag) => {
                        let part = format!("({{{}}})", expression_tag.expression).to_string();
                        result.push_str(&part);
                    }
                });

                return write!(f, "{}", result);
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ExpressionTag {
    pub start: usize,
    pub end: usize,
    pub expression: String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Concatenation {
    pub start: usize,
    pub end: usize,
    pub parts: Vec<ConcatenationPart>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ConcatenationPart {
    String(String),
    Expression(ExpressionTag),
}

#[derive(Debug)]
pub struct Token {
    pub r#type: TokenType,
    pub line: usize,
    pub lexeme: &'static str,
}

pub struct Scanner {
    source: &'static str,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: &'static str) -> Scanner {
        return Scanner {
            source,
            tokens: vec![],
            current: 0,
            start: 0,
            line: 1,
        };
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, ScannerError> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        self.tokens.push(Token {
            r#type: TokenType::EOF,
            line: self.line,
            lexeme: "",
        });

        let tokens = mem::replace(&mut self.tokens, vec![]);

        return Ok(tokens);
    }

    pub fn scan_token(&mut self) -> Result<(), ScannerError> {
        let char = self.advance();

        if char == '<' {
            if self.peek() == Some('/') {
                return self.end_tag();
            } else {
                return self.start_tag();
            }
        }

        if char == '{' {
            return self.interpolation();
        }

        self.text();

        return Ok(());
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text = &self.source[self.start..self.current];

        self.tokens.push(Token {
            r#type: token_type,
            lexeme: &text,
            line: self.line,
        });
    }

    fn advance(&mut self) -> char {
        let char = self.source.chars().nth(self.current).unwrap();
        self.current += 1;

        if char == '\n' {
            self.line += 1;
        }

        return char;
    }

    fn is_at_end(&self) -> bool {
        return self.current >= self.source.chars().count();
    }

    fn identifier(&mut self) -> String {
        let mut identifier = String::new();

        while let Some(ch) = self.peek() {
            if ch.is_alphanumeric() || ch == '-' {
                identifier.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        identifier
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self
            .source
            .chars()
            .nth(self.current)
            .is_some_and(|c| c != expected)
        {
            return false;
        }

        self.current += 1;

        return true;
    }

    fn peek(&self) -> Option<char> {
        if self.is_at_end() {
            return None;
        }

        return self.source.chars().nth(self.current);
    }

    fn collect_until<F>(&mut self, condition: F) -> Result<&'static str, ScannerError>
    where
        F: Fn(char) -> bool,
    {
        let start = self.current;

        while !self.is_at_end() {
            if self.peek().is_some_and(|c| condition(c)) {
                break;
            }

            self.advance();
        }

        if self.is_at_end() {
            return Err(ScannerError::unexpected_end_of_file(self.line));
        }

        return Ok(&self.source[start..self.current]);
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

    fn start_tag(&mut self) -> Result<(), ScannerError> {
        let name = self.identifier();

        if name.is_empty() {
            return Err(ScannerError::invalid_tag_name(self.line));
        }

        let attributes = self.attributes()?;
        let self_closing = self.match_char('/');

        if !self.match_char('>') {
            return Err(ScannerError::unterminated_start_tag(self.line));
        }

        self.add_token(TokenType::StartTag(StartTag {
            attributes,
            name,
            self_closing,
        }));

        return Ok(());
    }

    fn attributes(&mut self) -> Result<Vec<Attribute>, ScannerError> {
        let mut attributes: Vec<Attribute> = vec![];

        while let Some(ch) = self.peek() {
            if ch == '/' || ch == '>' {
                break;
            }

            self.skip_whitespace();

            if self.peek() == Some('{') {
                let expression_tag = self.expression_tag()?;

                attributes.push(Attribute::ExpressionTag(expression_tag));
            } else {
                let name = self.identifier();

                if name.is_empty() {
                    return Err(ScannerError::invalid_attribute_name(self.line));
                }

                let mut value: AttributeValue = AttributeValue::Empty;

                if self.match_char('=') {
                    value = self.attribute_value()?;
                }

                attributes.push(Attribute::HTMLAttribute(HTMLAttribute { name, value }));
            }

            // Чтобы сразу дойти до ">" в позиции когда прочитали attr2 <div attr1 attr2   >
            self.skip_whitespace();
        }

        return Ok(attributes);
    }

    fn attribute_value(&mut self) -> Result<AttributeValue, ScannerError> {
        let peeked = self.peek();

        if self.peek() == Some('{') {
            return self
                .expression_tag()
                .map(|v| AttributeValue::ExpressionTag(v));
        }

        if let Some(quote) = peeked.filter(|c| *c == '"' || *c == '\'') {
            return self.attribute_concatenation_or_string(quote);
        }

        /*
         * must not contain any literal space characters
         * must not contain any """, "'", "=", ">", "<", or "`", characters
         * must not be the empty string
         */

        let value = self.collect_until(|char| {
            return match char {
                '"' | '\'' | '>' | '<' | '`' => true,
                char => char.is_whitespace(),
            };
        })?;

        return Ok(AttributeValue::String(value.to_string()));
    }

    fn expression_tag(&mut self) -> Result<ExpressionTag, ScannerError> {
        debug_assert_eq!(self.peek(), Some('{'));

        self.advance();

        let result = self.collect_js_expression()?;

        return Ok(ExpressionTag {
            end: result.end,
            expression: result.expression,
            start: result.end,
        });
    }

    fn attribute_concatenation_or_string(
        &mut self,
        quote: char,
    ) -> Result<AttributeValue, ScannerError> {
        debug_assert_eq!(self.peek(), Some(quote));

        let mut has_expression = false;
        let start = self.current;
        let mut parts: Vec<ConcatenationPart> = vec![];
        let mut current_part = String::new();

        // consume first quote
        self.advance();

        while let Some(char) = self.peek() {
            if char == quote {
                break;
            }

            if char == '{' {
                has_expression = true;

                if !current_part.is_empty() {
                    parts.push(ConcatenationPart::String(current_part));
                    current_part = String::new();
                }

                let expression_tag = self.expression_tag()?;

                parts.push(ConcatenationPart::Expression(expression_tag));

                continue;
            }

            current_part.push(char);
            self.advance();
        }

        // consume last quote
        self.advance();

        if has_expression && !current_part.is_empty() {
            parts.push(ConcatenationPart::String(current_part.clone()));
        }

        if !has_expression && parts.is_empty() {
            return Ok(AttributeValue::String(current_part));
        }

        return Ok(AttributeValue::Concatenation(Concatenation {
            start,
            end: self.current,
            parts,
        }));
    }

    fn end_tag(&mut self) -> Result<(), ScannerError> {
        self.collect_until(|c| c == '>')?;

        self.advance();

        self.add_token(TokenType::EndTag);

        return Ok(());
    }

    fn text(&mut self) {
        while self.peek() != Some('<') && self.peek() != Some('{') && !self.is_at_end() {
            self.advance();
        }

        self.add_token(TokenType::Text);
    }

    fn interpolation(&mut self) -> Result<(), ScannerError> {
        debug_assert_eq!(self.source.chars().nth(self.current - 1), Some('{'));

        self.collect_js_expression()?;

        self.add_token(TokenType::Interpolation);
        return Ok(());
    }

    fn collect_js_expression(&mut self) -> Result<JsExpression, ScannerError> {
        let mut stack: Vec<bool> = vec![];
        let start = self.current;

        while !self.is_at_end() {
            let char = self.advance();

            if char == '\n' {
                self.line += 1;
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

            if char == '}' {
                if stack.pop().is_none() {
                    let expression = if self.current - start > 2 {
                        self.source[start..self.current - 1].to_string()
                    } else {
                        String::new()
                    };

                    return Ok(JsExpression {
                        start,
                        end: self.current,
                        expression,
                    });
                }
            }
        }

        return Err(ScannerError::unexpected_end_of_file(self.line));
    }

    fn skip_js_string(&mut self, quote: char) -> Result<(), ScannerError> {
        while self.peek() != Some(quote) && !self.is_at_end() {
            self.advance();
        }

        if self.is_at_end() {
            return Err(ScannerError::unexpected_end_of_file(self.line));
        }

        self.advance();

        return Ok(());
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use error::ScannerErrorType;

    use super::*;

    #[test]
    fn smoke() {
        let mut scanner = Scanner::new("<div>kek {name} hello</div>");

        let tokens = scanner.scan_tokens().unwrap();

        assert!(matches!(tokens[0].r#type, TokenType::StartTag(_)));
        assert!(tokens[1].r#type == TokenType::Text);
        assert!(tokens[2].r#type == TokenType::Interpolation);
        assert!(tokens[3].r#type == TokenType::Text);
        assert!(tokens[4].r#type == TokenType::EndTag);
        assert!(tokens[5].r#type == TokenType::EOF);
    }

    #[test]
    fn interpolation_with_js_strings() {
        let mut scanner = Scanner::new("{ name + '}' + \"{}\" + `{\n}` }");

        let tokens = scanner.scan_tokens().unwrap();

        assert!(tokens[0].r#type == TokenType::Interpolation);
        assert!(tokens[1].r#type == TokenType::EOF);
    }

    #[test]
    fn interpolation_js_curly_braces_balance() {
        let mut scanner = Scanner::new("{ { field: 1} + (function(){return {}}) }");

        let tokens = scanner.scan_tokens().unwrap();

        assert!(tokens[0].r#type == TokenType::Interpolation);
        assert!(tokens[1].r#type == TokenType::EOF);
    }

    #[test]
    fn self_closed_start_tag() {
        let mut scanner = Scanner::new("<input/>");

        let tokens = scanner.scan_tokens().unwrap();

        assert_start_tag(&tokens[0], "input", vec![], true);

        assert!(tokens[1].r#type == TokenType::EOF);
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

        assert!(tokens[1].r#type == TokenType::EOF);
    }

    #[test]
    fn attribute_mustache_tag_value() {
        let mut scanner = Scanner::new("<div value={666} input={} trace={\"{another}\"}>");

        let tokens = scanner.scan_tokens().unwrap();

        assert_start_tag(
            &tokens[0],
            "div",
            vec![("value", "666"), ("input", ""), ("trace", "\"{another}\"")],
            false,
        );

        assert!(tokens[1].r#type == TokenType::EOF);
    }

    #[test]
    fn concatenation_attribute_value() {
        let mut scanner = Scanner::new(
            r#"<input
                value='prefix_{value}{value}_suffix_{value}'
                id="pre{ middle }post"
                one="{one}"
                between="{one}___{two}"
            />"#,
        );

        let tokens = scanner.scan_tokens().unwrap();

        assert_start_tag(
            &tokens[0],
            "input",
            vec![
                ("value", "(prefix_)({value})({value})(_suffix_)({value})"),
                ("id", "(pre)({ middle })(post)"),
                ("one", "({one})"),
                ("between", "({one})(___)({two})"),
            ],
            true,
        );

        assert!(tokens[1].r#type == TokenType::EOF);
    }

    #[test]
    fn shorthand_expression_tag_attribute() {
        let mut scanner = Scanner::new(r#"<input { name } {...value} />"#);

        let tokens = scanner.scan_tokens().unwrap();

        assert_start_tag(
            &tokens[0],
            "input",
            vec![("$expression", " name "), ("$expression", "...value")],
            true,
        );

        assert!(tokens[1].r#type == TokenType::EOF);
    }

    #[test]
    fn unterminated_start_tag() {
        let mut scanner = Scanner::new("<div disabled");

        let result = scanner.scan_tokens();

        assert_error_result(result, ScannerErrorType::UnterminatedStartTag)
    }

    fn assert_start_tag(
        token: &Token,
        expected_name: &str,
        expected_attributes: Vec<(&str, &str)>,
        expected_self_closing: bool,
    ) {
        let start_tag = match &token.r#type {
            TokenType::StartTag(t) => t,
            _ => panic!("Expected token.type = StartTag."),
        };

        assert_eq!(start_tag.name, expected_name, "Tag name did not match");

        assert_eq!(
            start_tag.self_closing, expected_self_closing,
            "Self-closing flag did not match"
        );

        assert_attributes(&start_tag.attributes, expected_attributes);
    }

    fn assert_attributes(
        actual_attributes: &Vec<Attribute>,
        expected_attributes: Vec<(&str, &str)>,
    ) {
        assert_eq!(
            actual_attributes.len(),
            expected_attributes.len(),
            "Number of attributes did not match"
        );

        for (index, (expected_name, expected_value)) in expected_attributes.iter().enumerate() {
            // let HTMLAttribute { name, value } = &actual_attributes[index];

            let attribute = &actual_attributes[index];

            let name = match attribute {
                Attribute::HTMLAttribute(value) => value.name.clone(),
                Attribute::ExpressionTag(_) => "$expression".to_string(),
            };

            let value: AttributeValue = match attribute {
                Attribute::HTMLAttribute(value) => value.value.clone(),
                Attribute::ExpressionTag(value) => {
                    let res = AttributeValue::String(value.expression.clone());
                    res
                }
            };

            assert_eq!(name, *expected_name, "Attribute name did not match");
            assert_eq!(
                value.to_string(),
                expected_value.to_string(),
                "Attribute value did not match"
            );
        }
    }

    fn assert_error_result<T>(res: Result<T, ScannerError>, err_type: ScannerErrorType)
    where
        T: Debug,
    {
        assert!(res.is_err());

        assert_eq!(res.unwrap_err().error_type, err_type);
    }
}
