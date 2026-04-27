import { MergeView } from "@codemirror/merge";
import { EditorState, Compartment } from "@codemirror/state";
import { EditorView, lineNumbers } from "@codemirror/view";
import { foldGutter, bracketMatching } from "@codemirror/language";
import { javascript } from "@codemirror/lang-javascript";
import { highlightForTheme } from "./editor.js";

const themeCompartment = new Compartment();

function baseExtensions(theme) {
    return [
        lineNumbers(),
        foldGutter(),
        bracketMatching(),
        javascript(),
        themeCompartment.of(highlightForTheme(theme)),
        EditorView.theme({
            "&": { height: "100%" },
            ".cm-scroller": { overflow: "auto" },
            ".cm-content": { padding: "12px 0" },
            ".cm-line": { padding: "0 12px" },
        }),
        EditorView.editable.of(false),
        EditorState.readOnly.of(true),
        EditorView.lineWrapping,
    ];
}

export function createDiff({ parent, original = "", modified = "", theme }) {
    const view = new MergeView({
        parent,
        a: {
            doc: original,
            extensions: baseExtensions(theme),
        },
        b: {
            doc: modified,
            extensions: baseExtensions(theme),
        },
        orientation: "a-b",
        revertControls: false,
        highlightChanges: true,
        gutter: true,
    });
    syncScroll(view.a.scrollDOM, view.b.scrollDOM);
    return view;
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

export function setOriginal(diff, doc) {
    const view = diff.a;
    view.dispatch({
        changes: { from: 0, to: view.state.doc.length, insert: doc },
    });
}

export function setModified(diff, doc) {
    const view = diff.b;
    view.dispatch({
        changes: { from: 0, to: view.state.doc.length, insert: doc },
    });
}

export function applyDiffTheme(diff, theme) {
    for (const view of [diff.a, diff.b]) {
        view.dispatch({
            effects: themeCompartment.reconfigure(highlightForTheme(theme)),
        });
    }
}
