use oxc_ast::ast::{
    ArrowFunctionExpression, AssignmentPattern, BindingIdentifier, BindingPattern, Expression,
    FormalParameter, IdentifierReference, Statement, VariableDeclarator,
};
use oxc_ast_visit::Visit;
use oxc_semantic::ScopeId;
use rustc_hash::FxHashSet;
use svelte_ast::{
    Attribute, AwaitBlock, Component, EachBlock, Element, LetDirectiveLegacy, Node, NodeId,
    SnippetBlock,
};
use svelte_component_semantics::{OxcNodeId, ReferenceId};

use super::super::data::{
    BindingFacts, ContextualBindingSemantics, ContextualReadKind, DeclaratorGroup,
    DeclaratorSemantics, EachIndexStrategy, EachItemStrategy, LegacyStateSemantics,
    ReferenceSemantics, SnippetParamStrategy,
};
use crate::scope::SymbolId;
use crate::types::data::{AnalysisData, JsAst};
use crate::utils::legacy_slot::legacy_slot_pattern;
use crate::walker::{ParentKind, TemplateVisitor, VisitContext, walk_template};

#[derive(Clone, Copy)]
enum PendingKind {
    EachItem,
    EachIndex,
    AwaitValue,
    AwaitError,
    LetDirective,
    SnippetParam,
}

#[derive(Default)]
struct ContextualStaging {
    pending: Vec<(SymbolId, PendingKind)>,
    getter_symbols: FxHashSet<SymbolId>,
    each_non_reactive_symbols: FxHashSet<SymbolId>,
    signal_read_symbols: FxHashSet<SymbolId>,
    select_bind_roots: Vec<SymbolId>,
}

impl ContextualStaging {
    fn mark_getter(&mut self, sym: SymbolId) {
        self.getter_symbols.insert(sym);
    }
    fn mark_each_non_reactive(&mut self, sym: SymbolId) {
        self.each_non_reactive_symbols.insert(sym);
    }
    fn mark_signal_read(&mut self, sym: SymbolId) {
        self.signal_read_symbols.insert(sym);
    }
    fn push_select_bind_root(&mut self, sym: SymbolId) {
        if !self.select_bind_roots.contains(&sym) {
            self.select_bind_roots.push(sym);
        }
    }
    fn push(&mut self, sym: SymbolId, kind: PendingKind) {
        self.pending.push((sym, kind));
    }
}

pub(super) fn collect_template_declarations<'a>(
    component: &Component,
    parsed: &JsAst<'a>,
    data: &mut AnalysisData<'a>,
) {
    let root = data.scoping.root_scope_id();
    let component_name = data.component_name.clone();
    let mut staging = ContextualStaging::default();
    let mut ctx = VisitContext::with_parsed(
        root,
        data,
        &component.store,
        parsed,
        &component.source,
        data.uses_runes(),
        &component_name,
        "",
    );
    let mut collector = TemplateDeclarationCollector {
        staging: &mut staging,
    };
    let mut visitors: [&mut dyn TemplateVisitor; 1] = [&mut collector];
    walk_template(component.root, &mut ctx, &mut visitors);

    finalize_contextual_declarations(data, staging);
}

fn finalize_contextual_declarations(data: &mut AnalysisData<'_>, staging: ContextualStaging) {
    let ContextualStaging {
        pending,
        getter_symbols,
        each_non_reactive_symbols,
        signal_read_symbols,
        select_bind_roots,
    } = staging;
    record_legacy_indirect_bindings(data, &select_bind_roots);
    for (sym, kind) in pending {
        let semantics = match kind {
            PendingKind::EachItem => {
                let mutated_legacy = !data.uses_runes() && data.scoping.is_mutated(sym);
                let strategy = if getter_symbols.contains(&sym) {
                    EachItemStrategy::Accessor
                } else if signal_read_symbols.contains(&sym) {
                    EachItemStrategy::Signal
                } else if mutated_legacy {
                    EachItemStrategy::IndexedLegacy
                } else if each_non_reactive_symbols.contains(&sym) {
                    EachItemStrategy::Direct
                } else {
                    EachItemStrategy::Signal
                };
                ContextualBindingSemantics::EachItem(strategy)
            }
            PendingKind::EachIndex => {
                let strategy = if each_non_reactive_symbols.contains(&sym) {
                    EachIndexStrategy::Direct
                } else {
                    EachIndexStrategy::Signal
                };
                ContextualBindingSemantics::EachIndex(strategy)
            }
            PendingKind::AwaitValue => ContextualBindingSemantics::AwaitValue,
            PendingKind::AwaitError => ContextualBindingSemantics::AwaitError,
            PendingKind::LetDirective => ContextualBindingSemantics::LetDirective,
            PendingKind::SnippetParam => {
                let strategy = if getter_symbols.contains(&sym) {
                    SnippetParamStrategy::Accessor
                } else {
                    SnippetParamStrategy::Signal
                };
                ContextualBindingSemantics::SnippetParam(strategy)
            }
        };
        data.reactivity.record_contextual_binding(sym, semantics);
    }
}

