use std::env;
use std::fs;
use std::hint::black_box;
use std::io::{self, BufWriter, Write};
use std::time::{Duration, Instant};

use benchmark as _;

enum Mode {
    Compile,
    CompileModule,
}

fn main() {
    let mut path: Option<String> = None;
    let mut seconds: f64 = 3.0;
    let mut warmup: f64 = 0.5;
    let mut min_iters: u64 = 5;
    let mut dev = false;
    let mut async_ = false;
    let mut mode: Option<Mode> = None;

    let mut positional = 0usize;
    let args = env::args().skip(1);
    for arg in args {
        match arg.as_str() {
            "--dev" => dev = true,
            "--prod" => dev = false,
            "--async" => async_ = true,
            "--module" => mode = Some(Mode::CompileModule),
            "--compile" => mode = Some(Mode::Compile),
            _ => match positional {
                0 => {
                    path = Some(arg);
                    positional += 1;
                }
                1 => {
                    seconds = arg.parse().expect("seconds must be f64");
                    positional += 1;
                }
                2 => {
                    warmup = arg.parse().expect("warmup must be f64");
                    positional += 1;
                }
                3 => {
                    min_iters = arg.parse().expect("min_iters must be u64");
                    positional += 1;
                }
                _ => panic!("unexpected positional arg: {arg}"),
            },
        }
    }

    let path = path.expect(
        "usage: bench_once <path> [seconds] [warmup_s] [min_iters] [--dev] [--async] [--module]",
    );
    let source = fs::read_to_string(&path).expect("read source");

    let resolved_mode = mode.unwrap_or_else(|| {
        if path.ends_with(".svelte.js") || path.ends_with(".js") {
            Mode::CompileModule
        } else {
            Mode::Compile
        }
    });

    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    let warmup_dur = Duration::from_secs_f64(warmup);
    let bench_dur = Duration::from_secs_f64(seconds);

    match resolved_mode {
        Mode::Compile => {
            let opts = svelte_compiler::CompileOptions {
                dev,
                filename: path.clone(),
                experimental: svelte_compiler::ExperimentalOptions { async_ },
                ..svelte_compiler::CompileOptions::default()
            };
            let warmup_deadline = Instant::now() + warmup_dur;
            while Instant::now() < warmup_deadline {
                black_box(svelte_compiler::compile(&source, &opts));
            }
            let deadline = Instant::now() + bench_dur;
            let mut iters: u64 = 0;
            loop {
                let t0 = Instant::now();
                let result = svelte_compiler::compile(&source, &opts);
                let elapsed = t0.elapsed().as_nanos();
                black_box(result);
                writeln!(out, "{elapsed}").expect("write");
                iters += 1;
                if iters >= min_iters && Instant::now() >= deadline {
                    break;
                }
            }
        }
        Mode::CompileModule => {
            let opts = svelte_compiler::ModuleCompileOptions {
                dev,
                filename: path.clone(),
                ..svelte_compiler::ModuleCompileOptions::default()
            };
            let warmup_deadline = Instant::now() + warmup_dur;
            while Instant::now() < warmup_deadline {
                black_box(svelte_compiler::compile_module(&source, &opts));
            }
            let deadline = Instant::now() + bench_dur;
            let mut iters: u64 = 0;
            loop {
                let t0 = Instant::now();
                let result = svelte_compiler::compile_module(&source, &opts);
                let elapsed = t0.elapsed().as_nanos();
                black_box(result);
                writeln!(out, "{elapsed}").expect("write");
                iters += 1;
                if iters >= min_iters && Instant::now() >= deadline {
                    break;
                }
            }
        }
    }
}
