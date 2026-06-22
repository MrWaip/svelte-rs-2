use oxc_ast::AstKind;
use oxc_ast::ast::{
    ArrowFunctionExpression, AssignmentTarget, Declaration, Expression, Function,
    IdentifierReference, LabeledStatement, ModuleExportName, Program, Statement,
    VariableDeclarationKind,
};
use oxc_semantic::{ReferenceId, SymbolId};
use oxc_span::{GetSpan, Span as OxcSpan};
use oxc_syntax::scope::ScopeFlags;
use rustc_hash::FxHashSet;
use svelte_component_semantics::OxcNodeId;
use svelte_diagnostics::{Diagnostic, DiagnosticKind};
use svelte_span::Span;

use crate::AnalysisData;

pub(super) fn validate_legacy_diagnostics(
    data: &AnalysisData,
    program: &Program<'_>,
    runes: bool,
    diags: &mut Vec<Diagnostic>,
) {
    if runes {
        validate_legacy_export_invalid(program, diags);
        validate_legacy_props_invalid(data, diags);
        validate_legacy_rest_props_invalid(data, diags);
    } else {
        validate_export_let_unused(data, program, diags);
        validate_reactive_declaration_invalid_placement(program, diags);
        validate_reactive_declaration_cycle(data, diags);
        validate_reactive_declaration_module_script_dependency(data, program, diags);
    }
}

fn validate_reactive_declaration_invalid_placement(
    program: &Program<'_>,
    diags: &mut Vec<Diagnostic>,
) {
    use oxc_ast_visit::Visit;
    use oxc_ast_visit::walk::{
        walk_arrow_function_expression, walk_function, walk_labeled_statement,
    };
    struct Visitor<'a> {
        diags: &'a mut Vec<Diagnostic>,
        depth: u32,
    }
    impl<'v, 'a> Visit<'a> for Visitor<'v> {
        fn visit_function(&mut self, func: &Function<'a>, flags: ScopeFlags) {
            self.depth += 1;
            walk_function(self, func, flags);
            self.depth -= 1;
        }
        fn visit_arrow_function_expression(&mut self, arrow: &ArrowFunctionExpression<'a>) {
            self.depth += 1;
            walk_arrow_function_expression(self, arrow);
            self.depth -= 1;
        }
        fn visit_labeled_statement(&mut self, stmt: &LabeledStatement<'a>) {
            if self.depth > 0 && stmt.label.name == "$" {
                self.diags.push(Diagnostic::warning(
                    DiagnosticKind::ReactiveDeclarationInvalidPlacement,
                    Span::new(stmt.span.start, stmt.span.end),
                ));
            }
            walk_labeled_statement(self, stmt);
        }
    }
    let mut v = Visitor { diags, depth: 0 };
    v.visit_program(program);
}

fn validate_reactive_declaration_cycle(data: &AnalysisData, diags: &mut Vec<Diagnostic>) {
    let Some(cycle) = data.reactivity.legacy_reactive().cycle_path() else {
        return;
    };
    let Some(stmt_node) = cycle.first().copied() else {
        return;
    };
    let labeled_first = match data.scoping.js_kind(stmt_node) {
        Some(AstKind::LabeledStatement(l)) => l,
        _ => return,
    };
    let span = labeled_first.span();
    let names: Vec<String> = cycle
        .iter()
        .filter_map(|node_id| {
            let labeled = match data.scoping.js_kind(*node_id)? {
                AstKind::LabeledStatement(l) => l,
                _ => return None,
            };
            let Statement::ExpressionStatement(es) = &labeled.body else {
                return None;
            };
            let Expression::AssignmentExpression(assign) = es.expression.get_inner_expression()
            else {
                return None;
            };
            match &assign.left {
                AssignmentTarget::AssignmentTargetIdentifier(id) => {
                    Some(id.name.as_str().to_string())
                }
                _ => None,
            }
        })
        .collect();
    let cycle_text = if names.is_empty() {
        "<cycle>".into()
    } else {
        format!("{} → {}", names.join(" → "), names[0])
    };
    diags.push(Diagnostic::error(
        DiagnosticKind::ReactiveDeclarationCycle { cycle: cycle_text },
        Span::new(span.start, span.end),
    ));
}

fn validate_reactive_declaration_module_script_dependency(
    data: &AnalysisData,
    program: &Program<'_>,
    diags: &mut Vec<Diagnostic>,
) {
    let Some(module_scope) = data.scoping.module_scope_id() else {
        return;
    };
    let stmt_nodes: Vec<_> = data
        .reactivity
        .legacy_reactive()
        .iter_statements_topo()
        .map(|s| s.stmt_node)
        .collect();
    for stmt_node in stmt_nodes {
        let Some(AstKind::LabeledStatement(labeled)) = data.scoping.js_kind(stmt_node) else {
            continue;
        };
        for (ref_id, span) in collect_reference_sites(&labeled.body) {
            let Some(dep_sym) = data.scoping.symbol_for_reference(ref_id) else {
                continue;
            };
            if data.scoping.symbol_scope_id(dep_sym) != module_scope {
                continue;
            }
            if !data.scoping.is_mutated_any(dep_sym) {
                continue;
            }
            diags.push(Diagnostic::warning(
                DiagnosticKind::ReactiveDeclarationModuleScriptDependency,
                span,
            ));
            break;
        }
    }
    let _ = program;
}

