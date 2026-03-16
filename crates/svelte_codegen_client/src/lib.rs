mod builder;
mod context;
mod rune_transform;
mod script;
mod template;

use oxc_allocator::Allocator;
use oxc_ast::ast::{ExportDefaultDeclarationKind, Statement};
use oxc_codegen::Codegen;

use svelte_analyze::{AnalysisData, IdentGen, ParsedExprs};
use svelte_ast::Component;

use builder::{Arg, Builder, ObjProp};
use context::Ctx;

/// Generate JavaScript client-side code for a compiled Svelte component.
pub fn generate<'a>(alloc: &'a Allocator, component: &'a Component, analysis: &'a AnalysisData, parsed: &'a mut ParsedExprs<'a>, ident_gen: &'a mut IdentGen) -> String {
    let mut ctx = Ctx::new(alloc, component, analysis, parsed, ident_gen);

    // -----------------------------------------------------------------------
    // 1. Script transformation
    // -----------------------------------------------------------------------
    let (script_imports, script_body) = script::gen_script(&mut ctx);

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
    let has_exports = !ctx.analysis.exports.is_empty();
    let has_bindable = ctx.analysis.props.as_ref().is_some_and(|p| p.has_bindable);
    let has_stores = !ctx.analysis.store_subscriptions.is_empty();
    let needs_push = has_bindable || has_exports || ctx.analysis.needs_context;

    let mut fn_body: Vec<Statement<'_>> = Vec::new();
    if ctx.needs_binding_group {
        fn_body.push(ctx.b.const_stmt("binding_group", ctx.b.empty_array_expr()));
    }
    if needs_push {
        fn_body.push(ctx.b.expr_stmt(ctx.b.call_expr("$.push", [
            Arg::Ident("$$props"),
            Arg::Expr(ctx.b.bool_expr(true)),
        ])));
    }

    // Store subscription setup:
    //   const $count = () => $.store_get(count, "$count", $$stores);
    //   const [$$stores, $$cleanup] = $.setup_stores();
    if has_stores {
        // Sort store names for deterministic output
        let mut store_names: Vec<&String> = ctx.analysis.store_subscriptions.iter().collect();
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

    // var $$exports = { PI, greet, ... }
    if has_exports {
        let props: Vec<ObjProp<'_>> = ctx.analysis.exports.iter().map(|e| {
            let name: &str = ctx.b.alloc_str(&e.name);
            if let Some(alias) = &e.alias {
                let alias: &str = ctx.b.alloc_str(alias);
                ObjProp::KeyValue(alias, ctx.b.rid_expr(name))
            } else {
                ObjProp::Shorthand(name)
            }
        }).collect();
        fn_body.push(ctx.b.var_stmt("$$exports", ctx.b.object_expr(props)));
    }

    fn_body.extend(template_body);

    if needs_push {
        if has_exports {
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

    let fn_params = if ctx.analysis.props.is_some() || needs_push {
        b.params(["$$anchor", "$$props"])
    } else {
        b.params(["$$anchor"])
    };

    let fn_decl = b.function_decl(b.bid("App"), fn_body, fn_params);
    let export_default = b.export_default(ExportDefaultDeclarationKind::FunctionDeclaration(
        b.alloc(fn_decl),
    ));

    let mut program_body: Vec<Statement<'_>> = Vec::new();
    program_body.push(import_svelte);
    program_body.extend(script_imports);
    program_body.extend(all_hoisted);
    program_body.push(export_default);
    program_body.extend(delegate_stmts);

    let program = b.program(program_body);

    Codegen::default().build(&program).code
}

/// Generate JavaScript for a standalone `.svelte.js`/`.svelte.ts` module.
/// Applies rune transforms but produces a plain ES module (no component wrapping).
pub fn generate_module(alloc: &Allocator, source: &str, is_ts: bool, analysis: &AnalysisData) -> String {
    let arena_source: &str = alloc.alloc_str(source);
    let (imports, body) = script::transform_module_script(alloc, arena_source, is_ts, &analysis.scoping);

    let b = Builder::new(alloc);
    let import_svelte = b.import_all("$", "svelte/internal/client");

    let mut program_body: Vec<Statement<'_>> = Vec::new();
    program_body.push(import_svelte);
    program_body.extend(imports);
    program_body.extend(body);

    let program = b.program(program_body);
    Codegen::default().build(&program).code
}
