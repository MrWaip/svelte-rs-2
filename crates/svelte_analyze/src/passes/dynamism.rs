use oxc_ast::ast::Expression;
use svelte_ast::{Attribute, AwaitBlock, ComponentNode, ConstTag, NodeId, SlotElementLegacy};

use crate::expression_semantics::Volatility;
use crate::reactivity_semantics::builder_v2::expression_root_reference_id;
use crate::scope::SymbolId;
use crate::types::data::{AnalysisData, BindingSemantics, ParentKind};
use crate::types::node_table::NodeBitSet;
use crate::walker::{TemplateVisitor, VisitContext};

#[derive(Debug)]
pub struct DynamismData {
    dynamic_nodes: NodeBitSet,
    dynamic_attrs: NodeBitSet,
    dynamic_components: NodeBitSet,
}

impl DynamismData {
    pub fn new(node_count: u32) -> Self {
        Self {
            dynamic_nodes: NodeBitSet::new(node_count),
            dynamic_attrs: NodeBitSet::new(node_count),
            dynamic_components: NodeBitSet::new(node_count),
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

    pub(crate) fn mark_dynamic_node(&mut self, id: NodeId) {
        self.dynamic_nodes.insert(id);
    }

    pub(crate) fn mark_dynamic_attr(&mut self, id: NodeId) {
        self.dynamic_attrs.insert(id);
    }

    pub(crate) fn mark_dynamic_component(&mut self, id: NodeId) {
        self.dynamic_components.insert(id);
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
        let Some(data) = ctx.data.expression_data(tag.id) else {
            return;
        };
        if matches!(
            data.volatility,
            Volatility::Reactive | Volatility::Heavy | Volatility::Asynchronous
        ) {
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
        let Some(expr) = ctx.parsed.and_then(|p| p.expr(cn.name.id())) else {
            return;
        };
        if uses_runes
            && matches!(
                expr.get_inner_expression(),
                Expression::StaticMemberExpression(_)
            )
        {
            ctx.data.dynamism.mark_dynamic_component(cn.id);
            return;
        }
        if let Some(ref_id) = expression_root_reference_id(expr)
            && let Some(sym_id) = data.scoping.symbol_for_reference(ref_id)
            && uses_runes
            && is_reactive_component_binding(data, sym_id)
        {
            ctx.data.dynamism.mark_dynamic_component(cn.id);
        }
    }

    fn visit_svelte_component_legacy(
        &mut self,
        cn: &svelte_ast::SvelteComponentLegacy,
        ctx: &mut VisitContext<'_, '_>,
    ) {
        ctx.data.dynamism.mark_dynamic_component(cn.id);
    }

    fn visit_js_expression(
        &mut self,
        node_id: NodeId,
        _expr: &Expression<'_>,
        ctx: &mut VisitContext<'_, '_>,
    ) {
        let parent = ctx.data.expr_parent(node_id);
        let parent_kind = parent.map(|p| p.kind);

        if parent_kind.is_some_and(|k| k.is_attr()) {
            let attr_id = parent.map_or(node_id, |p| p.id);
            let in_component = ctx.data.expr_ancestors(node_id).nth(1).is_some_and(|gp| {
                matches!(
                    gp.kind,
                    ParentKind::ComponentNode | ParentKind::SvelteSelf | ParentKind::SvelteBoundary
                )
            });

            let Some(data) = ctx.data.expression_data(node_id) else {
                return;
            };

            if matches!(
                data.volatility,
                Volatility::Reactive | Volatility::Heavy | Volatility::Asynchronous
            ) {
                ctx.data.dynamism.mark_dynamic_attr(attr_id);
                if !in_component && let Some(el_id) = ctx.data.nearest_element_for_expr(node_id) {
                    ctx.data.elements.flags.needs_ref.insert(el_id);
                }
            }
        } else if !(matches!(parent_kind, Some(ParentKind::SvelteElement))
            && parent.is_some_and(|p| p.id == node_id))
        {
            let Some(data) = ctx.data.expression_data(node_id) else {
                return;
            };
            if matches!(
                data.volatility,
                Volatility::Reactive | Volatility::Heavy | Volatility::Asynchronous
            ) {
                ctx.data.dynamism.mark_dynamic_node(node_id);
            }
        }
    }
}

fn is_reactive_component_binding(data: &AnalysisData<'_>, sym: SymbolId) -> bool {
    !matches!(
        data.binding_semantics(sym),
        BindingSemantics::MaybeReactive
            | BindingSemantics::NonReactive
            | BindingSemantics::Unresolved,
    )
}
