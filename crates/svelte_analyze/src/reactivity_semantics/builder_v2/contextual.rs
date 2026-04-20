//! Cluster C5: template-contextual declarations (each / snippet / await / let:)
//! and the reference-read classification rules that follow from them.
//!
//! Flow:
//! 1. Walk template, mark raw facts in builder-local `ContextualStaging`
//!    (getter / non-reactive / key-is-item / rest / snippet-name / template-decl).
//! 2. Finalize: convert staged facts into `ContextualDeclarationSemantics`
//!    variants with strategy payloads, write them to `ReactivitySemantics`.
//!
//! Consumers see only the final declaration shape — no `is_getter` /
//! `is_each_non_reactive` lookups required.

use oxc_ast::ast::{ArrowFunctionExpression, BindingIdentifier, Statement, VariableDeclarator};
use oxc_ast_visit::Visit;
use rustc_hash::FxHashSet;
use svelte_ast::{AwaitBlock, Component, EachBlock, LetDirectiveLegacy, NodeId, SnippetBlock};
use svelte_component_semantics::OxcNodeId;

use super::super::data::{
    ContextualDeclarationSemantics, ContextualReadKind, EachIndexStrategy, EachItemStrategy,
    SnippetParamStrategy, V2DeclarationFacts,
};
use crate::scope::SymbolId;
use crate::types::data::{AnalysisData, FragmentKey, ParserResult, StmtHandle};
use crate::utils::legacy_slot::legacy_slot_pattern;
use crate::walker::{walk_template, TemplateVisitor, VisitContext};

/// Pending contextual declaration kind, recorded during the template walk.
/// Final strategy is computed during finalization from `ContextualStaging`.
#[derive(Clone, Copy)]
enum PendingKind {
    EachItem,
    EachIndex,
    AwaitValue,
    AwaitError,
    LetDirective,
    SnippetParam,
}

/// Builder-local staging: raw classification facts collected during the
/// template walk. Finalization turns them into `ContextualDeclarationSemantics`
/// variants with strategy payloads.
#[derive(Default)]
struct ContextualStaging {
    pending: Vec<(SymbolId, OxcNodeId, PendingKind)>,
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
    fn push(&mut self, sym: SymbolId, node_id: OxcNodeId, kind: PendingKind) {
        self.pending.push((sym, node_id, kind));
    }
}

pub(super) fn collect_template_declarations<'a>(
    component: &Component,
    parsed: &ParserResult<'a>,
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
    walk_template(&component.fragment, &mut ctx, &mut visitors);

    finalize_contextual_declarations(data, staging);
}

fn finalize_contextual_declarations(data: &mut AnalysisData<'_>, staging: ContextualStaging) {
    let ContextualStaging {
        pending,
        getter_symbols,
        each_non_reactive_symbols,
    } = staging;
    for (sym, node_id, kind) in pending {
        let semantics = match kind {
            PendingKind::EachItem => {
                let strategy = if getter_symbols.contains(&sym) {
                    EachItemStrategy::Accessor
                } else if each_non_reactive_symbols.contains(&sym) {
                    EachItemStrategy::Direct
                } else {
                    EachItemStrategy::Signal
                };
                ContextualDeclarationSemantics::EachItem(strategy)
            }
            PendingKind::EachIndex => {
                let strategy = if each_non_reactive_symbols.contains(&sym) {
                    EachIndexStrategy::Direct
                } else {
                    EachIndexStrategy::Signal
                };
                ContextualDeclarationSemantics::EachIndex(strategy)
            }
            PendingKind::AwaitValue => ContextualDeclarationSemantics::AwaitValue,
            PendingKind::AwaitError => ContextualDeclarationSemantics::AwaitError,
            PendingKind::LetDirective => ContextualDeclarationSemantics::LetDirective,
            PendingKind::SnippetParam => {
                let strategy = if getter_symbols.contains(&sym) {
                    SnippetParamStrategy::Accessor
                } else {
                    SnippetParamStrategy::Signal
                };
                ContextualDeclarationSemantics::SnippetParam(strategy)
            }
        };
        let _ = sym;
        data.reactivity
            .record_contextual_declaration_v2(node_id, semantics);
    }
}

struct TemplateDeclarationCollector<'s> {
    staging: &'s mut ContextualStaging,
}

