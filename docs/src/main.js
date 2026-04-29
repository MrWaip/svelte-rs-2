import { example, moduleExample, benchmarkExample } from "../example.js";
import { createStore, loadOptions, loadTheme } from "./state.js";
import {
    createSourceEditor,
    setMode as setEditorMode,
    setTheme as setEditorTheme,
    publishLintDiagnostics,
} from "./editor.js";
import { createDiff, setOriginal, setModified, applyDiffTheme } from "./diff.js";
import { loadWasmCompiler, isWasmReady, compileRust, compileSvelte, warmup } from "./compilers.js";
import { normalizeAll, renderPanel, toLintDiagnostics } from "./diagnostics.js";
import {
    bindModeTabs,
    bindMobileTabs,
    bindThemeToggle,
    bindSettings,
    bindDiagnosticsToggle,
    bindBenchmark,
    bindResizer,
    bindMobileActions,
    setStatus,
    setParity,
    setPerf,
    setSpeedup,
    setRecompileState,
} from "./ui.js";

const app = document.querySelector(".app");
const sourceEl = document.getElementById("source-editor");
const diffEl = document.getElementById("diff-editor");

const store = createStore({
    mode: "component",
    theme: loadTheme(),
    options: loadOptions(),
    source: example,
});

app.dataset.theme = store.get().theme;

let sourceView = null;
let diffView = null;
let compileTimer = null;

const wasmReady = await loadWasmCompiler();
if (!wasmReady) {
    setStatus(app, "wasm unavailable", "muted");
}

sourceView = createSourceEditor({
    parent: sourceEl,
    doc: store.get().source,
    mode: store.get().mode,
    theme: store.get().theme,
    onChange: (doc) => {
        if (store.get().source === doc) return;
        store.set({ source: doc });
    },
});

diffView = createDiff({
    parent: diffEl,
    original: "",
    modified: "",
    theme: store.get().theme,
});

bindModeTabs(app, store);
bindMobileTabs(app, store);
bindThemeToggle(app, store);
bindSettings(app, store);
bindDiagnosticsToggle(app);
bindResizer(app);
bindBenchmark(app, () => {
    const next = store.get().mode === "module" ? moduleExample : benchmarkExample;
    sourceView.dispatch({
        changes: { from: 0, to: sourceView.state.doc.length, insert: next },
        selection: { anchor: 0 },
    });
    runCompile();
});
bindMobileActions(app, {
    onRecompile: () => runCompile(),
    onShare: async () => {
        try {
            await navigator.clipboard.writeText(window.location.href);
        } catch { /* ignore */ }
    },
});

let prev = store.get();
store.subscribe((s) => {
    const previous = prev;
    prev = s;
    if (s.mode !== previous.mode) {
        setEditorMode(sourceView, s.mode);
        if (s.source === previous.source) {
            const next = s.mode === "module" ? moduleExample : example;
            sourceView.dispatch({
                changes: { from: 0, to: sourceView.state.doc.length, insert: next },
            });
        }
    }
    if (s.theme !== previous.theme) {
        setEditorTheme([sourceView], s.theme);
        applyDiffTheme(diffView, s.theme);
    }
    if (
        s.source !== previous.source
        || s.mode !== previous.mode
        || s.options !== previous.options
    ) {
        scheduleCompile();
    }
});

setStatus(app, wasmReady ? "warming up" : "wasm unavailable", wasmReady ? "warming" : "muted");

(async () => {
    if (wasmReady) {
        await warmup({
            onProgress: (done, total) => {
                setStatus(app, `warming ${done}/${total}`, "warming");
            },
        });
    }
    setStatus(app, "ready", "muted");
    runCompile();
})();

function scheduleCompile() {
    clearTimeout(compileTimer);
    compileTimer = setTimeout(runCompile, 120);
}

function runCompile() {
    const { source, mode, options } = store.get();
    const rust = isWasmReady() ? compileRust(source, mode, options) : null;
    const svelte = compileSvelte(source, mode, options);

    setOriginal(diffView, svelte.js);
    setModified(diffView, rust ? rust.js : "// WASM compiler not available");

    setPerf(app, "svelte", svelte.ok ? svelte.ms : null);
    setPerf(app, "rust", rust && rust.ok ? rust.ms : null);

    if (svelte.ok && rust && rust.ok) {
        const ratio = svelte.ms / Math.max(rust.ms, 0.001);
        setSpeedup(app, ratio);
    } else {
        setSpeedup(app, null);
    }

    if (!svelte.ok || (rust && !rust.ok)) {
        setParity(app, "error", "error");
        setRecompileState(app, "error");
    } else if (rust && rust.js === svelte.js) {
        setParity(app, "match", "parity");
        setRecompileState(app, "ok");
    } else {
        setParity(app, "diverged", "diverged");
        setRecompileState(app, "ok");
    }

    const { rust: rustDiags, svelte: svelteDiags } = normalizeAll(
        rust?.diagnostics ?? [],
        svelte.diagnostics ?? [],
    );

    publishLintDiagnostics(sourceView, toLintDiagnostics(sourceView, [...rustDiags, ...svelteDiags]));

    renderPanel({
        root: app.querySelector("[data-diag-list]"),
        rust: rustDiags,
        svelte: svelteDiags,
        sourceView,
        mobileBadge: app.querySelector("[data-mobile-diag-count]"),
        panelEl: app.querySelector(".diagnostics-panel"),
        rustBadge: app.querySelector("[data-rust-count]"),
        svelteBadge: app.querySelector("[data-svelte-count]"),
        cleanBadge: app.querySelector("[data-clean]"),
    });
}