struct TemplateDeclarationCollector<'s> {
    staging: &'s mut ContextualStaging,
}

impl TemplateVisitor for TemplateDeclarationCollector<'_> {
    fn visit_element(&mut self, el: &Element, ctx: &mut VisitContext<'_, '_>) {
        if ctx.data.script.runes() || el.name != "select" {
            return;
        }
        let Some(parsed) = ctx.parsed else {
            return;
        };
        for attr in &el.attributes {
            let Attribute::BindDirective(directive) = attr else {
                continue;
            };
            if directive.name != "value" {
                continue;
            }
            let Some(expr) = parsed.expr(directive.expression.id()) else {
                continue;
            };
            let Some(ref_id) = super::expression_root_reference_id(expr) else {
                continue;
            };
            let Some(sym) = ctx.data.scoping.symbol_for_reference(ref_id) else {
                continue;
            };
            self.staging.push_select_bind_root(sym);
        }
    }

    fn visit_const_tag(&mut self, tag: &svelte_ast::ConstTag, ctx: &mut VisitContext<'_, '_>) {
        let Some(parsed) = ctx.parsed() else {
            return;
        };
        let Some(stmt) = parsed.stmt(tag.decl.id()) else {
            return;
        };
        let Statement::VariableDeclaration(decl) = stmt else {
            return;
        };
        let Some(declarator) = decl.declarations.first() else {
            return;
        };
        let is_destructured = !matches!(declarator.id, BindingPattern::BindingIdentifier(_),);
        let initial_is_function = !is_destructured
            && matches!(
                declarator.init.as_ref().map(|e| e.get_inner_expression()),
                Some(Expression::ArrowFunctionExpression(_) | Expression::FunctionExpression(_),),
            );
        let mut syms: Vec<SymbolId> = Vec::new();
        svelte_component_semantics::walk_bindings(&declarator.id, |v| syms.push(v.symbol));

        for sym in syms.iter().copied() {
            ctx.data.reactivity.record_const_binding(
                sym,
                is_destructured,
                initial_is_function,
                tag.id,
            );
        }
    }

    fn visit_declaration_tag(
        &mut self,
        tag: &svelte_ast::DeclarationTag,
        ctx: &mut VisitContext<'_, '_>,
    ) {
        let Some(parsed) = ctx.parsed() else {
            return;
        };
        let Some(stmt) = parsed.stmt(tag.declaration.id()) else {
            return;
        };
        let Statement::VariableDeclaration(decl) = stmt else {
            return;
        };
        let mut syms: Vec<SymbolId> = Vec::new();
        for declarator in &decl.declarations {
            if matches!(
                ctx.data
                    .reactivity
                    .declarator_semantics(declarator.node_id())
                    .group(),
                DeclaratorGroup::Rune
            ) {
                continue;
            }
            svelte_component_semantics::walk_bindings(&declarator.id, |v| syms.push(v.symbol));
        }
        for sym in syms {
            ctx.data.reactivity.record_declaration_tag_binding(sym);
        }
    }

    fn visit_let_directive_legacy(
        &mut self,
        dir: &LetDirectiveLegacy,
        ctx: &mut VisitContext<'_, '_>,
    ) {
        let (syms, is_destructured, stmt_node_id) = {
            let Some(binding_ref) = dir.binding.as_ref() else {
                return;
            };
            let Some(stmt) = ctx.parsed().and_then(|p| p.stmt(binding_ref.id())) else {
                return;
            };
            let stmt_node_id = match stmt {
                Statement::VariableDeclaration(decl) => decl.node_id(),
                _ => return,
            };
            let Some(pattern) = legacy_slot_pattern(stmt) else {
                return;
            };
            let is_destructured = !matches!(pattern, BindingPattern::BindingIdentifier(_));
            let mut syms: Vec<(SymbolId, bool)> = Vec::new();
            svelte_component_semantics::walk_bindings(pattern, |v| {
                let leaked = v.path.iter().any(|s| {
                    s.default.is_some()
                        || matches!(s.access, svelte_component_semantics::Access::Slice { .. })
                });
                syms.push((v.symbol, leaked));
            });
            (syms, is_destructured, stmt_node_id)
        };

        let carrier_sym = if is_destructured {
            Some(ensure_slot_let_carrier(
                ctx.data,
                ctx.scope,
                stmt_node_id,
                dir.name.as_str(),
            ))
        } else {
            ctx.data.reactivity.record_let_simple_binding(stmt_node_id);
            None
        };

        for (sym, leaked) in syms {
            ctx.data.reactivity.record_contextual_owner(sym, dir.id);
            if let Some(carrier) = carrier_sym {
                if leaked {
                    ctx.data.reactivity.record_let_direct_sym(sym);
                } else {
                    ctx.data
                        .reactivity
                        .record_carrier_alias_binding(sym, carrier);
                }
            } else {
                self.staging.push(sym, PendingKind::LetDirective);
            }
        }
    }

    fn visit_each_block(&mut self, block: &EachBlock, ctx: &mut VisitContext<'_, '_>) {
        let body_scope = ctx.child_scope_by_id(block.body, ctx.scope);
        let context_declarator = block
            .context
            .as_ref()
            .and_then(|r| ctx.parsed().and_then(|p| p.stmt(r.id())))
            .and_then(declarator_from_stmt_local);
        let is_destructured = context_declarator
            .is_some_and(|d| !matches!(&d.id, BindingPattern::BindingIdentifier(_)));
        let item_pattern_node = context_declarator.and_then(|d| destructure_pattern_node(&d.id));

        if let Some(node) = item_pattern_node {
            ctx.data
                .reactivity
                .record_declarator_semantics(node, DeclaratorSemantics::EachItem);
        }

        run_each_context_marker(block, ctx, self.staging, is_destructured);

        if ctx.data.script.runes()
            && !is_destructured
            && let Some(key_ref) = block.key.as_ref()
            && !each_collection_uses_store(block, ctx)
        {
            mark_key_is_item_each_binding(block, body_scope, key_ref.span, ctx, self.staging);
        }

        if !each_collection_has_external_deps(block, body_scope, ctx) {
            mark_each_item_syms_non_reactive(block, ctx, self.staging);
        }

        run_each_index_marker(block, ctx, self.staging);
    }

    fn visit_snippet_block(&mut self, block: &SnippetBlock, ctx: &mut VisitContext<'_, '_>) {
        run_snippet_param_marker(block, ctx, self.staging);
    }

    fn visit_await_block(&mut self, block: &AwaitBlock, ctx: &mut VisitContext<'_, '_>) {
        let then_scope = match block.then {
            Some(t) => ctx.child_scope_by_id(t, ctx.scope),
            None => ctx.scope,
        };
        if let Some(stmt) = block
            .value
            .as_ref()
            .and_then(|r| ctx.parsed().and_then(|p| p.stmt(r.id())))
        {
            let declarator_node = await_destructure_node(stmt);
            for sym in scoped_stmt_symbols(ctx.data, then_scope, stmt) {
                ctx.data.reactivity.record_contextual_owner(sym, block.id);
                self.staging.push(sym, PendingKind::AwaitValue);
            }
            if let Some(node) = declarator_node {
                ctx.data
                    .reactivity
                    .record_declarator_semantics(node, DeclaratorSemantics::AwaitValue);
            }
        }

        let catch_scope = match block.catch {
            Some(c) => ctx.child_scope_by_id(c, ctx.scope),
            None => ctx.scope,
        };
        if let Some(stmt) = block
            .error
            .as_ref()
            .and_then(|r| ctx.parsed().and_then(|p| p.stmt(r.id())))
        {
            let declarator_node = await_destructure_node(stmt);
            for sym in scoped_stmt_symbols(ctx.data, catch_scope, stmt) {
                ctx.data.reactivity.record_contextual_owner(sym, block.id);
                self.staging.push(sym, PendingKind::AwaitError);
            }
            if let Some(node) = declarator_node {
                ctx.data
                    .reactivity
                    .record_declarator_semantics(node, DeclaratorSemantics::AwaitValue);
            }
        }
    }
}

