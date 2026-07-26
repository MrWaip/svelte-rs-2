use std::env;
use std::fs;
use std::time::{Duration, Instant};

use benchmark as _;
use glob::glob;

fn is_module(path: &str) -> bool {
    path.ends_with(".svelte.js") || path.ends_with(".js")
}

fn main() {
    let mut seconds: u64 = 10;
    let mut dev = false;
    let mut root: Option<String> = None;
    let mut fixed_iters: Option<u64> = None;
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--dev" => dev = true,
            "--prod" => dev = false,
            "--dir" => root = Some(args.next().expect("--dir requires value")),
            "--iters" => {
                fixed_iters = Some(
                    args.next()
                        .expect("--iters requires value")
                        .parse()
                        .expect("u64"),
                );
            }
            other => seconds = other.parse().expect("seconds must be u64"),
        }
    }

    let pattern = match &root {
        Some(dir) => format!("{dir}/**/*"),
        None => format!("{}/benches/compiler/**/*", env!("CARGO_MANIFEST_DIR")),
    };
    let mut sources: Vec<(String, String, bool)> = glob(&pattern)
        .expect("glob")
        .filter_map(|e| e.ok())
        .filter(|p| {
            let n = p.to_string_lossy();
            (n.ends_with(".svelte") || n.ends_with(".svelte.js")) && !n.ends_with("big_v6.svelte")
        })
        .map(|p| {
            let d = p.display().to_string();
            let src = fs::read_to_string(&p).expect("read");
            let m = is_module(&d);
            (d, src, m)
        })
        .collect();
    sources.sort();

    let deadline = Instant::now() + Duration::from_secs(seconds);
    let mut iters: u64 = 0;
    while fixed_iters.map_or_else(|| Instant::now() < deadline, |target| iters < target) {
        for (path, source, module) in &sources {
            if *module {
                let opts = svelte_compiler::ModuleCompileOptions {
                    dev,
                    filename: path.clone(),
                    ..svelte_compiler::ModuleCompileOptions::default()
                };
                let _ = svelte_compiler::compile_module(source, &opts);
            } else {
                let opts = svelte_compiler::CompileOptions {
                    dev,
                    filename: path.clone(),
                    ..svelte_compiler::CompileOptions::default()
                };
                let _ = svelte_compiler::compile(source, &opts);
            }
        }
        iters += 1;
    }
    eprintln!("iters over suite: {iters} (dev: {dev})");
}
