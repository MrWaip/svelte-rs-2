pub(crate) mod codegen;
mod context;
mod custom_element;
mod script;

use oxc_allocator::Allocator;
use oxc_ast::ast::{ExportDefaultDeclarationKind, Statement};
use oxc_codegen::Codegen;
use oxc_span::{GetSpanMut, Span};

use svelte_analyze::AnalysisData;
use svelte_ast::{Attribute, Node};
use svelte_ast_builder::{Arg, AssignLeft, Builder, ObjProp};
use svelte_transform::TransformData;

use context::Ctx;

pub fn generate<'a>(
    compile_ctx: svelte_types::CompileContext<'a, 'a>,
    options: &svelte_types::CodegenOptions,
    transform_data: TransformData,
    css_text: Option<&str>,
) -> String {
    let alloc = compile_ctx.alloc;
    let component = compile_ctx.component;
    let analysis = compile_ctx.analysis;
    let dev = options.dev;
    let mut ctx = Ctx::new(compile_ctx, options, transform_data, css_text);

    let script_output = script::gen_script(&mut ctx, dev);
    let script_imports = script_output.imports;
    let script_body = script_output.body;
    let has_tracing = script_output.has_tracing;
    let needs_ownership_validator = script_output.needs_ownership_validator;
    let mut script_comments = script_output.comments;
    let mut script_source_text = script_output.source_text;
    let mut script_span_end = script_output.program_span_end;

    let mut module_imports: Vec<Statement<'_>> = Vec::new();
    let mut module_body: Vec<Statement<'_>> = Vec::new();
    if let Some(module_script) = component.module_script.as_ref() {
        let module_source = component.source_text(module_script.content_span);
        let mut module_output = if let Some(program) = ctx.state.parsed.module_program.take() {
            script::transform_component_module_program(
                alloc,
                program,
                Some(analysis),
                &analysis.scoping,
                Some(analysis.script_rune_calls()),
            )
        } else {
            let is_ts = module_script.language == svelte_ast::ScriptLanguage::TypeScript;
            script::transform_component_module_script(alloc, module_source, is_ts)
        };

        if script_source_text.is_empty() {
            script_comments = module_output.comments;
            script_source_text = module_output.source_text;
            script_span_end = module_output.program_span_end;
            module_imports = module_output.imports;
            module_body = module_output.body;
        } else {
            let module_offset = script_span_end + 1;
            shift_statement_spans(&mut module_output.imports, module_offset);
            shift_statement_spans(&mut module_output.body, module_offset);
            shift_comments(&mut module_output.comments, module_offset);

            let combined_source =
                alloc.alloc_str(&format!("{script_source_text}\n{module_source}"));
            script_source_text = combined_source;
            script_span_end = module_offset + module_output.program_span_end;
            script_comments.extend(module_output.comments);
            module_imports = module_output.imports;
            module_body = module_output.body;
        }
    }

    let codegen_result = codegen::codegen_root_fragment(&mut ctx).expect("codegen failed");
    let hoisted = codegen_result.hoisted;
    let template_body = codegen_result.body;
    let instance_snippets = codegen_result.instance_snippets;
    let hoistable_snippets = codegen_result.hoistable_snippets;

    let mut all_hoisted: Vec<Statement<'_>> = Vec::new();
    all_hoisted.append(&mut ctx.state.module_hoisted);
    all_hoisted.extend(hoisted);

    let runtime = ctx.runtime_plan();

    let mut fn_body: Vec<Statement<'_>> = Vec::new();

    if ctx.query.view.inject_styles() && ctx.state.css_text.is_some() {
        fn_body.push(ctx.b.expr_stmt(ctx.b.call_expr(
            "$.append_styles",
            [Arg::Ident("$$anchor"), Arg::Ident("$$css")],
        )));
    }

    if let Some(props_id_name) = ctx.query.props_id() {
        let name: &str = ctx.b.alloc_str(props_id_name);
        let call = ctx
            .b
            .call_expr("$.props_id", std::iter::empty::<Arg<'_, '_>>());
        fn_body.push(ctx.b.const_stmt(name, call));
    }

    if ctx.query.needs_sanitized_legacy_slots() {
        fn_body.push(ctx.b.const_stmt(
            "$$slots",
            ctx.b.call_expr("$.sanitize_slots", [Arg::Ident("$$props")]),
        ));
    }

    if ctx.state.dev {
        fn_body.push(
            ctx.b.expr_stmt(
                ctx.b
                    .call_expr("$.check_target", [Arg::Expr(ctx.b.new_target_expr())]),
            ),
        );
    }
    if runtime.needs_push {
        let mut push_args: Vec<Arg<'_, '_>> = vec![
            Arg::Ident("$$props"),
            Arg::Expr(ctx.b.bool_expr(ctx.query.runes())),
        ];
        if ctx.state.dev {
            push_args.push(Arg::Ident(ctx.state.name));
        }
        fn_body.push(ctx.b.expr_stmt(ctx.b.call_expr("$.push", push_args)));
    }
    if ctx.state.needs_binding_group {
        fn_body.push(ctx.b.const_stmt("binding_group", ctx.b.empty_array_expr()));
    }

    if ctx.state.dev && needs_ownership_validator {
        fn_body.push(
            ctx.b.var_stmt(
                "$$ownership_validator",
                ctx.b
                    .call_expr("$.create_ownership_validator", [Arg::Ident("$$props")]),
            ),
        );
    }

    if runtime.has_stores {
        let scoping = ctx.query.scoping();
        let mut store_names: Vec<&str> = ctx
            .query
            .view
            .iter_store_declarations()
            .map(|(_, store)| scoping.symbol_name(store.base_symbol))
            .collect();
        store_names.sort();

        for base_name in &store_names {
            let mut dollar_name = String::with_capacity(1 + base_name.len());
            dollar_name.push('$');
            dollar_name.push_str(base_name);
            let dollar_name_str: &str = ctx.b.alloc_str(&dollar_name);
            let base_str: &str = ctx.b.alloc_str(base_name);
            let store_get = ctx.b.call_expr(
                "$.store_get",
                [
                    Arg::Ident(base_str),
                    Arg::Str(dollar_name.clone()),
                    Arg::Ident("$$stores"),
                ],
            );

            let thunk_body = if ctx.state.dev {
                let validate = ctx.b.call_expr(
                    "$.validate_store",
                    [
                        Arg::Ident(base_str),
                        Arg::StrRef(ctx.b.alloc_str(base_name)),
                    ],
                );
                ctx.b.seq_expr([validate, store_get])
            } else {
                store_get
            };
            let thunk = ctx.b.thunk(thunk_body);
            fn_body.push(ctx.b.const_stmt(dollar_name_str, thunk));
        }

        let setup_call = ctx
            .b
            .call_expr("$.setup_stores", std::iter::empty::<Arg<'_, '_>>());
        fn_body.push(
            ctx.b
                .const_array_destruct_stmt(&["$$stores", "$$cleanup"], setup_call),
        );
    }

    fn_body.extend(instance_snippets);

    if ctx.state.experimental_async && ctx.query.blocker_data().has_async() {
        let split_body = split_async_instance_body(&ctx.b, script_body, ctx.query.blocker_data());
        fn_body.extend(split_body);
    } else {
        fn_body.extend(script_body);
    }

    if runtime.has_exports || runtime.has_ce_props || ctx.query.accessors() {
        let mut export_props: Vec<ObjProp<'_>> = Vec::new();

        for e in ctx.query.exports() {
            let name: &str = ctx.b.alloc_str(&e.name);
            if let Some(alias) = &e.alias {
                let alias: &str = ctx.b.alloc_str(alias);
                export_props.push(ObjProp::KeyValue(alias, ctx.b.rid_expr(name)));
            } else {
                export_props.push(ObjProp::Shorthand(name));
            }
        }

        if ctx.query.accessors() || runtime.has_ce_props {
            if let Some(props_decl) = ctx.query.props() {
                for prop in &props_decl.props {
                    if prop.is_rest || prop.is_reserved() {
                        continue;
                    }
                    let key: &str = ctx.b.alloc_str(&prop.prop_name);
                    let local: &str = ctx.b.alloc_str(&prop.local_name);

                    let getter_expr = ctx.b.call_expr(local, std::iter::empty::<Arg<'_, '_>>());
                    export_props.push(ObjProp::Getter(key, getter_expr));

                    let default_expr = if ctx.query.runes() {
                        prop.default_text
                            .as_deref()
                            .map(|text| ctx.b.parse_expression(text))
                    } else {
                        None
                    };
                    let setter_body = vec![
                        ctx.b
                            .expr_stmt(ctx.b.call_expr(local, [Arg::Ident("$$value")])),
                        ctx.b
                            .call_stmt("$.flush", std::iter::empty::<Arg<'_, '_>>()),
                    ];
                    export_props.push(ObjProp::Setter(key, "$$value", default_expr, setter_body));
                }
            }
        }

        fn_body.push(ctx.b.var_stmt("$$exports", ctx.b.object_expr(export_props)));
    } else if ctx.state.dev && runtime.needs_push {
        let legacy_call = ctx
            .b
            .call_expr("$.legacy_api", std::iter::empty::<Arg<'_, '_>>());
        fn_body.push(ctx.b.var_stmt(
            "$$exports",
            ctx.b.object_expr([ObjProp::Spread(legacy_call)]),
        ));
    }

    if !ctx.query.runes() && ctx.query.immutable() {
        fn_body.push(ctx.b.call_stmt("$.init", [Arg::Bool(true)]));
    }

    fn_body.extend(template_body);

    if runtime.needs_push {
        if runtime.needs_pop_with_return && runtime.has_stores {
            let pop_call = ctx.b.call_expr("$.pop", [Arg::Ident("$$exports")]);
            fn_body.push(ctx.b.var_stmt("$$pop", pop_call));
            fn_body.push(
                ctx.b
                    .call_stmt("$$cleanup", std::iter::empty::<Arg<'_, '_>>()),
            );
            fn_body.push(ctx.b.return_stmt(ctx.b.rid_expr("$$pop")));
        } else if runtime.needs_pop_with_return {
            fn_body.push(
                ctx.b
                    .return_stmt(ctx.b.call_expr("$.pop", [Arg::Ident("$$exports")])),
            );
        } else {
            fn_body.push(
                ctx.b
                    .expr_stmt(ctx.b.call_expr("$.pop", std::iter::empty::<Arg<'_, '_>>())),
            );

            if runtime.has_stores {
                fn_body.push(
                    ctx.b
                        .call_stmt("$$cleanup", std::iter::empty::<Arg<'_, '_>>()),
                );
            }
        }
    } else if runtime.has_stores {
        fn_body.push(
            ctx.b
                .call_stmt("$$cleanup", std::iter::empty::<Arg<'_, '_>>()),
        );
    }

    let mut delegate_stmts: Vec<Statement<'_>> = Vec::new();
    if !ctx.state.delegated_events.is_empty() {
        let events: Vec<Arg<'_, '_>> = ctx
            .state
            .delegated_events
            .iter()
            .map(|e| Arg::Str(e.clone()))
            .collect();
        delegate_stmts.push(
            ctx.b
                .call_stmt("$.delegate", [Arg::Expr(ctx.b.array_from_args(events))]),
        );
    }

    let b = &ctx.b;

    let import_svelte = b.import_all("$", "svelte/internal/client");

    let has_bubble_events = component
        .store
        .fragment(component.root)
        .nodes
        .iter()
        .any(|&id| {
            let node = component.store.get(id);
            let attrs = match node {
                Node::SvelteWindow(w) => Some(&w.attributes),
                Node::SvelteDocument(d) => Some(&d.attributes),
                _ => None,
            };
            attrs.is_some_and(|attrs| {
                attrs.iter().any(
                    |a| matches!(a, Attribute::OnDirectiveLegacy(od) if od.expression.is_none()),
                )
            })
        });

    let has_legacy_slots = (0..component.node_count()).any(|raw_id| {
        let id = svelte_ast::NodeId(raw_id);
        matches!(component.store.get(id), Node::SlotElementLegacy(_))
    });

    let fn_params = if runtime.needs_props_param
        || has_bubble_events
        || has_legacy_slots
        || ctx.query.needs_sanitized_legacy_slots()
    {
        b.params(["$$anchor", "$$props"])
    } else {
        b.params(["$$anchor"])
    };

    let body_span = if script_span_end > 0 {
        Span::new(0, script_span_end + 1)
    } else {
        Span::default()
    };
    let fn_decl = b.function_decl(b.bid(ctx.state.name), fn_body, fn_params, body_span);
    let export_default = b.export_default(ExportDefaultDeclarationKind::FunctionDeclaration(
        b.alloc(fn_decl),
    ));

    let mut program_body: Vec<Statement<'_>> = Vec::new();
    if ctx.state.experimental_async {
        program_body.push(b.bare_import("svelte/internal/flags/async"));
    }
    if !ctx.query.runes() {
        program_body.push(b.bare_import("svelte/internal/flags/legacy"));
    }
    if has_tracing || ctx.state.has_tracing {
        program_body.push(b.bare_import("svelte/internal/flags/tracing"));
    }
    if ctx.state.dev {
        let left = AssignLeft::ComputedMember(b.computed_member(
            b.rid_expr(ctx.state.name),
            b.static_member_expr(b.rid_expr("$"), "FILENAME"),
        ));
        let right = b.str_expr(ctx.state.filename);
        program_body.push(b.assign_stmt(left, right));
    }
    program_body.extend(module_imports);
    program_body.push(import_svelte);
    program_body.extend(script_imports);

    program_body.extend(hoistable_snippets);
    program_body.extend(module_body);
    program_body.extend(all_hoisted);

    if let Some(code) = ctx.state.css_text {
        let hash: &str = b.alloc_str(ctx.query.view.css_hash());
        let code: &str = b.alloc_str(code);
        let css_obj = b.object_expr([
            ObjProp::KeyValue("hash", b.str_expr(hash)),
            ObjProp::KeyValue("code", b.str_expr(code)),
        ]);
        program_body.push(b.const_stmt("$$css", css_obj));
    }
    program_body.push(export_default);
    program_body.extend(delegate_stmts);

    if let Some(ce_config) = component
        .options
        .as_ref()
        .and_then(|o| o.custom_element.as_ref())
    {
        let ce_stmts = custom_element::gen_custom_element(&mut ctx, ce_config);
        program_body.extend(ce_stmts);
    }

    let program = ctx.b.program(
        program_body,
        script_comments,
        script_source_text,
        script_span_end,
    );

    Codegen::default().build(&program).code
}

