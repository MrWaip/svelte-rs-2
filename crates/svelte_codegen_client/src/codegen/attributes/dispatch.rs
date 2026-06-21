use std::mem;

use oxc_ast::ast::{Expression, Statement};
use svelte_analyze::{AttributeSemantics, MustBePropertyValue, Volatility};
use svelte_ast::{Attribute, ExpressionAttribute, NodeId};
use svelte_ast_builder::{Arg, AssignLeft};

use super::super::data_structures::EmitState;
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
                    Attribute::StringAttribute(s) => s.name != "class",
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
        let has_style_directives = self.ctx.has_style_directives(owner_id);
        let is_scoped = self.ctx.is_css_scoped(owner_id);
        let css_hash = self.ctx.css_hash().to_string();

        let mut emitted_class = false;
        let mut emitted_style_directives = false;
        let mut wrote_class_attr = false;
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
                    if a.name == "class" && (has_class_directives || has_class_attribute) {
                        if !emitted_class {
                            self.emit_class_attribute_and_directives(
                                state, owner_id, owner_var, is_html,
                            )?;
                            emitted_class = true;
                        }
                        continue;
                    }
                    if a.name == "style" && has_style_directives {
                        self.emit_style_directives_aggregate(
                            state,
                            owner_id,
                            owner_var,
                            Some(a.id),
                        )?;
                        emitted_style_directives = true;
                        continue;
                    }
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
                    _ => {
                        return CodegenError::semantic_mismatch(
                            attr_id,
                            "SpecialValueAttr requires ExpressionAttribute or ConcatenationAttribute",
                        );
                    }
                },
                AttributeSemantics::MustBeProperty(s) => {
                    let b = &self.ctx.b;
                    let value_expr = match &s.value {
                        MustBePropertyValue::BoolTrue => b.bool_expr(true),
                        MustBePropertyValue::Str(text) => b.str_expr(text.as_str()),
                    };
                    let target = AssignLeft::StaticMember(
                        b.static_member(b.rid_expr(owner_var), s.property.as_str()),
                    );
                    state.init.push(b.assign_stmt(target, value_expr));
                }
                AttributeSemantics::Autofocus => {
                    let value = match attr {
                        Attribute::BooleanAttribute(_) => self.ctx.b.bool_expr(true),
                        Attribute::StringAttribute(a) => {
                            let text = a.value(&self.ctx.query.component.source).to_string();
                            self.ctx.b.str_expr(&text)
                        }
                        Attribute::ExpressionAttribute(a) => {
                            self.take_attr_expr(a.id, &a.expression)?
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
                AttributeSemantics::StyleDirectives(_) => {}
                AttributeSemantics::NonSpecial
                    if self.ctx.query.view.is_custom_element(owner_id) =>
                {
                    match attr {
                        Attribute::StringAttribute(a) => {
                            if a.name == "class" {
                                let val = a.value(&self.ctx.query.component.source).to_string();
                                self.emit_custom_element_static_class(
                                    state, owner_id, owner_var, &val, is_html,
                                );
                                emitted_class = true;
                                continue;
                            }
                            if a.name == "style" {
                                let val = a.value(&self.ctx.query.component.source).to_string();
                                let style_call = self.ctx.b.call_expr(
                                    "$.set_style",
                                    [Arg::Ident(owner_var), Arg::Str(val)],
                                );
                                state.init.push(self.ctx.b.expr_stmt(style_call));
                                continue;
                            }
                            if a.name == "is" && is_html {
                                let val = a.value(&self.ctx.query.component.source);
                                state.template.set_attribute(&a.name, Some(val.to_string()));
                                continue;
                            }
                            let val = a.value(&self.ctx.query.component.source).to_string();
                            let value = self.ctx.b.str_expr(&val);
                            self.emit_custom_element_data(state, owner_var, &a.name, value, false);
                        }
                        Attribute::BooleanAttribute(a) => {
                            let value = self.ctx.b.bool_expr(true);
                            self.emit_custom_element_data(state, owner_var, &a.name, value, false);
                        }
                        Attribute::ExpressionAttribute(a) => {
                            if a.name == "class" {
                                if !emitted_class {
                                    self.emit_class_attribute_and_directives(
                                        state, owner_id, owner_var, is_html,
                                    )?;
                                    emitted_class = true;
                                }
                                continue;
                            }
                            if a.name == "style" {
                                if has_style_directives {
                                    self.emit_style_directives_aggregate(
                                        state,
                                        owner_id,
                                        owner_var,
                                        Some(a.id),
                                    )?;
                                    emitted_style_directives = true;
                                } else {
                                    self.emit_attr_expression(
                                        state, owner_id, owner_tag, owner_var, a,
                                    )?;
                                }
                                continue;
                            }
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
                        if a.name == "class" {
                            if has_class_directives || has_class_attribute {
                                if !emitted_class {
                                    self.emit_class_attribute_and_directives(
                                        state, owner_id, owner_var, is_html,
                                    )?;
                                    emitted_class = true;
                                }
                                continue;
                            }
                            let val = a.value(&self.ctx.query.component.source);
                            let full = if is_scoped {
                                format!("{val} {css_hash}")
                            } else {
                                val.to_string()
                            };
                            if full.is_empty() {
                                continue;
                            }
                            state.template.set_attribute("class", Some(full));
                            wrote_class_attr = true;
                            continue;
                        }
                        if a.name == "value"
                            && (self.ctx.has_bind_group(owner_id) || owner_tag == "option")
                        {
                            if (self.ctx.has_bind_group(owner_id) && owner_tag == "input")
                                || owner_tag == "option"
                            {
                                let val = a.value(&self.ctx.query.component.source);
                                self.emit_bind_group_static_value(state, owner_var, val);
                            }
                            continue;
                        }
                        if a.name == "style" && has_style_directives {
                            self.emit_style_directives_aggregate(state, owner_id, owner_var, None)?;
                            emitted_style_directives = true;
                            continue;
                        }
                        let val = a.value(&self.ctx.query.component.source);
                        state.template.set_attribute(&a.name, Some(val.to_string()));
                    }
                    Attribute::BooleanAttribute(a) => {
                        state.template.set_attribute(&a.name, Some(String::new()));
                    }
                    Attribute::ExpressionAttribute(a) => {
                        if a.name == "class" && (has_class_directives || has_class_attribute) {
                            if !emitted_class {
                                self.emit_class_attribute_and_directives(
                                    state, owner_id, owner_var, is_html,
                                )?;
                                emitted_class = true;
                            }
                            continue;
                        }
                        if a.name == "style" && has_style_directives {
                            self.emit_style_directives_aggregate(
                                state,
                                owner_id,
                                owner_var,
                                Some(a.id),
                            )?;
                            emitted_style_directives = true;
                            continue;
                        }
                        if a.name == "value"
                            && owner_tag == "input"
                            && self.ctx.has_bind_group(owner_id)
                        {
                            deferred_bind_group_value = Some(a);
                            continue;
                        }
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

        if !emitted_class && (has_class_directives || has_class_attribute) {
            self.emit_class_attribute_and_directives(state, owner_id, owner_var, is_html)?;
        }

        if let Some(a) = deferred_bind_group_value {
            self.emit_attr_expression(state, owner_id, owner_tag, owner_var, a)?;
        }

        if is_scoped && !wrote_class_attr && !has_class_directives && !has_class_attribute {
            state.template.set_attribute("class", Some(css_hash));
        }

        if !emitted_style_directives {
            self.emit_style_directives_aggregate(state, owner_id, owner_var, None)?;
        }

        let scoped = mem::replace(&mut state.after_update, saved_after_update);
        state.after_update.extend(event_stmts);
        state.element_after_update.extend(scoped);

        Ok(None)
    }
}
