use oxc_ast::ast::Expression;
use svelte_ast::{Attribute, NodeId};

use super::super::data_structures::EmitState;
use super::super::{Codegen, Result};
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
            let ns_thunk = if attributes.is_empty() {
                None
            } else {
                self.emit_attr_spread_full(
                    state,
                    owner_id,
                    owner_tag,
                    owner_var,
                    attributes,
                    SpreadOptions::for_svelte_element(),
                )?
            };
            self.emit_svelte_element_class_directives(state, owner_id, owner_var)?;
            let animates: Vec<_> = std::mem::take(&mut state.after_update);
            for stmt in animates {
                state.init.push(stmt);
            }
            return Ok(ns_thunk);
        }

        if self.ctx.has_spread(owner_id) {
            self.emit_attr_spread(state, owner_id, owner_tag, owner_var, attributes)?;
            return Ok(None);
        }

        let has_class_directives = self.ctx.has_class_directives(owner_id);
        let has_class_attribute = self.ctx.has_class_attribute(owner_id);
        let is_scoped = self.ctx.is_css_scoped(owner_id);
        let css_hash = self.ctx.css_hash().to_string();

        let mut emitted_class = false;
        let mut wrote_class_attr = false;

        for attr in attributes {
            match attr {
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
                        let val = self.ctx.query.component.source_text(a.value_span);
                        let full = if is_scoped {
                            format!("{val} {css_hash}")
                        } else {
                            val.to_string()
                        };
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
                            let val = self.ctx.query.component.source_text(a.value_span);
                            self.emit_bind_group_static_value(state, owner_var, val);
                        }
                        continue;
                    }
                    let val = self.ctx.query.component.source_text(a.value_span);
                    state.template.set_attribute(&a.name, Some(val.to_string()));
                }
                Attribute::BooleanAttribute(a) => {
                    state.template.set_attribute(&a.name, Some(String::new()));
                }
                Attribute::ExpressionAttribute(a) => {
                    if self.ctx.event_handler_mode(a.id).is_some() {
                        self.emit_attr_expression(state, owner_id, owner_tag, owner_var, a)?;
                        continue;
                    }
                    if a.name == "class" && (has_class_directives || has_class_attribute) {
                        if !emitted_class {
                            self.emit_class_attribute_and_directives(state, owner_id, owner_var)?;
                            emitted_class = true;
                        }
                        continue;
                    }
                    self.emit_attr_expression(state, owner_id, owner_tag, owner_var, a)?;
                }
                Attribute::ConcatenationAttribute(a) => {
                    if a.name == "class" && (has_class_directives || has_class_attribute) {
                        if !emitted_class {
                            self.emit_class_attribute_and_directives(state, owner_id, owner_var)?;
                            emitted_class = true;
                        }
                        continue;
                    }
                    self.emit_attr_concatenation(state, owner_id, owner_tag, owner_var, a)?;
                }
                Attribute::SpreadAttribute(_)
                | Attribute::ClassDirective(_)
                | Attribute::StyleDirective(_) => continue,
                Attribute::BindDirective(d) => {
                    self.emit_bind_directive(state, owner_id, owner_tag, owner_var, d)?;
                }
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
            }
        }

        if !emitted_class && (has_class_directives || has_class_attribute) {
            self.emit_class_attribute_and_directives(state, owner_id, owner_var)?;
        }

        if is_scoped && !wrote_class_attr && !has_class_directives && !has_class_attribute {
            state.template.set_attribute("class", Some(css_hash));
        }

        self.emit_style_directives_aggregate(state, owner_id, owner_var)?;

        Ok(None)
    }
}
