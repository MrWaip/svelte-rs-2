use crate::codegen::expr::coarse_wrap;
use oxc_ast::ast::{Expression, Statement};
use oxc_syntax::node::NodeId as OxcNodeId;
use svelte_analyze::Volatility;
use svelte_ast::{Attribute, NodeId};
use svelte_ast_builder::{Arg, AssignLeft, ObjProp};

use crate::context::Ctx;

use super::super::data_structures::{EmitState, TemplateMemoState};
use super::super::{Codegen, CodegenError, Result};

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(in super::super) fn emit_class_attribute_and_directives(
        &mut self,
        state: &mut EmitState<'a>,
        owner_id: NodeId,
        owner_var: &str,
        is_html: bool,
    ) -> Result<()> {
        let has_class_attr = self.ctx.has_class_attribute(owner_id);
        let has_class_dirs = self.ctx.has_class_directives(owner_id);

        if !has_class_attr && !has_class_dirs {
            return Ok(());
        }

        let class_value = if has_class_attr {
            let Some(class_attr_id) = self.ctx.class_attr_id(owner_id) else {
                return CodegenError::unexpected_node(
                    owner_id,
                    "element with class attribute should have class_attr_id",
                );
            };
            self.build_class_attr_value(owner_id, class_attr_id, &mut state.shared_memo)?
        } else {
            let static_class = self.ctx.static_class(owner_id).unwrap_or("").to_string();
            let hash = self.ctx.css_hash().to_string();
            if self.ctx.is_css_scoped(owner_id) && !hash.is_empty() {
                let combined = if static_class.is_empty() {
                    hash
                } else {
                    format!("{static_class} {hash}")
                };
                self.ctx.b.str_expr(&combined)
            } else {
                self.ctx.b.str_expr(&static_class)
            }
        };

        let directives_obj = if has_class_dirs {
            self.build_class_directives_object(owner_id)?
        } else {
            None
        };

        let class_volatility = self.ctx.class_state_volatility(owner_id);

        let directives_obj = match (directives_obj, class_volatility) {
            (Some(obj), Volatility::Reactive | Volatility::Heavy | Volatility::Asynchronous) => {
                Some(self.maybe_hoist_class_directives_obj(state, owner_id, obj))
            }
            (other, _) => other,
        };

        let hash = self.ctx.css_hash().to_string();
        let scope_hash = (has_class_attr && self.ctx.is_css_scoped(owner_id) && !hash.is_empty())
            .then_some(hash);

        emit_set_class_call(
            self.ctx,
            &mut state.init,
            &mut state.update,
            owner_var,
            class_value,
            scope_hash.as_deref(),
            directives_obj,
            class_volatility,
            is_html,
        );

        Ok(())
    }

    fn maybe_hoist_class_directives_obj(
        &mut self,
        state: &mut EmitState<'a>,
        owner_id: NodeId,
        dir_obj: Expression<'a>,
    ) -> Expression<'a> {
        let Some(dirs) = self.ctx.query.view.class_directive_info(owner_id) else {
            return dir_obj;
        };
        let dir_ids: Vec<NodeId> = dirs.iter().map(|cd| cd.id).collect();
        let Some(_) =
            dir_ids.iter().find(
                |&&id| match self.ctx.expression_data(id).map(|d| d.volatility) {
                    Some(Volatility::Heavy) => true,
                    Some(Volatility::Static | Volatility::Reactive | Volatility::Asynchronous)
                    | None => false,
                },
            )
        else {
            return dir_obj;
        };
        for &id in &dir_ids {
            if let Some(data) = self.ctx.expression_data(id) {
                state.shared_memo.push_expression_data(self.ctx, data);
            }
        }
        let idx = state.shared_memo.sync_values_push(dir_obj);
        state.shared_memo.sync_param_expr(self.ctx, idx)
    }

    fn build_class_attr_value(
        &mut self,
        owner_id: NodeId,
        class_attr_id: NodeId,
        memo: &mut TemplateMemoState<'a>,
    ) -> Result<Expression<'a>> {
        let el = self.ctx.element(owner_id);
        let attributes = el.attributes.clone();

        let Some(attr) = self
            .ctx
            .attr_index(owner_id)
            .and_then(|index| index.find_by_id(&attributes, class_attr_id))
        else {
            return CodegenError::unexpected_node(
                class_attr_id,
                "class attr id not found on element",
            );
        };

        match attr {
            Attribute::ExpressionAttribute(ea) => {
                let mut expr = self.take_attr_expr(class_attr_id, &ea.expression)?;
                let _ = ea;
                let data = self.ctx.expression_data(class_attr_id).cloned();
                expr = coarse_wrap(self.ctx, expr, data.as_ref());
                if self.ctx.needs_clsx(class_attr_id) {
                    expr = self.ctx.b.call_expr("$.clsx", [Arg::Expr(expr)]);
                }
                let class_value = match data.as_ref().map(|d| d.volatility) {
                    Some(Volatility::Heavy) => {
                        if let Some(d) = &data {
                            memo.push_expression_data(self.ctx, d);
                        }
                        let index = memo.sync_values_push(expr);
                        memo.sync_param_expr(self.ctx, index)
                    }
                    Some(Volatility::Asynchronous) => {
                        if let Some(d) = &data {
                            memo.push_expression_data(self.ctx, d);
                        }
                        let index = memo.async_values_push(expr);
                        memo.async_param_expr(self.ctx, index)
                    }
                    _ => expr,
                };
                Ok(class_value)
            }
            Attribute::ConcatenationAttribute(a) => self.build_html_concat_for_class(a, memo),
            _ => CodegenError::unexpected_node(
                class_attr_id,
                "class_attr_id must reference ExpressionAttribute or ConcatenationAttribute",
            ),
        }
    }

    fn build_html_concat_for_class(
        &mut self,
        attr: &svelte_ast::ConcatenationAttribute,
        memo: &mut TemplateMemoState<'a>,
    ) -> Result<Expression<'a>> {
        use svelte_analyze::{AttributeSemantics, HtmlConcatSemantics};
        let semantics: HtmlConcatSemantics = match self.ctx.query.analysis.attributes.get(attr.id) {
            AttributeSemantics::HtmlConcat(s) => s.clone(),
            _ => {
                return CodegenError::semantic_mismatch(
                    attr.id,
                    "class ConcatenationAttribute requires HtmlConcat semantics",
                );
            }
        };
        self.build_html_concat_expr(attr, &semantics, memo)
    }

    pub(in super::super) fn emit_svelte_element_class_directives(
        &mut self,
        state: &mut EmitState<'a>,
        owner_id: NodeId,
        owner_var: &str,
    ) -> Result<()> {
        let dir_obj_opt = self.build_class_directives_object(owner_id)?;
        let Some(dir_obj) = dir_obj_opt else {
            return Ok(());
        };

        let class_volatility = self.ctx.class_state_volatility(owner_id);
        let dir_obj = match class_volatility {
            Volatility::Static => dir_obj,
            Volatility::Reactive | Volatility::Heavy | Volatility::Asynchronous => {
                self.maybe_hoist_class_directives_obj(state, owner_id, dir_obj)
            }
        };

        let static_value = self.ctx.static_class(owner_id).unwrap_or("").to_string();
        let hash = self.ctx.css_hash().to_string();
        let scoped = self.ctx.is_css_scoped(owner_id) && !hash.is_empty();
        let value = if scoped {
            if static_value.is_empty() {
                hash
            } else {
                format!("{static_value} {hash}")
            }
        } else {
            static_value
        };
        let class_value = self.ctx.b.str_expr(&value);

        emit_set_class_call(
            self.ctx,
            &mut state.init,
            &mut state.update,
            owner_var,
            class_value,
            None,
            Some(dir_obj),
            class_volatility,
            false,
        );
        Ok(())
    }

    pub(in super::super) fn build_class_directives_object(
        &mut self,
        owner_id: NodeId,
    ) -> Result<Option<Expression<'a>>> {
        let dir_snapshot: Vec<(NodeId, String, bool, OxcNodeId)> =
            match self.ctx.query.view.class_directive_info(owner_id) {
                Some(dirs) => dirs
                    .iter()
                    .map(|cd| (cd.id, cd.name.clone(), cd.has_expression, cd.expr_id))
                    .collect(),
                None => return Ok(None),
            };

        let class_volatility = self.ctx.class_state_volatility(owner_id);
        let mut props: Vec<ObjProp<'a>> = Vec::new();
        for (id, name, has_expression, expr_id) in &dir_snapshot {
            let (expr, same_name) = if *has_expression {
                let Some(parsed) = self.ctx.state.parsed.take_expr(*expr_id) else {
                    return CodegenError::missing_expression(*id);
                };
                let parsed = match class_volatility {
                    Volatility::Static => parsed,
                    Volatility::Reactive | Volatility::Heavy | Volatility::Asynchronous => {
                        self.maybe_wrap_legacy_slots_read(parsed)
                    }
                };
                (parsed, self.ctx.is_expression_shorthand(*id))
            } else {
                (self.ctx.b.rid_expr(name), true)
            };
            props.push(self.ctx.b.directive_prop(name, expr, same_name));
        }

        Ok(Some(self.ctx.b.object_expr(props)))
    }
}

