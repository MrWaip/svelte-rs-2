use oxc_ast::ast::{Expression, Statement};
use oxc_syntax::node::NodeId as OxcNodeId;
use svelte_analyze::{
    AttributeSemantics, BlockSemantics, ComponentBindKind, ComponentCssPropValue, ElementSemantics,
    LegacyDefaultSlot, NamespaceKind, SnippetSlotKey,
};
use svelte_ast::{Attribute, ComponentNode, FragmentId, LegacySlot, Node, NodeId, SvelteSelf};
use svelte_ast_builder::{Arg, ObjProp};

use crate::attribute::PropOrSpread;
use crate::error::{CodegenError, Result};
use crate::model::ServerCodegen;

#[derive(Clone, Copy)]
enum ComponentAttrPass {
    NonBind,
    Bind,
}

enum ComponentAttrRouting {
    Skip,
    Bind,
    NonBind,
}

impl<'a> ServerCodegen<'a> {
    pub(crate) fn component_node(
        &mut self,
        cn: &'a ComponentNode,
        is_standalone: bool,
    ) -> Result<()> {
        let callee = self.take_expression(cn.id, &cn.name)?;

        if self.analysis.has_component_css_props(cn.id) {
            let dynamic_test = self
                .expression_is_volatile(cn.id)
                .then(|| self.b.clone_expr(&callee));
            return self.emit_component_css_props(
                cn.id,
                cn.fragment,
                &cn.attributes,
                &cn.legacy_slots,
                callee,
                dynamic_test,
            );
        }

        if self.expression_is_volatile(cn.id) {
            return self.emit_dynamic_component(
                cn.id,
                cn.fragment,
                &cn.attributes,
                &cn.legacy_slots,
                callee,
            );
        }

        let (call_stmt, snippet_decls) = self.build_component_invocation(
            cn.id,
            cn.fragment,
            &cn.attributes,
            &cn.legacy_slots,
            callee,
        )?;
        self.push_component_call(call_stmt, snippet_decls);

        if !is_standalone {
            self.push_text("<!---->");
        }
        Ok(())
    }

    pub(crate) fn svelte_self(&mut self, node: &'a SvelteSelf, is_standalone: bool) -> Result<()> {
        let name: &'a str = self.b.alloc_str(self.analysis.component_name());
        let callee = self.b.rid_expr(name);
        if self.analysis.has_component_css_props(node.id) {
            return self.emit_component_css_props(
                node.id,
                node.fragment,
                &node.attributes,
                &node.legacy_slots,
                callee,
                None,
            );
        }
        let (call_stmt, snippet_decls) = self.build_component_invocation(
            node.id,
            node.fragment,
            &node.attributes,
            &node.legacy_slots,
            callee,
        )?;
        self.push_component_call(call_stmt, snippet_decls);

        if !is_standalone {
            self.push_text("<!---->");
        }
        Ok(())
    }

    pub(crate) fn svelte_component_legacy(
        &mut self,
        node: &'a svelte_ast::SvelteComponentLegacy,
    ) -> Result<()> {
        let this_expr_id =
            self.svelte_component_this_id(&node.attributes)
                .ok_or(CodegenError::Unsupported(
                    node.id,
                    "svelte:component without this",
                ))?;
        let this_expr = self.take_expr_by_oxc_id(node.id, this_expr_id)?;

        if self.analysis.has_component_css_props(node.id) {
            let callee = self.b.clone_expr(&this_expr);
            return self.emit_component_css_props(
                node.id,
                node.fragment,
                &node.attributes,
                &node.legacy_slots,
                callee,
                Some(this_expr),
            );
        }

        self.emit_dynamic_component(
            node.id,
            node.fragment,
            &node.attributes,
            &node.legacy_slots,
            this_expr,
        )
    }

    fn emit_dynamic_component(
        &mut self,
        id: NodeId,
        fragment: FragmentId,
        attributes: &'a [Attribute],
        legacy_slots: &'a [LegacySlot],
        this_expr: Expression<'a>,
    ) -> Result<()> {
        let callee = self.b.clone_expr(&this_expr);
        let (call_stmt, snippet_decls) =
            self.build_component_invocation(id, fragment, attributes, legacy_slots, callee)?;

        let consequent = vec![
            self.renderer_push_string_stmt("<!--[-->"),
            call_stmt,
            self.renderer_push_string_stmt("<!--]-->"),
        ];

        let alternate = vec![
            self.renderer_push_string_stmt("<!--[!-->"),
            self.renderer_push_string_stmt("<!--]-->"),
        ];

        let if_stmt = self.b.if_stmt(
            this_expr,
            self.b.block_stmt(consequent),
            Some(self.b.block_stmt(alternate)),
        );

        if snippet_decls.is_empty() {
            self.push_stmt(if_stmt);
        } else {
            let mut block = snippet_decls;
            block.push(if_stmt);
            self.push_stmt(self.b.block_stmt(block));
        }
        Ok(())
    }

