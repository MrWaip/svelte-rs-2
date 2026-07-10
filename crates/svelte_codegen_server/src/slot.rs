use oxc_ast::NONE;
use oxc_ast::ast::{BindingPattern, BindingProperty, Expression, PropertyKey, Statement};
use oxc_span::SPAN;
use svelte_analyze::ElementSemantics;
use svelte_ast::{
    Attribute, FragmentId, LegacySlot, LetDirectiveLegacy, Node, NodeId, SlotElementLegacy,
};
use svelte_ast_builder::{Arg, ObjProp};

use crate::attribute::PropOrSpread;
use crate::error::Result;
use crate::fragment::FragmentParent;
use crate::model::ServerCodegen;

impl<'a> ServerCodegen<'a> {
    pub(crate) fn slot_element_legacy(&mut self, el: &'a SlotElementLegacy) -> Result<()> {
        let (slot_name, has_fallback) = match self.analysis.element_semantics.query(el.id) {
            ElementSemantics::LegacySlot(sem) => (self.b.alloc_str(&sem.name), sem.has_fallback),
            _ => ("default", false),
        };

        let mut items: Vec<PropOrSpread<'a>> = Vec::new();
        for attr in &el.attributes {
            self.emit_component_attribute(attr, &mut items)?;
        }
        let props_expr = self.build_slot_props_expr(items);

        let fallback = self.slot_fallback(el, has_fallback)?;

        let slot_stmt = self.b.call_stmt(
            "$.slot",
            [
                Arg::Ident("$$renderer"),
                Arg::Ident("$$props"),
                Arg::StrRef(slot_name),
                Arg::Expr(props_expr),
                Arg::Expr(fallback),
            ],
        );

        self.push_text("<!--[-->");
        self.push_stmt(slot_stmt);
        self.push_text("<!--]-->");
        Ok(())
    }

    fn build_slot_props_expr(&self, items: Vec<PropOrSpread<'a>>) -> Expression<'a> {
        let mut props: Vec<ObjProp<'a>> = Vec::new();
        let mut spreads: Vec<Expression<'a>> = Vec::new();
        for item in items {
            match item {
                PropOrSpread::Prop(prop) => props.push(prop),
                PropOrSpread::Spread(expr) => spreads.push(expr),
            }
        }
        if spreads.is_empty() {
            return self.b.object_expr(props);
        }
        let mut args: Vec<Arg<'a, 'a>> = Vec::with_capacity(spreads.len() + 1);
        args.push(Arg::Expr(self.b.object_expr(props)));
        for spread in spreads {
            args.push(Arg::Expr(spread));
        }
        let array = self.b.array_from_args(args);
        self.b.call_expr("$.spread_props", [Arg::Expr(array)])
    }

    fn slot_fallback(
        &mut self,
        el: &'a SlotElementLegacy,
        has_fallback: bool,
    ) -> Result<Expression<'a>> {
        if !has_fallback {
            return Ok(self.b.null_expr());
        }
        let fragment = el.fragment;
        let body = self.child_statements(|codegen| {
            codegen.emit_fragment_const_tags_hoisted(fragment)?;
            codegen.fragment_children_only(fragment, FragmentParent::Block)
        })?;
        Ok(self.b.arrow_block_expr(self.b.no_params(), body))
    }

