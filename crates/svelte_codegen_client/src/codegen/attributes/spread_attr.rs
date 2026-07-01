use crate::codegen::expr::coarse_wrap;
use oxc_ast::ast::Expression;
use svelte_analyze::{AttributeSemantics, SpecialValueKind, Volatility};
use svelte_ast::{Attribute, NodeId};
use svelte_ast_builder::{Arg, ObjProp};

use super::super::data_structures::{EmitState, TemplateMemoState};
use super::super::{Codegen, CodegenError, Result};

fn expr_is_ident_named(expr: &Expression<'_>, name: &str) -> bool {
    matches!(expr.get_inner_expression(), Expression::Identifier(id) if id.name.as_str() == name)
}

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(in super::super) fn emit_attr_spread(
        &mut self,
        state: &mut EmitState<'a>,
        owner_id: NodeId,
        owner_tag: &str,
        owner_var: &str,
        attributes: &[Attribute],
    ) -> Result<()> {
        let mut options = SpreadOptions::for_regular_element();
        options.is_input = owner_tag == "input" && self.ctx.needs_input_defaults(owner_id);
        self.emit_attr_spread_full(state, owner_id, owner_tag, owner_var, attributes, options)?;
        Ok(())
    }

    pub(in super::super) fn emit_attr_spread_full(
        &mut self,
        state: &mut EmitState<'a>,
        owner_id: NodeId,
        owner_tag: &str,
        owner_var: &str,
        attributes: &[Attribute],
        options: SpreadOptions,
    ) -> Result<Option<Expression<'a>>> {
        let mut props: Vec<ObjProp<'a>> = Vec::new();
        let mut ns_thunk: Option<Expression<'a>> = None;
        let mut memo: TemplateMemoState<'a> = TemplateMemoState::default();

        if !options.skip_directives {
            for attr in attributes {
                match attr {
                    Attribute::UseDirective(d) => {
                        self.emit_use_directive(state, owner_id, owner_var, d)?;
                    }
                    Attribute::OnDirectiveLegacy(d) => {
                        self.emit_on_directive_legacy(state, owner_id, owner_var, d)?;
                    }
                    Attribute::TransitionDirective(d) => {
                        self.emit_transition_directive(state, owner_id, owner_var, d)?;
                    }
                    Attribute::AnimateDirective(d) => {
                        self.emit_animate_directive(state, owner_id, owner_var, d)?;
                    }
                    Attribute::BindDirective(d) => {
                        self.emit_bind_directive(state, owner_id, owner_tag, owner_var, d)?;
                    }
                    _ => {}
                }
            }
        }

        for attr in attributes {
            let attr_id = attr.id();
            match attr {
                Attribute::BooleanAttribute(a) => {
                    let name_alloc = self.ctx.b.alloc_str(&a.name);
                    props.push(ObjProp::KeyValue(name_alloc, self.ctx.b.bool_expr(true)));
                }
                Attribute::StringAttribute(a) => {
                    let val = a.value(&self.ctx.query.component.source).to_string();
                    if matches!(
                        self.ctx.query.analysis.attributes.get(a.id),
                        AttributeSemantics::StaticAttr
                    ) {
                        state.template.set_attribute(&a.name, Some(val));
                        continue;
                    }
                    let name_alloc = self.ctx.b.alloc_str(&a.name);
                    props.push(ObjProp::KeyValue(name_alloc, self.ctx.b.str_expr(&val)));
                }
                Attribute::ExpressionAttribute(a) => {
                    let preserves_for_bind_group = matches!(
                        self.ctx.query.analysis.attributes.get(attr_id),
                        AttributeSemantics::SpecialValueAttr(s)
                            if matches!(s.kind, SpecialValueKind::InputBindGroup)
                    );
                    let backup = if preserves_for_bind_group {
                        self.ctx
                            .state
                            .parsed
                            .expr(a.expression.id())
                            .map(|e| self.ctx.b.clone_expr(e))
                    } else {
                        None
                    };
                    let expr = self.take_attr_expr(attr_id, &a.expression)?;
                    if let Some(backup) = backup {
                        self.ctx
                            .state
                            .parsed
                            .replace_expr(a.expression.id(), backup);
                    }
                    let expr = {
                        let data = self.ctx.expression_data(attr_id).cloned();
                        coarse_wrap(self.ctx, expr, data.as_ref())
                    };
                    let is_event = a.event_name.is_some();
                    let is_fn = matches!(
                        expr.get_inner_expression(),
                        Expression::ArrowFunctionExpression(_) | Expression::FunctionExpression(_)
                    );
                    if is_event && is_fn {
                        let handler_name = self.ctx.state.gen_ident("event_handler");
                        state.init.push(self.ctx.b.var_stmt(&handler_name, expr));
                        let name_alloc = self.ctx.b.alloc_str(&a.name);
                        props.push(ObjProp::KeyValue(
                            name_alloc,
                            self.ctx.b.rid_expr(&handler_name),
                        ));
                        continue;
                    }

                    if a.name == "xmlns" {
                        let clone = self.ctx.b.clone_expr(&expr);
                        ns_thunk = Some(self.ctx.b.thunk(clone));
                    }
                    let name_alloc = self.ctx.b.alloc_str(&a.name);
                    match self.ctx.expression_data(attr_id).map(|d| d.volatility) {
                        Some(Volatility::Heavy) => {
                            if let Some(data) = self.ctx.expression_data(attr_id) {
                                memo.push_expression_data(self.ctx, data);
                            }
                            let idx = memo.sync_values_push(expr);
                            let param = memo.sync_param_expr(self.ctx, idx);
                            props.push(ObjProp::KeyValue(name_alloc, param));
                        }
                        Some(
                            Volatility::Static | Volatility::Reactive | Volatility::Asynchronous,
                        )
                        | None => {
                            if self.ctx.is_expression_shorthand(attr_id)
                                && expr_is_ident_named(&expr, &a.name)
                            {
                                props.push(ObjProp::Shorthand(name_alloc));
                            } else {
                                props.push(ObjProp::KeyValue(name_alloc, expr));
                            }
                        }
                    }
                }
                Attribute::ConcatenationAttribute(a) => {
                    let semantics = match self.ctx.query.analysis.attributes.get(attr_id) {
                        AttributeSemantics::HtmlConcat(s) => s.clone(),
                        _ => {
                            return CodegenError::semantic_mismatch(
                                attr_id,
                                "ConcatenationAttribute requires HtmlConcat semantics",
                            );
                        }
                    };
                    let val = self.build_html_concat_expr(a, &semantics, &mut memo)?;
                    let name_alloc = self.ctx.b.alloc_str(&a.name);
                    props.push(ObjProp::KeyValue(name_alloc, val));
                }
                Attribute::SpreadAttribute(sa) => {
                    let expr = self.take_attr_expr(attr_id, &sa.expression)?;
                    match self.ctx.expression_data(attr_id).map(|d| d.volatility) {
                        Some(Volatility::Heavy) => {
                            if let Some(data) = self.ctx.expression_data(attr_id) {
                                memo.push_expression_data(self.ctx, data);
                            }
                            let idx = memo.sync_values_push(expr);
                            let param = memo.sync_param_expr(self.ctx, idx);
                            props.push(ObjProp::Spread(param));
                        }
                        Some(
                            Volatility::Static | Volatility::Reactive | Volatility::Asynchronous,
                        )
                        | None => {
                            props.push(ObjProp::Spread(expr));
                        }
                    }
                }
                Attribute::BindDirective(_)
                | Attribute::LetDirectiveLegacy(_)
                | Attribute::ClassDirective(_)
                | Attribute::StyleDirective(_)
                | Attribute::UseDirective(_)
                | Attribute::OnDirectiveLegacy(_)
                | Attribute::TransitionDirective(_)
                | Attribute::AnimateDirective(_)
                | Attribute::AttachTag(_) => continue,
            }
        }

        let class_directives_obj = if options.include_class_directives {
            self.build_class_directives_object(owner_id)?
        } else {
            None
        };

        if class_directives_obj.is_some() && self.ctx.needs_class_base(owner_id) {
            let class_key = self.ctx.b.alloc_str("class");
            props.push(ObjProp::KeyValue(class_key, self.ctx.b.str_expr("")));
        }
        if self.ctx.has_style_directives(owner_id) && self.ctx.needs_style_base(owner_id) {
            let style_key = self.ctx.b.alloc_str("style");
            props.push(ObjProp::KeyValue(style_key, self.ctx.b.str_expr("")));
        }

        if let Some(class_obj) = class_directives_obj {
            let volatility = self.ctx.query.view.class_directives_volatility(owner_id);
            let class_obj =
                super::hoist_directives_object(self.ctx, &mut memo, volatility, class_obj);
            let class_key_expr = self
                .ctx
                .b
                .static_member_expr(self.ctx.b.rid_expr("$"), "CLASS");
            props.push(ObjProp::Computed(class_key_expr, class_obj));
        }

        if self.ctx.has_style_directives(owner_id) {
            let style_props = self.build_style_props(owner_id)?;
            let style_obj = if style_props.important.is_empty() {
                self.ctx.b.object_expr(style_props.normal)
            } else {
                let normal_obj = self.ctx.b.object_expr(style_props.normal);
                let important_obj = self.ctx.b.object_expr(style_props.important);
                self.ctx
                    .b
                    .array_from_args([Arg::Expr(normal_obj), Arg::Expr(important_obj)])
            };
            let volatility = self.ctx.query.view.style_directives_volatility(owner_id);
            let style_obj =
                super::hoist_directives_object(self.ctx, &mut memo, volatility, style_obj);
            let style_key_expr = self
                .ctx
                .b
                .static_member_expr(self.ctx.b.rid_expr("$"), "STYLE");
            props.push(ObjProp::Computed(style_key_expr, style_obj));
        }

        if !props.is_empty() {
            let obj = self.ctx.b.object_expr(props);
            let param_names = memo.param_names();
            let params = if param_names.is_empty() {
                self.ctx.b.no_params()
            } else {
                self.ctx.b.params(param_names.iter().map(|s| s.as_str()))
            };
            let arrow = self.ctx.b.arrow_expr(params, [self.ctx.b.expr_stmt(obj)]);
            let hash = self.ctx.css_hash().to_string();
            let css_scoped = self.ctx.is_css_scoped(owner_id) && !hash.is_empty();
            let has_deps = memo.has_deps();
            let hydration_ignored = self.ctx.hydration_attribute_changed_ignored(owner_id);
            let mut args: Vec<Arg<'a, '_>> = if css_scoped || options.is_input {
                let sync_arg = if memo.has_sync_values() {
                    memo.sync_values_expr(self.ctx)
                } else {
                    self.ctx.b.void_zero_expr()
                };
                let async_arg = if memo.has_async_values() {
                    memo.async_values_expr(self.ctx)
                } else {
                    self.ctx.b.void_zero_expr()
                };
                let blockers_arg = if memo.has_blockers() {
                    memo.blockers_expr(self.ctx)
                } else {
                    self.ctx.b.void_zero_expr()
                };
                let hash_arg = if css_scoped {
                    Arg::Str(hash)
                } else {
                    Arg::Expr(self.ctx.b.void_zero_expr())
                };
                let mut args = vec![
                    Arg::Ident(owner_var),
                    Arg::Expr(arrow),
                    Arg::Expr(sync_arg),
                    Arg::Expr(async_arg),
                    Arg::Expr(blockers_arg),
                    hash_arg,
                ];
                if options.is_input {
                    args.push(Arg::Bool(true));
                }
                args
            } else if has_deps {
                let mut args = vec![Arg::Ident(owner_var), Arg::Expr(arrow)];
                args.push(Arg::Expr(memo.sync_values_expr(self.ctx)));
                if memo.has_async_values() || memo.has_blockers() {
                    args.push(Arg::Expr(memo.async_values_expr(self.ctx)));
                }
                if memo.has_blockers() {
                    args.push(Arg::Expr(memo.blockers_expr(self.ctx)));
                }
                args
            } else {
                vec![Arg::Ident(owner_var), Arg::Expr(arrow)]
            };
            if hydration_ignored {
                while args.len() < 7 {
                    args.push(Arg::Expr(self.ctx.b.void_zero_expr()));
                }
                args.push(Arg::Bool(true));
            }
            state
                .init
                .push(self.ctx.b.call_stmt("$.attribute_effect", args));
        }

        Ok(ns_thunk)
    }
}

pub(in super::super) struct SpreadOptions {
    pub include_class_directives: bool,
    pub is_input: bool,
    pub skip_directives: bool,
}

impl SpreadOptions {
    pub fn for_regular_element() -> Self {
        Self {
            include_class_directives: true,
            is_input: false,
            skip_directives: false,
        }
    }

    pub fn for_svelte_element(include_class_directives: bool) -> Self {
        Self {
            include_class_directives,
            is_input: false,
            skip_directives: true,
        }
    }
}
