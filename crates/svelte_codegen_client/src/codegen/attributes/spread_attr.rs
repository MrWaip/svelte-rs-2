use oxc_ast::ast::Expression;
use svelte_ast::{Attribute, NodeId};
use svelte_ast_builder::{Arg, ObjProp};

use super::super::data_structures::EmitState;
use super::super::{Codegen, Result};

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(in super::super) fn emit_attr_spread(
        &mut self,
        state: &mut EmitState<'a>,
        owner_id: NodeId,
        owner_tag: &str,
        owner_var: &str,
        attributes: &[Attribute],
    ) -> Result<()> {
        self.emit_attr_spread_full(
            state,
            owner_id,
            owner_tag,
            owner_var,
            attributes,
            SpreadOptions::for_regular_element(),
        )?;
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

        for attr in attributes {
            let attr_id = attr.id();
            match attr {
                Attribute::BooleanAttribute(a) => {
                    let name_alloc = self.ctx.b.alloc_str(&a.name);
                    props.push(ObjProp::KeyValue(name_alloc, self.ctx.b.bool_expr(true)));
                }
                Attribute::StringAttribute(a) => {
                    let val = self
                        .ctx
                        .query
                        .component
                        .source_text(a.value_span)
                        .to_string();
                    let name_alloc = self.ctx.b.alloc_str(&a.name);
                    props.push(ObjProp::KeyValue(name_alloc, self.ctx.b.str_expr(&val)));
                }
                Attribute::ExpressionAttribute(a) => {
                    if self.ctx.is_expression_shorthand(attr_id) {
                        let name_alloc = self.ctx.b.alloc_str(&a.name);
                        props.push(ObjProp::Shorthand(name_alloc));
                        continue;
                    }

                    let expr = self.take_attr_expr(attr_id, &a.expression)?;
                    let is_event = a.event_name.is_some();
                    let is_fn = matches!(
                        &expr,
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
                    props.push(ObjProp::KeyValue(name_alloc, expr));
                }
                Attribute::ConcatenationAttribute(a) => {
                    let val = self.build_concat_expr_collapse_single(attr_id, &a.parts)?;
                    let name_alloc = self.ctx.b.alloc_str(&a.name);
                    props.push(ObjProp::KeyValue(name_alloc, val));
                }
                Attribute::SpreadAttribute(sa) => {
                    let expr = self.take_attr_expr(attr_id, &sa.expression)?;
                    props.push(ObjProp::Spread(expr));
                }
                Attribute::BindDirective(d) => {
                    self.emit_bind_directive(state, owner_id, owner_tag, owner_var, d)?;
                }
                Attribute::LetDirectiveLegacy(_)
                | Attribute::ClassDirective(_)
                | Attribute::StyleDirective(_)
                | Attribute::UseDirective(_)
                | Attribute::OnDirectiveLegacy(_)
                | Attribute::TransitionDirective(_)
                | Attribute::AnimateDirective(_)
                | Attribute::AttachTag(_) => continue,
            }
        }

        if options.include_class_directives {
            if let Some(class_obj) = self.build_class_directives_object(owner_id)? {
                let class_key_expr = self
                    .ctx
                    .b
                    .static_member_expr(self.ctx.b.rid_expr("$"), "CLASS");
                props.push(ObjProp::Computed(class_key_expr, class_obj));
            }
        }

        if self.ctx.has_style_directives(owner_id) {
            if options.include_style_base {
                let style_key = self.ctx.b.alloc_str("style");
                props.push(ObjProp::KeyValue(style_key, self.ctx.b.str_expr("")));
            }

            let style_props = self.build_style_props(owner_id)?;
            let mut merged = style_props.normal;
            merged.extend(style_props.important);

            let style_obj = self.ctx.b.object_expr(merged);
            let style_key_expr = self
                .ctx
                .b
                .static_member_expr(self.ctx.b.rid_expr("$"), "STYLE");
            props.push(ObjProp::Computed(style_key_expr, style_obj));
        }

        if !props.is_empty() {
            let obj = self.ctx.b.object_expr(props);
            let arrow = self
                .ctx
                .b
                .arrow_expr(self.ctx.b.no_params(), [self.ctx.b.expr_stmt(obj)]);
            let hash = self.ctx.css_hash().to_string();
            let args: Vec<Arg<'a, '_>> = if self.ctx.is_css_scoped(owner_id) && !hash.is_empty() {
                vec![
                    Arg::Ident(owner_var),
                    Arg::Expr(arrow),
                    Arg::Expr(self.ctx.b.void_zero_expr()),
                    Arg::Expr(self.ctx.b.void_zero_expr()),
                    Arg::Expr(self.ctx.b.void_zero_expr()),
                    Arg::Str(hash),
                ]
            } else {
                vec![Arg::Ident(owner_var), Arg::Expr(arrow)]
            };
            state
                .init
                .push(self.ctx.b.call_stmt("$.attribute_effect", args));
        }

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
                _ => {}
            }
        }

        Ok(ns_thunk)
    }
}

pub(in super::super) struct SpreadOptions {
    pub include_class_directives: bool,
    pub include_style_base: bool,
}

impl SpreadOptions {
    pub fn for_regular_element() -> Self {
        Self {
            include_class_directives: true,
            include_style_base: false,
        }
    }

    pub fn for_svelte_element() -> Self {
        Self {
            include_class_directives: false,
            include_style_base: true,
        }
    }
}
