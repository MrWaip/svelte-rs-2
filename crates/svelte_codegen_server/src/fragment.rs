use std::borrow::Cow;

use svelte_analyze::BlockSemantics;
use svelte_ast::{Element, FragmentId, Node, NodeId};

use crate::error::Result;
use crate::escape::escape_text;
use crate::model::ServerCodegen;

#[derive(Clone, Copy)]
pub(crate) enum FragmentParent<'n> {
    Root,
    Element(&'n Element),
    Component,
    Snippet,
}

impl<'a> ServerCodegen<'a> {
    pub(crate) fn fragment(
        &mut self,
        id: FragmentId,
        parent: FragmentParent<'a>,
        preserve_whitespace: bool,
    ) -> Result<()> {
        self.fragment_impl(id, parent, preserve_whitespace, true)
    }

    pub(crate) fn fragment_children_only(
        &mut self,
        id: FragmentId,
        parent: FragmentParent<'a>,
        preserve_whitespace: bool,
    ) -> Result<()> {
        self.fragment_impl(id, parent, preserve_whitespace, false)
    }

    fn fragment_impl(
        &mut self,
        id: FragmentId,
        parent: FragmentParent<'a>,
        preserve_whitespace: bool,
        emit_snippets: bool,
    ) -> Result<()> {
        let component = self.component;
        let source = component.source.as_str();
        let preserve_comments = self.analysis.script.preserve_comments;

        if emit_snippets {
            self.emit_fragment_snippets(id)?;
        }

        let fragment = component.store.fragment(id);
        let mut kept: Vec<&'a Node> = Vec::with_capacity(fragment.nodes.len());
        for &node_id in &fragment.nodes {
            let node = component.store.get(node_id);
            if is_filtered_out(node, preserve_comments) {
                continue;
            }
            kept.push(node);
        }

        let window = if preserve_whitespace {
            &kept[..]
        } else {
            boundary_window(&kept, source)
        };
        let window = strip_pre_first_newline(window, parent, source);

        if is_text_first(parent, window) {
            self.push_text("<!---->");
        }

        let is_standalone = self.fragment_is_standalone(window);
        let can_remove_space = space_only_text_removable(parent);
        let mut prev_text_ends_ws = false;
        for (i, node) in window.iter().enumerate() {
            let Node::Text(text) = node else {
                prev_text_ends_ws = false;
                self.node(node, preserve_whitespace, is_standalone)?;
                continue;
            };

            let raw_value = text.value(source);
            let trimmed = if preserve_whitespace {
                Cow::Borrowed(raw_value)
            } else {
                let prev_is_expr = i > 0 && is_expression_tag(window[i - 1]);
                let next_is_expr = i + 1 < window.len() && is_expression_tag(window[i + 1]);
                trim_text(
                    raw_value,
                    i == 0,
                    i == window.len() - 1,
                    prev_is_expr,
                    next_is_expr,
                    prev_text_ends_ws,
                )
            };

            prev_text_ends_ws = ends_with_ws(&trimmed);

            if trimmed.is_empty() {
                continue;
            }
            if can_remove_space && trimmed.as_ref() == " " {
                continue;
            }
            self.push_text(&escape_text(&trimmed));
        }

        if is_lone_script(window) {
            self.push_text("<!---->");
        }
        Ok(())
    }

    fn node(
        &mut self,
        node: &'a Node,
        preserve_whitespace: bool,
        is_standalone: bool,
    ) -> Result<()> {
        match node {
            Node::Comment(comment) => self.comment(comment),
            Node::ExpressionTag(tag) => return self.expression_tag(tag),
            Node::Element(element) => return self.element(element, preserve_whitespace),
            Node::ComponentNode(component) => {
                return self.component_node(component, is_standalone);
            }
            Node::RenderTag(tag) => return self.render_tag(tag, is_standalone),
            Node::Text(_)
            | Node::SlotElementLegacy(_)
            | Node::IfBlock(_)
            | Node::EachBlock(_)
            | Node::SnippetBlock(_)
            | Node::HtmlTag(_)
            | Node::ConstTag(_)
            | Node::DebugTag(_)
            | Node::KeyBlock(_)
            | Node::SvelteHead(_)
            | Node::SvelteFragmentLegacy(_)
            | Node::SvelteComponentLegacy(_)
            | Node::SvelteElement(_)
            | Node::SvelteWindow(_)
            | Node::SvelteDocument(_)
            | Node::SvelteBody(_)
            | Node::SvelteSelf(_)
            | Node::SvelteBoundary(_)
            | Node::AwaitBlock(_)
            | Node::Error(_) => {}
        }
        Ok(())
    }

