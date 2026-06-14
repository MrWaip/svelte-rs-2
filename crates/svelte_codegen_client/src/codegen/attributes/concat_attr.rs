use crate::codegen::expr::coarse_wrap;
use oxc_ast::ast::Expression;
use svelte_analyze::{
    AttributeSemantics, HtmlConcatPart, HtmlConcatSemantics, SpecialValueKind, TemplateEffect,
    normalize_regular_attribute_name,
};
use svelte_ast::{ConcatPart, ConcatenationAttribute, NodeId};
use svelte_ast_builder::TemplatePart;

use super::super::data_structures::{EmitState, TemplateMemoState};
use super::super::{Codegen, CodegenError, Result};
use super::option_value::OptionValueForm;

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(in super::super) fn emit_attr_concatenation(
        &mut self,
        state: &mut EmitState<'a>,
        owner_id: NodeId,
        owner_tag: &str,
        owner_var: &str,
        attr: &ConcatenationAttribute,
    ) -> Result<()> {
        if attr.name == "class" {
            return Ok(());
        }

        let (semantics, special) = match self.ctx.query.analysis.attributes.get(attr.id) {
            AttributeSemantics::HtmlConcat(s) => (s.clone(), None),
            AttributeSemantics::SpecialValueAttr(s) => {
                let Some(concat) = s.concat.as_ref() else {
                    return CodegenError::semantic_mismatch(
                        attr.id,
                        "SpecialValueAttr on ConcatenationAttribute requires concat semantics",
                    );
                };
                (concat.clone(), Some((s.kind, s.defined, s.volatile)))
            }
            _ => {
                return CodegenError::semantic_mismatch(
                    attr.id,
                    "ConcatenationAttribute requires HtmlConcat semantics",
                );
            }
        };

        let val = self.build_html_concat_expr(attr, &semantics, &mut state.shared_memo)?;

        if let Some((kind, defined, volatile)) = special {
            let coalesce = !defined;
            match kind {
                SpecialValueKind::Option => {
                    let form = OptionValueForm::Reflected { coalesce };
                    self.emit_option_value(state, owner_var, val, form, volatile);
                    return Ok(());
                }
                SpecialValueKind::Select => {
                    self.emit_select_value(state, owner_var, val, coalesce, volatile);
                    return Ok(());
                }
                SpecialValueKind::InputBindGroup | SpecialValueKind::InputBindChecked => {
                    self.emit_input_value(state, owner_var, val, coalesce);
                    return Ok(());
                }
            }
        }

        let html_attr_namespace = self.is_html_attr_namespace(owner_id);
        let attr_name = normalize_regular_attribute_name(&attr.name, html_attr_namespace);
        let attr_update = self.regular_attr_update(owner_id, owner_tag, &attr_name);

        let target = match semantics.effect {
            TemplateEffect::None => &mut state.init,
            TemplateEffect::Sync | TemplateEffect::Async => &mut state.update,
        };
        self.push_regular_attr_update(target, owner_var, attr_update, val, owner_id);

        Ok(())
    }

    pub(super) fn build_html_concat_expr(
        &mut self,
        attr: &ConcatenationAttribute,
        semantics: &HtmlConcatSemantics,
        memo_deps: &mut TemplateMemoState<'a>,
    ) -> Result<Expression<'a>> {
        if attr.parts.len() != semantics.parts.len() {
            return CodegenError::semantic_mismatch(
                attr.id,
                "HtmlConcatSemantics.parts length mismatch with attribute parts",
            );
        }

        let mut tpl_parts: Vec<TemplatePart<'a>> = Vec::with_capacity(semantics.parts.len());

        for (part, plan) in attr.parts.iter().zip(semantics.parts.iter()) {
            match plan {
                HtmlConcatPart::StaticText(text) => {
                    push_template_str(&mut tpl_parts, text.as_str());
                }
                HtmlConcatPart::Inline {
                    part_id, defined, ..
                } => {
                    let ConcatPart::Dynamic { expr, .. } = part else {
                        return CodegenError::semantic_mismatch(
                            *part_id,
                            "Inline plan over Static ConcatPart",
                        );
                    };
                    let Some(expr_value) = self.take_expr_by_ref(expr) else {
                        return CodegenError::missing_expression(attr.id);
                    };
                    let data = self.ctx.expression_data(*part_id).cloned();
                    let wrapped = coarse_wrap(self.ctx, expr_value, data.as_ref());
                    if let Some(d) = &data {
                        memo_deps.push_expression_data(self.ctx, d);
                    }
                    tpl_parts.push(TemplatePart::Expr(wrapped, *defined));
                }
                HtmlConcatPart::SyncMemoSlot {
                    part_id, defined, ..
                } => {
                    let ConcatPart::Dynamic { expr, .. } = part else {
                        return CodegenError::semantic_mismatch(
                            *part_id,
                            "SyncMemoSlot plan over Static ConcatPart",
                        );
                    };
                    let Some(expr_value) = self.take_expr_by_ref(expr) else {
                        return CodegenError::missing_expression(attr.id);
                    };
                    let data = self.ctx.expression_data(*part_id).cloned();
                    let wrapped = coarse_wrap(self.ctx, expr_value, data.as_ref());
                    if let Some(d) = &data {
                        memo_deps.push_expression_data(self.ctx, d);
                    }
                    let index = memo_deps.sync_values_push(wrapped);
                    let slot = memo_deps.sync_param_expr(self.ctx, index);
                    tpl_parts.push(TemplatePart::Expr(slot, *defined));
                }
                HtmlConcatPart::AsyncMemoSlot {
                    part_id, defined, ..
                } => {
                    let ConcatPart::Dynamic { expr, .. } = part else {
                        return CodegenError::semantic_mismatch(
                            *part_id,
                            "AsyncMemoSlot plan over Static ConcatPart",
                        );
                    };
                    let Some(expr_value) = self.take_expr_by_ref(expr) else {
                        return CodegenError::missing_expression(attr.id);
                    };
                    let data = self.ctx.expression_data(*part_id).cloned();
                    let wrapped = coarse_wrap(self.ctx, expr_value, data.as_ref());
                    if let Some(d) = &data {
                        memo_deps.push_expression_data(self.ctx, d);
                    }
                    let index = memo_deps.async_values_push(wrapped);
                    let slot = memo_deps.async_param_expr(self.ctx, index);
                    tpl_parts.push(TemplatePart::Expr(slot, *defined));
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

fn push_template_str<'a>(parts: &mut Vec<TemplatePart<'a>>, text: &str) {
    if let Some(TemplatePart::Str(prev)) = parts.last_mut() {
        prev.push_str(text);
    } else {
        parts.push(TemplatePart::Str(text.to_string()));
    }
}
