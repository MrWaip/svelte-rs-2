//! LEGACY(svelte4): classification of legacy `export let` / `export var` props
//! and `$$props` / `$$restProps` identifier reads in non-runes mode.
//!
//! Deprecated in Svelte 5, remove in Svelte 6.
//! Removal recipe: `rg 'LEGACY\(svelte4\):' crates/svelte_analyze/src/reactivity_semantics/`,
//! drop the matched variants, delete this file.

use oxc_ast::{
    ast::{
        BindingPattern, ExportNamedDeclaration, Expression, VariableDeclaration,
        VariableDeclarationKind,
    },
    AstKind,
};
use svelte_component_semantics::{walk_bindings, OxcNodeId, Step};

use crate::scope::SymbolId;
use crate::types::data::AnalysisData;
use crate::utils::is_simple_expression;

use super::super::data::{LegacyBindablePropSemantics, PropDefaultLowering, V2ReferenceFacts};
use crate::{PROPS_IS_BINDABLE, PROPS_IS_IMMUTABLE, PROPS_IS_LAZY_INITIAL, PROPS_IS_UPDATED};

const PROPS_NAME: &str = "$$props";
const REST_PROPS_NAME: &str = "$$restProps";

/// LEGACY(svelte4): classify a saved `ExportNamedDeclaration` by its node id.
/// Called from `ScriptSemanticCollector::finish` so member-mutation tracking
/// is full before `PROPS_IS_UPDATED` is computed.
pub(super) fn classify_export_node<'a>(
    data: &mut AnalysisData<'a>,
    node_id: OxcNodeId,
    member_mutated: &rustc_hash::FxHashSet<SymbolId>,
) {
    if data.script.runes {
        return;
    }
    let Some(AstKind::ExportNamedDeclaration(export)) = data.scoping.js_kind(node_id) else {
        return;
    };
    if let Some(decl) = &export.declaration {
        let oxc_ast::ast::Declaration::VariableDeclaration(var_decl) = decl else {
            return;
        };
        if !is_let_or_var(var_decl.kind) {
            return;
        }
        classify_variable_declaration(data, var_decl, member_mutated);
    } else {
        classify_specifiers(data, export, member_mutated);
    }
}

fn classify_variable_declaration<'a>(
    data: &mut AnalysisData<'a>,
    decl: &VariableDeclaration<'a>,
    member_mutated: &rustc_hash::FxHashSet<SymbolId>,
) {
    let immutable = data.script.immutable;
    let accessors = data.script.accessors;
    for declarator in &decl.declarations {
        let init_default = declarator
            .init
            .as_ref()
            .map(|init| classify_expression_default(init));
        let pattern_has_outer = is_destructured_pattern(&declarator.id);

        walk_bindings(&declarator.id, |visit| {
            let leaf_default = if pattern_has_outer {
                leaf_default_lowering(visit.path)
            } else {
                PropDefaultLowering::None
            };
            let default_lowering = match (leaf_default, init_default) {
                (PropDefaultLowering::None, Some(d)) if !pattern_has_outer => d,
                (leaf, _) => leaf,
            };
            let updated =
                data.scoping.is_mutated_any(visit.symbol) || member_mutated.contains(&visit.symbol);
            let flags = compute_flags(default_lowering, updated, accessors, immutable);
            let semantics = LegacyBindablePropSemantics {
                default_lowering,
                flags,
            };
            let node_id = data.scoping.symbol_declaration(visit.symbol);
            let node_id: OxcNodeId = node_id;
            data.reactivity
                .record_legacy_bindable_prop_declaration_v2(node_id, semantics);
        });
    }
}

fn classify_specifiers<'a>(
    data: &mut AnalysisData<'a>,
    export: &ExportNamedDeclaration<'a>,
    member_mutated: &rustc_hash::FxHashSet<SymbolId>,
) {
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
            // Non-let/var binding (function/class/const) — reference compiler
            // routes those to component exports, not props.
            continue;
        };
        if !is_let_or_var(kind) {
            continue;
        }
        let default_lowering = match init {
            Some(init_expr) => classify_expression_default(init_expr),
            None => PropDefaultLowering::None,
        };
        let updated = data.scoping.is_mutated_any(symbol) || member_mutated.contains(&symbol);
        let flags = compute_flags(
            default_lowering,
            updated,
            data.script.accessors,
            data.script.immutable,
        );
        let node_id = data.scoping.symbol_declaration(symbol);
        let node_id: OxcNodeId = node_id;
        data.reactivity.record_legacy_bindable_prop_declaration_v2(
            node_id,
            LegacyBindablePropSemantics {
                default_lowering,
                flags,
            },
        );
    }
}

fn compute_flags(
    default_lowering: PropDefaultLowering,
    updated: bool,
    accessors: bool,
    immutable: bool,
) -> u32 {
    let mut flags = PROPS_IS_BINDABLE;
    if immutable {
        flags |= PROPS_IS_IMMUTABLE;
    }
    if accessors || updated {
        flags |= PROPS_IS_UPDATED;
    }
    if matches!(default_lowering, PropDefaultLowering::Lazy) {
        flags |= PROPS_IS_LAZY_INITIAL;
    }
    flags
}

/// LEGACY(svelte4): emit `LegacyPropsIdentifierRead` / `LegacyRestPropsIdentifierRead`
/// for every read site of the unresolved synthetic identifiers `$$props` / `$$restProps`
/// in non-runes mode. Sites are read from `ComponentSemantics.root_unresolved_references`,
/// which is already populated by the component-semantics builder pass.
pub(super) fn classify_unresolved_legacy_identifiers(data: &mut AnalysisData<'_>) {
    if data.script.runes {
        return;
    }
    let unresolved = data.scoping.root_unresolved_references().clone();
    for (name, refs) in &unresolved {
        let fact = if name.as_str() == PROPS_NAME {
            V2ReferenceFacts::LegacyPropsIdentifierRead
        } else if name.as_str() == REST_PROPS_NAME {
            V2ReferenceFacts::LegacyRestPropsIdentifierRead
        } else {
            continue;
        };
        for &ref_id in refs {
            let reference = data.scoping.get_reference(ref_id);
            if !reference.is_read() || reference.is_write() {
                continue;
            }
            data.reactivity
                .record_reference_semantics_v2(ref_id, fact.clone());
        }
    }
}

fn is_let_or_var(kind: VariableDeclarationKind) -> bool {
    matches!(
        kind,
        VariableDeclarationKind::Let | VariableDeclarationKind::Var
    )
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

fn leaf_default_lowering(path: &[Step<'_>]) -> PropDefaultLowering {
    let Some(last) = path.last() else {
        return PropDefaultLowering::None;
    };
    let Some(default) = last.default else {
        return PropDefaultLowering::None;
    };
    if is_simple_expression(default) {
        PropDefaultLowering::Eager
    } else {
        PropDefaultLowering::Lazy
    }
}

/// Walk from a symbol's declaration node up to its enclosing `VariableDeclaration`
/// so we can read the `let`/`var` kind and the declarator's `init`.
/// Returns `None` for symbols not declared by a `VariableDeclaration` (function /
/// class / import / etc.), which are not eligible for legacy prop classification.
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