    fn emit_fragment_snippets(&mut self, id: FragmentId) -> Result<()> {
        let node_ids: Vec<NodeId> = self
            .component
            .store
            .fragment(id)
            .nodes
            .iter()
            .copied()
            .filter(|nid| matches!(self.component.store.get(*nid), Node::SnippetBlock(_)))
            .collect();

        for node_id in node_ids {
            let mut local = Vec::new();
            self.route_snippet(node_id, &mut local)?;
            for decl in local {
                self.push_stmt(decl);
            }
        }
        Ok(())
    }

    fn fragment_is_standalone(&self, window: &[&Node]) -> bool {
        let [only] = window else {
            return false;
        };
        match only {
            Node::ComponentNode(cn) => {
                !self.expression_is_volatile(cn.id) && !self.analysis.has_component_css_props(cn.id)
            }
            Node::RenderTag(tag) => match self.analysis.block_semantics(tag.id) {
                BlockSemantics::Render(sem) => !sem.callee_volatility.is_volatile(),
                _ => false,
            },
            _ => false,
        }
    }
}

fn is_expression_tag(node: &Node) -> bool {
    match node {
        Node::ExpressionTag(_) => true,
        Node::Text(_)
        | Node::Element(_)
        | Node::SlotElementLegacy(_)
        | Node::ComponentNode(_)
        | Node::Comment(_)
        | Node::IfBlock(_)
        | Node::EachBlock(_)
        | Node::SnippetBlock(_)
        | Node::RenderTag(_)
        | Node::HtmlTag(_)
        | Node::ConstTag(_)
        | Node::DebugTag(_)
        | Node::KeyBlock(_)
        | Node::SvelteHead(_)
        | Node::SvelteFragmentLegacy(_)
        | Node::SvelteComponentLegacy(_)
        | Node::SvelteElement(_)
        | Node::SvelteWindow(_)
        | Node::SvelteDocument(_)
        | Node::SvelteBody(_)
        | Node::SvelteSelf(_)
        | Node::SvelteBoundary(_)
        | Node::AwaitBlock(_)
        | Node::Error(_) => false,
    }
}

fn is_filtered_out(node: &Node, preserve_comments: bool) -> bool {
    match node {
        Node::Comment(_) => !preserve_comments,
        Node::SnippetBlock(_)
        | Node::ConstTag(_)
        | Node::DebugTag(_)
        | Node::SvelteHead(_)
        | Node::SvelteWindow(_)
        | Node::SvelteDocument(_)
        | Node::SvelteBody(_)
        | Node::Error(_) => true,
        Node::Text(_)
        | Node::Element(_)
        | Node::SlotElementLegacy(_)
        | Node::ComponentNode(_)
        | Node::ExpressionTag(_)
        | Node::IfBlock(_)
        | Node::EachBlock(_)
        | Node::RenderTag(_)
        | Node::HtmlTag(_)
        | Node::KeyBlock(_)
        | Node::SvelteFragmentLegacy(_)
        | Node::SvelteComponentLegacy(_)
        | Node::SvelteElement(_)
        | Node::SvelteSelf(_)
        | Node::SvelteBoundary(_)
        | Node::AwaitBlock(_) => false,
    }
}

fn is_text_first(parent: FragmentParent<'_>, window: &[&Node]) -> bool {
    match parent {
        FragmentParent::Root | FragmentParent::Component | FragmentParent::Snippet => {}
        FragmentParent::Element(_) => return false,
    }
    let Some(first) = window.first() else {
        return false;
    };
    match first {
        Node::Text(_) => true,
        other => is_expression_tag(other),
    }
}

