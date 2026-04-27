export function createStore(initial) {
    let state = initial;
    const subs = new Set();

    return {
        get: () => state,
        set(next) {
            const prev = state;
            state = typeof next === "function" ? next(state) : { ...state, ...next };
            for (const fn of subs) fn(state, prev);
        },
        subscribe(fn) {
            subs.add(fn);
            return () => subs.delete(fn);
        },
    };
}

export const DEFAULT_OPTIONS = {
    dev: false,
    runes: true,
    discloseVersion: false,
    hmr: false,
    customElement: false,
    preserveComments: false,
    preserveWhitespace: false,
    generate: "client",
    css: "external",
    name: "App",
};

const STORAGE_KEY = "playground:options:v1";

export function loadOptions() {
    try {
        const raw = localStorage.getItem(STORAGE_KEY);
        if (!raw) return { ...DEFAULT_OPTIONS };
        const parsed = JSON.parse(raw);
        return { ...DEFAULT_OPTIONS, ...parsed };
    } catch {
        return { ...DEFAULT_OPTIONS };
    }
}

export function saveOptions(options) {
    try {
        localStorage.setItem(STORAGE_KEY, JSON.stringify(options));
    } catch {
        /* quota / private mode — ignore */
    }
}

export function loadTheme() {
    const stored = localStorage.getItem("playground:theme");
    if (stored === "dark" || stored === "light") return stored;
    return window.matchMedia("(prefers-color-scheme: light)").matches ? "light" : "dark";
}

export function saveTheme(theme) {
    localStorage.setItem("playground:theme", theme);
}
