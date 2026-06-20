use compact_str::CompactString;
use oxc_ast::AstKind;
use oxc_ast::ast::{
    AssignmentExpression, AssignmentOperator, AssignmentTarget, AssignmentTargetMaybeDefault,
    AssignmentTargetProperty, Expression, IdentifierReference, ImportDeclarationSpecifier,
    LabeledStatement, Program, SimpleAssignmentTarget, Statement, SwitchCase, TSAsExpression,
    TSInstantiationExpression, TSSatisfiesExpression, TSType, TSTypeAnnotation, TSTypeAssertion,
    TSTypeParameterInstantiation, UpdateExpression,
};
use oxc_ast_visit::Visit;
use oxc_ast_visit::walk::{walk_assignment_expression, walk_update_expression};
use oxc_syntax::scope::ScopeId;
use rustc_hash::{FxHashMap, FxHashSet};
use smallvec::SmallVec;
use svelte_component_semantics::{
    OxcNodeId, ReferenceId, SymbolOwner, walk_assignment_target_idents,
};

use super::super::data::{BindingSemantics, LegacyStateSemantics, ReferenceFacts};
use super::super::legacy_reactive::{LegacyReactiveKind, LegacyReactiveStatement};
use crate::scope::SymbolId;
use crate::types::data::AnalysisData;

pub(super) fn collect_top_level_meta<'a>(
    stmt: &Statement<'a>,
    data: &AnalysisData<'a>,
    labeled_nodes: &mut Vec<OxcNodeId>,
    implicit_names: &mut Vec<CompactString>,
    mutated_imports: &mut SmallVec<[SymbolId; 2]>,
) {
    let Some(instance_scope) = data.scoping.instance_scope_id() else {
        return;
    };
    match stmt {
        Statement::ImportDeclaration(import) => {
            let Some(specifiers) = &import.specifiers else {
                return;
            };
            for spec in specifiers {
                let local_name = match spec {
                    ImportDeclarationSpecifier::ImportSpecifier(s) => s.local.name.as_str(),
                    ImportDeclarationSpecifier::ImportDefaultSpecifier(s) => s.local.name.as_str(),
                    ImportDeclarationSpecifier::ImportNamespaceSpecifier(s) => {
                        s.local.name.as_str()
                    }
                };
                let Some(sym) = data.scoping.find_binding(instance_scope, local_name) else {
                    continue;
                };
                if !data.scoping.is_import(sym) {
                    continue;
                }
                if data.scoping.is_mutated_any(sym) || data.scoping.is_member_mutated(sym) {
                    mutated_imports.push(sym);
                }
            }
        }
        Statement::LabeledStatement(labeled) if labeled.label.name == "$" => {
            labeled_nodes.push(labeled.node_id());
            let Statement::ExpressionStatement(es) = &labeled.body else {
                return;
            };
            let Some(assign) = unwrap_assignment_expression(&es.expression) else {
                return;
            };
            if !matches!(assign.operator, AssignmentOperator::Assign) {
                return;
            }
            if matches!(
                &assign.left,
                AssignmentTarget::AssignmentTargetIdentifier(_)
            ) {
                if let AssignmentTarget::AssignmentTargetIdentifier(id) = &assign.left {
                    push_implicit_name(implicit_names, id.name.as_str());
                }
            } else {
                let mut leaves: SmallVec<[&str; 4]> = SmallVec::new();
                walk_assignment_target_idents(&assign.left, |id| {
                    leaves.push(id.name.as_str());
                });
                for name in leaves {
                    push_implicit_name(implicit_names, name);
                }
            }
        }
        _ => {}
    }
}

fn push_implicit_name(out: &mut Vec<CompactString>, name: &str) {
    if name.starts_with('$') {
        return;
    }
    let cs = CompactString::from(name);
    if !out.contains(&cs) {
        out.push(cs);
    }
}

