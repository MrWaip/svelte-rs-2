use std::collections::{HashMap, HashSet};

use oxc_ast::ast::Statement;
use svelte_analyze::AnalysisData;
use svelte_ast::{Component, EachBlock, Element, Fragment, IfBlock, Node, NodeId};
use svelte_span::Span;

use crate::builder::Builder;

/// Central codegen context. Holds refs to allocator, builder, component, analysis,
/// and mutable state (ident counter, node index, rune caches).
pub struct Ctx<'a> {
    pub b: Builder<'a>,
    pub component: &'a Component,
    pub analysis: &'a AnalysisData,
    /// Monotonically incrementing counter per name prefix.
    ident_counters: HashMap<String, u32>,
    /// Template declarations from nested fragments, hoisted to module scope.
    pub module_hoisted: Vec<Statement<'a>>,

    // -- Node index (O(1) lookup by NodeId) --
    elements: HashMap<NodeId, &'a Element>,
    if_blocks: HashMap<NodeId, &'a IfBlock>,
    each_blocks: HashMap<NodeId, &'a EachBlock>,
    expr_spans: HashMap<NodeId, Span>,

    // -- Cached rune metadata --
    pub rune_names: HashSet<String>,
}

impl<'a> Ctx<'a> {
    pub fn new(
        allocator: &'a oxc_allocator::Allocator,
        component: &'a Component,
        analysis: &'a AnalysisData,
    ) -> Self {
        let mut elements = HashMap::new();
        let mut if_blocks = HashMap::new();
        let mut each_blocks = HashMap::new();
        let mut expr_spans = HashMap::new();
        build_node_index(
            &component.fragment,
            &mut elements,
            &mut if_blocks,
            &mut each_blocks,
            &mut expr_spans,
        );

        let rune_names: HashSet<String> = analysis
            .symbol_by_name
            .iter()
            .filter_map(|(name, sid)| analysis.runes.contains_key(sid).then(|| name.clone()))
            .collect();

        Self {
            b: Builder::new(allocator),
            component,
            analysis,
            ident_counters: HashMap::new(),
            module_hoisted: Vec::new(),
            elements,
            if_blocks,
            each_blocks,
            expr_spans,
            rune_names,
        }
    }

    // -- Node lookups (O(1)) --

    pub fn element(&self, id: NodeId) -> &'a Element {
        self.elements.get(&id).copied().expect("element not found")
    }

    pub fn if_block(&self, id: NodeId) -> &'a IfBlock {
        self.if_blocks.get(&id).copied().expect("if block not found")
    }

    pub fn each_block(&self, id: NodeId) -> &'a EachBlock {
        self.each_blocks.get(&id).copied().expect("each block not found")
    }

    pub fn expr_span(&self, id: NodeId) -> Span {
        self.expr_spans.get(&id).copied().expect("expr tag not found")
    }

    // -- Identifiers --

    /// Generate a unique identifier like `root`, `root_1`, `root_2`, …
    pub fn gen_ident(&mut self, prefix: &str) -> String {
        let count = self.ident_counters.entry(prefix.to_string()).or_insert(0);
        let name = if *count == 0 {
            prefix.to_string()
        } else {
            format!("{}_{}", prefix, count)
        };
        *count += 1;
        name
    }

}


// ---------------------------------------------------------------------------
// Index builder — walks AST once, populates HashMaps
// ---------------------------------------------------------------------------

fn build_node_index<'a>(
    fragment: &'a Fragment,
    elements: &mut HashMap<NodeId, &'a Element>,
    if_blocks: &mut HashMap<NodeId, &'a IfBlock>,
    each_blocks: &mut HashMap<NodeId, &'a EachBlock>,
    expr_spans: &mut HashMap<NodeId, Span>,
) {
    for node in &fragment.nodes {
        match node {
            Node::Element(el) => {
                elements.insert(el.id, el);
                build_node_index(&el.fragment, elements, if_blocks, each_blocks, expr_spans);
            }
            Node::IfBlock(b) => {
                if_blocks.insert(b.id, b);
                build_node_index(&b.consequent, elements, if_blocks, each_blocks, expr_spans);
                if let Some(alt) = &b.alternate {
                    build_node_index(alt, elements, if_blocks, each_blocks, expr_spans);
                }
            }
            Node::EachBlock(b) => {
                each_blocks.insert(b.id, b);
                build_node_index(&b.body, elements, if_blocks, each_blocks, expr_spans);
                if let Some(fb) = &b.fallback {
                    build_node_index(fb, elements, if_blocks, each_blocks, expr_spans);
                }
            }
            Node::ExpressionTag(t) => {
                expr_spans.insert(t.id, t.expression_span);
            }
            _ => {}
        }
    }
}
