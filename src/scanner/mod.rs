mod error;
mod interpolation_scanner;

use error::{ScannerError, ScannerErrorType};
use interpolation_scanner::InterpolationScanner;
use std::mem;

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
    errors: Vec<ScannerError>,
}

impl Scanner {
    pub fn new(source: &'static str) -> Scanner {
        return Scanner {
            source,
            tokens: vec![],
            current: 0,
            start: 0,
            line: 1,
            errors: vec![],
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

    fn collect_until<F>(&mut self, condition: F) -> Option<&'static str>
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
            return None;
        }

        return Some(&self.source[start..self.current]);
    }

    fn collect_until_with_recovery<F>(
        &mut self,
        condition: F,
        recovery_error_type: ScannerErrorType,
    ) -> Result<&str, ScannerError>
    where
        F: Fn(char) -> bool,
    {
        let start = self.current;

        let result = self.collect_until(condition);

        if result.is_none() {
            return self.recovery_from(start, recovery_error_type);
        }

        return Ok(&self.source[start..self.current]);
    }

    fn recovery_from(
        &mut self,
        start: usize,
        recovery_error_type: ScannerErrorType,
    ) -> Result<&str, ScannerError> {
        self.current = start;
        let line: usize = self.line;
        let result = self.collect_until(|c| c == ' ' || c == '\n');

        if result.is_none() {
            return Err(ScannerError::new(
                ScannerErrorType::UnexpectedEndOfFile,
                line,
                None,
            ));
        }

        self.errors
            .push(ScannerError::new(recovery_error_type, line, None));

        Ok(result.unwrap())
    }

    fn start_tag(&mut self) -> Result<(), ScannerError> {
        self.collect_until_with_recovery(|c| c == '>', ScannerErrorType::UnterminatedStartTag)?;

        self.advance();

        self.add_token(TokenType::StartTag);

        return Ok(());
    }

    fn end_tag(&mut self) -> Result<(), ScannerError> {
        self.collect_until_with_recovery(|c| c == '>', ScannerErrorType::UnterminatedStartTag)?;

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
        let mut scanner = Scanner::new("<div>kek {name} hello</div>");

        let tokens = scanner.scan_tokens();

        assert!(tokens[0].r#type == TokenType::StartTag);
        assert!(tokens[1].r#type == TokenType::Text);
        assert!(tokens[2].r#type == TokenType::Interpolation);
        assert!(tokens[3].r#type == TokenType::Text);
        assert!(tokens[4].r#type == TokenType::EndTag);
        assert!(tokens[5].r#type == TokenType::EOF);
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

    #[test]
    fn unterminated_start_tag() {
        // Возможно невозможно сделать recovery если встречен EOF
        // это кажется очень невозможный сценарий
        //  к тому же считать что тэги могут быть только от < до > неверно. Эти символы можно указывать в атрибуты
        // подумать что делать с recovery незакрытых тэгов
        let mut scanner = Scanner::new("<div \n <input />");

        let tokens = scanner.scan_tokens();


        assert!(tokens[0].r#type == TokenType::StartTag);
        assert!(tokens[1].r#type == TokenType::Text);
        assert!(tokens[2].r#type == TokenType::StartTag);
        assert!(tokens[3].r#type == TokenType::EOF);
    }
}
