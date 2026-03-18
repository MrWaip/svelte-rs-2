use oxc_semantic::ScopeId;
use svelte_ast::{
    AnimateDirective, AttachTag, Attribute, BindDirective, ComponentNode, ConstTag, EachBlock,
    Element, ExpressionTag, Fragment, HtmlTag, IfBlock, KeyBlock, Node, RenderTag, SnippetBlock,
    SvelteDocument, SvelteElement, SvelteWindow, TransitionDirective, UseDirective,
};

use crate::data::AnalysisData;

/// Trait for template analysis visitors.
///
/// Each analysis pass implements only the visit methods it cares about.
/// All methods have default no-op implementations. The `walk_template` function
/// handles structural recursion and scope tracking — visitors never need to
/// recurse manually.
///
/// Multiple visitors can be combined into a single tree traversal using tuple
/// composite visitors: `(VisitorA, VisitorB)` implements `TemplateVisitor` and
/// dispatches to both visitors at each node.
#[allow(unused_variables)]
pub(crate) trait TemplateVisitor {
    fn visit_expression_tag(&mut self, tag: &ExpressionTag, scope: ScopeId, data: &mut AnalysisData) {}
    fn visit_render_tag(&mut self, tag: &RenderTag, scope: ScopeId, data: &mut AnalysisData) {}
    fn visit_html_tag(&mut self, tag: &HtmlTag, scope: ScopeId, data: &mut AnalysisData) {}
    fn visit_const_tag(&mut self, tag: &ConstTag, scope: ScopeId, data: &mut AnalysisData) {}
    fn visit_element(&mut self, el: &Element, scope: ScopeId, data: &mut AnalysisData) {}
    fn visit_if_block(&mut self, block: &IfBlock, scope: ScopeId, data: &mut AnalysisData) {}
    fn visit_each_block(&mut self, block: &EachBlock, parent_scope: ScopeId, body_scope: ScopeId, data: &mut AnalysisData) {}
    fn visit_snippet_block(&mut self, block: &SnippetBlock, scope: ScopeId, data: &mut AnalysisData) {}
    fn visit_key_block(&mut self, block: &KeyBlock, scope: ScopeId, data: &mut AnalysisData) {}
    fn visit_svelte_element(&mut self, el: &SvelteElement, scope: ScopeId, data: &mut AnalysisData) {}
    fn visit_svelte_window(&mut self, w: &SvelteWindow, scope: ScopeId, data: &mut AnalysisData) {}
    fn visit_svelte_document(&mut self, doc: &SvelteDocument, scope: ScopeId, data: &mut AnalysisData) {}
    fn visit_attribute(&mut self, attr: &Attribute, el: &Element, scope: ScopeId, data: &mut AnalysisData) {}
    fn visit_bind_directive(&mut self, dir: &BindDirective, el: &Element, scope: ScopeId, data: &mut AnalysisData) {}
    fn visit_use_directive(&mut self, dir: &UseDirective, el: &Element, scope: ScopeId, data: &mut AnalysisData) {}
    fn visit_transition_directive(&mut self, dir: &TransitionDirective, el: &Element, scope: ScopeId, data: &mut AnalysisData) {}
    fn visit_animate_directive(&mut self, dir: &AnimateDirective, el: &Element, scope: ScopeId, data: &mut AnalysisData) {}
    fn visit_attach_tag(&mut self, tag: &AttachTag, el: &Element, scope: ScopeId, data: &mut AnalysisData) {}
    fn visit_component_attribute(&mut self, attr: &Attribute, cn: &ComponentNode, scope: ScopeId, data: &mut AnalysisData) {}
    /// Called after element children have been walked. Use for bottom-up computations.
    fn leave_element(&mut self, el: &Element, scope: ScopeId, data: &mut AnalysisData) {}
    /// Called after snippet block body has been walked. Use with `visit_snippet_block` for stack-based context.
    fn leave_snippet_block(&mut self, block: &SnippetBlock, scope: ScopeId, data: &mut AnalysisData) {}
}

