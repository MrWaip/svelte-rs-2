use compact_str::CompactString;
use oxc_ast::{
    AstKind,
    ast::{
        BindingPattern, Class, Declaration, ExportNamedDeclaration, ExportSpecifier, Expression,
        Function, ModuleExportName, VariableDeclaration, VariableDeclarationKind,
    },
};
use svelte_component_semantics::{ReferenceId, walk_bindings};

use crate::scope::SymbolId;
use crate::types::data::{AnalysisData, ApiExport};
use crate::utils::{is_let_or_var, is_simple_expression};

use super::super::data::{
    BindingFacts, DeclaratorSemantics, LegacyBindablePropSemantics, PropDefaultKind,
    ReferenceFacts, ReferenceSemantics,
};
use crate::PropsFlags;

const PROPS_NAME: &str = "$$props";
const REST_PROPS_NAME: &str = "$$restProps";

pub(super) fn classify_export_named_declaration<'a>(
    data: &mut AnalysisData<'a>,
    export: &ExportNamedDeclaration<'a>,
) {
    if data.script.runes() {
        classify_runes_export(data, export);
        return;
    }
    data.output.legacy_has_export_declaration = true;
    if let Some(decl) = &export.declaration {
        match decl {
            Declaration::VariableDeclaration(var_decl) if is_let_or_var(var_decl.kind) => {
                classify_variable_declaration(data, var_decl);
            }
            Declaration::VariableDeclaration(var_decl) => {
                record_api_export_variable_symbols(data, var_decl);
            }
            Declaration::FunctionDeclaration(func) => {
                record_api_export_function_symbol(data, func);
            }
            Declaration::ClassDeclaration(cls) => {
                record_api_export_class_symbol(data, cls);
            }
            _ => {}
        }
    } else {
        classify_specifiers(data, export);
    }
}

fn classify_runes_export<'a>(data: &mut AnalysisData<'a>, export: &ExportNamedDeclaration<'a>) {
    for spec in &export.specifiers {
        let ModuleExportName::IdentifierReference(local) = &spec.local else {
            continue;
        };
        let Some(ref_id) = local.reference_id.get() else {
            continue;
        };
        let Some(symbol) = data.scoping.semantics().symbol_for_reference(ref_id) else {
            continue;
        };
        let exported = spec.exported.name();
        let alias = if local.name != exported {
            Some(CompactString::from(exported.as_str()))
        } else {
            None
        };
        data.output.api_exports.push(ApiExport {
            local: symbol,
            reference_id: Some(ref_id),
            alias,
        });
    }
    let Some(decl) = &export.declaration else {
        return;
    };
    match decl {
        Declaration::VariableDeclaration(var_decl)
            if var_decl.kind == VariableDeclarationKind::Const =>
        {
            for declarator in &var_decl.declarations {
                walk_bindings(&declarator.id, |visit| {
                    data.output.api_exports.push(ApiExport {
                        local: visit.symbol,
                        reference_id: None,
                        alias: None,
                    });
                });
            }
        }
        Declaration::VariableDeclaration(_) => {}
        Declaration::FunctionDeclaration(func) => {
            if let Some(ident) = &func.id
                && let Some(symbol) = ident.symbol_id.get()
            {
                data.output.api_exports.push(ApiExport {
                    local: symbol,
                    reference_id: None,
                    alias: None,
                });
            }
        }
        Declaration::ClassDeclaration(cls) => {
            if let Some(ident) = &cls.id
                && let Some(symbol) = ident.symbol_id.get()
            {
                data.output.api_exports.push(ApiExport {
                    local: symbol,
                    reference_id: None,
                    alias: None,
                });
            }
        }
        _ => {}
    }
}

fn record_api_export_variable_symbols<'a>(
    data: &mut AnalysisData<'a>,
    decl: &VariableDeclaration<'a>,
) {
    for declarator in &decl.declarations {
        walk_bindings(&declarator.id, |visit| {
            record_api_export(data, visit.symbol, None, None);
        });
    }
}

fn record_api_export_function_symbol<'a>(data: &mut AnalysisData<'a>, func: &Function<'a>) {
    let Some(ident) = func.id.as_ref() else {
        return;
    };
    let Some(symbol) = ident.symbol_id.get() else {
        return;
    };
    record_api_export(data, symbol, None, None);
}

fn record_api_export_class_symbol<'a>(data: &mut AnalysisData<'a>, cls: &Class<'a>) {
    let Some(ident) = cls.id.as_ref() else {
        return;
    };
    let Some(symbol) = ident.symbol_id.get() else {
        return;
    };
    record_api_export(data, symbol, None, None);
}

