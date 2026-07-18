use svelte_emit_builders::store::build_store_base_read;
pub(crate) mod codegen;
mod context;
mod custom_element;
mod script;

use oxc_allocator::Allocator;
use oxc_ast::ast::{
    AssignmentOperator, AssignmentTarget, BindingPattern, ExportDefaultDeclarationKind, Expression,
    Program, Statement,
};
use oxc_codegen::{Codegen, CodegenOptions as OxcCodegenOptions};
use oxc_span::Span;
use std::iter::empty;
use std::path::PathBuf;

use svelte_analyze::reactivity_semantics::legacy_reactive::legacy_reactive_import_wrapper_name;
use svelte_analyze::{
    AnalysisData, BindingSemantics, ReferenceSemantics, SignalReferenceKind,
    StateDeclarationSemantics, StateKind,
};
use svelte_ast_builder::{Arg, AssignLeft, Builder, ObjProp};
use svelte_sourcemap::{JsOutput, SourcemapKind};
use svelte_transform_client::{RestExcludeKey, TransformData};

use context::Ctx;
use svelte_analyze::types::data::binding_group_name;

fn export_reactive_read<'a>(
    b: &Builder<'a>,
    sem: &ReferenceSemantics,
    name: &'a str,
) -> Option<Expression<'a>> {
    match sem {
        ReferenceSemantics::SignalRead { safe: true, .. }
        | ReferenceSemantics::LegacyStateRead { safe: true }
        | ReferenceSemantics::LegacyStateSubscribedRead { safe: true, .. } => {
            Some(b.call_expr("$.safe_get", [Arg::Ident(name)]))
        }
        ReferenceSemantics::SignalRead { safe: false, .. }
        | ReferenceSemantics::LegacyStateRead { safe: false }
        | ReferenceSemantics::LegacyStateSubscribedRead { safe: false, .. } => {
            Some(b.call_expr("$.get", [Arg::Ident(name)]))
        }
        ReferenceSemantics::PropRead(_) | ReferenceSemantics::StoreRead { .. } => {
            Some(b.call_expr(name, empty::<Arg<'_, '_>>()))
        }
        ReferenceSemantics::LegacyReactiveImportRead => {
            let wrapper: &str = b.alloc_str(&legacy_reactive_import_wrapper_name(name));
            Some(b.call_expr(wrapper, empty::<Arg<'_, '_>>()))
        }
        ReferenceSemantics::NonReactive
        | ReferenceSemantics::Proxy
        | ReferenceSemantics::Unresolved
        | ReferenceSemantics::SignalWrite { .. }
        | ReferenceSemantics::SignalUpdate { .. }
        | ReferenceSemantics::DerivedWrite
        | ReferenceSemantics::DerivedUpdate
        | ReferenceSemantics::StoreWrite { .. }
        | ReferenceSemantics::StoreUpdate { .. }
        | ReferenceSemantics::PropMutation { .. }
        | ReferenceSemantics::PropSourceMemberMutationRoot { .. }
        | ReferenceSemantics::PropNonSourceMemberMutationRoot { .. }
        | ReferenceSemantics::ConstAliasRead { .. }
        | ReferenceSemantics::ContextualRead(_)
        | ReferenceSemantics::CarrierMemberRead(_)
        | ReferenceSemantics::RestPropMemberRewrite
        | ReferenceSemantics::LegacyPropsIdentifierRead
        | ReferenceSemantics::LegacyRestPropsIdentifierRead
        | ReferenceSemantics::LegacySlotsIdentifierRead
        | ReferenceSemantics::LegacyStateWrite
        | ReferenceSemantics::LegacyStateUpdate { .. }
        | ReferenceSemantics::LegacyStateSubscribedWrite { .. }
        | ReferenceSemantics::LegacyStateSubscribedUpdate { .. }
        | ReferenceSemantics::LegacyStateMemberMutationRoot { .. }
        | ReferenceSemantics::LegacyReactiveImportMemberMutationRoot { .. }
        | ReferenceSemantics::ImportSubscribedRead { .. }
        | ReferenceSemantics::LegacyEachItemMemberMutationRoot { .. }
        | ReferenceSemantics::EachItemMemberMutationStoreInvalidate { .. }
        | ReferenceSemantics::EachItemIndexedLegacy { .. }
        | ReferenceSemantics::IllegalWrite => None,
    }
}

