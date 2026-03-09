use std::collections::HashMap;

use oxc_ast::ast::{Expression, Statement};
use svelte_analyze::AnalysisData;
use svelte_ast::{Component, NodeId};

use crate::builder::Builder;

/// Accumulated output for one "fragment owner" (root, element child, if branch, each body).
pub struct FragmentCtx<'a> {
    /// HTML string pieces for the `$.template(...)` call.
    pub template: Vec<String>,
    /// DOM traversal + one-time init statements.
    pub init: Vec<Statement<'a>>,
    /// Statements that go inside `$.template_effect(...)`.
    pub update: Vec<Statement<'a>>,
    /// Statements that run after the template_effect (e.g. bind directives).
    pub after_update: Vec<Statement<'a>>,
}

impl<'a> FragmentCtx<'a> {
    pub fn new() -> Self {
        Self {
            template: Vec::new(),
            init: Vec::new(),
            update: Vec::new(),
            after_update: Vec::new(),
        }
    }

    pub fn template_html(&self) -> String {
        self.template.concat()
    }
}

/// Tracks the DOM cursor position while traversing fragment items.
pub struct DomCursor<'a> {
    /// The current "previous" DOM expression (e.g. `$.first_child(fragment)` or a var reference).
    pub prev: Expression<'a>,
    /// Number of DOM siblings traversed since the last assigned variable.
    pub sibling_offset: usize,
    /// True until the first variable assignment inside this fragment has been made.
    pub is_first: bool,
}

impl<'a> DomCursor<'a> {
    pub fn new(anchor: Expression<'a>) -> Self {
        Self { prev: anchor, sibling_offset: 0, is_first: true }
    }

    /// Advance past a static sibling (increment offset).
    pub fn next_sibling(&mut self) {
        self.sibling_offset += 1;
    }

    /// Build the expression to get the current item node, then reset offset to 0.
    /// Returns the expression WITHOUT assigning it to a variable.
    pub fn take_node_expr<'b>(&mut self, b: &Builder<'a>) -> Expression<'a> {
        let expr = b.move_expr(&mut self.prev);

        if self.sibling_offset == 0 {
            // Current item IS the prev expression
            return expr;
        }

        let args: Vec<crate::builder::Arg<'a, '_>> = if self.sibling_offset == 1 {
            vec![crate::builder::Arg::Expr(expr)]
        } else {
            vec![
                crate::builder::Arg::Expr(expr),
                crate::builder::Arg::Num(self.sibling_offset as f64),
            ]
        };

        b.call_expr("$.sibling", args)
    }

    /// Reset after assigning a variable: set sibling_offset = 1 (next is 1 away),
    /// and record the variable as prev.
    pub fn after_assign(&mut self, var_expr: Expression<'a>) {
        self.prev = var_expr;
        self.sibling_offset = 1;
        self.is_first = false;
    }
}

/// Central codegen context. Holds refs to allocator, builder, component, analysis,
/// and mutable state (ident counter, mutated runes set).
pub struct Ctx<'a> {
    pub b: Builder<'a>,
    pub component: &'a Component,
    pub analysis: &'a AnalysisData,
    /// Monotonically incrementing counter per name prefix.
    ident_counters: HashMap<String, u32>,
    /// Rune symbol names that are mutated (assigned at least once in script).
    pub mutated_runes: std::collections::HashSet<String>,
    /// Template declarations from nested fragments, hoisted to module scope.
    pub module_hoisted: Vec<Statement<'a>>,
}

impl<'a> Ctx<'a> {
    pub fn new(
        allocator: &'a oxc_allocator::Allocator,
        component: &'a Component,
        analysis: &'a AnalysisData,
    ) -> Self {
        Self {
            b: Builder::new(allocator),
            component,
            analysis,
            ident_counters: HashMap::new(),
            mutated_runes: std::collections::HashSet::new(),
            module_hoisted: Vec::new(),
        }
    }

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

    /// Look up the source text for a span.
    pub fn span_text(&self, span: svelte_span::Span) -> &str {
        self.component.source_text(span)
    }

    /// Check if a node (ExpressionTag / IfBlock / EachBlock) is dynamic
    /// (references a rune symbol).
    pub fn is_dynamic_node(&self, id: NodeId) -> bool {
        self.analysis.dynamic_nodes.contains(&id)
    }

    /// Check if a rune symbol name is mutated (assigned in script).
    pub fn rune_is_mutated(&self, name: &str) -> bool {
        self.mutated_runes.contains(name)
    }
}
