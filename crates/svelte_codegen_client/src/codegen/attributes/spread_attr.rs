use crate::codegen::expr::coarse_wrap;
use oxc_ast::ast::Expression;
use svelte_analyze::ExprKind;
use svelte_ast::{Attribute, NodeId};
use svelte_ast_builder::{Arg, ObjProp};

use super::super::data_structures::{EmitState, TemplateMemoState};
use super::super::{Codegen, Result};

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

        for attr in attributes {
            let attr_id = attr.id();
            match attr {
                Attribute::BooleanAttribute(a) => {
                    let name_alloc = self.ctx.b.alloc_str(&a.name);
                    props.push(ObjProp::KeyValue(name_alloc, self.ctx.b.bool_expr(true)));
                }
                Attribute::StringAttribute(a) => {
                    let val = a.value(&self.ctx.query.component.source).to_string();
                    let name_alloc = self.ctx.b.alloc_str(&a.name);
                    props.push(ObjProp::KeyValue(name_alloc, self.ctx.b.str_expr(&val)));
                }
                Attribute::ExpressionAttribute(a) => {
                    let expr = self.take_attr_expr(attr_id, &a.expression)?;
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
                    if self.ctx.is_expression_shorthand(attr_id)
                        && expr_is_ident_named(&expr, &a.name)
                    {
                        props.push(ObjProp::Shorthand(name_alloc));
                    } else {
                        props.push(ObjProp::KeyValue(name_alloc, expr));
                    }
                }
                Attribute::ConcatenationAttribute(a) => {
                    let val = self.build_concat_expr_collapse_single(attr_id, &a.parts)?;
                    let name_alloc = self.ctx.b.alloc_str(&a.name);
                    props.push(ObjProp::KeyValue(name_alloc, val));
                }
                Attribute::SpreadAttribute(sa) => {
                    let expr = self.take_attr_expr(attr_id, &sa.expression)?;
                    let needs_hoist = self.ctx.expression_data(attr_id).is_some_and(|d| {
                        matches!(d.kind, ExprKind::Call { dynamic: true })
                    });
                    if needs_hoist {
                        if let Some(data) = self.ctx.expression_data(attr_id) {
                            memo.push_expression_data(self.ctx, data);
                        }
                        let idx = memo.sync_values_push(expr);
                        let param = memo.sync_param_expr(self.ctx, idx);
                        props.push(ObjProp::Spread(param));
                    } else {
                        props.push(ObjProp::Spread(expr));
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

        if class_directives_obj.is_some() && options.include_class_base {
            let class_key = self.ctx.b.alloc_str("class");
            props.push(ObjProp::KeyValue(class_key, self.ctx.b.str_expr("")));
        }
        if self.ctx.has_style_directives(owner_id) && options.include_style_base {
            let style_key = self.ctx.b.alloc_str("style");
            props.push(ObjProp::KeyValue(style_key, self.ctx.b.str_expr("")));
        }

        if let Some(class_obj) = class_directives_obj {
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
            let args: Vec<Arg<'a, '_>> = if css_scoped || options.is_input {
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
            state
                .init
                .push(self.ctx.b.call_stmt("$.attribute_effect", args));
        }

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

        Ok(ns_thunk)
    }
}

pub(in super::super) struct SpreadOptions {
    pub include_class_directives: bool,
    pub include_class_base: bool,
    pub include_style_base: bool,
    pub is_input: bool,
    pub skip_directives: bool,
}

impl SpreadOptions {
    pub fn for_regular_element() -> Self {
        Self {
            include_class_directives: true,
            include_class_base: false,
            include_style_base: false,
            is_input: false,
            skip_directives: false,
        }
    }

    pub fn for_svelte_element(
        include_class_directives: bool,
        include_class_base: bool,
        include_style_base: bool,
    ) -> Self {
        Self {
            include_class_directives,
            include_class_base,
            include_style_base,
            is_input: false,
            skip_directives: true,
        }
    }
}
