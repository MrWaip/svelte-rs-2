import { compile as svelteCompile, compileModule as svelteCompileModule } from "svelte/compiler";
import { benchmarkExample } from "../example.js";

let wasm = null;
let compiler = null;

const SMALL_SAMPLES = [
    `<script>let count = $state(0);</script><button onclick={() => count++}>{count}</button>`,
    `<script>let { name = "world" } = $props();</script><h1>Hello {name}!</h1>`,
    `<script>let items = $state([1,2,3]);</script>{#each items as item}<li>{item}</li>{/each}`,
];

const WARMUP_MODULE = `let count = $state(0); export const doubled = $derived(count * 2);`;
const HEAVY_ITERATIONS = 5;

export async function loadWasmCompiler() {
    try {
        wasm = await import("../compiler/wasm_compiler.js");
        await wasm.default();
        compiler = new wasm.WasmCompiler();
        return true;
    } catch (e) {
        console.warn("WASM compiler not available:", e);
        return false;
    }
}

export function isWasmReady() {
    return compiler != null;
}

export function formatJs(code) {
    if (!compiler) return code;
    try {
        return compiler.format(code);
    } catch {
        return code;
    }
}

export function formatCss(code) {
    if (!compiler) return code;
    try {
        return compiler.format_css(code);
    } catch {
        return code;
    }
}

export async function warmup({ onProgress } = {}) {
    const total = SMALL_SAMPLES.length * 2 + HEAVY_ITERATIONS * 2 + 2;
    let done = 0;
    const tick = () => {
        done++;
        onProgress?.(done, total);
    };

    for (const sample of SMALL_SAMPLES) {
        try {
            svelteCompile(sample, { runes: true, name: "Warm", dev: false, modernAst: true });
        } catch {}
        tick();
        if (compiler) {
            try {
                compiler.compile(sample, { runes: true, name: "Warm" });
            } catch {}
        }
        tick();
        await new Promise(r => setTimeout(r, 0));
    }

    for (let i = 0; i < HEAVY_ITERATIONS; i++) {
        try {
            svelteCompile(benchmarkExample, { runes: true, name: "Bench", dev: false, modernAst: true });
        } catch {}
        tick();
        if (compiler) {
            try {
                compiler.compile(benchmarkExample, { runes: true, name: "Bench" });
            } catch {}
        }
        tick();
        await new Promise(r => setTimeout(r, 0));
    }

    try {
        svelteCompileModule(WARMUP_MODULE, { dev: false });
    } catch {}
    tick();
    if (compiler) {
        try {
            compiler.compile_module(WARMUP_MODULE, { dev: false });
        } catch {}
    }
    tick();
}

function buildRustOptions(mode, options) {
    if (mode === "module") {
        return {
            dev: options.dev,
            generate: options.generate,
        };
    }
    return {
        dev: options.dev,
        runes: options.runes,
        discloseVersion: options.discloseVersion,
        hmr: options.hmr,
        customElement: options.customElement,
        preserveComments: options.preserveComments,
        preserveWhitespace: options.preserveWhitespace,
        generate: options.generate,
        css: options.css,
        name: options.name || "App",
        experimental: { async: !!options.experimentalAsync },
    };
}

function buildSvelteOptions(mode, options) {
    if (mode === "module") {
        return {
            dev: options.dev,
            generate: options.generate,
        };
    }
    return {
        dev: options.dev,
        runes: options.runes,
        discloseVersion: options.discloseVersion,
        hmr: options.hmr,
        customElement: options.customElement,
        preserveComments: options.preserveComments,
        preserveWhitespace: options.preserveWhitespace,
        generate: options.generate,
        css: options.css,
        name: options.name || "App",
        modernAst: true,
        experimental: { async: !!options.experimentalAsync },
    };
}

function timeIt(fn) {
    const start = performance.now();
    let result, error;
    try {
        result = fn();
    } catch (e) {
        error = e;
    }
    return { result, error, ms: performance.now() - start };
}

export function compileRust(source, mode, options) {
    if (!compiler) {
        return {
            ok: false,
            error: new Error("WASM compiler not loaded"),
            js: "// WASM compiler not available\n// Run: wasm-pack build --target web ./crates/wasm_compiler -d ../../docs/compiler",
            diagnostics: [],
            ms: 0,
        };
    }
    const opts = buildRustOptions(mode, options);
    const fn = mode === "module"
        ? () => compiler.compile_module(source, opts)
        : () => compiler.compile(source, opts);
    const { result, error, ms } = timeIt(fn);

    if (error) {
        return { ok: false, error, js: "// compilation failed", diagnostics: [], ms };
    }
    const css = result?.css ?? null;
    const js = result?.js ?? "// no output";
    const formatted = joinJsCss(formatJs(js), css);
    return {
        ok: true,
        js: formatted,
        css,
        diagnostics: result?.diagnostics ?? [],
        ms,
    };
}

export function compileSvelte(source, mode, options) {
    const opts = buildSvelteOptions(mode, options);
    const fn = mode === "module"
        ? () => svelteCompileModule(source, opts)
        : () => svelteCompile(source, opts);
    const { result, error, ms } = timeIt(fn);

    if (error) {
        return {
            ok: false,
            error,
            js: typeof error?.message === "string" ? `/* compilation error */\n// ${error.message}` : "// compilation failed",
            diagnostics: error?.code
                ? [{ code: error.code, message: error.message, severity: "Error", start: error.start, end: error.end }]
                : [],
            ms,
        };
    }
    let code = result?.js?.code ?? "";
    if (mode === "module") {
        code = code.replace(/^\/\*.*?generated by Svelte.*?\*\/\n/, "");
    }
    code = formatJs(code);
    const css = result?.css?.code ?? null;
    return {
        ok: true,
        js: joinJsCss(code, css),
        css,
        diagnostics: result?.warnings ?? [],
        ms,
    };
}

function joinJsCss(js, css) {
    if (!css) return js;
    const cleanedCss = formatCss(css)
        .replace(/\/\*[\s\S]*?\*\//g, "")
        .replace(/\n{3,}/g, "\n\n")
        .trim();
    if (!cleanedCss) return js;
    return `${js}\n\n/* ---- CSS ---- */\n${cleanedCss}`;
}
