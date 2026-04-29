use oxc_ast::ast::Expression;
use rustc_hash::{FxHashMap, FxHashSet};

use svelte_analyze::{
    AnalysisData, BindingSemantics, ComponentScoping, DerivedKind, RuneKind, ScriptRuneCalls,
    StateKind,
};

use svelte_ast_builder::Builder;

pub(crate) struct PendingPropMutationValidation<'a> {
    pub(crate) prop_alias: String,
    pub(crate) root_name: String,
    pub(crate) segments: Vec<Expression<'a>>,
}

pub(crate) struct FunctionInfo {
    pub(crate) is_async: bool,
    pub(crate) name: Option<String>,
    pub(crate) span_start: u32,
    pub(crate) in_constructor: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum AsyncDerivedMode {
    Await,
    Save,
}

pub(crate) struct ClassStateField {
    pub(crate) public_name: Option<String>,
    pub(crate) private_name: String,
    pub(crate) rune_kind: RuneKind,
}

pub(crate) struct ClassStateInfo {
    pub(crate) fields: Vec<ClassStateField>,

    pub(crate) ctor_synth_names: FxHashSet<String>,

    pub(crate) ctor_placeholder_names: FxHashSet<String>,
}

#[derive(Clone, Copy)]
pub struct IgnoreQuery<'d, 'a> {
    analysis: Option<&'d AnalysisData<'a>>,
}

impl<'d, 'a> IgnoreQuery<'d, 'a> {
    pub fn new(analysis: &'d AnalysisData<'a>) -> Self {
        Self {
            analysis: Some(analysis),
        }
    }

    pub fn empty() -> Self {
        Self { analysis: None }
    }

    pub(crate) fn is_ignored_at_span(&self, span_start: u32, code: &str) -> bool {
        self.analysis
            .is_some_and(|a| a.output.ignore_data.is_ignored_at_span(span_start, code))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum TransformMode {
    Template,
    Script,
}

pub(crate) struct ComponentTransformer<'b, 'a> {
    pub(crate) mode: TransformMode,

    pub(crate) transform_data: crate::data::TransformData,
    pub(crate) b: &'b Builder<'a>,
    pub(crate) component_scoping: &'b ComponentScoping<'a>,

    pub(crate) analysis: Option<&'b AnalysisData<'a>>,
    pub(crate) runes: bool,
    pub(crate) accessors: bool,
    pub(crate) immutable: bool,
    pub(crate) derived_pending: FxHashSet<oxc_semantic::SymbolId>,

    pub(crate) async_derived_pending: FxHashMap<oxc_semantic::SymbolId, AsyncDerivedMode>,
    pub(crate) strip_exports: bool,
    pub(crate) dev: bool,
    pub(crate) is_ts: bool,
    pub(crate) function_info_stack: Vec<FunctionInfo>,
    pub(crate) has_tracing: bool,
    pub(crate) needs_ownership_validator: bool,
    pub(crate) pending_prop_update_validations: FxHashMap<u32, PendingPropMutationValidation<'a>>,
    pub(crate) component_source: &'b str,
    pub(crate) script_content_start: u32,
    pub(crate) filename: &'b str,
    pub(crate) next_arrow_name: Option<String>,
    pub(crate) ident_counter: u32,
    pub(crate) class_state_stack: Vec<ClassStateInfo>,
    pub(crate) class_name_stack: Vec<Option<String>>,
    pub(crate) script_rune_calls: Option<&'b ScriptRuneCalls>,
    pub(crate) script_node_id_offset: u32,
    pub(crate) experimental_async: bool,

    pub(crate) ignore_query: IgnoreQuery<'b, 'a>,

    pub(crate) enclosing_stmt_start: Vec<u32>,

    pub(crate) template_owner_node: Option<svelte_ast::NodeId>,

    pub(crate) in_bind_setter_traverse: bool,
}

impl<'b, 'a> ComponentTransformer<'b, 'a> {
    pub(crate) fn is_in_ignored_stmt(&self, code: &str) -> bool {
        self.enclosing_stmt_start
            .last()
            .is_some_and(|&start| self.ignore_query.is_ignored_at_span(start, code))
    }

    pub(crate) fn rune_for_symbol(&self, sym_id: oxc_semantic::SymbolId) -> Option<RuneKind> {
        let kind = match self.binding_semantics_for_symbol(sym_id)? {
            BindingSemantics::State(state) => match state.kind {
                StateKind::State => RuneKind::State,
                StateKind::StateRaw => RuneKind::StateRaw,
                StateKind::StateEager => RuneKind::StateEager,
            },
            BindingSemantics::Derived(derived) => match derived.kind {
                DerivedKind::Derived => RuneKind::Derived,
                DerivedKind::DerivedBy => RuneKind::DerivedBy,
            },
            _ => return None,
        };
        Some(kind)
    }

    pub(crate) fn binding_semantics_for_symbol(
        &self,
        sym_id: oxc_semantic::SymbolId,
    ) -> Option<BindingSemantics> {
        let analysis = self.analysis.as_ref()?;
        Some(analysis.binding_semantics(sym_id))
    }
}
