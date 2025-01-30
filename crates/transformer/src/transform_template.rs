use std::{
    cell::RefCell,
    collections::HashMap,
    mem::{self, replace},
    rc::Rc,
};

use oxc_ast::ast::{Expression, Statement};
use oxc_semantic::{ScopeTree, SymbolId, SymbolTable};
use rccell::RcCell;

use ast::{
    Attribute, AttributeValue, Concatenation, ConcatenationPart, Element, HTMLAttribute, IfBlock,
    Node, Text,
};

use span::SPAN;

use analyzer::Rune;

use super::{
    builder::{
        Builder, BuilderExpression as BExpr, BuilderFunctionArgument as BArg,
        BuilderStatement as BStmt,
    },
    scope::Scope,
    transform_script::TransformScript,
};

pub struct TransformTemplate<'a, 'link> {
    b: &'a Builder<'a>,
    hoisted: Vec<Statement<'a>>,
    root_scope: Rc<RefCell<Scope>>,
    transform_script: &'link TransformScript<'a>,
    symbols: SymbolTable,
    runes: &'link HashMap<SymbolId, Rune>,
    scopes: ScopeTree,
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
    use_fragment_anchor: bool,
}

#[derive(Debug)]
pub struct TrimResult {
    has_only_text_and_interpolation: bool,
    has_single_text_node: bool,
    has_single_element: bool,
    is_first_compressible: bool,
}

pub enum FragmentParent {
    IfBlock,
    Template,
}

