use oxc_ast::ast::{Expression, Statement};
use svelte_analyze::ComponentPropKind;
use svelte_ast::{NodeId, SVELTE_COMPONENT};
use svelte_ast_builder::{Arg, ObjProp};

use super::super::{Codegen, Result};

pub(in super::super) enum PropOrSpread<'a> {
    Prop(ObjProp<'a>),
    Spread(Expression<'a>),
}

pub(in super::super) struct EventRaw {
    pub name: String,
    pub attr_id: NodeId,
    pub has_expression: bool,
    pub has_once_modifier: bool,
}

pub(in super::super) struct ComponentPropsOutput<'a> {
    pub items: Vec<PropOrSpread<'a>>,
    pub bind_this: Option<NodeId>,
    pub events: Vec<EventRaw>,
    pub svelte_component_this: Option<Expression<'a>>,
    pub memo_decls: Vec<Statement<'a>>,
}

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(in super::super) fn build_component_props(
        &mut self,
        el_id: NodeId,
        cn_name: &str,
    ) -> Result<ComponentPropsOutput<'a>> {
        let prop_infos: Vec<(ComponentPropKind, bool)> = self
            .ctx
            .component_props(el_id)
            .iter()
            .map(|p| (p.kind.clone(), p.is_dynamic))
            .collect();

        let mut out = ComponentPropsOutput {
            items: Vec::new(),
            bind_this: None,
            events: Vec::new(),
            svelte_component_this: None,
            memo_decls: Vec::new(),
        };
        let mut memo_counter: u32 = 0;

        for (kind, is_dynamic) in prop_infos {
            match kind {
                ComponentPropKind::String { name, value_span } => {
                    self.emit_component_prop_string(&name, value_span, &mut out.items);
                }
                ComponentPropKind::Boolean { name } => {
                    self.emit_component_prop_boolean(&name, &mut out.items);
                }
                ComponentPropKind::Expression {
                    name,
                    attr_id,
                    shorthand,
                    needs_memo,
                } => {
                    if cn_name == SVELTE_COMPONENT && name == "this" {
                        out.svelte_component_this = Some(self.take_attr_expr(attr_id)?);
                        continue;
                    }
                    self.emit_component_prop_expression(
                        &name,
                        attr_id,
                        shorthand,
                        needs_memo,
                        is_dynamic,
                        &mut out.items,
                        &mut out.memo_decls,
                        &mut memo_counter,
                    )?;
                }
                ComponentPropKind::Spread { attr_id } => {
                    self.emit_component_prop_spread(attr_id, is_dynamic, &mut out.items)?;
                }
                ComponentPropKind::Concatenation {
                    name,
                    attr_id,
                    parts,
                } => {
                    self.emit_component_prop_concat(
                        &name,
                        attr_id,
                        &parts,
                        is_dynamic,
                        &mut out.items,
                    )?;
                }
                ComponentPropKind::Bind {
                    name,
                    mode,
                    expr_name,
                    ..
                } => {
                    self.emit_component_prop_bind(el_id, &name, mode, expr_name, &mut out.items)?;
                }
                ComponentPropKind::BindThis { bind_id } => {
                    out.bind_this = Some(bind_id);
                }
                ComponentPropKind::Attach { attr_id } => {
                    self.emit_component_prop_attach(attr_id, is_dynamic, &mut out.items)?;
                }
                ComponentPropKind::Event {
                    name,
                    attr_id,
                    has_expression,
                    has_once_modifier,
                } => {
                    out.events.push(EventRaw {
                        name,
                        attr_id,
                        has_expression,
                        has_once_modifier,
                    });
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
}
