use std::mem::replace;

use oxc_allocator::CloneIn;
use oxc_ast::ast::{Expression, Statement};
use rccell::RcCell;

use crate::ast::{
    Ast, Attribute, AttributeValue, Concatenation, ConcatenationPart, Element, HTMLAttribute,
    Interpolation, Node, Text,
};

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

        self.transform_nodes(nodes, &mut context);

        self.add_template(&mut context, &template_name);
        body.extend(context.before_init);
        body.extend(context.init);
        body.push(self.build_template_effect(context.update));
        body.extend(context.after_update);

        let close = self
            .b
            .call("$.append", [BArg::Ident("$$anchor"), BArg::Ident(id)]);

        body.push(self.b.stmt(BStmt::Expr(self.b.expr(BExpr::Call(close)))));

        return FragmentResult { body };
    }

    fn transform_nodes(
        &mut self,
        nodes: &Vec<RcCell<Node<'a>>>,
        context: &mut FragmentContext<'a>,
    ) {
        let mut to_compress: Vec<RcCell<Node<'a>>> = vec![];
        let mut idx = 0;
        let mut iter = nodes.iter();

        while let Some(node) = iter.next() {
            let can_compress = node.borrow().is_compressible();

            if can_compress {
                to_compress.push(node.clone());
                continue;
            }

            self.compress_and_transform(&mut to_compress, context, &mut idx);

            self.transform_node(&*node.borrow(), context, idx);
            idx += 1;
        }

        self.compress_and_transform(&mut to_compress, context, &mut idx);
    }

    fn compress_and_transform(
        &mut self,
        to_compress: &mut Vec<RcCell<Node<'a>>>,
        context: &mut FragmentContext<'a>,
        idx: &mut usize,
    ) {
        let len = to_compress.len();

        if len == 1 {
            self.transform_node(&*to_compress[0].borrow(), context, *idx);
            *to_compress = vec![];
            *idx += 1;
        } else if len > 1 {
            self.compress_nodes(to_compress, context, *idx);
            *to_compress = vec![];
            *idx += 1;
        }
    }

    fn transform_node(&mut self, node: &Node<'a>, ctx: &mut FragmentContext<'a>, idx: usize) {
        match node {
            Node::Element(element) => self.transform_element(element, ctx),
            Node::Text(text) => self.transform_text(text, ctx),
            Node::Interpolation(interpolation) => {
                self.transform_interpolation(&interpolation.expression, ctx, idx)
            }
            Node::IfBlock(_if_block) => todo!(),
        };
    }

    fn transform_element(&mut self, element: &Element<'a>, ctx: &mut FragmentContext<'a>) {
        ctx.template.push(format!("<{}", &element.name));

        if !element.attributes.is_empty() {
            self.transform_attributes(element, ctx);
        } else {
            ctx.template.push(">".into());
        }

        self.transform_nodes(&element.nodes, ctx);

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

    fn transform_attribute(&self, attr: &Attribute<'a>, ctx: &mut FragmentContext<'a>) {
        match attr {
            Attribute::HTMLAttribute(attr) => self.transform_html_attribute(attr, ctx),
            Attribute::Expression(expression) => {
                self.transform_expression_attribute(expression, ctx)
            }
        }
    }

    fn transform_expression_attribute(
        &self,
        expression: &Expression<'a>,
        ctx: &mut FragmentContext<'a>,
    ) {
        let node_id = "root";

        let expression = expression.clone_in(&self.b.ast.allocator);

        let arg: BArg = match &expression {
            Expression::Identifier(id) => BArg::Str(id.name.to_string()),
            _ => unreachable!(),
        };

        let call = self.b.call(
            "$.set_attribute",
            [BArg::Ident(node_id), arg, BArg::Expr(expression)],
        );

        ctx.update
            .push(self.b.stmt(BStmt::Expr(self.b.expr(BExpr::Call(call)))));
    }

    fn transform_html_attribute(&self, attr: &HTMLAttribute<'a>, ctx: &mut FragmentContext<'a>) {
        if matches!(
            attr.value,
            AttributeValue::String(_) | AttributeValue::Boolean
        ) {
            ctx.template.push(" ".into());
            ctx.template.push(attr.name.into());
        }

        match &attr.value {
            AttributeValue::String(value) => self.transform_string_attribute_value(*value, ctx),
            AttributeValue::Expression(value) => {
                self.transform_expression_attribute_value(attr, value, ctx)
            }
            AttributeValue::Boolean => (),
            AttributeValue::Concatenation(value) => {
                self.transform_concatenation_attribute_value(attr, value, ctx)
            }
        };
    }

    fn transform_string_attribute_value(&self, value: &str, ctx: &mut FragmentContext<'a>) {
        ctx.template.push(format!("=\"{value}\"").into());
    }

    fn transform_expression_attribute_value(
        &self,
        attr: &HTMLAttribute<'a>,
        value: &Expression<'a>,
        ctx: &mut FragmentContext<'a>,
    ) {
        let node_id = "root";

        let value = value.clone_in(&self.b.ast.allocator);

        let call = self.b.call(
            "$.set_attribute",
            [
                BArg::Ident(node_id),
                BArg::Str(attr.name.into()),
                BArg::Expr(value),
            ],
        );

        ctx.update
            .push(self.b.stmt(BStmt::Expr(self.b.expr(BExpr::Call(call)))));
    }

    fn transform_concatenation_attribute_value(
        &self,
        attr: &HTMLAttribute<'a>,
        value: &Concatenation<'a>,
        ctx: &mut FragmentContext<'a>,
    ) {
        let node_id = "root";

        let template_literal = self.b.template_literal(&value.parts);
        let call = self.b.call(
            "$.set_attribute",
            [
                BArg::Ident(node_id),
                BArg::Str(attr.name.into()),
                BArg::TemplateStr(template_literal),
            ],
        );

        ctx.update
            .push(self.b.stmt(BStmt::Expr(self.b.expr(BExpr::Call(call)))));
    }

    fn transform_attributes(&self, element: &Element<'a>, ctx: &mut FragmentContext<'a>) {
        for attr in element.attributes.iter() {
            self.transform_attribute(attr, ctx);
        }

        ctx.template.push(">".into());
    }

    fn build_template_effect(&self, update: Vec<Statement<'a>>) -> Statement<'a> {
        let b = self.b;

        let call = b.call("$.template_effect", [BArg::Arrow(b.arrow(update))]);

        return b.stmt(BStmt::Expr(b.expr(BExpr::Call(call))));
    }

    fn transform_text(&self, text: &Text<'a>, ctx: &mut FragmentContext<'a>) {
        ctx.template.push(text.value.to_string());
    }

    fn transform_interpolation(
        &self,
        expression: &Expression<'a>,
        ctx: &mut FragmentContext<'a>,
        idx: usize,
    ) {
        let b = self.b;
        let node_id = "root";
        let sibling_id = "text";
        let is_text = true;
        let expression = expression.clone_in(&b.ast.allocator);

        // $.set_text(text, id)
        let set_text = b.call(
            "$.set_text",
            [BArg::Ident(sibling_id), BArg::Expr(expression)],
        );

        // $.first_child(fragment)
        let first_child = b.call("$.first_child", [BArg::Ident(&node_id)]);

        // $.sibling($.first_child(fragment), 3, true);
        let sibling = b.call(
            "$.sibling",
            [
                BArg::Call(first_child),
                BArg::Num(idx as f64),
                BArg::Bool(is_text),
            ],
        );

        // var text = $.sibling($.first_child(fragment), 3, true)
        let var = self.b.var(sibling_id, BExpr::Call(sibling));

        ctx.init.push(var);
        ctx.update
            .push(b.stmt(BStmt::Expr(b.expr(BExpr::Call(set_text)))));
    }

    fn compress_nodes(
        &self,
        to_compress: &Vec<RcCell<Node<'a>>>,
        ctx: &mut FragmentContext<'a>,
        idx: usize,
    ) {
        let parts = to_compress
            .iter()
            .map(|v| {
                let node = &*v.borrow();

                match node {
                    Node::Text(text) => ConcatenationPart::String(text.value),
                    Node::Interpolation(interpolation) => ConcatenationPart::Expression(
                        interpolation.expression.clone_in(&self.b.ast.allocator),
                    ),
                    _ => unreachable!(),
                }
            })
            .collect();

        let tmp = self.b.template_literal(&parts);
        let expr = self.b.expr(BExpr::TemplateLiteral(tmp));

        self.transform_interpolation(&expr, ctx, idx);
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
