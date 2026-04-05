mod builder;
mod context;
mod custom_element;
mod script;
mod template;

use oxc_allocator::Allocator;
use oxc_ast::ast::{ExportDefaultDeclarationKind, Statement};
use oxc_codegen::Codegen;
use oxc_span::{GetSpanMut, Span};

use svelte_analyze::{AnalysisData, IdentGen, ParserResult};
use svelte_ast::{Attribute, Component, Node};
use svelte_transform::TransformData;

use builder::{Arg, AssignLeft, Builder, ObjProp};
use context::Ctx;

/// Generate JavaScript client-side code for a compiled Svelte component.
pub fn generate<'a>(
    alloc: &'a Allocator,
    component: &'a Component,
    analysis: &'a AnalysisData,
    parsed: &'a mut ParserResult<'a>,
    ident_gen: &'a mut IdentGen,
    transform_data: TransformData,
    css_text: Option<&str>,
    name: &str,
    dev: bool,
    source: &'a str,
    filename: &str,
    experimental_async: bool,
) -> String {
    let mut ctx = Ctx::new(
        alloc,
        component,
        analysis,
        parsed,
        ident_gen,
        transform_data,
        css_text,
        name,
        dev,
        source,
        filename,
        experimental_async,
    );

    // -----------------------------------------------------------------------
    // 1. Script transformation
    // -----------------------------------------------------------------------
    let script_output = script::gen_script(&mut ctx, dev);
    let script_imports = script_output.imports;
    let script_body = script_output.body;
    let has_tracing = script_output.has_tracing;
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
                &analysis.scoping,
                Some(analysis.script_rune_call_kinds()),
            )
        } else {
            let is_ts = module_script.language == svelte_ast::ScriptLanguage::TypeScript;
            script::transform_component_module_script(alloc, module_source, is_ts)
        };

        // Module script comments need to be preserved in the final top-level program even when
        // there is no instance script to carry comment/source metadata.
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

    // -----------------------------------------------------------------------
    // 2. Template generation (consumes "root" ident first)
    //    instance_snippets are generated inside gen_root_fragment for correct numbering
    // -----------------------------------------------------------------------
    let (hoisted, template_body, instance_snippets, hoistable_snippets) =
        template::gen_root_fragment(&mut ctx);

    // Layout: hoistable snippets → inner hoisted → root hoisted
    let mut all_hoisted: Vec<Statement<'_>> = Vec::new();
    all_hoisted.extend(hoistable_snippets);
    all_hoisted.extend(ctx.state.module_hoisted.drain(..));
    all_hoisted.extend(hoisted);

    // -----------------------------------------------------------------------
    // 3. Build function body (needs &mut ctx for snippets)
    // -----------------------------------------------------------------------
    let runtime = ctx.runtime_plan();

    let mut fn_body: Vec<Statement<'_>> = Vec::new();

    // CSS injection — $.append_styles() must be the very first statement in the function body
    // so the styles are available before any DOM nodes are created.
    if ctx.query.view.inject_styles() && ctx.state.css_text.is_some() {
        fn_body.push(
            ctx.b.expr_stmt(ctx.b.call_expr(
                "$.append_styles",
                [Arg::Ident("$$anchor"), Arg::Ident("$$css")],
            )),
        );
    }

    // $props.id() → must be first statement for hydration correctness
    if let Some(props_id_name) = ctx.query.props_id() {
        let name: &str = ctx.b.alloc_str(props_id_name);
        let call = ctx
            .b
            .call_expr("$.props_id", std::iter::empty::<Arg<'_, '_>>());
        fn_body.push(ctx.b.const_stmt(name, call));
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
        let mut push_args: Vec<Arg<'_, '_>> =
            vec![Arg::Ident("$$props"), Arg::Expr(ctx.b.bool_expr(true))];
        if ctx.state.dev {
            push_args.push(Arg::Ident(ctx.state.name));
        }
        fn_body.push(ctx.b.expr_stmt(ctx.b.call_expr("$.push", push_args)));
    }
    if ctx.state.needs_binding_group {
        fn_body.push(ctx.b.const_stmt("binding_group", ctx.b.empty_array_expr()));
    }

    // Store subscription setup:
    //   const $count = () => $.store_get(count, "$count", $$stores);
    //   const [$$stores, $$cleanup] = $.setup_stores();
    if runtime.has_stores {
        // Sort store base names for deterministic output
        let mut store_names: Vec<&str> = ctx
            .query
            .scoping()
            .store_symbol_ids()
            .map(|sym| ctx.query.scoping().symbol_name(sym))
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
            // Dev mode: ($.validate_store(name, "name"), $.store_get(...))
            // Prod mode: $.store_get(...)
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

        // const [$$stores, $$cleanup] = $.setup_stores()
        let setup_call = ctx
            .b
            .call_expr("$.setup_stores", std::iter::empty::<Arg<'_, '_>>());
        fn_body.push(
            ctx.b
                .const_array_destruct_stmt(&["$$stores", "$$cleanup"], setup_call),
        );
    }

    // Instance-level snippet declarations (generated during root template for correct numbering)
    fn_body.extend(instance_snippets);

    // Instance body splitting for experimental.async:
    // Statements after first `await` become async thunks in $.run([...])
    if ctx.state.experimental_async && ctx.query.blocker_data().has_async() {
        let split_body = split_async_instance_body(&ctx.b, script_body, ctx.query.blocker_data());
        fn_body.extend(split_body);
    } else {
        fn_body.extend(script_body);
    }

    // var $$exports = { ... }
    if runtime.has_exports || runtime.has_ce_props {
        let mut export_props: Vec<ObjProp<'_>> = Vec::new();

        // Regular exports (e.g., `export function reset()`)
        for e in ctx.query.exports() {
            let name: &str = ctx.b.alloc_str(&e.name);
            if let Some(alias) = &e.alias {
                let alias: &str = ctx.b.alloc_str(alias);
                export_props.push(ObjProp::KeyValue(alias, ctx.b.rid_expr(name)));
            } else {
                export_props.push(ObjProp::Shorthand(name));
            }
        }

        // Custom element prop getter/setters
        if runtime.has_ce_props {
            if let Some(props_analysis) = ctx.query.props() {
                for prop in &props_analysis.props {
                    if prop.is_rest || prop.is_reserved {
                        continue;
                    }
                    let key: &str = ctx.b.alloc_str(&prop.prop_name);
                    let local: &str = ctx.b.alloc_str(&prop.local_name);

                    // get name() { return name(); }
                    let getter_expr = ctx.b.call_expr(local, std::iter::empty::<Arg<'_, '_>>());
                    export_props.push(ObjProp::Getter(key, getter_expr));

                    // set name($$value = default?) { name($$value); $.flush(); }
                    let default_expr = prop
                        .default_text
                        .as_deref()
                        .map(|text| ctx.b.parse_expression(text));
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
        // var $$exports = { ...$.legacy_api() }
        let legacy_call = ctx
            .b
            .call_expr("$.legacy_api", std::iter::empty::<Arg<'_, '_>>());
        fn_body.push(ctx.b.var_stmt(
            "$$exports",
            ctx.b.object_expr([ObjProp::Spread(legacy_call)]),
        ));
    }

    fn_body.extend(template_body);

    if runtime.needs_push {
        if runtime.needs_pop_with_return && runtime.has_stores {
            // var $$pop = $.pop($$exports); $$cleanup(); return $$pop;
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
            // Store cleanup: $$cleanup() — runs after $.pop()
            if runtime.has_stores {
                fn_body.push(
                    ctx.b
                        .call_stmt("$$cleanup", std::iter::empty::<Arg<'_, '_>>()),
                );
            }
        }
    } else if runtime.has_stores {
        // No push/pop but still have stores — just cleanup
        fn_body.push(
            ctx.b
                .call_stmt("$$cleanup", std::iter::empty::<Arg<'_, '_>>()),
        );
    }

    // -----------------------------------------------------------------------
    // 4. Module-level delegate calls
    // -----------------------------------------------------------------------
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

    // -----------------------------------------------------------------------
    // 5. Assemble the program
    // -----------------------------------------------------------------------
    let b = &ctx.b;

    let import_svelte = b.import_all("$", "svelte/internal/client");

    // Bubble events on special elements (on:event with no expression) reference $$props
    let has_bubble_events = component.fragment.nodes.iter().any(|&id| {
        let node = component.store.get(id);
        let attrs = match node {
            Node::SvelteWindow(w) => Some(&w.attributes),
            Node::SvelteDocument(d) => Some(&d.attributes),
            _ => None,
        };
        attrs.is_some_and(|attrs| {
            attrs.iter().any(
                |a| matches!(a, Attribute::OnDirectiveLegacy(od) if od.expression_span.is_none()),
            )
        })
    });

    let fn_params = if runtime.needs_props_param || has_bubble_events {
        b.params(["$$anchor", "$$props"])
    } else {
        b.params(["$$anchor"])
    };

    // Set body span so OXC Codegen can find trailing comments inside the function.
    // FunctionBody.gen() looks for comments at span_end - 1.
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
        // App[$.FILENAME] = "filename"
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
    program_body.extend(module_body);
    program_body.extend(all_hoisted);
    // const $$css = { hash: "svelte-HASH", code: "scoped CSS" } — placed after template
    // vars so it appears between `var root = ...` and the component function, matching
    // the reference compiler's output order.
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

    // Custom element wrapping: customElements.define(tag, $.create_custom_element(App, ...))
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

/// Split instance body into sync prefix + `var $$promises = $.run([thunks])`.
/// Statements before the first `await` are kept as-is.
/// Statements after (inclusive) become thunks in the `$.run()` call.
///
/// Uses pre-computed `BlockerData` metadata from analyze to determine
/// `has_await` and `hoist_names` per statement — no AST re-walking.
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

        // Collect pre-computed hoist names
        for name in meta.hoist_names() {
            hoisted_names.push(b.alloc_str(name));
        }

        // Unwrap ExportNamedDeclaration to process inner declaration
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
                    // Function-valued init: keep in sync section
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

                    // Simple identifiers → expression-body thunk: () => x = val
                    // Complex patterns → block-body thunk preserving var statement
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

    // Emit hoisted `var data, y;`
    if !hoisted_names.is_empty() {
        result.push(b.var_multi_stmt(&hoisted_names));
    }

    // Emit `var $$promises = $.run([thunks])`
    if !thunks.is_empty() {
        let thunk_array = b.array_expr(thunks);
        let run_call = b.call_expr("$.run", [Arg::Expr(thunk_array)]);
        result.push(b.var_stmt("$$promises", run_call));
    }

    result
}

/// Try to convert a BindingPattern to an AssignmentTarget for expression-body thunks.
/// Returns `Some(target)` for simple identifiers, `None` for complex patterns
/// (destructuring is emitted as block-body thunks preserving the original var statement).
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

/// Generate JavaScript for a standalone `.svelte.js`/`.svelte.ts` module.
/// Applies rune transforms but produces a plain ES module (no component wrapping).
pub fn generate_module(
    alloc: &Allocator,
    source: &str,
    is_ts: bool,
    analysis: &AnalysisData,
    dev: bool,
) -> String {
    let _ = dev; // reserved for future dev-mode codegen (e.g. $.tag, strict_equals)
    let arena_source: &str = alloc.alloc_str(source);
    let script_output =
        script::transform_module_script(alloc, arena_source, is_ts, &analysis.scoping);

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
