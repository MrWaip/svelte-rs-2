use std::fs::read_to_string;

use benchmark::{criterion_group, criterion_main, BenchmarkId, Criterion};
use compiler::{self, Compiler};
use glob::glob;
use oxc_allocator::Allocator;

fn bench_svelte_compiler(criterion: &mut Criterion) {
    let files = glob("./benches/compiler/**/*.svelte").expect("Не удалось считать компоненты");

    let mut group = criterion.benchmark_group("compiler");

    for entry in files {
        let path = entry.unwrap();
        let source = read_to_string(&path).unwrap();
        let id = BenchmarkId::from_parameter(&path.display().to_string());
        let mut allocator = Allocator::default();

        group.bench_function(id, |b| {
            b.iter_with_setup_wrapper(|runner| {
                allocator.reset();

                let _ = runner.run(|| {
                    let compiler = Compiler::new();

                    let result = compiler.compile(&source, &allocator);

                    result
                });
            });
        });
    }

    group.finish();
}

criterion_group!(compiler, bench_svelte_compiler);
criterion_main!(compiler);
