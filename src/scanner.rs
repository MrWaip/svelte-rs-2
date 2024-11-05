use std::mem;

#[derive(PartialEq, Eq)]
pub enum TokenType {
    SMOKE,
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

        match char {
            '<' => self.add_token(TokenType::SMOKE),
            _ => {
                self.error(self.line, "Unexpected character.");
            }
        }
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

    fn error(&mut self, line: usize, message: &str) {
        print!("[Line {line}] Error: {message}");
        self.has_error = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        let mut scanner = Scanner::new("some");

        let tokens = scanner.scan_tokens();

        assert!(tokens[0].r#type == TokenType::EOF)
    }
}
