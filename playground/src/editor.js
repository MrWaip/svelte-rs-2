import { EditorView, keymap, lineNumbers, highlightActiveLine, highlightActiveLineGutter } from "@codemirror/view";
import { EditorState, Compartment } from "@codemirror/state";
import { defaultKeymap, history, historyKeymap } from "@codemirror/commands";
import { syntaxHighlighting, HighlightStyle, bracketMatching, indentOnInput, foldKeymap, foldGutter } from "@codemirror/language";
import { searchKeymap } from "@codemirror/search";
import { closeBrackets, closeBracketsKeymap } from "@codemirror/autocomplete";
import { lintGutter, setDiagnostics } from "@codemirror/lint";
import { html } from "@codemirror/lang-html";
import { javascript } from "@codemirror/lang-javascript";
import { tags as t } from "@lezer/highlight";

const darkHighlight = HighlightStyle.define([
    { tag: t.comment, color: "#6b6b6b", fontStyle: "italic" },
    { tag: t.lineComment, color: "#6b6b6b", fontStyle: "italic" },
    { tag: t.blockComment, color: "#6b6b6b", fontStyle: "italic" },
    { tag: t.keyword, color: "#ff7849", fontWeight: "500" },
    { tag: t.controlKeyword, color: "#ff7849" },
    { tag: t.operatorKeyword, color: "#ff7849" },
    { tag: t.string, color: "#8fc8a3" },
    { tag: t.special(t.string), color: "#8fc8a3" },
    { tag: t.number, color: "#d4a268" },
    { tag: t.bool, color: "#d4a268" },
    { tag: t.null, color: "#d4a268" },
    { tag: t.atom, color: "#d4a268" },
    { tag: t.function(t.variableName), color: "#62a0ea" },
    { tag: t.function(t.propertyName), color: "#62a0ea" },
    { tag: t.variableName, color: "#e6e6e6" },
    { tag: t.propertyName, color: "#c8a8e9" },
    { tag: t.definition(t.variableName), color: "#e6e6e6" },
    { tag: t.tagName, color: "#ff7849" },
    { tag: t.attributeName, color: "#d4a268" },
    { tag: t.attributeValue, color: "#8fc8a3" },
    { tag: t.angleBracket, color: "#5a5a5a" },
    { tag: t.brace, color: "#a0a0a0" },
    { tag: t.bracket, color: "#a0a0a0" },
    { tag: t.paren, color: "#a0a0a0" },
    { tag: t.punctuation, color: "#a0a0a0" },
    { tag: t.operator, color: "#ff7849" },
    { tag: t.regexp, color: "#8fc8a3" },
    { tag: t.escape, color: "#d4a268" },
    { tag: t.typeName, color: "#62a0ea" },
    { tag: t.className, color: "#62a0ea" },
    { tag: t.namespace, color: "#62a0ea" },
    { tag: t.heading, color: "#ff7849", fontWeight: "700" },
    { tag: t.link, color: "#62a0ea", textDecoration: "underline" },
    { tag: t.invalid, color: "#ff5c5c" },
]);

const lightHighlight = HighlightStyle.define([
    { tag: t.comment, color: "#9aa0a6", fontStyle: "italic" },
    { tag: t.lineComment, color: "#9aa0a6", fontStyle: "italic" },
    { tag: t.blockComment, color: "#9aa0a6", fontStyle: "italic" },
    { tag: t.keyword, color: "#c0392b", fontWeight: "500" },
    { tag: t.controlKeyword, color: "#c0392b" },
    { tag: t.operatorKeyword, color: "#c0392b" },
    { tag: t.string, color: "#27875f" },
    { tag: t.special(t.string), color: "#27875f" },
    { tag: t.number, color: "#a6731f" },
    { tag: t.bool, color: "#a6731f" },
    { tag: t.null, color: "#a6731f" },
    { tag: t.atom, color: "#a6731f" },
    { tag: t.function(t.variableName), color: "#1e6fb8" },
    { tag: t.function(t.propertyName), color: "#1e6fb8" },
    { tag: t.variableName, color: "#1a1a1a" },
    { tag: t.propertyName, color: "#7a3aa1" },
    { tag: t.tagName, color: "#c0392b" },
    { tag: t.attributeName, color: "#a6731f" },
    { tag: t.attributeValue, color: "#27875f" },
    { tag: t.angleBracket, color: "#9a9a9a" },
    { tag: t.brace, color: "#5a5a5a" },
    { tag: t.bracket, color: "#5a5a5a" },
    { tag: t.paren, color: "#5a5a5a" },
    { tag: t.punctuation, color: "#5a5a5a" },
    { tag: t.operator, color: "#c0392b" },
    { tag: t.regexp, color: "#27875f" },
    { tag: t.typeName, color: "#1e6fb8" },
    { tag: t.className, color: "#1e6fb8" },
    { tag: t.invalid, color: "#d63031" },
]);

