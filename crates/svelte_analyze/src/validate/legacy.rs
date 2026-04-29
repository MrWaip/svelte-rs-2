use oxc_ast::AstKind;
use oxc_ast::ast::{
    Declaration, IdentifierReference, ModuleExportName, Program, Statement, VariableDeclarationKind,
};
use oxc_semantic::ReferenceId;
use oxc_span::GetSpan;
use rustc_hash::FxHashSet;
use svelte_component_semantics::OxcNodeId;
use svelte_diagnostics::{Diagnostic, DiagnosticKind};
use svelte_span::Span;

use crate::AnalysisData;

pub(super) fn validate_legacy_diagnostics(
    data: &AnalysisData,
    program: &Program<'_>,
    offset: u32,
    runes: bool,
    diags: &mut Vec<Diagnostic>,
) {
    if runes {
        validate_legacy_export_invalid(program, offset, diags);
        validate_legacy_props_invalid(data, offset, diags);
        validate_legacy_rest_props_invalid(data, offset, diags);
    } else {
        validate_export_let_unused(data, program, offset, diags);
        validate_reactive_declaration_invalid_placement(program, offset, diags);
        validate_reactive_declaration_cycle(data, offset, diags);
        validate_reactive_declaration_module_script_dependency(data, program, offset, diags);
    }
}

fn validate_reactive_declaration_invalid_placement(
    program: &Program<'_>,
    offset: u32,
    diags: &mut Vec<Diagnostic>,
) {
    use oxc_ast_visit::Visit;
    struct Visitor<'a> {
        offset: u32,
        diags: &'a mut Vec<Diagnostic>,
        depth: u32,
    }
    impl<'v, 'a> Visit<'a> for Visitor<'v> {
        fn visit_function(
            &mut self,
            func: &oxc_ast::ast::Function<'a>,
            flags: oxc_syntax::scope::ScopeFlags,
        ) {
            self.depth += 1;
            oxc_ast_visit::walk::walk_function(self, func, flags);
            self.depth -= 1;
        }
        fn visit_arrow_function_expression(
            &mut self,
            arrow: &oxc_ast::ast::ArrowFunctionExpression<'a>,
        ) {
            self.depth += 1;
            oxc_ast_visit::walk::walk_arrow_function_expression(self, arrow);
            self.depth -= 1;
        }
        fn visit_labeled_statement(&mut self, stmt: &oxc_ast::ast::LabeledStatement<'a>) {
            if self.depth > 0 && stmt.label.name == "$" {
                self.diags.push(Diagnostic::warning(
                    DiagnosticKind::ReactiveDeclarationInvalidPlacement,
                    Span::new(stmt.span.start + self.offset, stmt.span.end + self.offset),
                ));
            }
            oxc_ast_visit::walk::walk_labeled_statement(self, stmt);
        }
    }
    let mut v = Visitor {
        offset,
        diags,
        depth: 0,
    };
    v.visit_program(program);
}

fn validate_reactive_declaration_cycle(
    data: &AnalysisData,
    offset: u32,
    diags: &mut Vec<Diagnostic>,
) {
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
            let oxc_ast::ast::Statement::ExpressionStatement(es) = &labeled.body else {
                return None;
            };
            let oxc_ast::ast::Expression::AssignmentExpression(assign) = &es.expression else {
                return None;
            };
            match &assign.left {
                oxc_ast::ast::AssignmentTarget::AssignmentTargetIdentifier(id) => {
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
        Span::new(span.start + offset, span.end + offset),
    ));
}

