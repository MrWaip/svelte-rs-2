use oxc_ast::ast::Expression;

pub struct Parser<'a> {
    tokens: Vec<String>,
    expressions: Vec<Expression<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new() -> Parser<'a> {
        return Parser {
            tokens: vec!["a".into(), "b".into()],
            expressions: vec![],
        };
    }

    pub fn parse(&mut self) -> Result<(), ()> {
        while let Some(token) = self.tokens.pop() {
            self.parse_interpolation(&token)?; // Process each token
        }

        Ok(())
    }

    fn add_expression(&mut self, exp: Expression<'a>) {
        self.expressions.push(exp);
    }

    fn parse_interpolation(&mut self, token: &str) -> Result<(), ()> {
        // let parser = oxc_parser::Parser::new(token);

        // let exp = parser.parse_expression();

        // self.add_expression(exp);

        return Ok(());
    }
}
