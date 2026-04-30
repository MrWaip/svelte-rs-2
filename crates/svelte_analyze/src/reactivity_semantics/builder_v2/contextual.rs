use oxc_ast::ast::{ArrowFunctionExpression, BindingIdentifier, Statement, VariableDeclarator};
use oxc_ast_visit::Visit;
use rustc_hash::FxHashSet;
use svelte_ast::{AwaitBlock, Component, EachBlock, LetDirectiveLegacy, NodeId, SnippetBlock};
use svelte_component_semantics::OxcNodeId;

use super::super::data::{
    ContextualBindingSemantics, ContextualReadKind, EachIndexStrategy, EachItemStrategy,
    LegacyStateSemantics, SnippetParamStrategy,
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
        let syms: Vec<SymbolId> = collect_const_tag_syms(tag, ctx);

        let is_destructured = syms.len() > 1;

        for sym in syms.iter().copied() {
            ctx.data
                .reactivity
                .record_const_binding(sym, is_destructured);
            if is_destructured {
                ctx.data.reactivity.record_const_alias_owner(sym, tag.id);
            }
        }
        let _ = syms;
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
                !matches!(pattern, oxc_ast::ast::BindingPattern::BindingIdentifier(_));
            let mut syms: Vec<svelte_component_semantics::SymbolId> = Vec::new();
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
            .is_some_and(|d| !matches!(&d.id, oxc_ast::ast::BindingPattern::BindingIdentifier(_)));

        run_each_context_marker(block, ctx, self.staging, is_destructured);

        if !is_destructured && let Some(key_ref) = block.key.as_ref() {
            mark_key_is_item_each_binding(block, body_scope, key_ref.span, ctx, self.staging);
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
            for sym in scoped_stmt_symbols(ctx.data, then_scope, stmt) {
                ctx.data.reactivity.record_contextual_owner(sym, block.id);
                self.staging.push(sym, PendingKind::AwaitValue);
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
            for sym in scoped_stmt_symbols(ctx.data, catch_scope, stmt) {
                ctx.data.reactivity.record_contextual_owner(sym, block.id);
                self.staging.push(sym, PendingKind::AwaitError);
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

fn ensure_slot_let_carrier(
    data: &mut AnalysisData,
    scope: crate::scope::ScopeId,
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
    if data.script.runes {
        return;
    }
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

        if !item_syms
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
        let mut promoted_sources: Vec<svelte_component_semantics::SymbolId> = Vec::new();
        for ref_id in collector.refs {
            let Some(sym) = ctx.data.scoping.get_reference(ref_id).symbol_id() else {
                continue;
            };
            if !ctx.data.scoping.is_component_top_level_symbol(sym) {
                continue;
            }
            if ctx.data.reactivity.binding_facts(sym).is_some() {
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
    }
}

struct ExprRefCollector {
    refs: Vec<svelte_component_semantics::ReferenceId>,
}

impl<'a> Visit<'a> for ExprRefCollector {
    fn visit_identifier_reference(&mut self, ident: &oxc_ast::ast::IdentifierReference<'a>) {
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
        ContextualBindingSemantics::LetDirective => ContextualReadKind::LetDirective,
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
    let mut marker = EachIndexMarker {
        data: ctx.data,
        staging,
        owner_node: block.id,
        mark_non_reactive: block.key.is_none(),
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

fn mark_key_is_item_each_binding(
    block: &EachBlock,
    body_scope: oxc_semantic::ScopeId,
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

fn declarator_from_stmt_local<'a>(stmt: &'a Statement<'a>) -> Option<&'a VariableDeclarator<'a>> {
    match stmt {
        Statement::VariableDeclaration(decl) => decl.declarations.first(),
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

fn collect_const_tag_syms(
    tag: &svelte_ast::ConstTag,
    ctx: &mut VisitContext<'_, '_>,
) -> Vec<SymbolId> {
    let Some(parsed) = ctx.parsed() else {
        return Vec::new();
    };
    let Some(stmt) = parsed.stmt(tag.decl.id()) else {
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
