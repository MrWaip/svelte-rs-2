//! `{#each}` population for Block Semantics.
//!
//! One template walk. All declaration/expression identities come from
//! `ParserResult` (pre-parsed JS for template spans); scope and symbol
//! lookups come from `ComponentSemantics`. No `AnalysisData` access.

use super::super::{
    BlockSemantics, BlockSemanticsStore, EachAsyncKind, EachBlockSemantics, EachFlags, EachFlavor,
    EachIndexKind, EachItemKind, EachKeyKind,
};
use crate::reactivity_semantics::data::{ReactivitySemantics, ReferenceSemantics};
use crate::types::data::{BlockerData, ParserResult};
use oxc_ast::ast::{BindingPattern, Expression, IdentifierReference, Statement};
use oxc_ast_visit::Visit;
use rustc_hash::FxHashSet;
use smallvec::SmallVec;
use svelte_ast::{Attribute, BindDirective, Component, EachBlock, FragmentKey, Node, NodeId};
use svelte_component_semantics::{ComponentSemantics, OxcNodeId, ReferenceId, SymbolId};

pub(super) fn populate(
    component: &Component,
    parsed: &ParserResult<'_>,
    semantics: &ComponentSemantics<'_>,
    reactivity: &ReactivitySemantics,
    blockers: &BlockerData,
    store: &mut BlockSemanticsStore,
) {
    let mut ctx = Ctx {
        component,
        parsed,
        semantics,
        reactivity,
        blockers,
        store,
        each_stack: SmallVec::new(),
        bind_group_hits: FxHashSet::default(),
    };
    for &node_id in &component.fragment.nodes {
        ctx.visit_node(node_id);
    }
}

struct Ctx<'c, 'a> {
    component: &'c Component,
    parsed: &'c ParserResult<'a>,
    semantics: &'c ComponentSemantics<'a>,
    reactivity: &'c ReactivitySemantics,
    blockers: &'c BlockerData,
    store: &'c mut BlockSemanticsStore,
    /// Stack of enclosing each-blocks during the template walk. Each
    /// frame carries the node id and the symbols that each introduces
    /// in its body scope (item / index / destructured leaves). Used to
    /// attribute `bind:group={...}` directives to the correct enclosing
    /// each frame (Svelte's scope-qualified rule).
    each_stack: SmallVec<[EachFrame; 4]>,
    /// Set of each-block node ids that contain a `bind:group` whose
    /// expression references one of their introduced symbols.
    bind_group_hits: FxHashSet<NodeId>,
}

#[derive(Clone)]
struct EachFrame {
    block_id: NodeId,
    introduced: SmallVec<[SymbolId; 4]>,
}

