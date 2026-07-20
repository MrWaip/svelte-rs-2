# Generate expected JS output (case-svelte.js) for all compiler test cases
generate:
    cargo run -p generate_test_cases

# Rebuild napi in release and benchmark our compiler vs svelte/compiler across .svelte files in dir (default: whole repo)
bench-compare dir='.':
    npm run --prefix packages/svelte-rs2 build:release
    node tasks/compiler_bench/compare.mjs {{dir}}

# Run all diagnostic integration tests (aggregated summary via nextest)
test-diagnostics:
    cargo nextest run -p diagnostic_tests --status-level fail

# Run a single diagnostic test case
test-diagnostic-case name:
    cargo test -p diagnostic_tests --test diagnostic_tests {{name}} -- --include-ignored

# Run all compiler integration tests (aggregated summary via nextest)
test-compiler:
    cargo nextest run -p compiler_tests --status-level fail

# List only #[ignore]d tests — known divergences (pkg defaults to compiler_tests)
test-ignored-list pkg='compiler_tests':
    cargo nextest list -p {{pkg}} --run-ignored ignored-only

# Run only #[ignore]d tests — see which divergences now pass (pkg defaults to compiler_tests)
test-ignored pkg='compiler_tests':
    cargo nextest run -p {{pkg}} --run-ignored ignored-only --no-fail-fast

# Run a single compiler test case
test-case name:
    cargo test -p compiler_tests --test compiler_tests_v3 {{name}} -- --include-ignored

# Run a single compiler test case with output
test-case-verbose name:
    cargo test -p compiler_tests --test compiler_tests_v3 {{name}} -- --include-ignored --nocapture

# Run a single cluster test case (cluster_cases/)
test-cluster name:
    cargo test -p compiler_tests --test compiler_tests_clusters {{name}} -- --include-ignored

# Run all tests across all crates (aggregated summary via nextest; pass extra flags via `just test-all --locked`)
test-all *args:
    cargo nextest run --workspace --status-level fail {{args}}

# Remove Cargo build artifacts, including incremental caches, not used for 2 days
sweep-2d:
    cargo sweep -t 2

# Run all lints (clippy + custom dylint rules) and fail on any warning
lint:
    cargo clippy --workspace --all-targets -- -D warnings
    DYLINT_RUSTFLAGS="-D warnings --cap-lints=deny" cargo dylint --all

# Apply Clippy's machine-applicable fixes across the workspace
clippy-fix:
    cargo clippy --workspace --all-targets --fix --allow-dirty --allow-staged

# Run parser tests (aggregated summary via nextest)
test-parser:
    cargo nextest run -p svelte_parser --status-level fail

# Run analyzer tests (aggregated summary via nextest)
test-analyzer:
    cargo nextest run -p svelte_analyze --status-level fail

# Walltime benchmark across all cases (criterion mean ms).
bench-walltime-all:
    cargo bench -p benchmark

# Run Node benchmarks against svelte/compiler
bench-node:
    node tasks/benchmark/bench.mjs

# Walltime comparison: rust vs svelte/compiler. Time-budgeted per side, mean/median/speedup table. Flags: --seconds T --warmup W --min-iters N --filter SUBSTR.
bench-compare-walltime *flags:
    cargo build --release -p benchmark --bin bench_once
    node tasks/benchmark/scripts/compare-walltime.mjs {{flags}}

# Run a single bench by substring filter (criterion mean ms).
bench-case filter:
    cargo bench -p benchmark --bench svelte_compiler -- '{{filter}}'

# Profile one file. Extra flags forwarded to profile bin: --dev, --mode compile|compile_module. Requires: cargo install samply && samply setup.
bench-flame path *flags:
    cargo build --profile profiling -p benchmark --bin profile
    bash tasks/benchmark/scripts/bench-flame.sh '{{path}}' {{flags}}

# Profile every case × {prod, dev}. Writes profile/<slug>_<mode>/* per case + profile/aggregate.top.txt.
bench-flame-all:
    cargo build --profile profiling -p benchmark --bin profile
    bash tasks/benchmark/scripts/bench-flame-all.sh

# Dump OXC AST as JSON for a JS expression
dump-ast expr:
    cargo run -p svelte_parser --example dump_ast -- '{{expr}}'

# Quick-check one Svelte component against the reference compiler (usage: just quick-check path/to/component.svelte [--mode=auto|runes|legacy] [--generate=client|server] [--dev] [--filename=<name>])
quick-check path *flags:
    cargo run -q -p quick_check -- {{path}} {{flags}}

# List case pairs whose server output already matches the committed references (candidates to flip to live ssr/ssr_dev)
ssr-flip-scan:
    cargo run -q -p compiler_tests --bin ssr_flip_scan

# Build WASM and serve the playground
playground:
    wasm-pack build --target web ./crates/wasm_compiler -d ../../playground/compiler
    cd playground && python3 -m http.server 8080

# Build the debug addon, wire it into the local package, and run the JS smoke test
npm-smoke:
    npm run --prefix packages/svelte-rs2 build
    node packages/svelte-rs2/scripts/smoke.mjs

# Build production-like local npm tarballs for testing in a consumer app
npm-build:
    npm run --prefix packages/svelte-rs2 build:release
    node_modules/.bin/napi create-npm-dirs --package-json-path packages/svelte-rs2/package.json --npm-dir packages/svelte-rs2/npm
    node_modules/.bin/napi artifacts --package-json-path packages/svelte-rs2/package.json --output-dir packages/svelte-rs2/compiler/native --npm-dir packages/svelte-rs2/npm
    npm pack ./packages/svelte-rs2 --silent

# Build the native addon and stage it into the local dev path of the main package
build-native:
    npm run --prefix packages/svelte-rs2 build:release

# Parity-sweep a directory: our compiler vs svelte/compiler across client+server × dev+prod, always dry-run. Flags: --mode=auto|runes|legacy --chunk=N (default auto) --print-diffs --out=<file>
sweep-run pathname *flags:
    cargo run --profile sweep -p sweep -- {{pathname}} {{flags}}
