use oxc_ast::ast::{
    ArrowFunctionExpression, AssignmentPattern, BindingIdentifier, BindingPattern,
    BindingRestElement, Expression, IdentifierReference, Statement, VariableDeclarator,
};
use oxc_ast_visit::Visit;
use oxc_semantic::ScopeId;
use rustc_hash::FxHashSet;
use svelte_ast::{AwaitBlock, Component, EachBlock, LetDirectiveLegacy, NodeId, SnippetBlock};
use svelte_component_semantics::{OxcNodeId, ReferenceId};

use super::super::data::{
    ContextualBindingSemantics, ContextualReadKind, DeclaratorSemantics, EachIndexStrategy,
    EachItemStrategy, LegacyStateSemantics, ReferenceSemantics, SnippetParamStrategy,
};
use crate::scope::SymbolId;
use crate::types::data::{AnalysisData, JsAst};
use crate::utils::legacy_slot::legacy_slot_pattern;
use crate::walker::{TemplateVisitor, VisitContext, walk_template};

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
}

impl ContextualStaging {
    fn mark_getter(&mut self, sym: SymbolId) {
        self.getter_symbols.insert(sym);
    }
    fn mark_each_non_reactive(&mut self, sym: SymbolId) {
        self.each_non_reactive_symbols.insert(sym);
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
    let component_name = data.output.component_name.clone();
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
    } = staging;
    for (sym, kind) in pending {
        let semantics = match kind {
            PendingKind::EachItem => {
                let strategy = if getter_symbols.contains(&sym) {
                    EachItemStrategy::Accessor
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
        let is_destructured = !matches!(
            declarator.id,
            BindingPattern::BindingIdentifier(_),
        );
        let initial_is_function = !is_destructured
            && matches!(
                declarator.init.as_ref().map(|e| e.get_inner_expression()),
                Some(
                    Expression::ArrowFunctionExpression(_) | Expression::FunctionExpression(_),
                ),
            );
        let mut syms: Vec<SymbolId> = Vec::new();
        svelte_component_semantics::walk_bindings(&declarator.id, |v| syms.push(v.symbol));

        for sym in syms.iter().copied() {
            ctx.data
                .reactivity
                .record_const_binding(sym, is_destructured, initial_is_function, tag.id);
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
            let is_destructured =
                !matches!(pattern, BindingPattern::BindingIdentifier(_));
            let mut syms: Vec<SymbolId> = Vec::new();
            svelte_component_semantics::walk_bindings(pattern, |v| syms.push(v.symbol));
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
            None
        };

        for sym in syms {
            ctx.data.reactivity.record_contextual_owner(sym, dir.id);
            if let Some(carrier) = carrier_sym {
                ctx.data
                    .reactivity
                    .record_carrier_alias_binding(sym, carrier);
            } else {
                self.staging.push(sym, PendingKind::LetDirective);
            }
        }
    }

    fn visit_each_block(&mut self, block: &EachBlock, ctx: &mut VisitContext<'_, '_>) {
        let body_scope = ctx.child_scope_by_id(block.body, ctx.scope);
        let is_destructured = block
            .context
            .as_ref()
            .and_then(|r| ctx.parsed().and_then(|p| p.stmt(r.id())))
            .and_then(declarator_from_stmt_local)
            .is_some_and(|d| !matches!(&d.id, BindingPattern::BindingIdentifier(_)));

        run_each_context_marker(block, ctx, self.staging, is_destructured);

        if ctx.data.script.runes()
            && !is_destructured
            && let Some(key_ref) = block.key.as_ref()
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
    if let DeclaratorSemantics::LetCarrier { carrier_symbol } =
        data.reactivity.declarator_semantics(stmt_node_id)
    {
        return carrier_symbol;
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
    let component_name = data.output.component_name.clone();
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
        collect_each_body_inner_mutation(
            block,
            ctx,
            parsed,
            &mut inner_mutated_bindings,
        );

        if !inner_mutated_bindings
            && !item_syms
                .iter()
                .any(|&sym| ctx.data.scoping.is_mutated_any(sym))
        {
            return;
        }

        let Some(expr) = parsed.expr(block.expression.id()) else {
            return;
        };

        let mut collector = ExprRefCollector { refs: Vec::new() };
        collector.visit_expression(expr);

        let immutable = ctx.data.script.immutable;
        let store_candidate_refs: FxHashSet<ReferenceId> =
            ctx.data
                .scoping
                .store_candidate_refs()
                .iter()
                .map(|(_, ref_id)| *ref_id)
                .collect();
        let mut promoted_sources: Vec<SymbolId> = Vec::new();
        let mut collection_store: Option<SymbolId> = None;
        for ref_id in collector.refs {
            if store_candidate_refs.contains(&ref_id) {
                let Some(base_sym) = ctx.data.scoping.get_reference(ref_id).symbol_id() else {
                    continue;
                };
                if let Some(store_sym) = ctx.data.reactivity.store_shadow_of_internal(base_sym) {
                    if collection_store.is_none() {
                        collection_store = Some(store_sym);
                    }
                    if !runes {
                        promoted_sources.push(store_sym);
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
            if !ctx.data.scoping.is_component_top_level_symbol(sym) {
                continue;
            }
            if ctx.data.reactivity.binding_facts(sym).is_some() {
                continue;
            }
            if ctx.data.reactivity.store_shadow_of_internal(sym).is_some() {
                continue;
            }
            ctx.data.reactivity.record_legacy_state_binding(
                sym,
                LegacyStateSemantics {
                    var_declared: false,
                    immutable,
                },
            );
            promoted_sources.push(sym);
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

pub(super) fn classify_contextual_read_kind(
    data: &AnalysisData,
    sym: SymbolId,
    kind: ContextualBindingSemantics,
) -> ContextualReadKind {
    let _ = (data, sym);
    match kind {
        ContextualBindingSemantics::EachItem(EachItemStrategy::Accessor) => {
            ContextualReadKind::EachItem {
                accessor: true,
                signal: false,
            }
        }
        ContextualBindingSemantics::EachItem(EachItemStrategy::Direct) => {
            ContextualReadKind::EachItem {
                accessor: false,
                signal: false,
            }
        }
        ContextualBindingSemantics::EachItem(EachItemStrategy::Signal) => {
            ContextualReadKind::EachItem {
                accessor: false,
                signal: true,
            }
        }
        ContextualBindingSemantics::EachIndex(EachIndexStrategy::Direct) => {
            ContextualReadKind::EachIndex { signal: false }
        }
        ContextualBindingSemantics::EachIndex(EachIndexStrategy::Signal) => {
            ContextualReadKind::EachIndex { signal: true }
        }
        ContextualBindingSemantics::AwaitValue => ContextualReadKind::AwaitValue,
        ContextualBindingSemantics::AwaitError => ContextualReadKind::AwaitError,
        ContextualBindingSemantics::LetDirective
        | ContextualBindingSemantics::LetDirectiveCarrierMember { .. } => {
            ContextualReadKind::LetDirective
        }
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
    let mut marker = EachContextMarker {
        data: ctx.data,
        staging,
        owner_node: block.id,
        classify_leaves: is_destructured,
        in_default: false,
    };
    marker.visit_statement(stmt);
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

fn await_destructure_node(stmt: &Statement<'_>) -> Option<OxcNodeId> {
    let decl = declarator_from_stmt_local(stmt)?;
    match &decl.id {
        BindingPattern::ObjectPattern(p) => Some(p.node_id()),
        BindingPattern::ArrayPattern(p) => Some(p.node_id()),
        _ => None,
    }
}

struct EachContextMarker<'d, 's, 'a> {
    data: &'d mut AnalysisData<'a>,
    staging: &'s mut ContextualStaging,
    owner_node: NodeId,
    classify_leaves: bool,
    in_default: bool,
}

impl<'a> EachContextMarker<'_, '_, 'a> {
    fn record_declaration(&mut self, sym: SymbolId) {
        self.data
            .reactivity
            .record_contextual_owner(sym, self.owner_node);
        self.staging.push(sym, PendingKind::EachItem);
    }
}

impl<'a> Visit<'a> for EachContextMarker<'_, '_, 'a> {
    fn visit_binding_identifier(&mut self, ident: &BindingIdentifier<'a>) {
        let Some(sym_id) = ident.symbol_id.get() else {
            return;
        };
        self.record_declaration(sym_id);
        if self.classify_leaves && !self.in_default {
            self.staging.mark_getter(sym_id);
        }
    }

    fn visit_binding_rest_element(&mut self, it: &BindingRestElement<'a>) {
        let Some(ident) = it.argument.get_binding_identifier() else {
            self.visit_binding_pattern(&it.argument);
            return;
        };
        let Some(sym_id) = ident.symbol_id.get() else {
            return;
        };
        self.record_declaration(sym_id);
        if self.classify_leaves {
            self.data.reactivity.mark_each_rest(sym_id);
            if !self.in_default {
                self.staging.mark_getter(sym_id);
            }
        }
    }

    fn visit_assignment_pattern(&mut self, pat: &AssignmentPattern<'a>) {
        let was_in_default = self.in_default;
        self.in_default = true;
        self.visit_binding_pattern(&pat.left);
        self.in_default = was_in_default;
    }

    fn visit_variable_declarator(&mut self, decl: &VariableDeclarator<'a>) {
        self.visit_binding_pattern(&decl.id);
    }
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

    fn visit_variable_declarator(&mut self, decl: &VariableDeclarator<'a>) {
        if let Some(init) = &decl.init {
            self.visit_expression(init);
        }
    }

    fn visit_arrow_function_expression(&mut self, arrow: &ArrowFunctionExpression<'a>) {
        self.visit_formal_parameters(&arrow.params);
    }
}

