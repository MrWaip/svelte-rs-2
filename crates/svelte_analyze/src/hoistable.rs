//! HoistableSnippetsVisitor — detect which top-level snippets can be hoisted.
//!
//! A snippet is hoistable if its body doesn't reference any script-declared variables.

use oxc_semantic::SymbolId;
use rustc_hash::FxHashSet;
use svelte_ast::{
    AnimateDirective, AttachTag, BindDirective, ClassDirective, ComponentNode,
    ConcatenationAttribute, EachBlock, ExpressionAttribute, ExpressionTag, HtmlTag, IfBlock,
    NodeId, OnDirectiveLegacy, RenderTag, Shorthand, SnippetBlock, SpreadAttribute,
    StyleDirective, TransitionDirective, UseDirective,
};

use crate::data::AnalysisData;
use crate::walker::TemplateVisitor;

pub(crate) struct HoistableSnippetsVisitor {
    script_syms: FxHashSet<SymbolId>,
    top_level_ids: FxHashSet<NodeId>,
    /// Currently active top-level snippet (set between visit/leave_snippet_block)
    current_root: Option<NodeId>,
    tainted: FxHashSet<NodeId>,
}

impl HoistableSnippetsVisitor {
    pub(crate) fn new(
        script_syms: FxHashSet<SymbolId>,
        top_level_ids: FxHashSet<NodeId>,
    ) -> Self {
        Self {
            script_syms,
            top_level_ids,
            current_root: None,
            tainted: FxHashSet::default(),
        }
    }

    fn check_expr(&mut self, node_id: &NodeId, data: &AnalysisData) {
        let Some(root) = self.current_root else { return };
        if let Some(info) = data.expressions.get(*node_id) {
            if info
                .references
                .iter()
                .any(|r| r.symbol_id.is_some_and(|sym| self.script_syms.contains(&sym)))
            {
                self.tainted.insert(root);
            }
        }
    }

    fn check_attr_expr(&mut self, attr_id: NodeId, data: &AnalysisData) {
        let Some(root) = self.current_root else { return };
        if let Some(info) = data.attr_expressions.get(attr_id) {
            if info
                .references
                .iter()
                .any(|r| r.symbol_id.is_some_and(|sym| self.script_syms.contains(&sym)))
            {
                self.tainted.insert(root);
            }
        }
    }
}

impl TemplateVisitor for HoistableSnippetsVisitor {
    fn visit_snippet_block(
        &mut self,
        block: &SnippetBlock,
        _ctx: &mut crate::walker::VisitContext<'_>,
    ) {
        if self.top_level_ids.contains(&block.id) {
            self.current_root = Some(block.id);
        }
    }

    fn leave_snippet_block(
        &mut self,
        block: &SnippetBlock,
        ctx: &mut crate::walker::VisitContext<'_>,
    ) {
        if self.top_level_ids.contains(&block.id) {
            if !self.tainted.contains(&block.id) {
                ctx.data.snippets.hoistable.insert(block.id);
            }
            self.current_root = None;
        }
    }

    fn visit_expression_tag(
        &mut self,
        tag: &ExpressionTag,
        ctx: &mut crate::walker::VisitContext<'_>,
    ) {
        self.check_expr(&tag.id, ctx.data);
    }

    fn visit_render_tag(&mut self, tag: &RenderTag, ctx: &mut crate::walker::VisitContext<'_>) {
        self.check_expr(&tag.id, ctx.data);
    }

    fn visit_html_tag(&mut self, tag: &HtmlTag, ctx: &mut crate::walker::VisitContext<'_>) {
        self.check_expr(&tag.id, ctx.data);
    }

    fn visit_if_block(&mut self, block: &IfBlock, ctx: &mut crate::walker::VisitContext<'_>) {
        self.check_expr(&block.id, ctx.data);
    }

    fn visit_each_block(
        &mut self,
        block: &EachBlock,
        ctx: &mut crate::walker::VisitContext<'_>,
    ) {
        self.check_expr(&block.id, ctx.data);
    }

    fn visit_expression_attribute(&mut self, attr: &ExpressionAttribute, ctx: &mut crate::walker::VisitContext<'_>) {
        self.check_attr_expr(attr.id, ctx.data);
    }
    fn visit_concatenation_attribute(&mut self, attr: &ConcatenationAttribute, ctx: &mut crate::walker::VisitContext<'_>) {
        self.check_attr_expr(attr.id, ctx.data);
    }
    fn visit_spread_attribute(&mut self, attr: &SpreadAttribute, ctx: &mut crate::walker::VisitContext<'_>) {
        self.check_attr_expr(attr.id, ctx.data);
    }
    fn visit_shorthand(&mut self, attr: &Shorthand, ctx: &mut crate::walker::VisitContext<'_>) {
        self.check_attr_expr(attr.id, ctx.data);
    }
    fn visit_class_directive(&mut self, dir: &ClassDirective, ctx: &mut crate::walker::VisitContext<'_>) {
        self.check_attr_expr(dir.id, ctx.data);
    }
    fn visit_style_directive(&mut self, dir: &StyleDirective, ctx: &mut crate::walker::VisitContext<'_>) {
        self.check_attr_expr(dir.id, ctx.data);
    }
    fn visit_bind_directive(&mut self, dir: &BindDirective, ctx: &mut crate::walker::VisitContext<'_>) {
        self.check_attr_expr(dir.id, ctx.data);
    }
    fn visit_use_directive(&mut self, dir: &UseDirective, ctx: &mut crate::walker::VisitContext<'_>) {
        self.check_attr_expr(dir.id, ctx.data);
    }
    fn visit_on_directive_legacy(&mut self, dir: &OnDirectiveLegacy, ctx: &mut crate::walker::VisitContext<'_>) {
        self.check_attr_expr(dir.id, ctx.data);
    }
    fn visit_transition_directive(&mut self, dir: &TransitionDirective, ctx: &mut crate::walker::VisitContext<'_>) {
        self.check_attr_expr(dir.id, ctx.data);
    }
    fn visit_animate_directive(&mut self, dir: &AnimateDirective, ctx: &mut crate::walker::VisitContext<'_>) {
        self.check_attr_expr(dir.id, ctx.data);
    }
    fn visit_attach_tag(&mut self, tag: &AttachTag, ctx: &mut crate::walker::VisitContext<'_>) {
        self.check_attr_expr(tag.id, ctx.data);
    }

    fn visit_component_node(&mut self, cn: &ComponentNode, ctx: &mut crate::walker::VisitContext<'_>) {
        for attr in &cn.attributes {
            self.check_attr_expr(attr.id(), ctx.data);
        }
    }
}
