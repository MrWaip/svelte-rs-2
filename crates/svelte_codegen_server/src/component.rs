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
    CssProp,
}

struct ComponentInvocation<'a> {
    call: Statement<'a>,
    snippet_decls: Vec<Statement<'a>>,
    css_props: Vec<ObjProp<'a>>,
}

struct ComponentEmit<'a> {
    id: NodeId,
    fragment: FragmentId,
    attributes: &'a [Attribute],
    legacy_slots: &'a [LegacySlot],
    callee: Expression<'a>,
    dynamic_test: Option<Expression<'a>>,
    is_standalone: bool,
}

impl<'a> ServerCodegen<'a> {
    pub(crate) fn component_node(
        &mut self,
        cn: &'a ComponentNode,
        is_standalone: bool,
    ) -> Result<()> {
        let callee = self.take_expression(cn.id, &cn.name)?;
        let dynamic_test = self
            .expression_is_volatile(cn.id)
            .then(|| self.b.clone_expr(&callee));
        self.emit_component_like(ComponentEmit {
            id: cn.id,
            fragment: cn.fragment,
            attributes: &cn.attributes,
            legacy_slots: &cn.legacy_slots,
            callee,
            dynamic_test,
            is_standalone,
        })
    }

    pub(crate) fn svelte_self(&mut self, node: &'a SvelteSelf, is_standalone: bool) -> Result<()> {
        let name: &'a str = self.b.alloc_str(self.analysis.component_name());
        let callee = self.b.rid_expr(name);
        self.emit_component_like(ComponentEmit {
            id: node.id,
            fragment: node.fragment,
            attributes: &node.attributes,
            legacy_slots: &node.legacy_slots,
            callee,
            dynamic_test: None,
            is_standalone,
        })
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
        let callee = self.b.clone_expr(&this_expr);
        self.emit_component_like(ComponentEmit {
            id: node.id,
            fragment: node.fragment,
            attributes: &node.attributes,
            legacy_slots: &node.legacy_slots,
            callee,
            dynamic_test: Some(this_expr),
            is_standalone: false,
        })
    }

    fn emit_component_like(&mut self, emit: ComponentEmit<'a>) -> Result<()> {
        let ComponentEmit {
            id,
            fragment,
            attributes,
            legacy_slots,
            callee,
            dynamic_test,
            is_standalone,
        } = emit;
        let async_kind = self
            .analysis
            .element_semantics
            .query(id)
            .async_kind()
            .clone();
        let is_dynamic = dynamic_test.is_some();

        let (built, hoists) = self.with_promise_hoisting(|cg| {
            let invocation =
                cg.build_component_invocation(id, fragment, attributes, legacy_slots, callee)?;
            cg.assemble_component_statement(id, invocation, dynamic_test)
        });
        let statement = built?;

        if !async_kind.is_sync() {
            let mut body = hoists;
            body.push(statement);
            let wrapped =
                self.wrap_async_block_flagged(body, async_kind.blockers(), async_kind.awaited());
            self.push_stmt(wrapped);
            return Ok(());
        }

        for hoist in hoists {
            self.push_stmt(hoist);
        }
        self.push_stmt(statement);
        if self.component_call_needs_marker(id, is_dynamic, is_standalone) {
            self.push_text("<!---->");
        }
        Ok(())
    }

    fn component_call_needs_marker(
        &self,
        id: NodeId,
        is_dynamic: bool,
        is_standalone: bool,
    ) -> bool {
        if is_dynamic {
            return false;
        }
        if is_standalone {
            return false;
        }
        !self.analysis.has_component_css_props(id)
    }

    fn assemble_component_statement(
        &mut self,
        id: NodeId,
        invocation: ComponentInvocation<'a>,
        dynamic_test: Option<Expression<'a>>,
    ) -> Result<Statement<'a>> {
        let ComponentInvocation {
            call,
            snippet_decls,
            css_props,
        } = invocation;

        let is_dynamic = dynamic_test.is_some();
        let mut statement = match dynamic_test {
            Some(test) => {
                let consequent = vec![
                    self.renderer_push_string_stmt("<!--[-->"),
                    call,
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
            None => call,
        };

        if !snippet_decls.is_empty() {
            let mut block = snippet_decls;
            block.push(statement);
            statement = self.b.block_stmt(block);
        }

        if css_props.is_empty() {
            return Ok(statement);
        }

        let is_html = match self.analysis.namespace(id) {
            None
            | Some(NamespaceKind::Html)
            | Some(NamespaceKind::ForeignObject)
            | Some(NamespaceKind::AnnotationXml) => true,
            Some(NamespaceKind::Svg) | Some(NamespaceKind::MathMl) => false,
        };
        let arrow = self.b.arrow_block_expr(self.b.no_params(), vec![statement]);
        let mut args = vec![
            Arg::Ident("$$renderer"),
            Arg::Bool(is_html),
            Arg::Expr(self.b.object_expr(css_props)),
            Arg::Expr(arrow),
        ];
        if is_dynamic {
            args.push(Arg::Bool(true));
        }
        Ok(self.b.expr_stmt(self.b.call_expr("$.css_props", args)))
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

    fn build_css_prop(
        &mut self,
        id: NodeId,
        attr: &'a Attribute,
        props: &mut Vec<ObjProp<'a>>,
    ) -> Result<()> {
        let AttributeSemantics::ComponentCssProp(value) = self.analysis.attributes.get(attr.id())
        else {
            return Ok(());
        };
        let value = value.clone();
        let Some(name) = attr.name() else {
            return Ok(());
        };
        let key: &'a str = self.b.alloc_str(name);
        let expr = match value {
            ComponentCssPropValue::Expression(oxc_id) => {
                let expr = self.take_expr_by_oxc_id(id, oxc_id)?;
                self.maybe_hoist_async_expr(attr.id(), expr)
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
        Ok(())
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
            AttributeSemantics::ComponentCssProp(_) => ComponentAttrRouting::CssProp,
            AttributeSemantics::SvelteComponentThis(_)
            | AttributeSemantics::ComponentAttach(_)
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
    ) -> Result<ComponentInvocation<'a>> {
        let mut items: Vec<PropOrSpread<'a>> = Vec::new();
        let mut css_props: Vec<ObjProp<'a>> = Vec::new();
        for pass in [ComponentAttrPass::NonBind, ComponentAttrPass::Bind] {
            for attr in attributes {
                let routing = self.component_attr_routing(attr);
                match (&pass, routing) {
                    (ComponentAttrPass::NonBind, ComponentAttrRouting::NonBind)
                    | (ComponentAttrPass::Bind, ComponentAttrRouting::Bind) => {
                        self.emit_component_attribute(attr, &mut items)?;
                    }
                    (ComponentAttrPass::NonBind, ComponentAttrRouting::CssProp) => {
                        self.build_css_prop(id, attr, &mut css_props)?;
                    }
                    (ComponentAttrPass::NonBind, ComponentAttrRouting::Bind)
                    | (ComponentAttrPass::Bind, ComponentAttrRouting::NonBind)
                    | (ComponentAttrPass::Bind, ComponentAttrRouting::CssProp)
                    | (_, ComponentAttrRouting::Skip) => {}
                }
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
        Ok(ComponentInvocation {
            call: self.b.expr_stmt(call),
            snippet_decls,
            css_props,
        })
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
                ElementSemantics::Component(sem) => (
                    sem.legacy_slots.default_slot,
                    sem.legacy_slots.default_wrapper,
                    sem.legacy_slots.default_let_owner,
                ),
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