fn collect_reference_sites(body: &Statement<'_>) -> Vec<(ReferenceId, Span)> {
    use oxc_ast_visit::Visit;
    struct Finder {
        out: Vec<(ReferenceId, Span)>,
    }
    impl<'b> Visit<'b> for Finder {
        fn visit_identifier_reference(&mut self, id: &IdentifierReference<'b>) {
            if let Some(ref_id) = id.reference_id.get() {
                self.out
                    .push((ref_id, Span::new(id.span.start, id.span.end)));
            }
        }
    }
    let mut f = Finder { out: Vec::new() };
    f.visit_statement(body);
    f.out
}

fn validate_legacy_export_invalid(program: &Program<'_>, diags: &mut Vec<Diagnostic>) {
    for stmt in &program.body {
        let Statement::ExportNamedDeclaration(export) = stmt else {
            continue;
        };
        let Some(Declaration::VariableDeclaration(var_decl)) = &export.declaration else {
            continue;
        };
        if !matches!(var_decl.kind, VariableDeclarationKind::Let) {
            continue;
        }
        diags.push(Diagnostic::error(
            DiagnosticKind::LegacyExportInvalid,
            Span::new(export.span.start, export.span.end),
        ));
    }
}

fn validate_legacy_props_invalid(data: &AnalysisData, diags: &mut Vec<Diagnostic>) {
    emit_first_unresolved_read(data, "$$props", diags, DiagnosticKind::LegacyPropsInvalid);
}

fn validate_legacy_rest_props_invalid(data: &AnalysisData, diags: &mut Vec<Diagnostic>) {
    emit_first_unresolved_read(
        data,
        "$$restProps",
        diags,
        DiagnosticKind::LegacyRestPropsInvalid,
    );
}

fn emit_first_unresolved_read(
    data: &AnalysisData,
    name: &str,
    diags: &mut Vec<Diagnostic>,
    kind: DiagnosticKind,
) {
    let unresolved = data.scoping.root_unresolved_references();
    let Some(refs) = unresolved.get(name) else {
        return;
    };
    let Some(&ref_id) = refs.first() else {
        return;
    };
    let node_id = data.scoping.get_reference(ref_id).node_id();
    let Some(span) = identifier_reference_span(data, node_id) else {
        return;
    };
    diags.push(Diagnostic::error(kind, Span::new(span.start, span.end)));
}

fn identifier_reference_span(data: &AnalysisData, node_id: OxcNodeId) -> Option<OxcSpan> {
    match data.scoping.js_kind(node_id)? {
        AstKind::IdentifierReference(id) => Some(id.span),
        _ => None,
    }
}

fn validate_export_let_unused(
    data: &AnalysisData,
    program: &Program<'_>,
    diags: &mut Vec<Diagnostic>,
) {
    let symbols: Vec<SymbolId> = data.reactivity.legacy_bindable_prop_symbols().to_vec();
    let export_specifier_refs = collect_export_specifier_refs(program);
    for sym in symbols {
        let decl_node = data.scoping.symbol_declaration(sym);
        if has_companion_store(data, sym) {
            continue;
        }
        if has_non_export_read(data, sym, decl_node, &export_specifier_refs) {
            continue;
        }
        let span = data.scoping.symbol_span(sym);
        let name = data.scoping.symbol_name(sym).to_string();
        diags.push(Diagnostic::warning(
            DiagnosticKind::ExportLetUnused { name },
            Span::new(span.start, span.end),
        ));
    }
}

fn collect_export_specifier_refs(program: &Program<'_>) -> FxHashSet<ReferenceId> {
    let mut out = FxHashSet::default();
    for stmt in &program.body {
        let Statement::ExportNamedDeclaration(export) = stmt else {
            continue;
        };
        for spec in &export.specifiers {
            if let ModuleExportName::IdentifierReference(IdentifierReference {
                reference_id, ..
            }) = &spec.local
                && let Some(id) = reference_id.get()
            {
                out.insert(id);
            }
        }
    }
    out
}

fn has_companion_store(data: &AnalysisData, sym: SymbolId) -> bool {
    let name = data.scoping.symbol_name(sym);
    let companion = format!("${name}");
    data.scoping
        .find_binding(data.scoping.root_scope_id(), &companion)
        .is_some()
}

fn has_non_export_read(
    data: &AnalysisData,
    sym: SymbolId,
    decl_node: OxcNodeId,
    export_specifier_refs: &FxHashSet<ReferenceId>,
) -> bool {
    for &ref_id in data.scoping.get_resolved_reference_ids(sym) {
        if export_specifier_refs.contains(&ref_id) {
            continue;
        }
        if data.scoping.get_reference(ref_id).node_id() == decl_node {
            continue;
        }
        return true;
    }
    false
}
