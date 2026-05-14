use crate::codegen::expr::coarse_wrap;
use oxc_ast::ast::{Expression, Statement};
use svelte_analyze::{AttributeSemantics, ExprKind, HtmlConcatSemantics};
use svelte_ast::{Attribute, ConcatPart, NodeId, StyleDirectiveValue};
use svelte_ast_builder::{Arg, AssignLeft, ObjProp};

use crate::context::Ctx;

use super::super::data_structures::{EmitState, MemoValueRef};
use super::super::{Codegen, CodegenError, Result};
use super::regular::literal_value;

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
        owner_var: &str,
        style_attr_id: Option<NodeId>,
    ) -> Result<()> {
        if !self.ctx.has_style_directives(owner_id) {
            return Ok(());
        }

        let style_attr_value = match style_attr_id {
            Some(id) => Some(self.build_style_attr_value(state, owner_id, id)?),
            None => None,
        };
        let style_attr_non_stateful = matches!(style_attr_value, Some((_, false)));
        let static_style = self.ctx.static_style(owner_id).unwrap_or("").to_string();
        let all_static = style_attr_value.is_none()
            && self
                .ctx
                .style_directives(owner_id)
                .iter()
                .all(|sd| match &sd.value {
                    StyleDirectiveValue::String(_) => true,
                    StyleDirectiveValue::Concatenation(parts) => parts.iter().all(|p| match p {
                        ConcatPart::Static(_) => true,
                        ConcatPart::Dynamic { expr, .. } => self
                            .ctx
                            .state
                            .parsed
                            .expr(expr.id())
                            .and_then(literal_value)
                            .is_some(),
                    }),
                    StyleDirectiveValue::Expression => false,
                });
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

        let directives_expr = if all_static {
            directives_expr
        } else {
            self.maybe_hoist_style_directives_obj(state, owner_id, directives_expr)
        };

        if all_static {
            let empty = self.ctx.b.object_expr(Vec::new());
            let set_style_call = self.ctx.b.call_expr(
                "$.set_style",
                [
                    Arg::Ident(owner_var),
                    Arg::Str(static_style),
                    Arg::Expr(empty),
                    Arg::Expr(directives_expr),
                ],
            );
            state.init.push(self.ctx.b.expr_stmt(set_style_call));
            return Ok(());
        }

        let value_expr = match style_attr_value {
            Some((expr, _)) => expr,
            None => self.ctx.b.str_expr(&static_style),
        };

        if style_attr_id.is_some()
            && style_attr_non_stateful
            && self.style_directives_all_literal(owner_id)
        {
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
        let el = self.ctx.element(owner_id);
        let attributes = el.attributes.clone();
        let Some(attr) = self
            .ctx
            .attr_index(owner_id)
            .and_then(|index| index.find_by_id(&attributes, style_attr_id))
        else {
            return CodegenError::unexpected_node(
                style_attr_id,
                "style attr id not found on element",
            );
        };

        match attr {
            Attribute::ExpressionAttribute(ea) => {
                let expr = self.take_attr_expr(style_attr_id, &ea.expression)?;
                let data = self.ctx.expression_data(style_attr_id).cloned();
                let expr = coarse_wrap(self.ctx, expr, data.as_ref());
                let needs_memo = data.as_ref().is_some_and(|d| match d.kind {
                    ExprKind::Async { has_await: true } => true,
                    ExprKind::Call { dynamic: true } => true,
                    ExprKind::KnownLiteral
                    | ExprKind::SimpleRead { .. }
                    | ExprKind::Computed { .. }
                    | ExprKind::Call { dynamic: false }
                    | ExprKind::Async { has_await: false } => false,
                });
                let has_state = !data
                    .as_ref()
                    .is_some_and(|d| matches!(d.kind, ExprKind::Call { dynamic: false }));
                if !needs_memo {
                    if let Some(d) = data.as_ref() {
                        state.shared_memo.push_expression_data(self.ctx, d);
                    }
                    return Ok((expr, has_state));
                }
                let Some(d) = data else {
                    return CodegenError::missing_expression_deps(style_attr_id);
                };
                let placeholder = match state.shared_memo.add_memoized_expr(self.ctx, &d, expr) {
                    Some(MemoValueRef::Sync(i)) => state.shared_memo.sync_param_expr(self.ctx, i),
                    Some(MemoValueRef::Async(i)) => state.shared_memo.async_param_expr(self.ctx, i),
                    None => return CodegenError::missing_expression_deps(style_attr_id),
                };
                Ok((placeholder, has_state))
            }
            Attribute::ConcatenationAttribute(a) => {
                let semantics: HtmlConcatSemantics =
                    match self.ctx.query.analysis.attributes.get(a.id) {
                        AttributeSemantics::HtmlConcat(s) => s.clone(),
                        _ => {
                            return CodegenError::semantic_mismatch(
                                a.id,
                                "style ConcatenationAttribute requires HtmlConcat semantics",
                            );
                        }
                    };
                let (expr, mut memo_deps) = self.build_html_concat_expr(a, &semantics)?;
                let has_state = !memo_deps.sync_values.is_empty()
                    || !memo_deps.async_values.is_empty();
                state.shared_memo.sync_values.append(&mut memo_deps.sync_values);
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

    fn style_directives_all_literal(&self, owner_id: NodeId) -> bool {
        self.ctx.style_directives(owner_id).iter().all(|sd| match &sd.value {
            StyleDirectiveValue::String(_) => true,
            StyleDirectiveValue::Concatenation(parts) => parts.iter().all(|p| match p {
                ConcatPart::Static(_) => true,
                ConcatPart::Dynamic { expr, .. } => self
                    .ctx
                    .state
                    .parsed
                    .expr(expr.id())
                    .and_then(literal_value)
                    .is_some(),
            }),
            StyleDirectiveValue::Expression => false,
        })
    }

    fn maybe_hoist_style_directives_obj(
        &mut self,
        state: &mut EmitState<'a>,
        owner_id: NodeId,
        dir_obj: Expression<'a>,
    ) -> Expression<'a> {
        use svelte_analyze::ExprKind;
        let dir_ids: Vec<NodeId> = self
            .ctx
            .style_directives(owner_id)
            .iter()
            .map(|sd| sd.id)
            .collect();
        let needs_hoist = dir_ids.iter().any(|&id| {
            self.ctx
                .expression_data(id)
                .is_some_and(|d| matches!(d.kind, ExprKind::Call { dynamic: true }))
        });
        if !needs_hoist {
            return dir_obj;
        }
        for &id in &dir_ids {
            if let Some(data) = self.ctx.expression_data(id) {
                state.shared_memo.push_expression_data(self.ctx, data);
            }
        }
        let idx = state.shared_memo.sync_values_push(dir_obj);
        state.shared_memo.sync_param_expr(self.ctx, idx)
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