impl<'a> Ctx<'_, 'a> {
    fn visit_node(&mut self, id: NodeId) {
        let node = self.component.store.get(id);
        match node {
            Node::EachBlock(block) => self.visit_each(block),
            Node::Element(el) => {
                self.check_bind_group_in_attrs(&el.attributes);
                self.visit_fragment(&el.fragment.nodes);
            }
            Node::SlotElementLegacy(el) => self.visit_fragment(&el.fragment.nodes),
            Node::ComponentNode(cn) => self.visit_fragment(&cn.fragment.nodes),
            Node::IfBlock(block) => {
                self.visit_fragment(&block.consequent.nodes);
                if let Some(alt) = &block.alternate {
                    self.visit_fragment(&alt.nodes);
                }
            }
            Node::SnippetBlock(block) => self.visit_fragment(&block.body.nodes),
            Node::KeyBlock(block) => self.visit_fragment(&block.fragment.nodes),
            Node::SvelteHead(el) => self.visit_fragment(&el.fragment.nodes),
            Node::SvelteFragmentLegacy(el) => self.visit_fragment(&el.fragment.nodes),
            Node::SvelteElement(el) => {
                self.check_bind_group_in_attrs(&el.attributes);
                self.visit_fragment(&el.fragment.nodes);
            }
            Node::SvelteBoundary(el) => self.visit_fragment(&el.fragment.nodes),
            Node::AwaitBlock(block) => {
                if let Some(f) = &block.pending {
                    self.visit_fragment(&f.nodes);
                }
                if let Some(f) = &block.then {
                    self.visit_fragment(&f.nodes);
                }
                if let Some(f) = &block.catch {
                    self.visit_fragment(&f.nodes);
                }
            }
            _ => {}
        }
    }

    fn visit_fragment(&mut self, nodes: &[NodeId]) {
        for &id in nodes {
            self.visit_node(id);
        }
    }

    fn visit_each(&mut self, block: &EachBlock) {
        let context_declarator = block
            .context_span
            .and_then(|cs| self.parsed.stmt_handle(cs.start))
            .and_then(|h| declarator_from_stmt(self.parsed.stmt(h)?));

        let index_declarator = block
            .index_span
            .and_then(|cs| self.parsed.stmt_handle(cs.start))
            .and_then(|h| declarator_from_stmt(self.parsed.stmt(h)?));

        let key_expr = block
            .key_span
            .and_then(|ks| self.parsed.expr_handle(ks.start))
            .and_then(|h| self.parsed.expr(h));

        // Collection expression for flag computation.
        let collection_expr = self
            .parsed
            .expr_handle(block.expression_span.start)
            .and_then(|h| self.parsed.expr(h));

        // Item
        let (item, item_sym) = match context_declarator {
            None => (EachItemKind::NoBinding, None),
            Some(d) => match &d.id {
                BindingPattern::BindingIdentifier(ident) => {
                    let sym = self
                        .semantics
                        .fragment_scope(&FragmentKey::EachBody(block.id))
                        .and_then(|scope| self.semantics.find_binding(scope, ident.name.as_str()));
                    match sym {
                        Some(sym) => (EachItemKind::Identifier(sym), Some(sym)),
                        None => (EachItemKind::NoBinding, None),
                    }
                }
                _ => (EachItemKind::Pattern(binding_pattern_node_id(&d.id)), None),
            },
        };

        let body_scope = self
            .semantics
            .fragment_scope(&FragmentKey::EachBody(block.id));

        // Index — raw symbol + usage-in-body/in-key split resolved below.
        let index_sym = index_declarator
            .and_then(|d| binding_ident_of(&d.id))
            .and_then(|ident| {
                body_scope.and_then(|scope| self.semantics.find_binding(scope, ident.name.as_str()))
            });

        // Key
        let key = match key_expr {
            None => EachKeyKind::Unkeyed,
            Some(expr) => {
                if let Some(sym) = item_sym {
                    if expression_is_identifier_of(expr, sym, self.semantics) {
                        EachKeyKind::KeyedByItem
                    } else {
                        EachKeyKind::KeyedByExpr(expression_node_id(expr))
                    }
                } else {
                    EachKeyKind::KeyedByExpr(expression_node_id(expr))
                }
            }
        };

        // Collect all symbols this each introduces in its body scope.
        // Used for scope-qualified `bind:group` attribution via the walk
        // stack below.
        let introduced = self.collect_introduced_symbols(block, &item, index_sym);

        // Index-usage split. The total count of resolved references on
        // the index symbol comes from `ComponentSemantics`. We sub-walk
        // the key expression (a separate parser-owned subtree) to count
        // references living there; anything else is "body usage".
        let index = match index_sym {
            Some(sym) => {
                self.store.record_each_index_sym(sym, block.id);
                let all_refs = self.semantics.get_resolved_reference_ids(sym);
                let used_in_key = match key_expr {
                    Some(expr) => expression_contains_reference_to(expr, sym, self.semantics),
                    None => false,
                };
                // used_in_body = any ref exists that is NOT the key ref.
                // When used_in_key is false, any ref implies body use.
                // When used_in_key is true, we need at least one ref NOT
                // in the key expression — so count refs in key and
                // compare against the total.
                let used_in_body = if !used_in_key {
                    !all_refs.is_empty()
                } else {
                    let key_ref_count = key_expr
                        .map(|e| count_references_to_in_expr(e, sym, self.semantics))
                        .unwrap_or(0);
                    all_refs.len() > key_ref_count
                };
                EachIndexKind::Declared {
                    sym,
                    used_in_body,
                    used_in_key,
                }
            }
            None => EachIndexKind::Absent,
        };

        // has_animate — shallow: direct child element carries `animate:`.
        let has_animate = self.body_has_direct_animate(&block.body.nodes);

        // shadows_outer — any own binding in body scope has the same
        // name as some binding visible in the parent scope.
        let shadows_outer = body_scope
            .and_then(|child| {
                self.semantics
                    .scope_parent_id(child)
                    .map(|parent| (child, parent))
            })
            .is_some_and(|(child, parent)| {
                self.semantics
                    .own_binding_names(child)
                    .any(|name| self.semantics.find_binding(parent, name).is_some())
            });

        // Collection expression facts — drive ITEM_REACTIVE /
        // ITEM_IMMUTABLE and the async-lowering decision.
        let collection_facts = match (collection_expr, body_scope) {
            (Some(expr), Some(scope)) => self.collection_expression_facts(expr, scope),
            _ => CollectionExprFacts::default(),
        };
        let has_external = collection_facts.has_external;
        let uses_store = collection_facts.uses_store;
        let async_kind = if collection_facts.has_await || !collection_facts.blockers.is_empty() {
            EachAsyncKind::Async {
                has_await: collection_facts.has_await,
                blockers: collection_facts.blockers,
            }
        } else {
            EachAsyncKind::Sync
        };

        let has_key = !matches!(key, EachKeyKind::Unkeyed);
        let has_index = matches!(index, EachIndexKind::Declared { .. });
        let key_is_item = matches!(key, EachKeyKind::KeyedByItem);
        let runes = self.reactivity.uses_runes();

        let mut each_flags = EachFlags::empty();
        // EACH_INDEX_REACTIVE: keyed block with user-declared index.
        if has_key && has_index {
            each_flags |= EachFlags::INDEX_REACTIVE;
        }
        // EACH_ITEM_REACTIVE: see reference EachBlock.js — set when the
        // collection expression references external state, unless runes
        // mode lets us elide it because the key IS the item identifier
        // (and there are no store deps to force the reactive path).
        if has_external && (!runes || !key_is_item || uses_store) {
            each_flags |= EachFlags::ITEM_REACTIVE;
        }
        // EACH_ITEM_IMMUTABLE: runes-mode optimization, suppressed by
        // store deps.
        if runes && !uses_store {
            each_flags |= EachFlags::ITEM_IMMUTABLE;
        }
        // EACH_IS_ANIMATED: keyed + direct animate child.
        if has_key && has_animate {
            each_flags |= EachFlags::ANIMATED;
        }

        // Push this each onto the walk stack so nested `bind:group`
        // directives can attribute themselves to us when their operand
        // references any of `introduced`.
        self.each_stack.push(EachFrame {
            block_id: block.id,
            introduced,
        });
        self.visit_fragment(&block.body.nodes);
        if let Some(fb) = &block.fallback {
            self.visit_fragment(&fb.nodes);
        }
        self.each_stack.pop();

        // Flavor is decided *after* the body walk: `BindGroup` iff at
        // least one `bind:group={expr}` inside the body (at any depth)
        // references a symbol introduced by this each.
        let flavor = if self.bind_group_hits.contains(&block.id) {
            EachFlavor::BindGroup
        } else {
            EachFlavor::Regular
        };

        self.store.set(
            block.id,
            BlockSemantics::Each(EachBlockSemantics {
                item,
                index,
                key,
                flavor,
                each_flags,
                shadows_outer,
                async_kind,
            }),
        );
    }

    /// Collect all `SymbolId`s introduced by this each-block in its
    /// body scope: item binding (identifier OR destructured leaves) and
    /// the optional index binding. Used for scope-qualified
    /// `bind:group` attribution.
    fn collect_introduced_symbols(
        &self,
        block: &EachBlock,
        item: &EachItemKind,
        index_sym: Option<SymbolId>,
    ) -> SmallVec<[SymbolId; 4]> {
        let mut out: SmallVec<[SymbolId; 4]> = SmallVec::new();
        match item {
            EachItemKind::Identifier(sym) => out.push(*sym),
            EachItemKind::Pattern(_) => {
                // Walk the pattern's BindingPattern AST to collect every
                // leaf BindingIdentifier's symbol_id.
                if let Some(decl) = block
                    .context_span
                    .and_then(|cs| self.parsed.stmt_handle(cs.start))
                    .and_then(|h| declarator_from_stmt(self.parsed.stmt(h)?))
                {
                    collect_binding_pattern_symbols(&decl.id, &mut out);
                }
            }
            EachItemKind::NoBinding => {}
        }
        if let Some(sym) = index_sym {
            out.push(sym);
        }
        out
    }

    /// Scan element attributes for a `bind:group={...}` directive. If
    /// found, walk its expression and, for each enclosing each on the
    /// stack whose `introduced` symbols match any referenced symbol,
    /// record a hit.
    fn check_bind_group_in_attrs(&mut self, attrs: &[Attribute]) {
        if self.each_stack.is_empty() {
            return;
        }
        for attr in attrs {
            let Attribute::BindDirective(dir) = attr else {
                continue;
            };
            if dir.name != "group" {
                continue;
            }
            self.attribute_bind_group(dir);
        }
    }

    fn attribute_bind_group(&mut self, dir: &BindDirective) {
        let Some(expr) = self
            .parsed
            .expr_handle(dir.expression_span.start)
            .and_then(|h| self.parsed.expr(h))
        else {
            return;
        };
        let mut collector = RefCollector { refs: Vec::new() };
        collector.visit_expression(expr);
        // For every ref, find which enclosing each (if any) introduced
        // its resolved symbol.
        for ref_id in collector.refs {
            let Some(sym) = self.semantics.get_reference(ref_id).symbol_id() else {
                continue;
            };
            for frame in &self.each_stack {
                if frame.introduced.contains(&sym) {
                    self.bind_group_hits.insert(frame.block_id);
                }
            }
        }
    }

    /// Collection expression facts gathered in a single walk:
    ///
    /// - `has_external` — some reference resolves to a symbol declared
    ///   in a function scope shallower than the each-body scope.
    /// - `uses_store` — some reference is classified as a store op by
    ///   `reactivity_semantics`.
    /// - `has_await` — an `await` literal appears in the expression.
    /// - `blockers` — sorted, de-duplicated blocker indices from
    ///   `BlockerData::symbol_blockers` for every ref.
    fn collection_expression_facts(
        &self,
        expr: &Expression<'a>,
        body_scope: oxc_syntax::scope::ScopeId,
    ) -> CollectionExprFacts {
        // The each body lowers as an arrow callback (`($$anchor, item) => {...}`),
        // which is one function level deeper than the body-scope itself at
        // analyze time. Reference-compiler measures "external" as
        // `binding.function_depth < emit_scope.function_depth`; we mirror
        // that here by comparing binding scopes against body_scope+1.
        let each_depth = self.semantics.function_depth(body_scope) + 1;
        let mut collector = CollectionExprCollector {
            refs: Vec::new(),
            has_await: false,
        };
        collector.visit_expression(expr);

        let mut has_external = false;
        let mut uses_store = false;
        let mut blockers: SmallVec<[u32; 2]> = SmallVec::new();
        for ref_id in &collector.refs {
            let sem = self.reactivity.reference_semantics(*ref_id);
            // Effective symbol for scope-depth and blocker lookup. Store
            // reads come with `symbol_id=None` on the Reference itself
            // but carry the underlying store binding inside the semantic
            // payload; use that so scope / blocker resolution works
            // uniformly.
            let effective_sym = match sem {
                ReferenceSemantics::StoreRead { symbol }
                | ReferenceSemantics::StoreWrite { symbol }
                | ReferenceSemantics::StoreUpdate { symbol } => Some(symbol),
                _ => self.semantics.get_reference(*ref_id).symbol_id(),
            };
            if !uses_store
                && matches!(
                    sem,
                    ReferenceSemantics::StoreRead { .. }
                        | ReferenceSemantics::StoreWrite { .. }
                        | ReferenceSemantics::StoreUpdate { .. }
                )
            {
                uses_store = true;
            }
            if let Some(sym) = effective_sym {
                if !has_external {
                    let decl_scope = self.semantics.symbol_scope_id(sym);
                    if self.semantics.function_depth(decl_scope) < each_depth {
                        has_external = true;
                    }
                }
                if let Some(idx) = self.blockers.symbol_blocker(sym) {
                    if !blockers.contains(&idx) {
                        blockers.push(idx);
                    }
                }
            }
        }
        blockers.sort_unstable();
        CollectionExprFacts {
            has_external,
            uses_store,
            has_await: collector.has_await,
            blockers,
        }
    }

    /// Shallow scan of direct-child elements for an `animate:` directive.
    fn body_has_direct_animate(&self, nodes: &[NodeId]) -> bool {
        nodes.iter().any(|&id| {
            let node = self.component.store.get(id);
            match node {
                Node::Element(el) => el
                    .attributes
                    .iter()
                    .any(|a| matches!(a, Attribute::AnimateDirective(_))),
                Node::SvelteElement(el) => el
                    .attributes
                    .iter()
                    .any(|a| matches!(a, Attribute::AnimateDirective(_))),
                _ => false,
            }
        })
    }
}

