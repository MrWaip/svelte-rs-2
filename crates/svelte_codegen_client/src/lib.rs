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
    //    instance_snippets are generated inside gen_root_fragment for correct numbering
    // -----------------------------------------------------------------------
    let (hoisted, template_body, instance_snippets) = template::gen_root_fragment(&mut ctx);

    // Module-level snippet declarations (hoistable — don't reference script vars)
    let mut snippet_hoisted: Vec<Statement<'_>> = Vec::new();
    for node in &component.fragment.nodes {
        if let svelte_ast::Node::SnippetBlock(block) = node {
            if ctx.analysis.hoistable_snippets.contains(&block.id) {
                let stmt = template::snippet::gen_snippet_block(&mut ctx, block.id);
                snippet_hoisted.push(stmt);
            }
        }
    }

    // Layout: hoistable snippets → inner hoisted → root hoisted
    let mut all_hoisted: Vec<Statement<'_>> = Vec::new();
    all_hoisted.extend(snippet_hoisted);
    all_hoisted.extend(ctx.module_hoisted.drain(..));
    all_hoisted.extend(hoisted);

    // -----------------------------------------------------------------------
    // 3. Build function body (needs &mut ctx for snippets)
    // -----------------------------------------------------------------------
    let has_exports = !ctx.analysis.exports.is_empty();
    let has_bindable = ctx.analysis.props.as_ref().is_some_and(|p| p.has_bindable);
    let needs_push = has_bindable || has_exports;

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
