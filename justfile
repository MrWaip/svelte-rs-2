generate:
    cd tasks/generate_test_cases && npm install --silent
    cargo run -p generate_test_cases

# Build WASM and serve the playground
playground:
    wasm-pack build --target web ./crates/wasm_compiler -d ../../docs/compiler
    cd docs && python3 -m http.server 8080
