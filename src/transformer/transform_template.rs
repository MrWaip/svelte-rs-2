use std::{cell::RefCell, mem::replace, rc::Rc};

use oxc_allocator::CloneIn;
use oxc_ast::ast::{Expression, Statement};
use rccell::RcCell;

use crate::ast::{
    Ast, Attribute, AttributeValue, Concatenation, ConcatenationPart, Element, HTMLAttribute, Node,
    Text,
};

use super::{
    builder::{
        Builder, BuilderExpression as BExpr, BuilderFunctionArgument as BArg,
        BuilderStatement as BStmt,
    },
    scope::Scope,
};

pub struct TransformTemplate<'a> {
    b: &'a Builder<'a>,
    hoisted: Vec<Statement<'a>>,
    root_scope: Rc<RefCell<Scope>>,
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
    scope: Rc<RefCell<Scope>>,
    /** identifier на фрагмент */
    anchor: Expression<'a>,
}

pub struct FragmentResult<'a> {
    body: Vec<Statement<'a>>,
}

impl<'a> TransformTemplate<'a> {
    pub fn new(builder: &'a Builder<'a>) -> TransformTemplate<'a> {
        return TransformTemplate {
            b: builder,
            hoisted: vec![],
            root_scope: Rc::new(RefCell::new(Scope::new(None))),
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
        let mut fragment_scope = Scope::new(Some(self.root_scope.clone()));
        let template_name = self.root_scope.borrow_mut().generate("template");
        let identifier = fragment_scope.generate("root");

        let mut context = FragmentContext {
            before_init: vec![],
            init: vec![],
            update: vec![],
            after_update: vec![],
            template: vec![],
            scope: Rc::new(RefCell::new(fragment_scope)),
            anchor: self.b.expr(BExpr::Ident(self.b.rid(&identifier))),
        };

        let call = self.b.call(&template_name, []);
        body.push(self.b.var(&identifier, BExpr::Call(call)));

        self.transform_nodes(nodes, &mut context, None);

        self.add_template(&mut context, &template_name);
        body.extend(context.before_init);
        body.extend(context.init);
        body.push(self.build_template_effect(context.update));
        body.extend(context.after_update);

        let close = self.b.call(
            "$.append",
            [BArg::Ident("$$anchor"), BArg::Ident(&identifier)],
        );

        body.push(self.b.stmt(BStmt::Expr(self.b.expr(BExpr::Call(close)))));

        return FragmentResult { body };
    }

    fn transform_nodes(
        &mut self,
        nodes: &Vec<RcCell<Node<'a>>>,
        context: &mut FragmentContext<'a>,
        parent_node: Option<&Expression<'a>>,
    ) {
        let mut to_compress: Vec<RcCell<Node<'a>>> = vec![];
        let mut idx = 0;
        let mut iter = nodes.iter();
        let mut anchor = &context.anchor;
        let mut callee = "$.first_child";

        if let Some(expr) = parent_node {
            anchor = expr;
            callee = "$.child";
        }

        let mut get_self = self.b.expr(BExpr::Call(
            self.b.call(callee, [BArg::Expr(self.b.clone_expr(anchor))]),
        ));

        while let Some(node) = iter.next() {
            let can_compress = node.borrow().is_compressible();

            if can_compress {
                to_compress.push(node.clone());
                continue;
            }

            get_self =
                self.compress_text_and_interpolation(&mut to_compress, context, &mut idx, get_self);

            get_self = self.transform_node(&*node.borrow(), context, idx, get_self);

            idx += 1;
        }

        self.compress_text_and_interpolation(&mut to_compress, context, &mut idx, get_self);
    }

    fn compress_text_and_interpolation(
        &mut self,
        to_compress: &mut Vec<RcCell<Node<'a>>>,
        context: &mut FragmentContext<'a>,
        idx: &mut usize,
        get_self: Expression<'a>,
    ) -> Expression<'a> {
        let len = to_compress.len();

        if len == 1 {
            let res = self.transform_node(&*to_compress[0].borrow(), context, *idx, get_self);
            *to_compress = vec![];
            *idx += 1;

            return res;
        } else if len > 1 {
            let res = self.compress_nodes(to_compress, context, *idx, get_self);
            *to_compress = vec![];
            *idx += 1;

            return res;
        }

