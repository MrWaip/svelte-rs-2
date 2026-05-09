use oxc_ast::ast::{Expression, Statement};
use svelte_analyze::{
    AttributeSemantics, ComponentAttachSemantics, ComponentBindKind, ComponentBindSemantics,
    ComponentPropSemantics, ComponentSpreadSemantics, EventEmit, EventModifier, EventSemantics,
};
use svelte_ast::{Attribute, NodeId, SVELTE_COMPONENT};
use svelte_ast_builder::{Arg, ObjProp};

use super::super::{Codegen, CodegenError, Result};

pub(in super::super) enum PropOrSpread<'a> {
    Prop(ObjProp<'a>),
    Spread(Expression<'a>),
}

pub(in super::super) struct EventRaw {
    pub name: String,
    pub attr_id: NodeId,
    pub expr_id: Option<oxc_syntax::node::NodeId>,
    pub has_expression: bool,
    pub has_once_modifier: bool,
}

pub(in super::super) struct ComponentPropsOutput<'a> {
    pub items: Vec<PropOrSpread<'a>>,
    pub bind_this: Option<NodeId>,
    pub events: Vec<EventRaw>,
    pub svelte_component_this: Option<Expression<'a>>,
    pub memo_decls: Vec<Statement<'a>>,
    pub ownership_bindings: Vec<OwnershipBinding<'a>>,
}

