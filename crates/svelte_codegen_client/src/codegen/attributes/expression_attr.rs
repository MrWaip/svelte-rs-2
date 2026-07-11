use crate::codegen::expr::coarse_wrap;
use svelte_analyze::{
    AttributeSemantics, DefaultAttrKind, EventSemantics, SpecialValueKind, Volatility,
    normalize_regular_attribute_name,
};

use super::regular::RegularAttrUpdate;
use svelte_ast::{ConcatenationAttribute, ExprRef, ExpressionAttribute, NodeId};
use svelte_ast_builder::Arg;

use super::super::data_structures::{EmitState, MemoValueRef};
use super::super::{Codegen, CodegenError, Result};
use super::option_value::OptionValueForm;

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(in super::super) fn emit_attr_expression(
        &mut self,
        state: &mut EmitState<'a>,
        owner_id: NodeId,
        _owner_tag: &str,
        owner_var: &str,
        attr: &ExpressionAttribute,
    ) -> Result<()> {
        if matches!(
            self.ctx.query.analysis.attributes.get(attr.id),
            AttributeSemantics::Class(_)
        ) {
            return Ok(());
        }

        if let AttributeSemantics::Event(event) = self.ctx.query.analysis.attributes.get(attr.id) {
            return self.emit_event_attribute(state, owner_var, attr.id, &attr.expression, event);
        }

        if let AttributeSemantics::SpecialValueAttr(s) =
            self.ctx.query.analysis.attributes.get(attr.id)
        {
            let coalesce = !s.defined;
            let volatile = s.volatile;
            match s.kind {
                SpecialValueKind::InputBindGroup => {
                    let backup = self
                        .ctx
                        .state
                        .parsed
                        .expr(attr.expression.id())
                        .map(|e| self.ctx.b.clone_expr(e));
                    let val = self.take_attr_expr(attr.id, &attr.expression)?;
                    if let Some(backup) = backup {
                        self.ctx
                            .state
                            .parsed
                            .replace_expr(attr.expression.id(), backup);
                    }
                    self.emit_bind_group_value(state, owner_var, attr.id, val);
                    return Ok(());
                }
                SpecialValueKind::Option => {
                    let val = self.take_attr_expr(attr.id, &attr.expression)?;
                    let val = {
                        let data = self.ctx.expression_data(attr.id).cloned();
                        coarse_wrap(self.ctx, val, data.as_ref())
                    };
                    let form = OptionValueForm::Reflected { coalesce };
                    self.emit_option_value(state, owner_var, val, form, volatile);
                    return Ok(());
                }
                SpecialValueKind::Select => {
                    let val = self.take_attr_expr(attr.id, &attr.expression)?;
                    return self.emit_select_value(state, owner_var, attr.id, val, coalesce);
                }
                SpecialValueKind::InputBindChecked => {
                    let val = self.take_attr_expr(attr.id, &attr.expression)?;
                    self.emit_input_value(state, owner_var, val, coalesce);
                    return Ok(());
                }
            }
        }

        let attr_id = attr.id;
        let expr = self.take_attr_expr(attr_id, &attr.expression)?;
        let expr = {
            let data = self.ctx.expression_data(attr_id).cloned();
            coarse_wrap(self.ctx, expr, data.as_ref())
        };
        let html_attr_namespace = self.is_html_attr_namespace(owner_id);
        let attr_name = normalize_regular_attribute_name(&attr.name, html_attr_namespace);
        let attr_update = match self.ctx.query.analysis.attributes.get(attr_id) {
            AttributeSemantics::CannotBeStatic(sem) => match sem.kind {
                DefaultAttrKind::ReconcileValue => RegularAttrUpdate::Call {
                    setter_fn: "$.set_default_value",
                    attr_name: None,
                },
                DefaultAttrKind::ReconcileChecked => RegularAttrUpdate::Call {
                    setter_fn: "$.set_default_checked",
                    attr_name: None,
                },
                DefaultAttrKind::PlainProperty => RegularAttrUpdate::Assignment {
                    property: attr_name.to_string(),
                },
            },
            _ => self.regular_attr_update(&attr_name),
        };

        match self.ctx.expression_data(attr_id).map(|d| d.volatility) {
            Some(Volatility::Heavy | Volatility::Asynchronous) => {
                let Some(data) = self.ctx.expression_data(attr_id).cloned() else {
                    return CodegenError::missing_expression_deps(attr_id);
                };
                let placeholder = match state.shared_memo.add_memoized_expr(self.ctx, &data, expr) {
                    Some(MemoValueRef::Sync(i)) => state.shared_memo.sync_param_expr(self.ctx, i),
                    Some(MemoValueRef::Async(i)) => state.shared_memo.async_param_expr(self.ctx, i),
                    None => return CodegenError::missing_expression_deps(attr_id),
                };
                self.push_regular_attr_update(
                    &mut state.update,
                    owner_var,
                    attr_update,
                    placeholder,
                    owner_id,
                );
            }
            Some(Volatility::Reactive) => {
                self.push_regular_attr_update(
                    &mut state.update,
                    owner_var,
                    attr_update,
                    expr,
                    owner_id,
                );
            }
            Some(Volatility::Static) | None => {
                self.push_regular_attr_update(
                    &mut state.init,
                    owner_var,
                    attr_update,
                    expr,
                    owner_id,
                );
            }
        }

        Ok(())
    }

    pub(in super::super) fn emit_concat_event(
        &mut self,
        state: &mut EmitState<'a>,
        owner_var: &str,
        attr: &ConcatenationAttribute,
    ) -> Result<()> {
        let Some(expr) = svelte_analyze::concat_single_dynamic_expr(attr) else {
            return Ok(());
        };
        let AttributeSemantics::Event(event) = self.ctx.query.analysis.attributes.get(attr.id)
        else {
            return Ok(());
        };
        self.emit_event_attribute(state, owner_var, attr.id, expr, event)
    }

    fn emit_event_attribute(
        &mut self,
        state: &mut EmitState<'a>,
        owner_var: &str,
        attr_id: NodeId,
        expression: &ExprRef,
        event: &EventSemantics,
    ) -> Result<()> {
        let capture = event.capture;
        let passive = event.passive;
        let delegated = event.delegatable;
        let event_name = event.name.clone();

        if !event.handler.is_user() {
            return Ok(());
        }

        let expr_offset = expression.span.start;
        let expr = self.take_attr_expr(attr_id, expression)?;

        let handler =
            self.build_event_handler_s5(attr_id, expr, event.handler, &mut state.init, expr_offset);
        let handler = self.dev_event_handler(attr_id, handler, &event_name)?;

        if delegated {
            let mut args: Vec<Arg<'a, '_>> = vec![
                Arg::StrRef(&event_name),
                Arg::Ident(owner_var),
                Arg::Expr(handler),
            ];
            if passive {
                args.push(Arg::Expr(self.ctx.b.void_zero_expr()));
                args.push(Arg::Bool(true));
            }
            state
                .after_update
                .push(self.ctx.b.call_stmt("$.delegated", args));
            self.ctx.add_delegated_event(event_name);
            return Ok(());
        }

        let mut args: Vec<Arg<'a, '_>> = vec![
            Arg::Str(event_name),
            Arg::Ident(owner_var),
            Arg::Expr(handler),
        ];
        if capture || passive {
            args.push(if capture {
                Arg::Bool(true)
            } else {
                Arg::Expr(self.ctx.b.void_zero_expr())
            });
        }
        if passive {
            args.push(Arg::Bool(true));
        }
        state
            .after_update
            .push(self.ctx.b.call_stmt("$.event", args));
        Ok(())
    }
}
