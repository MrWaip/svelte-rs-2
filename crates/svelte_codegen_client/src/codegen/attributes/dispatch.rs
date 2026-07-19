use std::mem;

use oxc_ast::ast::{Expression, Statement};
use svelte_analyze::{AttributeSemantics, DefaultAttrKind, Volatility};
use svelte_ast::{Attribute, ExpressionAttribute, NodeId};
use svelte_ast_builder::{Arg, AssignLeft};

use super::super::data_structures::EmitState;
use super::super::expr::coarse_wrap;
use super::super::{Codegen, CodegenError, Result};
use super::spread_attr::SpreadOptions;

pub(in super::super) enum AttributeOwnerKind {
    Regular,
    SvelteElement,
}

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(in super::super) fn emit_dom_attributes(
        &mut self,
        state: &mut EmitState<'a>,
        owner_id: NodeId,
        owner_tag: &str,
        owner_var: &str,
        attributes: &[Attribute],
        is_html: bool,
    ) -> Result<()> {
        self.emit_dom_attributes_with_kind(
            state,
            owner_id,
            owner_tag,
            owner_var,
            attributes,
            AttributeOwnerKind::Regular,
            is_html,
        )?;
        Ok(())
    }

    pub(in super::super) fn emit_dom_attributes_with_kind(
        &mut self,
        state: &mut EmitState<'a>,
        owner_id: NodeId,
        owner_tag: &str,
        owner_var: &str,
        attributes: &[Attribute],
        kind: AttributeOwnerKind,
        is_html: bool,
    ) -> Result<Option<Expression<'a>>> {
        if matches!(kind, AttributeOwnerKind::SvelteElement) {
            let has_spread = self.ctx.has_spread(owner_id);
            let has_class_directives = self.ctx.has_class_directives(owner_id);
            let has_style_directives = self.ctx.has_style_directives(owner_id);
            let triggers_effect = |a: &Attribute| -> bool {
                match a {
                    Attribute::SpreadAttribute(_) => true,
                    Attribute::BooleanAttribute(_) => true,
                    Attribute::StringAttribute(s) => !matches!(
                        self.ctx.query.analysis.attributes.get(s.id),
                        AttributeSemantics::Class(_)
                    ),
                    Attribute::ExpressionAttribute(_) => true,
                    Attribute::ConcatenationAttribute(_) => true,
                    _ => false,
                }
            };
            let has_effect_payload =
                has_spread || has_style_directives || attributes.iter().any(triggers_effect);
            let fold_class_directives = has_effect_payload && has_class_directives;

            for attr in attributes {
                match attr {
                    Attribute::BindDirective(d) => {
                        self.emit_bind_directive(state, owner_id, owner_tag, owner_var, d)?;
                    }
                    Attribute::UseDirective(d) => {
                        self.emit_use_directive(state, owner_id, owner_var, d)?;
                    }
                    Attribute::TransitionDirective(d) => {
                        self.emit_transition_directive(state, owner_id, owner_var, d)?;
                    }
                    Attribute::AnimateDirective(d) => {
                        self.emit_animate_directive(state, owner_id, owner_var, d)?;
                    }
                    Attribute::OnDirectiveLegacy(d) => {
                        self.emit_on_directive_legacy(state, owner_id, owner_var, d)?;
                    }
                    Attribute::AttachTag(a) => {
                        self.emit_attach_tag(state, owner_id, owner_var, a)?;
                    }
                    _ => {}
                }
            }
            let pending = mem::take(&mut state.pending_element_init);
            state.init.extend(pending);

            let ns_thunk = if has_effect_payload {
                self.emit_attr_spread_full(
                    state,
                    owner_id,
                    owner_tag,
                    owner_var,
                    attributes,
                    SpreadOptions::for_svelte_element(fold_class_directives),
                )?
            } else {
                None
            };
            if !fold_class_directives {
                self.emit_svelte_element_class_directives(state, owner_id, owner_var)?;
            }
            let animates: Vec<_> = mem::take(&mut state.after_update);
            for stmt in animates {
                state.init.push(stmt);
            }
            return Ok(ns_thunk);
        }

        if self.ctx.has_spread(owner_id) {
            let saved_after_update = mem::take(&mut state.after_update);
            self.emit_attr_spread(state, owner_id, owner_tag, owner_var, attributes)?;
            let scoped = mem::replace(&mut state.after_update, saved_after_update);
            state.element_after_update.extend(scoped);
            return Ok(None);
        }

        let has_class_directives = self.ctx.has_class_directives(owner_id);
        let has_class_attribute = self.ctx.has_class_attribute(owner_id);
        let is_scoped = self.ctx.is_css_scoped(owner_id);
        let css_hash = self.ctx.css_hash().to_string();

        let mut deferred_bind_group_value: Option<&ExpressionAttribute> = None;

        let saved_after_update = mem::take(&mut state.after_update);
        let mut event_stmts: Vec<Statement<'a>> = Vec::new();

        for attr in attributes {
            let attr_id = attr.id();
            match self.ctx.query.analysis.attributes.get(attr_id) {
                AttributeSemantics::ElementBind(_) => {
                    if !matches!(attr, Attribute::BindDirective(_)) {
                        return CodegenError::semantic_mismatch(
                            attr_id,
                            "ElementBind requires BindDirective",
                        );
                    }
                }
                AttributeSemantics::Event(_) => match attr {
                    Attribute::ExpressionAttribute(a) => {
                        let before = state.after_update.len();
                        self.emit_attr_expression(state, owner_id, owner_tag, owner_var, a)?;
                        event_stmts.extend(state.after_update.drain(before..));
                    }
                    Attribute::ConcatenationAttribute(a) => {
                        let before = state.after_update.len();
                        self.emit_concat_event(state, owner_var, a)?;
                        event_stmts.extend(state.after_update.drain(before..));
                    }
                    Attribute::OnDirectiveLegacy(_) => {}
                    _ => {
                        return CodegenError::semantic_mismatch(
                            attr_id,
                            "Event on element requires ExpressionAttribute or OnDirectiveLegacy",
                        );
                    }
                },
                AttributeSemantics::WindowBind(_)
                | AttributeSemantics::DocumentBind(_)
                | AttributeSemantics::ComponentBind(_)
                | AttributeSemantics::ComponentProp(_)
                | AttributeSemantics::ComponentCssProp(_)
                | AttributeSemantics::SvelteComponentThis(_)
                | AttributeSemantics::ComponentSpread(_)
                | AttributeSemantics::ComponentAttach(_)
                | AttributeSemantics::BoundaryProp(_) => {
                    return CodegenError::semantic_mismatch(
                        attr_id,
                        "non-element semantics on HTML element",
                    );
                }
                AttributeSemantics::HtmlConcat(_) => {
                    let Attribute::ConcatenationAttribute(a) = attr else {
                        return CodegenError::semantic_mismatch(
                            attr_id,
                            "HtmlConcat requires ConcatenationAttribute",
                        );
                    };
                    self.emit_attr_concatenation(state, owner_id, owner_tag, owner_var, a)?;
                }
                AttributeSemantics::SpecialValueAttr(s) => match attr {
                    Attribute::ExpressionAttribute(a) => {
                        if matches!(s.kind, svelte_analyze::SpecialValueKind::InputBindGroup) {
                            deferred_bind_group_value = Some(a);
                            continue;
                        }
                        self.emit_attr_expression(state, owner_id, owner_tag, owner_var, a)?;
                    }
                    Attribute::ConcatenationAttribute(a) => {
                        self.emit_attr_concatenation(state, owner_id, owner_tag, owner_var, a)?;
                    }
                    Attribute::StringAttribute(a) => {
                        let text = a.value(&self.ctx.query.component.source).to_string();
                        let val = self.ctx.b.str_expr(&text);
                        self.emit_special_value_static(state, owner_var, val, !s.defined);
                    }
                    Attribute::BooleanAttribute(_) => {
                        let val = self.ctx.b.bool_expr(true);
                        self.emit_special_value_static(state, owner_var, val, !s.defined);
                    }
                    _ => {
                        return CodegenError::semantic_mismatch(
                            attr_id,
                            "SpecialValueAttr requires a value attribute",
                        );
                    }
                },
                AttributeSemantics::StaticAttr => {
                    if let Attribute::StringAttribute(a) = attr {
                        let val = a.value(&self.ctx.query.component.source).to_string();
                        state.template.set_attribute(&a.name, Some(val));
                    }
                }
                AttributeSemantics::CannotBeStatic(sem) => {
                    if let Attribute::ExpressionAttribute(a) = attr {
                        self.emit_attr_expression(state, owner_id, owner_tag, owner_var, a)?;
                    } else {
                        let property = attr.name().unwrap_or_default().to_string();
                        let value_expr = match attr {
                            Attribute::BooleanAttribute(_) => self.ctx.b.bool_expr(true),
                            Attribute::StringAttribute(a) => {
                                let text = a.value(&self.ctx.query.component.source).to_string();
                                self.ctx.b.str_expr(&text)
                            }
                            _ => {
                                return CodegenError::semantic_mismatch(
                                    attr_id,
                                    "CannotBeStatic requires boolean, string, or expression attribute",
                                );
                            }
                        };
                        let setter_fn = match sem.kind {
                            DefaultAttrKind::ReconcileValue => Some("$.set_default_value"),
                            DefaultAttrKind::ReconcileChecked => Some("$.set_default_checked"),
                            DefaultAttrKind::PlainProperty => None,
                        };
                        let b = &self.ctx.b;
                        match setter_fn {
                            Some(setter_fn) => {
                                state.init.push(b.call_stmt(
                                    setter_fn,
                                    [Arg::Ident(owner_var), Arg::Expr(value_expr)],
                                ));
                            }
                            None => {
                                let target = AssignLeft::StaticMember(
                                    b.static_member(b.rid_expr(owner_var), &property),
                                );
                                state.init.push(b.assign_stmt(target, value_expr));
                            }
                        }
                    }
                }
                AttributeSemantics::Autofocus => {
                    let value = match attr {
                        Attribute::BooleanAttribute(_) => self.ctx.b.bool_expr(true),
                        Attribute::StringAttribute(a) => {
                            let text = a.value(&self.ctx.query.component.source).to_string();
                            self.ctx.b.str_expr(&text)
                        }
                        Attribute::ExpressionAttribute(a) => {
                            let expr = self.take_attr_expr(a.id, &a.expression)?;
                            let data = self.ctx.expression_data(a.id).cloned();
                            coarse_wrap(self.ctx, expr, data.as_ref())
                        }
                        _ => {
                            return CodegenError::semantic_mismatch(
                                attr_id,
                                "Autofocus requires boolean, string, or expression attribute",
                            );
                        }
                    };
                    state.init.push(
                        self.ctx
                            .b
                            .call_stmt("$.autofocus", [Arg::Ident(owner_var), Arg::Expr(value)]),
                    );
                }
                AttributeSemantics::Class(_) => {
                    if !self.ctx.class_is_directives_only(owner_id) {
                        if has_class_directives || has_class_attribute {
                            self.emit_class_attribute_and_directives(
                                state, owner_id, owner_var, is_html,
                            )?;
                        } else if let Some(base) =
                            self.ctx.static_class(owner_id).map(str::to_string)
                        {
                            if self.ctx.query.view.is_custom_element(owner_id) {
                                self.emit_custom_element_static_class(
                                    state, owner_id, owner_var, &base, is_html,
                                );
                            } else {
                                let full = if is_scoped && !css_hash.is_empty() {
                                    if base.is_empty() {
                                        css_hash.clone()
                                    } else {
                                        format!("{base} {css_hash}")
                                    }
                                } else {
                                    base
                                };
                                if !full.is_empty() {
                                    state.template.set_attribute("class", Some(full));
                                }
                            }
                        }
                    }
                }
                AttributeSemantics::Style(_) => {
                    if !self.ctx.style_is_directives_only(owner_id) {
                        self.emit_style_directives_aggregate(
                            state, owner_id, owner_tag, owner_var, None,
                        )?;
                    }
                }
                AttributeSemantics::Skip(_) => {}
                AttributeSemantics::RuntimeBehavior => {}
                AttributeSemantics::NonSpecial
                    if self.ctx.query.view.is_custom_element(owner_id) =>
                {
                    match attr {
                        Attribute::StringAttribute(a) => {
                            let val = a.value(&self.ctx.query.component.source).to_string();
                            let value = self.ctx.b.str_expr(&val);
                            self.emit_custom_element_data(state, owner_var, &a.name, value, false);
                        }
                        Attribute::BooleanAttribute(a) => {
                            let value = self.ctx.b.bool_expr(true);
                            self.emit_custom_element_data(state, owner_var, &a.name, value, false);
                        }
                        Attribute::ExpressionAttribute(a) => {
                            let attr_id = a.id;
                            let value = self.take_attr_expr(attr_id, &a.expression)?;
                            let reactive =
                                match self.ctx.expression_data(attr_id).map(|d| d.volatility) {
                                    Some(
                                        Volatility::Reactive
                                        | Volatility::Heavy
                                        | Volatility::Asynchronous,
                                    ) => true,
                                    Some(Volatility::Static) | None => false,
                                };
                            self.emit_custom_element_data(
                                state, owner_var, &a.name, value, reactive,
                            );
                        }
                        Attribute::ConcatenationAttribute(_) => {
                            return CodegenError::semantic_mismatch(
                                attr_id,
                                "ConcatenationAttribute must classify as HtmlConcat",
                            );
                        }
                        Attribute::SpreadAttribute(_)
                        | Attribute::ClassDirective(_)
                        | Attribute::StyleDirective(_)
                        | Attribute::LetDirectiveLegacy(_) => continue,
                        Attribute::UseDirective(_)
                        | Attribute::OnDirectiveLegacy(_)
                        | Attribute::TransitionDirective(_)
                        | Attribute::AnimateDirective(_) => {}
                        Attribute::AttachTag(a) => {
                            self.emit_attach_tag(state, owner_id, owner_var, a)?;
                        }
                        Attribute::BindDirective(_) => {
                            return CodegenError::semantic_mismatch(
                                attr_id,
                                "BindDirective in NonSpecial branch",
                            );
                        }
                    }
                }
                AttributeSemantics::NonSpecial => match attr {
                    Attribute::StringAttribute(a) => {
                        let val = a.value(&self.ctx.query.component.source);
                        state.template.set_attribute(&a.name, Some(val.to_string()));
                    }
                    Attribute::BooleanAttribute(a) => {
                        state.template.set_attribute(&a.name, Some(String::new()));
                    }
                    Attribute::ExpressionAttribute(a) => {
                        self.emit_attr_expression(state, owner_id, owner_tag, owner_var, a)?;
                    }
                    Attribute::ConcatenationAttribute(_) => {
                        return CodegenError::semantic_mismatch(
                            attr_id,
                            "ConcatenationAttribute must classify as HtmlConcat",
                        );
                    }
                    Attribute::SpreadAttribute(_)
                    | Attribute::ClassDirective(_)
                    | Attribute::StyleDirective(_) => continue,
                    Attribute::LetDirectiveLegacy(_) => continue,
                    Attribute::UseDirective(_)
                    | Attribute::OnDirectiveLegacy(_)
                    | Attribute::TransitionDirective(_)
                    | Attribute::AnimateDirective(_) => {}
                    Attribute::AttachTag(a) => {
                        self.emit_attach_tag(state, owner_id, owner_var, a)?;
                    }
                    Attribute::BindDirective(_) => {
                        return CodegenError::semantic_mismatch(
                            attr_id,
                            "BindDirective in NonSpecial branch",
                        );
                    }
                },
            }
        }

        if self.ctx.class_is_directives_only(owner_id) {
            self.emit_class_attribute_and_directives(state, owner_id, owner_var, is_html)?;
        }

        if let Some(a) = deferred_bind_group_value {
            self.emit_attr_expression(state, owner_id, owner_tag, owner_var, a)?;
        }

        if is_scoped && self.ctx.class_semantics(owner_id).is_none() {
            if self.ctx.query.view.is_custom_element(owner_id) {
                if !css_hash.is_empty() {
                    let call = self.ctx.b.call_expr(
                        "$.set_class",
                        [
                            Arg::Ident(owner_var),
                            Arg::Num(if is_html { 1.0 } else { 0.0 }),
                            Arg::Str(css_hash),
                        ],
                    );
                    state.init.push(self.ctx.b.expr_stmt(call));
                }
            } else {
                state.template.set_attribute("class", Some(css_hash));
            }
        }

        if self.ctx.style_is_directives_only(owner_id) {
            self.emit_style_directives_aggregate(state, owner_id, owner_tag, owner_var, None)?;
        }

        let scoped = mem::replace(&mut state.after_update, saved_after_update);
        state.after_update.extend(event_stmts);
        state.element_after_update.extend(scoped);

        Ok(None)
    }
}
