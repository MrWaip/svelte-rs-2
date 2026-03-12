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

use builder::{Arg, ObjProp};
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
    // 2. Template generation (consumes "root" ident first)
    // -----------------------------------------------------------------------
    let (hoisted, template_body) = template::gen_root_fragment(&mut ctx);

    // -----------------------------------------------------------------------
    // 3. Snippet declarations (module-level, after root consumes "root")
    // -----------------------------------------------------------------------
    let mut snippet_stmts: Vec<Statement<'_>> = Vec::new();
    for node in &component.fragment.nodes {
        if let svelte_ast::Node::SnippetBlock(block) = node {
            let stmt = template::snippet::gen_snippet_block(&mut ctx, block.id);
            snippet_stmts.push(stmt);
        }
    }

    // Layout: snippets → inner hoisted → root hoisted
    let mut all_hoisted: Vec<Statement<'_>> = Vec::new();
    all_hoisted.extend(snippet_stmts);
    all_hoisted.extend(ctx.module_hoisted.drain(..));
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
    let has_exports = !ctx.analysis.exports.is_empty();
    let has_bindable = ctx.analysis.props.as_ref().is_some_and(|p| p.has_bindable);
    let needs_push = has_bindable || has_exports;

    let fn_params = if ctx.analysis.props.is_some() || needs_push {
        b.params(["$$anchor", "$$props"])
    } else {
        b.params(["$$anchor"])
    };

    let mut fn_body: Vec<Statement<'_>> = Vec::new();
    if ctx.needs_binding_group {
        fn_body.push(b.const_stmt("binding_group", b.empty_array_expr()));
    }
    if needs_push {
        fn_body.push(b.expr_stmt(b.call_expr("$.push", [
            Arg::Ident("$$props"),
            Arg::Expr(b.bool_expr(true)),
        ])));
    }
    fn_body.extend(script_body);

    // var $$exports = { PI, greet, ... }
    if has_exports {
        let props: Vec<ObjProp<'_>> = ctx.analysis.exports.iter().map(|e| {
            let name: &str = b.alloc_str(&e.name);
            if let Some(alias) = &e.alias {
                let alias: &str = b.alloc_str(alias);
                ObjProp::KeyValue(alias, b.rid_expr(name))
            } else {
                ObjProp::Shorthand(name)
            }
        }).collect();
        fn_body.push(b.var_stmt("$$exports", b.object_expr(props)));
    }

    fn_body.extend(template_body);

    if needs_push {
        if has_exports {
            fn_body.push(b.return_stmt(b.call_expr("$.pop", [Arg::Ident("$$exports")])));
        } else {
            fn_body.push(b.expr_stmt(b.call_expr("$.pop", std::iter::empty::<Arg<'_, '_>>())));
        }
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