fn emit_set_class_call<'a>(
    ctx: &mut Ctx<'a>,
    init: &mut Vec<Statement<'a>>,
    update: &mut Vec<Statement<'a>>,
    el_name: &str,
    class_value: Expression<'a>,
    scope_hash: Option<&str>,
    directives_obj: Option<Expression<'a>>,
    class_volatility: Volatility,
    is_html: bool,
) {
    let scope_expr = |ctx: &mut Ctx<'a>| match scope_hash {
        Some(h) => ctx.b.str_expr(h),
        None => ctx.b.null_expr(),
    };

    if let Some(dir_obj) = directives_obj {
        match class_volatility {
            Volatility::Reactive | Volatility::Heavy | Volatility::Asynchronous => {
                let classes_name = ctx.gen_ident("classes");
                let scope = scope_expr(ctx);
                let set_class_call = ctx.b.call_expr(
                    "$.set_class",
                    [
                        Arg::Ident(el_name),
                        Arg::Num(if is_html { 1.0 } else { 0.0 }),
                        Arg::Expr(class_value),
                        Arg::Expr(scope),
                        Arg::Ident(&classes_name),
                        Arg::Expr(dir_obj),
                    ],
                );
                let assign = ctx
                    .b
                    .assign_expr(AssignLeft::Ident(classes_name.clone()), set_class_call);
                init.push(ctx.b.let_stmt(&classes_name));
                update.push(ctx.b.expr_stmt(assign));
            }
            Volatility::Static => {
                let scope = scope_expr(ctx);
                let set_class_call = ctx.b.call_expr(
                    "$.set_class",
                    [
                        Arg::Ident(el_name),
                        Arg::Num(if is_html { 1.0 } else { 0.0 }),
                        Arg::Expr(class_value),
                        Arg::Expr(scope),
                        Arg::Expr(ctx.b.object_expr(vec![])),
                        Arg::Expr(dir_obj),
                    ],
                );
                init.push(ctx.b.expr_stmt(set_class_call));
            }
        }
    } else {
        let mut args = vec![
            Arg::Ident(el_name),
            Arg::Num(if is_html { 1.0 } else { 0.0 }),
            Arg::Expr(class_value),
        ];
        if let Some(h) = scope_hash {
            args.push(Arg::Expr(ctx.b.str_expr(h)));
        }
        let set_class_call = ctx.b.call_expr("$.set_class", args);
        let target = match class_volatility {
            Volatility::Static => &mut *init,
            Volatility::Reactive | Volatility::Heavy | Volatility::Asynchronous => &mut *update,
        };
        target.push(ctx.b.expr_stmt(set_class_call));
    }
}
