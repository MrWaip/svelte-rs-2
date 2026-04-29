use svelte_ast::{
    Attribute, AwaitBlock, ComponentNode, ConstTag, NodeId, SVELTE_COMPONENT, SlotElementLegacy,
};
use svelte_span::Span;

use crate::scope::{ComponentScoping, SymbolId};
use crate::types::data::{
    AnalysisData, BindingSemantics, ExpressionInfo, ExpressionKind, ParentKind, PropBindingKind,
    PropBindingSemantics, ReactivitySemantics,
};
use crate::types::node_table::NodeBitSet;
use crate::walker::{TemplateVisitor, VisitContext};

#[derive(Debug)]
pub struct DynamismData {
    dynamic_nodes: NodeBitSet,
    dynamic_attrs: NodeBitSet,
    dynamic_components: NodeBitSet,
    has_state_attrs: NodeBitSet,
}

impl DynamismData {
    pub fn new(node_count: u32) -> Self {
        Self {
            dynamic_nodes: NodeBitSet::new(node_count),
            dynamic_attrs: NodeBitSet::new(node_count),
            dynamic_components: NodeBitSet::new(node_count),
            has_state_attrs: NodeBitSet::new(node_count),
        }
    }

    pub fn is_dynamic_node(&self, id: NodeId) -> bool {
        self.dynamic_nodes.contains(&id)
    }

    pub fn is_dynamic_attr(&self, id: NodeId) -> bool {
        self.dynamic_attrs.contains(&id)
    }

    pub fn is_dynamic_component(&self, id: NodeId) -> bool {
        self.dynamic_components.contains(&id)
    }

    pub fn has_state_attr(&self, id: NodeId) -> bool {
        self.has_state_attrs.contains(&id)
    }

    pub(crate) fn mark_dynamic_node(&mut self, id: NodeId) {
        self.dynamic_nodes.insert(id);
    }

    pub(crate) fn mark_dynamic_attr(&mut self, id: NodeId) {
        self.dynamic_attrs.insert(id);
    }

    pub(crate) fn mark_dynamic_component(&mut self, id: NodeId) {
        self.dynamic_components.insert(id);
    }

    pub(crate) fn mark_has_state_attr(&mut self, id: NodeId) {
        self.has_state_attrs.insert(id);
    }
}

pub(crate) struct DynamismVisitor;

impl DynamismVisitor {
    pub(crate) fn new() -> Self {
        Self
    }
}

impl TemplateVisitor for DynamismVisitor {
    fn visit_await_block(&mut self, block: &AwaitBlock, ctx: &mut VisitContext<'_, '_>) {
        ctx.data.dynamism.mark_dynamic_node(block.id);
    }

    fn visit_const_tag(&mut self, tag: &ConstTag, ctx: &mut VisitContext<'_, '_>) {
        let Some(info) = ctx.data.expressions.get(tag.id) else {
            return;
        };
        if classify_template_info(info, ctx.data) {
            ctx.data.dynamism.mark_dynamic_node(tag.id);
        }
    }

    fn visit_slot_element_legacy(
        &mut self,
        el: &SlotElementLegacy,
        ctx: &mut VisitContext<'_, '_>,
    ) {
        ctx.data.dynamism.mark_dynamic_node(el.id);
    }

    fn visit_attribute(&mut self, attr: &Attribute, ctx: &mut VisitContext<'_, '_>) {
        if ParentKind::from_attr(attr).is_some_and(|k| k.needs_element_ref())
            && let Some(el_id) = ctx.data.nearest_element(attr.id())
        {
            ctx.data.elements.flags.needs_ref.insert(el_id);
        }
    }

    fn visit_component_node(&mut self, cn: &ComponentNode, ctx: &mut VisitContext<'_, '_>) {
        let data = &*ctx.data;
        let uses_runes = data.uses_runes();
        let base_name = cn.name.split('.').next().unwrap_or(cn.name.as_str());
        if let Some(sym_id) = data.scoping.find_binding(ctx.scope, base_name)
            && uses_runes
            && is_reactive_component_binding(data, sym_id)
        {
            ctx.data.dynamism.mark_dynamic_component(cn.id);
        }
        if uses_runes && cn.name.contains('.') {
            ctx.data.dynamism.mark_dynamic_component(cn.id);
        }
        if cn.name == SVELTE_COMPONENT {
            ctx.data.dynamism.mark_dynamic_component(cn.id);
        }
    }

