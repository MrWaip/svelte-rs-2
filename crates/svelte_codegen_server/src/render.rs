use oxc_ast::ast::{Argument, ChainElement, Expression};
use svelte_analyze::{BlockSemantics, RenderAsyncKind, RenderCallKind};
use svelte_ast::RenderTag;
use svelte_ast_builder::Arg;

use crate::error::{CodegenError, Result};
use crate::model::ServerCodegen;

impl<'a> ServerCodegen<'a> {
    pub(crate) fn render_tag(&mut self, tag: &'a RenderTag, is_standalone: bool) -> Result<()> {
        let sem = match self.analysis.block_semantics(tag.id) {
            BlockSemantics::Render(sem) => sem.clone(),
            _ => return Err(CodegenError::Unsupported(tag.id, "render tag")),
        };
        if matches!(sem.async_kind, RenderAsyncKind::Async { .. }) {
            return Err(CodegenError::Unsupported(tag.id, "async render tag"));
        }

        let expr = self.take_expression(tag.id, &tag.expression)?;
        let call = match expr.into_inner_expression() {
            Expression::CallExpression(call) => call.unbox(),
            Expression::ChainExpression(chain) => match chain.unbox().expression {
                ChainElement::CallExpression(call) => call.unbox(),
                _ => return Err(CodegenError::Unsupported(tag.id, "render tag call")),
            },
            _ => return Err(CodegenError::Unsupported(tag.id, "render tag call")),
        };

        let callee = call.callee;
        let mut args: Vec<Arg<'a, '_>> = vec![Arg::Ident("$$renderer")];
        for argument in call.arguments {
            match argument {
                Argument::SpreadElement(spread) => {
                    args.push(Arg::Spread(spread.unbox().argument));
                }
                other => args.push(Arg::Expr(other.into_expression())),
            }
        }

        let final_call = match sem.call_kind {
            RenderCallKind::Plain => self.b.call_expr_callee(callee, args),
            RenderCallKind::OptionalChain => self.b.maybe_call_expr(callee, args),
        };
        let stmt = self.b.expr_stmt(final_call);
        self.push_stmt(stmt);

        if !is_standalone {
            self.push_text("<!---->");
        }
        Ok(())
    }
}
