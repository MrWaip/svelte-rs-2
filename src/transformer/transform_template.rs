use std::{cell::RefCell, mem::replace, rc::Rc};

use oxc_allocator::CloneIn;
use oxc_ast::ast::{Expression, Statement};
use rccell::RcCell;

use crate::{
    ast::{
        Ast, Attribute, AttributeValue, Concatenation, ConcatenationPart, Element, HTMLAttribute,
        IfBlock, Node, Text,
    },
    parser::span::SPAN,
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

pub enum AnchorNodeType {
    Interpolation,
    VirtualConcatenation,
    IfBlock,
    Element,
}

#[derive(Debug)]
pub struct TransformTemplateResult<'a> {
    pub body: Vec<Statement<'a>>,
    pub hoisted: Vec<Statement<'a>>,
}

const COMMENT_NODE_ANCHOR: &str = "<!>";

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

impl<'a> FragmentContext<'a> {
    fn template_has_one_comment(&self) -> bool {
        if self.template.len() != 1 {
            return false;
        }

        return self
            .template
            .first()
            .is_some_and(|v| v == COMMENT_NODE_ANCHOR);
    }
}

pub struct NodeContext<'ast, 'reference> {
    fragment: &'reference mut FragmentContext<'ast>,
    builder: &'reference Builder<'ast>,
    node_anchor: Expression<'ast>,
    sibling_offset: usize,
}

impl<'ast, 'local> NodeContext<'ast, 'local> {
    pub fn get_node_anchor(&mut self) -> Expression<'ast> {
        return replace(&mut self.node_anchor, self.builder.cheap_expr());
    }

    pub fn generate(&mut self, preferable_name: &str) -> String {
        return self.fragment.scope.borrow_mut().generate(preferable_name);
    }

    pub fn push_init(&mut self, stmt: Statement<'ast>) {
        self.fragment.init.push(stmt);
    }

    pub fn push_template(&mut self, value: String) {
        self.fragment.template.push(value);
    }

    pub fn push_after_update(&mut self, stmt: Statement<'ast>) {
        self.fragment.after_update.push(stmt);
    }

    pub fn push_update(&mut self, stmt: Statement<'ast>) {
        self.fragment.update.push(stmt);
    }

    pub fn push_before_init(&mut self, stmt: Statement<'ast>) {
        self.fragment.before_init.push(stmt);
    }

    /*
       Когда например element создает переменную для себя (node_anchor),
       в качестве оптимизации следующая Node может оттолкнуться от предыдущей
    */
    pub fn reset_sibling_offset(&mut self) {
        self.sibling_offset = 0;
    }

    pub fn next_sibling_offset(&mut self) {
        self.sibling_offset += 1;
    }
}

struct CompressNodesIter<'a, 'reference> {
    nodes: &'reference Vec<RcCell<Node<'a>>>,
    idx: usize,
    to_compress: Vec<RcCell<Node<'a>>>,
    builder: &'reference Builder<'a>,
}

// !svelte optimization
impl<'a, 'reference> CompressNodesIter<'a, 'reference> {
    pub fn iter(
        nodes: &'reference Vec<RcCell<Node<'a>>>,
        builder: &'reference Builder<'a>,
    ) -> Self {
        return Self {
            builder,
            nodes,
            idx: 0,
            to_compress: vec![],
        };
    }

    fn validate_to_compress<'local>(&mut self) -> Option<RcCell<Node<'a>>> {
        let len = self.to_compress.len();

        if len == 1 {
            return self.to_compress.pop();
        } else if len > 1 {
            let res = Some(self.compress_nodes());
            self.to_compress = vec![];
            return res;
        }

        return None;
    }

    fn compress_nodes<'local>(&self) -> RcCell<Node<'a>> {
        let parts = self
            .to_compress
            .iter()
            .map(|v| {
                let node = &*v.borrow();

                match node {
                    Node::Text(text) => ConcatenationPart::String(text.value),
                    Node::Interpolation(interpolation) => ConcatenationPart::Expression(
                        self.builder.clone_expr(&interpolation.expression),
                    ),
                    _ => unreachable!(),
                }
            })
            .collect();

        return Node::VirtualConcatenation(Concatenation { parts, span: SPAN }).as_rc_cell();
    }
}