    pub(crate) fn build_named_slot_fill(
        &mut self,
        slot: &'a LegacySlot,
    ) -> Result<Option<Expression<'a>>> {
        let nodes = &self.component.store.fragment(slot.fragment).nodes;
        let Some(&fill_node_id) = nodes.first() else {
            return Ok(None);
        };
        let body = self.slot_fill_body(fill_node_id, slot.fragment)?;
        let arrow = self.slot_fill_arrow(fill_node_id, body)?;
        Ok(Some(arrow))
    }

    pub(crate) fn build_default_slot_fill(
        &mut self,
        owner_id: NodeId,
        fragment: FragmentId,
        wrap_block: bool,
    ) -> Result<Option<Expression<'a>>> {
        let parent = if wrap_block {
            FragmentParent::Block
        } else {
            FragmentParent::Component
        };
        let inner = self.child_statements(|codegen| {
            codegen.emit_fragment_const_tags_hoisted(fragment)?;
            codegen.fragment_children_only(fragment, parent)
        })?;
        if inner.is_empty() {
            return Ok(None);
        }
        let body = if wrap_block {
            vec![self.b.block_stmt(inner)]
        } else {
            inner
        };
        Ok(Some(self.slot_fill_arrow(owner_id, body)?))
    }

    fn slot_fill_body(
        &mut self,
        fill_node_id: NodeId,
        slot_fragment: FragmentId,
    ) -> Result<Vec<Statement<'a>>> {
        if let Node::SvelteFragmentLegacy(el) = self.component.store.get(fill_node_id) {
            let inner_fragment = el.fragment;
            let inner = self.child_statements(|codegen| {
                codegen.emit_fragment_const_tags_hoisted(inner_fragment)?;
                codegen.fragment_children_only(inner_fragment, FragmentParent::Block)
            })?;
            return Ok(vec![self.b.block_stmt(inner)]);
        }
        self.child_statements(|codegen| {
            codegen.emit_fragment_const_tags_hoisted(slot_fragment)?;
            codegen.fragment_children_only(slot_fragment, FragmentParent::Component)
        })
    }

    fn slot_fill_arrow(
        &mut self,
        let_owner_id: NodeId,
        body: Vec<Statement<'a>>,
    ) -> Result<Expression<'a>> {
        let let_pattern = self.build_let_object_pattern(let_owner_id)?;
        let params = match let_pattern {
            Some(pattern) => self.b.formal_parameters([
                self.b.formal_parameter_from_str("$$renderer"),
                self.b.formal_parameter_from_pattern(pattern),
            ]),
            None => self.b.params(["$$renderer"]),
        };
        Ok(self.b.arrow_block_expr(params, body))
    }

    fn build_let_object_pattern(&mut self, owner_id: NodeId) -> Result<Option<BindingPattern<'a>>> {
        let directives = self.collect_let_directives(owner_id);
        if directives.is_empty() {
            return Ok(None);
        }
        let mut properties = self.b.ast.vec();
        for dir in &directives {
            let Some(property) = self.let_binding_property(dir)? else {
                continue;
            };
            properties.push(property);
        }
        if properties.is_empty() {
            return Ok(None);
        }
        let object = self.b.ast.object_pattern(SPAN, properties, NONE);
        Ok(Some(BindingPattern::ObjectPattern(self.b.alloc(object))))
    }

    fn let_binding_property(
        &mut self,
        dir: &LetDirectiveLegacy,
    ) -> Result<Option<BindingProperty<'a>>> {
        let Some(binding_ref) = dir.binding.as_ref() else {
            return Ok(None);
        };
        let Some(stmt) = self.js_arena.take_stmt(binding_ref.id()) else {
            return Ok(None);
        };
        let Statement::VariableDeclaration(mut decl) = stmt else {
            return Ok(None);
        };
        if decl.declarations.is_empty() {
            return Ok(None);
        }
        let pattern = decl.declarations.remove(0).id;
        let key_name: &'a str = self.b.alloc_str(&dir.name);
        let shorthand = matches!(
            &pattern,
            BindingPattern::BindingIdentifier(id) if id.name.as_str() == key_name
        );
        let key = PropertyKey::StaticIdentifier(
            self.b
                .alloc(self.b.ast.identifier_name(SPAN, self.b.ast.atom(key_name))),
        );
        Ok(Some(
            self.b
                .ast
                .binding_property(SPAN, key, pattern, shorthand, false),
        ))
    }

    fn collect_let_directives(&self, node_id: NodeId) -> Vec<LetDirectiveLegacy> {
        let node = self.component.store.get(node_id);
        let attrs: &[Attribute] = match node {
            Node::Element(el) => &el.attributes,
            Node::SvelteElement(el) => &el.attributes,
            Node::SvelteFragmentLegacy(el) => &el.attributes,
            Node::SlotElementLegacy(el) => &el.attributes,
            _ => match node.as_component_like() {
                Some(view) => view.attributes,
                None => return Vec::new(),
            },
        };
        let mut out = Vec::new();
        for attr in attrs {
            if let Attribute::LetDirectiveLegacy(dir) = attr {
                out.push(dir.clone());
            }
        }
        out
    }
}
