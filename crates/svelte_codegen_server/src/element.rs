use std::iter::empty;

use oxc_ast::ast::{Expression, Statement};
use svelte_analyze::{ElementAsyncKind, ElementSemantics, ElementValueRole};
use svelte_ast::{Element, Node, NodeId, is_void};
use svelte_ast_builder::{Arg, TemplatePart};

use crate::error::{CodegenError, Result};
use crate::fragment::FragmentParent;
use crate::model::ServerCodegen;

impl<'a> ServerCodegen<'a> {
    pub(crate) fn element(
        &mut self,
        element: &'a Element,
        preserve_whitespace: bool,
    ) -> Result<()> {
        let async_kind = match self.analysis.element_semantics.query(element.id) {
            ElementSemantics::RegularElement(sem) => sem.async_kind.clone(),
            _ => ElementAsyncKind::Sync,
        };
        if async_kind.is_sync() {
            return self.emit_element_inline(element, preserve_whitespace);
        }
        let blockers = async_kind.blockers().to_vec();
        let awaited = async_kind.awaited();

        let (stmts, hoists) = self.with_promise_hoisting(|cg| {
            cg.child_statements(|c| c.emit_element_inline(element, preserve_whitespace))
        });
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
            ElementSemantics::RegularElement(sem) => sem.value_role,
            _ => ElementValueRole::Plain,
        }
    }

    fn emit_element_inline(
        &mut self,
        element: &'a Element,
        preserve_whitespace: bool,
    ) -> Result<()> {
        if let Some(raw_content) = self.raw_text_content(element) {
            let name = element.name.clone();
            let raw_statements = self.child_statements(|codegen| {
                codegen.push_text(&format!("<{name}"));
                codegen.emit_element_attributes(element.id, &element.attributes)?;
                codegen.push_text(&format!(">{raw_content}</{name}>"));
                Ok(())
            })?;
            for stmt in raw_statements {
                self.push_stmt(stmt);
            }
            return Ok(());
        }

        let value_role = self.value_role(element.id);
        match value_role {
            ElementValueRole::Select { rich } => {
                return self.emit_select_container(element, preserve_whitespace, rich);
            }
            ElementValueRole::Option { value, rich } => {
                return self.emit_option_container(element, preserve_whitespace, value, rich);
            }
            ElementValueRole::Plain
            | ElementValueRole::RichContainer
            | ElementValueRole::TextareaValue { .. } => {}
        }

        self.push_text(&format!("<{}", element.name));
        self.emit_element_attributes(element.id, &element.attributes)?;
        if is_void(&element.name) {
            self.push_text("/>");
        } else {
            self.push_text(">");
        }

        if self.dev {
            let push_element = self.push_element_stmt(element);
            self.push_stmt(push_element);
        }

        match value_role {
            ElementValueRole::TextareaValue { value } => self.emit_textarea_value_body(value)?,
            ElementValueRole::Plain
            | ElementValueRole::RichContainer
            | ElementValueRole::Select { .. }
            | ElementValueRole::Option { .. } => {
                let child_preserve_whitespace =
                    preserve_whitespace || element.name == "pre" || element.name == "textarea";
                self.fragment(
                    element.fragment,
                    FragmentParent::Element(element),
                    child_preserve_whitespace,
                )?;
            }
        }

        let needs_rich_marker = match value_role {
            ElementValueRole::RichContainer => true,
            ElementValueRole::Plain
            | ElementValueRole::Select { .. }
            | ElementValueRole::Option { .. }
            | ElementValueRole::TextareaValue { .. } => false,
        };
        if needs_rich_marker {
            self.push_text("<!>");
        }

        if !is_void(&element.name) {
            self.push_text(&format!("</{}>", element.name));
        }

        if self.dev {
            let pop_element = self.pop_element_stmt();
            self.push_stmt(pop_element);
        }
        Ok(())
    }

    fn emit_select_container(
        &mut self,
        element: &'a Element,
        preserve_whitespace: bool,
        rich: bool,
    ) -> Result<()> {
        let (object, optionals) =
            self.build_element_attribute_object(element.id, &element.attributes)?;
        let body = self.child_statements(|c| {
            c.fragment(
                element.fragment,
                FragmentParent::Element(element),
                preserve_whitespace,
            )
        })?;
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
        preserve_whitespace: bool,
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
                    c.fragment(
                        element.fragment,
                        FragmentParent::Element(element),
                        preserve_whitespace,
                    )?;
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

    fn emit_textarea_value_body(&mut self, value: NodeId) -> Result<()> {
        let expression = self.take_expression_tag(value)?;
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

    fn raw_text_content(&self, element: &Element) -> Option<String> {
        if element.name != "script" && element.name != "style" {
            return None;
        }
        let nodes = self.component.store.fragment_nodes(element.fragment);
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