fn shift_statement_spans(stmts: &mut [Statement<'_>], offset: u32) {
    for stmt in stmts {
        let span = stmt.span_mut();
        span.start += offset;
        span.end += offset;
    }
}

fn shift_comments(comments: &mut [oxc_ast::Comment], offset: u32) {
    for comment in comments {
        comment.span.start += offset;
        comment.span.end += offset;
        comment.attached_to += offset;
    }
}

fn split_async_instance_body<'a>(
    b: &Builder<'a>,
    body: Vec<Statement<'a>>,
    blocker_data: &svelte_analyze::BlockerData,
) -> Vec<Statement<'a>> {
    let first_await_idx = match blocker_data.first_await_index() {
        Some(idx) => idx,
        None => return body,
    };

    let mut result = Vec::new();
    let mut hoisted_names: Vec<&str> = Vec::new();
    let mut thunks: Vec<oxc_ast::ast::Expression<'a>> = Vec::new();

    for (i, stmt) in body.into_iter().enumerate() {
        if i < first_await_idx {
            result.push(stmt);
            continue;
        }

        let meta = blocker_data.stmt_meta(i).expect("stmt_meta out of range");
        let has_await = meta.has_await();

        for name in meta.hoist_names() {
            hoisted_names.push(b.alloc_str(name));
        }

        let stmt = match stmt {
            Statement::ExportNamedDeclaration(export) => {
                if let Some(decl) = export.unbox().declaration {
                    Statement::from(decl)
                } else {
                    continue;
                }
            }
            other => other,
        };

        match stmt {
            Statement::VariableDeclaration(var_decl) => {
                let var_decl = var_decl.unbox();
                for declarator in var_decl.declarations {
                    if matches!(
                        &declarator.init,
                        Some(
                            oxc_ast::ast::Expression::ArrowFunctionExpression(_)
                                | oxc_ast::ast::Expression::FunctionExpression(_)
                        )
                    ) {
                        result.push(b.var_init_stmt(declarator));
                        continue;
                    }

                    if let Some(assign_target) = try_binding_to_assignment(&declarator.id, b) {
                        let init = declarator.init.unwrap_or_else(|| b.void_zero_expr());
                        let assign = b.ast.expression_assignment(
                            oxc_span::SPAN,
                            oxc_ast::ast::AssignmentOperator::Assign,
                            assign_target,
                            init,
                        );
                        if has_await {
                            thunks.push(b.async_arrow_expr_body(assign));
                        } else {
                            thunks.push(b.thunk(assign));
                        }
                    } else {
                        let var_stmt = b.var_init_stmt(declarator);
                        if has_await {
                            thunks.push(b.async_thunk_block(vec![var_stmt]));
                        } else {
                            thunks.push(b.thunk_block(vec![var_stmt]));
                        }
                    }
                }
            }
            Statement::FunctionDeclaration(_) => {
                result.push(stmt);
            }
            _ => {
                if has_await {
                    if let Statement::BlockStatement(block) = stmt {
                        let block = block.unbox();
                        thunks.push(b.async_thunk_block(block.body.into_iter().collect()));
                    } else {
                        thunks.push(b.async_thunk_block(vec![stmt]));
                    }
                } else {
                    thunks.push(b.thunk_block(vec![stmt]));
                }
            }
        }
    }

    if !hoisted_names.is_empty() {
        result.push(b.var_multi_stmt(&hoisted_names));
    }

    if !thunks.is_empty() {
        let thunk_array = b.array_expr(thunks);
        let run_call = b.call_expr("$.run", [Arg::Expr(thunk_array)]);
        result.push(b.var_stmt("$$promises", run_call));
    }

    result
}