fn record_api_export(
    data: &mut AnalysisData<'_>,
    symbol: SymbolId,
    reference_id: Option<ReferenceId>,
    alias: Option<CompactString>,
) {
    data.reactivity.record_legacy_api_export_binding(symbol);
    data.output.api_exports.push(ApiExport {
        local: symbol,
        reference_id,
        alias,
    });
}

fn classify_variable_declaration<'a>(data: &mut AnalysisData<'a>, decl: &VariableDeclaration<'a>) {
    let immutable = data.script.immutable;
    let accessors = data.script.accessors;
    for declarator in &decl.declarations {
        let init_default = declarator
            .init
            .as_ref()
            .map(|init| classify_expression_default(data, init));
        let pattern_has_outer = is_destructured_pattern(&declarator.id);

        walk_bindings(&declarator.id, |visit| {
            let default_lowering = if pattern_has_outer {
                PropDefaultKind::Lazy
            } else {
                init_default.unwrap_or(PropDefaultKind::None)
            };
            let updated_any = data.scoping.is_mutated_any(visit.symbol);
            let reassigned = data.scoping.is_mutated(visit.symbol);
            let updated = if immutable { reassigned } else { updated_any };
            let flags = compute_flags(updated, accessors, immutable);
            let semantics = LegacyBindablePropSemantics {
                default_kind: default_lowering,
                flags,
            };
            data.reactivity
                .record_legacy_bindable_prop_binding(visit.symbol, semantics);
            data.reactivity
                .record_legacy_bindable_prop_symbol(visit.symbol);
        });

        data.reactivity
            .record_declarator_semantics(declarator.node_id(), DeclaratorSemantics::LegacyProps);
    }
}

fn classify_specifiers<'a>(data: &mut AnalysisData<'a>, export: &ExportNamedDeclaration<'a>) {
    let Some(instance_scope) = data.scoping.instance_scope_id() else {
        return;
    };
    for spec in &export.specifiers {
        if matches!(spec.exported, ModuleExportName::StringLiteral(_)) {
            continue;
        }
        let local_name = spec.local.name();
        let Some(symbol) = data
            .scoping
            .find_binding(instance_scope, local_name.as_str())
        else {
            continue;
        };
        let exported = spec.exported.name();
        let alias = if local_name != exported {
            Some(CompactString::from(exported.as_str()))
        } else {
            None
        };
        if !data.scoping.is_reassignable_declaration(symbol) {
            record_api_export(data, symbol, specifier_reference_id(spec), alias);
            continue;
        }
        let init = lookup_let_or_var_init(data, symbol).and_then(|(_, init)| init);
        let default_kind = match init {
            Some(init_expr) => classify_expression_default(data, init_expr),
            None => PropDefaultKind::None,
        };

        let updated = if data.script.immutable {
            data.scoping.is_mutated(symbol)
        } else {
            data.scoping.is_mutated_any(symbol)
        };
        let flags = compute_flags(updated, data.script.accessors, data.script.immutable);
        data.reactivity.record_legacy_bindable_prop_binding(
            symbol,
            LegacyBindablePropSemantics {
                default_kind,
                flags,
            },
        );
        data.reactivity.record_legacy_bindable_prop_symbol(symbol);
    }
}

fn specifier_reference_id(spec: &ExportSpecifier<'_>) -> Option<ReferenceId> {
    let ModuleExportName::IdentifierReference(local) = &spec.local else {
        return None;
    };
    local.reference_id.get()
}

fn compute_flags(updated: bool, accessors: bool, immutable: bool) -> PropsFlags {
    let mut flags = PropsFlags::BINDABLE;
    if immutable {
        flags |= PropsFlags::IMMUTABLE;
    }
    if accessors || updated {
        flags |= PropsFlags::UPDATED;
    }
    flags
}

pub(super) fn classify_unresolved_legacy_identifiers(data: &mut AnalysisData<'_>) {
    if data.script.runes() {
        return;
    }
    let unresolved = data.scoping.root_unresolved_references().clone();
    let mut uses_props = false;
    let mut uses_rest_props = false;
    for (name, refs) in &unresolved {
        let (fact, props_kind) = if name.as_str() == PROPS_NAME {
            (ReferenceFacts::LegacyPropsIdentifierRead, true)
        } else if name.as_str() == REST_PROPS_NAME {
            (ReferenceFacts::LegacyRestPropsIdentifierRead, false)
        } else {
            continue;
        };
        for &ref_id in refs {
            let reference = data.scoping.get_reference(ref_id);
            if !reference.is_read() || reference.is_write() {
                continue;
            }
            data.reactivity
                .record_reference_semantics(ref_id, fact.clone());
            if props_kind {
                uses_props = true;
            } else {
                uses_rest_props = true;
            }
        }
    }
    data.reactivity
        .set_legacy_unresolved_usage(uses_props, uses_rest_props);
}

