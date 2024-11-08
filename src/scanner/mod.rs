mod error;
mod interpolation_scanner;

use error::ScannerError;
use interpolation_scanner::InterpolationScanner;
use std::mem;

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
}

#[derive(Debug, PartialEq, Eq)]
pub struct Attribute {
    pub name: String,
    pub value: String,
}

#[derive(Debug)]
pub struct Token {
    pub r#type: TokenType,
    pub line: usize,
    pub lexeme: &'static str, //    literal: Object
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

        return char;
    }

    fn track_new_line(&mut self) {
        if self.peek() == Some('\n') {
            self.line += 1;
        }
    }

    fn is_at_end(&self) -> bool {
        return self.current >= self.source.chars().count();
    }

    fn tag_name(&mut self) -> String {
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

    fn _match_char(&mut self, expected: char) -> bool {
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

            self.track_new_line();
            self.advance();
        }

        if self.is_at_end() {
            return Err(ScannerError::unexpected_end_of_file(self.line));
        }

        return Ok(&self.source[start..self.current]);
    }

    fn start_tag(&mut self) -> Result<(), ScannerError> {
        let name = self.tag_name();
        let attributes = vec![];

        if name.is_empty() {
            return Err(ScannerError::invalid_tag_name(self.line));
        }

        self.collect_until(|c| c == '>')?;

        self.advance();

        self.add_token(TokenType::StartTag(StartTag { attributes, name }));

        return Ok(());
    }

    fn _attributes(&mut self) {}

    fn end_tag(&mut self) -> Result<(), ScannerError> {
        self.collect_until(|c| c == '>')?;

        self.advance();

        self.add_token(TokenType::EndTag);

        return Ok(());
    }

    fn text(&mut self) {
        while self.peek() != Some('<') && self.peek() != Some('{') && !self.is_at_end() {
            self.track_new_line();
            self.advance();
        }

        self.add_token(TokenType::Text);
    }

    fn interpolation(&mut self) -> Result<(), ScannerError> {
        let mut interpolation_scanner =
            InterpolationScanner::new(self.source, self.line, self.current);

        let result = interpolation_scanner
            .scan()
            .map_err(|_x| ScannerError::unexpected_end_of_file(self.line))?;

        self.current = result.position;
        self.line = result.line;
        self.add_token(TokenType::Interpolation);
        return Ok(());
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use error::ScannerErrorType;

    use super::*;

    fn check_error_result<T>(res: Result<T, ScannerError>, err_type: ScannerErrorType)
    where
        T: Debug,
    {
        assert!(res.is_err());

        assert_eq!(res.unwrap_err().error_type, err_type);
    }

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
    fn unterminated_start_tag() {
        let mut scanner = Scanner::new("<div disabled\n");

        let result = scanner.scan_tokens();

        check_error_result(result, ScannerErrorType::UnexpectedEndOfFile)
    }
}