impl TemplateVisitor for TemplateDeclarationCollector<'_> {
    fn visit_const_tag(&mut self, tag: &svelte_ast::ConstTag, ctx: &mut VisitContext<'_, '_>) {
        // Resolve binding leaves locally from the pre-parsed
        // `{@const <pattern> = ...}` statement — one walk of the
        // binding pattern plus a `find_binding` per leaf, against the
        // enclosing fragment scope carried by `ctx.scope`. This used
        // to read `const_tags.syms(tag.id)` which was populated by a
        // separate pass; moving the derivation in-line drops that
        // side-table and lets the ConstTagData cluster shrink to
        // `by_fragment` only.
        let syms: Vec<SymbolId> = collect_const_tag_syms(tag, ctx);

        // Only destructured const-tags (`{@const { a, b } = ...}`) carry a
        // per-leaf owner — single-identifier const-tags don't need it because
        // consumers don't drill into them.
        let is_destructured = syms.len() > 1;

        for sym in syms {
            let node_id = ctx.data.scoping.symbol_declaration(sym);
            ctx.data
                .reactivity
                .record_symbol_declaration_root(sym, node_id);
            ctx.data
                .reactivity
                .record_const_declaration_v2(node_id, is_destructured);
            if is_destructured {
                ctx.data.reactivity.record_const_alias_owner_v2(sym, tag.id);
            }
        }
    }

    fn visit_let_directive_legacy(
        &mut self,
        dir: &LetDirectiveLegacy,
        ctx: &mut VisitContext<'_, '_>,
    ) {
        // Take the bindings snapshot first so the parsed borrow is released
        // before we mutate `ctx.data` (carrier synthesis needs `&mut`).
        let (syms, is_destructured, stmt_node_id) = {
            let Some(stmt) = ctx
                .parsed()
                .and_then(|parsed| parsed.stmt_handle(dir.name_span.start))
                .and_then(|handle| ctx.parsed().and_then(|parsed| parsed.stmt(handle)))
            else {
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
                !matches!(pattern, oxc_ast::ast::BindingPattern::BindingIdentifier(_));
            let mut syms: Vec<svelte_component_semantics::SymbolId> = Vec::new();
            svelte_component_semantics::walk_bindings(pattern, |v| syms.push(v.symbol));
            (syms, is_destructured, stmt_node_id)
        };

        // v2 owns carrier synthesis for destructured `let:` forms: one
        // synthesized symbol per directive, shared by all destructure leaves.
        // The carrier declaration is keyed by the destructuring statement's
        // OxcNodeId so consumers query it via `declaration_semantics`.
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
            let node_id = ctx.data.scoping.symbol_declaration(sym);
            ctx.data
                .reactivity
                .record_symbol_declaration_root(sym, node_id);
            ctx.data.reactivity.record_contextual_owner_v2(sym, dir.id);
            if let Some(carrier) = carrier_sym {
                // Destructured leaf — consumers read the alias relation to
                // route reads through the synthesized carrier.
                ctx.data
                    .reactivity
                    .record_carrier_alias_declaration_v2(node_id, carrier);
            } else {
                // Root `BindingIdentifier` — direct staging as if the
                // directive bound a single plain name.
                self.staging.push(sym, node_id, PendingKind::LetDirective);
            }
        }
    }

    fn visit_each_block(&mut self, block: &EachBlock, ctx: &mut VisitContext<'_, '_>) {
        let body_scope = ctx.child_scope(FragmentKey::EachBody(block.id), ctx.scope);
        let is_destructured = ctx.data.each_is_destructured(block.id);

        run_each_context_marker(block, ctx, self.staging, is_destructured);

        // Single-identifier context with a key expression that resolves to
        // the context symbol itself (`{#each items as item (item)}`) —
        // the each binding is not reactive: reads go through plain lookup.
        if !is_destructured {
            if let Some(key_span) = block.key_span {
                mark_key_is_item_each_binding(block, body_scope, key_span, ctx, self.staging);
            }
        }

        run_each_index_marker(block, ctx, self.staging);
    }

    fn visit_snippet_block(&mut self, block: &SnippetBlock, ctx: &mut VisitContext<'_, '_>) {
        run_snippet_param_marker(block, ctx, self.staging);
    }

    fn visit_await_block(&mut self, block: &AwaitBlock, ctx: &mut VisitContext<'_, '_>) {
        let then_scope = ctx.child_scope(FragmentKey::AwaitThen(block.id), ctx.scope);
        if let Some(stmt) = block.value_span.and_then(|span| {
            ctx.parsed()
                .and_then(|parsed| parsed.stmt_handle(span.start))
                .and_then(|handle| ctx.parsed().and_then(|parsed| parsed.stmt(handle)))
        }) {
            for sym in scoped_stmt_symbols(ctx.data, then_scope, stmt) {
                let node_id = ctx.data.scoping.symbol_declaration(sym);
                ctx.data
                    .reactivity
                    .record_symbol_declaration_root(sym, node_id);
                ctx.data
                    .reactivity
                    .record_contextual_owner_v2(sym, block.id);
                self.staging.push(sym, node_id, PendingKind::AwaitValue);
            }
        }

        let catch_scope = ctx.child_scope(FragmentKey::AwaitCatch(block.id), ctx.scope);
        if let Some(stmt) = block.error_span.and_then(|span| {
            ctx.parsed()
                .and_then(|parsed| parsed.stmt_handle(span.start))
                .and_then(|handle| ctx.parsed().and_then(|parsed| parsed.stmt(handle)))
        }) {
            for sym in scoped_stmt_symbols(ctx.data, catch_scope, stmt) {
                let node_id = ctx.data.scoping.symbol_declaration(sym);
                ctx.data
                    .reactivity
                    .record_symbol_declaration_root(sym, node_id);
                ctx.data
                    .reactivity
                    .record_contextual_owner_v2(sym, block.id);
                self.staging.push(sym, node_id, PendingKind::AwaitError);
            }
        }
    }
}