/// Walk a template fragment, dispatching to the visitor at each node.
///
/// Scope tracking is built-in: EachBlock bodies use their child scope,
/// EachBlock fallbacks use the parent scope.
pub(crate) fn walk_template<V: TemplateVisitor>(
    fragment: &Fragment,
    data: &mut AnalysisData,
    scope: ScopeId,
    visitor: &mut V,
) {
    for node in &fragment.nodes {
        match node {
            Node::ExpressionTag(tag) => {
                visitor.visit_expression_tag(tag, scope, data);
            }
            Node::Element(el) => {
                visitor.visit_element(el, scope, data);
                for attr in &el.attributes {
                    visitor.visit_attribute(attr, el, scope, data);
                    if let Attribute::BindDirective(dir) = attr {
                        visitor.visit_bind_directive(dir, el, scope, data);
                    }
                    if let Attribute::UseDirective(dir) = attr {
                        visitor.visit_use_directive(dir, el, scope, data);
                    }
                    if let Attribute::TransitionDirective(dir) = attr {
                        visitor.visit_transition_directive(dir, el, scope, data);
                    }
                    if let Attribute::AnimateDirective(dir) = attr {
                        visitor.visit_animate_directive(dir, el, scope, data);
                    }
                    if let Attribute::AttachTag(tag) = attr {
                        visitor.visit_attach_tag(tag, el, scope, data);
                    }
                }
                walk_template(&el.fragment, data, scope, visitor);
                visitor.leave_element(el, scope, data);
            }
            Node::IfBlock(block) => {
                visitor.visit_if_block(block, scope, data);
                walk_template(&block.consequent, data, scope, visitor);
                if let Some(alt) = &block.alternate {
                    walk_template(alt, data, scope, visitor);
                }
            }
            Node::EachBlock(block) => {
                let body_scope = data.scoping.node_scope(block.id).unwrap_or(scope);
                visitor.visit_each_block(block, scope, body_scope, data);
                walk_template(&block.body, data, body_scope, visitor);
                // Fallback uses parent scope (no context/index vars)
                if let Some(fb) = &block.fallback {
                    walk_template(fb, data, scope, visitor);
                }
            }
            Node::SnippetBlock(block) => {
                let body_scope = data.scoping.node_scope(block.id).unwrap_or(scope);
                visitor.visit_snippet_block(block, scope, data);
                walk_template(&block.body, data, body_scope, visitor);
                visitor.leave_snippet_block(block, scope, data);
            }
            Node::ComponentNode(cn) => {
                for attr in &cn.attributes {
                    visitor.visit_component_attribute(attr, cn, scope, data);
                }
                walk_template(&cn.fragment, data, scope, visitor);
            }
            Node::RenderTag(tag) => {
                visitor.visit_render_tag(tag, scope, data);
            }
            Node::HtmlTag(tag) => {
                visitor.visit_html_tag(tag, scope, data);
            }
            Node::ConstTag(tag) => {
                visitor.visit_const_tag(tag, scope, data);
            }
            Node::KeyBlock(block) => {
                visitor.visit_key_block(block, scope, data);
                walk_template(&block.fragment, data, scope, visitor);
            }
            Node::SvelteHead(head) => {
                walk_template(&head.fragment, data, scope, visitor);
            }
            Node::SvelteElement(el) => {
                visitor.visit_svelte_element(el, scope, data);
                walk_template(&el.fragment, data, scope, visitor);
            }
            Node::SvelteWindow(w) => {
                visitor.visit_svelte_window(w, scope, data);
            }
            Node::SvelteDocument(d) => {
                visitor.visit_svelte_document(d, scope, data);
            }
            Node::Text(_) | Node::Comment(_) | Node::Error(_) => {}
        }
    }
}

// ---------------------------------------------------------------------------
// Composite visitor — tuple impls
// ---------------------------------------------------------------------------

