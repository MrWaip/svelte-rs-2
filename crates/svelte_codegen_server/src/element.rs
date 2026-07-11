use std::iter::empty;

use oxc_allocator::CloneIn;
use oxc_ast::ast::{Expression, Statement};
use svelte_analyze::{
    ContentEditableKind, ElementAsyncKind, ElementSemantics, ElementValueRole, TextareaBody,
    TextareaSegment,
};
use svelte_ast::{Attribute, Element, Node, NodeId};
use svelte_ast_builder::{Arg, TemplatePart};

use crate::error::{CodegenError, Result};
use crate::fragment::FragmentParent;
use crate::model::ServerCodegen;

impl<'a> ServerCodegen<'a> {
    pub(crate) fn element(&mut self, element: &'a Element) -> Result<()> {
        let async_kind = match self.analysis.element_semantics.query(element.id) {
            ElementSemantics::RegularElement(sem) => sem.async_kind.clone(),
            _ => ElementAsyncKind::Sync,
        };
        if async_kind.is_sync() {
            return self.emit_element_inline(element);
        }
        let blockers = async_kind.blockers().to_vec();
        let awaited = async_kind.awaited();

        let (stmts, hoists) = self
            .with_promise_hoisting(|cg| cg.child_statements(|c| c.emit_element_inline(element)));
        let stmts = stmts?;

        let mut body = hoists;
        body.extend(stmts);
        let arrow = self
            .b
            .arrow_block_expr_async(self.b.params(["$$renderer"]), body, awaited);
        let call = self.wrap_arrow(arrow, &blockers, "$$renderer.child", "$$renderer.async");
        self.push_stmt(call);
        Ok(())
    }

    fn value_role(&self, id: NodeId) -> ElementValueRole {
        match self.analysis.element_semantics.query(id) {
            ElementSemantics::RegularElement(sem) => sem.value_role.clone(),
            _ => ElementValueRole::Plain,
        }
    }

