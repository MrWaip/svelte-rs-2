use std::{cell::RefCell, mem::replace, rc::Rc};

use analyzer::{
    compute_optimization::{ContentType, NodeOptimizationAction, TrimAction},
    svelte_table::SvelteTable,
};
use oxc_ast::ast::{Expression, Statement};
use oxc_semantic::NodeId;

use ast::{
    metadata::{FragmentAnchor, InterpolationMetadata, InterpolationSetterKind, WithMetadata},
    AsNode, Attribute, AttributeValue, Concatenation, ConcatenationPart, Element,
    ExpressionAttribute, ExpressionAttributeValue, Fragment, HTMLAttribute, IfBlock, Node,
    Template, Text, VirtualConcatenation,
};

use span::SPAN;

use super::{scope::Scope, transform_script::TransformScript};

use ast_builder::{
    Builder, BuilderAssignmentLeft as BAssignLeft, BuilderAssignmentRight as BAssignRight,
    BuilderExpression as BExpr, BuilderFunctionArgument as BArg, BuilderStatement as BStmt,
};

pub struct TransformTemplate<'a, 'link> {
    b: &'a Builder<'a>,
    hoisted: Vec<Statement<'a>>,
    root_scope: Rc<RefCell<Scope>>,
    transform_script: &'link TransformScript<'a, 'link>,
    svelte_table: &'link SvelteTable,
}

#[derive(Debug)]
pub enum AnchorNodeType {
    Interpolation(InterpolationSetterKind),
    VirtualConcatenation(InterpolationSetterKind),
    IfBlock,
    Element(String),
}

impl AnchorNodeType {
    pub fn is_element(&self) -> bool {
        return matches!(self, AnchorNodeType::Element(_));
    }
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

pub struct NodeContext<'ast, 'reference> {
    builder: &'reference Builder<'ast>,
    fragment: &'reference mut FragmentContext<'ast>,
    parent_node_anchor: Option<&'reference Expression<'ast>>,
    current_node_anchor: Expression<'ast>,
    sibling_offset: usize,
    skip_reset_element: bool,
    content_type: ContentType,

    /**
     * Было ли использовано обращение к parent_anchor или fragment_anchor
     * Все последующие обращения должен строится относительно current_node_anchor
     */
    parent_or_fragment_used: bool,
}

impl<'ast, 'local> NodeContext<'ast, 'local> {
    pub fn new(
        fragment_context: &'local mut FragmentContext<'ast>,
        builder: &'local Builder<'ast>,
        parent_node_anchor: Option<&'local Expression<'ast>>,
        content_type: ContentType,
    ) -> Self {
        return Self {
            fragment: fragment_context,
            current_node_anchor: builder.cheap_expr(),
            sibling_offset: 0,
            builder,
            parent_node_anchor,
            parent_or_fragment_used: false,
            skip_reset_element: false,
            content_type,
        };
    }

