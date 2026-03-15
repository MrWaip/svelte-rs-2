import { compile } from "svelte/compiler";
import { readFileSync } from "node:fs";
import { execSync } from "node:child_process";
import { resolve, dirname } from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = dirname(fileURLToPath(import.meta.url));
const projectRoot = resolve(__dirname, "../..");

const file = process.argv[2] || "tasks/benchmark/benches/compiler/big_v1.svelte";
const iterations = parseInt(process.argv[3] || "100", 10);
const filePath = resolve(projectRoot, file);

const source = readFileSync(filePath, "utf8");
const lines = source.split("\n").length;
const sizeKb = (Buffer.byteLength(source) / 1024).toFixed(1);

console.log(`\nFile: ${file} (${lines} lines, ${sizeKb} KB)`);
console.log(`Iterations: ${iterations}\n`);

// --- JS benchmark ---
const compileOpts = {
  discloseVersion: false,
  dev: false,
  name: "App",
  modernAst: true,
  runes: true,
};

// Warmup
for (let i = 0; i < 5; i++) {
  compile(source, compileOpts);
}

// Measure
const jsTimes = [];
for (let i = 0; i < iterations; i++) {
  const start = performance.now();
  compile(source, compileOpts);
  const elapsed = (performance.now() - start) * 1000; // to microseconds
  jsTimes.push(elapsed);
}
jsTimes.sort((a, b) => a - b);

const jsMedian = jsTimes[Math.floor(jsTimes.length / 2)];

// --- Rust benchmark ---
const benchCli = resolve(projectRoot, "target/release/bench_cli");
const rustOut = execSync(`${benchCli} ${filePath} ${iterations}`, {
  encoding: "utf8",
}).trim();
const rustStats = JSON.parse(rustOut);
const rustMedian = rustStats.median_us;

// --- Results ---
const speedup = (jsMedian / rustMedian).toFixed(1);

const fmt = (us) => {
  if (us >= 1000) return (us / 1000).toFixed(2) + " ms";
  return us.toFixed(0) + " μs";
};

console.log("┌────────────┬──────────────┐");
console.log("│ Compiler   │ Median       │");
console.log("├────────────┼──────────────┤");
console.log(`│ Svelte JS  │ ${fmt(jsMedian).padStart(12)} │`);
console.log(`│ Rust       │ ${fmt(rustMedian).padStart(12)} │`);
console.log("├────────────┼──────────────┤");
console.log(`│ Speedup    │ ${(speedup + "x").padStart(12)} │`);
console.log("└────────────┴──────────────┘");
