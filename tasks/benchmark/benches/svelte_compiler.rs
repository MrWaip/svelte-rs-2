use std::fs::read_to_string;

use benchmark::{BenchmarkId, Criterion, criterion_group, criterion_main};
use glob::glob;
use svelte_compiler::{CompileOptions, GenerateMode, ModuleCompileOptions};

struct Variant {
    label: &'static str,
    generate: GenerateMode,
    dev: bool,
}

const VARIANTS: [Variant; 4] = [
    Variant {
        label: "client/prod",
        generate: GenerateMode::Client,
        dev: false,
    },
    Variant {
        label: "client/dev",
        generate: GenerateMode::Client,
        dev: true,
    },
    Variant {
        label: "server/prod",
        generate: GenerateMode::Server,
        dev: false,
    },
    Variant {
        label: "server/dev",
        generate: GenerateMode::Server,
        dev: true,
    },
];

fn is_module(path: &str) -> bool {
    path.ends_with(".svelte.js") || path.ends_with(".js")
}

fn bench_svelte_compiler(criterion: &mut Criterion) {
    let pattern = format!("{}/benches/compiler/**/*", env!("CARGO_MANIFEST_DIR"));
    let mut files: Vec<_> = glob(&pattern)
        .expect("Не удалось считать компоненты")
        .filter_map(|entry| entry.ok())
        .filter(|path| {
            let name = path.to_string_lossy();
            name.ends_with(".svelte") || name.ends_with(".svelte.js")
        })
        .collect();
    files.sort();

    let mut group = criterion.benchmark_group("compiler");

    for path in files {
        let display = path.display().to_string();
        let source = read_to_string(&path).expect("test invariant");
        let module = is_module(&display);

        for variant in &VARIANTS {
            let id = BenchmarkId::from_parameter(format!("{display}::{}", variant.label));

            if module {
                let opts = ModuleCompileOptions {
                    dev: variant.dev,
                    generate: variant.generate,
                    filename: display.clone(),
                    ..ModuleCompileOptions::default()
                };
                group.bench_function(id, |b| {
                    b.iter(|| svelte_compiler::compile_module(&source, &opts));
                });
            } else {
                let opts = CompileOptions {
                    dev: variant.dev,
                    generate: variant.generate,
                    filename: display.clone(),
                    ..CompileOptions::default()
                };
                group.bench_function(id, |b| {
                    b.iter(|| svelte_compiler::compile(&source, &opts));
                });
            }
        }
    }

    group.finish();
}

criterion_group!(compiler, bench_svelte_compiler);
criterion_main!(compiler);
