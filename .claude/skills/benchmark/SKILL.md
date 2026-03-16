---
name: benchmark
description: Run wall-clock Rust vs JS benchmark comparison
user_invocable: true
argument: (optional) benchmark file name, defaults to latest
---

# Benchmark: Rust vs JS comparison

## Step 1: Run benchmark

If $ARGUMENTS is provided, use it as the benchmark file name:
```
just compare-benchmark $ARGUMENTS
```

If no argument, run with default:
```
just compare-benchmark
```

## Step 2: Interpret results

Parse the output and report:
- Rust compilation time
- Svelte JS compilation time
- Speedup ratio (Rust vs JS)
- Whether performance is within expected range

## Step 3: Check benchmark coverage

Read the benchmark `.svelte` file used. Compare the syntax it contains against features listed in `ROADMAP.md`. If recently ported features are missing from the benchmark file, suggest creating a new versioned benchmark:

```
just generate-benchmark big_vN <chunks>
```

where N is the next version number.

## Rules

- Never modify existing benchmark files — they are versioned for CodSpeed history
- Report raw numbers, not just ratios
- If the benchmark fails to compile, report the error and suggest checking which features the benchmark uses