fn declarator_from_stmt<'a>(
    stmt: &'a Statement<'a>,
) -> Option<&'a oxc_ast::ast::VariableDeclarator<'a>> {
    let Statement::VariableDeclaration(decl) = stmt else {
        return None;
    };
    decl.declarations.first()
}

fn binding_ident_of<'a>(
    pattern: &'a BindingPattern<'a>,
) -> Option<&'a oxc_ast::ast::BindingIdentifier<'a>> {
    match pattern {
        BindingPattern::BindingIdentifier(ident) => Some(ident),
        _ => None,
    }
}

/// Recursively collect `symbol_id` for every leaf `BindingIdentifier`
/// inside a `BindingPattern`. Used to enumerate the symbols introduced
/// by a destructured `{#each items as { a, b, ...rest }}` context.
fn collect_binding_pattern_symbols(
    pattern: &BindingPattern<'_>,
    out: &mut SmallVec<[SymbolId; 4]>,
) {
    use oxc_ast::ast::BindingPattern as BP;
    match pattern {
        BP::BindingIdentifier(ident) => {
            if let Some(sym) = ident.symbol_id.get() {
                out.push(sym);
            }
        }
        BP::ObjectPattern(obj) => {
            for prop in &obj.properties {
                collect_binding_pattern_symbols(&prop.value, out);
            }
            if let Some(rest) = &obj.rest {
                collect_binding_pattern_symbols(&rest.argument, out);
            }
        }
        BP::ArrayPattern(arr) => {
            for el in arr.elements.iter().flatten() {
                collect_binding_pattern_symbols(el, out);
            }
            if let Some(rest) = &arr.rest {
                collect_binding_pattern_symbols(&rest.argument, out);
            }
        }
        BP::AssignmentPattern(assign) => {
            collect_binding_pattern_symbols(&assign.left, out);
        }
    }
}

