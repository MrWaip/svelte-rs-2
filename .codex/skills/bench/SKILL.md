---
name: bench
description: Run and interpret Rust-versus-JS compiler benchmarks. Use when the user asks to benchmark the compiler, compare Rust and Svelte JS timings, check performance regressions, or decide whether the benchmark corpus needs a new version.
---

# Benchmark

## 1) Run the benchmark

Use:

```bash
just compare-benchmark [<benchmark-file>]
```

If no file is provided, use the default benchmark target.

## 2) Report raw results

Always report:

- Rust compilation time
- Svelte JS compilation time
- speedup or slowdown ratio
- whether the result looks normal or suspicious

Prefer raw numbers plus the ratio, not ratio alone.

## 3) Check benchmark coverage

Read the benchmark `.svelte` file that was used and compare its syntax coverage against recently ported features and `ROADMAP.md`.

If the benchmark misses important newly supported constructs, recommend generating a new versioned benchmark with:

```bash
just generate-benchmark big_vN <chunks>
```

## Rules

- never modify existing benchmark files in place
- if the benchmark fails, report the error and likely unsupported feature area
