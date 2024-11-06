use std::mem;

use crate::interpolation_scanner::InterpolationScanner;

#[derive(PartialEq, Eq)]
pub enum TokenType {
    Text,
    StartTag,
    EndTag,
    Interpolation,
    EOF,
}

pub struct Token {
    r#type: TokenType,
    line: usize,
    lexeme: &'static str, //    literal: Object
}

pub struct Scanner {
    source: &'static str,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    has_error: bool,
}

impl Scanner {
    pub fn new(source: &'static str) -> Scanner {
        return Scanner {
            source,
            tokens: vec![],
            current: 0,
            start: 0,
            line: 1,
            has_error: false,
        };
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token {
            r#type: TokenType::EOF,
            line: self.line,
            lexeme: "",
        });

        let tokens = mem::replace(&mut self.tokens, vec![]);

        return tokens;
    }

    pub fn scan_token(&mut self) {
        let char = self.advance();

        if char == '<' {
            if self.peek() == Some('/') {
                self.end_tag();
            } else {
                self.start_tag();
            }

            return;
        }

        if char == '{' {
            return self.interpolation();
        }

        self.text();
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

    fn error(&mut self, message: &str) {
        let line = self.line;
        print!("[Line {line}] Error: {message}");
        self.has_error = true;
    }

    fn start_tag(&mut self) {
        while self.peek() != Some('>') && !self.is_at_end() {
            self.track_new_line();
            self.advance();
        }

        if self.is_at_end() {
            self.error("Unterminated start tag.");
        }

        self.advance();

        self.add_token(TokenType::StartTag);
    }

    fn end_tag(&mut self) {
        while self.peek() != Some('>') && !self.is_at_end() {
            self.track_new_line();
            self.advance();
        }

        if self.is_at_end() {
            self.error("Unterminated end tag.");
        }

        self.advance();

        self.add_token(TokenType::EndTag);
    }

    fn text(&mut self) {
        while self.peek() != Some('<') && !self.is_at_end() {
            self.track_new_line();
            self.advance();
        }

        self.add_token(TokenType::Text);
    }

    fn interpolation(&mut self) {
        let mut interpolation_scanner =
            InterpolationScanner::new(self.source, self.line, self.current);

        let result = interpolation_scanner.scan();

        match result {
            Ok(result) => {
                self.current = result.position;
                self.line = result.line;
                self.add_token(TokenType::Interpolation);
            }
            Err(_) => unimplemented!("Handle interpolation scanner error"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        let mut scanner = Scanner::new("<div>{name} hello</div>");

        let tokens = scanner.scan_tokens();

        assert!(tokens[0].r#type == TokenType::StartTag);
        assert!(tokens[1].r#type == TokenType::Interpolation);
        assert!(tokens[2].r#type == TokenType::Text);
        assert!(tokens[3].r#type == TokenType::EndTag);
        assert!(tokens[4].r#type == TokenType::EOF);
    }

    #[test]
    fn interpolation_with_js_strings() {
        let mut scanner = Scanner::new("{ name + '}' + \"{}\" + `{\n}` }");

        let tokens = scanner.scan_tokens();

        assert!(tokens[0].r#type == TokenType::Interpolation);
        assert!(tokens[1].r#type == TokenType::EOF);
    }

    #[test]
    fn interpolation_js_curly_braces_balance() {
        let mut scanner = Scanner::new("{ { field: 1} + (function(){return {}}) }");

        let tokens = scanner.scan_tokens();

        assert!(tokens[0].r#type == TokenType::Interpolation);
        assert!(tokens[1].r#type == TokenType::EOF);
    }
}