impl FragmentParent {
    pub fn is_next_needed(&self) -> bool {
        return match self {
            FragmentParent::Template => true,
            _ => false,
        };
    }
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
            let res: Option<RcCell<Node<'a>>> = Some(self.compress_nodes());
            self.to_compress = vec![];
            return res;
        }

        return None;
    }

    fn compress_nodes<'local>(&mut self) -> RcCell<Node<'a>> {
        let parts = self
            .to_compress
            .iter_mut()
            .map(|v| {
                let node = &mut *v.borrow_mut();

                match node {
                    Node::Text(text) => ConcatenationPart::String(text.value),
                    Node::Interpolation(interpolation) => ConcatenationPart::Expression(
                        self.builder
                            .ast
                            .move_expression(&mut interpolation.expression),
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

impl<'a, 'link> TransformTemplate<'a, 'link> {
    pub fn new(
        builder: &'a Builder<'a>,
        transform_script: &'link TransformScript<'a>,
        symbols: SymbolTable,
        scopes: ScopeTree,
        runes: &'link HashMap<SymbolId, Rune>,
    ) -> Self {
        return Self {
            b: builder,
            hoisted: vec![],
            root_scope: Rc::new(RefCell::new(Scope::new(None))),
            transform_script,
            scopes,
            symbols,
            runes,
        };
    }

    pub fn transform(
        &mut self,
        template: &mut Vec<RcCell<Node<'a>>>,
    ) -> TransformTemplateResult<'a> {
        let result = self.transform_fragment(template, FragmentParent::Template);

        let hoisted = replace(&mut self.hoisted, vec![]);

        return TransformTemplateResult {
            body: result.body,
            hoisted,
        };
    }

    fn transform_fragment(
        &mut self,
        nodes: &mut Vec<RcCell<Node<'a>>>,
        parent: FragmentParent,
    ) -> FragmentResult<'a> {
        if nodes.is_empty() {
            return FragmentResult { body: vec![] };
        }

        let mut body: Vec<Statement<'a>> = vec![];
        let scope = self.root_scope.clone();
        let template_name = scope.borrow_mut().generate("root");
        let mut template_bit_flags = Some(1.0);

        // !svelte optimization
        let trim_result = self.trim_nodes(nodes);

        // !svelte optimization / hydration?
        if trim_result.is_first_compressible && parent.is_next_needed() {
            dbg!(nodes.first());
            body.push(self.b.call_stmt("$.next", []));
        }

        // !svelte specific
        let identifier: String = if trim_result.has_single_element {
            let Node::Element(element) = &*nodes[0].borrow() else {
                unreachable!()
            };

            template_bit_flags = None;
            scope.borrow_mut().generate(&element.name)
        } else if trim_result.has_only_text_and_interpolation {
            scope.borrow_mut().generate("text")
        } else if trim_result.has_single_text_node {
            scope.borrow_mut().generate("text")
        } else {
            scope.borrow_mut().generate("fragment")
        };

        let mut context = FragmentContext {
            before_init: vec![],
            init: vec![],
            update: vec![],
            after_update: vec![],
            template: vec![],
            scope: scope.clone(),
            anchor: self.b.expr(BExpr::Ident(self.b.rid(&identifier))),
        };

        let use_fragment_anchor =
            trim_result.has_single_element || trim_result.has_only_text_and_interpolation;

        self.transform_nodes(nodes, &mut context, None, use_fragment_anchor);

        // !svelte optimization
        if context.template_has_one_comment() {
            let call = self.b.call("$.comment", []);
            body.push(self.b.var(&identifier, BExpr::Call(call)));
        } else if trim_result.has_only_text_and_interpolation {
            let call = self.b.call("$.text", []);
            body.push(self.b.var(&identifier, BExpr::Call(call)));
        } else if trim_result.has_single_text_node {
            let Node::Text(text) = &*nodes[0].borrow() else {
                unreachable!()
            };

            let call = self.b.call("$.text", [BArg::Str(text.value.to_string())]);
            body.push(self.b.var(&identifier, BExpr::Call(call)));
        } else {
            let call = self.b.call(&template_name, []);
            body.push(self.b.var(&identifier, BExpr::Call(call)));
            self.add_template(&mut context, &template_name, template_bit_flags);
        }

        body.extend(context.before_init);
        body.extend(context.init);

        if !context.update.is_empty() {
            body.push(self.build_template_effect(context.update));
        }

        body.extend(context.after_update);

        let close: oxc_ast::ast::CallExpression<'_> = self.b.call(
            "$.append",
            [BArg::Ident("$$anchor"), BArg::Ident(&identifier)],
        );

        body.push(self.b.stmt(BStmt::Expr(self.b.expr(BExpr::Call(close)))));

        return FragmentResult { body };
    }

    fn transform_nodes(
        &mut self,
        nodes: &mut Vec<RcCell<Node<'a>>>,
        context: &mut FragmentContext<'a>,
        parent_node: Option<&Expression<'a>>,
        use_fragment_anchor: bool,
    ) {
        let node_anchor: Expression<'a> = if let Some(parent) = parent_node {
            self.b
                .call_expr("$.child", [BArg::Expr(self.b.clone_expr(parent))])
        } else {
            self.b.call_expr(
                "$.first_child",
                [BArg::Expr(self.b.clone_expr(&context.anchor))],
            )
        };

        let mut node_context = NodeContext {
            fragment: context,
            node_anchor,
            sibling_offset: 0,
            builder: self.b,
            use_fragment_anchor,
        };

        // !svelte optimization
        for cell in CompressNodesIter::iter(nodes, self.b) {
            let node = &mut *cell.borrow_mut();
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

    fn transform_node<'local>(&mut self, node: &mut Node<'a>, ctx: &mut NodeContext<'a, 'local>) {
        match node {
            Node::Element(element) => self.transform_element(element, ctx),
            Node::Text(text) => self.transform_text(text, ctx),
            Node::Interpolation(interpolation) => {
                self.transform_interpolation(&mut interpolation.expression, ctx, false)
            }
            Node::IfBlock(if_block) => self.transform_if_block(if_block, ctx),
            Node::VirtualConcatenation(concatenation) => {
                self.transform_virtual_concatenation(concatenation, ctx)
            }
            Node::ScriptTag(_script_tag) => todo!(),
        };
    }

    fn add_anchor<'local>(
        &self,
        ctx: &mut NodeContext<'a, 'local>,
        preferable_name: &str,
        anchor_type: AnchorNodeType,
    ) {
        // !svelte optimization
        if ctx.use_fragment_anchor {
            ctx.node_anchor = self.b.clone_expr(&ctx.fragment.anchor);
            return;
        }

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
        element: &mut Element<'a>,
        ctx: &mut NodeContext<'a, 'local>,
    ) {
        let has_children = !element.attributes.is_empty();
        ctx.push_template(format!("<{}", &element.name));

        // !svelte optimization
        if has_children || element.has_complex_nodes {
            self.add_anchor(ctx, &element.name, AnchorNodeType::Element);
        }

        if has_children {
            self.transform_attributes(element, ctx);
        } else {
            ctx.push_template(">".into());
        }

        // !svelte optimization
        self.trim_nodes(&mut element.nodes);
        self.transform_nodes(
            &mut element.nodes,
            ctx.fragment,
            Some(&ctx.node_anchor),
            false,
        );

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

    fn add_template(&mut self, ctx: &mut FragmentContext<'a>, name: &str, bit_flags: Option<f64>) {
        let template = ctx.template.concat();
        let lit = self.b.template_from_str(&template);
        let mut args = vec![BArg::TemplateStr(lit)];

        if let Some(flags) = bit_flags {
            args.push(BArg::Num(flags));
        }

        let call = self.b.call("$.template", args);

        let var = self.b.var(name, BExpr::Call(call));

        self.hoisted.push(var);
    }

    fn transform_attribute<'local>(
        &mut self,
        attr: &mut Attribute<'a>,
        ctx: &mut NodeContext<'a, 'local>,
    ) {
        match attr {
            Attribute::HTMLAttribute(attr) => self.transform_html_attribute(attr, ctx),
            Attribute::Expression(expression) => {
                self.transform_expression_attribute(expression, ctx)
            }
        }
    }

    fn transform_expression_attribute<'local>(
        &mut self,
        expression: &mut Expression<'a>,
        ctx: &mut NodeContext<'a, 'local>,
    ) {
        let node_id = self.b.clone_expr(&ctx.node_anchor);

        let arg: BArg = match &expression {
            Expression::Identifier(id) => BArg::Str(id.name.to_string()),
            _ => unreachable!(),
        };
        let expression = self.transform_expression(expression);

        let call = self.b.call(
            "$.set_attribute",
            [BArg::Expr(node_id), arg, BArg::Expr(expression)],
        );

        ctx.push_update(self.b.stmt(BStmt::Expr(self.b.expr(BExpr::Call(call)))));
    }

    fn transform_html_attribute<'local>(
        &mut self,
        attr: &mut HTMLAttribute<'a>,
        ctx: &mut NodeContext<'a, 'local>,
    ) {
        if matches!(
            attr.value,
            AttributeValue::String(_) | AttributeValue::Boolean
        ) {
            ctx.push_template(" ".into());
            ctx.push_template(attr.name.into());
        }

        match &mut attr.value {
            AttributeValue::String(value) => self.transform_string_attribute_value(*value, ctx),
            AttributeValue::Expression(value) => {
                self.transform_expression_attribute_value(&attr.name, value, ctx)
            }
            AttributeValue::Boolean => (),
            AttributeValue::Concatenation(value) => {
                self.transform_concatenation_attribute_value(&attr.name, value, ctx)
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
        &mut self,
        name: &str,
        value: &mut Expression<'a>,
        ctx: &mut NodeContext<'a, 'local>,
    ) {
        let node_id = self.b.clone_expr(&ctx.node_anchor);

        let value = self.transform_expression(value);
        let call = self.b.call(
            "$.set_attribute",
            [
                BArg::Expr(node_id),
                BArg::Str(name.into()),
                BArg::Expr(value),
            ],
        );

        ctx.push_update(self.b.stmt(BStmt::Expr(self.b.expr(BExpr::Call(call)))));
    }

    fn transform_concatenation_attribute_value<'local>(
        &mut self,
        name: &str,
        value: &mut Concatenation<'a>,
        ctx: &mut NodeContext<'a, 'local>,
    ) {
        let node_id = self.b.clone_expr(&ctx.node_anchor);

        for part in value.parts.iter_mut() {
            if let ConcatenationPart::Expression(expr) = part {
                *expr = self.transform_expression(expr);
            }
        }

        let template_literal = self.b.template_literal(&mut value.parts);
        let template_expr = self.b.expr(BExpr::TemplateLiteral(template_literal));

        let call = self.b.call(
            "$.set_attribute",
            [
                BArg::Expr(node_id),
                BArg::Str(name.into()),
                BArg::Expr(template_expr),
            ],
        );

        ctx.push_update(self.b.stmt(BStmt::Expr(self.b.expr(BExpr::Call(call)))));
    }

    fn transform_attributes<'local>(
        &mut self,
        element: &mut Element<'a>,
        ctx: &mut NodeContext<'a, 'local>,
    ) {
        for attr in element.attributes.iter_mut() {
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
        &mut self,
        expression: &mut Expression<'a>,
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

        let expression = self.transform_expression(expression);
        let node_id = self.b.clone_expr(&ctx.node_anchor);
        let set_text = self
            .b
            .call_stmt("$.set_text", [BArg::Expr(node_id), BArg::Expr(expression)]);

        ctx.push_update(set_text);
    }

    fn transform_virtual_concatenation<'local>(
        &mut self,
        concatenation: &mut Concatenation<'a>,
        ctx: &mut NodeContext<'a, 'local>,
    ) {
        let tmp = self.b.template_literal(&mut concatenation.parts);
        let mut expr = self.b.expr(BExpr::TemplateLiteral(tmp));

        self.transform_interpolation(&mut expr, ctx, true);
    }

    fn transform_if_block<'local>(
        &mut self,
        if_block: &mut IfBlock<'a>,
        ctx: &mut NodeContext<'a, 'local>,
    ) {
        ctx.push_template(COMMENT_NODE_ANCHOR.into());
        let mut statements = vec![];
        self.add_anchor(ctx, "node", AnchorNodeType::IfBlock);

        let consequent_fragment =
            self.transform_fragment(&mut if_block.consequent, FragmentParent::IfBlock);
        let consequent_id = ctx.generate("consequent");

        let consequent = self.b.var(
            &consequent_id,
            BExpr::Arrow(
                self.b
                    .arrow(self.b.params(["$$anchor"]), consequent_fragment.body),
            ),
        );

        statements.push(consequent);

        let alternate_stmt = if let Some(alt) = &mut if_block.alternate {
            let alternate_fragment = self.transform_fragment(alt, FragmentParent::IfBlock);
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

    fn transform_expression(&mut self, expression: &mut Expression<'a>) -> Expression<'a> {
        let expression = self.b.ast.move_expression(expression);

        let symbols = mem::take(&mut self.symbols);
        let scopes = mem::take(&mut self.scopes);

        let result = self
            .transform_script
            .transform_expression(expression, symbols, scopes, self.runes);

        self.symbols = result.symbols;
        self.scopes = result.scopes;

        return result.expression;
    }

    fn trim_nodes(&self, nodes: &mut Vec<RcCell<Node<'a>>>) -> TrimResult {
        if nodes.is_empty() {
            return TrimResult {
                has_only_text_and_interpolation: false,
                has_single_text_node: false,
                has_single_element: false,
                is_first_compressible: false,
            };
        }

        let mut trimmed: Vec<RcCell<Node<'a>>> = Vec::new();
        let mut start: usize = 0;
        let mut end = nodes.len();
        let mut has_only_text_or_interpolation = true;
        let mut has_elements = false;
        let mut has_interpolation = false;
        let mut has_text = false;

        // trim left
        for cell in nodes.iter_mut() {
            let node = &mut *cell.borrow_mut();

            if let Node::Text(text) = node {
                if text.is_removable() {
                    start += 1;
                    continue;
                } else {
                    text.trim_start();
                    break;
                }
            } else {
                break;
            }
        }

        for cell in nodes.iter_mut().rev() {
            let node = &mut *cell.borrow_mut();

            if let Node::Text(text) = node {
                if text.is_removable() {
                    end -= 1;
                    continue;
                } else {
                    text.trim_end();
                    break;
                }
            } else {
                break;
            }
        }

        for idx in start..end {
            let prev = if idx == 0 { None } else { nodes.get(idx - 1) };
            let mut current = nodes.get(idx).unwrap().borrow_mut();
            let next = nodes.get(idx + 1);

            if current.is_text() {
                let Node::Text(text) = &mut *current else {
                    unreachable!()
                };

                if !prev.is_some_and(|cell| cell.borrow().is_interpolation()) {
                    text.trim_start_one_whitespace(&self.b.ast.allocator);
                }

                if !next.is_some_and(|cell| cell.borrow().is_interpolation()) {
                    text.trim_end_one_whitespace(&self.b.ast.allocator);
                }
            }

            if has_only_text_or_interpolation {
                has_only_text_or_interpolation = current.is_compressible();
            }

            if !has_interpolation {
                has_interpolation = current.is_interpolation();
            }

            if !has_elements {
                has_elements = current.is_element();
            }

            if !has_text {
                has_text = current.is_text();
            }

            trimmed.push(nodes[idx].clone());
        }

        let result = TrimResult {
            has_only_text_and_interpolation: has_only_text_or_interpolation && has_interpolation,
            has_single_text_node: has_text && trimmed.len() == 1,
            has_single_element: has_elements && trimmed.len() == 1,
            // !svelte specific
            is_first_compressible: trimmed
                .first()
                .is_some_and(|cell| cell.borrow().is_compressible()),
        };

        *nodes = trimmed;

        return result;
    }
}
