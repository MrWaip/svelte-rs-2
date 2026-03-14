use oxc_semantic::ScopeId;
use svelte_ast::{
    Attribute, BindDirective, ComponentNode, EachBlock, Element, ExpressionTag, Fragment, IfBlock,
    Node, RenderTag, SnippetBlock,
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
    fn visit_element(&mut self, el: &Element, scope: ScopeId, data: &mut AnalysisData) {}
    fn visit_if_block(&mut self, block: &IfBlock, scope: ScopeId, data: &mut AnalysisData) {}
    fn visit_each_block(&mut self, block: &EachBlock, body_scope: ScopeId, data: &mut AnalysisData) {}
    fn visit_snippet_block(&mut self, block: &SnippetBlock, scope: ScopeId, data: &mut AnalysisData) {}
    fn visit_attribute(&mut self, attr: &Attribute, idx: usize, el: &Element, scope: ScopeId, data: &mut AnalysisData) {}
    fn visit_bind_directive(&mut self, dir: &BindDirective, el: &Element, scope: ScopeId, data: &mut AnalysisData) {}
    fn visit_component_attribute(&mut self, attr: &Attribute, idx: usize, cn: &ComponentNode, scope: ScopeId, data: &mut AnalysisData) {}
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
                for (idx, attr) in el.attributes.iter().enumerate() {
                    visitor.visit_attribute(attr, idx, el, scope, data);
                    if let Attribute::BindDirective(dir) = attr {
                        visitor.visit_bind_directive(dir, el, scope, data);
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
                visitor.visit_each_block(block, body_scope, data);
                walk_template(&block.body, data, body_scope, visitor);
                // Fallback uses parent scope (no context/index vars)
                if let Some(fb) = &block.fallback {
                    walk_template(fb, data, scope, visitor);
                }
            }
            Node::SnippetBlock(block) => {
                visitor.visit_snippet_block(block, scope, data);
                walk_template(&block.body, data, scope, visitor);
                visitor.leave_snippet_block(block, scope, data);
            }
            Node::ComponentNode(cn) => {
                for (idx, attr) in cn.attributes.iter().enumerate() {
                    visitor.visit_component_attribute(attr, idx, cn, scope, data);
                }
                walk_template(&cn.fragment, data, scope, visitor);
            }
            Node::RenderTag(tag) => {
                visitor.visit_render_tag(tag, scope, data);
            }
            Node::Text(_) | Node::Comment(_) => {}
        }
    }
}

// ---------------------------------------------------------------------------
// Composite visitor — tuple impls
// ---------------------------------------------------------------------------

macro_rules! impl_composite_visitor {
    ($($T:ident : $idx:tt),+) => {
        impl<$($T: TemplateVisitor),+> TemplateVisitor for ($($T,)+) {
            fn visit_expression_tag(&mut self, tag: &ExpressionTag, scope: ScopeId, data: &mut AnalysisData) {
                $(self.$idx.visit_expression_tag(tag, scope, data);)+
            }
            fn visit_render_tag(&mut self, tag: &RenderTag, scope: ScopeId, data: &mut AnalysisData) {
                $(self.$idx.visit_render_tag(tag, scope, data);)+
            }
            fn visit_element(&mut self, el: &Element, scope: ScopeId, data: &mut AnalysisData) {
                $(self.$idx.visit_element(el, scope, data);)+
            }
            fn visit_if_block(&mut self, block: &IfBlock, scope: ScopeId, data: &mut AnalysisData) {
                $(self.$idx.visit_if_block(block, scope, data);)+
            }
            fn visit_each_block(&mut self, block: &EachBlock, body_scope: ScopeId, data: &mut AnalysisData) {
                $(self.$idx.visit_each_block(block, body_scope, data);)+
            }
            fn visit_snippet_block(&mut self, block: &SnippetBlock, scope: ScopeId, data: &mut AnalysisData) {
                $(self.$idx.visit_snippet_block(block, scope, data);)+
            }
            fn visit_attribute(&mut self, attr: &Attribute, idx: usize, el: &Element, scope: ScopeId, data: &mut AnalysisData) {
                $(self.$idx.visit_attribute(attr, idx, el, scope, data);)+
            }
            fn visit_bind_directive(&mut self, dir: &BindDirective, el: &Element, scope: ScopeId, data: &mut AnalysisData) {
                $(self.$idx.visit_bind_directive(dir, el, scope, data);)+
            }
            fn visit_component_attribute(&mut self, attr: &Attribute, idx: usize, cn: &ComponentNode, scope: ScopeId, data: &mut AnalysisData) {
                $(self.$idx.visit_component_attribute(attr, idx, cn, scope, data);)+
            }
            fn leave_element(&mut self, el: &Element, scope: ScopeId, data: &mut AnalysisData) {
                $(self.$idx.leave_element(el, scope, data);)+
            }
            fn leave_snippet_block(&mut self, block: &SnippetBlock, scope: ScopeId, data: &mut AnalysisData) {
                $(self.$idx.leave_snippet_block(block, scope, data);)+
            }
        }
    };
}

impl_composite_visitor!(A: 0, B: 1);
impl_composite_visitor!(A: 0, B: 1, C: 2);
impl_composite_visitor!(A: 0, B: 1, C: 2, D: 3);
