use rustc_hash::FxHashMap;

use oxc_ast::ast::Statement;
use svelte_analyze::{AnalysisData, ContentStrategy, FragmentKey, IdentGen, LoweredFragment, ParsedExprs};
use svelte_ast::{Component, ComponentNode, ConstTag, EachBlock, Element, Fragment, IfBlock, Node, NodeId, RenderTag, SnippetBlock, SvelteDocument, SvelteElement, SvelteWindow};
use svelte_js::ExpressionInfo;
use svelte_span::Span;
use svelte_transform::TransformData;

use crate::builder::Builder;

/// Pre-built index for O(1) node lookup by NodeId.
struct NodeIndex<'a> {
    elements: FxHashMap<NodeId, &'a Element>,
    component_nodes: FxHashMap<NodeId, &'a ComponentNode>,
    if_blocks: FxHashMap<NodeId, &'a IfBlock>,
    each_blocks: FxHashMap<NodeId, &'a EachBlock>,
    snippet_blocks: FxHashMap<NodeId, &'a SnippetBlock>,
    render_tags: FxHashMap<NodeId, &'a RenderTag>,
    const_tags: FxHashMap<NodeId, &'a ConstTag>,
    svelte_elements: FxHashMap<NodeId, &'a SvelteElement>,
    svelte_windows: FxHashMap<NodeId, &'a SvelteWindow>,
    svelte_documents: FxHashMap<NodeId, &'a SvelteDocument>,
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
            const_tags: FxHashMap::default(),
            svelte_elements: FxHashMap::default(),
            svelte_windows: FxHashMap::default(),
            svelte_documents: FxHashMap::default(),
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
                Node::HtmlTag(_) => {}
                Node::ConstTag(t) => {
                    self.const_tags.insert(t.id, t);
                }
                Node::KeyBlock(b) => {
                    self.walk(&b.fragment);
                }
                Node::SvelteHead(h) => {
                    self.walk(&h.fragment);
                }
                Node::SvelteElement(el) => {
                    self.svelte_elements.insert(el.id, el);
                    self.walk(&el.fragment);
                }
                Node::SvelteWindow(w) => {
                    self.svelte_windows.insert(w.id, w);
                }
                Node::SvelteDocument(d) => {
                    self.svelte_documents.insert(d.id, d);
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
    /// Data produced by the transform phase (e.g. tmp names for destructured const tags).
    pub transform_data: TransformData,
    /// Pre-parsed and pre-transformed expression ASTs (mutable for ownership transfer via remove).
    pub parsed: &'a mut ParsedExprs<'a>,
    /// Shared unique identifier generator.
    ident_gen: &'a mut IdentGen,
    /// Template declarations from nested fragments, hoisted to module scope.
    pub module_hoisted: Vec<Statement<'a>>,

    // -- Node index (O(1) lookup by NodeId) --
    index: NodeIndex<'a>,

    // -- Bind group --
    pub needs_binding_group: bool,
    /// Snippet param names for the currently generating snippet body.
    pub snippet_param_names: Vec<String>,

    /// Event names that use delegation (e.g., "click" from `onclick={handler}`).
    pub delegated_events: Vec<String>,
}

impl<'a> Ctx<'a> {
    pub fn new(
        allocator: &'a oxc_allocator::Allocator,
        component: &'a Component,
        analysis: &'a AnalysisData,
        parsed: &'a mut ParsedExprs<'a>,
        ident_gen: &'a mut IdentGen,
        transform_data: TransformData,
    ) -> Self {
        let index = NodeIndex::build(&component.fragment);

        Self {
            b: Builder::new(allocator),
            component,
            analysis,
            transform_data,
            parsed,
            ident_gen,
            module_hoisted: Vec::new(),
            index,
            needs_binding_group: false,
            snippet_param_names: Vec::new(),
            delegated_events: Vec::new(),
        }
    }

    // -- Node lookups (O(1)) --