pub(super) fn collect_reactive_body_symbol_refs<'a>(
    program: &Program<'a>,
    data: &AnalysisData<'a>,
) -> FxHashSet<SymbolId> {
    let mut collector = ReactiveBodyRefCollector {
        data,
        symbols: FxHashSet::default(),
    };
    for stmt in &program.body {
        if let Statement::LabeledStatement(labeled) = stmt
            && labeled.label.name == "$"
        {
            collector.visit_statement(&labeled.body);
        }
    }
    collector.symbols
}

struct ReactiveBodyRefCollector<'d, 'a> {
    data: &'d AnalysisData<'a>,
    symbols: FxHashSet<SymbolId>,
}

impl<'a> Visit<'a> for ReactiveBodyRefCollector<'_, 'a> {
    fn visit_identifier_reference(&mut self, ident: &IdentifierReference<'a>) {
        let Some(ref_id) = ident.reference_id.get() else {
            return;
        };
        if let Some(sym) = self.data.scoping.symbol_for_reference(ref_id) {
            self.symbols.insert(sym);
        }
    }

    fn visit_ts_type(&mut self, _it: &TSType<'a>) {}
    fn visit_ts_type_annotation(&mut self, _it: &TSTypeAnnotation<'a>) {}
    fn visit_ts_type_parameter_instantiation(&mut self, _it: &TSTypeParameterInstantiation<'a>) {}
}

pub(super) fn build_from_collected<'a>(
    data: &mut AnalysisData<'a>,
    labeled_nodes: Vec<OxcNodeId>,
    implicit_names: Vec<CompactString>,
) {
    if data.script.runes() {
        return;
    }
    if labeled_nodes.is_empty() {
        return;
    }

    let implicit_map = resolve_implicit_bindings(data, &implicit_names);
    mark_implicit_reactive_locals(data, &implicit_map);
    record_implicit_state_bindings(data, &implicit_map);

    let mut statements: Vec<LegacyReactiveStatement> = Vec::with_capacity(labeled_nodes.len());
    for node_id in &labeled_nodes {
        let Some(AstKind::LabeledStatement(labeled)) = data.scoping.js_kind(*node_id) else {
            continue;
        };
        let prelim = classify_statement(labeled);
        statements.push(build_statement(labeled, &prelim, &implicit_map, data));
    }

    let (order, cycle) = topological_sort(&statements);

    let lr = data.reactivity.legacy_reactive_mut();
    for stmt in statements {
        lr.record_statement(stmt);
    }
    lr.set_topo_order(order);
    if let Some(cycle_path) = cycle {
        lr.set_cycle_path(cycle_path);
    }
}

fn resolve_implicit_bindings(
    data: &AnalysisData<'_>,
    names: &[CompactString],
) -> FxHashMap<CompactString, SymbolId> {
    let mut map: FxHashMap<CompactString, SymbolId> = FxHashMap::default();
    let Some(instance_scope) = data.scoping.instance_scope_id() else {
        return map;
    };
    for name in names {
        add_implicit_binding(&mut map, data, instance_scope, name.as_str());
    }
    map
}

pub(super) fn classify_mutated_import_references(data: &mut AnalysisData<'_>) {
    let imports: Vec<SymbolId> = data
        .reactivity
        .legacy_reactive()
        .iter_mutated_imports()
        .collect();
    if imports.is_empty() {
        return;
    }
    for sym in imports {
        let ref_ids: Vec<_> = data.scoping.get_resolved_reference_ids(sym).to_vec();
        for ref_id in ref_ids {
            if data
                .reactivity
                .reference_semantics(ref_id)
                .is_store_subscription()
            {
                continue;
            }
            let is_member_mutation_root = data.reactivity.is_prop_member_mutation_root_ref(ref_id);
            let reference = data.scoping.get_reference(ref_id);
            let fact = if is_member_mutation_root {
                ReferenceFacts::LegacyReactiveImportMemberMutationRoot { symbol: sym }
            } else if reference.is_read() {
                ReferenceFacts::LegacyReactiveImportRead
            } else {
                continue;
            };
            data.reactivity.record_reference_semantics(ref_id, fact);
        }
    }
}

fn collect_destructure_target_syms(
    labeled: &LabeledStatement<'_>,
    implicit_map: &FxHashMap<CompactString, SymbolId>,
    data: &AnalysisData<'_>,
) -> (SmallVec<[SymbolId; 4]>, SmallVec<[SymbolId; 4]>) {
    let mut targets: SmallVec<[SymbolId; 4]> = SmallVec::new();
    let mut implicits: SmallVec<[SymbolId; 4]> = SmallVec::new();
    let Statement::ExpressionStatement(es) = &labeled.body else {
        return (targets, implicits);
    };
    let Some(assign) = unwrap_assignment_expression(&es.expression) else {
        return (targets, implicits);
    };
    let mut leaves: SmallVec<[&IdentifierReference<'_>; 4]> = SmallVec::new();
    walk_assignment_target_idents(&assign.left, |id| {
        leaves.push(id);
    });
    for id in leaves {
        let name = id.name.as_str();
        let Some(sym) = id
            .reference_id
            .get()
            .and_then(|r| data.scoping.symbol_for_reference(r))
            .or_else(|| implicit_map.get(name).copied())
            .or_else(|| data.scoping.find_binding_in_any_scope(name))
        else {
            continue;
        };
        targets.push(sym);
        if implicit_map.contains_key(name) {
            implicits.push(sym);
        }
    }
    (targets, implicits)
}

fn unwrap_assignment_expression<'r, 'a>(
    expr: &'r Expression<'a>,
) -> Option<&'r AssignmentExpression<'a>> {
    match expr.get_inner_expression() {
        Expression::AssignmentExpression(assign) => Some(assign),
        _ => None,
    }
}

fn add_implicit_binding(
    map: &mut FxHashMap<CompactString, SymbolId>,
    data: &AnalysisData<'_>,
    instance_scope: ScopeId,
    name: &str,
) {
    let Some(sym) = data.scoping.find_binding(instance_scope, name) else {
        return;
    };
    if !matches!(data.scoping.symbol_owner(sym), SymbolOwner::Synthetic) {
        return;
    }
    map.entry(CompactString::from(name)).or_insert(sym);
}

fn mark_implicit_reactive_locals(
    data: &mut AnalysisData<'_>,
    implicit_map: &FxHashMap<CompactString, SymbolId>,
) {
    let lr = data.reactivity.legacy_reactive_mut();
    for &sym in implicit_map.values() {
        lr.mark_implicit_reactive_local(sym);
    }
}

fn record_implicit_state_bindings(
    data: &mut AnalysisData<'_>,
    implicit_map: &FxHashMap<CompactString, SymbolId>,
) {
    let immutable = data.script.immutable;
    for &sym in implicit_map.values() {
        if data.reactivity.binding_facts(sym).is_some() {
            continue;
        }
        data.reactivity.record_legacy_state_binding(
            sym,
            LegacyStateSemantics {
                var_declared: false,
                immutable,
            },
        );
    }
}

struct Prelim {
    shape: PrelimKind,
}

enum PrelimKind {
    SimpleAssignmentIdent {
        target_name: CompactString,
        target_ref_id: Option<ReferenceId>,
    },
    DestructureAssignment,

    Block,

    Conditional,

    ExpressionOnly,
}

fn classify_statement(labeled: &LabeledStatement<'_>) -> Prelim {
    match &labeled.body {
        Statement::ExpressionStatement(es) => {
            if let Some(assign) = unwrap_assignment_expression(&es.expression)
                && matches!(assign.operator, AssignmentOperator::Assign)
            {
                match &assign.left {
                    AssignmentTarget::AssignmentTargetIdentifier(id) => {
                        let name = CompactString::from(id.name.as_str());
                        return Prelim {
                            shape: PrelimKind::SimpleAssignmentIdent {
                                target_name: name,
                                target_ref_id: id.reference_id.get(),
                            },
                        };
                    }
                    AssignmentTarget::ArrayAssignmentTarget(_)
                    | AssignmentTarget::ObjectAssignmentTarget(_) => {
                        return Prelim {
                            shape: PrelimKind::DestructureAssignment,
                        };
                    }
                    _ => {}
                }
            }
            Prelim {
                shape: PrelimKind::ExpressionOnly,
            }
        }
        Statement::BlockStatement(_)
        | Statement::ForStatement(_)
        | Statement::ForOfStatement(_)
        | Statement::ForInStatement(_)
        | Statement::WhileStatement(_)
        | Statement::DoWhileStatement(_)
        | Statement::TryStatement(_) => Prelim {
            shape: PrelimKind::Block,
        },
        Statement::IfStatement(_) | Statement::SwitchStatement(_) => Prelim {
            shape: PrelimKind::Conditional,
        },
        _ => Prelim {
            shape: PrelimKind::ExpressionOnly,
        },
    }
}

fn build_statement<'a>(
    labeled: &LabeledStatement<'a>,
    prelim: &Prelim,
    implicit_map: &FxHashMap<CompactString, SymbolId>,
    data: &AnalysisData<'a>,
) -> LegacyReactiveStatement {
    let kind = match &prelim.shape {
        PrelimKind::SimpleAssignmentIdent {
            target_name,
            target_ref_id,
        } => {
            let target_sym = target_ref_id
                .and_then(|r| data.scoping.symbol_for_reference(r))
                .or_else(|| implicit_map.get(target_name).copied())
                .or_else(|| data.scoping.find_binding_in_any_scope(target_name.as_str()));
            match target_sym {
                Some(sym) => LegacyReactiveKind::SimpleAssignment {
                    target_sym: sym,
                    implicit_decl: implicit_map.contains_key(target_name),
                },
                None => LegacyReactiveKind::ExpressionOnly,
            }
        }
        PrelimKind::DestructureAssignment => {
            let (target_syms, implicit_decl_syms) =
                collect_destructure_target_syms(labeled, implicit_map, data);
            LegacyReactiveKind::DestructureAssignment {
                target_syms,
                implicit_decl_syms,
            }
        }
        PrelimKind::Block => LegacyReactiveKind::Block,
        PrelimKind::Conditional => LegacyReactiveKind::Conditional,
        PrelimKind::ExpressionOnly => LegacyReactiveKind::ExpressionOnly,
    };

    let mut analyzer = LegacyBodyAnalyzer {
        data,
        implicit_map,
        assignments: SmallVec::new(),
        dependencies: SmallVec::new(),
        seen_assignments: FxHashSet::default(),
        seen_deps: FxHashSet::default(),
        read_deps: FxHashSet::default(),
        structural_reads: SmallVec::new(),
        seen_structural: FxHashSet::default(),
        direct_assign_skip: FxHashSet::default(),
        uses_props: false,
        uses_rest_props: false,
    };
    analyzer.visit_statement(&labeled.body);
    let read_deps = analyzer.read_deps;
    analyzer.dependencies.retain(|s| read_deps.contains(s));
    LegacyReactiveStatement {
        stmt_node: labeled.node_id(),
        kind,
        assignments: analyzer.assignments,
        dependencies: analyzer.dependencies,
        structural_reads: analyzer.structural_reads,
        uses_props: analyzer.uses_props,
        uses_rest_props: analyzer.uses_rest_props,
    }
}

struct LegacyBodyAnalyzer<'d, 'a> {
    data: &'d AnalysisData<'a>,
    implicit_map: &'d FxHashMap<CompactString, SymbolId>,
    assignments: SmallVec<[SymbolId; 4]>,
    dependencies: SmallVec<[SymbolId; 8]>,
    seen_assignments: FxHashSet<SymbolId>,
    seen_deps: FxHashSet<SymbolId>,
    read_deps: FxHashSet<SymbolId>,
    structural_reads: SmallVec<[SymbolId; 8]>,
    seen_structural: FxHashSet<SymbolId>,
    direct_assign_skip: FxHashSet<ReferenceId>,
    uses_props: bool,
    uses_rest_props: bool,
}

impl<'a> LegacyBodyAnalyzer<'_, 'a> {
    fn record_assignment_target(&mut self, target: &AssignmentTarget<'_>) {
        match target {
            AssignmentTarget::AssignmentTargetIdentifier(id) => {
                self.record_assignment_ident(id);
            }
            AssignmentTarget::StaticMemberExpression(m) => {
                self.record_member_root(&m.object);
            }
            AssignmentTarget::ComputedMemberExpression(m) => {
                self.record_member_root(&m.object);
            }
            AssignmentTarget::ObjectAssignmentTarget(obj) => {
                for prop in &obj.properties {
                    match prop {
                        AssignmentTargetProperty::AssignmentTargetPropertyIdentifier(shorthand) => {
                            self.record_assignment_ident(&shorthand.binding)
                        }
                        AssignmentTargetProperty::AssignmentTargetPropertyProperty(kv) => {
                            self.record_assignment_maybe_default(&kv.binding)
                        }
                    }
                }
                if let Some(rest) = &obj.rest {
                    self.record_assignment_target(&rest.target);
                }
            }
            AssignmentTarget::ArrayAssignmentTarget(arr) => {
                for elem in arr.elements.iter().flatten() {
                    self.record_assignment_maybe_default(elem);
                }
                if let Some(rest) = &arr.rest {
                    self.record_assignment_target(&rest.target);
                }
            }
            _ => {}
        }
    }

    fn record_assignment_maybe_default(&mut self, target: &AssignmentTargetMaybeDefault<'_>) {
        match target {
            AssignmentTargetMaybeDefault::AssignmentTargetWithDefault(with_def) => {
                self.record_assignment_target(&with_def.binding);
            }
            other => {
                if let Some(at) = other.as_assignment_target() {
                    self.record_assignment_target(at);
                }
            }
        }
    }

    fn record_simple_assignment_target(&mut self, target: &SimpleAssignmentTarget<'_>) {
        match target {
            SimpleAssignmentTarget::AssignmentTargetIdentifier(id) => {
                self.record_assignment_ident(id);
            }
            SimpleAssignmentTarget::StaticMemberExpression(m) => {
                self.record_member_root(&m.object);
            }
            SimpleAssignmentTarget::ComputedMemberExpression(m) => {
                self.record_member_root(&m.object);
            }
            _ => {}
        }
    }

    fn record_member_root(&mut self, expr: &Expression<'_>) {
        match expr.get_inner_expression() {
            Expression::Identifier(id) => self.record_member_root_dep(id),
            Expression::StaticMemberExpression(m) => self.record_member_root(&m.object),
            Expression::ComputedMemberExpression(m) => self.record_member_root(&m.object),
            _ => {}
        }
    }

    fn record_member_root_dep(&mut self, ident: &IdentifierReference<'_>) {
        let name = ident.name.as_str();
        let ref_id = ident.reference_id.get();
        let sym = ref_id
            .and_then(|r| self.subscribed_store_symbol(r))
            .or_else(|| ref_id.and_then(|r| self.data.scoping.symbol_for_reference(r)))
            .or_else(|| self.implicit_map.get(name).copied());
        let Some(sym) = sym else {
            return;
        };
        if !self.is_reactive_dep(sym) {
            return;
        }
        if self.seen_deps.insert(sym) {
            self.dependencies.push(sym);
        }
    }

    fn record_assignment_ident(&mut self, ident: &IdentifierReference<'_>) {
        let name = ident.name.as_str();
        let ref_id = ident.reference_id.get();
        let subscribed_store = ref_id.and_then(|r| self.subscribed_store_symbol(r));
        let sym = subscribed_store
            .or_else(|| ref_id.and_then(|r| self.data.scoping.symbol_for_reference(r)))
            .or_else(|| self.implicit_map.get(name).copied());
        if let Some(sym) = sym {
            if self.seen_assignments.insert(sym) {
                self.assignments.push(sym);
            }
            if self.is_reactive_dep(sym) && self.seen_deps.insert(sym) {
                self.dependencies.push(sym);
            }
        }
    }

    fn subscribed_store_symbol(&self, ref_id: ReferenceId) -> Option<SymbolId> {
        self.data
            .reactivity
            .reference_facts(ref_id)?
            .subscribed_store_symbol()
    }

    fn is_reactive_dep(&self, sym: SymbolId) -> bool {
        is_reactive_legacy_dep(self.data.reactivity.binding_semantics(sym))
    }
}

pub(super) fn is_reactive_legacy_dep(semantics: BindingSemantics) -> bool {
    match semantics {
        BindingSemantics::NonReactive
        | BindingSemantics::Unresolved
        | BindingSemantics::LegacyApiExport => false,
        BindingSemantics::MaybeReactive
        | BindingSemantics::State(_)
        | BindingSemantics::Derived(_)
        | BindingSemantics::OptimizedDerived(_)
        | BindingSemantics::OptimizedRune(_)
        | BindingSemantics::Prop(_)
        | BindingSemantics::LegacyBindableProp(_)
        | BindingSemantics::LegacyState(_)
        | BindingSemantics::Store(_)
        | BindingSemantics::Const(_)
        | BindingSemantics::Contextual(_)
        | BindingSemantics::RuntimeRune { .. } => true,
    }
}

impl<'a> Visit<'a> for LegacyBodyAnalyzer<'_, 'a> {
    fn visit_assignment_expression(&mut self, expr: &AssignmentExpression<'a>) {
        self.record_assignment_target(&expr.left);
        if matches!(expr.operator, AssignmentOperator::Assign) {
            collect_direct_assign_lhs_ref(&expr.left, &mut self.direct_assign_skip);
        }
        walk_assignment_expression(self, expr);
    }

