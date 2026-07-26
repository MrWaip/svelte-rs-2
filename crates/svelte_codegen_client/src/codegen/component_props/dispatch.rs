use std::mem;

use oxc_ast::ast::{Expression, Statement};
use oxc_syntax::node::NodeId as OxcNodeId;
use svelte_analyze::{
    AttributeSemantics, ComponentAttachSemantics, ComponentBindKind, ComponentBindSemantics,
    ComponentPropMemo, ComponentPropSemantics, ComponentSpreadEmit, ComponentSpreadSemantics,
    ConcatPartEmit, EventModifier, EventSemantics,
};
use svelte_ast::{Attribute, NodeId};
use svelte_ast_builder::{Arg, ObjProp};

use super::super::async_values::AsyncValues;
use super::super::{Codegen, CodegenError, Result};

pub(in super::super) fn sync_memo_slots_of_prop(memo: ComponentPropMemo) -> u32 {
    match memo {
        ComponentPropMemo::Derived => 1,
        ComponentPropMemo::Awaited | ComponentPropMemo::Getter | ComponentPropMemo::Inline => 0,
    }
}

fn sync_memo_slots_of_part(emit: ConcatPartEmit) -> u32 {
    match emit {
        ConcatPartEmit::HoistDerived => 1,
        ConcatPartEmit::Awaited | ConcatPartEmit::Inline | ConcatPartEmit::Static => 0,
    }
}

fn sync_memo_slots_of_spread(emit: ComponentSpreadEmit) -> u32 {
    match emit {
        ComponentSpreadEmit::MemoThunk => 1,
        ComponentSpreadEmit::AwaitedThunk
        | ComponentSpreadEmit::Thunk
        | ComponentSpreadEmit::Inline => 0,
    }
}

pub(in super::super) enum PropOrSpread<'a> {
    Prop(ObjProp<'a>),
    Spread(Expression<'a>),
}

pub(in super::super) struct EventRaw {
    pub name: String,
    pub attr_id: NodeId,
    pub expr_id: Option<OxcNodeId>,
    pub has_once_modifier: bool,
}

pub(in super::super) struct ComponentPropsOutput<'a> {
    pub items: Vec<PropOrSpread<'a>>,
    pub deferred_items: Vec<PropOrSpread<'a>>,
    pub bind_this: Option<NodeId>,
    pub events: Vec<EventRaw>,
    pub svelte_component_this: Option<Expression<'a>>,
    pub memo_decls: Vec<Statement<'a>>,
    pub ownership_bindings: Vec<OwnershipBinding<'a>>,
    pub bind_init_stmts: Vec<Statement<'a>>,
    pub validate_binding_stmts: Vec<Statement<'a>>,
    pub async_values: AsyncValues<'a>,
}

pub(in super::super) struct OwnershipBinding<'a> {
    pub name: &'a str,
    pub getter: OwnershipGetter<'a>,
}

