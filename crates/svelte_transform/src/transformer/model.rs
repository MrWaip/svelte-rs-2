use oxc_ast::ast::Expression;
use oxc_semantic::SymbolId;
use oxc_syntax::node::NodeId as OxcNodeId;
use oxc_syntax::scope::ScopeId;

use crate::data::TransformData;
use rustc_hash::{FxHashMap, FxHashSet};
use svelte_ast::{Component, NodeId as SvelteNodeId};

use svelte_analyze::{AnalysisData, BindingSemantics, ComponentScoping, IdentGen, JsAst};

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
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum AsyncDerivedMode {
    Await,
    Save,
}

#[derive(Default)]
pub(crate) struct ClassStateInfo {
    pub(crate) backing: FxHashMap<OxcNodeId, String>,

    pub(crate) ctor_synth_nodes: Vec<OxcNodeId>,

    pub(crate) ctor_placeholder_names: FxHashSet<String>,

    pub(crate) has_rune_field: bool,
}

impl ClassStateInfo {
    pub(crate) fn is_empty(&self) -> bool {
        !self.has_rune_field
    }
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

    pub(crate) transform_data: TransformData,
    pub(crate) b: &'b Builder<'a>,
    pub(crate) component_scoping: &'b ComponentScoping<'a>,

    pub(crate) analysis: Option<&'b AnalysisData<'a>>,
    pub(crate) runes: bool,
    pub(crate) accessors: bool,
    pub(crate) immutable: bool,
    pub(crate) strip_exports: bool,
    pub(crate) dev: bool,
    pub(crate) function_info_stack: Vec<FunctionInfo>,
    pub(crate) has_tracing: bool,
    pub(crate) needs_ownership_validator: bool,
    pub(crate) pending_prop_update_validations: FxHashMap<u32, PendingPropMutationValidation<'a>>,
    pub(crate) component_source: &'b str,
    pub(crate) component_line_index: &'b svelte_span::LineIndex,
    pub(crate) filename: &'b str,
    pub(crate) next_arrow_name: Option<String>,
    pub(crate) ident_gen: &'b mut IdentGen,
    pub(crate) class_name_stack: Vec<Option<String>>,
    pub(crate) experimental_async: bool,

    pub(crate) ignore_query: IgnoreQuery<'b, 'a>,

    pub(crate) enclosing_stmt_start: Vec<u32>,

    pub(crate) template_owner_node: Option<SvelteNodeId>,

    pub(crate) in_bind_setter_traverse: bool,

    pub(crate) destructure_lhs_depth: u32,

    pub(crate) gen_arrow_scope: Option<ScopeId>,

    pub(crate) parsed: Option<&'b mut JsAst<'a>>,

    pub(crate) component: Option<&'b Component>,
}

impl<'b, 'a> ComponentTransformer<'b, 'a> {
    pub(crate) fn is_in_ignored_stmt(&self, code: &str) -> bool {
        self.enclosing_stmt_start
            .last()
            .is_some_and(|&start| self.ignore_query.is_ignored_at_span(start, code))
    }

    pub(crate) fn binding_semantics_for_symbol(
        &self,
        sym_id: SymbolId,
    ) -> Option<BindingSemantics> {
        let analysis = self.analysis.as_ref()?;
        Some(analysis.binding_semantics(sym_id))
    }
}