    fn visit_update_expression(&mut self, expr: &UpdateExpression<'a>) {
        self.record_simple_assignment_target(&expr.argument);
        walk_update_expression(self, expr);
    }

    fn visit_identifier_reference(&mut self, ident: &IdentifierReference<'a>) {
        let Some(ref_id) = ident.reference_id.get() else {
            return;
        };
        if self.direct_assign_skip.contains(&ref_id) {
            return;
        }
        let store_sym = self.subscribed_store_symbol(ref_id);
        let sym = store_sym
            .or_else(|| self.data.scoping.symbol_for_reference(ref_id))
            .or_else(|| self.implicit_map.get(ident.name.as_str()).copied());
        let Some(sym) = sym else {
            match ident.name.as_str() {
                "$$props" => self.uses_props = true,
                "$$restProps" => self.uses_rest_props = true,
                _ => {}
            }
            return;
        };
        if self.data.scoping.is_component_top_level_symbol(sym) && self.seen_structural.insert(sym)
        {
            self.structural_reads.push(sym);
        }
        if !self.is_reactive_dep(sym) {
            return;
        }
        if self.seen_deps.insert(sym) {
            self.dependencies.push(sym);
        }
        self.read_deps.insert(sym);
    }

    fn visit_switch_case(&mut self, case: &SwitchCase<'a>) {
        self.visit_statements(&case.consequent);
        if let Some(test) = &case.test {
            self.visit_expression(test);
        }
    }