fn scoped_stmt_symbols(
    _data: &AnalysisData,
    _scope: ScopeId,
    stmt: &Statement<'_>,
) -> Vec<SymbolId> {
    let Statement::VariableDeclaration(decl) = stmt else {
        return Vec::new();
    };
    let Some(declarator) = decl.declarations.first() else {
        return Vec::new();
    };
    let mut out = Vec::new();
    svelte_component_semantics::walk_bindings(&declarator.id, |v| out.push(v.symbol));
    out
}

fn ensure_slot_let_carrier(
    data: &mut AnalysisData,
    scope: ScopeId,
    stmt_node_id: OxcNodeId,
    preferred_name: &str,
) -> SymbolId {
    use super::super::data::DeclaratorSemantics;
    if let DeclaratorSemantics::LetCarrier {
        carrier_symbol: Some(s),
    } = data.reactivity.declarator_semantics(stmt_node_id)
    {
        return s;
    }
    let sym = data
        .scoping
        .add_unique_synthetic_binding(scope, preferred_name);
    data.reactivity
        .record_let_carrier_binding(stmt_node_id, sym);
    sym
}

pub(super) fn promote_each_sources_to_legacy_state<'a>(
    component: &Component,
    parsed: &JsAst<'a>,
    data: &mut AnalysisData<'a>,
) {
    let root = data.scoping.root_scope_id();
    let component_name = data.component_name.clone();
    let mut ctx = VisitContext::with_parsed(
        root,
        data,
        &component.store,
        parsed,
        &component.source,
        false,
        &component_name,
        "",
    );
    let mut promoter = EachSourcePromoter;
    let mut visitors: [&mut dyn TemplateVisitor; 1] = [&mut promoter];
    walk_template(component.root, &mut ctx, &mut visitors);
}