    pub fn element(&self, id: NodeId) -> &'a Element { self.get_node(&self.index.elements, id, "element") }
    pub fn component_node(&self, id: NodeId) -> &'a ComponentNode { self.get_node(&self.index.component_nodes, id, "component node") }
    pub fn if_block(&self, id: NodeId) -> &'a IfBlock { self.get_node(&self.index.if_blocks, id, "if block") }
    pub fn each_block(&self, id: NodeId) -> &'a EachBlock { self.get_node(&self.index.each_blocks, id, "each block") }
    pub fn snippet_block(&self, id: NodeId) -> &'a SnippetBlock { self.get_node(&self.index.snippet_blocks, id, "snippet block") }
    pub fn render_tag(&self, id: NodeId) -> &'a RenderTag { self.get_node(&self.index.render_tags, id, "render tag") }
    pub fn svelte_element(&self, id: NodeId) -> &'a SvelteElement { self.get_node(&self.index.svelte_elements, id, "svelte element") }
    pub fn svelte_window(&self, id: NodeId) -> &'a SvelteWindow { self.get_node(&self.index.svelte_windows, id, "svelte window") }
    pub fn svelte_document(&self, id: NodeId) -> &'a SvelteDocument { self.get_node(&self.index.svelte_documents, id, "svelte document") }
    fn get_node<T>(&self, map: &FxHashMap<NodeId, &'a T>, id: NodeId, label: &str) -> &'a T {
        map.get(&id).copied()
            .unwrap_or_else(|| panic!("{} {:?} not found in index", label, id))
    }

    pub fn lowered_fragment(&self, key: &FragmentKey) -> &LoweredFragment {
        self.analysis.fragments.lowered(key)
            .unwrap_or_else(|| panic!("lowered fragment {:?} not found", key))
    }

    // -- Identifiers --

    /// Generate a unique identifier like `root`, `root_1`, `root_2`, …
    pub fn gen_ident(&mut self, prefix: &str) -> String {
        self.ident_gen.gen(prefix)
    }

    // -- Analysis shortcuts --

    pub fn is_dynamic(&self, id: NodeId) -> bool { self.analysis.is_dynamic(id) }
    pub fn is_elseif_alt(&self, id: NodeId) -> bool { self.analysis.is_elseif_alt(id) }
    pub fn is_mutable_rune(&self, name: &str) -> bool {
        let root = self.analysis.scoping.root_scope_id();
        self.analysis.scoping.find_binding(root, name)
            .is_some_and(|sym| self.analysis.scoping.is_rune(sym) && self.analysis.scoping.is_mutated(sym))
    }
    pub fn expression(&self, id: NodeId) -> Option<&ExpressionInfo> { self.analysis.expression(id) }
    pub fn known_value(&self, name: &str) -> Option<&str> { self.analysis.known_value(name) }

    // -- Fragment shortcuts --

    pub fn content_type(&self, key: &FragmentKey) -> ContentStrategy { self.analysis.fragments.content_type(key) }
    pub fn has_dynamic_children(&self, key: &FragmentKey) -> bool { self.analysis.fragments.has_dynamic_children(key) }

    // -- Element flag shortcuts --

    pub fn has_spread(&self, id: NodeId) -> bool { self.analysis.element_flags.has_spread(id) }
    pub fn has_class_directives(&self, id: NodeId) -> bool { self.analysis.element_flags.has_class_directives(id) }
    pub fn has_class_attribute(&self, id: NodeId) -> bool { self.analysis.element_flags.has_class_attribute(id) }
    pub fn needs_clsx(&self, id: NodeId) -> bool { self.analysis.element_flags.needs_clsx(id) }
    pub fn has_style_directives(&self, id: NodeId) -> bool { self.analysis.element_flags.has_style_directives(id) }
    pub fn needs_input_defaults(&self, id: NodeId) -> bool { self.analysis.element_flags.needs_input_defaults(id) }
    pub fn needs_var(&self, id: NodeId) -> bool { self.analysis.element_flags.needs_var(id) }
    pub fn is_dynamic_attr(&self, id: NodeId) -> bool { self.analysis.element_flags.is_dynamic_attr(id) }
    pub fn static_class(&self, id: NodeId) -> Option<&str> { self.analysis.element_flags.static_class(id) }
    pub fn static_style(&self, id: NodeId) -> Option<&str> { self.analysis.element_flags.static_style(id) }

    // -- Snippet shortcuts --

    pub fn is_snippet_hoistable(&self, id: NodeId) -> bool { self.analysis.snippets.is_hoistable(id) }

    // -- ConstTag shortcuts --

    pub fn const_tag_names(&self, id: NodeId) -> Option<&Vec<String>> { self.analysis.const_tags.names(id) }
    pub fn const_tags_for_fragment(&self, key: &FragmentKey) -> Option<&Vec<NodeId>> { self.analysis.const_tags.by_fragment(key) }

}
