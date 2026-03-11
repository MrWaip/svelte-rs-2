mod builder;
mod context;
mod rune_transform;
mod script;
mod template;

use oxc_allocator::Allocator;
use oxc_ast::ast::{ExportDefaultDeclarationKind, Statement};
use oxc_codegen::Codegen;

use svelte_analyze::AnalysisData;
use svelte_ast::Component;

use builder::Arg;
use context::Ctx;

/// Generate JavaScript client-side code for a compiled Svelte component.
pub fn generate(component: &Component, analysis: &AnalysisData) -> String {
    let allocator = Allocator::default();
    let mut ctx = Ctx::new(&allocator, component, analysis);

    // -----------------------------------------------------------------------
    // 1. Script transformation
    // -----------------------------------------------------------------------
    let (script_imports, script_body) = script::gen_script(&mut ctx);

    // -----------------------------------------------------------------------
    // 2. Template generation
    // -----------------------------------------------------------------------
    let (hoisted, template_body) = template::gen_root_fragment(&mut ctx);

    // Nested fragment templates go before root template
    let mut all_hoisted: Vec<Statement<'_>> = ctx.module_hoisted.drain(..).collect();
    all_hoisted.extend(hoisted);

    // -----------------------------------------------------------------------
    // 3. Assemble the program
    // -----------------------------------------------------------------------
    let b = &ctx.b;

    // import * as $ from "svelte/internal/client";
    let import_svelte = b.import_all("$", "svelte/internal/client");

    // User imports from script (e.g. import { foo } from '...')
    // (passthrough from script_imports)

    // var root = $.template(...) — hoisted template declarations
    // (already in `hoisted`)

    // export default function App($$anchor, $$props?) { ... }
    let fn_params = if ctx.analysis.props.is_some() {
        b.params(["$$anchor", "$$props"])
    } else {
        b.params(["$$anchor"])
    };
    let has_bindable = ctx.analysis.props.as_ref().is_some_and(|p| p.has_bindable);

    let mut fn_body: Vec<Statement<'_>> = Vec::new();
    if ctx.needs_binding_group {
        fn_body.push(b.const_stmt("binding_group", b.empty_array_expr()));
    }
    if has_bindable {
        fn_body.push(b.expr_stmt(b.call_expr("$.push", [
            Arg::Ident("$$props"),
            Arg::Expr(b.bool_expr(true)),
        ])));
    }
    fn_body.extend(script_body);
    fn_body.extend(template_body);
    if has_bindable {
        fn_body.push(b.expr_stmt(b.call_expr("$.pop", std::iter::empty::<Arg<'_, '_>>())));
    }

    let fn_decl = b.function_decl(b.bid("App"), fn_body, fn_params);
    let export_default = b.export_default(ExportDefaultDeclarationKind::FunctionDeclaration(
        b.alloc(fn_decl),
    ));

    let mut program_body: Vec<Statement<'_>> = Vec::new();
    program_body.push(import_svelte);
    program_body.extend(script_imports);
    program_body.extend(all_hoisted);
    program_body.push(export_default);

    let program = b.program(program_body);

    Codegen::default().build(&program).code
}
