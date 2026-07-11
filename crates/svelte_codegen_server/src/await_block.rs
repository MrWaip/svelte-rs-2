use oxc_ast::ast::{BindingPattern, Expression, FormalParameters, Statement};
use svelte_analyze::{
    AwaitBinding, AwaitBlockSemantics, AwaitBranch, AwaitWrapper, BlockSemantics, Volatility,
};
use svelte_ast::AwaitBlock;
use svelte_ast_builder::Arg;

use crate::error::{CodegenError, Result};
use crate::fragment::FragmentParent;
use crate::model::ServerCodegen;

impl<'a> ServerCodegen<'a> {
    pub(crate) fn await_block(&mut self, block: &'a AwaitBlock) -> Result<()> {
        let sem = match self.analysis.block_semantics(block.id) {
            BlockSemantics::Await(sem) => sem.clone(),
            _ => return Err(CodegenError::Unsupported(block.id, "await block")),
        };

        let expression = self.build_await_expression(block, &sem)?;
        let pending_fn = self.build_await_pending_fn(block)?;
        let then_fn = self.build_await_then_fn(block, &sem)?;

        let await_stmt = self.b.call_stmt(
            "$.await",
            [
                Arg::Ident("$$renderer"),
                Arg::Expr(expression),
                Arg::Expr(pending_fn),
                Arg::Expr(then_fn),
            ],
        );

        match &sem.wrapper {
            AwaitWrapper::None => self.push_stmt(await_stmt),
            AwaitWrapper::AsyncWrap { blockers } => {
                let is_async = sem.expression_volatility == Volatility::Asynchronous;
                let inner = self.b.arrow_block_expr_async(
                    self.b.params(["$$renderer"]),
                    [await_stmt],
                    is_async,
                );
                let wrapped = if blockers.is_empty() {
                    self.b
                        .call_stmt("$$renderer.child_block", [Arg::Expr(inner)])
                } else {
                    let blockers_expr = self.b.promises_array(blockers);
                    self.b.call_stmt(
                        "$$renderer.async_block",
                        [Arg::Expr(blockers_expr), Arg::Expr(inner)],
                    )
                };
                self.push_stmt(wrapped);
            }
        }

        self.push_text("<!--]-->");
        Ok(())
    }

    fn build_await_expression(
        &mut self,
        block: &'a AwaitBlock,
        sem: &AwaitBlockSemantics,
    ) -> Result<Expression<'a>> {
        let expr = self.take_expression(block.id, &block.expression)?;
        if sem.expression_volatility == Volatility::Asynchronous {
            let saved = self.save_block_await(expr);
            let arrow = self.b.async_arrow_expr_body(saved);
            Ok(self.b.call_expr_callee(arrow, []))
        } else {
            Ok(expr)
        }
    }

    fn build_await_pending_fn(&mut self, block: &'a AwaitBlock) -> Result<Expression<'a>> {
        let body = match block.pending {
            Some(fragment) => {
                self.child_statements(|cg| cg.fragment(fragment, FragmentParent::Block))?
            }
            None => Vec::new(),
        };
        Ok(self.b.arrow_block_expr(self.b.no_params(), body))
    }

    fn build_await_then_fn(
        &mut self,
        block: &'a AwaitBlock,
        sem: &AwaitBlockSemantics,
    ) -> Result<Expression<'a>> {
        let params = self.build_await_then_params(block, &sem.then)?;
        let body = match block.then {
            Some(fragment) => {
                self.child_statements(|cg| cg.fragment(fragment, FragmentParent::Block))?
            }
            None => Vec::new(),
        };
        Ok(self.b.arrow_block_expr(params, body))
    }

    fn build_await_then_params(
        &mut self,
        block: &'a AwaitBlock,
        then: &AwaitBranch,
    ) -> Result<FormalParameters<'a>> {
        let binding = match then {
            AwaitBranch::Present { binding } => binding,
            AwaitBranch::Absent => return Ok(self.b.no_params()),
        };
        match binding {
            AwaitBinding::None => Ok(self.b.no_params()),
            AwaitBinding::Identifier(sym) => {
                let name = self.analysis.scoping.symbol_name(*sym).to_string();
                Ok(self.b.params([name]))
            }
            AwaitBinding::Pattern { .. } => {
                let pattern = self.take_await_value_pattern(block)?;
                let param = self.b.formal_parameter_from_pattern(pattern);
                Ok(self.b.formal_parameters([param]))
            }
        }
    }

    fn take_await_value_pattern(&mut self, block: &AwaitBlock) -> Result<BindingPattern<'a>> {
        let Some(value_ref) = block.value.as_ref() else {
            return Err(CodegenError::MissingExpression(block.id));
        };
        let Some(stmt) = self.js_arena.take_stmt(value_ref.id()) else {
            return Err(CodegenError::MissingExpression(block.id));
        };
        let Statement::VariableDeclaration(mut decl) = stmt else {
            return Err(CodegenError::Unsupported(
                block.id,
                "await value declaration",
            ));
        };
        if decl.declarations.is_empty() {
            return Err(CodegenError::Unsupported(
                block.id,
                "await value declarator",
            ));
        }
        Ok(decl.declarations.remove(0).id)
    }
}
