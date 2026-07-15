mod class_state_fields;
mod experimental_async;
mod legacy;
mod non_reactive_update;
mod runes;
mod stores;
mod syntax_bundle;
mod typescript;

use oxc_ast::ast::{
    BindingPattern, Declaration, ExportSpecifier, ImportDeclarationSpecifier, ModuleExportName,
    Program, Statement,
};
use svelte_ast::Component;
use svelte_diagnostics::{Diagnostic, DiagnosticKind};
use svelte_span::Span;

use crate::block_semantics::data::BlockSemantics;
use crate::{AnalysisData, types::data::JsAst};

pub fn validate(
    component: &Component,
    data: &AnalysisData,
    parsed: &JsAst,
    runes: bool,
    legacy_explicit: bool,
    diags: &mut Vec<Diagnostic>,
) {
    if let Some(program) = &parsed.program {
        validate_program(data, program, runes, diags);
        runes::validate_invalid_exports(data, program, true, None, diags);
        validate_illegal_default_export(program, diags);
    }

    stores::validate_global_references(data, legacy_explicit, diags);

    validate_module_program(parsed, diags);
    if let Some(module_program) = &parsed.module_program {
        runes::validate_module_props_runes(data, module_program, runes, diags);
        runes::validate_invalid_exports(
            data,
            module_program,
            false,
            data.scoping.module_scope_id(),
            diags,
        );
        stores::validate_module(data, module_program, diags);
        syntax_bundle::validate_module(data, module_program, diags);
    }
    non_reactive_update::validate(component, data, parsed, runes, diags);
    validate_snippet_exports(component, data, parsed, diags);
    validate_svelte_options_warnings(component, data, runes, diags);
    validate_script_context(component, runes, diags);
    runes::validate_const_tag_runes(component, parsed, data, diags);
    validate_const_tag_cycle_legacy(component, data, diags);
}

fn validate_const_tag_cycle_legacy(
    component: &Component,
    data: &AnalysisData,
    diags: &mut Vec<Diagnostic>,
) {
    let Some(cycle) = data.reactivity.const_tag_cycle_legacy() else {
        return;
    };
    diags.push(Diagnostic::error(
        DiagnosticKind::ConstTagCycle {
            cycle: cycle.names.clone(),
        },
        component.store.get(cycle.at_node).span(),
    ));
}

fn validate_script_context(component: &Component, runes: bool, diags: &mut Vec<Diagnostic>) {
    if !runes {
        return;
    }
    if let Some(script) = &component.module_script
        && script.context_deprecated
    {
        diags.push(Diagnostic::warning(
            DiagnosticKind::ScriptContextDeprecated,
            script.span,
        ));
    }
}

pub fn validate_program(
    data: &AnalysisData,
    program: &Program<'_>,
    runes: bool,
    diags: &mut Vec<Diagnostic>,
) {
    legacy::validate_legacy_diagnostics(data, program, runes, diags);
    runes::validate(data, program, runes, diags);
    stores::validate_scoped_subscriptions(data, diags);
    experimental_async::validate_instance_program(data, program, diags);
    syntax_bundle::validate_instance(data, program, runes, diags);
}

pub(crate) fn span_already_taken(diags: &[Diagnostic], span: Span) -> bool {
    diags.iter().any(|d| d.span == span)
}

pub fn validate_standalone_module(
    data: &AnalysisData,
    program: &Program<'_>,
    runes: bool,
    diags: &mut Vec<Diagnostic>,
) {
    runes::validate(data, program, runes, diags);
    runes::validate_invalid_exports(
        data,
        program,
        true,
        Some(data.scoping.root_scope_id()),
        diags,
    );
    stores::validate_standalone_module(data, program, diags);
    syntax_bundle::validate_module(data, program, diags);
}

pub fn validate_module_experimental_async(
    data: &AnalysisData,
    program: &Program<'_>,
    diags: &mut Vec<Diagnostic>,
) {
    experimental_async::validate_module_program(data, program, diags);
}

fn validate_module_program(parsed: &JsAst, diags: &mut Vec<Diagnostic>) {
    let Some(module_program) = &parsed.module_program else {
        return;
    };
    validate_illegal_default_export(module_program, diags);
}

fn validate_illegal_default_export(program: &Program<'_>, diags: &mut Vec<Diagnostic>) {
    for stmt in &program.body {
        match stmt {
            Statement::ExportDefaultDeclaration(export) => {
                diags.push(Diagnostic::error(
                    DiagnosticKind::ModuleIllegalDefaultExport,
                    Span::new(export.span.start, export.span.end),
                ));
            }
            Statement::ExportNamedDeclaration(export)
                if export.specifiers.iter().any(export_specifier_is_default) =>
            {
                diags.push(Diagnostic::error(
                    DiagnosticKind::ModuleIllegalDefaultExport,
                    Span::new(export.span.start, export.span.end),
                ));
            }
            _ => {}
        }
    }
}

