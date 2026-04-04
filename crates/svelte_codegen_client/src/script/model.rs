use oxc_ast::ast::Expression;
use oxc_semantic::Scoping;
use rustc_hash::{FxHashMap, FxHashSet};

use svelte_analyze::{ComponentScoping, IgnoreData, PropsAnalysis, RuneKind};

use crate::builder::Builder;

pub(super) enum PropKind {
    Source,
    NonSource(String),
}

pub(super) struct PropsGenInfo {
    pub(super) props: Vec<PropGenItem>,
    pub(super) is_identifier_pattern: bool,
}

impl PropsGenInfo {
    pub(super) fn from_analysis(pa: &PropsAnalysis) -> Self {
        PropsGenInfo {
            is_identifier_pattern: pa.is_identifier_pattern,
            props: pa
                .props
                .iter()
                .map(|p| PropGenItem {
                    local_name: p.local_name.clone(),
                    prop_name: p.prop_name.clone(),
                    is_prop_source: p.is_prop_source,
                    is_bindable: p.is_bindable,
                    is_rest: p.is_rest,
                    is_mutated: p.is_mutated,
                    default_text: p.default_text.clone(),
                    is_lazy_default: p.is_lazy_default,
                })
                .collect(),
        }
    }
}

pub(super) struct PropGenItem {
    pub(super) local_name: String,
    pub(super) prop_name: String,
    pub(super) is_prop_source: bool,
    pub(super) is_bindable: bool,
    pub(super) is_rest: bool,
    pub(super) is_mutated: bool,
    pub(super) default_text: Option<String>,
    pub(super) is_lazy_default: bool,
}

pub(super) struct FunctionInfo {
    pub(super) is_async: bool,
    pub(super) name: Option<String>,
    pub(super) span_start: u32,
    pub(super) in_constructor: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum AsyncDerivedMode {
    Await,
    Save,
}

pub(super) struct ClassStateField {
    pub(super) public_name: Option<String>,
    pub(super) private_name: String,
    pub(super) rune_kind: RuneKind,
}

pub(super) struct ClassStateInfo {
    pub(super) fields: Vec<ClassStateField>,
    /// Public field names whose backing/getter/setter come from constructor rune assignments.
    pub(super) ctor_synth_names: FxHashSet<String>,
    /// Bare placeholder declarations (`field;`) that are replaced by constructor-owned lowering.
    pub(super) ctor_placeholder_names: FxHashSet<String>,
}

pub(super) struct ScriptTransformer<'b, 'a> {
    pub(super) b: &'b Builder<'a>,
    pub(super) component_scoping: &'b ComponentScoping,
    pub(super) scoping: Scoping,
    pub(super) props_gen: Option<PropsGenInfo>,
    pub(super) derived_pending: FxHashSet<oxc_semantic::SymbolId>,
    /// Subset of `derived_pending`: symbols whose `$derived` init was `$derived(await expr)`.
    /// Used by `wrap_derived_thunks` to determine async thunk form and outer wrapping
    /// after dev transforms run.
    pub(super) async_derived_pending: FxHashMap<oxc_semantic::SymbolId, AsyncDerivedMode>,
    pub(super) strip_exports: bool,
    pub(super) dev: bool,
    pub(super) is_ts: bool,
    pub(super) function_info_stack: Vec<FunctionInfo>,
    pub(super) has_tracing: bool,
    pub(super) component_source: &'b str,
    pub(super) script_content_start: u32,
    pub(super) filename: &'b str,
    pub(super) next_arrow_name: Option<String>,
    pub(super) ident_counter: u32,
    pub(super) class_state_stack: Vec<ClassStateInfo>,
    pub(super) class_name_stack: Vec<Option<String>>,
    pub(super) prop_default_exprs: Vec<Option<Expression<'a>>>,
    pub(super) script_rune_call_kinds: Option<&'b FxHashMap<u32, RuneKind>>,
    pub(super) experimental_async: bool,
    pub(super) custom_element: bool,
    /// Svelte-ignore directives (populated in analyze, includes span-based JS comment lookups).
    pub(super) ignore_data: &'b IgnoreData,
    /// Stack of enclosing statement start positions for ignore lookups.
    /// Pushed on enter_*_statement, popped on exit_*_statement.
    pub(super) enclosing_stmt_start: Vec<u32>,
}

impl<'b, 'a> ScriptTransformer<'b, 'a> {
    pub(super) fn is_in_ignored_stmt(&self, code: &str) -> bool {
        self.enclosing_stmt_start
            .last()
            .is_some_and(|&start| self.ignore_data.is_ignored_at_span(start, code))
    }

    pub(super) fn rune_for_binding(
        &self,
        id: &oxc_ast::ast::BindingIdentifier<'a>,
    ) -> Option<(RuneKind, bool)> {
        let sym_id = id.symbol_id.get()?;
        let kind = self.component_scoping.rune_kind(sym_id)?;
        Some((kind, self.component_scoping.is_mutated(sym_id)))
    }

    pub(super) fn rune_for_ref(
        &self,
        id: &oxc_ast::ast::IdentifierReference<'a>,
    ) -> Option<(RuneKind, bool)> {
        let ref_id = id.reference_id.get()?;
        let sym_id = self.scoping.get_reference(ref_id).symbol_id()?;
        let kind = self.component_scoping.rune_kind(sym_id)?;
        Some((kind, self.component_scoping.is_mutated(sym_id)))
    }

    pub(super) fn prop_kind_for_ref(
        &self,
        id: &oxc_ast::ast::IdentifierReference<'a>,
    ) -> Option<PropKind> {
        let ref_id = id.reference_id.get()?;
        let sym_id = self.scoping.get_reference(ref_id).symbol_id()?;
        if self.scoping.symbol_scope_id(sym_id) != self.scoping.root_scope_id() {
            return None;
        }
        if self.component_scoping.is_prop_source(sym_id) {
            Some(PropKind::Source)
        } else if let Some(prop_name) = self.component_scoping.prop_non_source_name(sym_id) {
            Some(PropKind::NonSource(prop_name.to_string()))
        } else {
            None
        }
    }

    pub(super) fn is_rest_prop_ref(&self, id: &oxc_ast::ast::IdentifierReference<'a>) -> bool {
        let Some(ref_id) = id.reference_id.get() else {
            return false;
        };
        let Some(sym_id) = self.scoping.get_reference(ref_id).symbol_id() else {
            return false;
        };
        self.component_scoping.is_rest_prop(sym_id)
    }

    pub(super) fn extract_assign_member_store_root<'t>(
        &self,
        target: &'t oxc_ast::ast::AssignmentTarget<'a>,
    ) -> Option<(&'t str, &'t str)> {
        let name = svelte_transform::rune_refs::find_expr_root_name(
            target.as_member_expression()?.object(),
        )?;
        self.component_scoping
            .store_base_name(name)
            .map(|base| (name, base))
    }

    pub(super) fn extract_simple_member_store_root<'t>(
        &self,
        target: &'t oxc_ast::ast::SimpleAssignmentTarget<'a>,
    ) -> Option<(&'t str, &'t str)> {
        let name = svelte_transform::rune_refs::find_expr_root_name(
            target.as_member_expression()?.object(),
        )?;
        self.component_scoping
            .store_base_name(name)
            .map(|base| (name, base))
    }
}