fn is_lone_script(window: &[&Node]) -> bool {
    let [Node::Element(el)] = window else {
        return false;
    };
    el.name == "script"
}

fn boundary_window<'w, 'n>(kept: &'w [&'n Node], source: &str) -> &'w [&'n Node] {
    let mut start = 0;
    while start < kept.len() {
        let Node::Text(t) = kept[start] else {
            break;
        };
        if !is_ws_only(t.value(source)) {
            break;
        }
        start += 1;
    }
    let mut end = kept.len();
    while end > start {
        let Node::Text(t) = kept[end - 1] else {
            break;
        };
        if !is_ws_only(t.value(source)) {
            break;
        }
        end -= 1;
    }
    &kept[start..end]
}

fn strip_pre_first_newline<'w, 'n>(
    window: &'w [&'n Node],
    parent: FragmentParent<'n>,
    source: &str,
) -> &'w [&'n Node] {
    let FragmentParent::Element(el) = parent else {
        return window;
    };
    if el.name != "pre" {
        return window;
    }
    let Some(Node::Text(t)) = window.first() else {
        return window;
    };
    let raw = t.raw_value(source);
    if raw == "\n" || raw == "\r\n" {
        return &window[1..];
    }
    window
}

fn space_only_text_removable(parent: FragmentParent<'_>) -> bool {
    let FragmentParent::Element(el) = parent else {
        return false;
    };
    matches!(
        el.name.as_str(),
        "select" | "tr" | "table" | "tbody" | "thead" | "tfoot" | "colgroup" | "datalist"
    )
}

fn trim_text<'s>(
    raw: &'s str,
    is_first: bool,
    is_last: bool,
    prev_is_expr: bool,
    next_is_expr: bool,
    prev_text_ends_ws: bool,
) -> Cow<'s, str> {
    let mut data: Cow<'s, str> = Cow::Borrowed(raw);

    if is_first {
        data = replace_leading_ws(data, "");
    } else if !prev_is_expr {
        let replacement = if prev_text_ends_ws { "" } else { " " };
        data = replace_leading_ws(data, replacement);
    }

    if is_last {
        data = replace_trailing_ws(data, "");
    } else if !next_is_expr {
        data = replace_trailing_ws(data, " ");
    }

    data
}

fn is_ws_byte(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\r' | b'\n')
}

fn is_ws_only(s: &str) -> bool {
    !s.is_empty() && s.as_bytes().iter().copied().all(is_ws_byte)
}

fn ends_with_ws(s: &str) -> bool {
    s.as_bytes().last().copied().is_some_and(is_ws_byte)
}

fn leading_ws_len(s: &str) -> usize {
    s.as_bytes().iter().take_while(|&&b| is_ws_byte(b)).count()
}

fn trailing_ws_len(s: &str) -> usize {
    s.as_bytes()
        .iter()
        .rev()
        .take_while(|&&b| is_ws_byte(b))
        .count()
}

fn replace_leading_ws<'s>(s: Cow<'s, str>, replacement: &str) -> Cow<'s, str> {
    let ws = leading_ws_len(&s);
    if ws == 0 {
        return s;
    }
    let rest_range = ws..s.len();
    match s {
        Cow::Borrowed(borrowed) if replacement.is_empty() => Cow::Borrowed(&borrowed[rest_range]),
        s => {
            let mut out = String::with_capacity(replacement.len() + s.len() - ws);
            out.push_str(replacement);
            out.push_str(&s[rest_range]);
            Cow::Owned(out)
        }
    }
}

fn replace_trailing_ws<'s>(s: Cow<'s, str>, replacement: &str) -> Cow<'s, str> {
    let ws = trailing_ws_len(&s);
    if ws == 0 {
        return s;
    }
    let prefix_end = s.len() - ws;
    match s {
        Cow::Borrowed(borrowed) if replacement.is_empty() => Cow::Borrowed(&borrowed[..prefix_end]),
        s => {
            let mut out = String::with_capacity(prefix_end + replacement.len());
            out.push_str(&s[..prefix_end]);
            out.push_str(replacement);
            Cow::Owned(out)
        }
    }
}