    fn svelte_component_this_id(&self, attributes: &'a [Attribute]) -> Option<OxcNodeId> {
        for attr in attributes {
            if let AttributeSemantics::SvelteComponentThis(sem) =
                self.analysis.attributes.get(attr.id())
            {
                return Some(sem.expr_id);
            }
        }
        None
    }

    fn push_component_call(
        &mut self,
        call_stmt: Statement<'a>,
        mut snippet_decls: Vec<Statement<'a>>,
    ) {
        if snippet_decls.is_empty() {
            self.push_stmt(call_stmt);
        } else {
            snippet_decls.push(call_stmt);
            self.push_stmt(self.b.block_stmt(snippet_decls));
        }
    }

    fn emit_component_css_props(
        &mut self,
        id: NodeId,
        fragment: FragmentId,
        attributes: &'a [Attribute],
        legacy_slots: &'a [LegacySlot],
        callee: Expression<'a>,
        dynamic_test: Option<Expression<'a>>,
    ) -> Result<()> {
        let (call_stmt, snippet_decls) =
            self.build_component_invocation(id, fragment, attributes, legacy_slots, callee)?;
        let props_obj = self.build_css_props_object(id, attributes)?;
        let is_html = match self.analysis.namespace(id) {
            None
            | Some(NamespaceKind::Html)
            | Some(NamespaceKind::ForeignObject)
            | Some(NamespaceKind::AnnotationXml) => true,
            Some(NamespaceKind::Svg) | Some(NamespaceKind::MathMl) => false,
        };

        let is_dynamic = dynamic_test.is_some();
        let tail_stmt = match dynamic_test {
            Some(test) => {
                let consequent = vec![
                    self.renderer_push_string_stmt("<!--[-->"),
                    call_stmt,
                    self.renderer_push_string_stmt("<!--]-->"),
                ];
                let alternate = vec![
                    self.renderer_push_string_stmt("<!--[!-->"),
                    self.renderer_push_string_stmt("<!--]-->"),
                ];
                self.b.if_stmt(
                    test,
                    self.b.block_stmt(consequent),
                    Some(self.b.block_stmt(alternate)),
                )
            }
            None => call_stmt,
        };

        let body = if snippet_decls.is_empty() {
            vec![tail_stmt]
        } else {
            let mut inner = snippet_decls;
            inner.push(tail_stmt);
            vec![self.b.block_stmt(inner)]
        };

        let arrow = self.b.arrow_block_expr(self.b.no_params(), body);
        let mut args = vec![
            Arg::Ident("$$renderer"),
            Arg::Bool(is_html),
            Arg::Expr(props_obj),
            Arg::Expr(arrow),
        ];
        if is_dynamic {
            args.push(Arg::Bool(true));
        }
        let call = self.b.call_expr("$.css_props", args);
        self.push_stmt(self.b.expr_stmt(call));
        Ok(())
    }

