pub struct InterpolationScanner {
    source: &'static str,
    start: usize,
    current: usize,
    line: usize,
}

pub struct ScanResult {
    pub position: usize,
    pub line: usize,
}

impl InterpolationScanner {
    pub fn new(source: &'static str, line: usize, position: usize) -> InterpolationScanner {
        return InterpolationScanner {
            source,
            current: position,
            start: position,
            line,
        };
    }

    pub fn scan(&mut self) -> Result<ScanResult, &str> {
        let mut stack: Vec<bool> = vec![];

        while !self.is_at_end() {
            self.start = self.current;

            let char = self.advance();

            if char == '\n' {
                self.line += 1;
                continue;
            }

            if char == '\'' || char == '"' || char == '`' {
                self.string(char);
                continue;
            }

            if char == '{' {
                stack.push(true);
                continue;
            }

            if char == '}' {
                if stack.pop().is_none() {
                    return Ok(ScanResult {
                        position: self.current,
                        line: self.line,
                    });
                }
            }
        }

        return Err("Unexpected end.");
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

    fn peek(&self) -> Option<char> {
        if self.is_at_end() {
            return None;
        }

        return self.source.chars().nth(self.current);
    }

    fn string(&mut self, quote: char) {
        while self.peek() != Some(quote) && !self.is_at_end() {
            self.track_new_line();
            self.advance();
        }

        if self.is_at_end() {
            unimplemented!("unexpected string end");
        }

        self.advance();
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn smoke() {
        // let mut scanner = JsScanner::new("<div>content</div>");

        // let tokens = scanner.scan_tokens();

        // assert!(tokens[0].r#type == TokenType::StartTag);
        // assert!(tokens[1].r#type == TokenType::Text);
        // assert!(tokens[2].r#type == TokenType::EndTag);
        // assert!(tokens[3].r#type == TokenType::EOF);
    }
}
