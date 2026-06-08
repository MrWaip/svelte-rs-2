use svelte_ast::{FragmentRole, Namespace, is_whitespace_removable_parent};
use svelte_span::Span;

use super::concat::ConcatPart;
use super::fragment_anchor::FragmentAnchor;
use crate::context::Ctx;

fn fragment_children_are_svg<'a>(ctx: &Ctx<'a>, fragment_id: svelte_ast::FragmentId) -> bool {
    let fragment = ctx.query.component.store.fragment(fragment_id);
    let mut saw_svg = false;
    for &id in &fragment.nodes {
        if !matches!(
            ctx.query.component.store.get(id),
            svelte_ast::Node::Element(_) | svelte_ast::Node::SvelteElement(_)
        ) {
            continue;
        }
        match ctx.query.view.creation_namespace(id) {
            Some(Namespace::Svg) => saw_svg = true,
            Some(_) | None => return false,
        }
    }
    saw_svg
}

#[derive(Clone)]
pub(crate) struct FragmentCtx<'a> {
    pub preserve_whitespace: bool,
    pub preserve_comments: bool,
    pub inside_pre: bool,
    pub inside_textarea: bool,
    pub inside_script: bool,
    pub inside_svg_text: bool,
    pub can_remove_entirely: bool,
    pub inside_head: bool,
    pub parent_element_name: Option<String>,

    pub namespace: Namespace,
    pub role: FragmentRole,
    pub source: &'a str,
    pub anchor: FragmentAnchor,
    pub in_block_callback: bool,
}

impl<'a> FragmentCtx<'a> {
    pub fn root(ctx: &Ctx<'a>, fragment_id: svelte_ast::FragmentId) -> Self {
        let fragment = ctx.query.component.store.fragment(fragment_id);
        let namespace = ctx.query.view.fragment_namespace(fragment_id);
        let svg_fragment =
            matches!(namespace, Namespace::Svg) || fragment_children_are_svg(ctx, fragment_id);
        Self {
            preserve_whitespace: ctx.query.view.preserve_whitespace(),
            preserve_comments: ctx.query.view.preserve_comments(),
            inside_pre: false,
            inside_textarea: false,
            inside_script: false,
            inside_svg_text: false,
            can_remove_entirely: svg_fragment,
            inside_head: false,
            parent_element_name: None,
            namespace,
            role: fragment.role,
            source: ctx.state.source,
            anchor: FragmentAnchor::Root,
            in_block_callback: false,
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
            "pre" => next.inside_pre = true,
            "textarea" => next.inside_textarea = true,
            "script" => next.inside_script = true,
            _ => {}
        }
        next.parent_element_name = Some(el_name.to_string());
        if el_name == "foreignObject" {
            next.inside_svg_text = false;
        } else if el_name == "text" && matches!(new_ns, Namespace::Svg) {
            next.inside_svg_text = true;
        }
        next.can_remove_entirely = if el_name == "foreignObject" {
            false
        } else if matches!(new_ns, Namespace::Svg) {
            el_name != "text" && !next.inside_svg_text
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
        next.parent_element_name = None;
        next.inside_svg_text = false;
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
        next.parent_element_name = None;
        next.namespace = ctx.query.view.fragment_namespace(fragment_id);
        if !next.inside_svg_text
            && (matches!(next.namespace, Namespace::Svg)
                || fragment_children_are_svg(ctx, fragment_id))
        {
            next.can_remove_entirely = true;
        }
        next.role = role;
        next.anchor = new_anchor;
        next
    }

    pub fn child_of_named_slot(
        &self,
        ctx: &Ctx<'a>,
        fragment_id: Option<svelte_ast::FragmentId>,
        new_anchor: FragmentAnchor,
    ) -> Self {
        let mut next = self.clone();
        next.parent_element_name = None;
        next.inside_svg_text = false;
        next.namespace = Namespace::Html;
        next.can_remove_entirely = false;
        if let Some(fid) = fragment_id
            && fragment_children_are_svg(ctx, fid)
        {
            next.can_remove_entirely = true;
        }
        next.role = FragmentRole::NamedSlot;
        next.anchor = new_anchor;
        next
    }

    pub fn child_of_sibling(&self, sibling_var: String) -> Self {
        let mut next = self.clone();
        next.parent_element_name = None;
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
            ConcatPart::StaticEntities { text, .. } => Some(text.as_str()),
            ConcatPart::Expr(_) => None,
        }
    }

    pub fn static_html_of(&self, part: &'a ConcatPart) -> Option<&'a str> {
        match part {
            ConcatPart::Static(span) => Some(self.source_of(*span)),
            ConcatPart::StaticOwned(s) => Some(s.as_str()),
            ConcatPart::StaticEntities { html, .. } => Some(html.as_str()),
            ConcatPart::Expr(_) => None,
        }
    }
}
