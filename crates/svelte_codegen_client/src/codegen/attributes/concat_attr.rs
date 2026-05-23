use crate::codegen::expr::coarse_wrap;
use oxc_ast::ast::{Expression, Statement};
use svelte_analyze::{
    AttributeSemantics, HtmlConcatPart, HtmlConcatSemantics, normalize_regular_attribute_name,
    TemplateEffect,
};
use svelte_ast::{ConcatPart, ConcatenationAttribute, NodeId};
use svelte_ast_builder::TemplatePart;

use super::super::data_structures::{EmitState, TemplateMemoState};
use super::super::effect::emit_effect_call_extern;
use super::super::{Codegen, CodegenError, Result};

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

        let semantics = match self.ctx.query.analysis.attributes.get(attr.id) {
            AttributeSemantics::HtmlConcat(s) => s.clone(),
            _ => {
                return CodegenError::semantic_mismatch(
                    attr.id,
                    "ConcatenationAttribute requires HtmlConcat semantics",
                );
            }
        };

        let (val, mut memo_deps) = self.build_html_concat_expr(attr, &semantics)?;

        if attr.name == "value" && owner_tag == "option" {
            self.emit_option_concat_value(state, owner_var, val);
            return Ok(());
        }

        let html_attr_namespace = self.is_html_attr_namespace(owner_id);
        let attr_name = normalize_regular_attribute_name(&attr.name, html_attr_namespace);
        let attr_update = self.regular_attr_update(owner_id, owner_tag, &attr_name);

        match semantics.effect {
            TemplateEffect::None => {
                self.push_regular_attr_update(&mut state.init, owner_var, attr_update, val);
            }
            TemplateEffect::Sync | TemplateEffect::Async => {
                if memo_deps.has_deps() {
                    let param_names = memo_deps.param_names();
                    let params = if param_names.is_empty() {
                        self.ctx.b.no_params()
                    } else {
                        self.ctx.b.params(param_names.iter().map(|s| s.as_str()))
                    };
                    let mut update_stmts: Vec<Statement<'a>> = Vec::new();
                    self.push_regular_attr_update(
                        &mut update_stmts,
                        owner_var,
                        attr_update,
                        val,
                    );
                    let callback = self.ctx.b.arrow_expr(params, update_stmts);
                    emit_effect_call_extern(
                        self.ctx,
                        "$.template_effect",
                        callback,
                        &mut memo_deps,
                        &mut state.after_update,
                    );
                } else {
                    self.push_regular_attr_update(&mut state.update, owner_var, attr_update, val);
                }
            }
        }

        Ok(())
    }

    pub(super) fn build_html_concat_expr(
        &mut self,
        attr: &ConcatenationAttribute,
        semantics: &HtmlConcatSemantics,
    ) -> Result<(Expression<'a>, TemplateMemoState<'a>)> {
        if attr.parts.len() != semantics.parts.len() {
            return CodegenError::semantic_mismatch(
                attr.id,
                "HtmlConcatSemantics.parts length mismatch with attribute parts",
            );
        }

        let mut tpl_parts: Vec<TemplatePart<'a>> = Vec::with_capacity(semantics.parts.len());
        let mut memo_deps = TemplateMemoState::default();

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

        if tpl_parts.len() == 1
            && let TemplatePart::Str(s) = &tpl_parts[0]
        {
            return Ok((self.ctx.b.str_expr(s), memo_deps));
        }

        Ok((self.ctx.b.template_parts_expr(tpl_parts), memo_deps))
    }
}

fn push_template_str<'a>(parts: &mut Vec<TemplatePart<'a>>, text: &str) {
    if let Some(TemplatePart::Str(prev)) = parts.last_mut() {
        prev.push_str(text);
    } else {
        parts.push(TemplatePart::Str(text.to_string()));
    }
}
