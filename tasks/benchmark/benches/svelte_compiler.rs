use std::fs::read_to_string;

use benchmark::{BenchmarkId, Criterion, criterion_group, criterion_main};
use glob::glob;

fn bench_svelte_compiler(criterion: &mut Criterion) {
    let pattern = format!(
        "{}/benches/compiler/**/*.svelte",
        env!("CARGO_MANIFEST_DIR")
    );
    let files = glob(&pattern).expect("Не удалось считать компоненты");

    let mut group = criterion.benchmark_group("compiler");

    for entry in files {
        let path = entry.expect("test invariant");
        let source = read_to_string(&path).expect("test invariant");
        let id = BenchmarkId::from_parameter(path.display().to_string());

        let opts = svelte_compiler::CompileOptions::default();
        group.bench_function(id, |b| {
            b.iter(|| svelte_compiler::compile(&source, &opts));
        });
    }

    group.finish();
}

criterion_group!(compiler, bench_svelte_compiler);
criterion_main!(compiler);
