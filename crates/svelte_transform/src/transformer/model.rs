use oxc_ast::ast::Expression;
use rustc_hash::{FxHashMap, FxHashSet};

use svelte_analyze::{
    AnalysisData, ComponentScoping, DeclarationSemantics, DerivedKind, RuneKind, ScriptRuneCalls,
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
    /// Public field names whose backing/getter/setter come from constructor rune assignments.
    pub(crate) ctor_synth_names: FxHashSet<String>,
    /// Bare placeholder declarations (`field;`) that are replaced by constructor-owned lowering.
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

/// Phase selector for `ComponentTransformer`. Determines which enter_/exit_ branches
/// run and which script-only fields are considered valid.
///
/// - `Template`: transformer is driven from the Svelte template walker over a synthetic
///   `Program` whose body cycles through each template expression/statement. Script-only
///   methods (class state, TS strip, $inspect, ownership validation, etc.) short-circuit.
/// - `Script`: transformer runs over the full instance script `Program` via standard
///   `oxc_traverse::traverse_mut`. All branches active.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum TransformMode {
    Template,
    Script,
}

pub(crate) struct ComponentTransformer<'b, 'a> {
    pub(crate) mode: TransformMode,
    /// Template-only: result of the pre-traverse fragment walk. Used by the
    /// `ConstAliasRead` branch in template-mode `enter_expression`.
    pub(crate) transform_data: crate::data::TransformData,
    pub(crate) b: &'b Builder<'a>,
    pub(crate) component_scoping: &'b ComponentScoping<'a>,
    /// Reactive semantics + template/script analysis. Present in both Template and
    /// Script modes (wrapped in `Option` for module-script codegen which has no
    /// component analysis). Methods that previously went through `CodegenView`
    /// (`reference_semantics`, `declaration_semantics`, `binding_origin_key`,
    /// etc.) are available directly on `AnalysisData`.
    pub(crate) analysis: Option<&'b AnalysisData<'a>>,
    pub(crate) runes: bool,
    pub(crate) accessors: bool,
    pub(crate) immutable: bool,
    pub(crate) derived_pending: FxHashSet<oxc_semantic::SymbolId>,
    /// Subset of `derived_pending`: symbols whose `$derived` init was `$derived(await expr)`.
    /// Used by `wrap_derived_thunks` to determine async thunk form and outer wrapping
    /// after dev transforms run.
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
    /// Svelte-ignore queries for span-based JS comment lookups.
    pub(crate) ignore_query: IgnoreQuery<'b, 'a>,
    /// Stack of enclosing statement start positions for ignore lookups.
    /// Pushed on enter_*_statement, popped on exit_*_statement.
    pub(crate) enclosing_stmt_start: Vec<u32>,
    /// Template node owning the expression/statement currently being
    /// processed (Template mode only). Seeded by the template-entry
    /// driver before each per-handle traverse run so node-level predicates
    /// like `is_ignored(node_id, "await_reactivity_loss")` can resolve on
    /// the correct owner. `None` when the owner is unknown (concat parts).
    pub(crate) template_owner_node: Option<svelte_ast::NodeId>,
    /// True while the bind-directive transform pass is visiting the
    /// synthetic `<lhs> = $$value` assignment that produces a bind setter.
    /// The assignment LHS is guaranteed to be a primitive target (bound to
    /// a DOM element property), so the `$.set` call must skip its proxy
    /// third argument. Reference compiler's equivalent lives in
    /// `AssignmentExpression.js` (`is_primitive` check on `BindDirective`
    /// in `context.path`).
    pub(crate) in_bind_setter_traverse: bool,
}

impl<'b, 'a> ComponentTransformer<'b, 'a> {
    pub(crate) fn is_in_ignored_stmt(&self, code: &str) -> bool {
        self.enclosing_stmt_start
            .last()
            .is_some_and(|&start| self.ignore_query.is_ignored_at_span(start, code))
    }

    pub(crate) fn rune_for_binding(
        &self,
        id: &oxc_ast::ast::BindingIdentifier<'a>,
    ) -> Option<RuneKind> {
        self.rune_for_symbol(id.symbol_id.get()?)
    }

    pub(crate) fn rune_for_symbol(&self, sym_id: oxc_semantic::SymbolId) -> Option<RuneKind> {
        let kind = match self.declaration_semantics_for_symbol(sym_id)? {
            DeclarationSemantics::State(state) => match state.kind {
                StateKind::State => RuneKind::State,
                StateKind::StateRaw => RuneKind::StateRaw,
                StateKind::StateEager => RuneKind::StateEager,
            },
            DeclarationSemantics::Derived(derived) => match derived.kind {
                DerivedKind::Derived => RuneKind::Derived,
                DerivedKind::DerivedBy => RuneKind::DerivedBy,
            },
            _ => return None,
        };
        Some(kind)
    }

    pub(crate) fn declaration_semantics_for_symbol(
        &self,
        sym_id: oxc_semantic::SymbolId,
    ) -> Option<DeclarationSemantics> {
        let analysis = self.analysis.as_ref()?;
        Some(analysis.declaration_semantics(self.component_scoping.symbol_declaration(sym_id)))
    }
}
