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

# Dump OXC AST as JSON for a JS expression
dump-ast expr:
    cargo run -p svelte_parser --example dump_ast -- '{{expr}}'

# Build WASM and serve the playground
playground:
    wasm-pack build --target web ./crates/wasm_compiler -d ../../docs/compiler
    cd docs && python3 -m http.server 8080

# Build N-API compiler crate in debug mode
napi-build:
    cargo build -p napi_compiler

# Run JS facade smoke tests (builds debug addon + checks canary contract)
napi-smoke:
    node packages/svelte-rs2/scripts/smoke.mjs

# Build N-API compiler crate in release mode
napi-build-release:
    cargo build -p napi_compiler --release

# Copy current-platform release addon into platform npm package
napi-prepare-platform:
    npm run --prefix packages/svelte-rs2 prepare-platform-package

# Create npm tarballs for main package and current platform package
napi-pack:
    npm pack ./packages/svelte-rs2 --silent
    TARGET="$(node -p "`${process.platform}-${process.arch}`")"; \
    if [ "$TARGET" = "linux-x64" ]; then \
      npm pack ./packages/svelte-rs2-linux-x64-gnu --silent; \
    elif [ "$TARGET" = "darwin-arm64" ]; then \
      npm pack ./packages/svelte-rs2-darwin-arm64 --silent; \
    elif [ "$TARGET" = "darwin-x64" ]; then \
      npm pack ./packages/svelte-rs2-darwin-x64 --silent; \
    else \
      echo "Unsupported target for pack: $TARGET" >&2; \
      exit 1; \
    fi

# Publish current-platform package to npm (dry-run by default)
napi-publish-platform tag='canary' dry='true':
    TARGET="$(node -p "`${process.platform}-${process.arch}`")"; \
    if [ "$TARGET" = "linux-x64" ]; then \
      PKG="./packages/svelte-rs2-linux-x64-gnu"; \
    elif [ "$TARGET" = "darwin-arm64" ]; then \
      PKG="./packages/svelte-rs2-darwin-arm64"; \
    elif [ "$TARGET" = "darwin-x64" ]; then \
      PKG="./packages/svelte-rs2-darwin-x64"; \
    else \
      echo "Unsupported target for publish: $TARGET" >&2; \
      exit 1; \
    fi; \
    if [ "{{dry}}" = "true" ]; then \
      npm publish "$PKG" --tag {{tag}} --access public --dry-run; \
    else \
      npm publish "$PKG" --tag {{tag}} --access public; \
    fi

# Publish main facade package to npm (dry-run by default)
napi-publish-main tag='canary' dry='true':
    if [ "{{dry}}" = "true" ]; then \
      npm publish ./packages/svelte-rs2 --tag {{tag}} --access public --dry-run; \
    else \
      npm publish ./packages/svelte-rs2 --tag {{tag}} --access public; \
    fi

# Publish current-platform package first, then main facade (dry-run by default)
napi-publish-all tag='canary' dry='true':
    just napi-publish-platform {{tag}} {{dry}}
    just napi-publish-main {{tag}} {{dry}}
