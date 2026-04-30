use oxc_allocator::{Allocator, CloneIn};
use oxc_ast::AstBuilder;
use oxc_ast::ast::Statement;
use oxc_span::{SPAN, SourceType};
use oxc_traverse::ReusableTraverseCtx;

use oxc_syntax::node::NodeId as OxcNodeId;

use svelte_analyze::{AnalysisData, ComponentScoping, JsAst};
use svelte_ast_builder::Builder;

use super::model::{ComponentTransformer, IgnoreQuery, TransformMode};
use crate::data::TransformData;

pub(crate) fn run_template<'a>(
    alloc: &'a Allocator,
    analysis: &AnalysisData<'a>,
    component_scoping: &ComponentScoping<'a>,
    expr_handles: Vec<(OxcNodeId, Option<svelte_ast::NodeId>)>,
    stmt_handles: Vec<(OxcNodeId, Option<svelte_ast::NodeId>)>,
    bind_expr_handles: Vec<(OxcNodeId, svelte_ast::NodeId)>,
    transform_data: TransformData,
    parsed: &mut JsAst<'a>,
    dev: bool,
) -> TransformData {
    let b = Builder::new(alloc);
    let is_ts = parsed.typescript;

    let mut transformer = ComponentTransformer {
        mode: TransformMode::Template,
        transform_data,
        b: &b,
        component_scoping,
        analysis: Some(analysis),
        runes: false,
        accessors: false,
        immutable: false,
        derived_pending: rustc_hash::FxHashSet::default(),
        async_derived_pending: rustc_hash::FxHashMap::default(),
        strip_exports: false,
        dev,
        is_ts,
        function_info_stack: Vec::new(),
        has_tracing: false,
        needs_ownership_validator: false,
        pending_prop_update_validations: rustc_hash::FxHashMap::default(),
        component_source: "",
        script_content_start: 0,
        filename: "",
        next_arrow_name: None,
        ident_counter: 0,
        class_state_stack: Vec::new(),
        class_name_stack: Vec::new(),
        script_rune_calls: None,
        script_node_id_offset: 0,
        experimental_async: false,
        ignore_query: IgnoreQuery::empty(),
        enclosing_stmt_start: Vec::new(),
        template_owner_node: None,
        in_bind_setter_traverse: false,
    };

    let ast = AstBuilder::new(alloc);
    let mut program = ast.program(
        SPAN,
        SourceType::mjs(),
        "",
        oxc_allocator::Vec::new_in(alloc),
        None,
        oxc_allocator::Vec::new_in(alloc),
        oxc_allocator::Vec::new_in(alloc),
    );
    program
        .scope_id
        .set(Some(component_scoping.root_scope_id()));

    let mut reusable = ReusableTraverseCtx::new((), oxc_semantic::Scoping::default(), alloc);

    for (handle, owner) in expr_handles {
        let Some(expr) = parsed.take_expr(handle) else {
            continue;
        };
        transformer.template_owner_node = owner;
        let stmt = ast.statement_expression(SPAN, expr);
        program.body.clear();
        program.body.push(stmt);

        oxc_traverse::traverse_mut_with_ctx(&mut transformer, &mut program, &mut reusable);

        let Statement::ExpressionStatement(es) = program
            .body
            .pop()
            .expect("body was pushed with a single statement above")
        else {
            unreachable!()
        };
        parsed.replace_expr(handle, es.unbox().expression);
    }

    for (handle, owner) in stmt_handles {
        let Some(stmt) = parsed.take_stmt(handle) else {
            continue;
        };
        transformer.template_owner_node = owner;
        program.body.clear();
        program.body.push(stmt);

        oxc_traverse::traverse_mut_with_ctx(&mut transformer, &mut program, &mut reusable);

        parsed.replace_stmt(
            handle,
            program
                .body
                .pop()
                .expect("body was pushed with a single statement above"),
        );
    }

    for (handle, owner) in bind_expr_handles {
        let Some(orig) = parsed.take_expr(handle) else {
            continue;
        };

        let setter_lhs_expr = orig.clone_in_with_semantic_ids(alloc);

        transformer.template_owner_node = Some(owner);

        program.body.clear();
        program.body.push(ast.statement_expression(SPAN, orig));
        oxc_traverse::traverse_mut_with_ctx(&mut transformer, &mut program, &mut reusable);
        let Statement::ExpressionStatement(es) = program
            .body
            .pop()
            .expect("body was pushed with a single statement above")
        else {
            unreachable!()
        };
        let getter_body = es.unbox().expression;

        let value_ident = b.rid_expr("$$value");
        let assign_target = b.expr_to_assignment_target(setter_lhs_expr);
        let assign_expr = b.assign_expr_raw(assign_target, value_ident);

        program.body.clear();
        program
            .body
            .push(ast.statement_expression(SPAN, assign_expr));
        transformer.in_bind_setter_traverse = true;
        oxc_traverse::traverse_mut_with_ctx(&mut transformer, &mut program, &mut reusable);
        transformer.in_bind_setter_traverse = false;
        let Statement::ExpressionStatement(es) = program
            .body
            .pop()
            .expect("body was pushed with a single statement above")
        else {
            unreachable!()
        };
        let setter_body = es.unbox().expression;

        let getter = b.thunk(getter_body);
        let setter = b.arrow_expr(b.params(["$$value"]), [b.expr_stmt(setter_body)]);
        let seq = b.seq_expr([getter, setter]);

        parsed.replace_expr(handle, seq);
    }

    transformer.transform_data
}
