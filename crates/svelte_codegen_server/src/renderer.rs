use std::mem;

use oxc_ast::ast::{Expression, Statement};
use svelte_ast_builder::{Arg, Builder, TemplatePart};

use crate::error::Result;
use crate::model::ServerCodegen;

pub(crate) enum TemplateItem<'a> {
    Text(String),
    Expr(Expression<'a>),
    Stmt(Statement<'a>),
}

impl<'a> ServerCodegen<'a> {
    pub(crate) fn push_text(&mut self, text: &str) {
        if let Some(TemplateItem::Text(prev)) = self.items.last_mut() {
            prev.push_str(text);
            return;
        }
        self.items.push(TemplateItem::Text(text.to_string()));
    }

    pub(crate) fn push_stmt(&mut self, stmt: Statement<'a>) {
        self.items.push(TemplateItem::Stmt(stmt));
    }

    pub(crate) fn renderer_push_string_stmt(&self, literal: &str) -> Statement<'a> {
        self.b
            .call_stmt("$$renderer.push", [Arg::Str(literal.to_string())])
    }

    pub(crate) fn renderer_push_template_stmt(&self, text: &str) -> Statement<'a> {
        let template = self
            .b
            .template_parts_expr(vec![TemplatePart::Str(text.to_string())]);
        self.b
            .expr_stmt(self.b.call_expr("$$renderer.push", [Arg::Expr(template)]))
    }

    pub(crate) fn push_expr(&mut self, expression: Expression<'a>) {
        self.items.push(TemplateItem::Expr(expression));
    }

    pub(crate) fn take_renderer_statements(&mut self) -> Vec<Statement<'a>> {
        let items = mem::take(&mut self.items);
        renderer_statements(&self.b, items)
    }

    pub(crate) fn child_statements<F>(&mut self, fill: F) -> Result<Vec<Statement<'a>>>
    where
        F: FnOnce(&mut Self) -> Result<()>,
    {
        let parent_items = mem::take(&mut self.items);
        let outcome = fill(self);
        let child_items = mem::replace(&mut self.items, parent_items);
        outcome?;
        Ok(renderer_statements(&self.b, child_items))
    }
}

fn renderer_statements<'a>(b: &Builder<'a>, items: Vec<TemplateItem<'a>>) -> Vec<Statement<'a>> {
    let mut statements = Vec::new();
    let mut parts: Vec<TemplatePart<'a>> = Vec::new();
    for item in items {
        match item {
            TemplateItem::Text(text) => parts.push(TemplatePart::Str(text)),
            TemplateItem::Expr(expression) => parts.push(TemplatePart::Expr(expression, true)),
            TemplateItem::Stmt(statement) => {
                flush_push(b, &mut parts, &mut statements);
                statements.push(statement);
            }
        }
    }
    flush_push(b, &mut parts, &mut statements);
    statements
}

fn flush_push<'a>(
    b: &Builder<'a>,
    parts: &mut Vec<TemplatePart<'a>>,
    statements: &mut Vec<Statement<'a>>,
) {
    if parts.is_empty() {
        return;
    }
    let template = b.template_parts_expr(mem::take(parts));
    statements.push(b.expr_stmt(b.call_expr("$$renderer.push", [Arg::Expr(template)])));
}
