use std::cmp::Ordering;
use std::env;
use std::fs;
use std::hint::black_box;
use std::path::PathBuf;
use std::time::Instant;

use benchmark as _;
use rayon::prelude::*;
use svelte_compiler::{CompileOptions, GenerateMode};

struct Args {
    dir: String,
    parallel: bool,
    threads: usize,
    both_targets: bool,
}

fn parse_args() -> Args {
    let mut dir = String::from(".");
    let mut parallel = false;
    let mut threads = 0;
    let mut both_targets = true;

    for arg in env::args().skip(1) {
        if arg == "--parallel" {
            parallel = true;
            continue;
        }
        if arg == "--client-only" {
            both_targets = false;
            continue;
        }
        if let Some(value) = arg.strip_prefix("--threads=") {
            threads = value.parse().unwrap_or(0);
            continue;
        }
        dir = arg;
    }

    Args {
        dir,
        parallel,
        threads,
        both_targets,
    }
}

fn collect_files(dir: &str) -> Vec<PathBuf> {
    let pattern = format!("{dir}/**/*.svelte");
    let mut files: Vec<PathBuf> = glob::glob(&pattern)
        .expect("glob pattern")
        .filter_map(Result::ok)
        .collect();
    files.sort();
    files
}

fn compile_one(source: &str, path: &str, generate: GenerateMode) -> usize {
    let options = CompileOptions {
        dev: false,
        filename: path.to_string(),
        generate,
        transform_typescript: true,
        ..CompileOptions::default()
    };
    let result = svelte_compiler::compile(source, &options);
    result.js.map_or(0, |js| js.code.len())
}

fn main() {
    let args = parse_args();
    if args.threads > 0 {
        let _ = rayon::ThreadPoolBuilder::new()
            .num_threads(args.threads)
            .build_global();
    }

    let files = collect_files(&args.dir);
    let sources: Vec<(String, String)> = files
        .iter()
        .filter_map(|path| {
            let source = fs::read_to_string(path).ok()?;
            Some((path.to_string_lossy().into_owned(), source))
        })
        .collect();

    let targets: Vec<GenerateMode> = match args.both_targets {
        true => vec![GenerateMode::Client, GenerateMode::Server],
        false => vec![GenerateMode::Client],
    };

    let mut per_file: Vec<(f64, String)> = Vec::new();
    let started = Instant::now();
    let mut bytes = 0usize;
    if args.parallel {
        bytes = sources
            .par_iter()
            .map(|(path, source)| {
                let mut local = 0;
                for target in &targets {
                    local += compile_one(source, path, *target);
                }
                local
            })
            .sum();
    } else {
        for (path, source) in &sources {
            let mut best = f64::MAX;
            for _ in 0..5 {
                let file_started = Instant::now();
                for target in &targets {
                    bytes += compile_one(source, path, *target);
                }
                let ms = file_started.elapsed().as_secs_f64() * 1000.0;
                if ms < best {
                    best = ms;
                }
            }
            per_file.push((best, path.clone()));
        }
    }
    let elapsed = started.elapsed();
    black_box(bytes);

    if !per_file.is_empty() {
        let mut times: Vec<f64> = per_file.iter().map(|(ms, _)| *ms).collect();
        times.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
        let percentile = |p: f64| times[((times.len() as f64 - 1.0) * p) as usize];
        let sum: f64 = times.iter().sum();
        let top1 = ((times.len() as f64) * 0.01) as usize;
        let top1_sum: f64 = times.iter().rev().take(top1.max(1)).sum();
        println!(
            "per-file ms: p50={:.3} p90={:.3} p99={:.3} max={:.3} | top1%={} files = {:.1}% of total",
            percentile(0.5),
            percentile(0.9),
            percentile(0.99),
            times[times.len() - 1],
            top1.max(1),
            top1_sum / sum * 100.0
        );
        let mut slowest = per_file.clone();
        slowest.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(Ordering::Equal));
        for (ms, path) in slowest.iter().take(5) {
            println!("   {:.3} ms  {}", ms, path);
        }
    }

    let compiles = sources.len() * targets.len();
    println!(
        "files={} compiles={} mode={} threads={} total={:.2}s per_compile={:.3}ms out={:.1}MB",
        sources.len(),
        compiles,
        if args.parallel { "parallel" } else { "serial" },
        match args.threads {
            0 => rayon::current_num_threads(),
            n => n,
        },
        elapsed.as_secs_f64(),
        elapsed.as_secs_f64() * 1000.0 / compiles as f64,
        bytes as f64 / 1024.0 / 1024.0
    );
}
