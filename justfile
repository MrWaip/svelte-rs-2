# Generate expected JS output (case-svelte.js) for all compiler test cases
generate:
    cargo run -p generate_test_cases

# Run all diagnostic integration tests
test-diagnostics:
    cargo test -p diagnostic_tests --test diagnostic_tests

# Run a single diagnostic test case
test-diagnostic-case name:
    cargo test -p diagnostic_tests --test diagnostic_tests {{name}} -- --include-ignored

# Run all compiler integration tests
test-compiler:
    cargo test -p compiler_tests --test compiler_tests_v3

# Run a single compiler test case
test-case name:
    cargo test -p compiler_tests --test compiler_tests_v3 {{name}} -- --include-ignored

# Run a single compiler test case with output
test-case-verbose name:
    cargo test -p compiler_tests --test compiler_tests_v3 {{name}} -- --include-ignored --nocapture

# Run all tests across all crates
test-all:
    cargo test --workspace

# Remove Cargo build artifacts, including incremental caches, not used for 2 days
sweep-2d:
    cargo sweep -t 2

# Run Clippy across the workspace
clippy:
    cargo clippy --workspace --all-targets

# Run Clippy and fail on any warning
clippy-strict:
    cargo clippy --workspace --all-targets -- -D warnings

# Apply Clippy's machine-applicable fixes across the workspace
clippy-fix:
    cargo clippy --workspace --all-targets --fix --allow-dirty --allow-staged

# Run parser tests
test-parser:
    cargo test -p svelte_parser

# Run analyzer tests
test-analyzer:
    cargo test -p svelte_analyze

# Run all Rust benchmarks
bench:
    cargo bench -p benchmark

# Run Node benchmarks against svelte/compiler
bench-node:
    node tasks/benchmark/bench.mjs

# Run a single bench by substring filter
bench-case filter:
    cargo bench -p benchmark --bench svelte_compiler -- '{{filter}}'

# Profile a single .svelte file with samply. Writes profile/profile.json. Requires: cargo install samply && samply setup.
# Usage: just bench-flame tasks/benchmark/benches/compiler/snippets/case_01.svelte 5
bench-flame path seconds='5':
    cargo build --profile profiling -p benchmark --bin profile
    mkdir -p profile
    samply record --save-only --no-open --unstable-presymbolicate -d {{seconds}} -o profile/profile.json.gz -- target/profiling/profile '{{path}}' {{seconds}}
    gunzip -f profile/profile.json.gz
    node tasks/benchmark/scripts/gecko-to-folded.mjs profile

# Dump OXC AST as JSON for a JS expression
dump-ast expr:
    cargo run -p svelte_parser --example dump_ast -- '{{expr}}'

# Quick-check one Svelte component against the reference compiler (usage: just quick-check path/to/component.svelte)
quick-check path:
    cargo run -q -p quick_check -- {{path}}

# Build WASM and serve the playground
playground:
    wasm-pack build --target web ./crates/wasm_compiler -d ../../docs/compiler
    cd docs && python3 -m http.server 8080

# Build the debug addon, wire it into the local package, and run the JS smoke test
npm-smoke:
    cargo build -p napi_compiler
    node packages/svelte-rs2/scripts/prepare-local-main-package.mjs
    node packages/svelte-rs2/scripts/smoke.mjs

# Build production-like local npm tarballs for testing in a consumer app
npm-build:
    cargo build -p napi_compiler --release
    npm run --prefix packages/svelte-rs2 prepare-platform-package
    npm pack ./packages/svelte-rs2 --silent
    npm pack ./packages/svelte-rs2-linux-x64-gnu --silent
    npm pack ./packages/svelte-rs2-darwin-arm64 --silent
    npm pack ./packages/svelte-rs2-darwin-x64 --silent