    fn as_child(&self, expr: &Expression<'ast>) -> Expression<'ast> {
        self.builder
            .call_expr("$.child", [BArg::Expr(self.builder.clone_expr(expr))])
    }

    fn as_first_child(&self, expr: &Expression<'ast>) -> Expression<'ast> {
        self.builder
            .call_expr("$.first_child", [BArg::Expr(self.builder.clone_expr(expr))])
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

    fn need_direct_fragment_access(&self, anchor_type: &AnchorNodeType) -> bool {
        let at_fragment: bool = self.at_fragment();

        if !at_fragment {
            return false;
        }

        return match anchor_type {
            AnchorNodeType::Interpolation(_) | AnchorNodeType::VirtualConcatenation(_) => {
                self.content_type.is_compressible_sequence()
            }
            AnchorNodeType::IfBlock => false,
            AnchorNodeType::Element(_) => self.content_type.is_element(),
        };
    }

    fn need_direct_parent_access(&self, anchor_type: &AnchorNodeType) -> bool {
        return match anchor_type {
            AnchorNodeType::Interpolation(kind) => {
                self.content_type.is_compressible_sequence()
                    && *kind != InterpolationSetterKind::SetText
            }
            AnchorNodeType::VirtualConcatenation(kind) => {
                self.content_type.is_compressible_sequence()
                    && *kind != InterpolationSetterKind::SetText
            }
            AnchorNodeType::IfBlock => false,
            AnchorNodeType::Element(_) => false,
        };
    }

    fn at_fragment(&self) -> bool {
        return self.parent_node_anchor.is_none();
    }

    fn preferable_name(&self, anchor_type: &AnchorNodeType) -> String {
        return match anchor_type {
            AnchorNodeType::Interpolation(_) | AnchorNodeType::VirtualConcatenation(_) => {
                String::from("text")
            }
            AnchorNodeType::IfBlock => String::from("node"),
            AnchorNodeType::Element(name) => name.clone(),
        };
    }

    /**
     * Match complex svelte logic. mixed optimizations
     */
    fn next_anchor(&mut self, anchor_type: &AnchorNodeType) -> (Expression<'ast>, bool) {
        if self.parent_or_fragment_used {
            return (
                self.builder
                    .ast
                    .move_expression(&mut self.current_node_anchor),
                false,
            );
        } else {
            self.parent_or_fragment_used = true;

            if self.need_direct_fragment_access(anchor_type) {
                return (self.builder.clone_expr(&self.fragment.anchor), true);
            } else {
                if self.parent_node_anchor.is_some() {
                    if self.need_direct_parent_access(anchor_type) {
                        return (
                            self.builder.clone_expr(&self.parent_node_anchor.unwrap()),
                            true,
                        );
                    } else {
                        return (self.as_child(self.parent_node_anchor.unwrap()), false);
                    }
                } else {
                    return (self.as_first_child(&self.fragment.anchor), false);
                }
            }
        };
    }

    pub fn set_skip_reset_element(&mut self) {
        self.skip_reset_element = true;
    }

    pub fn add_anchor(&mut self, anchor_type: AnchorNodeType) {
        let preferable_name = self.preferable_name(&anchor_type);
        let (mut anchor, early_return) = self.next_anchor(&anchor_type);

        // !svelte specific
        if early_return {
            self.current_node_anchor = anchor;
            return;
        }

        let identifier = self.generate(&preferable_name);

        // /*
        //  * if this is a standalone `{expression}`, make sure we handle the case where
        //  * no text node was created because the expression was empty during SSR
        //  */
        let possibly_create_empty_text_node =
            matches!(anchor_type, AnchorNodeType::Interpolation(_));

        if self.sibling_offset > 0 {
            let mut args = vec![BArg::Expr(anchor)];

            if self.sibling_offset != 1 || possibly_create_empty_text_node {
                args.push(BArg::Num((self.sibling_offset) as f64));
            }

            if possibly_create_empty_text_node {
                args.push(BArg::Bool(true));
            }

            anchor = self.builder.call_expr("$.sibling", args);
        } else {
            if let Expression::CallExpression(call) = &mut anchor {
                if possibly_create_empty_text_node {
                    call.arguments.push(self.builder.arg(BArg::Bool(true)));
                }
            }
        }

        let stmt = self.builder.var(&identifier, BExpr::Expr(anchor));
        self.current_node_anchor = self
            .builder
            .expr(BExpr::Ident(self.builder.rid(&identifier)));
        self.reset_sibling_offset();

        self.push_init(stmt);
    }
}

struct CompressNodesIter<'a, 'reference> {
    nodes: &'reference Vec<Node<'a>>,
    idx: usize,
    to_compress: Vec<Node<'a>>,
    builder: &'reference Builder<'a>,
}

// !svelte optimization
impl<'a, 'reference> CompressNodesIter<'a, 'reference> {
    pub fn iter(nodes: &'reference Vec<Node<'a>>, builder: &'reference Builder<'a>) -> Self {
        return Self {
            builder,
            nodes,
            idx: 0,
            to_compress: vec![],
        };
    }

    fn validate_to_compress<'local>(&mut self) -> Option<Node<'a>> {
        let len = self.to_compress.len();

        if len == 1 {
            return self.to_compress.pop();
        } else if len > 1 {
            let res: Option<Node<'a>> = Some(self.compress_nodes());
            self.to_compress = vec![];
            return res;
        }

        return None;
    }

    fn compress_nodes<'local>(&mut self) -> Node<'a> {
        let mut metadata = InterpolationMetadata::default();
        let parts = self
            .to_compress
            .iter_mut()
            .map(|node| match node {
                Node::Text(text) => ConcatenationPart::String(text.borrow().value),
                Node::Interpolation(interpolation) => {
                    let new_expr = self
                        .builder
                        .ast
                        .move_expression(&mut interpolation.borrow_mut().expression);

                    metadata.add(interpolation.borrow().get_metadata());

                    ConcatenationPart::Expression(new_expr)
                }
                _ => unreachable!(),
            })
            .collect();

        return VirtualConcatenation {
            parts,
            span: SPAN,
            metadata,
        }
        .as_node();
    }
}

impl<'a, 'reference> Iterator for CompressNodesIter<'a, 'reference> {
    type Item = Node<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let rc = &self.nodes.get(self.idx);
            self.idx += 1;

            if rc.is_none() {
                break;
            }

            let rc = rc.unwrap();
            let can_compress = rc.is_compressible();

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
        transform_script: &'link TransformScript<'a, 'link>,
        svelte_table: &'link SvelteTable,
    ) -> Self {
        return Self {
            b: builder,
            hoisted: vec![],
            root_scope: Rc::new(RefCell::new(Scope::new(None))),
            transform_script,
            svelte_table,
        };
    }

    pub fn transform(&mut self, fragment: &mut Template<'a>) -> TransformTemplateResult<'a> {
        let result = self.transform_fragment(&mut fragment.nodes);

        let hoisted = replace(&mut self.hoisted, vec![]);

        return TransformTemplateResult {
            body: result.body,
            hoisted,
        };
    }

    fn transform_fragment(&mut self, fragment: &mut Fragment<'a>) -> FragmentResult<'a> {
        // !svelte optimization
        let node_id = fragment.node_id();
        let metadata = fragment.get_metadata();

        if metadata.is_empty {
            return FragmentResult { body: vec![] };
        }

        let mut body: Vec<Statement<'a>> = vec![];
        let scope = self.root_scope.clone();
        let template_name = scope.borrow_mut().generate("root");
        let mut template_bit_flags = Some(1.0);

        // !svelte optimization / hydration?
        if metadata.need_start_with_next {
            body.push(self.b.call_stmt("$.next", []));
        }

        // !svelte specific
        let identifier: String = match metadata.anchor {
            FragmentAnchor::Text | FragmentAnchor::TextInline => {
                scope.borrow_mut().generate("text")
            }
            FragmentAnchor::Element => {
                let node = fragment
                    .nodes
                    .iter()
                    .find(|cell| cell.is_element())
                    .unwrap();

                let Node::Element(element) = node else {
                    unreachable!()
                };

                template_bit_flags = None;
                scope.borrow_mut().generate(&element.borrow().name)
            }
            FragmentAnchor::Fragment | FragmentAnchor::Comment => {
                scope.borrow_mut().generate("fragment")
            }
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

        self.transform_nodes(&mut fragment.nodes, &mut context, None, node_id);

        match metadata.anchor {
            FragmentAnchor::Text => {
                let call = self.b.call("$.text", []);
                body.push(self.b.var(&identifier, BExpr::Call(call)));
            }
            FragmentAnchor::TextInline => {
                let Node::Text(text) = &fragment[0] else {
                    unreachable!()
                };

                let call = self
                    .b
                    .call("$.text", [BArg::Str(text.borrow().value.to_string())]);
                body.push(self.b.var(&identifier, BExpr::Call(call)));
            }
            FragmentAnchor::Comment => {
                let call = self.b.call("$.comment", []);
                body.push(self.b.var(&identifier, BExpr::Call(call)));
            }
            FragmentAnchor::Fragment | FragmentAnchor::Element => {
                let call = self.b.call(&template_name, []);
                body.push(self.b.var(&identifier, BExpr::Call(call)));
                self.add_template(&mut context, &template_name, template_bit_flags);
            }
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

    fn transform_nodes<'local>(
        &mut self,
        nodes: &mut Vec<Node<'a>>,
        context: &'local mut FragmentContext<'a>,
        parent_node_anchor: Option<&'local Expression<'a>>,
        node_id: NodeId,
    ) -> NodeContext<'a, 'local> {
        let optimization = self.svelte_table.get_optimization(node_id).unwrap();
        self.optimize_nodes(nodes, &optimization.actions);

        let mut node_context = NodeContext::new(
            context,
            self.b,
            parent_node_anchor,
            optimization.content_type.clone(),
        );

        // !svelte optimization
        for node in CompressNodesIter::iter(nodes, self.b) {
            // let node = &mut *node.borrow_mut();
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

        return node_context;
    }

    fn transform_node<'local>(&mut self, node: Node<'a>, ctx: &mut NodeContext<'a, 'local>) {
        match node {
            Node::Element(it) => self.transform_element(&mut *it.borrow_mut(), ctx),
            Node::Text(it) => self.transform_text(&mut *it.borrow_mut(), ctx),
            Node::Interpolation(it) => {
                let metadata = it.borrow().get_metadata();
                self.transform_interpolation(&mut it.borrow_mut().expression, ctx, false, metadata)
            }
            Node::IfBlock(it) => self.transform_if_block(&mut *it.borrow_mut(), ctx),
            Node::VirtualConcatenation(it) => {
                self.transform_virtual_concatenation(&mut *it.borrow_mut(), ctx)
            }
            Node::ScriptTag(_script_tag) => todo!(),
        };
    }

    fn transform_element<'local>(
        &mut self,
        element: &mut Element<'a>,
        ctx: &mut NodeContext<'a, 'local>,
    ) {
        let node_id = element.node_id();
        let metadata = element.get_metadata();
        let has_attributes = !element.attributes.is_empty();

        ctx.push_template(format!("<{}", &element.name));

        // !svelte specific
        if metadata.has_dynamic_nodes {
            ctx.add_anchor(AnchorNodeType::Element(element.name.to_string()));
        }

        if has_attributes {
            self.transform_attributes(element, ctx);
        } else {
            ctx.push_template(">".into());
        }

        let child_ctx = self.transform_nodes(
            &mut element.nodes,
            ctx.fragment,
            Some(&ctx.current_node_anchor),
            node_id,
        );

        if metadata.need_reset && !child_ctx.skip_reset_element {
            ctx.push_init(self.b.call_stmt(
                "$.reset",
                [BArg::Expr(self.b.clone_expr(&ctx.current_node_anchor))],
            ));
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
            Attribute::ClassDirective(directive) => {
                self.transform_class_directive_attribute(directive, ctx)
            }
            Attribute::BindDirective(bind_directive) => todo!(),
        }
    }

    fn transform_class_directive_attribute<'local>(
        &mut self,
        directive: &mut ast::ClassDirective<'a>,
        ctx: &mut NodeContext<'a, 'local>,
    ) {
        let metadata = directive.get_metadata();
        let node_id = self.b.clone_expr(&ctx.current_node_anchor);

        let expression = self.transform_expression(&mut directive.expression);

        let call = self.b.call_stmt(
            "$.toggle_class",
            [
                BArg::Expr(node_id),
                BArg::Str(directive.name.to_string()),
                BArg::Expr(expression),
            ],
        );

        if metadata.has_reactivity {
            ctx.push_update(call);
        } else {
            ctx.push_init(call);
        }
    }

    fn transform_expression_attribute<'local>(
        &mut self,
        attr: &mut ExpressionAttribute<'a>,
        ctx: &mut NodeContext<'a, 'local>,
    ) {
        let node_id = self.b.clone_expr(&ctx.current_node_anchor);
        let metadata = attr.get_metadata();

        let arg: BArg = match &attr.expression {
            Expression::Identifier(id) => BArg::Str(id.name.to_string()),
            _ => unreachable!(),
        };
        let expression = self.transform_expression(&mut attr.expression);

        let call = self.b.call_stmt(
            "$.set_attribute",
            [BArg::Expr(node_id), arg, BArg::Expr(expression)],
        );

        if metadata.has_reactivity {
            ctx.push_update(call);
        } else {
            ctx.push_init(call);
        }
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
            AttributeValue::Boolean => {
                ctx.push_template("=\"\"".to_string());
            }
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
        value: &mut ExpressionAttributeValue<'a>,
        ctx: &mut NodeContext<'a, 'local>,
    ) {
        let metadata = value.get_metadata();
        let node_id = self.b.clone_expr(&ctx.current_node_anchor);
        let value = self.transform_expression(&mut value.expression);
        let call = self.b.call_stmt(
            "$.set_attribute",
            [
                BArg::Expr(node_id),
                BArg::Str(name.into()),
                BArg::Expr(value),
            ],
        );

        if metadata.has_reactivity {
            ctx.push_update(call);
        } else {
            ctx.push_init(call);
        }
    }

    fn transform_concatenation_attribute_value<'local>(
        &mut self,
        name: &str,
        value: &mut Concatenation<'a>,
        ctx: &mut NodeContext<'a, 'local>,
    ) {
        let node_id = self.b.clone_expr(&ctx.current_node_anchor);
        let metadata = value.get_metadata();

        for part in value.parts.iter_mut() {
            if let ConcatenationPart::Expression(expr) = part {
                *expr = self.transform_expression(expr);
            }
        }

        let template_literal = self.b.template_literal(&mut value.parts);
        let template_expr = self.b.expr(BExpr::TemplateLiteral(template_literal));

        let call = self.b.call_stmt(
            "$.set_attribute",
            [
                BArg::Expr(node_id),
                BArg::Str(name.into()),
                BArg::Expr(template_expr),
            ],
        );

        if metadata.has_reactivity {
            ctx.push_update(call);
        } else {
            ctx.push_init(call);
        }
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
        metadata: InterpolationMetadata,
    ) {
        if metadata.need_template {
            ctx.push_template(" ".into());
        } else {
            ctx.set_skip_reset_element();
        }

        let anchor_type = if is_concatenation {
            AnchorNodeType::VirtualConcatenation(metadata.setter_kind)
        } else {
            AnchorNodeType::Interpolation(metadata.setter_kind)
        };

        ctx.add_anchor(anchor_type);

        let expression = self.transform_expression(expression);
        let node_id = self.b.clone_expr(&ctx.current_node_anchor);

        if metadata.setter_kind == InterpolationSetterKind::SetText {
            let set_text = self
                .b
                .call_stmt("$.set_text", [BArg::Expr(node_id), BArg::Expr(expression)]);

            ctx.push_update(set_text);
        } else {
            let prop = match metadata.setter_kind {
                InterpolationSetterKind::NodeValue => "nodeValue",
                InterpolationSetterKind::TextContent => "textContent",
                InterpolationSetterKind::SetText => unreachable!(),
            };

            let member = self.b.static_member_expr(node_id, prop);

            let set_text = self.b.assignment_expression_stmt(
                BAssignLeft::StaticMemberExpression(member),
                BAssignRight::Expr(expression),
            );

            ctx.push_init(set_text);
        }
    }

    fn transform_virtual_concatenation<'local>(
        &mut self,
        concatenation: &mut VirtualConcatenation<'a>,
        ctx: &mut NodeContext<'a, 'local>,
    ) {
        let tmp = self.b.template_literal(&mut concatenation.parts);
        let mut expr = self.b.expr(BExpr::TemplateLiteral(tmp));

        self.transform_interpolation(&mut expr, ctx, true, concatenation.metadata);
    }

    fn transform_if_block<'local>(
        &mut self,
        if_block: &mut IfBlock<'a>,
        ctx: &mut NodeContext<'a, 'local>,
    ) {
        ctx.push_template(COMMENT_NODE_ANCHOR.into());
        let mut statements = vec![];
        ctx.add_anchor(AnchorNodeType::IfBlock);

        let test = self.transform_expression(&mut if_block.test);

        let consequent_fragment = self.transform_fragment(&mut if_block.consequent);
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

        let mut args = vec![BArg::Expr(self.b.clone_expr(&ctx.current_node_anchor))];

        let if_stmt = self.b.if_stmt(
            test,
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

        let result = self.transform_script.transform_expression(expression);

        return result.expression;
    }

    fn optimize_nodes(&self, nodes: &mut Vec<Node<'a>>, actions: &Vec<NodeOptimizationAction>) {
        let mut new: Vec<Node<'a>> = vec![];

        for idx in 0..nodes.len() {
            let node = &mut nodes[idx];

            match &actions[idx] {
                NodeOptimizationAction::Trim(trims) => {
                    let mut text = node.as_text_mut().unwrap();

                    for action in trims {
                        match action {
                            TrimAction::Left => {
                                text.trim_start();
                            }
                            TrimAction::Right => {
                                text.trim_end();
                            }
                            TrimAction::LeftOneWhitespace => {
                                text.trim_start_one_whitespace(&self.b.ast.allocator);
                            }
                            TrimAction::RightOneWhitespace => {
                                text.trim_end_one_whitespace(&self.b.ast.allocator);
                            }
                        };
                    }
                }
                NodeOptimizationAction::Nope => (),
                NodeOptimizationAction::Remove => {
                    continue;
                }
            };

            new.push(node.clone());
        }

        *nodes = new;
    }
}