fn validate_reactive_declaration_module_script_dependency(
    data: &AnalysisData,
    program: &Program<'_>,
    offset: u32,
    diags: &mut Vec<Diagnostic>,
) {
    let Some(module_scope) = data.scoping.module_scope_id() else {
        return;
    };
    let stmts: Vec<_> = data
        .reactivity
        .legacy_reactive()
        .iter_statements_topo()
        .map(|s| {
            (
                s.stmt_node,
                s.dependencies.iter().copied().collect::<Vec<_>>(),
            )
        })
        .collect();
    for (stmt_node, deps) in stmts {
        for dep_sym in deps {
            if data.scoping.symbol_scope_id(dep_sym) != module_scope {
                continue;
            }
            if !data.scoping.is_mutated_any(dep_sym) {
                continue;
            }
            let dep_name = data.scoping.symbol_name(dep_sym).to_string();
            let Some(labeled) = (match data.scoping.js_kind(stmt_node) {
                Some(oxc_ast::AstKind::LabeledStatement(l)) => Some(l),
                _ => None,
            }) else {
                continue;
            };
            let body_span = labeled.body.span();
            let mut found_span: Option<Span> = None;
            collect_module_dep_ref_span(&labeled.body, &dep_name, &mut found_span);
            let span = found_span.unwrap_or_else(|| Span::new(body_span.start, body_span.end));
            diags.push(Diagnostic::warning(
                DiagnosticKind::ReactiveDeclarationModuleScriptDependency,
                Span::new(span.start + offset, span.end + offset),
            ));
            break;
        }
    }
    let _ = program;
}

fn collect_module_dep_ref_span(
    body: &oxc_ast::ast::Statement<'_>,
    name: &str,
    out: &mut Option<Span>,
) {
    use oxc_ast_visit::Visit;
    struct Finder<'a> {
        name: &'a str,
        out: &'a mut Option<Span>,
    }
    impl<'a, 'b> Visit<'b> for Finder<'a> {
        fn visit_identifier_reference(&mut self, id: &oxc_ast::ast::IdentifierReference<'b>) {
            if self.out.is_none() && id.name.as_str() == self.name {
                *self.out = Some(Span::new(id.span.start, id.span.end));
            }
        }
    }
    let mut f = Finder { name, out };
    f.visit_statement(body);
}

fn validate_legacy_export_invalid(program: &Program<'_>, offset: u32, diags: &mut Vec<Diagnostic>) {
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
            Span::new(export.span.start + offset, export.span.end + offset),
        ));
    }
}

fn validate_legacy_props_invalid(data: &AnalysisData, offset: u32, diags: &mut Vec<Diagnostic>) {
    emit_first_unresolved_read(
        data,
        "$$props",
        offset,
        diags,
        DiagnosticKind::LegacyPropsInvalid,
    );
}

fn validate_legacy_rest_props_invalid(
    data: &AnalysisData,
    offset: u32,
    diags: &mut Vec<Diagnostic>,
) {
    emit_first_unresolved_read(
        data,
        "$$restProps",
        offset,
        diags,
        DiagnosticKind::LegacyRestPropsInvalid,
    );
}

fn emit_first_unresolved_read(
    data: &AnalysisData,
    name: &str,
    offset: u32,
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
    diags.push(Diagnostic::error(
        kind,
        Span::new(span.start + offset, span.end + offset),
    ));
}

fn identifier_reference_span(data: &AnalysisData, node_id: OxcNodeId) -> Option<oxc_span::Span> {
    match data.scoping.js_kind(node_id)? {
        AstKind::IdentifierReference(id) => Some(id.span),
        _ => None,
    }
}

fn validate_export_let_unused(
    data: &AnalysisData,
    program: &Program<'_>,
    offset: u32,
    diags: &mut Vec<Diagnostic>,
) {
    let symbols: Vec<oxc_semantic::SymbolId> =
        data.reactivity.legacy_bindable_prop_symbols().to_vec();
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
            Span::new(span.start + offset, span.end + offset),
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

fn has_companion_store(data: &AnalysisData, sym: oxc_semantic::SymbolId) -> bool {
    let name = data.scoping.symbol_name(sym);
    let companion = format!("${name}");
    data.scoping
        .find_binding(data.scoping.root_scope_id(), &companion)
        .is_some()
}

fn has_non_export_read(
    data: &AnalysisData,
    sym: oxc_semantic::SymbolId,
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