    fn visit_ts_type(&mut self, _it: &TSType<'a>) {}

    fn visit_ts_type_annotation(&mut self, _it: &TSTypeAnnotation<'a>) {}

    fn visit_ts_type_parameter_instantiation(&mut self, _it: &TSTypeParameterInstantiation<'a>) {}

    fn visit_ts_as_expression(&mut self, it: &TSAsExpression<'a>) {
        self.visit_expression(&it.expression);
    }

    fn visit_ts_satisfies_expression(&mut self, it: &TSSatisfiesExpression<'a>) {
        self.visit_expression(&it.expression);
    }

    fn visit_ts_type_assertion(&mut self, it: &TSTypeAssertion<'a>) {
        self.visit_expression(&it.expression);
    }

    fn visit_ts_instantiation_expression(&mut self, it: &TSInstantiationExpression<'a>) {
        self.visit_expression(&it.expression);
    }
}

fn collect_direct_assign_lhs_ref(
    target: &AssignmentTarget<'_>,
    skips: &mut FxHashSet<ReferenceId>,
) {
    match target {
        AssignmentTarget::AssignmentTargetIdentifier(id) => {
            if let Some(ref_id) = id.reference_id.get() {
                skips.insert(ref_id);
            }
        }
        AssignmentTarget::StaticMemberExpression(m) => {
            collect_member_root_ref(&m.object, skips);
        }
        AssignmentTarget::ComputedMemberExpression(m) => {
            collect_member_root_ref(&m.object, skips);
        }
        _ => {}
    }
}

fn collect_member_root_ref(expr: &Expression<'_>, skips: &mut FxHashSet<ReferenceId>) {
    match expr.get_inner_expression() {
        Expression::Identifier(id) => {
            if let Some(ref_id) = id.reference_id.get() {
                skips.insert(ref_id);
            }
        }
        Expression::StaticMemberExpression(m) => collect_member_root_ref(&m.object, skips),
        Expression::ComputedMemberExpression(m) => collect_member_root_ref(&m.object, skips),
        _ => {}
    }
}

fn topological_sort(
    statements: &[LegacyReactiveStatement],
) -> (SmallVec<[OxcNodeId; 4]>, Option<SmallVec<[OxcNodeId; 4]>>) {
    let mut by_assignment: FxHashMap<SymbolId, SmallVec<[usize; 2]>> = FxHashMap::default();
    for (idx, stmt) in statements.iter().enumerate() {
        for &sym in &stmt.assignments {
            by_assignment.entry(sym).or_default().push(idx);
        }
    }

    let mut order: SmallVec<[OxcNodeId; 4]> = SmallVec::new();
    let mut visiting: Vec<usize> = Vec::new();
    let mut visiting_set: FxHashSet<usize> = FxHashSet::default();
    let mut visited: FxHashSet<usize> = FxHashSet::default();
    let mut cycle: Option<SmallVec<[OxcNodeId; 4]>> = None;

    for start in 0..statements.len() {
        visit_node(
            start,
            statements,
            &by_assignment,
            &mut visiting,
            &mut visiting_set,
            &mut visited,
            &mut order,
            &mut cycle,
        );
    }
    (order, cycle)
}

fn visit_node(
    idx: usize,
    statements: &[LegacyReactiveStatement],
    by_assignment: &FxHashMap<SymbolId, SmallVec<[usize; 2]>>,
    visiting: &mut Vec<usize>,
    visiting_set: &mut FxHashSet<usize>,
    visited: &mut FxHashSet<usize>,
    order: &mut SmallVec<[OxcNodeId; 4]>,
    cycle: &mut Option<SmallVec<[OxcNodeId; 4]>>,
) {
    if visited.contains(&idx) {
        return;
    }
    if visiting_set.contains(&idx) {
        if cycle.is_none() {
            let start = visiting.iter().position(|&v| v == idx).unwrap_or(0);
            let path: SmallVec<[OxcNodeId; 4]> = visiting[start..]
                .iter()
                .map(|&i| statements[i].stmt_node)
                .collect();
            *cycle = Some(path);
        }
        return;
    }
    visiting.push(idx);
    visiting_set.insert(idx);
    let stmt = &statements[idx];
    for &dep_sym in &stmt.dependencies {
        if stmt.assignments.contains(&dep_sym) {
            continue;
        }
        if let Some(producers) = by_assignment.get(&dep_sym) {
            for &producer in producers {
                if producer == idx {
                    continue;
                }
                visit_node(
                    producer,
                    statements,
                    by_assignment,
                    visiting,
                    visiting_set,
                    visited,
                    order,
                    cycle,
                );
            }
        }
    }
    visiting.pop();
    visiting_set.remove(&idx);
    if visited.insert(idx) {
        order.push(stmt.stmt_node);
    }
}
