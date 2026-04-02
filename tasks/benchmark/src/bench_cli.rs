use std::fs::read_to_string;
use std::time::Instant;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let file = args
        .get(1)
        .expect("Usage: bench_cli <file.svelte> [iterations]");
    let iterations: u64 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(100);

    let source = read_to_string(file).expect("failed to read file");

    // Warmup
    for _ in 0..5 {
        let _ = svelte_compiler::compile(&source, &svelte_compiler::CompileOptions::default());
    }

    // Measure
    let mut times_us: Vec<u64> = Vec::with_capacity(iterations as usize);

    for _ in 0..iterations {
        let start = Instant::now();
        let _ = svelte_compiler::compile(&source, &svelte_compiler::CompileOptions::default());
        times_us.push(start.elapsed().as_micros() as u64);
    }

    times_us.sort();

    let median = times_us[times_us.len() / 2];
    let min = times_us[0];
    let max = times_us[times_us.len() - 1];
    let mean: u64 = times_us.iter().sum::<u64>() / iterations;

    println!(
        r#"{{"median_us":{},"mean_us":{},"min_us":{},"max_us":{},"iterations":{}}}"#,
        median, mean, min, max, iterations
    );
}
