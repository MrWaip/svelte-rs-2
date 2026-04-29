import { saveOptions, saveTheme, DEFAULT_OPTIONS } from "./state.js";

const SECTION_KEYS = {
    output: ["generate", "css", "dev", "hmr", "discloseVersion"],
    component: ["runes", "customElement", "preserveComments", "preserveWhitespace", "name"],
};

const PRESETS = {
    default: { ...DEFAULT_OPTIONS },
    legacy: { ...DEFAULT_OPTIONS, runes: false, hmr: false },
    ssr: { ...DEFAULT_OPTIONS, generate: "server" },
};

function detectPreset(options) {
    for (const [id, preset] of Object.entries(PRESETS)) {
        if (Object.keys(preset).every((k) => options[k] === preset[k])) return id;
    }
    return "custom";
}

function countOverrides(options, keys = Object.keys(DEFAULT_OPTIONS)) {
    return keys.filter((k) => options[k] !== DEFAULT_OPTIONS[k]).length;
}

export function bindModeTabs(app, store) {
    const buttons = app.querySelectorAll(".mode-tabs button[data-mode]");
    buttons.forEach((btn) => {
        btn.addEventListener("click", () => {
            store.set({ mode: btn.dataset.mode });
        });
    });
    store.subscribe((s) => {
        buttons.forEach((b) => b.classList.toggle("active", b.dataset.mode === s.mode));
        app.querySelectorAll("[data-component-only]").forEach((el) => {
            el.style.display = s.mode === "module" ? "none" : "";
        });
    });
}

export function bindMobileTabs(app, store) {
    const buttons = app.querySelectorAll(".mobile-tabs button[data-tab]");
    buttons.forEach((btn) => {
        btn.addEventListener("click", () => {
            app.dataset.mobileTab = btn.dataset.tab;
            buttons.forEach((b) => b.classList.toggle("active", b === btn));
            if (btn.dataset.tab === "diagnostics") {
                const panel = app.querySelector(".diagnostics-panel");
                panel.dataset.collapsed = "false";
            }
            window.dispatchEvent(new Event("resize"));
        });
    });
}

export function bindThemeToggle(app, store) {
    const btn = app.querySelector('[data-action="theme"]');
    btn.addEventListener("click", () => {
        const next = store.get().theme === "dark" ? "light" : "dark";
        store.set({ theme: next });
        saveTheme(next);
    });
    store.subscribe((s) => {
        app.dataset.theme = s.theme;
    });
}

export function bindSettings(app, store) {
    const drawer = app.querySelector(".settings-drawer");
    const open = () => drawer.dataset.open = "true";
    const close = () => drawer.dataset.open = "false";

    app.querySelectorAll('[data-action="settings"]').forEach((b) => b.addEventListener("click", open));
    app.querySelectorAll('[data-action="close-settings"]').forEach((b) => b.addEventListener("click", close));

    document.addEventListener("keydown", (e) => {
        if (e.key === "Escape") close();
    });

    const inputs = drawer.querySelectorAll("[data-opt]");
    const presetChips = drawer.querySelectorAll(".preset-chip");
    const sectionEls = drawer.querySelectorAll("[data-section]");
    const overridesMeta = drawer.querySelector("[data-overrides-count]");

    const sync = (options) => {
        inputs.forEach((input) => {
            const key = input.dataset.opt;
            const value = options[key];
            if (input.type === "checkbox") {
                input.checked = !!value;
            } else {
                input.value = value ?? "";
            }
        });

        const activePreset = detectPreset(options);
        presetChips.forEach((chip) => {
            const id = chip.dataset.preset;
            chip.toggleAttribute("data-active", id === activePreset);
        });

        sectionEls.forEach((section) => {
            const keys = SECTION_KEYS[section.dataset.section] ?? [];
            const mod = countOverrides(options, keys);
            const countEl = section.querySelector("[data-section-count]");
            const modEl = section.querySelector("[data-section-mod]");
            if (countEl) countEl.textContent = `${keys.length} fields`;
            if (modEl) {
                if (mod > 0) {
                    modEl.textContent = `${mod} mod`;
                    modEl.hidden = false;
                } else {
                    modEl.hidden = true;
                }
            }
        });

        if (overridesMeta) {
            const total = countOverrides(options);
            overridesMeta.textContent = total === 0
                ? "matches defaults"
                : `${total} override${total === 1 ? "" : "s"}`;
        }
    };

    inputs.forEach((input) => {
        input.addEventListener("change", () => {
            const key = input.dataset.opt;
            const next = { ...store.get().options };
            if (input.type === "checkbox") {
                next[key] = input.checked;
            } else {
                next[key] = input.value;
            }
            store.set({ options: next });
            saveOptions(next);
        });
    });

    presetChips.forEach((chip) => {
        if (chip.hasAttribute("data-readonly")) return;
        chip.addEventListener("click", () => {
            const preset = PRESETS[chip.dataset.preset];
            if (!preset) return;
            const next = { ...preset };
            store.set({ options: next });
            saveOptions(next);
        });
    });

    app.querySelector('[data-action="reset-options"]').addEventListener("click", () => {
        const next = { ...DEFAULT_OPTIONS };
        store.set({ options: next });
        saveOptions(next);
    });

    sync(store.get().options);
    store.subscribe((s, prev) => {
        if (s.options !== prev.options) sync(s.options);
    });
}