fn declaration_export_semantics(binding: BindingSemantics) -> ReferenceSemantics {
    match binding {
        BindingSemantics::State(StateDeclarationSemantics {
            kind: kind @ (StateKind::State | StateKind::StateRaw),
            ..
        }) => ReferenceSemantics::SignalRead {
            kind: SignalReferenceKind::State(kind),
            safe: false,
        },
        BindingSemantics::Derived(d) | BindingSemantics::OptimizedDerived(d) => {
            ReferenceSemantics::SignalRead {
                kind: SignalReferenceKind::Derived(d.kind),
                safe: false,
            }
        }
        BindingSemantics::LegacyState(state) => ReferenceSemantics::LegacyStateRead {
            safe: state.var_declared,
        },
        _ => ReferenceSemantics::NonReactive,
    }
}

pub fn generate<'a>(
    compile_ctx: svelte_types::CompileContext<'a, 'a>,
    options: &svelte_types::CodegenOptions,
    transform_data: TransformData,
    css_text: Option<&str>,
) -> JsOutput {
    let alloc = compile_ctx.alloc;
    let component = compile_ctx.component;
    let analysis = compile_ctx.analysis;
    let dev = options.dev;
    let mut ctx = Ctx::new(compile_ctx, options, transform_data, css_text);

    let script_output = script::gen_script(&mut ctx, dev);
    let script_imports = script_output.imports;
    let script_body = script_output.body;
    let has_tracing = script_output.has_tracing;
    let needs_ownership_validator = script_output.needs_ownership_validator
        || analysis.output.needs_component_bind_ownership
        || ctx.transform_data.needs_ownership_validator;
    let mut script_comments = script_output.comments;
    let script_rest_excludes = script_output.rest_excludes;

    let mut module_imports: Vec<Statement<'_>> = Vec::new();
    let mut module_body: Vec<Statement<'_>> = Vec::new();
    if component.module_script.is_some()
        && let Some(program) = ctx.state.parsed.module_program.take()
    {
        let module_output = script::transform_component_module_program(
            alloc,
            program,
            Some(analysis),
            &analysis.scoping,
            &mut *ctx.state.ident_gen,
            ctx.state.line_index,
            ctx.state.dev,
        );

        script_comments.extend(module_output.comments);
        module_imports = module_output.imports;
        module_body = module_output.body;
    }

    let codegen_result = codegen::codegen_root_fragment(&mut ctx).expect("codegen failed");
    let hoisted = codegen_result.hoisted;
    let template_body = codegen_result.body;
    let instance_snippets = codegen_result.instance_snippets;
    let hoistable_snippets = codegen_result.hoistable_snippets;

    for re in script_rest_excludes {
        let set_stmt = {
            let keys: Vec<Arg<'_, '_>> = re
                .keys
                .iter()
                .map(|k| match k {
                    RestExcludeKey::Str(s) => Arg::StrRef(ctx.b.alloc_str(s)),
                    RestExcludeKey::Num(n) => Arg::Num(*n),
                })
                .collect();
            let arr = ctx.b.array_from_args(keys);
            let new_set = ctx.b.new_expr("Set", [Arg::Expr(arr)]);
            ctx.b.var_stmt(&re.name, new_set)
        };
        ctx.state.module_hoisted.push(set_stmt);
    }

    let mut all_hoisted: Vec<Statement<'_>> = Vec::new();
    all_hoisted.append(&mut ctx.state.module_hoisted);
    all_hoisted.extend(hoisted);

    let runtime = ctx.runtime_plan();

    let mut fn_body: Vec<Statement<'_>> = Vec::new();

    if let Some(props_id_name) = ctx.query.props_id() {
        let name: &str = ctx.b.alloc_str(props_id_name);
        let call = ctx.b.call_expr("$.props_id", empty::<Arg<'_, '_>>());
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

    if ctx.query.needs_sanitized_legacy_slots() {
        fn_body.push(ctx.b.const_stmt(
            "$$slots",
            ctx.b.call_expr("$.sanitize_slots", [Arg::Ident("$$props")]),
        ));
    }

    if ctx.query.needs_sanitized_legacy_props() {
        let arr = ctx.b.array_from_args(
            ctx.query
                .legacy_sanitized_props_excluded_keys()
                .iter()
                .map(|s| Arg::Str((*s).to_string()))
                .collect::<Vec<_>>(),
        );
        fn_body.push(ctx.b.const_stmt(
            "$$sanitized_props",
            ctx.b.call_expr(
                "$.legacy_rest_props",
                [Arg::Ident("$$props"), Arg::Expr(arr)],
            ),
        ));
    }
    if ctx.query.needs_legacy_rest_props() {
        let keys = ctx.query.legacy_bindable_prop_keys();
        let arr = ctx
            .b
            .array_from_args(keys.into_iter().map(Arg::Str).collect::<Vec<_>>());
        fn_body.push(ctx.b.const_stmt(
            "$$restProps",
            ctx.b.call_expr(
                "$.legacy_rest_props",
                [Arg::Ident("$$sanitized_props"), Arg::Expr(arr)],
            ),
        ));
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

    if ctx.query.view.inject_styles() && ctx.state.css_text.is_some() {
        fn_body.push(ctx.b.expr_stmt(ctx.b.call_expr(
            "$.append_styles",
            [Arg::Ident("$$anchor"), Arg::Ident("$$css")],
        )));
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
        let stores: Vec<(&str, &str, svelte_component_semantics::SymbolId)> = ctx
            .query
            .view
            .iter_store_bindings()
            .map(|(_, store)| {
                (
                    scoping.symbol_name(store.base_symbol),
                    scoping.symbol_name(store.store_symbol),
                    store.base_symbol,
                )
            })
            .collect();

        for (base_name, dollar_name, base_symbol) in &stores {
            let dollar_name_str: &str = ctx.b.alloc_str(dollar_name);
            let make_base_arg = || -> Arg<'_, '_> {
                Arg::Expr(build_store_base_read(
                    &ctx.b,
                    ctx.query.analysis,
                    *base_symbol,
                ))
            };
            let store_get = ctx.b.call_expr(
                "$.store_get",
                [
                    make_base_arg(),
                    Arg::StrRef(dollar_name_str),
                    Arg::Ident("$$stores"),
                ],
            );

            let thunk_body = if ctx.state.dev {
                let validate = ctx.b.call_expr(
                    "$.validate_store",
                    [make_base_arg(), Arg::StrRef(ctx.b.alloc_str(base_name))],
                );
                ctx.b.seq_expr([validate, store_get])
            } else {
                store_get
            };
            let thunk = ctx.b.thunk(thunk_body);
            fn_body.push(ctx.b.const_stmt(dollar_name_str, thunk));
        }

        let setup_call = ctx.b.call_expr("$.setup_stores", empty::<Arg<'_, '_>>());
        fn_body.push(
            ctx.b
                .const_array_destruct_stmt(&["$$stores", "$$cleanup"], setup_call),
        );
    }

    {
        let legacy_reactive_emitted_binding_groups = ctx
            .query
            .analysis
            .reactivity
            .legacy_reactive()
            .iter_statements_topo()
            .next()
            .is_some();
        if !legacy_reactive_emitted_binding_groups {
            let count = ctx.query.view.binding_group_count();
            for id in 0..count {
                let name = binding_group_name(id);
                let name_ref: &str = ctx.b.alloc_str(&name);
                fn_body.push(ctx.b.const_stmt(name_ref, ctx.b.empty_array_expr()));
            }
        }
    }

    fn_body.extend(instance_snippets);

    if ctx.state.experimental_async && ctx.query.blocker_data().has_async() {
        let split_body = split_async_instance_body(&ctx.b, script_body, ctx.query.blocker_data());
        fn_body.extend(split_body);
    } else {
        fn_body.extend(script_body);
    }

    let has_explicit_exports =
        runtime.has_exports || runtime.has_ce_props || runtime.has_legacy_accessor_props;
    let dev_legacy_only = ctx.state.dev && runtime.needs_push;
    let mut bind_prop_stmts: Vec<Statement<'_>> = Vec::new();
    if has_explicit_exports || dev_legacy_only {
        let mut export_props: Vec<ObjProp<'_>> = Vec::new();

        if has_explicit_exports {
            for e in ctx.query.exports() {
                let local_sym = e.local;
                let name: &str = ctx.b.alloc_str(ctx.query.symbol_name(local_sym));
                let key: &str = e
                    .alias
                    .as_deref()
                    .map(|a| ctx.b.alloc_str(a))
                    .unwrap_or(name);

                let read_semantics = e
                    .reference_id
                    .map(|ref_id| ctx.query.reference_semantics(ref_id))
                    .unwrap_or_else(|| {
                        declaration_export_semantics(ctx.query.binding_semantics(local_sym))
                    });
                let reactive_read = export_reactive_read(&ctx.b, &read_semantics, name);

                let setter_body: Option<Vec<Statement<'_>>> =
                    match ctx.query.binding_semantics(local_sym) {
                        BindingSemantics::Prop(_) => Some(vec![
                            ctx.b
                                .expr_stmt(ctx.b.call_expr(name, [Arg::Ident("$$value")])),
                        ]),
                        BindingSemantics::State(StateDeclarationSemantics {
                            kind: kind @ (StateKind::State | StateKind::StateRaw),
                            ..
                        }) => {
                            let value = if matches!(kind, StateKind::State) {
                                Arg::Expr(ctx.b.call_expr("$.proxy", [Arg::Ident("$$value")]))
                            } else {
                                Arg::Ident("$$value")
                            };
                            Some(vec![ctx.b.expr_stmt(
                                ctx.b.call_expr("$.set", [Arg::Ident(name), value]),
                            )])
                        }
                        BindingSemantics::LegacyState(_) => {
                            Some(vec![ctx.b.expr_stmt(ctx.b.call_expr(
                                "$.set",
                                [
                                    Arg::Ident(name),
                                    Arg::Expr(ctx.b.call_expr("$.proxy", [Arg::Ident("$$value")])),
                                ],
                            ))])
                        }
                        BindingSemantics::NonReactive | BindingSemantics::LegacyApiExport => ctx
                            .query
                            .scoping()
                            .is_reassignable_declaration(local_sym)
                            .then(|| {
                                vec![ctx.b.assign_stmt(
                                    AssignLeft::Ident(name.to_string()),
                                    ctx.b.rid_expr("$$value"),
                                )]
                            }),
                        _ => None,
                    };

                if let Some(stmts) = setter_body {
                    let getter_body = reactive_read.unwrap_or_else(|| ctx.b.rid_expr(name));
                    export_props.push(ObjProp::Getter(key, getter_body));
                    export_props.push(ObjProp::Setter(key, "$$value", None, stmts));
                } else if let Some(read) = reactive_read {
                    export_props.push(ObjProp::Getter(key, read));
                } else if ctx.state.dev {
                    export_props.push(ObjProp::Getter(key, ctx.b.rid_expr(name)));
                } else if e.alias.is_some() {
                    export_props.push(ObjProp::KeyValue(key, ctx.b.rid_expr(name)));
                } else {
                    export_props.push(ObjProp::Shorthand(name));
                }
            }

            if ctx.query.accessors() || runtime.has_ce_props {
                for prop in ctx.query.component_prop_accessors() {
                    let key: &str = ctx.b.alloc_str(&prop.key);
                    let local: &str = ctx.b.alloc_str(prop.local);

                    let getter_expr = ctx.b.call_expr(local, empty::<Arg<'_, '_>>());
                    export_props.push(ObjProp::Getter(key, getter_expr));

                    let default_expr = if ctx.query.runes() {
                        prop.default_span.map(|span| {
                            let text = &ctx.state.source[span.start as usize..span.end as usize];
                            ctx.b.parse_expression(text)
                        })
                    } else {
                        None
                    };
                    let setter_body = vec![
                        ctx.b
                            .expr_stmt(ctx.b.call_expr(local, [Arg::Ident("$$value")])),
                        ctx.b.call_stmt("$.flush", empty::<Arg<'_, '_>>()),
                    ];
                    export_props.push(ObjProp::Setter(key, "$$value", default_expr, setter_body));
                }
            }
        }

        if ctx.state.dev {
            let legacy_call = ctx.b.call_expr("$.legacy_api", empty::<Arg<'_, '_>>());
            export_props.insert(0, ObjProp::Spread(legacy_call));
        }

        fn_body.push(ctx.b.var_stmt("$$exports", ctx.b.object_expr(export_props)));

        if !ctx.query.runes() {
            for e in ctx.query.exports() {
                let name: &str = ctx.b.alloc_str(ctx.query.symbol_name(e.local));
                let key: &str = e
                    .alias
                    .as_deref()
                    .map(|a| ctx.b.alloc_str(a))
                    .unwrap_or(name);
                let value = match ctx.query.binding_semantics(e.local).legacy_state() {
                    Some(state) => {
                        let getter = if state.var_declared {
                            "$.safe_get"
                        } else {
                            "$.get"
                        };
                        Arg::Expr(ctx.b.call_expr(getter, [Arg::Ident(name)]))
                    }
                    None => Arg::Ident(name),
                };
                bind_prop_stmts.push(ctx.b.call_stmt(
                    "$.bind_prop",
                    [Arg::Ident("$$props"), Arg::StrRef(key), value],
                ));
            }
        }
    }

    match runtime.legacy_init {
        svelte_analyze::LegacyInit::None => {}
        svelte_analyze::LegacyInit::Plain => {
            fn_body.push(ctx.b.call_stmt("$.init", empty::<Arg<'_, '_>>()))
        }
        svelte_analyze::LegacyInit::Immutable => {
            fn_body.push(ctx.b.call_stmt("$.init", [Arg::Bool(true)]))
        }
    }

    fn_body.extend(template_body);

    fn_body.extend(bind_prop_stmts);

    if runtime.needs_push {
        if runtime.needs_pop_with_return && runtime.has_stores {
            let pop_call = ctx.b.call_expr("$.pop", [Arg::Ident("$$exports")]);
            fn_body.push(ctx.b.var_stmt("$$pop", pop_call));
            fn_body.push(ctx.b.call_stmt("$$cleanup", empty::<Arg<'_, '_>>()));
            fn_body.push(ctx.b.return_stmt(ctx.b.rid_expr("$$pop")));
        } else if runtime.needs_pop_with_return {
            fn_body.push(
                ctx.b
                    .return_stmt(ctx.b.call_expr("$.pop", [Arg::Ident("$$exports")])),
            );
        } else {
            fn_body.push(
                ctx.b
                    .expr_stmt(ctx.b.call_expr("$.pop", empty::<Arg<'_, '_>>())),
            );

            if runtime.has_stores {
                fn_body.push(ctx.b.call_stmt("$$cleanup", empty::<Arg<'_, '_>>()));
            }
        }
    } else if runtime.has_stores {
        fn_body.push(ctx.b.call_stmt("$$cleanup", empty::<Arg<'_, '_>>()));
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

    let fn_params = if runtime.needs_props_param {
        b.params(["$$anchor", "$$props"])
    } else {
        b.params(["$$anchor"])
    };

    let body_span = component
        .instance_script
        .as_ref()
        .or(component.module_script.as_ref())
        .map(|s| Span::new(s.content_span.start, s.content_span.end))
        .unwrap_or_default();
    let fn_decl = b.function_decl(b.bid(ctx.state.name), fn_body, fn_params, body_span);
    let component_tail: Vec<Statement<'_>> = if ctx.state.hmr {
        let name = ctx.state.name;
        let css_hash: &str = b.alloc_str(ctx.query.view.css_hash());

        let mut accept_body: Vec<Statement<'_>> = Vec::new();
        if !css_hash.is_empty() {
            accept_body.push(b.expr_stmt(b.call_expr("$.cleanup_styles", [Arg::StrRef(css_hash)])));
        }
        let update_callee = b.static_member_expr(
            b.computed_member_expr(b.rid_expr(name), b.rid_expr("$.HMR")),
            "update",
        );
        let module_default = b.static_member_expr(b.rid_expr("module"), "default");
        accept_body
            .push(b.expr_stmt(b.call_expr_callee(update_callee, [Arg::Expr(module_default)])));

        let accept_callee =
            b.static_member_expr(b.static_member_expr(b.import_meta_expr(), "hot"), "accept");
        let accept_arrow = b.arrow_block(b.params(["module"]), accept_body);
        let accept_stmt =
            b.expr_stmt(b.call_expr_callee(accept_callee, [Arg::Arrow(accept_arrow)]));

        let assign = b.assign_stmt(
            AssignLeft::Ident(name.to_string()),
            b.call_expr("$.hmr", [Arg::Ident(name)]),
        );
        let hmr_block = b.block_stmt(vec![assign, accept_stmt]);
        let hmr_guard = b.if_stmt(
            b.static_member_expr(b.import_meta_expr(), "hot"),
            hmr_block,
            None,
        );

        vec![
            b.function_decl_stmt(fn_decl),
            hmr_guard,
            b.export_default(ExportDefaultDeclarationKind::from(b.rid_expr(name))),
        ]
    } else {
        vec![
            b.export_default(ExportDefaultDeclarationKind::FunctionDeclaration(
                b.alloc(fn_decl),
            )),
        ]
    };

    let mut program_body: Vec<Statement<'_>> = Vec::new();
    if ctx.state.disclose_version {
        program_body.push(b.bare_import("svelte/internal/disclose-version"));
    }
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

    for sym in ctx
        .query
        .analysis
        .reactivity
        .legacy_reactive()
        .iter_mutated_imports()
    {
        let name: &str = b.alloc_str(ctx.query.analysis.scoping.symbol_name(sym));
        let var_name: &str = b.alloc_str(&legacy_reactive_import_wrapper_name(name));
        let thunk = b.thunk(b.rid_expr(name));
        let init = b.call_expr("$.reactive_import", [Arg::Expr(thunk)]);
        program_body.push(b.var_stmt(var_name, init));
    }

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
    program_body.extend(component_tail);
    program_body.extend(delegate_stmts);

    if ctx.query.view.is_custom_element_target() {
        let ce_config = component
            .options
            .as_ref()
            .and_then(|o| o.custom_element.as_ref());
        let ce_stmts = custom_element::gen_custom_element(&mut ctx, ce_config);
        program_body.extend(ce_stmts);
    }

    let full_source: &'a str = component.source.as_str();
    let span_end = full_source.len() as u32;
    let program = ctx
        .b
        .program(program_body, script_comments, full_source, span_end);

    build_codegen_output(
        &program,
        options.sourcemap_kind,
        &options.filename,
        full_source,
    )
}

fn build_codegen_output(
    program: &Program<'_>,
    kind: SourcemapKind,
    filename: &str,
    source_text: &str,
) -> JsOutput {
    if !kind.is_enabled() {
        return JsOutput {
            code: Codegen::default().build(program).code,
            map: None,
        };
    }
    let cg_options = OxcCodegenOptions {
        source_map_path: Some(PathBuf::from(filename)),
        ..OxcCodegenOptions::default()
    };
    let ret = Codegen::default()
        .with_options(cg_options)
        .with_source_text(source_text)
        .build(program);
    JsOutput {
        code: ret.code,
        map: ret.map,
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
    let mut thunks: Vec<Expression<'a>> = Vec::new();

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
                        declarator.init.as_ref().map(|e| e.get_inner_expression()),
                        Some(
                            Expression::ArrowFunctionExpression(_)
                                | Expression::FunctionExpression(_)
                        )
                    ) {
                        result.push(b.var_init_stmt(declarator));
                        continue;
                    }

                    if let Some(assign_target) = try_binding_to_assignment(&declarator.id, b) {
                        let init = declarator.init.unwrap_or_else(|| b.void_zero_expr());
                        let assign = b.ast.expression_assignment(
                            oxc_span::SPAN,
                            AssignmentOperator::Assign,
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
    pat: &BindingPattern<'a>,
    b: &Builder<'a>,
) -> Option<AssignmentTarget<'a>> {
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
    program: Program<'a>,
    analysis: &AnalysisData<'a>,
    line_index: &svelte_span::LineIndex,
    dev: bool,
    kind: SourcemapKind,
    filename: &str,
    source_text: &str,
) -> JsOutput {
    let mut ident_gen =
        svelte_analyze::IdentGen::with_conflicts(analysis.scoping.collect_all_symbol_names());
    let script_output = script::transform_module_program(
        alloc,
        program,
        Some(analysis),
        &analysis.scoping,
        &mut ident_gen,
        line_index,
        dev,
    );

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
    build_codegen_output(&program, kind, filename, source_text)
}
