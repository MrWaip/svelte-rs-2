use oxc_ast::AstKind;
use oxc_ast::ast::{
    Declaration, IdentifierReference, ModuleExportName, Program, Statement, VariableDeclarationKind,
};
use oxc_semantic::ReferenceId;
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
    }
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
