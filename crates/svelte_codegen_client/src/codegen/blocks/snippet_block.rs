use std::mem;

use oxc_ast::ast::{BindingPattern, Expression, FormalParameter, FormalParameters, Statement};
use oxc_span::SPAN;
use svelte_analyze::{SnippetBlockSemantics, SnippetPlacement};
use svelte_ast::NodeId;
use svelte_ast_builder::Arg;

use super::super::data_structures::EmitState;
use super::super::data_structures::{FragmentAnchor, FragmentCtx};
use super::super::{Codegen, CodegenError, Result};

#[derive(Default)]
pub(in crate::codegen) struct SnippetParamParsed<'a> {
    pub pattern: Option<BindingPattern<'a>>,
    pub default: Option<Expression<'a>>,
}

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(in super::super) fn emit_snippet_block(
        &mut self,
        state: &mut EmitState<'a>,
        _ctx: &FragmentCtx<'a>,
        id: NodeId,
        sem: SnippetBlockSemantics,
    ) -> Result<()> {
        if state.skip_snippets {
            return Ok(());
        }
        let stmt = self.build_snippet_const(id, &sem)?;
        match sem.placement {
            SnippetPlacement::ModuleLevel => self.hoistable_snippets.push(stmt),
            SnippetPlacement::InstanceLevel | SnippetPlacement::Local => {
                self.instance_snippets.push(stmt)
            }
        }
        Ok(())
    }

    pub(in super::super) fn build_snippet_const(
        &mut self,
        block_id: NodeId,
        sem: &SnippetBlockSemantics,
    ) -> Result<Statement<'a>> {
        self.build_snippet_const_with_prefix(block_id, sem, Vec::new())
    }

    pub(in super::super) fn build_snippet_const_with_prefix(
        &mut self,
        block_id: NodeId,
        sem: &SnippetBlockSemantics,
        prepend_stmts: Vec<Statement<'a>>,
    ) -> Result<Statement<'a>> {
        self.build_snippet_const_inner(block_id, sem, prepend_stmts)
    }

    fn build_snippet_const_inner(
        &mut self,
        block_id: NodeId,
        sem: &SnippetBlockSemantics,
        prepend_stmts: Vec<Statement<'a>>,
    ) -> Result<Statement<'a>> {
        let name = self.ctx.query.view.symbol_name(sem.name).to_string();

        let block = match self.ctx.query.component.store.get(block_id) {
            svelte_ast::Node::SnippetBlock(b) => b,
            _ => return CodegenError::unexpected_node(block_id, "SnippetBlock"),
        };
        let Some(mut parsed_stmt) = self.ctx.state.parsed.take_stmt(block.decl.id()) else {
            return CodegenError::missing_expression(block_id);
        };

        let parsed_patterns = self.take_snippet_param_patterns(&mut parsed_stmt, sem.params.len());

        let (params, binding_decls) =
            self.build_snippet_params_with_patterns(block_id, sem, parsed_patterns)?;

        let body = match self.ctx.query.component.store.get(block_id) {
            svelte_ast::Node::SnippetBlock(block) => block.body,
            _ => return CodegenError::unexpected_node(block_id, "SnippetBlock"),
        };
        let mut inner_ctx = FragmentCtx::root(self.ctx, body);
        inner_ctx.anchor = FragmentAnchor::callback_param("$$anchor", false);
        let mut inner_state = EmitState::new();
        self.emit_fragment(&mut inner_state, &inner_ctx, body)?;
        let body_stmts = self.pack_callback_body(inner_state, "$$anchor")?;

        let mut all: Vec<Statement<'a>> = Vec::new();
        all.extend(prepend_stmts);
        if self.ctx.state.dev {
            let args_id = self.ctx.b.rid_expr("arguments");
            all.push(
                self.ctx
                    .b
                    .call_stmt("$.validate_snippet_args", [Arg::Spread(args_id)]),
            );
        }
        all.extend(binding_decls);
        all.extend(body_stmts);

        let snippet_expr = if self.ctx.state.dev {
            let fn_expr = self.ctx.b.function_expr(params, all);
            let component = self.ctx.b.rid_expr(self.ctx.state.name);
            self.ctx
                .b
                .call_expr("$.wrap_snippet", [Arg::Expr(component), Arg::Expr(fn_expr)])
        } else {
            let arrow = self.ctx.b.arrow_block(params, all);
            Expression::ArrowFunctionExpression(self.ctx.b.alloc(arrow))
        };
        Ok(self.ctx.b.const_stmt(&name, snippet_expr))
    }

    fn build_snippet_params_with_patterns(
        &mut self,
        _block_id: NodeId,
        sem: &SnippetBlockSemantics,
        mut parsed_patterns: Vec<SnippetParamParsed<'a>>,
    ) -> Result<(FormalParameters<'a>, Vec<Statement<'a>>)> {
        use oxc_ast::ast::FormalParameterKind;

        let mut params: Vec<FormalParameter<'a>> = Vec::new();
        params.push(self.formal_param_ident("$$anchor", false));

        let mut binding_decls: Vec<Statement<'a>> = Vec::new();

        for (idx, param) in sem.params.iter().enumerate() {
            let (pattern, default) = match parsed_patterns.get_mut(idx) {
                Some(slot) => (slot.pattern.take(), slot.default.take()),
                None => (None, None),
            };
            let (formal, mut stmts) = self.emit_snippet_param(param, idx, pattern, default)?;
            params.push(formal);
            binding_decls.append(&mut stmts);
        }

        let params = self.ctx.b.ast.formal_parameters(
            SPAN,
            FormalParameterKind::ArrowFormalParameters,
            self.ctx.b.ast.vec_from_iter(params),
            oxc_ast::NONE,
        );
        Ok((params, binding_decls))
    }

    pub(in crate::codegen) fn formal_param_ident(
        &self,
        name: &str,
        with_noop_default: bool,
    ) -> FormalParameter<'a> {
        let inner = self
            .ctx
            .b
            .ast
            .binding_pattern_binding_identifier(SPAN, self.ctx.b.ast.atom(name));
        let pattern = if with_noop_default {
            let default_expr = self
                .ctx
                .b
                .static_member_expr(self.ctx.b.rid_expr("$"), "noop");
            self.ctx
                .b
                .ast
                .binding_pattern_assignment_pattern(SPAN, inner, default_expr)
        } else {
            inner
        };
        self.ctx.b.ast.formal_parameter(
            SPAN,
            self.ctx.b.ast.vec(),
            pattern,
            oxc_ast::NONE,
            oxc_ast::NONE,
            false,
            None,
            false,
            false,
        )
    }

    fn take_snippet_param_patterns(
        &mut self,
        stmt: &mut Statement<'a>,
        expected: usize,
    ) -> Vec<SnippetParamParsed<'a>> {
        let mut out: Vec<SnippetParamParsed<'a>> = Vec::with_capacity(expected);
        let arrow = match stmt {
            Statement::VariableDeclaration(decl) => decl
                .declarations
                .first_mut()
                .and_then(|d| d.init.as_mut())
                .and_then(|init| match init {
                    Expression::ArrowFunctionExpression(arrow) => Some(arrow),
                    _ => None,
                }),
            _ => None,
        };
        let Some(arrow) = arrow else {
            out.resize_with(expected, SnippetParamParsed::default);
            return out;
        };
        for param in arrow.params.items.iter_mut().take(expected) {
            let default = param.initializer.take().map(|init| init.unbox());
            let pattern = if matches!(param.pattern, BindingPattern::BindingIdentifier(_)) {
                None
            } else {
                let dummy = self
                    .ctx
                    .b
                    .ast
                    .binding_pattern_binding_identifier(SPAN, self.ctx.b.ast.atom("$$snip"));
                Some(mem::replace(&mut param.pattern, dummy))
            };
            out.push(SnippetParamParsed { pattern, default });
        }
        out.resize_with(expected, SnippetParamParsed::default);
        out
    }
}
