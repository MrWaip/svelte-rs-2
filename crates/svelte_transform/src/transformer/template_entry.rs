use std::iter;

use oxc_allocator::{Allocator, CloneIn, Vec as OxcVec};
use oxc_ast::AstBuilder;
use oxc_ast::ast::{Expression, Statement};
use oxc_span::{SPAN, SourceType};
use oxc_traverse::{ReusableTraverseCtx, traverse_mut_with_ctx};

use oxc_syntax::node::NodeId as OxcNodeId;

use svelte_analyze::{
    AnalysisData, AttributeSemantics, BindingSemantics, ComponentScoping, HtmlBindKind, IdentGen,
    JsAst,
};
use svelte_ast::NodeId as SvelteNodeId;
use svelte_ast_builder::{Arg, Builder};

use super::model::{ComponentTransformer, IgnoreQuery, TransformMode};
use crate::data::TransformData;
use crate::{BindExprHandle, BindHandleKind};

pub(crate) fn run_template<'a, 'b>(
    alloc: &'a Allocator,
    analysis: &'b AnalysisData<'a>,
    component_scoping: &'b ComponentScoping<'a>,
    ident_gen: &'b mut IdentGen,
    expr_handles: Vec<(OxcNodeId, Option<SvelteNodeId>)>,
    stmt_handles: Vec<(OxcNodeId, Option<SvelteNodeId>)>,
    bind_expr_handles: Vec<BindExprHandle>,
    transform_data: TransformData,
    parsed: &mut JsAst<'a>,
    line_index: &'b svelte_span::LineIndex,
    dev: bool,
) -> TransformData {
    let b = Builder::new(alloc);
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
        function_info_stack: Vec::new(),
        has_tracing: false,
        needs_ownership_validator: false,
        pending_prop_update_validations: rustc_hash::FxHashMap::default(),
        component_source: "",
        component_line_index: line_index,
        filename: "",
        next_arrow_name: None,
        ident_gen,
        class_state_stack: Vec::new(),
        class_name_stack: Vec::new(),
        experimental_async: false,
        ignore_query: IgnoreQuery::empty(),
        enclosing_stmt_start: Vec::new(),
        template_owner_node: None,
        in_bind_setter_traverse: false,
        gen_arrow_scope: None,
    };

    let ast = AstBuilder::new(alloc);
    let mut program = ast.program(
        SPAN,
        SourceType::mjs(),
        "",
        OxcVec::new_in(alloc),
        None,
        OxcVec::new_in(alloc),
        OxcVec::new_in(alloc),
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

        traverse_mut_with_ctx(&mut transformer, &mut program, &mut reusable);

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

        traverse_mut_with_ctx(&mut transformer, &mut program, &mut reusable);

        parsed.replace_stmt(
            handle,
            program
                .body
                .pop()
                .expect("body was pushed with a single statement above"),
        );
    }

    for BindExprHandle {
        bind_id: handle,
        owner,
        kind: bind_handle_kind,
    } in bind_expr_handles
    {
        let Some(orig) = parsed.take_expr(handle) else {
            continue;
        };

        let setter_lhs_expr = orig.clone_in_with_semantic_ids(alloc);
        let store_base_symbol: Option<oxc_semantic::SymbolId> =
            match analysis.attributes.get(owner) {
                AttributeSemantics::ElementBind(b) => match &b.kind {
                    HtmlBindKind::StoreSubscribed { base_symbol } => {
                        Some(*base_symbol)
                    }
                    _ => None,
                },
                AttributeSemantics::WindowBind(b) => match &b.kind {
                    HtmlBindKind::StoreSubscribed { base_symbol } => {
                        Some(*base_symbol)
                    }
                    _ => None,
                },
                AttributeSemantics::DocumentBind(b) => match &b.kind {
                    HtmlBindKind::StoreSubscribed { base_symbol } => {
                        Some(*base_symbol)
                    }
                    _ => None,
                },
                _ => None,
            };

        transformer.template_owner_node = Some(owner);

        program.body.clear();
        program.body.push(ast.statement_expression(SPAN, orig));
        traverse_mut_with_ctx(&mut transformer, &mut program, &mut reusable);
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
        traverse_mut_with_ctx(&mut transformer, &mut program, &mut reusable);
        transformer.in_bind_setter_traverse = false;
        let Statement::ExpressionStatement(es) = program
            .body
            .pop()
            .expect("body was pushed with a single statement above")
        else {
            unreachable!()
        };
        let setter_body = es.unbox().expression;

        if let BindHandleKind::Component { prop_name } = &bind_handle_kind {
            let obj = transform_component_bind_pair(&b, prop_name, getter_body, setter_body);
            parsed.replace_expr(handle, obj);
            continue;
        }

        let (getter, setter) = if let Some(base_symbol) = store_base_symbol {
            let base_name = analysis.scoping.symbol_name(base_symbol);
            let dollar_name: &str = b.alloc_str(&format!("${base_name}"));
            let base_via_legacy_state = matches!(
                analysis.binding_semantics(base_symbol),
                BindingSemantics::LegacyState(_)
            );
            let getter_expr = if base_via_legacy_state {
                let thunk_call = b.call_expr_callee(
                    b.rid_expr(dollar_name),
                    iter::empty::<Arg<'_, '_>>(),
                );
                b.named_function_expr(
                    "get",
                    b.no_params(),
                    vec![b.return_stmt(thunk_call)],
                    false,
                )
            } else {
                b.rid_expr(dollar_name)
            };
            (
                getter_expr,
                if dev {
                    b.named_function_expr(
                        "set",
                        b.params(["$$value"]),
                        vec![b.expr_stmt(setter_body)],
                        false,
                    )
                } else {
                    b.arrow_expr(b.params(["$$value"]), [b.expr_stmt(setter_body)])
                },
            )
        } else if dev {
            (
                b.named_function_expr(
                    "get",
                    b.no_params(),
                    vec![b.return_stmt(getter_body)],
                    false,
                ),
                b.named_function_expr(
                    "set",
                    b.params(["$$value"]),
                    vec![b.expr_stmt(setter_body)],
                    false,
                ),
            )
        } else {
            (
                b.thunk(getter_body),
                b.arrow_expr(b.params(["$$value"]), [b.expr_stmt(setter_body)]),
            )
        };
        let seq = b.seq_expr([getter, setter]);

        parsed.replace_expr(handle, seq);
    }

    transformer.transform_data
}

fn transform_component_bind_pair<'a>(
    b: &Builder<'a>,
    prop_name: &str,
    getter_body: Expression<'a>,
    setter_body: Expression<'a>,
) -> Expression<'a> {
    let key: &'a str = b.alloc_str(prop_name);
    b.object_expr([
        svelte_ast_builder::ObjProp::Getter(key, getter_body),
        svelte_ast_builder::ObjProp::Setter(
            key,
            "$$value",
            None,
            vec![b.expr_stmt(setter_body)],
        ),
    ])
}
