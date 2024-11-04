use std::mem;

pub enum Token {
    EOF,
}

pub struct Scanner {
    source: &'static str,
    tokens: Vec<Token>,
    start: isize,
    current: isize,
    line: isize,
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

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::EOF);

        let tokens = mem::replace(&mut self.tokens, vec![]);

        return tokens;
    }

    pub fn scan_token(&self) {
        //
    }

    pub fn is_at_end(&self) -> bool {
        return self.current >= self.source.len() as isize;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        let mut scanner = Scanner::new("some");

        scanner.scan_tokens();

        assert!(true)
    }
}
