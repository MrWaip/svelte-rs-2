use std::fs::read_to_string;

use benchmark::{BenchmarkId, Criterion, criterion_group, criterion_main};
use glob::glob;

fn bench_svelte_compile_module_dev(criterion: &mut Criterion) {
    let files = glob("./benches/compiler/**/*.svelte.js").expect("Не удалось считать модули");

    let mut group = criterion.benchmark_group("compile_module_dev");

    for entry in files {
        let path = entry.expect("test invariant");
        let source = read_to_string(&path).expect("test invariant");
        let id = BenchmarkId::from_parameter(path.display().to_string());

        let opts = svelte_compiler::ModuleCompileOptions {
            dev: true,
            filename: path.display().to_string(),
            ..svelte_compiler::ModuleCompileOptions::default()
        };
        group.bench_function(id, |b| {
            b.iter(|| svelte_compiler::compile_module(&source, &opts));
        });
    }

    group.finish();
}

criterion_group!(compile_module_dev, bench_svelte_compile_module_dev);
criterion_main!(compile_module_dev);
