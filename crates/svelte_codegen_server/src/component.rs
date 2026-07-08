use oxc_ast::ast::Statement;
use svelte_analyze::{BlockSemantics, ElementSemantics, LegacyDefaultSlot, SnippetSlotKey};
use svelte_ast::{ComponentNode, Node, NodeId};
use svelte_ast_builder::{Arg, ObjProp};

use crate::attribute::PropOrSpread;
use crate::error::{CodegenError, Result};
use crate::model::ServerCodegen;

impl<'a> ServerCodegen<'a> {
    pub(crate) fn component_node(
        &mut self,
        cn: &'a ComponentNode,
        is_standalone: bool,
    ) -> Result<()> {
        if self.expression_is_volatile(cn.id) {
            return Err(CodegenError::Unsupported(cn.id, "dynamic component"));
        }
        if self.analysis.has_component_css_props(cn.id) {
            return Err(CodegenError::Unsupported(cn.id, "component css props"));
        }

        let callee = self.take_expression(cn.id, &cn.name)?;

        let mut items: Vec<PropOrSpread<'a>> = Vec::new();
        for attr in &cn.attributes {
            if matches!(attr, svelte_ast::Attribute::LetDirectiveLegacy(_)) {
                continue;
            }
            self.emit_component_attribute(attr, &mut items)?;
        }

        let mut snippet_decls: Vec<Statement<'a>> = Vec::new();
        let mut slot_entries: Vec<ObjProp<'a>> = Vec::new();
        self.build_component_children(cn, &mut items, &mut snippet_decls, &mut slot_entries)?;

        if !slot_entries.is_empty() {
            items.push(PropOrSpread::Prop(ObjProp::KeyValue(
                "$$slots",
                self.b.object_expr(slot_entries),
            )));
        }

        let props_expr = self.build_props_expr(items);
        let call = self
            .b
            .call_expr_callee(callee, [Arg::Ident("$$renderer"), Arg::Expr(props_expr)]);
        let call_stmt = self.b.expr_stmt(call);

        if snippet_decls.is_empty() {
            self.push_stmt(call_stmt);
        } else {
            snippet_decls.push(call_stmt);
            self.push_stmt(self.b.block_stmt(snippet_decls));
        }

        if !is_standalone {
            self.push_text("<!---->");
        }
        Ok(())
    }

    fn build_component_children(
        &mut self,
        cn: &'a ComponentNode,
        items: &mut Vec<PropOrSpread<'a>>,
        snippet_decls: &mut Vec<Statement<'a>>,
        slot_entries: &mut Vec<ObjProp<'a>>,
    ) -> Result<()> {
        let child_ids: Vec<NodeId> = self.component.store.fragment(cn.fragment).nodes.to_vec();
        for child_id in &child_ids {
            if let Node::SnippetBlock(_) = self.component.store.get(*child_id) {
                self.route_snippet(*child_id, snippet_decls)?;
                let BlockSemantics::Snippet(sem) = self.analysis.block_semantics(*child_id) else {
                    return Err(CodegenError::Unsupported(*child_id, "snippet child"));
                };
                let name: &'a str = self
                    .b
                    .alloc_str(self.analysis.scoping.symbol_name(sem.name));
                let slot_key: &'a str = match sem.slot_key {
                    SnippetSlotKey::Default => "default",
                    SnippetSlotKey::Named => name,
                };
                items.push(PropOrSpread::Prop(ObjProp::Shorthand(name)));
                slot_entries.push(ObjProp::KeyValue(slot_key, self.b.bool_expr(true)));
            }
        }

        self.build_default_children(cn, items, slot_entries)?;

        for slot in &cn.legacy_slots {
            let Some(arrow) = self.build_named_slot_fill(slot)? else {
                continue;
            };
            let key: &'a str = self.b.alloc_str(&slot.name);
            slot_entries.push(ObjProp::KeyValue(key, arrow));
        }
        Ok(())
    }

    fn build_default_children(
        &mut self,
        cn: &'a ComponentNode,
        items: &mut Vec<PropOrSpread<'a>>,
        slot_entries: &mut Vec<ObjProp<'a>>,
    ) -> Result<()> {
        let (default_slot, default_wrapper) = match self.analysis.element_semantics.query(cn.id) {
            ElementSemantics::LegacyComponentSlots(sem) => (sem.default_slot, sem.default_wrapper),
            _ => (LegacyDefaultSlot::ChildrenProp, None),
        };

        let (owner_id, fragment, wrap_block) = match default_wrapper {
            Some(wrapper_id) => match self.component.store.get(wrapper_id) {
                Node::SvelteFragmentLegacy(el) => (wrapper_id, el.fragment, true),
                _ => (cn.id, cn.fragment, false),
            },
            None => (cn.id, cn.fragment, false),
        };

        let Some(arrow) = self.build_default_slot_fill(owner_id, fragment, wrap_block)? else {
            return Ok(());
        };

        match default_slot {
            LegacyDefaultSlot::ChildrenProp => {
                let arrow = if self.dev {
                    self.b
                        .call_expr("$.prevent_snippet_stringification", [Arg::Expr(arrow)])
                } else {
                    arrow
                };
                items.push(PropOrSpread::Prop(ObjProp::KeyValue("children", arrow)));
                slot_entries.push(ObjProp::KeyValue("default", self.b.bool_expr(true)));
            }
            LegacyDefaultSlot::SlotDefaultInvalid => {
                items.push(PropOrSpread::Prop(ObjProp::KeyValue(
                    "children",
                    self.b
                        .static_member_expr(self.b.rid_expr("$"), "invalid_default_snippet"),
                )));
                slot_entries.push(ObjProp::KeyValue("default", arrow));
            }
            LegacyDefaultSlot::SlotDefault => {
                slot_entries.push(ObjProp::KeyValue("default", arrow));
            }
        }
        Ok(())
    }
}