impl<'a, 'reference> Iterator for CompressNodesIter<'a, 'reference> {
    type Item = RcCell<Node<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let rc = &self.nodes.get(self.idx);
            self.idx += 1;

            if rc.is_none() {
                break;
            }

            let rc = rc.unwrap();
            let can_compress = rc.borrow().is_compressible();

            if can_compress {
                self.to_compress.push(rc.clone());
                continue;
            }

            let node = self.validate_to_compress();

            if node.is_some() {
                self.idx -= 1;
                return node;
            }

            return Some(rc.clone());
        }

        return self.validate_to_compress();
    }
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
        let mut body: Vec<Statement<'a>> = vec![];
        let scope = self.root_scope.clone();
        let template_name = scope.borrow_mut().generate("root");
        let identifier = scope.borrow_mut().generate("fragment");

        let mut context = FragmentContext {
            before_init: vec![],
            init: vec![],
            update: vec![],
            after_update: vec![],
            template: vec![],
            scope,
            anchor: self.b.expr(BExpr::Ident(self.b.rid(&identifier))),
        };

        self.transform_nodes(nodes, &mut context, None);

        // !svelte optimization
        if context.template_has_one_comment() {
            let call = self.b.call("$.comment", []);
            body.push(self.b.var(&identifier, BExpr::Call(call)));
        } else {
            let call = self.b.call(&template_name, []);
            body.push(self.b.var(&identifier, BExpr::Call(call)));
            self.add_template(&mut context, &template_name);
        }

        body.extend(context.before_init);
        body.extend(context.init);

        if !context.update.is_empty() {
            body.push(self.build_template_effect(context.update));
        }

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
        let mut anchor = &context.anchor;
        let mut callee = "$.first_child";

        if let Some(expr) = parent_node {
            anchor = expr;
            callee = "$.child";
        }

        let node_anchor = self.b.expr(BExpr::Call(
            self.b.call(callee, [BArg::Expr(self.b.clone_expr(anchor))]),
        ));

        let mut node_context = NodeContext {
            fragment: context,
            node_anchor,
            sibling_offset: 0,
            builder: self.b,
        };

        for cell in CompressNodesIter::iter(nodes, self.b) {
            let node = &*cell.borrow();
            self.transform_node(node, &mut node_context);
            node_context.next_sibling_offset();
        }

        // if there are trailing static text nodes/elements,
        // traverse to the last (n - 1) one when hydrating
        if node_context.sibling_offset > 1 {
            let offset = node_context.sibling_offset - 1;
            let mut args = vec![];

            if offset != 1 {
                args.push(BArg::Num(offset as f64));
            }

            node_context.push_init(self.b.call_stmt("$.next", args));
        }
    }

    fn transform_node<'local>(&mut self, node: &Node<'a>, ctx: &mut NodeContext<'a, 'local>) {
        match node {
            Node::Element(element) => self.transform_element(element, ctx),
            Node::Text(text) => self.transform_text(text, ctx),
            Node::Interpolation(interpolation) => {
                self.transform_interpolation(&interpolation.expression, ctx, false)
            }
            Node::IfBlock(if_block) => self.transform_if_block(if_block, ctx),
            Node::VirtualConcatenation(concatenation) => {
                self.transform_virtual_concatenation(concatenation, ctx)
            }
        };
    }

    fn add_anchor<'local>(
        &self,
        ctx: &mut NodeContext<'a, 'local>,
        preferable_name: &str,
        anchor_type: AnchorNodeType,
    ) {
        let mut anchor = ctx.get_node_anchor();
        let identifier = ctx.generate(preferable_name);

        /*
         * if this is a standalone `{expression}`, make sure we handle the case where
         * no text node was created because the expression was empty during SSR
         */
        let possibly_create_empty_text_node = matches!(anchor_type, AnchorNodeType::Interpolation);

        if ctx.sibling_offset > 0 {
            let mut args = vec![BArg::Expr(anchor)];

            if ctx.sibling_offset != 1 || possibly_create_empty_text_node {
                args.push(BArg::Num((ctx.sibling_offset) as f64));
            }

            if possibly_create_empty_text_node {
                args.push(BArg::Bool(true));
            }

            anchor = self.b.expr(BExpr::Call(self.b.call("$.sibling", args)));
        } else {
            if let Expression::CallExpression(call) = &mut anchor {
                if possibly_create_empty_text_node {
                    call.arguments.push(self.b.arg(BArg::Bool(true)));
                }
            }
        }

        let stmt = self.b.var(&identifier, BExpr::Expr(anchor));
        ctx.push_init(stmt);
        ctx.node_anchor = self.b.expr(BExpr::Ident(self.b.rid(&identifier)));
        ctx.reset_sibling_offset();
    }

    fn transform_element<'local>(
        &mut self,
        element: &Element<'a>,
        ctx: &mut NodeContext<'a, 'local>,
    ) {
        let has_children = !element.attributes.is_empty();
        ctx.push_template(format!("<{}", &element.name));

        if has_children || element.has_complex_nodes {
            self.add_anchor(ctx, &element.name, AnchorNodeType::Element);
        }

        if has_children {
            self.transform_attributes(element, ctx);
        } else {
            ctx.push_template(">".into());
        }

        self.transform_nodes(&element.nodes, ctx.fragment, Some(&ctx.node_anchor));

        if element.has_complex_nodes {
            ctx.push_init(
                self.b.stmt(BStmt::Expr(self.b.expr(BExpr::Call(self.b.call(
                    "$.reset",
                    [BArg::Expr(self.b.clone_expr(&ctx.node_anchor))],
                ))))),
            );
        }

        if !element.self_closing {
            ctx.push_template(format!("</{}>", &element.name));
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

    fn transform_attribute<'local>(&self, attr: &Attribute<'a>, ctx: &mut NodeContext<'a, 'local>) {
        match attr {
            Attribute::HTMLAttribute(attr) => self.transform_html_attribute(attr, ctx),
            Attribute::Expression(expression) => {
                self.transform_expression_attribute(expression, ctx)
            }
        }
    }

    fn transform_expression_attribute<'local>(
        &self,
        expression: &Expression<'a>,
        ctx: &mut NodeContext<'a, 'local>,
    ) {
        let node_id = self.b.clone_expr(&ctx.node_anchor);
        let expression = expression.clone_in(&self.b.ast.allocator);

        let arg: BArg = match &expression {
            Expression::Identifier(id) => BArg::Str(id.name.to_string()),
            _ => unreachable!(),
        };

        let call = self.b.call(
            "$.set_attribute",
            [BArg::Expr(node_id), arg, BArg::Expr(expression)],
        );

        ctx.push_update(self.b.stmt(BStmt::Expr(self.b.expr(BExpr::Call(call)))));
    }

    fn transform_html_attribute<'local>(
        &self,
        attr: &HTMLAttribute<'a>,
        ctx: &mut NodeContext<'a, 'local>,
    ) {
        if matches!(
            attr.value,
            AttributeValue::String(_) | AttributeValue::Boolean
        ) {
            ctx.push_template(" ".into());
            ctx.push_template(attr.name.into());
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

    fn transform_string_attribute_value<'local>(
        &self,
        value: &str,
        ctx: &mut NodeContext<'a, 'local>,
    ) {
        ctx.push_template(format!("=\"{value}\"").into());
    }

    fn transform_expression_attribute_value<'local>(
        &self,
        attr: &HTMLAttribute<'a>,
        value: &Expression<'a>,
        ctx: &mut NodeContext<'a, 'local>,
    ) {
        let node_id = self.b.clone_expr(&ctx.node_anchor);

        let value = value.clone_in(&self.b.ast.allocator);
        let call = self.b.call(
            "$.set_attribute",
            [
                BArg::Expr(node_id),
                BArg::Str(attr.name.into()),
                BArg::Expr(value),
            ],
        );

        ctx.push_update(self.b.stmt(BStmt::Expr(self.b.expr(BExpr::Call(call)))));
    }

    fn transform_concatenation_attribute_value<'local>(
        &self,
        attr: &HTMLAttribute<'a>,
        value: &Concatenation<'a>,
        ctx: &mut NodeContext<'a, 'local>,
    ) {
        let node_id = self.b.clone_expr(&ctx.node_anchor);

        let template_literal = self.b.template_literal(&value.parts);
        let call = self.b.call(
            "$.set_attribute",
            [
                BArg::Expr(node_id),
                BArg::Str(attr.name.into()),
                BArg::TemplateStr(template_literal),
            ],
        );

        ctx.push_update(self.b.stmt(BStmt::Expr(self.b.expr(BExpr::Call(call)))));
    }

    fn transform_attributes<'local>(
        &self,
        element: &Element<'a>,
        ctx: &mut NodeContext<'a, 'local>,
    ) {
        for attr in element.attributes.iter() {
            self.transform_attribute(attr, ctx);
        }

        ctx.push_template(">".into());
    }

    fn build_template_effect(&self, update: Vec<Statement<'a>>) -> Statement<'a> {
        let b = self.b;

        let call = b.call(
            "$.template_effect",
            [BArg::Arrow(b.arrow(self.b.params([]), update))],
        );

        return b.stmt(BStmt::Expr(b.expr(BExpr::Call(call))));
    }

    fn transform_text<'local>(&self, text: &Text<'a>, ctx: &mut NodeContext<'a, 'local>) {
        ctx.push_template(text.value.to_string());
    }

    fn transform_interpolation<'local>(
        &self,
        expression: &Expression<'a>,
        ctx: &mut NodeContext<'a, 'local>,
        is_concatenation: bool,
    ) {
        // whitespace for html text node for text anchor
        ctx.push_template(" ".into());

        let anchor_type = if is_concatenation {
            AnchorNodeType::VirtualConcatenation
        } else {
            AnchorNodeType::Interpolation
        };

        self.add_anchor(ctx, "text", anchor_type);

        let expression = self.b.clone_expr(expression);
        let node_id = self.b.clone_expr(&ctx.node_anchor);
        let set_text = self
            .b
            .call_stmt("$.set_text", [BArg::Expr(node_id), BArg::Expr(expression)]);

        ctx.push_update(set_text);
    }

    fn transform_virtual_concatenation<'local>(
        &self,
        concatenation: &Concatenation<'a>,
        ctx: &mut NodeContext<'a, 'local>,
    ) {
        let tmp = self.b.template_literal(&concatenation.parts);
        let expr = self.b.expr(BExpr::TemplateLiteral(tmp));

        self.transform_interpolation(&expr, ctx, true);
    }

    fn transform_if_block<'local>(
        &mut self,
        if_block: &IfBlock<'a>,
        ctx: &mut NodeContext<'a, 'local>,
    ) {
        ctx.push_template(COMMENT_NODE_ANCHOR.into());
        let mut statements = vec![];
        self.add_anchor(ctx, "node", AnchorNodeType::IfBlock);

        let consequent_fragment = self.transform_fragment(&if_block.consequent);
        let consequent_id = ctx.generate("consequent");

        let consequent = self.b.var(
            &consequent_id,
            BExpr::Arrow(
                self.b
                    .arrow(self.b.params(["$$anchor"]), consequent_fragment.body),
            ),
        );

        statements.push(consequent);

        let alternate_stmt = if let Some(alt) = &if_block.alternate {
            let alternate_fragment = self.transform_fragment(alt);
            let alternate_id = ctx.generate("alternate");

            let alternate = self.b.var(
                &alternate_id,
                BExpr::Arrow(
                    self.b
                        .arrow(self.b.params(["$$anchor"]), alternate_fragment.body),
                ),
            );

            statements.push(alternate);

            Some(
                self.b
                    .call_stmt("$$render", [BArg::Ident(&alternate_id), BArg::Bool(false)]),
            )
        } else {
            None
        };

        let mut args = vec![BArg::Expr(self.b.clone_expr(&ctx.node_anchor))];

        let if_stmt = self.b.if_stmt(
            self.b.clone_expr(&if_block.test),
            self.b.call_stmt("$$render", [BArg::Ident(&consequent_id)]),
            alternate_stmt,
        );

        let render = self.b.arrow(self.b.params(["$$render"]), [if_stmt]);

        args.push(BArg::Arrow(render));

        if if_block.is_elseif {
            args.push(BArg::Bool(true));
        }

        let if_call = self.b.call_stmt("$.if", args);

        statements.push(if_call);

        ctx.push_init(self.b.block(statements));
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
