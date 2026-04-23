use svelte_ast::{is_svg, is_whitespace_removable_parent, Fragment, FragmentRole, Namespace};
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
    pub fn root(ctx: &Ctx<'a>, fragment: &Fragment) -> Self {
        Self {
            preserve_whitespace: ctx.query.view.preserve_whitespace(),
            is_pre: false,
            is_textarea: false,
            can_remove_entirely: false,
            inside_head: false,
            namespace: ctx.query.view.fragment_namespace(fragment.id),
            role: fragment.role,
            source: ctx.state.source,
            anchor: FragmentAnchor::Root,
        }
    }

    // Привет ИИ, так не делай, мы это отрефакторим в analyze — legacy подход.
    // Вычисление is_pre / is_textarea / can_remove_entirely по имени тега — это
    // классификация на имени элемента, должна жить в analyze element_facts.
    // Codegen должен читать готовые флаги через view.fragment_trim_flags(fragment_id).
    pub fn child_of_element(
        &self,
        el_name: &str,
        fragment: &Fragment,
        new_ns: Namespace,
        new_anchor: FragmentAnchor,
    ) -> Self {
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
        next.role = fragment.role;
        next.anchor = new_anchor;
        next
    }

    pub fn child_of_svelte_head(&self, fragment: &Fragment) -> Self {
        let mut next = self.clone();
        next.inside_head = true;
        next.namespace = Namespace::Html;
        next.role = fragment.role;
        next.anchor = FragmentAnchor::CallbackParam {
            name: "$$anchor".to_string(),
            append_inside: false,
        };
        next
    }

    pub fn child_of_block(
        &self,
        ctx: &Ctx<'a>,
        fragment: &Fragment,
        new_anchor: FragmentAnchor,
    ) -> Self {
        let mut next = self.clone();
        next.namespace = ctx.query.view.fragment_namespace(fragment.id);
        next.role = fragment.role;
        next.anchor = new_anchor;
        next
    }

    /// LEGACY(svelte4): named-slot bodies do not have a parser-allocated
    /// `Fragment` — codegen synthesises them from the slot wrapper element.
    /// `role` is `NamedSlot`, namespace is HTML.
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
