use scanner::Scanner;

pub mod scanner;

pub struct Parser {
    scanner: Scanner,
    source: &'static str,
    // stack:
}

impl Parser {
    pub fn new(source: &'static str) -> Parser {
        let scanner = Scanner::new(source);

        return Parser { scanner, source };
    }

    pub fn parse(&mut self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        let mut parser = Parser::new("");

        parser.parse();
    }
}