pub(in super::super) struct OwnershipBinding<'a> {
    pub name: String,
    pub source_ident: &'a str,
}

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(in super::super) fn build_component_props(
        &mut self,
        el_id: NodeId,
        cn_name: &str,
    ) -> Result<ComponentPropsOutput<'a>> {
        let mut out = ComponentPropsOutput {
            items: Vec::new(),
            bind_this: None,
            events: Vec::new(),
            svelte_component_this: None,
            memo_decls: Vec::new(),
            ownership_bindings: Vec::new(),
        };
        let mut memo_counter: u32 = 0;

        let attrs: Vec<Attribute> = match self
            .ctx
            .query
            .component
            .store
            .get(el_id)
            .as_component_like()
        {
            Some(view) => view.attributes.to_vec(),
            None => {
                return CodegenError::semantic_mismatch(el_id, "component-like node expected");
            }
        };

        for attr in &attrs {
            let attr_id: NodeId = attr.id();
            if attr.name().is_some_and(|n| n.starts_with("--")) {
                continue;
            }
            match self.ctx.query.analysis.attributes.get(attr_id) {
                AttributeSemantics::ComponentBind(b) => {
                    self.dispatch_component_bind(el_id, b, attr, &mut out)?;
                }
                AttributeSemantics::ComponentSpread(ComponentSpreadSemantics { emit }) => {
                    let Attribute::SpreadAttribute(sa) = attr else {
                        return CodegenError::semantic_mismatch(
                            attr_id,
                            "ComponentSpread requires SpreadAttribute",
                        );
                    };
                    self.emit_component_prop_spread(
                        sa.id,
                        sa.expression.id(),
                        *emit,
                        &mut out.items,
                    )?;
                }
                AttributeSemantics::ComponentAttach(ComponentAttachSemantics { emit }) => {
                    let Attribute::AttachTag(at) = attr else {
                        return CodegenError::semantic_mismatch(
                            attr_id,
                            "ComponentAttach requires AttachTag",
                        );
                    };
                    self.emit_component_prop_attach(
                        at.id,
                        at.expression.id(),
                        *emit,
                        &mut out.items,
                    )?;
                }
                AttributeSemantics::ComponentProp(ComponentPropSemantics::Expression(e)) => {
                    let Attribute::ExpressionAttribute(ea) = attr else {
                        return CodegenError::semantic_mismatch(
                            attr_id,
                            "ComponentProp::Expression requires ExpressionAttribute",
                        );
                    };
                    if cn_name == SVELTE_COMPONENT && ea.name == "this" {
                        let expr = self
                            .ctx
                            .state
                            .parsed
                            .take_expr(ea.expression.id())
                            .ok_or(CodegenError::MissingExpression(ea.id))?;
                        out.svelte_component_this = Some(expr);
                        continue;
                    }
                    self.emit_component_prop_expression(
                        &ea.name,
                        ea.id,
                        ea.expression.id(),
                        e.shorthand,
                        e.memo,
                        &mut out.items,
                        &mut out.memo_decls,
                        &mut memo_counter,
                    )?;
                }
                AttributeSemantics::ComponentProp(ComponentPropSemantics::Concat(c)) => {
                    let Attribute::ConcatenationAttribute(ca) = attr else {
                        return CodegenError::semantic_mismatch(
                            attr_id,
                            "ComponentProp::Concat requires ConcatenationAttribute",
                        );
                    };
                    self.emit_component_prop_concat(
                        &ca.name,
                        ca.id,
                        &ca.parts,
                        c.memo,
                        &mut out.items,
                    )?;
                }
                AttributeSemantics::Event(EventSemantics { modifiers, emit }) => {
                    let EventEmit::Component { .. } = emit else {
                        return CodegenError::semantic_mismatch(
                            attr_id,
                            "non-Component Event variant on ComponentNode",
                        );
                    };
                    let Attribute::OnDirectiveLegacy(d) = attr else {
                        return CodegenError::semantic_mismatch(
                            attr_id,
                            "Component Event requires OnDirectiveLegacy",
                        );
                    };
                    out.events.push(EventRaw {
                        name: d.name.clone(),
                        attr_id: d.id,
                        expr_id: d.expression.as_ref().map(|r| r.id()),
                        has_expression: d.expression.is_some(),
                        has_once_modifier: modifiers.contains(EventModifier::ONCE),
                    });
                }
                AttributeSemantics::NonSpecial => match attr {
                    Attribute::StringAttribute(a) => {
                        self.emit_component_prop_string(&a.name, a.value_span, &mut out.items);
                    }
                    Attribute::BooleanAttribute(a) => {
                        self.emit_component_prop_boolean(&a.name, &mut out.items);
                    }
                    Attribute::LetDirectiveLegacy(_) => continue,
                    _ => {
                        return CodegenError::semantic_mismatch(
                            attr_id,
                            "unsupported NonSpecial attribute on ComponentNode",
                        );
                    }
                },
                _ => {
                    return CodegenError::semantic_mismatch(
                        attr_id,
                        "non-component semantics on ComponentNode",
                    );
                }
            }
        }
        Ok(out)
    }

    pub(in super::super) fn build_props_expr(
        &self,
        items: Vec<PropOrSpread<'a>>,
    ) -> Expression<'a> {
        let has_spread = items.iter().any(|i| matches!(i, PropOrSpread::Spread(_)));

        if !has_spread {
            let props: Vec<ObjProp<'a>> = items
                .into_iter()
                .filter_map(|i| match i {
                    PropOrSpread::Prop(p) => Some(p),
                    PropOrSpread::Spread(_) => None,
                })
                .collect();
            return self.ctx.b.object_expr(props);
        }

        let mut args: Vec<Arg<'a, 'a>> = Vec::new();
        let mut current_props: Vec<ObjProp<'a>> = Vec::new();

        for item in items {
            match item {
                PropOrSpread::Prop(p) => current_props.push(p),
                PropOrSpread::Spread(expr) => {
                    if !current_props.is_empty() {
                        args.push(Arg::Expr(
                            self.ctx.b.object_expr(std::mem::take(&mut current_props)),
                        ));
                    }
                    args.push(Arg::Expr(expr));
                }
            }
        }
        if !current_props.is_empty() {
            args.push(Arg::Expr(self.ctx.b.object_expr(current_props)));
        }

        self.ctx.b.call_expr("$.spread_props", args)
    }

    fn dispatch_component_bind(
        &mut self,
        el_id: NodeId,
        bind: &ComponentBindSemantics,
        attr: &Attribute,
        out: &mut ComponentPropsOutput<'a>,
    ) -> Result<()> {
        let Attribute::BindDirective(d) = attr else {
            return CodegenError::semantic_mismatch(
                attr.id(),
                "ComponentBind requires BindDirective",
            );
        };
        match &bind.kind {
            ComponentBindKind::This { .. } => {
                out.bind_this = Some(d.id);
                Ok(())
            }
            ComponentBindKind::Expression => {
                self.emit_bind_plain(&d.name, &d.name, &mut out.items);
                Ok(())
            }
            ComponentBindKind::Identifier { symbol, target } => {
                let symbol_name = self.ctx.query.view.symbol_name(*symbol).to_string();
                self.emit_bind_identifier(
                    el_id,
                    &d.name,
                    &symbol_name,
                    *target,
                    &mut out.items,
                    &mut out.ownership_bindings,
                );
                Ok(())
            }
            ComponentBindKind::StoreSubscribed { base_symbol } => {
                let base_name = self.ctx.query.view.symbol_name(*base_symbol).to_string();
                self.emit_bind_store(&d.name, &base_name, &mut out.items);
                Ok(())
            }
        }
    }
}