pub(in super::super) enum OwnershipGetter<'a> {
    Ident(&'a str),
    Thunk(Expression<'a>),
}

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(in super::super) fn build_component_props(
        &mut self,
        el_id: NodeId,
        initial_memo_counter: u32,
    ) -> Result<ComponentPropsOutput<'a>> {
        let mut out = ComponentPropsOutput {
            items: Vec::new(),
            deferred_items: Vec::new(),
            bind_this: None,
            events: Vec::new(),
            svelte_component_this: None,
            memo_decls: Vec::new(),
            ownership_bindings: Vec::new(),
            bind_init_stmts: Vec::new(),
            validate_binding_stmts: Vec::new(),
            async_values: AsyncValues::new(0),
        };
        let mut memo_counter: u32 = initial_memo_counter;

        let component = self.ctx.query.component;
        let attrs: &[Attribute] = match component.store.get(el_id).as_component_like() {
            Some(view) => view.attributes,
            None => {
                return CodegenError::semantic_mismatch(el_id, "component-like node expected");
            }
        };
        out.async_values = AsyncValues::new(initial_memo_counter + self.count_sync_memos(attrs));

        for attr in attrs {
            let attr_id: NodeId = attr.id();
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
                        &mut out.memo_decls,
                        &mut memo_counter,
                        &mut out.async_values,
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
                AttributeSemantics::SvelteComponentThis(s) => {
                    let expr = self
                        .ctx
                        .state
                        .parsed
                        .take_expr(s.expr_id)
                        .ok_or(CodegenError::MissingExpression(attr_id))?;
                    out.svelte_component_this = Some(expr);
                }
                AttributeSemantics::ComponentProp(ComponentPropSemantics::Expression(e)) => {
                    let Attribute::ExpressionAttribute(ea) = attr else {
                        return CodegenError::semantic_mismatch(
                            attr_id,
                            "ComponentProp::Expression requires ExpressionAttribute",
                        );
                    };
                    self.emit_component_prop_expression(
                        &ea.name,
                        ea.id,
                        ea.expression.id(),
                        e.shorthand,
                        e.memo,
                        &mut out.items,
                        &mut out.memo_decls,
                        &mut memo_counter,
                        &mut out.async_values,
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
                        &c.plan,
                        &mut out.items,
                        &mut out.memo_decls,
                        &mut memo_counter,
                        &mut out.async_values,
                    )?;
                }
                AttributeSemantics::Event(EventSemantics { modifiers, .. }) => {
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
                        has_once_modifier: modifiers.contains(EventModifier::ONCE),
                    });
                }
                AttributeSemantics::NonSpecial => match attr {
                    Attribute::StringAttribute(a) => {
                        let value = a.value(&self.ctx.query.component.source);
                        self.emit_component_prop_string(&a.name, value, &mut out.items);
                    }
                    Attribute::BooleanAttribute(a) => {
                        self.emit_component_prop_boolean(&a.name, &mut out.items);
                    }
                    _ => {
                        return CodegenError::semantic_mismatch(
                            attr_id,
                            "unsupported NonSpecial attribute on ComponentNode",
                        );
                    }
                },
                AttributeSemantics::ComponentCssProp(_) => continue,
                AttributeSemantics::Skip(_) => continue,
                _ => {
                    return CodegenError::semantic_mismatch(
                        attr_id,
                        "non-component semantics on ComponentNode",
                    );
                }
            }
        }
        out.items.append(&mut out.deferred_items);
        Ok(out)
    }

    fn count_sync_memos(&self, attrs: &[Attribute]) -> u32 {
        attrs
            .iter()
            .map(
                |attr| match self.ctx.query.analysis.attributes.get(attr.id()) {
                    AttributeSemantics::ComponentProp(ComponentPropSemantics::Expression(e)) => {
                        sync_memo_slots_of_prop(e.memo)
                    }
                    AttributeSemantics::ComponentProp(ComponentPropSemantics::Concat(c)) => {
                        c.plan.iter().copied().map(sync_memo_slots_of_part).sum()
                    }
                    AttributeSemantics::ComponentSpread(ComponentSpreadSemantics { emit }) => {
                        sync_memo_slots_of_spread(*emit)
                    }
                    AttributeSemantics::ComponentBind(_)
                    | AttributeSemantics::ComponentAttach(_)
                    | AttributeSemantics::SvelteComponentThis(_)
                    | AttributeSemantics::Event(_)
                    | AttributeSemantics::ComponentCssProp(_)
                    | AttributeSemantics::Skip(_)
                    | AttributeSemantics::NonSpecial
                    | AttributeSemantics::ElementBind(_)
                    | AttributeSemantics::WindowBind(_)
                    | AttributeSemantics::DocumentBind(_)
                    | AttributeSemantics::BoundaryProp(_)
                    | AttributeSemantics::HtmlConcat(_)
                    | AttributeSemantics::CannotBeStatic(_)
                    | AttributeSemantics::StaticAttr
                    | AttributeSemantics::SpecialValueAttr(_)
                    | AttributeSemantics::Class(_)
                    | AttributeSemantics::Style(_)
                    | AttributeSemantics::Autofocus
                    | AttributeSemantics::RuntimeBehavior => 0,
                },
            )
            .sum()
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
                            self.ctx.b.object_expr(mem::take(&mut current_props)),
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
            ComponentBindKind::Expression | ComponentBindKind::StoreMemberMutation { .. } => {
                let Some(expr) = self.take_expr_by_ref(&d.expression) else {
                    return CodegenError::missing_expression(d.id);
                };
                self.emit_bind_member_expr(
                    d,
                    bind,
                    expr,
                    &mut out.deferred_items,
                    &mut out.validate_binding_stmts,
                    &mut out.ownership_bindings,
                )
            }
            ComponentBindKind::FunctionPair => {
                let Some(expr) = self.take_expr_by_ref(&d.expression) else {
                    return CodegenError::missing_expression(d.id);
                };
                self.emit_bind_function_pair(
                    d.id,
                    &d.name,
                    expr,
                    &mut out.items,
                    &mut out.bind_init_stmts,
                )
            }
            ComponentBindKind::Identifier { symbol, target } => {
                let symbol_name = self.ctx.query.view.symbol_name(*symbol).to_string();
                self.emit_bind_identifier(
                    el_id,
                    &d.name,
                    &symbol_name,
                    *target,
                    &mut out.deferred_items,
                    &mut out.ownership_bindings,
                );
                Ok(())
            }
            ComponentBindKind::StoreSubscribed { base_symbol } => {
                self.emit_bind_store(&d.name, *base_symbol, &mut out.deferred_items);
                Ok(())
            }
        }
    }
}
