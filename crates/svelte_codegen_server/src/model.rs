use oxc_ast::ast::Expression;
use svelte_analyze::{AnalysisData, JsAst};
use svelte_ast::{Component, ExprRef, NodeId};
use svelte_ast_builder::Builder;
use svelte_span::LineIndex;

use crate::error::{CodegenError, Result};
use crate::renderer::TemplateItem;

pub(crate) struct ServerCodegen<'a> {
    pub b: Builder<'a>,
    pub component: &'a Component,
    pub analysis: &'a AnalysisData<'a>,
    pub js_arena: &'a mut JsAst<'a>,
    pub line_index: &'a LineIndex,
    pub dev: bool,
    pub filename: &'a str,
    pub items: Vec<TemplateItem<'a>>,
}

impl<'a> ServerCodegen<'a> {
    pub(crate) fn new(
        ctx: svelte_types::CompileContext<'a, 'a>,
        options: &svelte_types::CodegenOptions,
    ) -> Self {
        let b = Builder::new(ctx.alloc);
        let filename: &'a str = b.alloc_str(&options.filename);
        Self {
            b,
            component: ctx.component,
            analysis: ctx.analysis,
            js_arena: ctx.js_arena,
            line_index: ctx.line_index,
            dev: options.dev,
            filename,
            items: Vec::new(),
        }
    }

    pub(crate) fn take_expression(
        &mut self,
        node_id: NodeId,
        expr_ref: &ExprRef,
    ) -> Result<Expression<'a>> {
        match self.js_arena.take_expr(expr_ref.id()) {
            Some(expression) => Ok(expression),
            None => Err(CodegenError::MissingExpression(node_id)),
        }
    }
}
