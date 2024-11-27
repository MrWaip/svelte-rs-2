use scanner::Scanner;

use crate::ast::{Ast, Node};

pub mod scanner;

pub struct Parser {
    scanner: Scanner,
    source: &'static str,
    stack: Vec<Node>,
}

impl Parser {
    pub fn new(source: &'static str) -> Parser {
        let scanner = Scanner::new(source);

        return Parser {
            scanner,
            source,
            stack: vec![],
        };
    }

    pub fn parse(&mut self) -> Ast {
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