#[derive(Default)]
struct CollectionExprFacts {
    has_external: bool,
    uses_store: bool,
    has_await: bool,
    blockers: SmallVec<[u32; 2]>,
}

/// Collects identifier references plus detects `await` literals over a
/// single Expression walk.
struct CollectionExprCollector {
    refs: Vec<ReferenceId>,
    has_await: bool,
}

impl<'a> Visit<'a> for CollectionExprCollector {
    fn visit_identifier_reference(&mut self, ident: &IdentifierReference<'a>) {
        if let Some(ref_id) = ident.reference_id.get() {
            self.refs.push(ref_id);
        }
    }
    fn visit_await_expression(&mut self, expr: &oxc_ast::ast::AwaitExpression<'a>) {
        self.has_await = true;
        // Keep walking — nested refs inside the awaited expression count.
        oxc_ast_visit::walk::walk_await_expression(self, expr);
    }
}

struct RefCollector {
    refs: Vec<ReferenceId>,
}

impl<'a> Visit<'a> for RefCollector {
    fn visit_identifier_reference(&mut self, ident: &IdentifierReference<'a>) {
        if let Some(ref_id) = ident.reference_id.get() {
            self.refs.push(ref_id);
        }
    }
}

fn binding_pattern_node_id(pattern: &BindingPattern<'_>) -> OxcNodeId {
    match pattern {
        BindingPattern::BindingIdentifier(p) => p.node_id(),
        BindingPattern::ObjectPattern(p) => p.node_id(),
        BindingPattern::ArrayPattern(p) => p.node_id(),
        BindingPattern::AssignmentPattern(p) => p.node_id(),
    }
}

