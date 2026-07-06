use oxc_ast::ast::{Expression, Statement};
use svelte_analyze::{BlockSemantics, LegacyDefaultSlot, SnippetSlotKey};
use svelte_ast::{ComponentNode, Node, NodeId};
use svelte_ast_builder::{Arg, ObjProp};

use crate::attribute::PropOrSpread;
use crate::error::{CodegenError, Result};
use crate::fragment::FragmentParent;
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
        if !cn.legacy_slots.is_empty() {
            return Err(CodegenError::Unsupported(cn.id, "component named slots"));
        }

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

        self.build_default_children(cn, items, slot_entries)
    }

    fn build_default_children(
        &mut self,
        cn: &'a ComponentNode,
        items: &mut Vec<PropOrSpread<'a>>,
        slot_entries: &mut Vec<ObjProp<'a>>,
    ) -> Result<()> {
        let fragment = cn.fragment;
        let preserve = self.analysis.script.preserve_whitespace;
        let body = self.child_statements(|codegen| {
            codegen.fragment_children_only(fragment, FragmentParent::Component, preserve)
        })?;
        if body.is_empty() {
            return Ok(());
        }

        match self.analysis.legacy_default_slot(cn.id) {
            LegacyDefaultSlot::ChildrenProp => {
                let arrow = self.b.arrow_block(self.b.params(["$$renderer"]), body);
                let mut arrow_expr = Expression::ArrowFunctionExpression(self.b.alloc(arrow));
                if self.dev {
                    arrow_expr = self
                        .b
                        .call_expr("$.prevent_snippet_stringification", [Arg::Expr(arrow_expr)]);
                }
                items.push(PropOrSpread::Prop(ObjProp::KeyValue(
                    "children", arrow_expr,
                )));
                slot_entries.push(ObjProp::KeyValue("default", self.b.bool_expr(true)));
                Ok(())
            }
            LegacyDefaultSlot::SlotDefault | LegacyDefaultSlot::SlotDefaultInvalid => {
                Err(CodegenError::Unsupported(cn.id, "legacy default slot"))
            }
        }
    }
}