/// Delegates all `TemplateVisitor` methods to tuple elements `self.$idx`.
macro_rules! delegate_visitor_methods {
    ($($idx:tt),+) => {
        fn visit_expression_tag(&mut self, tag: &ExpressionTag, scope: ScopeId, data: &mut AnalysisData) {
            $(self.$idx.visit_expression_tag(tag, scope, data);)+
        }
        fn visit_render_tag(&mut self, tag: &RenderTag, scope: ScopeId, data: &mut AnalysisData) {
            $(self.$idx.visit_render_tag(tag, scope, data);)+
        }
        fn visit_html_tag(&mut self, tag: &HtmlTag, scope: ScopeId, data: &mut AnalysisData) {
            $(self.$idx.visit_html_tag(tag, scope, data);)+
        }
        fn visit_const_tag(&mut self, tag: &ConstTag, scope: ScopeId, data: &mut AnalysisData) {
            $(self.$idx.visit_const_tag(tag, scope, data);)+
        }
        fn visit_element(&mut self, el: &Element, scope: ScopeId, data: &mut AnalysisData) {
            $(self.$idx.visit_element(el, scope, data);)+
        }
        fn visit_if_block(&mut self, block: &IfBlock, scope: ScopeId, data: &mut AnalysisData) {
            $(self.$idx.visit_if_block(block, scope, data);)+
        }
        fn visit_each_block(&mut self, block: &EachBlock, parent_scope: ScopeId, body_scope: ScopeId, data: &mut AnalysisData) {
            $(self.$idx.visit_each_block(block, parent_scope, body_scope, data);)+
        }
        fn visit_snippet_block(&mut self, block: &SnippetBlock, scope: ScopeId, data: &mut AnalysisData) {
            $(self.$idx.visit_snippet_block(block, scope, data);)+
        }
        fn visit_key_block(&mut self, block: &KeyBlock, scope: ScopeId, data: &mut AnalysisData) {
            $(self.$idx.visit_key_block(block, scope, data);)+
        }
        fn visit_svelte_element(&mut self, el: &SvelteElement, scope: ScopeId, data: &mut AnalysisData) {
            $(self.$idx.visit_svelte_element(el, scope, data);)+
        }
        fn visit_svelte_window(&mut self, w: &SvelteWindow, scope: ScopeId, data: &mut AnalysisData) {
            $(self.$idx.visit_svelte_window(w, scope, data);)+
        }
        fn visit_svelte_document(&mut self, doc: &SvelteDocument, scope: ScopeId, data: &mut AnalysisData) {
            $(self.$idx.visit_svelte_document(doc, scope, data);)+
        }
        fn visit_attribute(&mut self, attr: &Attribute, el: &Element, scope: ScopeId, data: &mut AnalysisData) {
            $(self.$idx.visit_attribute(attr, el, scope, data);)+
        }
        fn visit_bind_directive(&mut self, dir: &BindDirective, el: &Element, scope: ScopeId, data: &mut AnalysisData) {
            $(self.$idx.visit_bind_directive(dir, el, scope, data);)+
        }
        fn visit_use_directive(&mut self, dir: &UseDirective, el: &Element, scope: ScopeId, data: &mut AnalysisData) {
            $(self.$idx.visit_use_directive(dir, el, scope, data);)+
        }
        fn visit_transition_directive(&mut self, dir: &TransitionDirective, el: &Element, scope: ScopeId, data: &mut AnalysisData) {
            $(self.$idx.visit_transition_directive(dir, el, scope, data);)+
        }
        fn visit_animate_directive(&mut self, dir: &AnimateDirective, el: &Element, scope: ScopeId, data: &mut AnalysisData) {
            $(self.$idx.visit_animate_directive(dir, el, scope, data);)+
        }
        fn visit_attach_tag(&mut self, tag: &AttachTag, el: &Element, scope: ScopeId, data: &mut AnalysisData) {
            $(self.$idx.visit_attach_tag(tag, el, scope, data);)+
        }
        fn visit_component_attribute(&mut self, attr: &Attribute, cn: &ComponentNode, scope: ScopeId, data: &mut AnalysisData) {
            $(self.$idx.visit_component_attribute(attr, cn, scope, data);)+
        }
        fn leave_element(&mut self, el: &Element, scope: ScopeId, data: &mut AnalysisData) {
            $(self.$idx.leave_element(el, scope, data);)+
        }
        fn leave_snippet_block(&mut self, block: &SnippetBlock, scope: ScopeId, data: &mut AnalysisData) {
            $(self.$idx.leave_snippet_block(block, scope, data);)+
        }
    };
}

macro_rules! impl_composite_visitor {
    ($($T:ident : $idx:tt),+) => {
        impl<$($T: TemplateVisitor),+> TemplateVisitor for ($($T,)+) {
            delegate_visitor_methods!($($idx),+);
        }
    };
}

impl_composite_visitor!(A: 0, B: 1);
impl_composite_visitor!(A: 0, B: 1, C: 2);
impl_composite_visitor!(A: 0, B: 1, C: 2, D: 3);
impl_composite_visitor!(A: 0, B: 1, C: 2, D: 3, E: 4);