    fn build_css_props_object(
        &mut self,
        id: NodeId,
        attributes: &'a [Attribute],
    ) -> Result<Expression<'a>> {
        let mut props: Vec<ObjProp<'a>> = Vec::new();
        for attr in attributes {
            let value = match self.analysis.attributes.get(attr.id()) {
                AttributeSemantics::ComponentCssProp(value) => value.clone(),
                _ => continue,
            };
            let Some(name) = attr.name() else {
                continue;
            };
            let key: &'a str = self.b.alloc_str(name);
            let expr = match value {
                ComponentCssPropValue::Expression(oxc_id) => {
                    self.take_expr_by_oxc_id(id, oxc_id)?
                }
                ComponentCssPropValue::StaticString(span) => {
                    let text: &'a str = self.b.alloc_str(self.component.source_text(span));
                    self.b.str_expr(text)
                }
                ComponentCssPropValue::Boolean => self.b.bool_expr(true),
                ComponentCssPropValue::Concatenation(_) => {
                    let Attribute::ConcatenationAttribute(ca) = attr else {
                        return Err(CodegenError::Unsupported(id, "css prop concatenation"));
                    };
                    self.concat_value_expr(&ca.parts)?
                }
            };
            props.push(ObjProp::KeyValue(key, expr));
        }
        Ok(self.b.object_expr(props))
    }

    fn component_attr_routing(&self, attr: &'a Attribute) -> ComponentAttrRouting {
        match self.analysis.attributes.get(attr.id()) {
            AttributeSemantics::ComponentBind(sem) => match sem.kind {
                ComponentBindKind::This { .. } => ComponentAttrRouting::Skip,
                ComponentBindKind::FunctionPair => ComponentAttrRouting::NonBind,
                ComponentBindKind::Expression
                | ComponentBindKind::Identifier { .. }
                | ComponentBindKind::StoreSubscribed { .. }
                | ComponentBindKind::StoreMemberMutation { .. } => ComponentAttrRouting::Bind,
            },
            AttributeSemantics::SvelteComponentThis(_)
            | AttributeSemantics::ComponentAttach(_)
            | AttributeSemantics::ComponentCssProp(_)
            | AttributeSemantics::Event(_)
            | AttributeSemantics::Skip(_) => ComponentAttrRouting::Skip,
            _ => ComponentAttrRouting::NonBind,
        }
    }

    fn build_component_invocation(
        &mut self,
        id: NodeId,
        fragment: FragmentId,
        attributes: &'a [Attribute],
        legacy_slots: &'a [LegacySlot],
        callee: Expression<'a>,
    ) -> Result<(Statement<'a>, Vec<Statement<'a>>)> {
        let mut items: Vec<PropOrSpread<'a>> = Vec::new();
        for pass in [ComponentAttrPass::NonBind, ComponentAttrPass::Bind] {
            for attr in attributes {
                let wanted = match (&pass, self.component_attr_routing(attr)) {
                    (ComponentAttrPass::NonBind, ComponentAttrRouting::NonBind) => true,
                    (ComponentAttrPass::Bind, ComponentAttrRouting::Bind) => true,
                    (ComponentAttrPass::NonBind, ComponentAttrRouting::Bind)
                    | (ComponentAttrPass::Bind, ComponentAttrRouting::NonBind)
                    | (_, ComponentAttrRouting::Skip) => false,
                };
                if !wanted {
                    continue;
                }
                self.emit_component_attribute(attr, &mut items)?;
            }
        }

        let mut snippet_decls: Vec<Statement<'a>> = Vec::new();
        let mut slot_entries: Vec<ObjProp<'a>> = Vec::new();
        self.build_component_children(
            id,
            fragment,
            legacy_slots,
            &mut items,
            &mut snippet_decls,
            &mut slot_entries,
        )?;

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
        Ok((self.b.expr_stmt(call), snippet_decls))
    }

    fn build_component_children(
        &mut self,
        id: NodeId,
        fragment: FragmentId,
        legacy_slots: &'a [LegacySlot],
        items: &mut Vec<PropOrSpread<'a>>,
        snippet_decls: &mut Vec<Statement<'a>>,
        slot_entries: &mut Vec<ObjProp<'a>>,
    ) -> Result<()> {
        let child_ids: Vec<NodeId> = self.component.store.fragment(fragment).nodes.to_vec();
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

        self.build_default_children(id, fragment, items, slot_entries)?;

        for slot in legacy_slots {
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
        id: NodeId,
        node_fragment: FragmentId,
        items: &mut Vec<PropOrSpread<'a>>,
        slot_entries: &mut Vec<ObjProp<'a>>,
    ) -> Result<()> {
        let (default_slot, default_wrapper, default_let_owner) =
            match self.analysis.element_semantics.query(id) {
                ElementSemantics::LegacyComponentSlots(sem) => {
                    (sem.default_slot, sem.default_wrapper, sem.default_let_owner)
                }
                _ => (LegacyDefaultSlot::ChildrenProp, None, None),
            };

        let (owner_id, fragment, wrap_block) = match default_let_owner {
            Some(let_owner) => (let_owner, node_fragment, false),
            None => match default_wrapper {
                Some(wrapper_id) => match self.component.store.get(wrapper_id) {
                    Node::SvelteFragmentLegacy(el) => (wrapper_id, el.fragment, true),
                    _ => (id, node_fragment, false),
                },
                None => (id, node_fragment, false),
            },
        };

        let apply_let_scope = !matches!(
            default_slot,
            LegacyDefaultSlot::ChildrenProp | LegacyDefaultSlot::OwnLetDisplaced
        );
        let Some(arrow) =
            self.build_default_slot_fill(owner_id, fragment, wrap_block, apply_let_scope)?
        else {
            return Ok(());
        };

        match default_slot {
            LegacyDefaultSlot::ChildrenProp | LegacyDefaultSlot::OwnLetDisplaced => {
                let arrow = if self.dev {
                    self.b
                        .call_expr("$.prevent_snippet_stringification", [Arg::Expr(arrow)])
                } else {
                    arrow
                };
                items.push(PropOrSpread::Prop(ObjProp::KeyValue("children", arrow)));
                slot_entries.push(ObjProp::KeyValue("default", self.b.bool_expr(true)));
            }
            LegacyDefaultSlot::SlotDefaultInvalid | LegacyDefaultSlot::SlotDefaultSlottedLet => {
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
