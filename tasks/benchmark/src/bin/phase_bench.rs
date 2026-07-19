use std::env;
use std::fs;
use std::hint::black_box;
use std::time::Instant;

use benchmark as _;

fn main() {
    let path = env::args()
        .nth(1)
        .expect("usage: phase_bench <path> [iters]");
    let iters: u64 = env::args()
        .nth(2)
        .and_then(|s| s.parse().ok())
        .unwrap_or(500);
    let source = fs::read_to_string(&path).expect("read source");

    let options = svelte_compiler::CompileOptions {
        dev: false,
        filename: path.clone(),
        ..svelte_compiler::CompileOptions::default()
    };

    for _ in 0..100 {
        black_box(svelte_compiler::compile(&source, &options));
    }

    let mut parse_ns: u128 = 0;
    let mut analyze_ns: u128 = 0;
    let mut transform_ns: u128 = 0;
    let mut codegen_ns: u128 = 0;
    let mut total_ns: u128 = 0;

    let analyze_opts = svelte_analyze::AnalyzeOptions {
        custom_element: options.custom_element,
        experimental_async: options.experimental.async_,
        runes: options.runes,
        inline_runes: None,
        accessors: false,
        immutable: false,
        preserve_whitespace: false,
        preserve_comments: options.preserve_comments,
        dev: options.dev,
        component_name: "Component".to_string(),
        filename_basename: "test.svelte".to_string(),
        warning_filter: None,
    };

    for _ in 0..iters {
        let t_total = Instant::now();

        let t0 = Instant::now();
        let js_alloc = oxc_allocator::Allocator::default();
        let (component, js_result, _diags) = svelte_parser::parse_with_js(&js_alloc, &source);
        parse_ns += t0.elapsed().as_nanos();

        let t0 = Instant::now();
        let (analysis, mut parsed, _analyze_diags) =
            svelte_analyze::analyze_with_options(&component, js_result, &analyze_opts);
        analyze_ns += t0.elapsed().as_nanos();

        let mut ident_gen =
            svelte_analyze::IdentGen::with_conflicts(analysis.scoping.collect_all_symbol_names());
        let name = analysis.component_name().to_string();
        let _ = ident_gen.generate(&name);
        let line_index = svelte_span::LineIndex::empty();
        let transform_options = svelte_types::TransformOptions { dev: false };
        let no_map = env::args().any(|a| a == "--no-sourcemap");
        let codegen_options = svelte_types::CodegenOptions {
            sourcemap_kind: if no_map {
                svelte_sourcemap::SourcemapKind::None
            } else {
                svelte_sourcemap::SourcemapKind::Default
            },
            ..svelte_types::CodegenOptions::default()
        };

        let t0 = Instant::now();
        let transform_data = {
            let mut compile_ctx = svelte_types::CompileContext {
                alloc: &js_alloc,
                component: &component,
                analysis: &analysis,
                js_arena: &mut parsed,
                ident_gen: &mut ident_gen,
                line_index: &line_index,
            };
            svelte_transform_client::transform_component(&mut compile_ctx, &transform_options)
        };
        transform_ns += t0.elapsed().as_nanos();

        let t0 = Instant::now();
        let compile_ctx = svelte_types::CompileContext {
            alloc: &js_alloc,
            component: &component,
            analysis: &analysis,
            js_arena: &mut parsed,
            ident_gen: &mut ident_gen,
            line_index: &line_index,
        };
        let result =
            svelte_codegen_client::generate(compile_ctx, &codegen_options, transform_data, None);
        codegen_ns += t0.elapsed().as_nanos();

        black_box(result);
        total_ns += t_total.elapsed().as_nanos();
    }

    let d = iters as f64;
    let total = total_ns as f64 / d;
    let parse = parse_ns as f64 / d;
    let analyze = analyze_ns as f64 / d;
    let transform = transform_ns as f64 / d;
    let codegen = codegen_ns as f64 / d;
    let overhead = total - parse - analyze - transform - codegen;

    println!("File: {path}  ({} lines)", source.lines().count());
    println!("Iterations: {iters}");
    println!();
    println!("Total:     {:8.0} ns  ({:.3} ms)", total, total / 1e6);
    println!(
        "  Parse:     {:8.0} ns  ({:5.1}%)",
        parse,
        parse / total * 100.0
    );
    println!(
        "  Analyze:   {:8.0} ns  ({:5.1}%)",
        analyze,
        analyze / total * 100.0
    );
    println!(
        "  Transform: {:8.0} ns  ({:5.1}%)",
        transform,
        transform / total * 100.0
    );
    println!(
        "  Codegen:   {:8.0} ns  ({:5.1}%)",
        codegen,
        codegen / total * 100.0
    );
    println!(
        "  Overhead:  {:8.0} ns  ({:5.1}%)",
        overhead,
        overhead / total * 100.0
    );
}
