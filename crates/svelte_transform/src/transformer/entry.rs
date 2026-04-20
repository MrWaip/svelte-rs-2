//! Public entry point for running the script transformer over an OXC Program.
//!
//! The caller (svelte_codegen_client) passes a parsed Program plus all context,
//! this module constructs the ComponentTransformer internally and runs the Traverse
//! pass, returning the modified Program plus post-traverse flags the caller needs
//! for further codegen decisions.

use oxc_allocator::Allocator;
use oxc_ast::ast::Program;
use oxc_semantic::SemanticBuilder;
use oxc_traverse::traverse_mut;

use svelte_analyze::{AnalysisData, ComponentScoping, ScriptRuneCalls};
use svelte_ast_builder::Builder;

use super::model::{ComponentTransformer, IgnoreQuery, TransformMode};

pub struct TransformScriptOutput {
    pub has_tracing: bool,
    pub needs_ownership_validator: bool,
}

pub fn transform_script<'a, 'b>(
    allocator: &'a Allocator,
    program: &mut Program<'a>,
    b: &'b Builder<'a>,
    analysis: Option<&'b AnalysisData<'a>>,
    component_scoping: &'b ComponentScoping<'a>,
    script_rune_calls: Option<&ScriptRuneCalls>,
    script_node_id_offset: u32,
    strip_exports: bool,
    dev: bool,
    component_source: &str,
    script_content_start: u32,
    filename: &str,
    runes: bool,
    accessors: bool,
    immutable: bool,
    experimental_async: bool,
    ignore_query: IgnoreQuery<'_, 'a>,
    prepare_semantic: bool,
) -> TransformScriptOutput {
    let is_ts = program.source_type.is_typescript();
    if prepare_semantic {
        let _ = SemanticBuilder::new().build(program);
    }

    let mut transformer = ComponentTransformer {
        mode: TransformMode::Script,
        transform_data: crate::data::TransformData::new(),
        b,
        component_scoping,
        analysis,
        runes,
        accessors,
        immutable,
        derived_pending: rustc_hash::FxHashSet::default(),
        async_derived_pending: rustc_hash::FxHashMap::default(),
        strip_exports,
        dev,
        is_ts,
        function_info_stack: Vec::new(),
        has_tracing: false,
        needs_ownership_validator: false,
        pending_prop_update_validations: rustc_hash::FxHashMap::default(),
        component_source,
        script_content_start,
        filename,
        next_arrow_name: None,
        ident_counter: 0,
        class_state_stack: Vec::new(),
        class_name_stack: Vec::new(),
        script_rune_calls,
        script_node_id_offset,
        experimental_async,
        ignore_query,
        enclosing_stmt_start: Vec::new(),
        template_owner_node: None,
        in_bind_setter_traverse: false,
    };

    let empty_scoping = oxc_semantic::Scoping::default();
    traverse_mut(&mut transformer, allocator, program, empty_scoping, ());

    if !transformer.derived_pending.is_empty() {
        let dev_ctx = dev.then_some(super::derived::DevContext {
            component_source,
            script_content_start,
            filename,
            ignore_query: transformer.ignore_query,
        });
        super::derived::wrap_derived_thunks(
            b,
            program,
            &transformer.derived_pending,
            &transformer.async_derived_pending,
            dev_ctx.as_ref(),
        );
    }

    TransformScriptOutput {
        has_tracing: transformer.has_tracing,
        needs_ownership_validator: transformer.needs_ownership_validator,
    }
}
