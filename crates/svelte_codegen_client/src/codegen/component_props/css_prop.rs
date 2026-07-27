use oxc_ast::ast::Statement;
use svelte_analyze::{ComponentCssPropValue, ConcatPartEmit, Volatility};
use svelte_ast::{Attribute, NodeId};
use svelte_ast_builder::{Arg, ObjProp};
use svelte_emit_builders::runes::rune_get;

use super::super::async_values::AsyncValues;
use super::super::expr::coarse_wrap;
use super::super::{Codegen, CodegenError, Result};

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(super) fn sync_memo_slots_of_css_prop(
        &self,
        attr_id: NodeId,
        value: &ComponentCssPropValue,
    ) -> u32 {
        match value {
            ComponentCssPropValue::Expression(_) => match self.css_prop_volatility(attr_id) {
                Volatility::Heavy => 1,
                Volatility::Static | Volatility::Reactive | Volatility::Asynchronous => 0,
            },
            ComponentCssPropValue::Concatenation(plan) => plan
                .iter()
                .map(|emit| match emit {
                    ConcatPartEmit::HoistDerived => 1,
                    ConcatPartEmit::Awaited | ConcatPartEmit::Inline | ConcatPartEmit::Static => 0,
                })
                .sum(),
            ComponentCssPropValue::StaticString(_) | ComponentCssPropValue::Boolean => 0,
        }
    }

    fn css_prop_volatility(&self, attr_id: NodeId) -> Volatility {
        self.ctx
            .expression_data(attr_id)
            .map(|data| data.volatility)
            .unwrap_or(Volatility::Static)
    }

    pub(super) fn emit_component_css_prop(
        &mut self,
        attr: &'a Attribute,
        value: &ComponentCssPropValue,
        css_props: &mut Vec<ObjProp<'a>>,
        memo_decls: &mut Vec<Statement<'a>>,
        memo_counter: &mut u32,
        async_values: &mut AsyncValues<'a>,
    ) -> Result<()> {
        let attr_id = attr.id();
        let Some(name) = attr.name() else {
            return CodegenError::semantic_mismatch(attr_id, "css property requires a name");
        };
        let key = self.ctx.b.alloc_str(name);
        let expr = match value {
            ComponentCssPropValue::Expression(expression_id) => {
                let Some(expression) = self.ctx.state.parsed.take_expr(*expression_id) else {
                    return CodegenError::missing_expression(attr_id);
                };
                let data = self.ctx.expression_data(attr_id).cloned();
                let expression = coarse_wrap(self.ctx, expression, data.as_ref());
                let volatility = data
                    .as_ref()
                    .map(|d| d.volatility)
                    .unwrap_or(Volatility::Static);
                match volatility {
                    Volatility::Asynchronous => {
                        let suspension = data.map(|d| d.suspension).unwrap_or_default();
                        let memo_ref = async_values.push(self.ctx, expression, suspension);
                        rune_get(&self.ctx.b, memo_ref)
                    }
                    Volatility::Heavy => {
                        let helper = self.ctx.query.view.derived_helper();
                        let memo_name = format!("${memo_counter}");
                        *memo_counter += 1;
                        let thunk = self.ctx.b.thunk(expression);
                        let derived = self.ctx.b.call_expr(helper, [Arg::Expr(thunk)]);
                        memo_decls.push(self.ctx.b.let_init_stmt(&memo_name, derived));
                        let memo_ref = self.ctx.b.alloc_str(&memo_name);
                        rune_get(&self.ctx.b, memo_ref)
                    }
                    Volatility::Static | Volatility::Reactive => expression,
                }
            }
            ComponentCssPropValue::StaticString(span) => {
                let text = self.ctx.query.component.source_text(*span);
                self.ctx.b.str_expr(text)
            }
            ComponentCssPropValue::Concatenation(plan) => {
                let Attribute::ConcatenationAttribute(concat) = attr else {
                    return CodegenError::semantic_mismatch(
                        attr_id,
                        "css property concatenation requires ConcatenationAttribute",
                    );
                };
                self.build_concat_expr_from_plan(
                    attr_id,
                    &concat.parts,
                    plan,
                    memo_decls,
                    memo_counter,
                    async_values,
                )?
            }
            ComponentCssPropValue::Boolean => self.ctx.b.bool_expr(true),
        };
        css_props.push(ObjProp::KeyValue(key, expr));
        Ok(())
    }
}