        return get_self;
    }

    fn transform_node(
        &mut self,
        node: &Node<'a>,
        ctx: &mut FragmentContext<'a>,
        idx: usize,
        get_self: Expression<'a>,
    ) -> Expression<'a> {
        match node {
            Node::Element(element) => return self.transform_element(element, ctx, get_self, idx),
            Node::Text(text) => self.transform_text(text, ctx),
            Node::Interpolation(interpolation) => {
                return self.transform_interpolation(&interpolation.expression, ctx, idx, get_self)
            }
            Node::IfBlock(_if_block) => todo!(),
        };

        return get_self;
    }

    fn transform_element(
        &mut self,
        element: &Element<'a>,
        ctx: &mut FragmentContext<'a>,
        mut get_self: Expression<'a>,
        idx: usize,
    ) -> Expression<'a> {
        ctx.template.push(format!("<{}", &element.name));

        if !element.attributes.is_empty() {
            let var_name = ctx.scope.borrow_mut().generate(&element.name);

            if idx > 0 {
                get_self = self.b.expr(BExpr::Call(
                    self.b
                        .call("$.sibling", [BArg::Expr(get_self), BArg::Num((idx) as f64)]),
                ));
            }

            let stmt = self.b.var(&var_name, BExpr::Expr(get_self));

            ctx.init.push(stmt);

            get_self = self.b.expr(BExpr::Ident(self.b.rid(&var_name)));

            self.transform_attributes(element, ctx, &get_self);
        } else {
            ctx.template.push(">".into());
        }

        self.transform_nodes(&element.nodes, ctx, Some(&get_self));

        if !element.self_closing {
            ctx.template.push(format!("</{}>", &element.name));
        }

        return get_self;
    }

    fn add_template(&mut self, ctx: &mut FragmentContext<'a>, name: &str) {
        let call = self.b.call(
            "$.template",
            [BArg::Str(ctx.template.concat()), BArg::Num(1.0)],
        );

        let var = self.b.var(name, BExpr::Call(call));

        self.hoisted.push(var);
    }

    fn transform_attribute(
        &self,
        attr: &Attribute<'a>,
        ctx: &mut FragmentContext<'a>,
        ident: &Expression<'a>,
    ) {
        match attr {
            Attribute::HTMLAttribute(attr) => self.transform_html_attribute(attr, ctx, ident),
            Attribute::Expression(expression) => {
                self.transform_expression_attribute(expression, ctx, ident)
            }
        }
    }

    fn transform_expression_attribute(
        &self,
        expression: &Expression<'a>,
        ctx: &mut FragmentContext<'a>,
        ident: &Expression<'a>,
    ) {
        let node_id = self.b.clone_expr(ident);
        let expression = expression.clone_in(&self.b.ast.allocator);

        let arg: BArg = match &expression {
            Expression::Identifier(id) => BArg::Str(id.name.to_string()),
            _ => unreachable!(),
        };

        let call = self.b.call(
            "$.set_attribute",
            [BArg::Expr(node_id), arg, BArg::Expr(expression)],
        );

        ctx.update
            .push(self.b.stmt(BStmt::Expr(self.b.expr(BExpr::Call(call)))));
    }

    fn transform_html_attribute(
        &self,
        attr: &HTMLAttribute<'a>,
        ctx: &mut FragmentContext<'a>,
        ident: &Expression<'a>,
    ) {
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
                self.transform_expression_attribute_value(attr, value, ctx, ident)
            }
            AttributeValue::Boolean => (),
            AttributeValue::Concatenation(value) => {
                self.transform_concatenation_attribute_value(attr, value, ctx, ident)
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
        ident: &Expression<'a>,
    ) {
        let node_id = self.b.clone_expr(ident);

        let value = value.clone_in(&self.b.ast.allocator);
        let call = self.b.call(
            "$.set_attribute",
            [
                BArg::Expr(node_id),
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
        ident: &Expression<'a>,
    ) {
        let node_id = self.b.clone_expr(ident);

        let template_literal = self.b.template_literal(&value.parts);
        let call = self.b.call(
            "$.set_attribute",
            [
                BArg::Expr(node_id),
                BArg::Str(attr.name.into()),
                BArg::TemplateStr(template_literal),
            ],
        );

        ctx.update
            .push(self.b.stmt(BStmt::Expr(self.b.expr(BExpr::Call(call)))));
    }

    fn transform_attributes(
        &self,
        element: &Element<'a>,
        ctx: &mut FragmentContext<'a>,
        ident: &Expression<'a>,
    ) {
        for attr in element.attributes.iter() {
            self.transform_attribute(attr, ctx, ident);
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
        mut get_self: Expression<'a>,
    ) -> Expression<'a> {
        let b = self.b;
        let var_name = ctx.scope.borrow_mut().generate("text");
        let expression = expression.clone_in(&b.ast.allocator);

        let set_text = b.call(
            "$.set_text",
            [BArg::Ident(&var_name), BArg::Expr(expression)],
        );

        if idx > 0 {
            get_self = b.expr(BExpr::Call(
                b.call("$.sibling", [BArg::Expr(get_self), BArg::Num(idx as f64)]),
            ));
        }

        let var = self.b.var(&var_name, BExpr::Expr(get_self));
        get_self = self.b.expr(BExpr::Ident(self.b.rid(&var_name)));

        ctx.init.push(var);
        ctx.update
            .push(b.stmt(BStmt::Expr(b.expr(BExpr::Call(set_text)))));

        return get_self;
    }

    fn compress_nodes(
        &self,
        to_compress: &Vec<RcCell<Node<'a>>>,
        ctx: &mut FragmentContext<'a>,
        idx: usize,
        get_self: Expression<'a>,
    ) -> Expression<'a> {
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

        return self.transform_interpolation(&expr, ctx, idx, get_self);
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
