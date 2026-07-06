use oxc_ast::ast::{Expression, Statement};
use oxc_syntax::node::NodeId as OxcNodeId;
use std::collections::HashMap;
use svelte_analyze::{AnalysisData, IdentGen, JsAst};
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
    pub ident_gen: &'a mut IdentGen,
    pub line_index: &'a LineIndex,
    pub dev: bool,
    pub filename: &'a str,
    pub items: Vec<TemplateItem<'a>>,
    pub hoisted: Vec<Statement<'a>>,
    pub svelte_element_tag_refs: HashMap<NodeId, String>,
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
            ident_gen: ctx.ident_gen,
            line_index: ctx.line_index,
            dev: options.dev,
            filename,
            items: Vec::new(),
            hoisted: Vec::new(),
            svelte_element_tag_refs: HashMap::new(),
        }
    }

    pub(crate) fn gen_ident(&mut self, prefix: &str) -> String {
        self.ident_gen.generate(prefix)
    }

    pub(crate) fn take_expression(
        &mut self,
        node_id: NodeId,
        expr_ref: &ExprRef,
    ) -> Result<Expression<'a>> {
        self.take_expr_by_oxc_id(node_id, expr_ref.id())
    }

    pub(crate) fn take_expr_by_oxc_id(
        &mut self,
        node_id: NodeId,
        oxc_id: OxcNodeId,
    ) -> Result<Expression<'a>> {
        self.js_arena
            .take_expr(oxc_id)
            .ok_or(CodegenError::MissingExpression(node_id))
    }

    pub(crate) fn expression_is_volatile(&self, node_id: NodeId) -> bool {
        self.analysis
            .expression_data(node_id)
            .is_some_and(|data| data.volatility.is_volatile())
    }
}
