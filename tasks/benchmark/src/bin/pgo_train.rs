use std::env;
use std::fs;
use std::hint::black_box;

use benchmark as _;
use glob::glob;

const DEFAULT_ROUNDS: u32 = 3;

fn is_module(path: &str) -> bool {
    path.ends_with(".svelte.js") || path.ends_with(".svelte.ts")
}

fn corpus_patterns(extra: &[String]) -> Vec<String> {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");
    let mut patterns = vec![
        format!("{root}/tasks/benchmark/benches/compiler/**/*.svelte"),
        format!("{root}/tasks/benchmark/benches/compiler/**/*.svelte.js"),
        format!("{root}/tasks/compiler_tests/cases2/**/*.svelte"),
        format!("{root}/tasks/compiler_tests/cases2/**/*.svelte.js"),
    ];
    for dir in extra {
        patterns.push(format!("{dir}/**/*.svelte"));
        patterns.push(format!("{dir}/**/*.svelte.js"));
    }
    patterns
}

fn main() {
    let mut rounds = DEFAULT_ROUNDS;
    let mut extra_dirs: Vec<String> = Vec::new();
    for arg in env::args().skip(1) {
        match arg.parse::<u32>() {
            Ok(value) => rounds = value,
            Err(_) => extra_dirs.push(arg),
        }
    }

    let mut sources: Vec<(String, String, bool)> = Vec::new();
    for pattern in corpus_patterns(&extra_dirs) {
        let Ok(entries) = glob(&pattern) else {
            continue;
        };
        for entry in entries.filter_map(Result::ok) {
            let path = entry.display().to_string();
            let Ok(source) = fs::read_to_string(&entry) else {
                continue;
            };
            let module = is_module(&path);
            sources.push((path, source, module));
        }
    }
    sources.sort();
    sources.dedup_by(|a, b| a.0 == b.0);

    eprintln!("pgo_train: {} files x {rounds} rounds", sources.len());

    let generate_modes = [
        svelte_compiler::GenerateMode::Client,
        svelte_compiler::GenerateMode::Server,
    ];

    for _ in 0..rounds {
        for (path, source, module) in &sources {
            for dev in [false, true] {
                for generate in generate_modes {
                    if *module {
                        let options = svelte_compiler::ModuleCompileOptions {
                            dev,
                            generate,
                            filename: path.clone(),
                            ..svelte_compiler::ModuleCompileOptions::default()
                        };
                        black_box(svelte_compiler::compile_module(source, &options));
                    } else {
                        let options = svelte_compiler::CompileOptions {
                            dev,
                            generate,
                            filename: path.clone(),
                            ..svelte_compiler::CompileOptions::default()
                        };
                        black_box(svelte_compiler::compile(source, &options));
                    }
                }
            }
        }
    }
}
