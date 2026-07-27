use oxc_ast::ast::Expression;
use svelte_analyze::Suspension;

use super::Ctx;
use super::effect::suspending_thunk;

pub(in crate::codegen) struct AsyncValues<'a> {
    base: u32,
    values: Vec<(Expression<'a>, Suspension)>,
}

impl<'a> AsyncValues<'a> {
    pub(in crate::codegen) fn new(base: u32) -> Self {
        Self {
            base,
            values: Vec::new(),
        }
    }

    pub(in crate::codegen) fn push(
        &mut self,
        ctx: &Ctx<'a>,
        expr: Expression<'a>,
        suspension: Suspension,
    ) -> &'a str {
        let index = self.base + self.values.len() as u32;
        self.values.push((expr, suspension));
        ctx.b.alloc_str(&format!("${index}"))
    }

    pub(in crate::codegen) fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub(in crate::codegen) fn ids(&self) -> Vec<String> {
        (0..self.values.len())
            .map(|i| format!("${}", self.base + i as u32))
            .collect()
    }

    pub(in crate::codegen) fn into_thunks(self, ctx: &Ctx<'a>) -> Vec<Expression<'a>> {
        self.values
            .into_iter()
            .map(|(expr, suspension)| suspending_thunk(ctx, expr, suspension))
            .collect()
    }
}
