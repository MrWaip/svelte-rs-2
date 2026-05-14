use oxc_ast::ast::{Expression, Statement};
use oxc_syntax::node::NodeId as OxcNodeId;
use svelte_analyze::{ComponentPropMemo, ConcatPartEmit};
use svelte_ast::{ConcatPart, NodeId, Span};
use svelte_ast_builder::{Arg, ObjProp, TemplatePart};

use super::super::expr::evaluation_is_defined;
use super::super::{Codegen, CodegenError, Result};
use super::dispatch::PropOrSpread;

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(super) fn emit_component_prop_string(
        &self,
        name: &str,
        value_span: Span,
        items: &mut Vec<PropOrSpread<'a>>,
    ) {
        let value_text = self.ctx.query.component.source_text(value_span);
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
        in_block_callback: bool,
        items: &mut Vec<PropOrSpread<'a>>,
        memo_decls: &mut Vec<Statement<'a>>,
        memo_counter: &mut u32,
    ) -> Result<()> {
        let key = self.ctx.b.alloc_str(name);
        let Some(expr) = self.ctx.state.parsed.take_expr(expr_id) else {
            return CodegenError::missing_expression(attr_id);
        };
        let expr = self.maybe_wrap_legacy_slots_read(expr);
        match memo {
            ComponentPropMemo::Derived => {
                let data = self.ctx.expression_data(attr_id).cloned();
                let thunk_body = if self.is_legacy_event_prop_call(name, &expr) {
                    let callee = match expr {
                        Expression::CallExpression(call) => call.unbox().callee,
                        _ => unreachable!(),
                    };
                    self.ctx.b.call_expr("$.untrack", [Arg::Expr(callee)])
                } else {
                    self.maybe_wrap_legacy_coarse_expr(expr, data.as_ref(), in_block_callback)
                };
                let helper = self.ctx.query.view.derived_helper();
                let memo_name = format!("${memo_counter}");
                *memo_counter += 1;
                let thunk = self.ctx.b.thunk(thunk_body);
                let derived = self.ctx.b.call_expr(helper, [Arg::Expr(thunk)]);
                memo_decls.push(self.ctx.b.let_init_stmt(&memo_name, derived));
                let memo_ref = self.ctx.b.alloc_str(&memo_name);
                let get = self.ctx.b.call_expr("$.get", [Arg::Ident(memo_ref)]);
                items.push(PropOrSpread::Prop(ObjProp::Getter(key, get)));
            }
            ComponentPropMemo::Getter => {
                let data = self.ctx.expression_data(attr_id).cloned();
                let expr =
                    self.maybe_wrap_legacy_coarse_expr(expr, data.as_ref(), in_block_callback);
                items.push(PropOrSpread::Prop(ObjProp::Getter(key, expr)));
            }
            ComponentPropMemo::Inline if shorthand => {
                items.push(PropOrSpread::Prop(ObjProp::Shorthand(key)));
            }
            ComponentPropMemo::Inline => {
                items.push(PropOrSpread::Prop(ObjProp::KeyValue(key, expr)));
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
        in_block_callback: bool,
        items: &mut Vec<PropOrSpread<'a>>,
        memo_decls: &mut Vec<Statement<'a>>,
        memo_counter: &mut u32,
    ) -> Result<()> {
        let key = self.ctx.b.alloc_str(name);
        let val = self.build_concat_expr_from_plan(
            attr_id,
            parts,
            plan,
            in_block_callback,
            memo_decls,
            memo_counter,
        )?;
        match memo {
            ComponentPropMemo::Derived | ComponentPropMemo::Getter => {
                items.push(PropOrSpread::Prop(ObjProp::Getter(key, val)));
            }
            ComponentPropMemo::Inline => {
                items.push(PropOrSpread::Prop(ObjProp::KeyValue(key, val)));
            }
        }
        Ok(())
    }

    fn build_concat_expr_from_plan(
        &mut self,
        attr_id: NodeId,
        parts: &[ConcatPart],
        plan: &[ConcatPartEmit],
        in_block_callback: bool,
        memo_decls: &mut Vec<Statement<'a>>,
        memo_counter: &mut u32,
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
                    let wrapped = self.maybe_wrap_legacy_coarse_expr(
                        part_expr,
                        data.as_ref(),
                        in_block_callback,
                    );
                    tpl_parts.push(TemplatePart::Expr(wrapped, defined));
                }
                (ConcatPart::Dynamic { id, expr }, ConcatPartEmit::HoistDerived) => {
                    let defined = self
                        .ctx
                        .expression_data(*id)
                        .map(|d| evaluation_is_defined(&d.evaluation))
                        .unwrap_or(false);
                    let Some(part_expr) = self.take_expr_by_ref(expr) else {
                        return CodegenError::missing_expression(attr_id);
                    };
                    let memo_name = format!("${memo_counter}");
                    *memo_counter += 1;
                    let thunk = self.ctx.b.thunk(part_expr);
                    let derived = self.ctx.b.call_expr(helper, [Arg::Expr(thunk)]);
                    memo_decls.push(self.ctx.b.let_init_stmt(&memo_name, derived));
                    let memo_ref = self.ctx.b.alloc_str(&memo_name);
                    let get = self.ctx.b.call_expr("$.get", [Arg::Ident(memo_ref)]);
                    tpl_parts.push(TemplatePart::Expr(get, defined));
                }
            }
        }

        if tpl_parts.len() == 1
            && let TemplatePart::Str(s) = &tpl_parts[0]
        {
            return Ok(self.ctx.b.str_expr(s));
        }
        Ok(self.ctx.b.template_parts_expr(tpl_parts))
    }

    fn is_legacy_event_prop_call(&self, name: &str, expr: &Expression<'a>) -> bool {
        !self.ctx.query.runes()
            && name.starts_with("on")
            && matches!(expr, Expression::CallExpression(_))
    }
}

fn push_template_str<'a>(tpl_parts: &mut Vec<TemplatePart<'a>>, value: String) {
    if let Some(TemplatePart::Str(prev)) = tpl_parts.last_mut() {
        prev.push_str(&value);
    } else {
        tpl_parts.push(TemplatePart::Str(value));
    }
}