fn expression_node_id(expr: &Expression<'_>) -> OxcNodeId {
    match expr {
        Expression::Identifier(e) => e.node_id(),
        Expression::StringLiteral(e) => e.node_id(),
        Expression::NumericLiteral(e) => e.node_id(),
        Expression::BooleanLiteral(e) => e.node_id(),
        Expression::NullLiteral(e) => e.node_id(),
        Expression::TemplateLiteral(e) => e.node_id(),
        Expression::BigIntLiteral(e) => e.node_id(),
        Expression::RegExpLiteral(e) => e.node_id(),
        Expression::ArrayExpression(e) => e.node_id(),
        Expression::ObjectExpression(e) => e.node_id(),
        Expression::ArrowFunctionExpression(e) => e.node_id(),
        Expression::FunctionExpression(e) => e.node_id(),
        Expression::AssignmentExpression(e) => e.node_id(),
        Expression::AwaitExpression(e) => e.node_id(),
        Expression::BinaryExpression(e) => e.node_id(),
        Expression::CallExpression(e) => e.node_id(),
        Expression::ChainExpression(e) => e.node_id(),
        Expression::ClassExpression(e) => e.node_id(),
        Expression::ConditionalExpression(e) => e.node_id(),
        Expression::LogicalExpression(e) => e.node_id(),
        Expression::NewExpression(e) => e.node_id(),
        Expression::ParenthesizedExpression(e) => e.node_id(),
        Expression::SequenceExpression(e) => e.node_id(),
        Expression::TaggedTemplateExpression(e) => e.node_id(),
        Expression::ThisExpression(e) => e.node_id(),
        Expression::UnaryExpression(e) => e.node_id(),
        Expression::UpdateExpression(e) => e.node_id(),
        Expression::YieldExpression(e) => e.node_id(),
        Expression::PrivateInExpression(e) => e.node_id(),
        Expression::JSXElement(e) => e.node_id(),
        Expression::JSXFragment(e) => e.node_id(),
        Expression::ImportExpression(e) => e.node_id(),
        Expression::MetaProperty(e) => e.node_id(),
        Expression::Super(e) => e.node_id(),
        Expression::V8IntrinsicExpression(e) => e.node_id(),
        Expression::ComputedMemberExpression(e) => e.node_id(),
        Expression::StaticMemberExpression(e) => e.node_id(),
        Expression::PrivateFieldExpression(e) => e.node_id(),
        Expression::TSAsExpression(e) => e.node_id(),
        Expression::TSSatisfiesExpression(e) => e.node_id(),
        Expression::TSTypeAssertion(e) => e.node_id(),
        Expression::TSNonNullExpression(e) => e.node_id(),
        Expression::TSInstantiationExpression(e) => e.node_id(),
    }
}

