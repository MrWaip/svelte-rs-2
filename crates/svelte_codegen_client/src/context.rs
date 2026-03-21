use rustc_hash::FxHashMap;

use oxc_ast::ast::Statement;
use svelte_analyze::{AnalysisData, ContentStrategy, FragmentKey, IdentGen, LoweredFragment, ParsedExprs};
use svelte_ast::{AwaitBlock, Component, ComponentNode, DebugTag, EachBlock, Element, Fragment, IfBlock, KeyBlock, Node, NodeId, RenderTag, SnippetBlock, SvelteBody, SvelteBoundary, SvelteDocument, SvelteElement, SvelteWindow};
use svelte_js::ExpressionInfo;
use svelte_transform::TransformData;

use crate::builder::Builder;

/// Pre-built index for O(1) node lookup by NodeId.
struct NodeIndex<'a> {
    nodes: FxHashMap<NodeId, &'a Node>,
}

impl<'a> NodeIndex<'a> {
    fn build(fragment: &'a Fragment) -> Self {
        let mut index = Self {
            nodes: FxHashMap::default(),
        };
        index.walk(fragment);
        index
    }

    fn walk(&mut self, fragment: &'a Fragment) {
        for node in &fragment.nodes {
            match node {
                Node::Text(_) | Node::Comment(_) | Node::Error(_) => {}
                _ => { self.nodes.insert(node.node_id(), node); }
            }
            match node {
                Node::Element(el) => self.walk(&el.fragment),
                Node::ComponentNode(cn) => self.walk(&cn.fragment),
                Node::IfBlock(b) => {
                    self.walk(&b.consequent);
                    if let Some(alt) = &b.alternate { self.walk(alt); }
                }
                Node::EachBlock(b) => {
                    self.walk(&b.body);
                    if let Some(fb) = &b.fallback { self.walk(fb); }
                }
                Node::SnippetBlock(b) => self.walk(&b.body),
                Node::KeyBlock(b) => self.walk(&b.fragment),
                Node::SvelteHead(h) => self.walk(&h.fragment),
                Node::SvelteElement(el) => self.walk(&el.fragment),
                Node::SvelteBoundary(b) => self.walk(&b.fragment),
                Node::AwaitBlock(b) => {
                    if let Some(ref p) = b.pending { self.walk(p); }
                    if let Some(ref t) = b.then { self.walk(t); }
                    if let Some(ref c) = b.catch { self.walk(c); }
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
    pub name: &'a str,
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
    /// Generated $$index names for each blocks with contains_group_binding
    /// (populated during each_block codegen, consumed by bind_group codegen).
    pub group_index_names: FxHashMap<NodeId, String>,
    /// Set while processing children of a contenteditable element with bind:innerHTML/innerText/textContent.
    /// Text nodes use `nodeValue=` init instead of `$.set_text()` update.
    pub bound_contenteditable: bool,
    /// Snippet param names for the currently generating snippet body.
    pub snippet_param_names: Vec<String>,

    /// Whether dev-mode features ($inspect, ownership checks, etc.) are enabled.
    pub dev: bool,

    /// Event names that use delegation (e.g., "click" from `onclick={handler}`).
    pub delegated_events: Vec<String>,

    /// Original component source text (for re-parsing expression spans in codegen).
    pub source: &'a str,
    /// Filename from CompileOptions (used in trace labels and $.FILENAME).
    pub filename: &'a str,
    /// Whether any $inspect.trace() was found in template expressions (triggers tracing import).
    pub has_tracing: bool,
}

impl<'a> Ctx<'a> {
    pub fn new(
        allocator: &'a oxc_allocator::Allocator,
        component: &'a Component,
        analysis: &'a AnalysisData,
        parsed: &'a mut ParsedExprs<'a>,
        ident_gen: &'a mut IdentGen,
        transform_data: TransformData,
        name: &str,
        dev: bool,
        source: &'a str,
        filename: &str,
    ) -> Self {
        let index = NodeIndex::build(&component.fragment);
        let name = allocator.alloc_str(name);
        let filename = allocator.alloc_str(filename);

        Self {
            b: Builder::new(allocator),
            component,
            analysis,
            name,
            transform_data,
            parsed,
            ident_gen,
            module_hoisted: Vec::new(),
            index,
            dev,
            needs_binding_group: false,
            group_index_names: FxHashMap::default(),
            bound_contenteditable: false,
            snippet_param_names: Vec::new(),
            delegated_events: Vec::new(),
            source,
            filename,
            has_tracing: false,
        }
    }

    // -- Node lookups (O(1)) --

    pub fn element(&self, id: NodeId) -> &'a Element { self.get_node(id, "element", Node::as_element) }
    pub fn component_node(&self, id: NodeId) -> &'a ComponentNode { self.get_node(id, "component node", Node::as_component_node) }
    pub fn if_block(&self, id: NodeId) -> &'a IfBlock { self.get_node(id, "if block", Node::as_if_block) }
    pub fn each_block(&self, id: NodeId) -> &'a EachBlock { self.get_node(id, "each block", Node::as_each_block) }
    pub fn snippet_block(&self, id: NodeId) -> &'a SnippetBlock { self.get_node(id, "snippet block", Node::as_snippet_block) }
    pub fn render_tag(&self, id: NodeId) -> &'a RenderTag { self.get_node(id, "render tag", Node::as_render_tag) }
    pub fn key_block(&self, id: NodeId) -> &'a KeyBlock { self.get_node(id, "key block", Node::as_key_block) }
    pub fn svelte_element(&self, id: NodeId) -> &'a SvelteElement { self.get_node(id, "svelte element", Node::as_svelte_element) }
    pub fn svelte_boundary(&self, id: NodeId) -> &'a SvelteBoundary { self.get_node(id, "svelte boundary", Node::as_svelte_boundary) }
    pub fn await_block(&self, id: NodeId) -> &'a AwaitBlock { self.get_node(id, "await block", Node::as_await_block) }
    pub fn svelte_window(&self, id: NodeId) -> &'a SvelteWindow { self.get_node(id, "svelte window", Node::as_svelte_window) }
    pub fn svelte_document(&self, id: NodeId) -> &'a SvelteDocument { self.get_node(id, "svelte document", Node::as_svelte_document) }
    pub fn svelte_body(&self, id: NodeId) -> &'a SvelteBody { self.get_node(id, "svelte body", Node::as_svelte_body) }
    fn get_node<T>(&self, id: NodeId, label: &str, extract: fn(&Node) -> Option<&T>) -> &'a T {
        let node = self.index.nodes.get(&id)
            .unwrap_or_else(|| panic!("{} {:?} not found in index", label, id));
        extract(node)
            .unwrap_or_else(|| panic!("{} {:?} is wrong node type", label, id))
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
    pub fn is_mutable_rune_target(&self, id: NodeId) -> bool {
        self.analysis.bind_semantics.is_mutable_rune_target(id)
    }
    pub fn is_prop_source_node(&self, id: NodeId) -> bool {
        self.analysis.bind_semantics.is_prop_source(id)
    }
    pub fn bind_each_context(&self, id: NodeId) -> Option<&Vec<String>> {
        self.analysis.bind_semantics.each_context(id)
    }
    pub fn each_context_name(&self, id: NodeId) -> String {
        self.analysis.each_blocks.context_name(id)
            .unwrap_or_else(|| panic!("missing context_name for each block {id:?}"))
            .to_string()
    }
    pub fn each_index_name(&self, id: NodeId) -> Option<String> {
        self.analysis.each_blocks.index_name(id).map(|s| s.to_string())
    }
    pub fn await_value_binding(&self, id: NodeId) -> Option<&svelte_js::AwaitBindingInfo> {
        self.analysis.await_bindings.value(id)
    }
    pub fn await_error_binding(&self, id: NodeId) -> Option<&svelte_js::AwaitBindingInfo> {
        self.analysis.await_bindings.error(id)
    }
    pub fn is_import_sym(&self, sym_id: oxc_semantic::SymbolId) -> bool {
        self.analysis.import_syms.contains(&sym_id)
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
    pub fn is_bound_contenteditable(&self, id: NodeId) -> bool { self.analysis.element_flags.is_bound_contenteditable(id) }
    pub fn has_use_directive(&self, id: NodeId) -> bool { self.analysis.element_flags.has_use_directive(id) }
    pub fn has_dynamic_class_directives(&self, id: NodeId) -> bool { self.analysis.element_flags.has_dynamic_class_directives(id) }
    pub fn is_expression_shorthand(&self, id: NodeId) -> bool { self.analysis.element_flags.is_expression_shorthand(id) }
    pub fn has_bind_group(&self, id: NodeId) -> bool { self.analysis.bind_semantics.has_bind_group(id) }
    pub fn bind_group_value_attr(&self, id: NodeId) -> Option<NodeId> { self.analysis.bind_semantics.bind_group_value_attr(id) }
    pub fn parent_each_blocks(&self, id: NodeId) -> Option<&Vec<NodeId>> { self.analysis.bind_semantics.parent_each_blocks(id) }
    pub fn contains_group_binding(&self, id: NodeId) -> bool { self.analysis.bind_semantics.contains_group_binding(id) }

    // -- Snippet shortcuts --

    pub fn is_snippet_hoistable(&self, id: NodeId) -> bool { self.analysis.snippets.is_hoistable(id) }

    // -- ConstTag shortcuts --

    pub fn const_tag_names(&self, id: NodeId) -> Option<&Vec<String>> { self.analysis.const_tags.names(id) }
    pub fn const_tags_for_fragment(&self, key: &FragmentKey) -> Option<&Vec<NodeId>> { self.analysis.const_tags.by_fragment(key) }

    // -- DebugTag shortcuts --

    pub fn debug_tag(&self, id: NodeId) -> &'a DebugTag { self.get_node(id, "debug tag", Node::as_debug_tag) }
    pub fn debug_tags_for_fragment(&self, key: &FragmentKey) -> Option<&Vec<NodeId>> { self.analysis.debug_tags.by_fragment(key) }

    // -- EachBlock shortcuts --

    pub fn each_key_uses_index(&self, id: NodeId) -> bool { self.analysis.each_blocks.key_uses_index(id) }
    pub fn each_is_destructured(&self, id: NodeId) -> bool { self.analysis.each_blocks.is_destructured(id) }
    pub fn each_body_uses_index(&self, id: NodeId) -> bool { self.analysis.each_blocks.body_uses_index(id) }
    pub fn each_key_is_item(&self, id: NodeId) -> bool { self.analysis.each_blocks.key_is_item(id) }

}