struct EachSourcePromoter;

impl TemplateVisitor for EachSourcePromoter {
    fn visit_each_block(&mut self, block: &EachBlock, ctx: &mut VisitContext<'_, '_>) {
        let Some(parsed) = ctx.parsed else { return };
        let runes = ctx.data.script.runes();

        let item_syms: Vec<SymbolId> = {
            let Some(stmt) = block.context.as_ref().and_then(|r| parsed.stmt(r.id())) else {
                return;
            };
            let Some(declarator) = declarator_from_stmt_local(stmt) else {
                return;
            };
            let mut syms = Vec::new();
            svelte_component_semantics::walk_bindings(&declarator.id, |v| syms.push(v.symbol));
            syms
        };

        let mut inner_mutated_bindings = false;
        collect_each_body_inner_mutation(block, ctx, parsed, &mut inner_mutated_bindings);

        if !inner_mutated_bindings
            && !item_syms
                .iter()
                .any(|&sym| ctx.data.scoping.is_mutated_any(sym))
        {
            return;
        }

        let mut promoted_sources: Vec<SymbolId> = Vec::new();
        let mut collection_store: Option<SymbolId> = None;
        collect_each_block_collection_sources_legacy(
            block,
            ctx,
            runes,
            true,
            &mut promoted_sources,
            &mut collection_store,
        );

        let mut ancestor_each_block_ids: Vec<NodeId> = ctx
            .ancestors()
            .filter(|parent| parent.kind == ParentKind::EachBlock)
            .map(|parent| parent.id)
            .collect();
        ancestor_each_block_ids.reverse();
        let store = ctx.store;
        for ancestor_id in ancestor_each_block_ids {
            let Node::EachBlock(ancestor) = store.get(ancestor_id) else {
                continue;
            };
            collect_each_block_collection_sources_legacy(
                ancestor,
                ctx,
                runes,
                false,
                &mut promoted_sources,
                &mut collection_store,
            );
        }

        if !promoted_sources.is_empty() {
            for item_sym in &item_syms {
                for &source_sym in &promoted_sources {
                    ctx.data
                        .reactivity
                        .add_each_item_indirect_source(*item_sym, source_sym);
                }
            }
        }

        if let Some(store_sym) = collection_store {
            for &item_sym in &item_syms {
                ctx.data
                    .reactivity
                    .set_each_item_collection_store(item_sym, store_sym);
            }
        }

        let index_sym = block
            .index
            .as_ref()
            .and_then(|r| parsed.stmt(r.id()))
            .and_then(declarator_from_stmt_local)
            .and_then(|d| d.id.get_binding_identifier())
            .and_then(|ident| ident.symbol_id.get());
        if let Some(index_sym) = index_sym {
            for &item_sym in &item_syms {
                ctx.data
                    .reactivity
                    .set_each_item_index_legacy(item_sym, index_sym);
            }
        }
    }
}

