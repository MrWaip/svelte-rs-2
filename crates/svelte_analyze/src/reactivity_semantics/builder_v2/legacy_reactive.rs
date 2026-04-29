use compact_str::CompactString;
use oxc_ast::ast::{
    AssignmentExpression, AssignmentOperator, AssignmentTarget, Expression, IdentifierReference,
    LabeledStatement, SimpleAssignmentTarget, Statement, UpdateExpression,
};
use oxc_ast_visit::Visit;
use rustc_hash::{FxHashMap, FxHashSet};
use smallvec::SmallVec;
use svelte_component_semantics::OxcNodeId;

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
                    oxc_ast::ast::ImportDeclarationSpecifier::ImportSpecifier(s) => {
                        s.local.name.as_str()
                    }
                    oxc_ast::ast::ImportDeclarationSpecifier::ImportDefaultSpecifier(s) => {
                        s.local.name.as_str()
                    }
                    oxc_ast::ast::ImportDeclarationSpecifier::ImportNamespaceSpecifier(s) => {
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
            match &assign.left {
                AssignmentTarget::AssignmentTargetIdentifier(id) => {
                    push_implicit_name(implicit_names, id.name.as_str());
                }
                AssignmentTarget::ObjectAssignmentTarget(obj) => {
                    for prop in &obj.properties {
                        if let oxc_ast::ast::AssignmentTargetProperty::AssignmentTargetPropertyIdentifier(shorthand) = prop {
                            push_implicit_name(implicit_names, shorthand.binding.name.as_str());
                        }
                    }
                }
                _ => {}
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

pub(super) fn build_from_collected<'a>(
    data: &mut AnalysisData<'a>,
    labeled_nodes: Vec<OxcNodeId>,
    implicit_names: Vec<CompactString>,
    mutated_imports: SmallVec<[SymbolId; 2]>,
) {
    if data.script.runes {
        return;
    }
    if !mutated_imports.is_empty() {
        let lr = data.reactivity.legacy_reactive_mut();
        for sym in &mutated_imports {
            lr.add_mutated_import(*sym);
        }
    }
    if labeled_nodes.is_empty() {
        return;
    }

    let implicit_map = resolve_implicit_bindings(data, &implicit_names);
    mark_implicit_reactive_locals(data, &implicit_map);
    record_implicit_state_bindings(data, &implicit_map);

    let mut statements: Vec<LegacyReactiveStatement> = Vec::with_capacity(labeled_nodes.len());
    for node_id in &labeled_nodes {
        let Some(oxc_ast::AstKind::LabeledStatement(labeled)) = data.scoping.js_kind(*node_id)
        else {
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
    let AssignmentTarget::ObjectAssignmentTarget(obj) = &assign.left else {
        return (targets, implicits);
    };
    for prop in &obj.properties {
        let oxc_ast::ast::AssignmentTargetProperty::AssignmentTargetPropertyIdentifier(shorthand) =
            prop
        else {
            continue;
        };
        let name = shorthand.binding.name.as_str();
        let Some(sym) = shorthand
            .binding
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
    let mut current = expr;
    loop {
        match current {
            Expression::AssignmentExpression(assign) => return Some(assign),
            Expression::ParenthesizedExpression(p) => current = &p.expression,
            _ => return None,
        }
    }
}

fn add_implicit_binding(
    map: &mut FxHashMap<CompactString, SymbolId>,
    data: &AnalysisData<'_>,
    instance_scope: oxc_syntax::scope::ScopeId,
    name: &str,
) {
    let Some(sym) = data.scoping.find_binding(instance_scope, name) else {
        return;
    };
    if !matches!(
        data.scoping.symbol_owner(sym),
        svelte_component_semantics::SymbolOwner::Synthetic
    ) {
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
    shape: PrelimShape,
}

enum PrelimShape {
    SimpleAssignmentIdent {
        target_name: CompactString,
        target_ref_id: Option<svelte_component_semantics::ReferenceId>,
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
                            shape: PrelimShape::SimpleAssignmentIdent {
                                target_name: name,
                                target_ref_id: id.reference_id.get(),
                            },
                        };
                    }
                    AssignmentTarget::ArrayAssignmentTarget(_)
                    | AssignmentTarget::ObjectAssignmentTarget(_) => {
                        return Prelim {
                            shape: PrelimShape::DestructureAssignment,
                        };
                    }
                    _ => {}
                }
            }
            Prelim {
                shape: PrelimShape::ExpressionOnly,
            }
        }
        Statement::BlockStatement(_) => Prelim {
            shape: PrelimShape::Block,
        },
        Statement::IfStatement(_) | Statement::SwitchStatement(_) => Prelim {
            shape: PrelimShape::Conditional,
        },
        _ => Prelim {
            shape: PrelimShape::ExpressionOnly,
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
        PrelimShape::SimpleAssignmentIdent {
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
        PrelimShape::DestructureAssignment => {
            let (target_syms, implicit_decl_syms) =
                collect_destructure_target_syms(labeled, implicit_map, data);
            LegacyReactiveKind::DestructureAssignment {
                target_syms,
                implicit_decl_syms,
            }
        }
        PrelimShape::Block => LegacyReactiveKind::Block,
        PrelimShape::Conditional => LegacyReactiveKind::Conditional,
        PrelimShape::ExpressionOnly => LegacyReactiveKind::ExpressionOnly,
    };

    let mut analyzer = LegacyBodyAnalyzer {
        data,
        implicit_map,
        assignments: SmallVec::new(),
        dependencies: SmallVec::new(),
        seen_assignments: FxHashSet::default(),
        seen_deps: FxHashSet::default(),
        direct_assign_skip: FxHashSet::default(),
        uses_props: false,
        uses_rest_props: false,
    };
    analyzer.visit_statement(&labeled.body);
    LegacyReactiveStatement {
        stmt_node: labeled.node_id(),
        kind,
        assignments: analyzer.assignments,
        dependencies: analyzer.dependencies,
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
    direct_assign_skip: FxHashSet<svelte_component_semantics::ReferenceId>,
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
                        oxc_ast::ast::AssignmentTargetProperty::AssignmentTargetPropertyIdentifier(
                            shorthand,
                        ) => self.record_assignment_ident(&shorthand.binding),
                        oxc_ast::ast::AssignmentTargetProperty::AssignmentTargetPropertyProperty(
                            kv,
                        ) => self.record_assignment_maybe_default(&kv.binding),
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

    fn record_assignment_maybe_default(
        &mut self,
        target: &oxc_ast::ast::AssignmentTargetMaybeDefault<'_>,
    ) {
        match target {
            oxc_ast::ast::AssignmentTargetMaybeDefault::AssignmentTargetWithDefault(with_def) => {
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
        match expr {
            Expression::Identifier(id) => self.record_assignment_ident(id),
            Expression::StaticMemberExpression(m) => self.record_member_root(&m.object),
            Expression::ComputedMemberExpression(m) => self.record_member_root(&m.object),
            _ => {}
        }
    }

    fn record_assignment_ident(&mut self, ident: &IdentifierReference<'_>) {
        let name = ident.name.as_str();
        let sym = ident
            .reference_id
            .get()
            .and_then(|r| self.data.scoping.symbol_for_reference(r))
            .or_else(|| self.implicit_map.get(name).copied());
        if let Some(sym) = sym
            && self.seen_assignments.insert(sym)
        {
            self.assignments.push(sym);
        }
    }

    fn is_reactive_dep(&self, sym: SymbolId) -> bool {
        if self.data.scoping.is_import(sym) {
            return true;
        }
        !matches!(
            self.data.reactivity.binding_semantics(sym),
            BindingSemantics::NonReactive | BindingSemantics::Unresolved
        )
    }
}

impl<'a> Visit<'a> for LegacyBodyAnalyzer<'_, 'a> {
    fn visit_assignment_expression(&mut self, expr: &AssignmentExpression<'a>) {
        self.record_assignment_target(&expr.left);
        if matches!(expr.operator, AssignmentOperator::Assign) {
            collect_direct_assign_lhs_ref(&expr.left, &mut self.direct_assign_skip);
        }
        oxc_ast_visit::walk::walk_assignment_expression(self, expr);
    }

    fn visit_update_expression(&mut self, expr: &UpdateExpression<'a>) {
        self.record_simple_assignment_target(&expr.argument);
        oxc_ast_visit::walk::walk_update_expression(self, expr);
    }

    fn visit_identifier_reference(&mut self, ident: &IdentifierReference<'a>) {
        let Some(ref_id) = ident.reference_id.get() else {
            return;
        };
        if self.direct_assign_skip.contains(&ref_id) {
            return;
        }
        let sym = self
            .data
            .scoping
            .symbol_for_reference(ref_id)
            .or_else(|| self.implicit_map.get(ident.name.as_str()).copied());
        let Some(sym) = sym else {
            match ident.name.as_str() {
                "$$props" => self.uses_props = true,
                "$$restProps" => self.uses_rest_props = true,
                _ => {}
            }
            return;
        };
        if !self.is_reactive_dep(sym) {
            return;
        }
        if self.seen_deps.insert(sym) {
            self.dependencies.push(sym);
        }
    }
}

fn collect_direct_assign_lhs_ref(
    target: &AssignmentTarget<'_>,
    skips: &mut FxHashSet<svelte_component_semantics::ReferenceId>,
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

fn collect_member_root_ref(
    expr: &Expression<'_>,
    skips: &mut FxHashSet<svelte_component_semantics::ReferenceId>,
) {
    match expr {
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
