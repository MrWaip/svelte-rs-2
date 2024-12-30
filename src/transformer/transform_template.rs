use std::mem::replace;

use oxc_ast::ast::Statement;
use rccell::RcCell;

use crate::ast::{Ast, Element, Node, Text};

use super::builder::{
    Builder, BuilderExpression as BExpr, BuilderFunctionArgument as BArg, BuilderStatement as BStmt,
};

pub struct TransformTemplate<'a> {
    b: &'a Builder<'a>,
    hoisted: Vec<Statement<'a>>,
}

#[derive(Debug)]
pub struct TransformTemplateResult<'a> {
    pub body: Vec<Statement<'a>>,
    pub hoisted: Vec<Statement<'a>>,
}

pub struct FragmentContext<'a> {
    before_init: Vec<Statement<'a>>,
    init: Vec<Statement<'a>>,
    update: Vec<Statement<'a>>,
    after_update: Vec<Statement<'a>>,
    template: Vec<String>,
}

pub struct FragmentResult<'a> {
    body: Vec<Statement<'a>>,
}

impl<'a> TransformTemplate<'a> {
    pub fn new(builder: &'a Builder<'a>) -> TransformTemplate<'a> {
        return TransformTemplate {
            b: builder,
            hoisted: vec![],
        };
    }

    pub fn transform(&mut self, ast: &Ast<'a>) -> TransformTemplateResult<'a> {
        let result = self.transform_fragment(&ast.template);

        let hoisted = replace(&mut self.hoisted, vec![]);

        return TransformTemplateResult {
            body: result.body,
            hoisted,
        };
    }

    fn transform_fragment(&mut self, nodes: &Vec<RcCell<Node<'a>>>) -> FragmentResult<'a> {
        let mut body = vec![];
        let template_name = "template";
        let id = "root";

        let mut context = FragmentContext {
            before_init: vec![],
            init: vec![],
            update: vec![],
            after_update: vec![],
            template: vec![],
        };

        let call = self.b.call(template_name, []);
        let var = self.b.var(id, BExpr::Call(call));

        body.push(var);

        for node in nodes.iter() {
            let node = &*node.borrow();

            self.transform_node(node, &mut context);
        }

        self.add_template(&mut context, &template_name);

        body.extend(context.before_init);
        body.extend(context.init);
        body.extend(context.update);
        body.extend(context.after_update);

        let close = self
            .b
            .call("$.append", [BArg::Ident("$$anchor"), BArg::Ident(id)]);

        body.push(self.b.stmt(BStmt::Expr(self.b.expr(BExpr::Call(close)))));

        return FragmentResult { body };
    }

    fn transform_node(&self, node: &Node<'a>, ctx: &mut FragmentContext<'a>) {
        match node {
            Node::Element(element) => self.transform_element(element, ctx),
            Node::Text(text) => self.transform_text(text, ctx),
            Node::Interpolation(_interpolation) => todo!(),
            Node::IfBlock(_if_block) => todo!(),
        };
    }

    fn transform_text(&self, text: &Text, ctx: &mut FragmentContext<'a>) {
        ctx.template.push(text.value.clone());
    }

    fn transform_element(&self, element: &Element<'a>, ctx: &mut FragmentContext<'a>) {
        ctx.template.push(format!("<{}>", &element.name));

        if !element.attributes.is_empty() {
            unimplemented!();
        }

        for node in element.nodes.iter() {
            let node = &*node.borrow();

            self.transform_node(node, ctx);
        }

        if !element.self_closing {
            ctx.template.push(format!("</{}>", &element.name));
        }
    }

    fn add_template(&mut self, ctx: &mut FragmentContext<'a>, name: &str) {
        let call = self.b.call(
            "$.template",
            [BArg::Str(ctx.template.concat()), BArg::Num(1.0)],
        );

        let var = self.b.var(name, BExpr::Call(call));

        self.hoisted.push(var);
    }
}

#[cfg(test)]
mod tests {
    use oxc_allocator::Allocator;
    use oxc_ast::AstBuilder;

    use crate::parser::Parser;

    use super::*;

    #[test]
    fn smoke() {
        let allocator = Allocator::default();
        let mut parser = Parser::new("prefix <div>text</div>", &allocator);
        let ast_builder = AstBuilder::new(&allocator);
        let builder = Builder::new(ast_builder);
        let ast = parser.parse().unwrap();
        let mut transformer = TransformTemplate::new(&builder);

        let result = transformer.transform(&ast);

        dbg!(result);
    }
}