fn collect_each_block_collection_sources_legacy(
    block: &EachBlock,
    ctx: &mut VisitContext<'_, '_>,
    runes: bool,
    is_primary_block: bool,
    out_sources: &mut Vec<SymbolId>,
    out_collection_store: &mut Option<SymbolId>,
) {
    let Some(parsed) = ctx.parsed else {
        return;
    };
    let Some(expr) = parsed.expr(block.expression.id()) else {
        return;
    };

    let mut collector = ExprRefCollector { refs: Vec::new() };
    collector.visit_expression(expr);

    let immutable = ctx.data.script.immutable;
    let store_candidate_refs: FxHashSet<ReferenceId> = ctx
        .data
        .scoping
        .store_candidate_refs()
        .iter()
        .map(|(_, ref_id)| *ref_id)
        .collect();
    for ref_id in collector.refs {
        if store_candidate_refs.contains(&ref_id) {
            if !is_primary_block {
                continue;
            }
            let Some(base_sym) = ctx.data.scoping.get_reference(ref_id).symbol_id() else {
                continue;
            };
            if let Some(store_sym) = ctx.data.reactivity.store_shadow_of_internal(base_sym) {
                if out_collection_store.is_none() {
                    *out_collection_store = Some(store_sym);
                }
                if !runes {
                    out_sources.push(store_sym);
                }
            }
            continue;
        }
        if runes {
            continue;
        }
        let Some(sym) = ctx.data.scoping.get_reference(ref_id).symbol_id() else {
            continue;
        };
        match ctx.data.reactivity.binding_facts(sym) {
            Some(
                BindingFacts::LegacyBindableProp(_)
                | BindingFacts::LegacyState(_)
                | BindingFacts::Contextual(
                    ContextualBindingSemantics::AwaitValue
                    | ContextualBindingSemantics::EachItem(_),
                ),
            ) => out_sources.push(sym),
            Some(
                BindingFacts::State(_)
                | BindingFacts::Derived(_)
                | BindingFacts::OptimizedDerived(_)
                | BindingFacts::OptimizedRune(_)
                | BindingFacts::Prop(_)
                | BindingFacts::LegacyApiExport
                | BindingFacts::Store(_)
                | BindingFacts::Const(_)
                | BindingFacts::OptimizedConst(_)
                | BindingFacts::DeclarationTag
                | BindingFacts::OptimizedDeclarationTag
                | BindingFacts::Contextual(
                    ContextualBindingSemantics::EachIndex(_)
                    | ContextualBindingSemantics::AwaitError
                    | ContextualBindingSemantics::LetDirective
                    | ContextualBindingSemantics::LetDirectiveCarrierMember { .. }
                    | ContextualBindingSemantics::LetDirectiveDirect
                    | ContextualBindingSemantics::SnippetParam(_),
                )
                | BindingFacts::RuntimeRune { .. }
                | BindingFacts::CarrierAlias { .. },
            ) => {}
            None => {
                if ctx.data.scoping.is_component_top_level_symbol(sym)
                    && ctx.data.reactivity.store_shadow_of_internal(sym).is_none()
                    && !super::symbol_is_function_declaration(&ctx.data.scoping, sym)
                {
                    if is_primary_block && !ctx.data.scoping.is_import(sym) {
                        ctx.data.reactivity.record_legacy_state_binding(
                            sym,
                            LegacyStateSemantics {
                                var_declared: false,
                                immutable,
                                is_signal_source: false,
                            },
                        );
                    }
                    out_sources.push(sym);
                }
            }
        }
    }
}

fn record_legacy_indirect_bindings(data: &mut AnalysisData<'_>, roots: &[SymbolId]) {
    if roots.is_empty() {
        return;
    }

    let mut referenced: Vec<SymbolId> = Vec::new();
    for sym in data.scoping.semantics().symbol_ids() {
        if !data.scoping.is_component_top_level_symbol(sym) {
            continue;
        }
        let has_template_reference = data
            .scoping
            .get_resolved_reference_ids(sym)
            .iter()
            .any(|&ref_id| data.scoping.is_template_reference(ref_id));
        if has_template_reference {
            referenced.push(sym);
        }
    }

    for &root in roots {
        for &sym in &referenced {
            if sym != root {
                data.reactivity.add_legacy_indirect_binding(root, sym);
            }
        }
    }
}

fn collect_each_body_inner_mutation<'a>(
    block: &EachBlock,
    ctx: &VisitContext<'_, 'a>,
    parsed: &JsAst<'a>,
    out: &mut bool,
) {
    if *out {
        return;
    }
    walk_fragment_const_tags(ctx.store, block.body, parsed, ctx, out);
}

fn walk_fragment_const_tags<'a>(
    store: &svelte_ast::AstStore,
    fragment_id: svelte_ast::FragmentId,
    parsed: &JsAst<'a>,
    ctx: &VisitContext<'_, 'a>,
    out: &mut bool,
) {
    if *out {
        return;
    }
    let fragment = store.fragment(fragment_id);
    for child_id in fragment.nodes.iter().copied() {
        if *out {
            return;
        }
        match store.get(child_id) {
            svelte_ast::Node::ConstTag(tag) => {
                let Some(stmt) = parsed.stmt(tag.decl.id()) else {
                    continue;
                };
                let Some(declarator) = declarator_from_stmt_local(stmt) else {
                    continue;
                };
                svelte_component_semantics::walk_bindings(&declarator.id, |v| {
                    if ctx.data.scoping.is_mutated_any(v.symbol) {
                        *out = true;
                    }
                });
            }
            svelte_ast::Node::IfBlock(b) => {
                walk_fragment_const_tags(store, b.consequent, parsed, ctx, out);
                if let Some(alt) = b.alternate {
                    walk_fragment_const_tags(store, alt, parsed, ctx, out);
                }
            }
            svelte_ast::Node::EachBlock(b) => {
                walk_fragment_const_tags(store, b.body, parsed, ctx, out);
                if let Some(fb) = b.fallback {
                    walk_fragment_const_tags(store, fb, parsed, ctx, out);
                }
            }
            svelte_ast::Node::AwaitBlock(b) => {
                if let Some(p) = b.pending {
                    walk_fragment_const_tags(store, p, parsed, ctx, out);
                }
                if let Some(t) = b.then {
                    walk_fragment_const_tags(store, t, parsed, ctx, out);
                }
                if let Some(c) = b.catch {
                    walk_fragment_const_tags(store, c, parsed, ctx, out);
                }
            }
            svelte_ast::Node::KeyBlock(b) => {
                walk_fragment_const_tags(store, b.fragment, parsed, ctx, out);
            }
            svelte_ast::Node::SnippetBlock(b) => {
                walk_fragment_const_tags(store, b.body, parsed, ctx, out);
            }
            svelte_ast::Node::Element(e) => {
                walk_fragment_const_tags(store, e.fragment, parsed, ctx, out);
            }
            svelte_ast::Node::SvelteElement(e) => {
                walk_fragment_const_tags(store, e.fragment, parsed, ctx, out);
            }
            svelte_ast::Node::ComponentNode(c) => {
                walk_fragment_const_tags(store, c.fragment, parsed, ctx, out);
            }
            svelte_ast::Node::SvelteComponentLegacy(c) => {
                walk_fragment_const_tags(store, c.fragment, parsed, ctx, out);
            }
            svelte_ast::Node::SvelteSelf(c) => {
                walk_fragment_const_tags(store, c.fragment, parsed, ctx, out);
            }
            svelte_ast::Node::SvelteFragmentLegacy(f) => {
                walk_fragment_const_tags(store, f.fragment, parsed, ctx, out);
            }
            svelte_ast::Node::SvelteBoundary(b) => {
                walk_fragment_const_tags(store, b.fragment, parsed, ctx, out);
            }
            svelte_ast::Node::SvelteHead(h) => {
                walk_fragment_const_tags(store, h.fragment, parsed, ctx, out);
            }
            _ => {}
        }
    }
}

