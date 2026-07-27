use crate::codegen::expr::coarse_wrap;
use oxc_ast::ast::{Expression, Statement};
use svelte_analyze::{ExpressionData, NamespaceKind, Volatility};
use svelte_ast::NodeId;
use svelte_ast_builder::{Arg, AssignLeft, TemplatePart};

use super::super::data_structures::{DeferredMemoValue, EmitState};
use super::super::expr::evaluation_is_defined;
use super::super::{Codegen, CodegenError, Result};

pub(super) enum RegularAttrUpdate {
    Call {
        setter_fn: &'static str,
        attr_name: Option<String>,
    },
    Assignment {
        property: String,
    },
}

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(super) fn is_html_attr_namespace(&self, el_id: NodeId) -> bool {
        !matches!(
            self.ctx.query.view.namespace(el_id),
            Some(NamespaceKind::Svg)
                | Some(NamespaceKind::MathMl)
                | Some(NamespaceKind::AnnotationXml)
        )
    }

    pub(super) fn defer_blockers(
        &self,
        state: &mut EmitState<'a>,
        node: NodeId,
        data: &ExpressionData,
    ) {
        state.deferred_memo_values.push(DeferredMemoValue {
            node,
            late_id: None,
            data: data.clone(),
            expr: None,
        });
    }

    pub(super) fn defer_memo_value(
        &mut self,
        state: &mut EmitState<'a>,
        node: NodeId,
        data: &ExpressionData,
        expr: Expression<'a>,
    ) -> Expression<'a> {
        match data.volatility {
            Volatility::Heavy | Volatility::Asynchronous => {}
            Volatility::Static | Volatility::Reactive => {
                self.defer_blockers(state, node, data);
                return expr;
            }
        }
        let late_id = state.shared_memo.reserve_late();
        state.deferred_memo_values.push(DeferredMemoValue {
            node,
            late_id: Some(late_id),
            data: data.clone(),
            expr: Some(expr),
        });
        state.shared_memo.late_param_expr(self.ctx, late_id)
    }

    pub(in super::super) fn flush_deferred_memo_values(
        &mut self,
        state: &mut EmitState<'a>,
        start: usize,
    ) -> Result<()> {
        let deferred: Vec<DeferredMemoValue<'a>> =
            state.deferred_memo_values.drain(start..).collect();
        for entry in deferred {
            let (Some(late_id), Some(expr)) = (entry.late_id, entry.expr) else {
                state
                    .shared_memo
                    .push_expression_data(self.ctx, &entry.data);
                continue;
            };
            let Some(slot) = state
                .shared_memo
                .add_memoized_expr(self.ctx, &entry.data, expr)
            else {
                return CodegenError::semantic_mismatch(
                    entry.node,
                    "deferred memo value reserved a slot but did not memoize",
                );
            };
            state.shared_memo.resolve_late(late_id, Some(slot));
        }
        Ok(())
    }

    pub(super) fn attr_blockers(&self, attr_id: NodeId) -> Vec<u32> {
        self.ctx
            .expression_data(attr_id)
            .map(|d| d.blockers.iter().copied().collect())
            .unwrap_or_default()
    }

    pub(super) fn regular_attr_update(&self, attr_name: &str) -> RegularAttrUpdate {
        if attr_name == "value" {
            return RegularAttrUpdate::Call {
                setter_fn: "$.set_value",
                attr_name: None,
            };
        }

        if attr_name == "checked" {
            return RegularAttrUpdate::Call {
                setter_fn: "$.set_checked",
                attr_name: None,
            };
        }

        if attr_name == "selected" {
            return RegularAttrUpdate::Call {
                setter_fn: "$.set_selected",
                attr_name: None,
            };
        }

        if attr_name == "style" {
            return RegularAttrUpdate::Call {
                setter_fn: "$.set_style",
                attr_name: None,
            };
        }

        if svelte_analyze::is_regular_dom_property(attr_name) {
            return RegularAttrUpdate::Assignment {
                property: attr_name.to_string(),
            };
        }

        RegularAttrUpdate::Call {
            setter_fn: if attr_name.starts_with("xlink") {
                "$.set_xlink_attribute"
            } else {
                "$.set_attribute"
            },
            attr_name: Some(attr_name.to_string()),
        }
    }

    pub(super) fn push_regular_attr_update(
        &self,
        target: &mut Vec<Statement<'a>>,
        el_name: &str,
        update: RegularAttrUpdate,
        val: Expression<'a>,
        owner_id: NodeId,
    ) {
        let b = &self.ctx.b;
        match update {
            RegularAttrUpdate::Call {
                setter_fn,
                attr_name,
            } => {
                let mut args: Vec<Arg<'a, '_>> = vec![Arg::Ident(el_name)];
                if let Some(name) = attr_name {
                    args.push(Arg::Str(name));
                }
                args.push(Arg::Expr(val));
                if matches!(setter_fn, "$.set_attribute" | "$.set_xlink_attribute")
                    && self.ctx.hydration_attribute_changed_ignored(owner_id)
                {
                    args.push(Arg::Bool(true));
                }
                target.push(b.call_stmt(setter_fn, args));
            }
            RegularAttrUpdate::Assignment { property } => {
                target.push(b.assign_stmt(
                    AssignLeft::StaticMember(b.static_member(b.rid_expr(el_name), &property)),
                    val,
                ));
            }
        }
    }

    pub(super) fn wrap_run_after_blockers(
        &self,
        stmt: Statement<'a>,
        blockers: &[u32],
    ) -> Statement<'a> {
        if blockers.is_empty() {
            return stmt;
        }
        let b = &self.ctx.b;
        let blockers_arr = b.promises_array(blockers);
        let thunk = b.thunk_block(vec![stmt]);
        b.call_stmt(
            "$.run_after_blockers",
            [Arg::Expr(blockers_arr), Arg::Expr(thunk)],
        )
    }

    pub(in super::super) fn build_concat_expr_collapse_single(
        &mut self,
        attr_id: NodeId,
        parts: &[svelte_ast::ConcatPart],
    ) -> Result<Expression<'a>> {
        let mut tpl_parts = self.concat_to_tpl_parts(attr_id, parts)?;

        if tpl_parts.len() == 1
            && let TemplatePart::Str(s) = &tpl_parts[0]
        {
            let expr = self.ctx.b.str_expr(s);
            tpl_parts.clear();
            return Ok(expr);
        }

        Ok(self.ctx.b.template_parts_expr(tpl_parts))
    }

    fn concat_to_tpl_parts(
        &mut self,
        attr_id: NodeId,
        parts: &[svelte_ast::ConcatPart],
    ) -> Result<Vec<TemplatePart<'a>>> {
        let mut tpl_parts: Vec<TemplatePart<'a>> = Vec::new();
        for part in parts {
            match part {
                svelte_ast::ConcatPart::Static(s) => push_template_str(&mut tpl_parts, s.clone()),
                svelte_ast::ConcatPart::Dynamic {
                    id: part_id,
                    expr: expr_ref,
                } => {
                    let Some(expr) = self.take_expr_by_ref(expr_ref) else {
                        return CodegenError::missing_expression(attr_id);
                    };
                    if let Some(lit) = literal_value(&expr) {
                        push_template_str(&mut tpl_parts, lit);
                        continue;
                    }

                    let data = self.ctx.expression_data(*part_id).cloned();
                    if let Some(s) = data.as_ref().and_then(|d| d.evaluation.known_str()) {
                        push_template_str(&mut tpl_parts, s);
                        continue;
                    }
                    let defined = data
                        .as_ref()
                        .map(|d| {
                            evaluation_is_defined(&d.evaluation)
                                && matches!(d.legacy_wrap, svelte_analyze::LegacyWrap::None)
                        })
                        .unwrap_or(false);
                    let wrapped = coarse_wrap(self.ctx, expr, data.as_ref());
                    tpl_parts.push(TemplatePart::Expr(wrapped, defined));
                }
            }
        }
        Ok(tpl_parts)
    }
}

fn push_template_str<'a>(tpl_parts: &mut Vec<TemplatePart<'a>>, value: String) {
    if let Some(TemplatePart::Str(prev)) = tpl_parts.last_mut() {
        prev.push_str(&value);
    } else {
        tpl_parts.push(TemplatePart::Str(value));
    }
}

pub(super) fn literal_value(expr: &Expression<'_>) -> Option<String> {
    match expr.get_inner_expression() {
        Expression::StringLiteral(lit) => Some(lit.value.as_str().to_string()),
        Expression::NumericLiteral(lit) => Some(lit.value.to_string()),
        Expression::BooleanLiteral(lit) => Some(lit.value.to_string()),
        Expression::NullLiteral(_) => Some(String::new()),
        _ => None,
    }
}
