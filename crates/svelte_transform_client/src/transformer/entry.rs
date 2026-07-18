use oxc_allocator::Allocator;
use oxc_ast::ast::Program;
use oxc_semantic::Scoping;
use oxc_traverse::traverse_mut;

use crate::data::{RestExcludes, TransformData};

use svelte_analyze::{AnalysisData, ComponentScoping, IdentGen};
use svelte_ast_builder::Builder;

use super::model::{ComponentTransformer, IgnoreQuery, TransformMode};

pub struct TransformScriptOutput {
    pub has_tracing: bool,
    pub needs_ownership_validator: bool,
    pub rest_excludes: Vec<RestExcludes>,
}

pub fn transform_script<'a, 'b>(
    allocator: &'a Allocator,
    program: &mut Program<'a>,
    b: &'b Builder<'a>,
    analysis: Option<&'b AnalysisData<'a>>,
    component_scoping: &'b ComponentScoping<'a>,
    ident_gen: &'b mut IdentGen,
    strip_exports: bool,
    dev: bool,
    component_source: &str,
    component_line_index: &svelte_span::LineIndex,
    filename: &str,
    runes: bool,
    accessors: bool,
    immutable: bool,
    experimental_async: bool,
    ignore_query: IgnoreQuery<'_, 'a>,
) -> TransformScriptOutput {
    let mut transformer = ComponentTransformer {
        mode: TransformMode::Script,
        transform_data: TransformData::new(),
        b,
        component_scoping,
        analysis,
        runes,
        accessors,
        immutable,
        strip_exports,
        dev,
        function_info_stack: Vec::new(),
        has_tracing: false,
        needs_ownership_validator: false,
        pending_prop_update_validations: rustc_hash::FxHashMap::default(),
        component_source,
        component_line_index,
        filename,
        next_arrow_name: None,
        ident_gen,
        class_name_stack: Vec::new(),
        experimental_async,
        ignore_query,
        enclosing_stmt_start: Vec::new(),
        template_owner_node: None,
        rewrite_top_level_declarations: false,
        in_bind_setter_traverse: false,
        dispatched_member_assignments: rustc_hash::FxHashSet::default(),
        destructure_lhs_depth: 0,
        gen_arrow_scope: None,
        parsed: None,
        component: None,
    };

    let empty_scoping = Scoping::default();
    traverse_mut(&mut transformer, allocator, program, empty_scoping, ());

    if let Some(analysis) = analysis {
        super::legacy_reactive::rewrite_legacy_reactive(b, program, analysis);
    }

    TransformScriptOutput {
        has_tracing: transformer.has_tracing,
        needs_ownership_validator: transformer.needs_ownership_validator,
        rest_excludes: transformer.transform_data.rest_excludes,
    }
}
