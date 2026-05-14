use std::mem;

use oxc_ast::ast::{Expression, Statement};
use svelte_analyze::{AttributeSemantics, MustBePropertyValue};
use svelte_ast::{Attribute, NodeId};
use svelte_ast_builder::AssignLeft;

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
    ) -> Result<()> {
        self.emit_dom_attributes_with_kind(
            state,
            owner_id,
            owner_tag,
            owner_var,
            attributes,
            AttributeOwnerKind::Regular,
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
    ) -> Result<Option<Expression<'a>>> {
        if matches!(kind, AttributeOwnerKind::SvelteElement) {
            let has_spread = self.ctx.has_spread(owner_id);
            let has_class_directives = self.ctx.has_class_directives(owner_id);
            let has_style_directives = self.ctx.has_style_directives(owner_id);
            let has_class_attr = attributes.iter().any(|a| match a {
                Attribute::StringAttribute(s) => s.name == "class",
                Attribute::ExpressionAttribute(e) => e.name == "class",
                Attribute::ConcatenationAttribute(c) => c.name == "class",
                _ => false,
            });
            let has_style_attr = attributes.iter().any(|a| match a {
                Attribute::StringAttribute(s) => s.name == "style",
                Attribute::ExpressionAttribute(e) => e.name == "style",
                Attribute::ConcatenationAttribute(c) => c.name == "style",
                _ => false,
            });
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
            let has_effect_payload = has_spread
                || has_style_directives
                || attributes.iter().any(triggers_effect);
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

            let include_class_base = fold_class_directives && !has_spread && !has_class_attr;
            let include_style_base = has_style_directives && !has_spread && !has_style_attr;
            let ns_thunk = if has_effect_payload {
                self.emit_attr_spread_full(
                    state,
                    owner_id,
                    owner_tag,
                    owner_var,
                    attributes,
                    SpreadOptions::for_svelte_element(
                        fold_class_directives,
                        include_class_base,
                        include_style_base,
                    ),
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
        let mut wrote_class_attr = false;
        let mut pending_style_attr_id: Option<NodeId> = None;

        let saved_after_update = mem::take(&mut state.after_update);
        let mut event_stmts: Vec<Statement<'a>> = Vec::new();

        let mut on_legacy_emitted: smallvec::SmallVec<[NodeId; 4]> = smallvec::SmallVec::new();
        let class_needs_state =
            (has_class_directives || has_class_attribute) && self.ctx.class_needs_state(owner_id);
        if class_needs_state && !self.ctx.has_use_directive(owner_id) {
            for attr in attributes {
                if let Attribute::OnDirectiveLegacy(d) = attr {
                    self.emit_on_directive_legacy(state, owner_id, owner_var, d)?;
                    on_legacy_emitted.push(attr.id());
                }
            }
        }

        for attr in attributes {
            let attr_id = attr.id();
            if on_legacy_emitted.contains(&attr_id) {
                continue;
            }
            match self.ctx.query.analysis.attributes.get(attr_id) {
                AttributeSemantics::ElementBind(_) => {
                    let Attribute::BindDirective(d) = attr else {
                        return CodegenError::semantic_mismatch(
                            attr_id,
                            "ElementBind requires BindDirective",
                        );
                    };
                    self.emit_bind_directive(state, owner_id, owner_tag, owner_var, d)?;
                }
                AttributeSemantics::Event(_) => match attr {
                    Attribute::ExpressionAttribute(a) => {
                        let before = state.after_update.len();
                        self.emit_attr_expression(state, owner_id, owner_tag, owner_var, a)?;
                        event_stmts.extend(state.after_update.drain(before..));
                    }
                    Attribute::OnDirectiveLegacy(d) => {
                        self.emit_on_directive_legacy(state, owner_id, owner_var, d)?;
                    }
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
                                state, owner_id, owner_var,
                            )?;
                            emitted_class = true;
                        }
                        continue;
                    }
                    if a.name == "style" && has_style_directives {
                        pending_style_attr_id = Some(a.id);
                        continue;
                    }
                    self.emit_attr_concatenation(state, owner_id, owner_tag, owner_var, a)?;
                }
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
                AttributeSemantics::NonSpecial => match attr {
                    Attribute::StringAttribute(a) => {
                        if a.name == "class" {
                            if has_class_directives || has_class_attribute {
                                if !emitted_class {
                                    self.emit_class_attribute_and_directives(
                                        state, owner_id, owner_var,
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
                                    state, owner_id, owner_var,
                                )?;
                                emitted_class = true;
                            }
                            continue;
                        }
                        if a.name == "style" && has_style_directives {
                            pending_style_attr_id = Some(a.id);
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
            self.emit_class_attribute_and_directives(state, owner_id, owner_var)?;
        }

        if is_scoped && !wrote_class_attr && !has_class_directives && !has_class_attribute {
            state.template.set_attribute("class", Some(css_hash));
        }

        self.emit_style_directives_aggregate(state, owner_id, owner_var, pending_style_attr_id)?;

        let scoped = mem::replace(&mut state.after_update, saved_after_update);
        state.after_update.extend(event_stmts);
        state.element_after_update.extend(scoped);

        Ok(None)
    }
}
