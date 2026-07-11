use crate::codegen::expr::coarse_wrap;
use oxc_ast::ast::{Expression, Statement};
use svelte_analyze::{HtmlConcatSemantics, Volatility};
use svelte_ast::{Attribute, NodeId, StyleDirectiveValue};
use svelte_ast_builder::{Arg, AssignLeft, ObjProp};

use crate::context::Ctx;

use super::super::data_structures::{EmitState, MemoValueRef, TemplateMemoState};
use super::super::{Codegen, CodegenError, Result};

pub(super) struct StyleProps<'a> {
    pub normal: Vec<ObjProp<'a>>,
    pub important: Vec<ObjProp<'a>>,
}

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(super) fn build_style_props(&mut self, owner_id: NodeId) -> Result<StyleProps<'a>> {
        let style_dirs = self.ctx.style_directives(owner_id).to_vec();
        let mut normal: Vec<ObjProp<'a>> = Vec::new();
        let mut important: Vec<ObjProp<'a>> = Vec::new();

        for sd in &style_dirs {
            let name = &sd.name;
            let prop = match &sd.value {
                StyleDirectiveValue::Expression => {
                    let parsed = self.take_attr_expr(sd.id, &sd.expression)?;
                    let data = self.ctx.expression_data(sd.id).cloned();
                    let parsed = coarse_wrap(self.ctx, parsed, data.as_ref());
                    let same_name = sd.shorthand || self.ctx.is_expression_shorthand(sd.id);
                    self.ctx.b.directive_prop(name, parsed, same_name)
                }
                StyleDirectiveValue::String(s) => {
                    let name_alloc = self.ctx.b.alloc_str(name);
                    ObjProp::KeyValue(name_alloc, self.ctx.b.str_expr(s))
                }
                StyleDirectiveValue::Concatenation(parts) => {
                    let parts = parts.clone();
                    let name_alloc = self.ctx.b.alloc_str(name);
                    let expr = self.build_concat_expr_collapse_single(sd.id, &parts)?;
                    ObjProp::KeyValue(name_alloc, expr)
                }
            };
            if sd.important {
                important.push(prop);
            } else {
                normal.push(prop);
            }
        }

        Ok(StyleProps { normal, important })
    }

    pub(in super::super) fn emit_style_directives_aggregate(
        &mut self,
        state: &mut EmitState<'a>,
        owner_id: NodeId,
        owner_tag: &str,
        owner_var: &str,
        _style_attr_id: Option<NodeId>,
    ) -> Result<()> {
        let Some((style_attr_id, style_static_attr, has_dirs, stateful)) =
            self.ctx.style_semantics(owner_id).map(|s| {
                (
                    s.attr,
                    s.static_attr,
                    !s.directives.is_empty(),
                    s.state_volatility.is_volatile(),
                )
            })
        else {
            return Ok(());
        };

        if !has_dirs {
            if let Some(attr_id) = style_attr_id {
                let attrs = self.ctx.node_attributes(owner_id);
                match self
                    .ctx
                    .attr_index(owner_id)
                    .and_then(|index| index.find_by_id(attrs, attr_id))
                {
                    Some(Attribute::ExpressionAttribute(a)) => {
                        self.emit_attr_expression(state, owner_id, owner_tag, owner_var, a)?;
                    }
                    Some(Attribute::ConcatenationAttribute(a)) => {
                        self.emit_attr_concatenation(state, owner_id, owner_tag, owner_var, a)?;
                    }
                    _ => {}
                }
            } else if style_static_attr.is_some() {
                let static_style = self.ctx.static_style(owner_id).unwrap_or("").to_string();
                if self.ctx.query.view.is_custom_element(owner_id) {
                    let style_call = self.ctx.b.call_expr(
                        "$.set_style",
                        [Arg::Ident(owner_var), Arg::Str(static_style)],
                    );
                    state.init.push(self.ctx.b.expr_stmt(style_call));
                } else {
                    state.template.set_attribute("style", Some(static_style));
                }
            }
            return Ok(());
        }

        let style_attr_value = match style_attr_id {
            Some(id) => Some(self.build_style_attr_value(state, owner_id, id)?),
            None => None,
        };
        let static_style = self.ctx.static_style(owner_id).unwrap_or("").to_string();
        let props = self.build_style_props(owner_id)?;

        let directives_expr = if props.important.is_empty() {
            self.ctx.b.object_expr(props.normal)
        } else {
            let normal_obj = self.ctx.b.object_expr(props.normal);
            let important_obj = self.ctx.b.object_expr(props.important);
            self.ctx
                .b
                .array_from_args([Arg::Expr(normal_obj), Arg::Expr(important_obj)])
        };

        let value_expr = match style_attr_value {
            Some((expr, _)) => expr,
            None => self.ctx.b.str_expr(&static_style),
        };

        if !stateful {
            let empty = self.ctx.b.object_expr(Vec::new());
            let set_style_call = self.ctx.b.call_expr(
                "$.set_style",
                [
                    Arg::Ident(owner_var),
                    Arg::Expr(value_expr),
                    Arg::Expr(empty),
                    Arg::Expr(directives_expr),
                ],
            );
            state.init.push(self.ctx.b.expr_stmt(set_style_call));
            return Ok(());
        }

        let directives_expr =
            self.maybe_hoist_style_directives_obj(state, owner_id, directives_expr);

        emit_set_style_call(
            self.ctx,
            &mut state.init,
            &mut state.update,
            owner_var,
            value_expr,
            directives_expr,
        );

        Ok(())
    }

    fn build_style_attr_value(
        &mut self,
        state: &mut EmitState<'a>,
        owner_id: NodeId,
        style_attr_id: NodeId,
    ) -> Result<(Expression<'a>, bool)> {
        let attributes = self.ctx.node_attributes(owner_id);
        let Some(attr) = self
            .ctx
            .attr_index(owner_id)
            .and_then(|index| index.find_by_id(attributes, style_attr_id))
        else {
            return CodegenError::unexpected_node(
                style_attr_id,
                "style attr id not found on element",
            );
        };

        match attr {
            Attribute::ExpressionAttribute(ea) => {
                let data = self.ctx.expression_data(style_attr_id).cloned();
                let expr = self.take_attr_expr(style_attr_id, &ea.expression)?;
                let expr = coarse_wrap(self.ctx, expr, data.as_ref());
                match data.as_ref().map(|d| d.volatility) {
                    Some(Volatility::Heavy | Volatility::Asynchronous) => {
                        let Some(d) = data else {
                            return CodegenError::missing_expression_deps(style_attr_id);
                        };
                        let placeholder =
                            match state.shared_memo.add_memoized_expr(self.ctx, &d, expr) {
                                Some(MemoValueRef::Sync(i)) => {
                                    state.shared_memo.sync_param_expr(self.ctx, i)
                                }
                                Some(MemoValueRef::Async(i)) => {
                                    state.shared_memo.async_param_expr(self.ctx, i)
                                }
                                None => {
                                    return CodegenError::missing_expression_deps(style_attr_id);
                                }
                            };
                        Ok((placeholder, true))
                    }
                    Some(Volatility::Reactive) => {
                        if let Some(d) = data.as_ref() {
                            state.shared_memo.push_expression_data(self.ctx, d);
                        }
                        Ok((expr, true))
                    }
                    Some(Volatility::Static) | None => {
                        if let Some(d) = data.as_ref() {
                            state.shared_memo.push_expression_data(self.ctx, d);
                        }
                        Ok((expr, false))
                    }
                }
            }
            Attribute::ConcatenationAttribute(a) => {
                let semantics: HtmlConcatSemantics = match self
                    .ctx
                    .style_semantics(owner_id)
                    .and_then(|s| s.attr_concat.clone())
                {
                    Some(semantics) => semantics,
                    None => {
                        return CodegenError::semantic_mismatch(
                            a.id,
                            "style ConcatenationAttribute requires attr_concat semantics",
                        );
                    }
                };
                let mut memo_deps = TemplateMemoState::default();
                let expr = self.build_html_concat_expr(a, &semantics, &mut memo_deps)?;
                let has_state =
                    !memo_deps.sync_values.is_empty() || !memo_deps.async_values.is_empty();
                state
                    .shared_memo
                    .sync_values
                    .append(&mut memo_deps.sync_values);
                state
                    .shared_memo
                    .async_values
                    .append(&mut memo_deps.async_values);
                for idx in memo_deps.blockers {
                    state.shared_memo.push_script_blocker(idx);
                }
                state
                    .shared_memo
                    .extra_blockers
                    .append(&mut memo_deps.extra_blockers);
                Ok((expr, has_state))
            }
            _ => CodegenError::unexpected_node(
                style_attr_id,
                "style_attr_id must reference ExpressionAttribute or ConcatenationAttribute",
            ),
        }
    }

    fn maybe_hoist_style_directives_obj(
        &mut self,
        state: &mut EmitState<'a>,
        owner_id: NodeId,
        dir_obj: Expression<'a>,
    ) -> Expression<'a> {
        let volatility = self.ctx.style_directives_volatility(owner_id);
        super::hoist_directives_object(self.ctx, &mut state.shared_memo, volatility, dir_obj)
    }
}

fn emit_set_style_call<'a>(
    ctx: &mut Ctx<'a>,
    init: &mut Vec<Statement<'a>>,
    update: &mut Vec<Statement<'a>>,
    el_name: &str,
    value_expr: Expression<'a>,
    directives_expr: Expression<'a>,
) {
    let styles_name = ctx.gen_ident("styles");

    let set_style_call = ctx.b.call_expr(
        "$.set_style",
        [
            Arg::Ident(el_name),
            Arg::Expr(value_expr),
            Arg::Ident(&styles_name),
            Arg::Expr(directives_expr),
        ],
    );

    let assign = ctx
        .b
        .assign_expr(AssignLeft::Ident(styles_name.clone()), set_style_call);

    init.push(ctx.b.let_stmt(&styles_name));
    update.push(ctx.b.expr_stmt(assign));
}