struct ExprRefCollector {
    refs: Vec<ReferenceId>,
}

impl<'a> Visit<'a> for ExprRefCollector {
    fn visit_identifier_reference(&mut self, ident: &IdentifierReference<'a>) {
        if let Some(ref_id) = ident.reference_id.get() {
            self.refs.push(ref_id);
        }
    }
}

pub(super) fn expression_reference_ids<'a>(expr: &Expression<'a>) -> Vec<ReferenceId> {
    let mut collector = ExprRefCollector { refs: Vec::new() };
    collector.visit_expression(expr);
    collector.refs
}

pub(super) fn classify_contextual_read_kind(
    data: &AnalysisData,
    sym: SymbolId,
    kind: ContextualBindingSemantics,
    raw_param: bool,
) -> ContextualReadKind {
    let _ = (data, sym);
    match kind {
        ContextualBindingSemantics::EachItem(EachItemStrategy::Accessor) => {
            ContextualReadKind::EachItem {
                accessor: true,
                signal: false,
                raw_param,
            }
        }
        ContextualBindingSemantics::EachItem(
            EachItemStrategy::Direct | EachItemStrategy::IndexedLegacy,
        ) => ContextualReadKind::EachItem {
            accessor: false,
            signal: false,
            raw_param,
        },
        ContextualBindingSemantics::EachItem(EachItemStrategy::Signal) => {
            ContextualReadKind::EachItem {
                accessor: false,
                signal: true,
                raw_param,
            }
        }
        ContextualBindingSemantics::EachIndex(EachIndexStrategy::Direct) => {
            ContextualReadKind::EachIndex {
                signal: false,
                raw_param,
            }
        }
        ContextualBindingSemantics::EachIndex(EachIndexStrategy::Signal) => {
            ContextualReadKind::EachIndex {
                signal: true,
                raw_param,
            }
        }
        ContextualBindingSemantics::AwaitValue => ContextualReadKind::AwaitValue,
        ContextualBindingSemantics::AwaitError => ContextualReadKind::AwaitError,
        ContextualBindingSemantics::LetDirective
        | ContextualBindingSemantics::LetDirectiveCarrierMember { .. } => {
            ContextualReadKind::LetDirective
        }
        ContextualBindingSemantics::LetDirectiveDirect => ContextualReadKind::LetDirectiveDirect,
        ContextualBindingSemantics::SnippetParam(SnippetParamStrategy::Accessor) => {
            ContextualReadKind::SnippetParam {
                accessor: true,
                signal: false,
            }
        }
        ContextualBindingSemantics::SnippetParam(SnippetParamStrategy::Signal) => {
            ContextualReadKind::SnippetParam {
                accessor: false,
                signal: true,
            }
        }
    }
}

