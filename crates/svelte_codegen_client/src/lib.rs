mod builder;
mod context;
mod custom_element;
mod script;
mod template;

use oxc_allocator::Allocator;
use oxc_ast::ast::{ExportDefaultDeclarationKind, Statement};
use oxc_codegen::Codegen;
use oxc_span::Span;

use svelte_analyze::{AnalysisData, IdentGen, ParsedExprs};
use svelte_ast::{Attribute, Component, Node};
use svelte_transform::TransformData;

use builder::{Arg, AssignLeft, Builder, ObjProp};
use context::Ctx;

/// Generate JavaScript client-side code for a compiled Svelte component.
pub fn generate<'a>(alloc: &'a Allocator, component: &'a Component, analysis: &'a AnalysisData, parsed: &'a mut ParsedExprs<'a>, ident_gen: &'a mut IdentGen, transform_data: TransformData, name: &str, dev: bool, source: &'a str, filename: &str) -> String {
    let mut ctx = Ctx::new(alloc, component, analysis, parsed, ident_gen, transform_data, name, dev, source, filename);

    // -----------------------------------------------------------------------
    // 1. Script transformation
    // -----------------------------------------------------------------------
    let script_output = script::gen_script(&mut ctx, dev);
    let script_imports = script_output.imports;
    let script_body = script_output.body;
    let has_tracing = script_output.has_tracing;
    let script_comments = script_output.comments;
    let script_source_text = script_output.source_text;
    let script_span_end = script_output.program_span_end;

    // -----------------------------------------------------------------------
    // 2. Template generation (consumes "root" ident first)
    //    instance_snippets are generated inside gen_root_fragment for correct numbering
    // -----------------------------------------------------------------------
    let (hoisted, template_body, instance_snippets, hoistable_snippets) = template::gen_root_fragment(&mut ctx);

    // Layout: hoistable snippets → inner hoisted → root hoisted
    let mut all_hoisted: Vec<Statement<'_>> = Vec::new();
    all_hoisted.extend(hoistable_snippets);
    all_hoisted.extend(ctx.module_hoisted.drain(..));
    all_hoisted.extend(hoisted);

    // -----------------------------------------------------------------------
    // 3. Build function body (needs &mut ctx for snippets)
    // -----------------------------------------------------------------------
    let is_custom_element = ctx.analysis.custom_element;
    let has_exports = !ctx.analysis.exports.is_empty();
    let has_bindable = ctx.analysis.props.as_ref().is_some_and(|p| p.has_bindable);
    let has_stores = !ctx.analysis.scoping.store_symbols().is_empty();
    let has_ce_props = is_custom_element && ctx.analysis.props.as_ref().is_some_and(|p| !p.props.is_empty());
    let needs_push = has_bindable || has_exports || has_ce_props || ctx.analysis.needs_context || ctx.dev;
    let has_component_exports = has_exports || has_ce_props || ctx.dev;

    let mut fn_body: Vec<Statement<'_>> = Vec::new();

    // $props.id() → must be first statement for hydration correctness
    if let Some(ref props_id_name) = ctx.analysis.props_id {
        let name: &str = ctx.b.alloc_str(props_id_name);
        let call = ctx.b.call_expr("$.props_id", std::iter::empty::<Arg<'_, '_>>());
        fn_body.push(ctx.b.const_stmt(name, call));
    }

    if ctx.dev {
        fn_body.push(ctx.b.expr_stmt(ctx.b.call_expr("$.check_target", [
            Arg::Expr(ctx.b.new_target_expr()),
        ])));
    }
    if needs_push {
        let mut push_args: Vec<Arg<'_, '_>> = vec![Arg::Ident("$$props"), Arg::Expr(ctx.b.bool_expr(true))];
        if ctx.dev {
            push_args.push(Arg::Ident(ctx.name));
        }
        fn_body.push(ctx.b.expr_stmt(ctx.b.call_expr("$.push", push_args)));
    }
    if ctx.needs_binding_group {
        fn_body.push(ctx.b.const_stmt("binding_group", ctx.b.empty_array_expr()));
    }

    // Store subscription setup:
    //   const $count = () => $.store_get(count, "$count", $$stores);
    //   const [$$stores, $$cleanup] = $.setup_stores();
    if has_stores {
        // Sort store base names for deterministic output
        let mut store_names: Vec<&str> = ctx.analysis.scoping.store_symbols().values().map(|s| s.as_str()).collect();
        store_names.sort();

        for base_name in &store_names {
            let dollar_name = format!("${}", base_name);
            let dollar_name_str: &str = ctx.b.alloc_str(&dollar_name);
            let base_str: &str = ctx.b.alloc_str(base_name);
            // const $name = () => $.store_get(name, "$name", $$stores)
            let store_get = ctx.b.call_expr("$.store_get", [
                Arg::Ident(base_str),
                Arg::Str(dollar_name.clone()),
                Arg::Ident("$$stores"),
            ]);
            let thunk = ctx.b.thunk(store_get);
            fn_body.push(ctx.b.const_stmt(dollar_name_str, thunk));
        }

        // const [$$stores, $$cleanup] = $.setup_stores()
        let setup_call = ctx.b.call_expr("$.setup_stores", std::iter::empty::<Arg<'_, '_>>());
        fn_body.push(ctx.b.const_array_destruct_stmt(&["$$stores", "$$cleanup"], setup_call));
    }

    // Instance-level snippet declarations (generated during root template for correct numbering)
    fn_body.extend(instance_snippets);

    fn_body.extend(script_body);

    // var $$exports = { ... }
    if has_exports || has_ce_props {
        let mut export_props: Vec<ObjProp<'_>> = Vec::new();

        // Regular exports (e.g., `export function reset()`)
        for e in &ctx.analysis.exports {
            let name: &str = ctx.b.alloc_str(&e.name);
            if let Some(alias) = &e.alias {
                let alias: &str = ctx.b.alloc_str(alias);
                export_props.push(ObjProp::KeyValue(alias, ctx.b.rid_expr(name)));
            } else {
                export_props.push(ObjProp::Shorthand(name));
            }
        }

        // Custom element prop getter/setters
        if has_ce_props {
            if let Some(ref props_analysis) = ctx.analysis.props {
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
                    let default_expr = prop.default_text.as_deref()
                        .map(|text| ctx.b.parse_expression(text));
                    let setter_body = vec![
                        ctx.b.expr_stmt(ctx.b.call_expr(local, [Arg::Ident("$$value")])),
                        ctx.b.call_stmt("$.flush", std::iter::empty::<Arg<'_, '_>>()),
                    ];
                    export_props.push(ObjProp::Setter(key, "$$value", default_expr, setter_body));
                }
            }
        }

        fn_body.push(ctx.b.var_stmt("$$exports", ctx.b.object_expr(export_props)));
    } else if ctx.dev && needs_push {
        // var $$exports = { ...$.legacy_api() }
        let legacy_call = ctx.b.call_expr("$.legacy_api", std::iter::empty::<Arg<'_, '_>>());
        fn_body.push(ctx.b.var_stmt("$$exports", ctx.b.object_expr([ObjProp::Spread(legacy_call)])));
    }

    fn_body.extend(template_body);

    if needs_push {
        if has_component_exports {
            fn_body.push(ctx.b.return_stmt(ctx.b.call_expr("$.pop", [Arg::Ident("$$exports")])));
        } else {
            fn_body.push(ctx.b.expr_stmt(ctx.b.call_expr("$.pop", std::iter::empty::<Arg<'_, '_>>())));
        }
    }

    // Store cleanup: $$cleanup() — runs after $.pop()
    if has_stores {
        fn_body.push(ctx.b.call_stmt("$$cleanup", std::iter::empty::<Arg<'_, '_>>()));
    }

    // -----------------------------------------------------------------------
    // 4. Module-level delegate calls
    // -----------------------------------------------------------------------
    let mut delegate_stmts: Vec<Statement<'_>> = Vec::new();
    if !ctx.delegated_events.is_empty() {
        let events: Vec<Arg<'_, '_>> = ctx.delegated_events.iter()
            .map(|e| Arg::Str(e.clone()))
            .collect();
        delegate_stmts.push(ctx.b.call_stmt("$.delegate", [
            Arg::Expr(ctx.b.array_from_args(events)),
        ]));
    }

    // -----------------------------------------------------------------------
    // 5. Assemble the program
    // -----------------------------------------------------------------------
    let b = &ctx.b;

    let import_svelte = b.import_all("$", "svelte/internal/client");

    // Bubble events on special elements (on:event with no expression) reference $$props
    let has_bubble_events = component.fragment.nodes.iter().any(|n| {
        let attrs = match n {
            Node::SvelteWindow(w) => Some(&w.attributes),
            Node::SvelteDocument(d) => Some(&d.attributes),
            _ => None,
        };
        attrs.is_some_and(|attrs| attrs.iter().any(|a| {
            matches!(a, Attribute::OnDirectiveLegacy(od) if od.expression_span.is_none())
        }))
    });

    let fn_params = if ctx.analysis.props.is_some() || needs_push || has_bubble_events {
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
    let fn_decl = b.function_decl(b.bid(ctx.name), fn_body, fn_params, body_span);
    let export_default = b.export_default(ExportDefaultDeclarationKind::FunctionDeclaration(
        b.alloc(fn_decl),
    ));

    let mut program_body: Vec<Statement<'_>> = Vec::new();
    if has_tracing || ctx.has_tracing {
        program_body.push(b.bare_import("svelte/internal/flags/tracing"));
    }
    if ctx.dev {
        // App[$.FILENAME] = "filename"
        let left = AssignLeft::ComputedMember(b.computed_member(
            b.rid_expr(ctx.name),
            b.static_member_expr(b.rid_expr("$"), "FILENAME"),
        ));
        let right = b.str_expr(ctx.filename);
        program_body.push(b.assign_stmt(left, right));
    }
    program_body.push(import_svelte);
    program_body.extend(script_imports);
    program_body.extend(all_hoisted);
    program_body.push(export_default);
    program_body.extend(delegate_stmts);

    // Custom element wrapping: customElements.define(tag, $.create_custom_element(App, ...))
    if let Some(ce_config) = component.options.as_ref().and_then(|o| o.custom_element.as_ref()) {
        let ce_stmts = custom_element::gen_custom_element(&mut ctx, ce_config);
        program_body.extend(ce_stmts);
    }

    let program = ctx.b.program(program_body, script_comments, script_source_text, script_span_end);

    Codegen::default().build(&program).code
}

/// Generate JavaScript for a standalone `.svelte.js`/`.svelte.ts` module.
/// Applies rune transforms but produces a plain ES module (no component wrapping).
pub fn generate_module(alloc: &Allocator, source: &str, is_ts: bool, analysis: &AnalysisData, dev: bool) -> String {
    let _ = dev; // reserved for future dev-mode codegen (e.g. $.tag, strict_equals)
    let arena_source: &str = alloc.alloc_str(source);
    let script_output = script::transform_module_script(alloc, arena_source, is_ts, &analysis.scoping);

    let b = Builder::new(alloc);
    let import_svelte = b.import_all("$", "svelte/internal/client");

    let mut program_body: Vec<Statement<'_>> = Vec::new();
    program_body.push(import_svelte);
    program_body.extend(script_output.imports);
    program_body.extend(script_output.body);

    let program = b.program(program_body, script_output.comments, script_output.source_text, script_output.program_span_end);
    Codegen::default().build(&program).code
}