export function bindMobileActions(app, { onRecompile, onShare }) {
    const recompile = app.querySelector('[data-action="recompile"]');
    const share = app.querySelector('[data-action="share"]');
    if (recompile && onRecompile) recompile.addEventListener("click", onRecompile);
    if (share && onShare) share.addEventListener("click", onShare);
}

export function setRecompileState(app, state) {
    const btn = app.querySelector('[data-action="recompile"]');
    const label = app.querySelector("[data-recompile-label]");
    if (!btn) return;
    if (state === "error") {
        btn.dataset.state = "error";
        if (label) label.textContent = "Fix errors";
    } else {
        delete btn.dataset.state;
        if (label) label.textContent = "Recompile";
    }
}

export function bindDiagnosticsToggle(app) {
    const panel = app.querySelector(".diagnostics-panel");
    const toggle = app.querySelector('[data-action="toggle-diagnostics"]');
    toggle.addEventListener("click", () => {
        const collapsed = panel.dataset.collapsed === "true";
        panel.dataset.collapsed = collapsed ? "false" : "true";
        toggle.setAttribute("aria-expanded", String(collapsed));
        window.dispatchEvent(new Event("resize"));
    });
}

export function bindBenchmark(app, callback) {
    app.querySelector('[data-action="benchmark"]').addEventListener("click", callback);
}

const SPLIT_KEY = "playground:split";

export function bindResizer(app) {
    const workspace = app.querySelector(".workspace");
    const resizer = app.querySelector("[data-resizer]");
    if (!resizer) return;

    const stored = parseFloat(localStorage.getItem(SPLIT_KEY));
    if (isFinite(stored) && stored >= 15 && stored <= 85) {
        workspace.style.setProperty("--split", `${stored}%`);
    }

    let dragging = false;

    const onMove = (e) => {
        if (!dragging) return;
        const rect = workspace.getBoundingClientRect();
        const clientX = e.touches ? e.touches[0].clientX : e.clientX;
        const pct = ((clientX - rect.left) / rect.width) * 100;
        const clamped = Math.max(15, Math.min(85, pct));
        workspace.style.setProperty("--split", `${clamped}%`);
    };

    const onUp = () => {
        if (!dragging) return;
        dragging = false;
        resizer.classList.remove("dragging");
        document.body.classList.remove("resizing");
        const split = workspace.style.getPropertyValue("--split");
        const num = parseFloat(split);
        if (isFinite(num)) localStorage.setItem(SPLIT_KEY, String(num));
        document.removeEventListener("mousemove", onMove);
        document.removeEventListener("mouseup", onUp);
        document.removeEventListener("touchmove", onMove);
        document.removeEventListener("touchend", onUp);
    };

    const onDown = (e) => {
        dragging = true;
        resizer.classList.add("dragging");
        document.body.classList.add("resizing");
        document.addEventListener("mousemove", onMove);
        document.addEventListener("mouseup", onUp);
        document.addEventListener("touchmove", onMove, { passive: true });
        document.addEventListener("touchend", onUp);
        e.preventDefault();
    };

    resizer.addEventListener("mousedown", onDown);
    resizer.addEventListener("touchstart", onDown, { passive: false });
    resizer.addEventListener("dblclick", () => {
        workspace.style.setProperty("--split", "50%");
        localStorage.setItem(SPLIT_KEY, "50");
    });
    resizer.addEventListener("keydown", (e) => {
        const current = parseFloat(workspace.style.getPropertyValue("--split")) || 50;
        let next = current;
        if (e.key === "ArrowLeft") next = Math.max(15, current - 2);
        else if (e.key === "ArrowRight") next = Math.min(85, current + 2);
        else return;
        workspace.style.setProperty("--split", `${next}%`);
        localStorage.setItem(SPLIT_KEY, String(next));
        e.preventDefault();
    });
}

export function setStatus(app, text, kind = "muted") {
    const chip = app.querySelector("[data-status]");
    chip.className = `chip ${kind}`;
    if (kind === "warming") {
        chip.innerHTML = `<span class="spinner" aria-hidden="true"></span><span>${text}</span>`;
    } else {
        chip.textContent = text;
    }
}

export function setParity(app, state, label) {
    const chip = app.querySelector(".chip[data-parity]");
    if (chip) {
        chip.dataset.state = state;
        chip.textContent = label;
    }
    const out = app.querySelector('[data-panel="output"]');
    if (out) out.dataset.parity = state;
}

export function setPerf(app, kind, ms) {
    const chip = app.querySelector(`[data-${kind}-perf]`);
    if (!chip) return;
    chip.textContent = ms == null ? `${kind} —` : `${kind} ${formatMs(ms)}`;
}

export function setSpeedup(app, ratio) {
    const chip = app.querySelector("[data-speedup]");
    if (!chip) return;
    if (ratio == null || !isFinite(ratio) || ratio <= 0) {
        chip.hidden = true;
        chip.dataset.state = "";
        return;
    }
    chip.hidden = false;
    if (ratio >= 1) {
        chip.dataset.state = "faster";
        chip.textContent = `${ratio.toFixed(1)}× faster`;
    } else {
        chip.dataset.state = "slower";
        chip.textContent = `${(1 / ratio).toFixed(1)}× slower`;
    }
}

function formatMs(ms) {
    if (ms < 1) return `${(ms * 1000).toFixed(0)}µs`;
    if (ms < 100) return `${ms.toFixed(2)}ms`;
    return `${ms.toFixed(0)}ms`;
}