    fn emit_replay_events(&mut self, id: NodeId) {
        let events: Vec<&'static str> = match self.analysis.element_semantics.query(id) {
            ElementSemantics::RegularElement(sem) => sem
                .replay_events
                .iter()
                .map(|e| e.attribute_name())
                .collect(),
            _ => Vec::new(),
        };
        for event in events {
            self.push_text(&format!(" {event}=\"this.__e=event\""));
        }
    }

    fn emit_element_inline(&mut self, element: &'a Element) -> Result<()> {
        let value_role = self.value_role(element.id);
        match &value_role {
            ElementValueRole::Select { rich } => {
                return self.emit_select_container(element, *rich);
            }
            ElementValueRole::Option { value, rich } => {
                return self.emit_option_container(element, *value, *rich);
            }
            ElementValueRole::RawText => {
                if let Some(raw_content) = self.raw_text_string(element) {
                    return self.emit_raw_text_element(element, raw_content);
                }
            }
            ElementValueRole::Plain
            | ElementValueRole::RichContainer
            | ElementValueRole::TextareaValue { .. }
            | ElementValueRole::ContentEditable { .. } => {}
        }

        let emits_const_tags = match &value_role {
            ElementValueRole::Plain
            | ElementValueRole::RichContainer
            | ElementValueRole::RawText => true,
            ElementValueRole::Select { .. }
            | ElementValueRole::Option { .. }
            | ElementValueRole::TextareaValue { .. }
            | ElementValueRole::ContentEditable { .. } => false,
        };
        if emits_const_tags {
            self.emit_fragment_const_tags_hoisted(element.fragment)?;
        }

        self.push_text(&format!("<{}", element.name));
        self.emit_element_attributes(element.id, &element.attributes)?;
        self.emit_replay_events(element.id);
        if self.analysis.is_void(element.id) {
            self.push_text("/>");
        } else {
            self.push_text(">");
        }

        match &value_role {
            ElementValueRole::TextareaValue { body } => {
                if self.dev {
                    let push_element = self.push_element_stmt(element);
                    self.push_stmt(push_element);
                }
                self.emit_textarea_value_body(element.id, body)?
            }
            ElementValueRole::ContentEditable { bind_id, kind } => {
                if self.dev {
                    let push_element = self.push_element_stmt(element);
                    self.push_stmt(push_element);
                }
                self.emit_contenteditable_body(element, *bind_id, *kind)?
            }
            ElementValueRole::Plain
            | ElementValueRole::RichContainer
            | ElementValueRole::RawText
            | ElementValueRole::Select { .. }
            | ElementValueRole::Option { .. } => {
                self.emit_fragment_snippets_debug_head(element.fragment)?;
                if self.dev {
                    let push_element = self.push_element_stmt(element);
                    self.push_stmt(push_element);
                }
                self.fragment_children_only(element.fragment, FragmentParent::Element(element))?;
            }
        }

        let needs_rich_marker = match value_role {
            ElementValueRole::RichContainer => true,
            ElementValueRole::Plain
            | ElementValueRole::RawText
            | ElementValueRole::Select { .. }
            | ElementValueRole::Option { .. }
            | ElementValueRole::TextareaValue { .. }
            | ElementValueRole::ContentEditable { .. } => false,
        };
        if needs_rich_marker {
            self.push_text("<!>");
        }

        if !self.analysis.is_void(element.id) {
            self.push_text(&format!("</{}>", element.name));
        }

        if self.dev {
            let pop_element = self.pop_element_stmt();
            self.push_stmt(pop_element);
        }
        Ok(())
    }

    fn emit_select_container(&mut self, element: &'a Element, rich: bool) -> Result<()> {
        let (object, optionals) =
            self.build_element_attribute_object(element.id, &element.attributes)?;
        let body = self
            .child_statements(|c| c.fragment(element.fragment, FragmentParent::Element(element)))?;
        let arrow = self.b.arrow_block_expr(self.b.params(["$$renderer"]), body);
        let mut args = vec![Arg::Expr(object), Arg::Expr(arrow)];
        args.extend(self.container_trailing_args(optionals, rich));
        let call = self.b.call_stmt("$$renderer.select", args);
        self.push_stmt(call);
        Ok(())
    }

    fn emit_option_container(
        &mut self,
        element: &'a Element,
        value: Option<NodeId>,
        rich: bool,
    ) -> Result<()> {
        let (object, optionals) =
            self.build_element_attribute_object(element.id, &element.attributes)?;
        let body = match value {
            Some(node) => self.take_expression_tag(node)?,
            None => {
                let stmts = self.child_statements(|c| {
                    if c.dev {
                        let push_element = c.push_element_stmt(element);
                        c.push_stmt(push_element);
                    }
                    c.fragment(element.fragment, FragmentParent::Element(element))?;
                    if c.dev {
                        let pop_element = c.pop_element_stmt();
                        c.push_stmt(pop_element);
                    }
                    Ok(())
                })?;
                self.b
                    .arrow_block_expr(self.b.params(["$$renderer"]), stmts)
            }
        };
        let mut args = vec![Arg::Expr(object), Arg::Expr(body)];
        args.extend(self.container_trailing_args(optionals, rich));
        let call = self.b.call_stmt("$$renderer.option", args);
        self.push_stmt(call);
        Ok(())
    }

    fn emit_contenteditable_body(
        &mut self,
        element: &'a Element,
        bind_id: NodeId,
        kind: ContentEditableKind,
    ) -> Result<()> {
        let expression = self.contenteditable_bind_expr(element, bind_id)?;
        let escape = match kind {
            ContentEditableKind::InnerText | ContentEditableKind::TextContent => true,
            ContentEditableKind::InnerHtml => false,
        };

        if escape {
            let escaped = self.b.call_expr("$.escape", [Arg::Expr(expression)]);
            let body_name: &str = self.b.alloc_str(&self.ident_gen.generate("$$body"));
            let body_decl = self.b.const_stmt(body_name, escaped);
            let else_block = self.contenteditable_else_block(element)?;
            let template = self
                .b
                .template_parts_expr([TemplatePart::Expr(self.b.rid_expr(body_name), true)]);
            let push = self.b.call_stmt("$$renderer.push", [Arg::Expr(template)]);
            let if_stmt = self.b.if_stmt(
                self.b.rid_expr(body_name),
                self.b.block_stmt(vec![push]),
                Some(else_block),
            );
            self.push_stmt(body_decl);
            self.push_stmt(if_stmt);
        } else {
            let test = expression.clone_in(self.b.ast.allocator);
            let else_block = self.contenteditable_else_block(element)?;
            let template = self
                .b
                .template_parts_expr([TemplatePart::Expr(expression, true)]);
            let push = self.b.call_stmt("$$renderer.push", [Arg::Expr(template)]);
            let if_stmt = self
                .b
                .if_stmt(test, self.b.block_stmt(vec![push]), Some(else_block));
            self.push_stmt(if_stmt);
        }
        Ok(())
    }

    fn contenteditable_bind_expr(
        &mut self,
        element: &'a Element,
        bind_id: NodeId,
    ) -> Result<Expression<'a>> {
        for attr in &element.attributes {
            if attr.id() != bind_id {
                continue;
            }
            if let Attribute::BindDirective(d) = attr {
                return self.take_expression(d.id, &d.expression);
            }
        }
        Err(CodegenError::MissingExpression(bind_id))
    }

    fn contenteditable_else_block(&mut self, element: &'a Element) -> Result<Statement<'a>> {
        let body = self.child_statements(|cg| {
            cg.fragment_children_only(element.fragment, FragmentParent::Element(element))
        })?;
        Ok(self.b.block_stmt(body))
    }

    fn emit_textarea_value_body(&mut self, owner: NodeId, body: &TextareaBody) -> Result<()> {
        let expression = match body {
            TextareaBody::Single(oxc_id) => self.take_expr_by_oxc_id(owner, *oxc_id)?,
            TextareaBody::Segments(segments) => self.textarea_segments_expr(segments)?,
        };
        let escaped = self.b.call_expr("$.escape", [Arg::Expr(expression)]);
        let body_name: &str = self.b.alloc_str(&self.ident_gen.generate("$$body"));
        let body_decl = self.b.const_stmt(body_name, escaped);

        let template = self
            .b
            .template_parts_expr([TemplatePart::Expr(self.b.rid_expr(body_name), true)]);
        let push = self.b.call_stmt("$$renderer.push", [Arg::Expr(template)]);
        let if_stmt = self.b.if_stmt(
            self.b.rid_expr(body_name),
            self.b.block_stmt(vec![push]),
            Some(self.b.block_stmt(vec![])),
        );

        self.push_stmt(body_decl);
        self.push_stmt(if_stmt);
        Ok(())
    }

    fn textarea_segments_expr(&mut self, segments: &[TextareaSegment]) -> Result<Expression<'a>> {
        let mut parts: Vec<TemplatePart<'a>> = Vec::with_capacity(segments.len());
        for segment in segments {
            match segment {
                TextareaSegment::Text(text) => parts.push(TemplatePart::Str(text.clone())),
                TextareaSegment::Expression { node_id, oxc_id } => {
                    let is_defined_string = self
                        .analysis
                        .expression_data(*node_id)
                        .is_some_and(|data| data.declared_evaluation.is_defined_string());
                    let expr = self.take_expr_by_oxc_id(*node_id, *oxc_id)?;
                    if is_defined_string {
                        parts.push(TemplatePart::Expr(expr, true));
                    } else {
                        let value = self.b.call_expr("$.stringify", [Arg::Expr(expr)]);
                        parts.push(TemplatePart::Expr(value, true));
                    }
                }
            }
        }

        Ok(self.b.template_parts_expr(parts))
    }

    fn container_trailing_args(
        &self,
        optionals: [Option<Expression<'a>>; 4],
        rich: bool,
    ) -> Vec<Arg<'a, 'a>> {
        let mut tail: Vec<Option<Expression<'a>>> = optionals.into_iter().collect();
        if rich {
            tail.push(Some(self.b.bool_expr(true)));
        }
        self.optional_trailing_args(tail)
    }

    fn take_expression_tag(&mut self, node: NodeId) -> Result<Expression<'a>> {
        let Node::ExpressionTag(tag) = self.component.store.get(node) else {
            return Err(CodegenError::Unsupported(node, "expression tag value"));
        };
        self.take_expression(tag.id, &tag.expression)
    }

    fn emit_raw_text_element(&mut self, element: &'a Element, raw_content: String) -> Result<()> {
        let name = element.name.clone();
        let raw_statements = self.child_statements(|codegen| {
            codegen.push_text(&format!("<{name}"));
            codegen.emit_element_attributes(element.id, &element.attributes)?;
            codegen.emit_replay_events(element.id);
            codegen.push_text(&format!(">{raw_content}</{name}>"));
            Ok(())
        })?;
        for stmt in raw_statements {
            self.push_stmt(stmt);
        }
        Ok(())
    }

    fn raw_text_string(&self, element: &Element) -> Option<String> {
        let nodes = self.component.store.fragment_nodes(element.fragment);
        if nodes.is_empty() {
            return Some(String::new());
        }
        let [only] = nodes else {
            return None;
        };
        let Node::Text(text) = self.component.store.get(*only) else {
            return None;
        };
        Some(text.value(self.component.source.as_str()).to_string())
    }

    fn push_element_stmt(&self, element: &Element) -> Statement<'a> {
        let (line, col) = self.line_index.line_col(element.span.start);
        let name: &str = self.b.alloc_str(&element.name);
        self.b.expr_stmt(self.b.call_expr(
            "$.push_element",
            [
                Arg::Ident("$$renderer"),
                Arg::StrRef(name),
                Arg::Num(line as f64),
                Arg::Num(col as f64),
            ],
        ))
    }

    fn pop_element_stmt(&self) -> Statement<'a> {
        self.b
            .expr_stmt(self.b.call_expr("$.pop_element", empty::<Arg<'_, '_>>()))
    }
}
