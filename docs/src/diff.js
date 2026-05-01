import { MergeView, unifiedMergeView } from "@codemirror/merge";
import { EditorState, Compartment } from "@codemirror/state";
import { EditorView, lineNumbers } from "@codemirror/view";
import { foldGutter, bracketMatching } from "@codemirror/language";
import { javascript } from "@codemirror/lang-javascript";
import { highlightForTheme } from "./editor.js";

const splitThemeCompartment = new Compartment();
const unifiedThemeCompartment = new Compartment();

const MOBILE_QUERY = "(max-width: 768px)";

export function isMobileLayout() {
    return window.matchMedia(MOBILE_QUERY).matches;
}

export function watchMobileLayout(callback) {
    const mql = window.matchMedia(MOBILE_QUERY);
    const handler = (e) => callback(e.matches);
    mql.addEventListener("change", handler);
    return () => mql.removeEventListener("change", handler);
}

function diffEditorTheme(theme) {
    return EditorView.theme({
        "&": { height: "100%", backgroundColor: "transparent" },
        ".cm-scroller": { overflow: "auto" },
        ".cm-content": { padding: "12px 0" },
        ".cm-line": { padding: "0 12px" },
        ".cm-gutters": {
            backgroundColor: "transparent",
            border: "none",
            color: "var(--fg-tertiary)",
        },
        ".cm-gutterElement": {
            color: "var(--fg-tertiary)",
        },
        ".cm-activeLineGutter": {
            backgroundColor: "transparent",
            color: "var(--fg-secondary)",
        },
        ".cm-foldGutter .cm-gutterElement": {
            color: "var(--fg-tertiary)",
        },
    }, { dark: theme === "dark" });
}

function splitExtensions(theme) {
    return [
        lineNumbers(),
        foldGutter(),
        bracketMatching(),
        javascript(),
        splitThemeCompartment.of(highlightForTheme(theme)),
        diffEditorTheme(theme),
        EditorView.editable.of(false),
        EditorState.readOnly.of(true),
        EditorView.lineWrapping,
    ];
}

function unifiedExtensions(theme, original) {
    return [
        unifiedMergeView({ original, mergeControls: false }),
        lineNumbers(),
        foldGutter(),
        bracketMatching(),
        javascript(),
        unifiedThemeCompartment.of(highlightForTheme(theme)),
        diffEditorTheme(theme),
        EditorView.editable.of(false),
        EditorState.readOnly.of(true),
        EditorView.lineWrapping,
    ];
}

export function createDiff({ parent, original = "", modified = "", theme, layout }) {
    if (layout === "unified") return createUnifiedDiff({ parent, original, modified, theme });
    return createSplitDiff({ parent, original, modified, theme });
}

function createSplitDiff({ parent, original, modified, theme }) {
    const view = new MergeView({
        parent,
        a: { doc: original, extensions: splitExtensions(theme) },
        b: { doc: modified, extensions: splitExtensions(theme) },
        orientation: "a-b",
        revertControls: false,
        highlightChanges: true,
        gutter: true,
    });
    syncScroll(view.a.scrollDOM, view.b.scrollDOM);
    return { kind: "split", view, a: view.a, b: view.b, parent, theme };
}

function createUnifiedDiff({ parent, original, modified, theme }) {
    const obj = { kind: "unified", parent, theme, original, modified, view: null };
    obj.view = buildUnifiedView(obj);
    return obj;
}

function buildUnifiedView(obj) {
    return new EditorView({
        parent: obj.parent,
        state: EditorState.create({
            doc: obj.modified,
            extensions: unifiedExtensions(obj.theme, obj.original),
        }),
    });
}

function rebuildUnifiedView(obj) {
    obj.view.destroy();
    obj.parent.innerHTML = "";
    obj.view = buildUnifiedView(obj);
}

function syncScroll(left, right) {
    let suppress = 0;
    const link = (src, dst) => {
        src.addEventListener("scroll", () => {
            if (suppress > 0) {
                suppress--;
                return;
            }
            suppress++;
            dst.scrollTop = src.scrollTop;
            dst.scrollLeft = src.scrollLeft;
        }, { passive: true });
    };
    link(left, right);
    link(right, left);
}

export function setDiffDocs(diff, original, modified) {
    if (diff.kind === "split") {
        diff.view.destroy();
        const parent = diff.view.dom.parentNode || diff.parent;
        if (parent) parent.innerHTML = "";
        const rebuilt = createSplitDiff({ parent, original, modified, theme: diff.theme });
        diff.view = rebuilt.view;
        diff.a = rebuilt.a;
        diff.b = rebuilt.b;
        diff.parent = parent;
    } else {
        diff.original = original;
        diff.modified = modified;
        rebuildUnifiedView(diff);
    }
}


export function applyDiffTheme(diff, theme) {
    if (diff.kind === "split") {
        for (const v of [diff.a, diff.b]) {
            v.dispatch({
                effects: splitThemeCompartment.reconfigure(highlightForTheme(theme)),
            });
        }
    } else {
        diff.theme = theme;
        rebuildUnifiedView(diff);
    }
}

export function destroyDiff(diff) {
    if (diff.kind === "split") diff.view.destroy();
    else diff.view.destroy();
}