function baseTheme(theme) {
    return EditorView.theme({
        "&": {
            height: "100%",
            backgroundColor: "transparent",
        },
        ".cm-scroller": {
            overflow: "auto",
        },
        ".cm-content": {
            padding: "12px 0",
        },
        ".cm-line": {
            padding: "0 12px",
        },
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

export const themeCompartment = new Compartment();
export const langCompartment = new Compartment();
export const readOnlyCompartment = new Compartment();

export function langForMode(mode) {
    return mode === "module" ? javascript() : html({ matchClosingTags: true, autoCloseTags: true });
}

export function highlightForTheme(theme) {
    return syntaxHighlighting(theme === "light" ? lightHighlight : darkHighlight);
}

function commonExtensions(theme) {
    return [
        lineNumbers(),
        foldGutter(),
        history(),
        bracketMatching(),
        closeBrackets(),
        indentOnInput(),
        highlightActiveLine(),
        highlightActiveLineGutter(),
        lintGutter(),
        keymap.of([
            ...closeBracketsKeymap,
            ...defaultKeymap,
            ...searchKeymap,
            ...historyKeymap,
            ...foldKeymap,
        ]),
        baseTheme(theme),
        EditorView.lineWrapping,
    ];
}

export function createSourceEditor({ parent, doc, mode, theme, onChange }) {
    const view = new EditorView({
        parent,
        state: EditorState.create({
            doc,
            extensions: [
                ...commonExtensions(theme),
                langCompartment.of(langForMode(mode)),
                themeCompartment.of(highlightForTheme(theme)),
                EditorView.updateListener.of((update) => {
                    if (update.docChanged) onChange(view.state.doc.toString());
                }),
            ],
        }),
    });
    return view;
}

export function createReadOnlyEditor({ parent, doc, theme }) {
    return new EditorView({
        parent,
        state: EditorState.create({
            doc,
            extensions: [
                lineNumbers(),
                foldGutter(),
                bracketMatching(),
                javascript(),
                themeCompartment.of(highlightForTheme(theme)),
                baseTheme(theme),
                EditorView.editable.of(false),
                EditorState.readOnly.of(true),
                EditorView.lineWrapping,
            ],
        }),
    });
}

export function setMode(view, mode) {
    view.dispatch({
        effects: langCompartment.reconfigure(langForMode(mode)),
    });
}

export function setTheme(views, theme) {
    for (const view of views) {
        if (!view) continue;
        view.dispatch({
            effects: themeCompartment.reconfigure(highlightForTheme(theme)),
        });
    }
}

export function publishLintDiagnostics(view, diagnostics) {
    if (!view) return;
    view.dispatch(setDiagnostics(view.state, diagnostics));
}

export function jumpTo(view, line, column) {
    if (!view) return;
    const lineCount = view.state.doc.lines;
    const safeLine = Math.max(1, Math.min(line, lineCount));
    const lineInfo = view.state.doc.line(safeLine);
    const col = Math.max(0, Math.min(column, lineInfo.length));
    const pos = lineInfo.from + col;
    view.dispatch({
        selection: { anchor: pos },
        effects: EditorView.scrollIntoView(pos, { y: "center" }),
    });
    view.focus();
}