pub(super) fn finalize_legacy_aggregates(data: &mut AnalysisData<'_>) {
    if data.script.runes() {
        return;
    }
    let symbols: Vec<SymbolId> = data.reactivity.legacy_bindable_prop_symbols().to_vec();

    let is_non_store_ref = |data: &AnalysisData<'_>, r| {
        !matches!(
            data.reactivity.reference_semantics(r),
            ReferenceSemantics::StoreRead { .. }
                | ReferenceSemantics::StoreWrite { .. }
                | ReferenceSemantics::StoreUpdate { .. }
        )
    };
    let prop_member_mutated = |data: &AnalysisData<'_>, sym| {
        data.scoping
            .get_resolved_reference_ids(sym)
            .iter()
            .any(|&r| {
                data.reactivity.is_prop_member_mutation_root_ref(r) && is_non_store_ref(data, r)
            })
    };

    let has_member_mutated = symbols.iter().any(|&sym| prop_member_mutated(data, sym));
    data.reactivity
        .set_legacy_has_member_mutated(has_member_mutated);

    let immutable = data.script.immutable;
    let accessors = data.script.accessors;
    for sym in symbols {
        let refs = data.scoping.get_resolved_reference_ids(sym).to_vec();
        let non_store_write = refs
            .iter()
            .any(|&r| data.scoping.get_reference(r).is_write() && is_non_store_ref(data, r));
        let member_mutated = prop_member_mutated(data, sym);
        let updated_any = non_store_write || member_mutated;
        let reassigned = non_store_write;
        let updated = if immutable { reassigned } else { updated_any };
        let new_flags = compute_flags(updated, accessors, immutable);
        if let Some(BindingFacts::LegacyBindableProp(legacy)) =
            data.reactivity.binding_facts_mut(sym)
        {
            legacy.flags = new_flags;
        }
    }
}

fn is_destructured_pattern(pat: &BindingPattern<'_>) -> bool {
    !matches!(pat, BindingPattern::BindingIdentifier(_))
}

fn classify_expression_default<'a>(
    data: &AnalysisData<'a>,
    init: &Expression<'a>,
) -> PropDefaultKind {
    if is_simple_expression(init) {
        if let Expression::Identifier(id) = init
            && let Some(sym) = data.scoping.symbol_for_identifier_reference(id)
            && matches!(
                data.reactivity.binding_semantics(sym),
                crate::BindingSemantics::LegacyBindableProp(_)
            )
        {
            return PropDefaultKind::LazyAccessor;
        }
        if references_legacy_bindable_prop(data, init) {
            return PropDefaultKind::Lazy;
        }
        PropDefaultKind::Eager
    } else {
        PropDefaultKind::Lazy
    }
}

fn references_legacy_bindable_prop<'a>(data: &AnalysisData<'a>, expr: &Expression<'a>) -> bool {
    match expr {
        Expression::Identifier(id) => data
            .scoping
            .symbol_for_identifier_reference(id)
            .is_some_and(|sym| {
                matches!(
                    data.reactivity.binding_semantics(sym),
                    crate::BindingSemantics::LegacyBindableProp(_)
                )
            }),
        Expression::ConditionalExpression(c) => {
            references_legacy_bindable_prop(data, &c.test)
                || references_legacy_bindable_prop(data, &c.consequent)
                || references_legacy_bindable_prop(data, &c.alternate)
        }
        Expression::BinaryExpression(b) => {
            references_legacy_bindable_prop(data, &b.left)
                || references_legacy_bindable_prop(data, &b.right)
        }
        Expression::LogicalExpression(b) => {
            references_legacy_bindable_prop(data, &b.left)
                || references_legacy_bindable_prop(data, &b.right)
        }
        Expression::ParenthesizedExpression(p) => {
            references_legacy_bindable_prop(data, &p.expression)
        }
        Expression::TSAsExpression(_)
        | Expression::TSSatisfiesExpression(_)
        | Expression::TSNonNullExpression(_)
        | Expression::TSTypeAssertion(_)
        | Expression::TSInstantiationExpression(_) => unreachable!("TS stripped at parse"),
        _ => false,
    }
}

fn lookup_let_or_var_init<'a>(
    data: &AnalysisData<'a>,
    symbol: SymbolId,
) -> Option<(VariableDeclarationKind, Option<&'a Expression<'a>>)> {
    let decl_node = data.scoping.symbol_declaration(symbol);
    let mut current = data.scoping.js_parent_id(decl_node)?;
    let mut declarator_init: Option<&'a Expression<'a>> = None;
    loop {
        match data.scoping.js_kind(current)? {
            AstKind::VariableDeclarator(declarator) => {
                declarator_init = declarator.init.as_ref();
            }
            AstKind::VariableDeclaration(decl) => {
                return Some((decl.kind, declarator_init));
            }
            _ => return None,
        }
        current = data.scoping.js_parent_id(current)?;
    }
}
