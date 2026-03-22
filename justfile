# Generate expected JS output (case-svelte.js) for all compiler test cases
generate:
    cd tasks/generate_test_cases && npm install --silent
    cargo run -p generate_test_cases

# Run all compiler integration tests
test-compiler:
    cargo test -p compiler_tests --test compiler_tests_v3

# Run a single compiler test case
test-case name:
    cargo test -p compiler_tests --test compiler_tests_v3 {{name}}

# Run a single compiler test case with output
test-case-verbose name:
    cargo test -p compiler_tests --test compiler_tests_v3 {{name}} -- --nocapture

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

# Build WASM and serve the playground
playground:
    wasm-pack build --target web ./crates/wasm_compiler -d ../../docs/compiler
    cd docs && python3 -m http.server 8080