fn export_specifier_is_default(specifier: &ExportSpecifier<'_>) -> bool {
    match &specifier.exported {
        ModuleExportName::IdentifierName(name) => name.name == "default",
        ModuleExportName::IdentifierReference(name) => name.name == "default",
        ModuleExportName::StringLiteral(name) => name.value == "default",
    }
}

fn validate_snippet_exports(
    component: &Component,
    data: &AnalysisData,
    parsed: &JsAst,
    diags: &mut Vec<Diagnostic>,
) {
    let Some(module_program) = &parsed.module_program else {
        return;
    };

    let snippets: Vec<(&str, bool)> = (0..component.store.len())
        .filter_map(|i| {
            let id = svelte_ast::NodeId(i);
            let snippet = component.store.get(id).as_snippet_block()?;
            let hoistable = matches!(
                data.block_semantics(id),
                BlockSemantics::Snippet(sem) if sem.placement.is_module_level()
            );
            Some((snippet.name(&component.source), hoistable))
        })
        .collect();

    if snippets.is_empty() {
        return;
    }

    for stmt in &module_program.body {
        let Statement::ExportNamedDeclaration(export) = stmt else {
            continue;
        };

        if export.declaration.is_some() || export.source.is_some() {
            continue;
        }
        for specifier in &export.specifiers {
            let ModuleExportName::IdentifierReference(ident) = &specifier.local else {
                continue;
            };
            let name = ident.name.as_str();

            let Some(&(_, hoistable)) = snippets.iter().find(|(n, _)| *n == name) else {
                continue;
            };
            if !hoistable && !is_module_bound(module_program, name) {
                let span = Span::new(specifier.span.start, specifier.span.end);
                diags.push(Diagnostic::error(
                    DiagnosticKind::SnippetInvalidExport,
                    span,
                ));
            }
        }
    }
}

fn is_module_bound<'a>(program: &Program<'a>, name: &str) -> bool {
    for stmt in &program.body {
        match stmt {
            Statement::VariableDeclaration(decl) => {
                if decl
                    .declarations
                    .iter()
                    .any(|d| binding_contains(&d.id, name))
                {
                    return true;
                }
            }
            Statement::FunctionDeclaration(func) => {
                if func.id.as_ref().is_some_and(|id| id.name == name) {
                    return true;
                }
            }
            Statement::ClassDeclaration(cls) => {
                if cls.id.as_ref().is_some_and(|id| id.name == name) {
                    return true;
                }
            }
            Statement::ImportDeclaration(import) => {
                if let Some(specifiers) = &import.specifiers {
                    for spec in specifiers {
                        let local = match spec {
                            ImportDeclarationSpecifier::ImportSpecifier(s) => s.local.name.as_str(),
                            ImportDeclarationSpecifier::ImportDefaultSpecifier(s) => {
                                s.local.name.as_str()
                            }
                            ImportDeclarationSpecifier::ImportNamespaceSpecifier(s) => {
                                s.local.name.as_str()
                            }
                        };
                        if local == name {
                            return true;
                        }
                    }
                }
            }
            Statement::ExportNamedDeclaration(export) => {
                if let Some(decl) = &export.declaration {
                    match decl {
                        Declaration::VariableDeclaration(d) => {
                            if d.declarations.iter().any(|v| binding_contains(&v.id, name)) {
                                return true;
                            }
                        }
                        Declaration::FunctionDeclaration(f) => {
                            if f.id.as_ref().is_some_and(|id| id.name == name) {
                                return true;
                            }
                        }
                        Declaration::ClassDeclaration(c) => {
                            if c.id.as_ref().is_some_and(|id| id.name == name) {
                                return true;
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
    false
}

fn binding_contains(pattern: &BindingPattern<'_>, name: &str) -> bool {
    match pattern {
        BindingPattern::BindingIdentifier(id) => id.name == name,
        BindingPattern::ObjectPattern(obj) => {
            obj.properties
                .iter()
                .any(|p| binding_contains(&p.value, name))
                || obj
                    .rest
                    .as_ref()
                    .is_some_and(|r| binding_contains(&r.argument, name))
        }
        BindingPattern::ArrayPattern(arr) => {
            arr.elements
                .iter()
                .flatten()
                .any(|e| binding_contains(e, name))
                || arr
                    .rest
                    .as_ref()
                    .is_some_and(|r| binding_contains(&r.argument, name))
        }
        BindingPattern::AssignmentPattern(assign) => binding_contains(&assign.left, name),
    }
}

fn validate_svelte_options_warnings(
    component: &Component,
    data: &AnalysisData,
    runes: bool,
    diags: &mut Vec<Diagnostic>,
) {
    let Some(options) = &component.options else {
        return;
    };

    for attr in &options.attributes {
        let kind = match attr.html_name() {
            "accessors" if runes => Some(DiagnosticKind::OptionsDeprecatedAccessors),
            "immutable" if runes => Some(DiagnosticKind::OptionsDeprecatedImmutable),
            "customElement" if !data.output.custom_element_compile_flag => {
                Some(DiagnosticKind::OptionsMissingCustomElement)
            }
            _ => None,
        };

        if let Some(kind) = kind {
            diags.push(Diagnostic::warning(kind, attr.span()));
        }
    }
}
