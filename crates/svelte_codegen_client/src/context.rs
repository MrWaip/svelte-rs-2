use rustc_hash::{FxHashMap, FxHashSet};

use oxc_ast::ast::Statement;
use svelte_analyze::AnalysisData;
use svelte_ast::{Component, ComponentNode, EachBlock, Element, Fragment, HtmlTag, IfBlock, Node, NodeId, RenderTag, SnippetBlock};
use svelte_span::Span;

use crate::builder::Builder;

/// Pre-built index for O(1) node lookup by NodeId.
struct NodeIndex<'a> {
    elements: FxHashMap<NodeId, &'a Element>,
    component_nodes: FxHashMap<NodeId, &'a ComponentNode>,
    if_blocks: FxHashMap<NodeId, &'a IfBlock>,
    each_blocks: FxHashMap<NodeId, &'a EachBlock>,
    snippet_blocks: FxHashMap<NodeId, &'a SnippetBlock>,
    render_tags: FxHashMap<NodeId, &'a RenderTag>,
    html_tags: FxHashMap<NodeId, &'a HtmlTag>,
    expr_spans: FxHashMap<NodeId, Span>,
}

impl<'a> NodeIndex<'a> {
    fn build(fragment: &'a Fragment) -> Self {
        let mut index = Self {
            elements: FxHashMap::default(),
            component_nodes: FxHashMap::default(),
            if_blocks: FxHashMap::default(),
            each_blocks: FxHashMap::default(),
            snippet_blocks: FxHashMap::default(),
            render_tags: FxHashMap::default(),
            html_tags: FxHashMap::default(),
            expr_spans: FxHashMap::default(),
        };
        index.walk(fragment);
        index
    }

    fn walk(&mut self, fragment: &'a Fragment) {
        for node in &fragment.nodes {
            match node {
                Node::Element(el) => {
                    self.elements.insert(el.id, el);
                    self.walk(&el.fragment);
                }
                Node::ComponentNode(cn) => {
                    self.component_nodes.insert(cn.id, cn);
                    self.walk(&cn.fragment);
                }
                Node::IfBlock(b) => {
                    self.if_blocks.insert(b.id, b);
                    self.walk(&b.consequent);
                    if let Some(alt) = &b.alternate {
                        self.walk(alt);
                    }
                }
                Node::EachBlock(b) => {
                    self.each_blocks.insert(b.id, b);
                    self.walk(&b.body);
                    if let Some(fb) = &b.fallback {
                        self.walk(fb);
                    }
                }
                Node::SnippetBlock(b) => {
                    self.snippet_blocks.insert(b.id, b);
                    self.walk(&b.body);
                }
                Node::RenderTag(t) => {
                    self.render_tags.insert(t.id, t);
                }
                Node::HtmlTag(t) => {
                    self.html_tags.insert(t.id, t);
                }
                Node::ExpressionTag(t) => {
                    self.expr_spans.insert(t.id, t.expression_span);
                }
                _ => {}
            }
        }
    }
}

/// Central codegen context. Holds refs to allocator, builder, component, analysis,
/// and mutable state (ident counter, node index).
pub struct Ctx<'a> {
    pub b: Builder<'a>,
    pub component: &'a Component,
    pub analysis: &'a AnalysisData,
    /// Monotonically incrementing counter per name prefix.
    ident_counters: FxHashMap<String, u32>,
    /// Template declarations from nested fragments, hoisted to module scope.
    pub module_hoisted: Vec<Statement<'a>>,

    // -- Node index (O(1) lookup by NodeId) --
    index: NodeIndex<'a>,

    // -- Bind group --
    pub needs_binding_group: bool,
    /// Snippet param names for the currently generating snippet body.
    pub snippet_param_names: Vec<String>,

    // -- Cached prop info (computed once from analysis) --
    /// Prop names that need `$.prop()` source wrappers (called as thunks).
    pub prop_sources: FxHashSet<String>,
    /// Non-source prop names → their original prop_name (accessed as `$$props.name`).
    pub prop_non_sources: FxHashMap<String, String>,
    /// Event names that use delegation (e.g., "click" from `onclick={handler}`).
    pub delegated_events: Vec<String>,
    /// Names of each-block context variables in scope (for `$.get()` wrapping).
    pub each_vars: FxHashSet<String>,
}

impl<'a> Ctx<'a> {
    pub fn new(
        allocator: &'a oxc_allocator::Allocator,
        component: &'a Component,
        analysis: &'a AnalysisData,
    ) -> Self {
        let index = NodeIndex::build(&component.fragment);

        let mut prop_sources = FxHashSet::default();
        let mut prop_non_sources = FxHashMap::default();
        if let Some(pa) = &analysis.props {
            for p in &pa.props {
                if p.is_rest {
                    // Rest props are accessed directly (from $.rest_props result)
                    continue;
                }
                if p.is_prop_source {
                    prop_sources.insert(p.local_name.clone());
                } else {
                    prop_non_sources.insert(p.local_name.clone(), p.prop_name.clone());
                }
            }
        }

        Self {
            b: Builder::new(allocator),
            component,
            analysis,
            ident_counters: FxHashMap::default(),
            module_hoisted: Vec::new(),
            index,
            needs_binding_group: false,
            snippet_param_names: Vec::new(),
            prop_sources,
            prop_non_sources,
            delegated_events: Vec::new(),
            each_vars: FxHashSet::default(),
        }
    }

    // -- Node lookups (O(1)) --

    pub fn element(&self, id: NodeId) -> &'a Element {
        self.index.elements.get(&id).copied().expect("element not found")
    }

    pub fn component_node(&self, id: NodeId) -> &'a ComponentNode {
        self.index.component_nodes.get(&id).copied().expect("component node not found")
    }

    pub fn if_block(&self, id: NodeId) -> &'a IfBlock {
        self.index.if_blocks.get(&id).copied().expect("if block not found")
    }

    pub fn each_block(&self, id: NodeId) -> &'a EachBlock {
        self.index.each_blocks.get(&id).copied().expect("each block not found")
    }

    pub fn snippet_block(&self, id: NodeId) -> &'a SnippetBlock {
        self.index.snippet_blocks.get(&id).copied().expect("snippet block not found")
    }

    pub fn render_tag(&self, id: NodeId) -> &'a RenderTag {
        self.index.render_tags.get(&id).copied().expect("render tag not found")
    }

    pub fn html_tag(&self, id: NodeId) -> &'a HtmlTag {
        self.index.html_tags.get(&id).copied().expect("html tag not found")
    }

    pub fn expr_span(&self, id: NodeId) -> Span {
        self.index.expr_spans.get(&id).copied().expect("expr tag not found")
    }

    // -- Identifiers --

    /// Generate a unique identifier like `root`, `root_1`, `root_2`, …
    pub fn gen_ident(&mut self, prefix: &str) -> String {
        // Avoid allocating a String key on every lookup (cache hit path is allocation-free)
        let count = if let Some(c) = self.ident_counters.get_mut(prefix) {
            let n = *c;
            *c += 1;
            n
        } else {
            self.ident_counters.insert(prefix.to_string(), 1);
            0
        };
        if count == 0 {
            prefix.to_string()
        } else {
            format!("{}_{}", prefix, count)
        }
    }

}
