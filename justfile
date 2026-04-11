# Generate expected JS output (case-svelte.js) for all compiler test cases
generate:
    cd tasks/generate_test_cases && npm install --silent
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

# Run parser tests
test-parser:
    cargo test -p svelte_parser

# Run analyzer tests
test-analyzer:
    cargo test -p svelte_analyze

# Generate benchmark .svelte file (usage: just generate-benchmark big_v2 50)
generate-benchmark name='big_v6' chunks='50':
    cargo run -p generate_benchmark -- {{name}} {{chunks}}

# Compare Rust vs JS compiler performance (wall-clock)
compare-benchmark file='tasks/benchmark/benches/compiler/big_v6.svelte':
    cargo build --release -p benchmark --bin bench_cli
    cd tasks/benchmark && npm install --silent
    node tasks/benchmark/compare.mjs {{file}}

# Dump OXC AST as JSON for a JS expression
dump-ast expr:
    cargo run -p svelte_parser --example dump_ast -- '{{expr}}'

# Build WASM and serve the playground
playground:
    wasm-pack build --target web ./crates/wasm_compiler -d ../../docs/compiler
    cd docs && python3 -m http.server 8080

# Build the debug addon, wire it into the local package, and run the JS smoke test
smoke-npm-package:
    cargo build -p napi_compiler
    node packages/svelte-rs2/scripts/prepare-local-main-package.mjs
    node packages/svelte-rs2/scripts/smoke.mjs

# Build production-like local npm tarballs for testing in a consumer app
local-pack:
    cargo build -p napi_compiler --release
    npm run --prefix packages/svelte-rs2 prepare-platform-package
    npm pack ./packages/svelte-rs2 --silent
    npm pack ./packages/svelte-rs2-linux-x64-gnu --silent
    npm pack ./packages/svelte-rs2-darwin-arm64 --silent
    npm pack ./packages/svelte-rs2-darwin-x64 --silent