    fn visit_expression(&mut self, node_id: NodeId, _span: Span, ctx: &mut VisitContext<'_, '_>) {
        let parent = ctx.data.expr_parent(node_id);
        let parent_kind = parent.map(|p| p.kind);

        if parent_kind.is_some_and(|k| k.is_attr()) {
            let attr_id = parent.map_or(node_id, |p| p.id);
            let in_component = ctx.data.expr_ancestors(node_id).nth(1).is_some_and(|gp| {
                matches!(
                    gp.kind,
                    ParentKind::ComponentNode | ParentKind::SvelteBoundary
                )
            });

            let Some(info) = ctx.data.attr_expressions.get(node_id) else {
                return;
            };

            let scoping = &ctx.data.scoping;
            let reactivity = &ctx.data.reactivity;
            let is_dyn_element = is_dynamic_element_attr(info, scoping, reactivity);
            let has_state_component = has_state_component_attr(info, scoping, reactivity);

            let classified_dynamic = if in_component {
                has_state_component
            } else {
                is_dyn_element
            };

            if classified_dynamic {
                ctx.data.dynamism.mark_dynamic_attr(attr_id);
                if !in_component && let Some(el_id) = ctx.data.nearest_element_for_expr(node_id) {
                    ctx.data.elements.flags.needs_ref.insert(el_id);
                    if matches!(parent_kind, Some(ParentKind::ClassDirective)) {
                        ctx.data
                            .elements
                            .flags
                            .has_dynamic_class_directives
                            .insert(el_id);
                    }
                }
            }

            if has_state_component {
                ctx.data.dynamism.mark_has_state_attr(attr_id);
            }
        } else if !(matches!(parent_kind, Some(ParentKind::SvelteElement))
            && parent.is_some_and(|p| p.id == node_id))
        {
            let Some(info) = ctx.data.expressions.get(node_id) else {
                return;
            };
            if classify_template_info(info, ctx.data) {
                ctx.data.dynamism.mark_dynamic_node(node_id);
            }
        }
    }
}

fn classify_template_info(info: &ExpressionInfo, data: &AnalysisData<'_>) -> bool {
    is_dynamic_template(
        info,
        &data.scoping,
        &data.reactivity,
        data.script.has_class_state_fields,
    )
}

pub(crate) fn is_symbol_dynamic(
    scoping: &ComponentScoping,
    reactivity: &ReactivitySemantics,
    sym_id: SymbolId,
) -> bool {
    use crate::types::data::ConstBindingSemantics;
    if scoping.is_each_index_non_dynamic(sym_id) {
        return false;
    }
    let decl = reactivity.binding_semantics(sym_id);
    match decl {
        BindingSemantics::State(_)
        | BindingSemantics::Prop(_)
        | BindingSemantics::LegacyBindableProp(_)
        | BindingSemantics::LegacyState(_)
        | BindingSemantics::Store(_)
        | BindingSemantics::Contextual(_)
        | BindingSemantics::RuntimeRune { .. } => true,
        BindingSemantics::Derived(d) => d.reactive,
        BindingSemantics::Const(ConstBindingSemantics::ConstTag { reactive, .. }) => reactive,
        BindingSemantics::OptimizedRune(opt) if opt.proxy_init => true,
        BindingSemantics::NonReactive
        | BindingSemantics::Unresolved
        | BindingSemantics::OptimizedRune(_) => !scoping.is_component_top_level_symbol(sym_id),
    }
}

