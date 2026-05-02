use std::env;
use std::fs;
use std::time::{Duration, Instant};

fn main() {
    let path = env::args()
        .nth(1)
        .expect("usage: profile <path/to/case.svelte> [seconds]");
    let seconds: u64 = env::args()
        .nth(2)
        .and_then(|s| s.parse().ok())
        .unwrap_or(10);
    let source = fs::read_to_string(&path).expect("read source");
    let opts = svelte_compiler::CompileOptions::default();
    let deadline = Instant::now() + Duration::from_secs(seconds);
    let mut iters: u64 = 0;
    while Instant::now() < deadline {
        let _ = svelte_compiler::compile(&source, &opts);
        iters += 1;
    }
    eprintln!("iters: {iters}");
}
