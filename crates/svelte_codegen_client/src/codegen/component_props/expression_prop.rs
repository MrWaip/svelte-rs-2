use crate::codegen::expr::coarse_wrap;
use oxc_ast::ast::{Expression, Statement};
use oxc_syntax::node::NodeId as OxcNodeId;
use svelte_analyze::{ComponentPropMemo, ConcatPartEmit};
use svelte_ast::{ConcatPart, NodeId};
use svelte_ast_builder::{Arg, ObjProp, TemplatePart};
use svelte_emit_builders::runes::rune_get;

use super::super::async_values::AsyncValues;
use super::super::expr::evaluation_is_defined;
use super::super::{Codegen, CodegenError, Result};
use super::dispatch::PropOrSpread;

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(super) fn emit_component_prop_string(
        &self,
        name: &str,
        value_text: &str,
        items: &mut Vec<PropOrSpread<'a>>,
    ) {
        let key = self.ctx.b.alloc_str(name);
        items.push(PropOrSpread::Prop(ObjProp::KeyValue(
            key,
            self.ctx.b.str_expr(value_text),
        )));
    }

    pub(super) fn emit_component_prop_boolean(
        &self,
        name: &str,
        items: &mut Vec<PropOrSpread<'a>>,
    ) {
        let key = self.ctx.b.alloc_str(name);
        items.push(PropOrSpread::Prop(ObjProp::KeyValue(
            key,
            self.ctx.b.bool_expr(true),
        )));
    }

    pub(super) fn emit_component_prop_expression(
        &mut self,
        name: &str,
        attr_id: NodeId,
        expr_id: OxcNodeId,
        shorthand: bool,
        memo: ComponentPropMemo,
        items: &mut Vec<PropOrSpread<'a>>,
        memo_decls: &mut Vec<Statement<'a>>,
        memo_counter: &mut u32,
        async_values: &mut AsyncValues<'a>,
    ) -> Result<()> {
        let key = self.ctx.b.alloc_str(name);
        let Some(expr) = self.ctx.state.parsed.take_expr(expr_id) else {
            return CodegenError::missing_expression(attr_id);
        };
        match memo {
            ComponentPropMemo::Awaited => {
                let data = self.ctx.expression_data(attr_id).cloned();
                let value = coarse_wrap(self.ctx, expr, data.as_ref());
                let suspension = data.map(|d| d.suspension).unwrap_or_default();
                let memo_ref = async_values.push(self.ctx, value, suspension);
                let get = rune_get(&self.ctx.b, memo_ref);
                items.push(PropOrSpread::Prop(ObjProp::Getter(key, get)));
            }
            ComponentPropMemo::Derived => {
                let data = self.ctx.expression_data(attr_id).cloned();
                let thunk_body = coarse_wrap(self.ctx, expr, data.as_ref());
                let helper = self.ctx.query.view.derived_helper();
                let memo_name = format!("${memo_counter}");
                *memo_counter += 1;
                let thunk = self.ctx.b.thunk(thunk_body);
                let derived = self.ctx.b.call_expr(helper, [Arg::Expr(thunk)]);
                memo_decls.push(self.ctx.b.let_init_stmt(&memo_name, derived));
                let memo_ref = self.ctx.b.alloc_str(&memo_name);
                let get = rune_get(&self.ctx.b, memo_ref);
                items.push(PropOrSpread::Prop(ObjProp::Getter(key, get)));
            }
            ComponentPropMemo::Getter => {
                let data = self.ctx.expression_data(attr_id).cloned();
                let expr = coarse_wrap(self.ctx, expr, data.as_ref());
                items.push(PropOrSpread::Prop(ObjProp::Getter(key, expr)));
            }
            ComponentPropMemo::Inline => {
                let data = self.ctx.expression_data(attr_id).cloned();
                let expr = coarse_wrap(self.ctx, expr, data.as_ref());
                if shorthand && expr_prints_as_ident(&expr, name) {
                    items.push(PropOrSpread::Prop(ObjProp::Shorthand(key)));
                } else {
                    items.push(PropOrSpread::Prop(ObjProp::KeyValue(key, expr)));
                }
            }
        }
        Ok(())
    }

    pub(super) fn emit_component_prop_concat(
        &mut self,
        name: &str,
        attr_id: NodeId,
        parts: &[ConcatPart],
        memo: ComponentPropMemo,
        plan: &[ConcatPartEmit],
        items: &mut Vec<PropOrSpread<'a>>,
        memo_decls: &mut Vec<Statement<'a>>,
        memo_counter: &mut u32,
        async_values: &mut AsyncValues<'a>,
    ) -> Result<()> {
        let key = self.ctx.b.alloc_str(name);
        let val = self.build_concat_expr_from_plan(
            attr_id,
            parts,
            plan,
            memo_decls,
            memo_counter,
            async_values,
        )?;
        match memo {
            ComponentPropMemo::Awaited | ComponentPropMemo::Derived | ComponentPropMemo::Getter => {
                items.push(PropOrSpread::Prop(ObjProp::Getter(key, val)));
            }
            ComponentPropMemo::Inline => {
                items.push(PropOrSpread::Prop(ObjProp::KeyValue(key, val)));
            }
        }
        Ok(())
    }

    pub(in crate::codegen) fn build_concat_expr_from_plan(
        &mut self,
        attr_id: NodeId,
        parts: &[ConcatPart],
        plan: &[ConcatPartEmit],
        memo_decls: &mut Vec<Statement<'a>>,
        memo_counter: &mut u32,
        async_values: &mut AsyncValues<'a>,
    ) -> Result<Expression<'a>> {
        let helper = self.ctx.query.view.derived_helper();
        let mut tpl_parts: Vec<TemplatePart<'a>> = Vec::new();
        for (part, emit) in parts.iter().zip(plan.iter()) {
            match (part, emit) {
                (ConcatPart::Static(s), _) => {
                    push_template_str(&mut tpl_parts, s.clone());
                }
                (ConcatPart::Dynamic { id, .. }, ConcatPartEmit::Static) => {
                    let known = self
                        .ctx
                        .expression_data(*id)
                        .and_then(|d| d.evaluation.known_str())
                        .unwrap_or_default();
                    push_template_str(&mut tpl_parts, known);
                }
                (ConcatPart::Dynamic { id, expr }, ConcatPartEmit::Inline) => {
                    let data = self.ctx.expression_data(*id).cloned();
                    let defined = data
                        .as_ref()
                        .map(|d| evaluation_is_defined(&d.evaluation))
                        .unwrap_or(false);
                    let Some(part_expr) = self.take_expr_by_ref(expr) else {
                        return CodegenError::missing_expression(attr_id);
                    };
                    let wrapped = coarse_wrap(self.ctx, part_expr, data.as_ref());
                    tpl_parts.push(TemplatePart::Expr(wrapped, defined));
                }
                (ConcatPart::Dynamic { id, expr }, ConcatPartEmit::Awaited) => {
                    let data = self.ctx.expression_data(*id).cloned();
                    let Some(part_expr) = self.take_expr_by_ref(expr) else {
                        return CodegenError::missing_expression(attr_id);
                    };
                    let value = coarse_wrap(self.ctx, part_expr, data.as_ref());
                    let suspension = data.map(|d| d.suspension).unwrap_or_default();
                    let memo_ref = async_values.push(self.ctx, value, suspension);
                    let get = rune_get(&self.ctx.b, memo_ref);
                    tpl_parts.push(TemplatePart::Expr(get, false));
                }
                (ConcatPart::Dynamic { id, expr }, ConcatPartEmit::HoistDerived) => {
                    let data = self.ctx.expression_data(*id).cloned();
                    let Some(part_expr) = self.take_expr_by_ref(expr) else {
                        return CodegenError::missing_expression(attr_id);
                    };
                    let thunk_body = coarse_wrap(self.ctx, part_expr, data.as_ref());
                    let memo_name = format!("${memo_counter}");
                    *memo_counter += 1;
                    let thunk = self.ctx.b.thunk(thunk_body);
                    let derived = self.ctx.b.call_expr(helper, [Arg::Expr(thunk)]);
                    memo_decls.push(self.ctx.b.let_init_stmt(&memo_name, derived));
                    let memo_ref = self.ctx.b.alloc_str(&memo_name);
                    let get = rune_get(&self.ctx.b, memo_ref);
                    tpl_parts.push(TemplatePart::Expr(get, false));
                }
            }
        }

        if tpl_parts.len() == 1 {
            return Ok(match tpl_parts.into_iter().next() {
                Some(TemplatePart::Str(s)) => self.ctx.b.str_expr(&s),
                Some(TemplatePart::Expr(expr, _)) => expr,
                None => self.ctx.b.template_parts_expr(Vec::new()),
            });
        }
        Ok(self.ctx.b.template_parts_expr(tpl_parts))
    }
}

fn expr_prints_as_ident(expr: &Expression<'_>, name: &str) -> bool {
    matches!(expr, Expression::Identifier(ident) if ident.name == name)
}

fn push_template_str<'a>(tpl_parts: &mut Vec<TemplatePart<'a>>, value: String) {
    if let Some(TemplatePart::Str(prev)) = tpl_parts.last_mut() {
        prev.push_str(&value);
    } else {
        tpl_parts.push(TemplatePart::Str(value));
    }
}