fn run_each_context_marker<'a>(
    block: &EachBlock,
    ctx: &mut VisitContext<'_, 'a>,
    staging: &mut ContextualStaging,
    is_destructured: bool,
) {
    let Some(parsed) = ctx.parsed else { return };
    let Some(stmt) = block.context.as_ref().and_then(|r| parsed.stmt(r.id())) else {
        return;
    };
    let Some(declarator) = declarator_from_stmt_local(stmt) else {
        return;
    };
    let owner = block.id;
    svelte_component_semantics::walk_bindings(&declarator.id, |v| {
        ctx.data.reactivity.record_contextual_owner(v.symbol, owner);
        staging.push(v.symbol, PendingKind::EachItem);
        if !is_destructured {
            return;
        }
        let has_default = v.path.iter().any(|step| step.default.is_some());
        if v.is_rest {
            ctx.data.reactivity.mark_each_rest(v.symbol);
            if !has_default {
                staging.mark_getter(v.symbol);
            }
        } else if has_default {
            staging.mark_signal_read(v.symbol);
        } else {
            staging.mark_getter(v.symbol);
        }
    });
}

fn run_each_index_marker<'a>(
    block: &EachBlock,
    ctx: &mut VisitContext<'_, 'a>,
    staging: &mut ContextualStaging,
) {
    let Some(parsed) = ctx.parsed else { return };
    let Some(stmt) = block.index.as_ref().and_then(|r| parsed.stmt(r.id())) else {
        return;
    };
    let idx_name = declarator_from_stmt_local(stmt)
        .and_then(|d| d.id.get_binding_identifier())
        .map(|ident| ident.name.as_str());
    let key_is_index = match (block.key.as_ref(), idx_name) {
        (Some(r), Some(name)) => parsed
            .expr(r.id())
            .is_some_and(|expr| matches!(expr.get_inner_expression(), Expression::Identifier(ident) if ident.name.as_str() == name)),
        _ => false,
    };
    let mut marker = EachIndexMarker {
        data: ctx.data,
        staging,
        owner_node: block.id,
        mark_non_reactive: block.key.is_none() || key_is_index,
    };
    marker.visit_statement(stmt);
}

fn run_snippet_param_marker<'a>(
    block: &SnippetBlock,
    ctx: &mut VisitContext<'_, 'a>,
    staging: &mut ContextualStaging,
) {
    let Some(parsed) = ctx.parsed else { return };
    let Some(stmt) = parsed.stmt(block.decl.id()) else {
        return;
    };
    let mut marker = SnippetParamMarker {
        data: ctx.data,
        staging,
        owner_node: block.id,
        in_default: false,
    };
    marker.visit_statement(stmt);
}

fn each_collection_has_external_deps(
    block: &EachBlock,
    body_scope: ScopeId,
    ctx: &VisitContext<'_, '_>,
) -> bool {
    let Some(parsed) = ctx.parsed else {
        return true;
    };
    let Some(expr) = parsed.expr(block.expression.id()) else {
        return true;
    };
    let each_depth = ctx.data.scoping.function_depth(body_scope) + 1;
    let mut collector = ExprRefCollector { refs: Vec::new() };
    collector.visit_expression(expr);
    for ref_id in collector.refs {
        let sym = ctx
            .data
            .scoping
            .get_reference(ref_id)
            .symbol_id()
            .or_else(|| match ctx.data.reactivity.reference_semantics(ref_id) {
                ReferenceSemantics::StoreRead { symbol }
                | ReferenceSemantics::StoreWrite { symbol }
                | ReferenceSemantics::StoreUpdate { symbol } => Some(symbol),
                _ => None,
            });
        let Some(sym) = sym else {
            continue;
        };
        let decl_scope = ctx.data.scoping.symbol_scope_id(sym);
        if ctx.data.scoping.function_depth(decl_scope) < each_depth {
            return true;
        }
    }
    false
}

fn each_collection_uses_store(block: &EachBlock, ctx: &VisitContext<'_, '_>) -> bool {
    let Some(parsed) = ctx.parsed else {
        return false;
    };
    let Some(expr) = parsed.expr(block.expression.id()) else {
        return false;
    };
    let mut collector = ExprRefCollector { refs: Vec::new() };
    collector.visit_expression(expr);
    collector.refs.iter().any(|&ref_id| {
        matches!(
            ctx.data.reactivity.reference_semantics(ref_id),
            ReferenceSemantics::StoreRead { .. }
                | ReferenceSemantics::StoreWrite { .. }
                | ReferenceSemantics::StoreUpdate { .. }
        )
    })
}

fn mark_each_item_syms_non_reactive(
    block: &EachBlock,
    ctx: &VisitContext<'_, '_>,
    staging: &mut ContextualStaging,
) {
    let Some(parsed) = ctx.parsed else { return };
    let Some(stmt) = block.context.as_ref().and_then(|r| parsed.stmt(r.id())) else {
        return;
    };
    let Some(declarator) = declarator_from_stmt_local(stmt) else {
        return;
    };
    svelte_component_semantics::walk_bindings(&declarator.id, |v| {
        staging.mark_each_non_reactive(v.symbol);
    });
}