fn expression_is_identifier_of(
    expr: &Expression<'_>,
    target: SymbolId,
    semantics: &ComponentSemantics<'_>,
) -> bool {
    let Expression::Identifier(ident) = expr else {
        return false;
    };
    let Some(ref_id) = ident.reference_id.get() else {
        return false;
    };
    semantics.get_reference(ref_id).symbol_id() == Some(target)
}

struct IdentRefCounter<'s, 'a> {
    target: SymbolId,
    semantics: &'s ComponentSemantics<'a>,
    count: usize,
    early_exit: bool,
}

impl<'a> Visit<'a> for IdentRefCounter<'_, 'a> {
    fn visit_identifier_reference(&mut self, ident: &IdentifierReference<'a>) {
        if self.early_exit && self.count > 0 {
            return;
        }
        if let Some(ref_id) = ident.reference_id.get() {
            if self.semantics.get_reference(ref_id).symbol_id() == Some(self.target) {
                self.count += 1;
            }
        }
    }
}

fn expression_contains_reference_to(
    expr: &Expression<'_>,
    target: SymbolId,
    semantics: &ComponentSemantics<'_>,
) -> bool {
    let mut counter = IdentRefCounter {
        target,
        semantics,
        count: 0,
        early_exit: true,
    };
    counter.visit_expression(expr);
    counter.count > 0
}

fn count_references_to_in_expr(
    expr: &Expression<'_>,
    target: SymbolId,
    semantics: &ComponentSemantics<'_>,
) -> usize {
    let mut counter = IdentRefCounter {
        target,
        semantics,
        count: 0,
        early_exit: false,
    };
    counter.visit_expression(expr);
    counter.count
}
