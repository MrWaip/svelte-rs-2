import { jumpTo } from "./editor.js";

function normalizeDiag(d, source) {
    const start = d.start ?? {
        line: (d.start_line ?? 0) + 1,
        column: d.start_col ?? 0,
    };
    const end = d.end ?? {
        line: (d.end_line ?? start.line - 1) + 1,
        column: d.end_col ?? start.column,
    };
    const severity = (d.severity ?? "warning").toString().toLowerCase();
    const sev = severity.includes("err") ? "error" : severity.includes("info") ? "info" : "warning";
    return {
        source,
        code: d.code ?? "",
        message: d.message ?? "",
        severity: sev,
        line: start.line,
        column: start.column,
        endLine: end.line,
        endColumn: end.column,
    };
}

export function toLintDiagnostics(view, diagnostics) {
    if (!view) return [];
    const doc = view.state.doc;
    return diagnostics.map((d) => {
        const safeLine = Math.max(1, Math.min(d.line, doc.lines));
        const lineInfo = doc.line(safeLine);
        const fromCol = Math.max(0, Math.min(d.column, lineInfo.length));
        const safeEndLine = Math.max(safeLine, Math.min(d.endLine, doc.lines));
        const endLineInfo = doc.line(safeEndLine);
        const toCol = Math.max(fromCol, Math.min(d.endColumn, endLineInfo.length));
        return {
            from: lineInfo.from + fromCol,
            to: endLineInfo.from + toCol,
            severity: d.severity,
            message: d.code ? `${d.code}: ${d.message}` : d.message,
            source: d.source,
        };
    });
}

export function normalizeAll(rustRaw, svelteRaw) {
    return {
        rust: rustRaw.map((d) => normalizeDiag(d, "rust")),
        svelte: svelteRaw.map((d) => normalizeDiag(d, "svelte")),
    };
}

export function renderPanel({ root, rust, svelte, sourceView, mobileBadge, panelEl, rustBadge, svelteBadge, cleanBadge }) {
    const total = rust.length + svelte.length;
    const all = [...rust, ...svelte].sort((a, b) => {
        if (a.line !== b.line) return a.line - b.line;
        return a.column - b.column;
    });

    if (total === 0) {
        root.innerHTML = `<p class="diag-empty">No diagnostics — clean compile.</p>`;
        cleanBadge.hidden = false;
        rustBadge.hidden = true;
        svelteBadge.hidden = true;
        mobileBadge.hidden = true;
        panelEl.dataset.collapsed = "true";
        return;
    }

    cleanBadge.hidden = true;
    rustBadge.hidden = rust.length === 0;
    svelteBadge.hidden = svelte.length === 0;
    rustBadge.textContent = `${rust.length} rust`;
    svelteBadge.textContent = `${svelte.length} svelte`;
    mobileBadge.hidden = false;
    mobileBadge.textContent = String(total);

    root.innerHTML = "";
    const frag = document.createDocumentFragment();
    for (const d of all) {
        const row = document.createElement("button");
        row.className = "diag-row";
        row.dataset.source = d.source;
        row.innerHTML = `
            <span class="diag-source">${d.source}</span>
            <span class="diag-severity" data-sev="${d.severity}" title="${d.severity}"></span>
            <span class="diag-message"><span class="diag-code">${escapeHtml(d.code)}</span>${escapeHtml(d.message)}</span>
            <span class="diag-loc">${d.line}:${d.column}</span>
        `;
        row.addEventListener("click", () => jumpTo(sourceView, d.line, d.column));
        frag.appendChild(row);
    }
    root.appendChild(frag);
}

function escapeHtml(s) {
    return String(s)
        .replace(/&/g, "&amp;")
        .replace(/</g, "&lt;")
        .replace(/>/g, "&gt;")
        .replace(/"/g, "&quot;");
}