fn scoped_stmt_symbols(
    _data: &AnalysisData,
    _scope: oxc_semantic::ScopeId,
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

/// Synthesize (or reuse) the carrier symbol for a destructured `let:`
/// directive. Shared by all destructure leaves of the same directive.
/// The carrier is recorded as `DeclarationSemantics::LetCarrier` on the
/// destructuring statement's `OxcNodeId`, so codegen reads it through the
/// declaration API.
fn ensure_slot_let_carrier(
    data: &mut AnalysisData,
    scope: crate::scope::ScopeId,
    stmt_node_id: OxcNodeId,
    preferred_name: &str,
) -> SymbolId {
    if let Some(V2DeclarationFacts::LetCarrier { carrier_symbol }) =
        data.reactivity.declaration_facts_v2(stmt_node_id)
    {
        return carrier_symbol;
    }
    let sym = data
        .scoping
        .add_unique_synthetic_binding(scope, preferred_name);
    data.reactivity
        .record_let_carrier_declaration_v2(stmt_node_id, sym);
    sym
}

/// Derive the contextual read wrap for one symbol from the finalized
/// `ContextualDeclarationSemantics` on that symbol's declaration. Used by
/// the reference classifier to emit `ContextualRead { kind, .. }`.
pub(super) fn classify_contextual_read_kind(
    data: &AnalysisData,
    sym: SymbolId,
    kind: ContextualDeclarationSemantics,
) -> ContextualReadKind {
    let _ = (data, sym);
    match kind {
        ContextualDeclarationSemantics::EachItem(EachItemStrategy::Accessor) => {
            ContextualReadKind::EachItem {
                accessor: true,
                signal: false,
            }
        }
        ContextualDeclarationSemantics::EachItem(EachItemStrategy::Direct) => {
            ContextualReadKind::EachItem {
                accessor: false,
                signal: false,
            }
        }
        ContextualDeclarationSemantics::EachItem(EachItemStrategy::Signal) => {
            ContextualReadKind::EachItem {
                accessor: false,
                signal: true,
            }
        }
        ContextualDeclarationSemantics::EachIndex(EachIndexStrategy::Direct) => {
            ContextualReadKind::EachIndex { signal: false }
        }
        ContextualDeclarationSemantics::EachIndex(EachIndexStrategy::Signal) => {
            ContextualReadKind::EachIndex { signal: true }
        }
        ContextualDeclarationSemantics::AwaitValue => ContextualReadKind::AwaitValue,
        ContextualDeclarationSemantics::AwaitError => ContextualReadKind::AwaitError,
        ContextualDeclarationSemantics::LetDirective => ContextualReadKind::LetDirective,
        ContextualDeclarationSemantics::SnippetParam(SnippetParamStrategy::Accessor) => {
            ContextualReadKind::SnippetParam {
                accessor: true,
                signal: false,
            }
        }
        ContextualDeclarationSemantics::SnippetParam(SnippetParamStrategy::Signal) => {
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
    let Some(stmt) = block
        .context_span
        .and_then(|span| parsed.stmt_handle(span.start))
        .and_then(|handle| parsed.stmt(handle))
    else {
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
    let Some(stmt) = block
        .index_span
        .and_then(|span| parsed.stmt_handle(span.start))
        .and_then(|handle| parsed.stmt(handle))
    else {
        return;
    };
    let mut marker = EachIndexMarker {
        data: ctx.data,
        staging,
        owner_node: block.id,
        mark_non_reactive: block.key_span.is_none(),
    };
    marker.visit_statement(stmt);
}

fn run_snippet_param_marker<'a>(
    block: &SnippetBlock,
    ctx: &mut VisitContext<'_, 'a>,
    staging: &mut ContextualStaging,
) {
    let Some(parsed) = ctx.parsed else { return };
    let Some(stmt) = parsed
        .stmt_handle(block.expression_span.start)
        .and_then(|handle| parsed.stmt(handle))
    else {
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

fn mark_key_is_item_each_binding(
    block: &EachBlock,
    body_scope: oxc_semantic::ScopeId,
    key_span: svelte_span::Span,
    ctx: &mut VisitContext<'_, '_>,
    staging: &mut ContextualStaging,
) {
    let Some(parsed) = ctx.parsed else { return };
    let Some(declarator) = block
        .context_span
        .and_then(|span| parsed.stmt_handle(span.start))
        .and_then(|h| get_declarator(parsed, h))
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
    let key_resolves_to_ctx = parsed
        .expr_handle(key_span.start)
        .and_then(|h| parsed.expr(h))
        .and_then(|expr| match expr {
            oxc_ast::ast::Expression::Identifier(ident) => ident.reference_id.get(),
            _ => None,
        })
        .and_then(|ref_id| ctx.data.scoping.get_reference(ref_id).symbol_id())
        .is_some_and(|sym| sym == ctx_sym);
    if key_resolves_to_ctx {
        staging.mark_each_non_reactive(ctx_sym);
    }
}

fn get_declarator<'a>(
    parsed: &'a ParserResult<'a>,
    handle: StmtHandle,
) -> Option<&'a VariableDeclarator<'a>> {
    parsed.stmt(handle).and_then(|stmt| match stmt {
        Statement::VariableDeclaration(decl) => decl.declarations.first(),
        _ => None,
    })
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
        let node_id = self.data.scoping.symbol_declaration(sym);
        self.data
            .reactivity
            .record_symbol_declaration_root(sym, node_id);
        self.data
            .reactivity
            .record_contextual_owner_v2(sym, self.owner_node);
        self.staging.push(sym, node_id, PendingKind::EachItem);
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

    fn visit_binding_rest_element(&mut self, it: &oxc_ast::ast::BindingRestElement<'a>) {
        let Some(ident) = it.argument.get_binding_identifier() else {
            return;
        };
        let Some(sym_id) = ident.symbol_id.get() else {
            return;
        };
        self.record_declaration(sym_id);
        if self.classify_leaves {
            self.data.reactivity.mark_each_rest(sym_id);
        }
    }

    fn visit_assignment_pattern(&mut self, pat: &oxc_ast::ast::AssignmentPattern<'a>) {
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
        let node_id = self.data.scoping.symbol_declaration(sym_id);
        self.data
            .reactivity
            .record_symbol_declaration_root(sym_id, node_id);
        self.data
            .reactivity
            .record_contextual_owner_v2(sym_id, self.owner_node);
        self.staging.push(sym_id, node_id, PendingKind::EachIndex);
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
        let node_id = self.data.scoping.symbol_declaration(sym_id);
        self.data
            .reactivity
            .record_symbol_declaration_root(sym_id, node_id);
        self.data
            .reactivity
            .record_contextual_owner_v2(sym_id, self.owner_node);
        self.staging
            .push(sym_id, node_id, PendingKind::SnippetParam);
        if !self.in_default {
            self.staging.mark_getter(sym_id);
        }
    }

    fn visit_assignment_pattern(&mut self, pat: &oxc_ast::ast::AssignmentPattern<'a>) {
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

/// Resolve the leaf `SymbolId`s introduced by a `{@const <pattern> = ...}`
/// tag. The binding pattern is walked over the pre-parsed statement and
/// each identifier name is looked up in the enclosing fragment's scope.
/// Used by `visit_const_tag` to avoid holding a precomputed side-table
/// (`ConstTagData::syms`) just for this consumer.
fn collect_const_tag_syms(
    tag: &svelte_ast::ConstTag,
    ctx: &mut VisitContext<'_, '_>,
) -> Vec<SymbolId> {
    let Some(parsed) = ctx.parsed() else {
        return Vec::new();
    };
    let Some(stmt) = parsed
        .stmt_handle(tag.expression_span.start)
        .and_then(|handle| parsed.stmt(handle))
    else {
        return Vec::new();
    };
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
