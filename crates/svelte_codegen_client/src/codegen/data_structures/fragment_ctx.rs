use svelte_ast::{FragmentRole, Namespace, is_svg, is_whitespace_removable_parent};
use svelte_span::Span;

use super::concat::ConcatPart;
use super::fragment_anchor::FragmentAnchor;
use crate::context::Ctx;

#[derive(Clone)]
pub(crate) struct FragmentCtx<'a> {
    pub preserve_whitespace: bool,
    pub is_pre: bool,
    pub is_textarea: bool,
    pub can_remove_entirely: bool,
    pub inside_head: bool,

    pub namespace: Namespace,
    pub role: FragmentRole,
    pub source: &'a str,
    pub anchor: FragmentAnchor,
}

impl<'a> FragmentCtx<'a> {
    pub fn root(ctx: &Ctx<'a>, fragment_id: svelte_ast::FragmentId) -> Self {
        let fragment = ctx.query.component.store.fragment(fragment_id);
        Self {
            preserve_whitespace: ctx.query.view.preserve_whitespace(),
            is_pre: false,
            is_textarea: false,
            can_remove_entirely: false,
            inside_head: false,
            namespace: ctx.query.view.fragment_namespace(fragment_id),
            role: fragment.role,
            source: ctx.state.source,
            anchor: FragmentAnchor::Root,
        }
    }

    pub fn child_of_element(
        &self,
        ctx: &Ctx<'a>,
        el_name: &str,
        fragment_id: svelte_ast::FragmentId,
        new_ns: Namespace,
        new_anchor: FragmentAnchor,
    ) -> Self {
        let role = ctx.query.component.store.fragment(fragment_id).role;
        let mut next = self.clone();
        match el_name {
            "pre" => next.is_pre = true,
            "textarea" => next.is_textarea = true,
            _ => {}
        }
        next.can_remove_entirely = if el_name == "foreignObject" {
            false
        } else if el_name != "text" && (is_svg(el_name) || self.can_remove_entirely) {
            true
        } else {
            is_whitespace_removable_parent(el_name)
        };
        next.inside_head = false;
        next.namespace = new_ns;
        next.role = role;
        next.anchor = new_anchor;
        next
    }

    pub fn child_of_svelte_head(&self, ctx: &Ctx<'a>, fragment_id: svelte_ast::FragmentId) -> Self {
        let role = ctx.query.component.store.fragment(fragment_id).role;
        let mut next = self.clone();
        next.inside_head = true;
        next.namespace = Namespace::Html;
        next.role = role;
        next.anchor = FragmentAnchor::CallbackParam {
            name: "$$anchor".to_string(),
            append_inside: false,
        };
        next
    }

    pub fn child_of_block(
        &self,
        ctx: &Ctx<'a>,
        fragment_id: svelte_ast::FragmentId,
        new_anchor: FragmentAnchor,
    ) -> Self {
        let role = ctx.query.component.store.fragment(fragment_id).role;
        let mut next = self.clone();
        next.namespace = ctx.query.view.fragment_namespace(fragment_id);
        next.role = role;
        next.anchor = new_anchor;
        next
    }

    pub fn child_of_named_slot(&self, new_anchor: FragmentAnchor) -> Self {
        let mut next = self.clone();
        next.namespace = Namespace::Html;
        next.role = FragmentRole::NamedSlot;
        next.anchor = new_anchor;
        next
    }

    pub fn child_of_sibling(&self, sibling_var: String) -> Self {
        let mut next = self.clone();
        next.anchor = FragmentAnchor::SiblingVar { var: sibling_var };
        next
    }

    pub fn source_of(&self, span: Span) -> &'a str {
        span.source_text(self.source)
    }

    pub fn static_text_of(&self, part: &'a ConcatPart) -> Option<&'a str> {
        match part {
            ConcatPart::Static(span) => Some(self.source_of(*span)),
            ConcatPart::StaticOwned(s) => Some(s.as_str()),
            ConcatPart::Expr(_) => None,
        }
    }
}