fn mark_key_is_item_each_binding(
    block: &EachBlock,
    body_scope: ScopeId,
    _key_span: svelte_span::Span,
    ctx: &mut VisitContext<'_, '_>,
    staging: &mut ContextualStaging,
) {
    let Some(parsed) = ctx.parsed else { return };
    let Some(declarator) = block
        .context
        .as_ref()
        .and_then(|r| parsed.stmt(r.id()))
        .and_then(declarator_from_stmt_local)
    else {
        return;
    };
    let Some(ident_name) = declarator
        .id
        .get_binding_identifier()
        .map(|i| i.name.as_str())
    else {
        return;
    };
    let Some(ctx_sym) = ctx.data.scoping.get_binding(body_scope, ident_name) else {
        return;
    };
    let Some(key_ref) = block.key.as_ref() else {
        return;
    };
    let key_resolves_to_ctx = parsed
        .expr(key_ref.id())
        .and_then(|expr| match expr.get_inner_expression() {
            Expression::Identifier(ident) => ident.reference_id.get(),
            _ => None,
        })
        .and_then(|ref_id| ctx.data.scoping.get_reference(ref_id).symbol_id())
        .is_some_and(|sym| sym == ctx_sym);
    if key_resolves_to_ctx {
        staging.mark_each_non_reactive(ctx_sym);
    }
}

fn declarator_from_stmt_local<'a>(stmt: &'a Statement<'a>) -> Option<&'a VariableDeclarator<'a>> {
    match stmt {
        Statement::VariableDeclaration(decl) => decl.declarations.first(),
        _ => None,
    }
}

fn destructure_pattern_node(pattern: &BindingPattern<'_>) -> Option<OxcNodeId> {
    match pattern {
        BindingPattern::ObjectPattern(p) => Some(p.node_id()),
        BindingPattern::ArrayPattern(p) => Some(p.node_id()),
        _ => None,
    }
}

fn await_destructure_node(stmt: &Statement<'_>) -> Option<OxcNodeId> {
    let decl = declarator_from_stmt_local(stmt)?;
    destructure_pattern_node(&decl.id)
}

struct EachIndexMarker<'d, 's, 'a> {
    data: &'d mut AnalysisData<'a>,
    staging: &'s mut ContextualStaging,
    owner_node: NodeId,
    mark_non_reactive: bool,
}

impl<'a> Visit<'a> for EachIndexMarker<'_, '_, 'a> {
    fn visit_binding_identifier(&mut self, ident: &BindingIdentifier<'a>) {
        let Some(sym_id) = ident.symbol_id.get() else {
            return;
        };
        self.data
            .reactivity
            .record_contextual_owner(sym_id, self.owner_node);
        self.staging.push(sym_id, PendingKind::EachIndex);
        if self.mark_non_reactive {
            self.staging.mark_each_non_reactive(sym_id);
        }
    }

    fn visit_variable_declarator(&mut self, decl: &VariableDeclarator<'a>) {
        self.visit_binding_pattern(&decl.id);
    }
}

struct SnippetParamMarker<'d, 's, 'a> {
    data: &'d mut AnalysisData<'a>,
    staging: &'s mut ContextualStaging,
    owner_node: NodeId,
    in_default: bool,
}

impl<'a> Visit<'a> for SnippetParamMarker<'_, '_, 'a> {
    fn visit_binding_identifier(&mut self, ident: &BindingIdentifier<'a>) {
        let Some(sym_id) = ident.symbol_id.get() else {
            return;
        };
        self.data
            .reactivity
            .record_contextual_owner(sym_id, self.owner_node);
        self.staging.push(sym_id, PendingKind::SnippetParam);
        if !self.in_default {
            self.staging.mark_getter(sym_id);
        }
    }

    fn visit_assignment_pattern(&mut self, pat: &AssignmentPattern<'a>) {
        let was_in_default = self.in_default;
        self.in_default = true;
        self.visit_binding_pattern(&pat.left);
        self.in_default = was_in_default;
    }

    fn visit_formal_parameter(&mut self, param: &FormalParameter<'a>) {
        let was_in_default = self.in_default;
        if param.initializer.is_some() {
            self.in_default = true;
        }
        self.visit_binding_pattern(&param.pattern);
        self.in_default = was_in_default;
    }

    fn visit_variable_declarator(&mut self, decl: &VariableDeclarator<'a>) {
        if let Some(init) = &decl.init {
            self.visit_expression(init);
        }
    }

    fn visit_arrow_function_expression(&mut self, arrow: &ArrowFunctionExpression<'a>) {
        for param in &arrow.params.items {
            let pattern = match &param.pattern {
                BindingPattern::AssignmentPattern(assign) => &assign.left,
                other => other,
            };
            if let Some(node) = destructure_pattern_node(pattern) {
                self.data
                    .reactivity
                    .record_declarator_semantics(node, DeclaratorSemantics::SnippetParam);
            }
        }
        self.visit_formal_parameters(&arrow.params);
    }
}
