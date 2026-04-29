use oxc_ast::{
    AstKind,
    ast::{
        BindingPattern, ExportNamedDeclaration, Expression, VariableDeclaration,
        VariableDeclarationKind,
    },
};
use svelte_component_semantics::walk_bindings;

use crate::scope::SymbolId;
use crate::types::data::AnalysisData;
use crate::utils::{is_let_or_var, is_simple_expression};

use super::super::data::{LegacyBindablePropSemantics, PropDefaultLowering, ReferenceFacts};
use crate::PropsFlags;

const PROPS_NAME: &str = "$$props";
const REST_PROPS_NAME: &str = "$$restProps";

pub(super) fn classify_export_named_declaration<'a>(
    data: &mut AnalysisData<'a>,
    export: &ExportNamedDeclaration<'a>,
) {
    if data.script.runes {
        return;
    }
    if let Some(decl) = &export.declaration {
        let oxc_ast::ast::Declaration::VariableDeclaration(var_decl) = decl else {
            return;
        };
        if !is_let_or_var(var_decl.kind) {
            return;
        }
        classify_variable_declaration(data, var_decl);
    } else {
        classify_specifiers(data, export);
    }
}

fn classify_variable_declaration<'a>(data: &mut AnalysisData<'a>, decl: &VariableDeclaration<'a>) {
    let immutable = data.script.immutable;
    let accessors = data.script.accessors;
    for declarator in &decl.declarations {
        let init_default = declarator
            .init
            .as_ref()
            .map(|init| classify_expression_default(init));
        let pattern_has_outer = is_destructured_pattern(&declarator.id);

        walk_bindings(&declarator.id, |visit| {
            let default_lowering = if pattern_has_outer {
                PropDefaultLowering::Lazy
            } else {
                init_default.unwrap_or(PropDefaultLowering::None)
            };
            let updated_any = data.scoping.is_mutated_any(visit.symbol);
            let reassigned = data.scoping.is_mutated(visit.symbol);
            let updated = if immutable { reassigned } else { updated_any };
            let flags = compute_flags(updated, accessors, immutable);
            let semantics = LegacyBindablePropSemantics {
                default_lowering,
                flags,
            };
            data.reactivity
                .record_legacy_bindable_prop_binding(visit.symbol, semantics);
            data.reactivity
                .record_legacy_bindable_prop_symbol(visit.symbol);
        });
    }
}

fn classify_specifiers<'a>(data: &mut AnalysisData<'a>, export: &ExportNamedDeclaration<'a>) {
    let Some(instance_scope) = data.scoping.instance_scope_id() else {
        return;
    };
    for spec in &export.specifiers {
        let local_name = spec.local.name();
        let Some(symbol) = data
            .scoping
            .find_binding(instance_scope, local_name.as_str())
        else {
            continue;
        };
        let Some((kind, init)) = lookup_let_or_var_init(data, symbol) else {
            continue;
        };
        if !is_let_or_var(kind) {
            continue;
        }
        let default_lowering = match init {
            Some(init_expr) => classify_expression_default(init_expr),
            None => PropDefaultLowering::None,
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
                default_lowering,
                flags,
            },
        );
        data.reactivity.record_legacy_bindable_prop_symbol(symbol);
    }
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
    if data.script.runes {
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
    if data.script.runes {
        return;
    }
    let symbols: Vec<SymbolId> = data.reactivity.legacy_bindable_prop_symbols().to_vec();
    let has_member_mutated = symbols
        .iter()
        .any(|&sym| data.scoping.is_member_mutated(sym));
    data.reactivity
        .set_legacy_has_member_mutated(has_member_mutated);
}

fn is_destructured_pattern(pat: &BindingPattern<'_>) -> bool {
    !matches!(pat, BindingPattern::BindingIdentifier(_))
}

fn classify_expression_default(init: &Expression<'_>) -> PropDefaultLowering {
    if is_simple_expression(init) {
        PropDefaultLowering::Eager
    } else {
        PropDefaultLowering::Lazy
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