fn try_binding_to_assignment<'a>(
    pat: &oxc_ast::ast::BindingPattern<'a>,
    b: &Builder<'a>,
) -> Option<oxc_ast::ast::AssignmentTarget<'a>> {
    use oxc_ast::ast::{AssignmentTarget, BindingPattern};
    match pat {
        BindingPattern::BindingIdentifier(id) => {
            let ident = b.ast.identifier_reference(oxc_span::SPAN, id.name.as_str());
            Some(AssignmentTarget::AssignmentTargetIdentifier(b.alloc(ident)))
        }
        _ => None,
    }
}

pub fn generate_module<'a>(
    alloc: &'a Allocator,
    program: oxc_ast::ast::Program<'a>,
    analysis: &AnalysisData<'a>,
    dev: bool,
) -> String {
    let _ = dev;
    let script_output =
        script::transform_module_program(alloc, program, Some(analysis), &analysis.scoping);

    let b = Builder::new(alloc);
    let import_svelte = b.import_all("$", "svelte/internal/client");

    let mut program_body: Vec<Statement<'_>> = Vec::new();
    program_body.push(import_svelte);
    program_body.extend(script_output.imports);
    program_body.extend(script_output.body);

    let program = b.program(
        program_body,
        script_output.comments,
        script_output.source_text,
        script_output.program_span_end,
    );
    Codegen::default().build(&program).code
}
