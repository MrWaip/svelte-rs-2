use std::fs;
use std::panic::{AssertUnwindSafe, catch_unwind, set_hook, take_hook};
use std::path::{Path, PathBuf};

use compiler_tests::cases::{load_cluster_case, load_v3_case};
use svelte_compiler::{CompileOptions, GenerateMode, compile};

fn collect_component_cases(root: &Path) -> Vec<String> {
    let mut found = Vec::new();
    let mut pending = vec![root.to_path_buf()];
    while let Some(dir) = pending.pop() {
        if dir.join("case.svelte").exists() {
            if let Ok(rel) = dir.strip_prefix(root) {
                found.push(rel.to_string_lossy().replace('\\', "/"));
            }
            continue;
        }
        let Ok(entries) = fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                pending.push(path);
            }
        }
    }
    found.sort();
    found
}

fn server_output(input: &str, opts: &CompileOptions, dev: bool) -> Option<String> {
    let mut server_opts = opts.clone();
    server_opts.generate = GenerateMode::Server;
    server_opts.dev = dev;
    let outcome = catch_unwind(AssertUnwindSafe(|| compile(input, &server_opts)));
    match outcome {
        Ok(result) => result.js.map(|out| out.code),
        Err(_) => None,
    }
}

fn reference(dir: &Path, dev: bool) -> Option<String> {
    let name = if dev {
        "case-svelte.server.dev.js"
    } else {
        "case-svelte.server.js"
    };
    fs::read_to_string(dir.join(name)).ok()
}

fn scan(base: &str, load: fn(&str) -> (String, CompileOptions), ready: &mut Vec<String>) -> usize {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(base);
    let cases = collect_component_cases(&root);
    let total = cases.len();
    for case in cases {
        let dir = root.join(&case);
        let (Some(prod_ref), Some(dev_ref)) = (reference(&dir, false), reference(&dir, true))
        else {
            continue;
        };
        let (input, opts) = load(&case);
        let prod_matches = server_output(&input, &opts, false).is_some_and(|js| js == prod_ref);
        if !prod_matches {
            continue;
        }
        let dev_matches = server_output(&input, &opts, true).is_some_and(|js| js == dev_ref);
        if dev_matches {
            ready.push(format!("{base}/{case}"));
        }
    }
    total
}

fn main() {
    let previous_hook = take_hook();
    set_hook(Box::new(|_| {}));
    let mut ready = Vec::new();
    let mut total = 0;
    total += scan("cases2", load_v3_case, &mut ready);
    total += scan("cluster_cases", load_cluster_case, &mut ready);
    set_hook(previous_hook);

    for case in &ready {
        println!("{case}");
    }
    eprintln!(
        "ssr pairs matching references (prod + dev): {} of {total} cases; flip candidates above (already-live pairs included)",
        ready.len()
    );
}
