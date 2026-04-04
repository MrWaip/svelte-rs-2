//! Rune marking pass.
//!
//! Marks `$state`, `$derived`, etc. on symbols in ComponentScoping.
//! Two entry points:
//! - `mark_script_runes` — root-scope runes from ScriptInfo declarations
//! - `mark_nested_runes` — non-root runes found via OXC Visit

use oxc_ast_visit::Visit;
use rustc_hash::FxHashMap;

use crate::scope::{ComponentScoping, ScopeId, SymbolId};
use crate::types::data::AnalysisData;
use crate::types::script::{DeclarationInfo, DeclarationKind, RuneKind};

/// Mark runes declared at the root scope from ScriptInfo.
pub(crate) fn mark_script_runes(data: &mut AnalysisData) {
    let Some(script_info) = &data.script else {
        return;
    };
    let root = data.scoping.root_scope_id();
    mark_root_script_runes_in_scope(
        &mut data.scoping,
        root,
        &script_info.declarations,
        &data.proxy_state_inits,
    );
}

pub(crate) fn mark_root_script_runes_in_scope(
    scoping: &mut ComponentScoping,
    scope: ScopeId,
    declarations: &[DeclarationInfo],
    proxy_state_inits: &FxHashMap<compact_str::CompactString, bool>,
) {
    for decl in declarations {
        let Some(rune_kind) = decl.is_rune else {
            continue;
        };
        let Some(sym_id) = scoping.find_binding(scope, &decl.name) else {
            continue;
        };
        let is_proxy = proxy_state_inits.get(&decl.name).copied().unwrap_or(false);
        scoping.mark_rune_with_proxy(sym_id, rune_kind, is_proxy);
        if decl.kind == DeclarationKind::Var
            && matches!(rune_kind, RuneKind::State | RuneKind::StateRaw)
        {
            scoping.mark_var_state(sym_id);
        }
        if rune_kind.is_derived() && !decl.rune_init_refs.is_empty() {
            let deps: Vec<SymbolId> = decl
                .rune_init_refs
                .iter()
                .filter_map(|name| scoping.find_binding(scope, name))
                .collect();
            if !deps.is_empty() {
                scoping.set_derived_deps(sym_id, deps);
            }
        }
    }
}

/// Mark runes declared in nested function scopes ($derived, $state, etc.).
/// Root-scope runes are already marked by `mark_script_runes`;
/// this handles the remaining non-root declarations that `extract_script_info` skips.
pub(crate) fn mark_nested_runes(
    program: &oxc_ast::ast::Program<'_>,
    scoping: &mut ComponentScoping,
) {
    let root = scoping.root_scope_id();
    let mut visitor = NestedRuneVisitor { scoping, root };
    visitor.visit_program(program);
}

struct NestedRuneVisitor<'s> {
    scoping: &'s mut ComponentScoping,
    root: ScopeId,
}

impl<'a> Visit<'a> for NestedRuneVisitor<'_> {
    fn visit_variable_declarator(&mut self, declarator: &oxc_ast::ast::VariableDeclarator<'a>) {
        let Some(init) = &declarator.init else { return };
        let Some(rune_kind) = crate::utils::script_info::detect_rune(init) else {
            return;
        };
        if !matches!(
            rune_kind,
            RuneKind::Derived | RuneKind::DerivedBy | RuneKind::State | RuneKind::StateRaw
        ) {
            return;
        }
        let Some(sym_id) = declarator
            .id
            .get_binding_identifier()
            .and_then(|id| id.symbol_id.get())
        else {
            return;
        };
        if self.scoping.symbol_scope_id(sym_id) == self.root {
            return;
        }
        self.scoping.mark_rune(sym_id, rune_kind);
    }
}