fn is_dynamic_template(
    info: &ExpressionInfo,
    scoping: &ComponentScoping,
    reactivity: &ReactivitySemantics,
    has_class_state_fields: bool,
) -> bool {
    if info.has_await() || info.has_state_rune() || info.needs_context() {
        return true;
    }

    if matches!(info.kind(), ExpressionKind::CallExpression { .. }) {
        return info.has_store_ref()
            || info.ref_symbols().iter().any(|&sym_id| {
                is_symbol_dynamic(scoping, reactivity, sym_id)
                    || (scoping.is_component_top_level_symbol(sym_id) && !scoping.is_import(sym_id))
            });
    }

    if matches!(info.kind(), ExpressionKind::MemberExpression) {
        return info.has_store_ref() || !info.ref_symbols().is_empty();
    }

    if info.has_store_ref() {
        return true;
    }
    info.ref_symbols().iter().any(|&sym_id| {
        if is_symbol_dynamic(scoping, reactivity, sym_id) {
            return true;
        }
        if is_unified_prop_source(scoping, reactivity, sym_id) {
            return true;
        }
        if has_class_state_fields
            && scoping.is_component_top_level_symbol(sym_id)
            && is_unified_plain_symbol(scoping, reactivity, sym_id)
        {
            return true;
        }
        false
    })
}

fn is_dynamic_element_attr(
    info: &ExpressionInfo,
    scoping: &ComponentScoping,
    reactivity: &ReactivitySemantics,
) -> bool {
    if info.has_await() {
        return true;
    }
    attr_symbols(info, scoping).any(|sym_id| {
        matches!(
            reactivity.binding_semantics(sym_id),
            BindingSemantics::Prop(PropBindingSemantics {
                kind: PropBindingKind::NonSource,
                ..
            })
        ) || is_symbol_dynamic(scoping, reactivity, sym_id)
    })
}

fn has_state_component_attr(
    info: &ExpressionInfo,
    scoping: &ComponentScoping,
    reactivity: &ReactivitySemantics,
) -> bool {
    if info.has_await() {
        return true;
    }
    attr_symbols(info, scoping).any(|sym_id| {
        !scoping.is_component_top_level_symbol(sym_id)
            || !is_unified_plain_symbol(scoping, reactivity, sym_id)
    })
}

fn attr_symbols<'a>(
    info: &'a ExpressionInfo,
    scoping: &'a ComponentScoping,
) -> impl Iterator<Item = SymbolId> + 'a {
    let fallback = if info.ref_symbols().is_empty() {
        info.identifier_name()
            .and_then(|name| scoping.find_binding(scoping.root_scope_id(), name))
    } else {
        None
    };
    info.ref_symbols().iter().copied().chain(fallback)
}

fn is_unified_prop_source(
    _scoping: &ComponentScoping,
    reactivity: &ReactivitySemantics,
    sym_id: SymbolId,
) -> bool {
    matches!(
        reactivity.binding_semantics(sym_id),
        BindingSemantics::Prop(PropBindingSemantics {
            kind: PropBindingKind::Source { .. },
            ..
        })
    )
}

fn is_unified_plain_symbol(
    _scoping: &ComponentScoping,
    reactivity: &ReactivitySemantics,
    sym_id: SymbolId,
) -> bool {
    matches!(
        reactivity.binding_semantics(sym_id),
        BindingSemantics::NonReactive | BindingSemantics::Const(_)
    )
}

fn is_reactive_component_binding(data: &AnalysisData<'_>, sym: SymbolId) -> bool {
    !matches!(
        data.binding_semantics(sym),
        BindingSemantics::NonReactive | BindingSemantics::Unresolved,
    )
}

pub(crate) fn populate_expr_roles(data: &mut AnalysisData<'_>) {
    if data.script.blocker_data.has_async {
        let blocked_node_ids: Vec<NodeId> = data
            .expressions
            .iter()
            .filter_map(|(id, info)| {
                if data.dynamism.is_dynamic_node(id) {
                    return None;
                }
                info.ref_symbols()
                    .iter()
                    .any(|sym| data.script.blocker_data.symbol_blockers.contains_key(sym))
                    .then_some(id)
            })
            .collect();
        for id in blocked_node_ids {
            data.dynamism.mark_dynamic_node(id);
        }
    }

    let dynamism = &data.dynamism;
    for (id, info) in data.expressions.iter_mut() {
        info.set_expr_role_from_dynamism(dynamism.is_dynamic_node(id));
    }
    for (id, info) in data.attr_expressions.iter_mut() {
        let is_dynamic = dynamism.is_dynamic_attr(id) || dynamism.has_state_attr(id);
        info.set_expr_role_from_dynamism(is_dynamic);
    }
}
