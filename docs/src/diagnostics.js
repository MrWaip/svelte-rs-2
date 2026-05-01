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
    const docLen = doc.length;
    return diagnostics.map((d) => {
        const safeLine = Math.max(1, Math.min(d.line, doc.lines));
        const lineInfo = doc.line(safeLine);
        const fromCol = Math.max(0, Math.min(d.column, lineInfo.length));
        const safeEndLine = Math.max(safeLine, Math.min(d.endLine, doc.lines));
        const endLineInfo = doc.line(safeEndLine);
        const toCol = Math.max(fromCol, Math.min(d.endColumn, endLineInfo.length));
        let from = Math.min(lineInfo.from + fromCol, docLen);
        let to = Math.min(endLineInfo.from + toCol, docLen);
        if (to < from) to = from;
        if (from === to && from < docLen) to = from + 1;
        return {
            from,
            to,
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

let activeFilter = "all";
let lastSnapshot = { all: [], rust: [], svelte: [], sourceView: null, root: null };

export function renderPanel({
    root, rust, svelte, sourceView, mobileBadge, panelEl,
    rustBadge, svelteBadge, cleanBadge, errorBadge, warnBadge,
    filterGroup, copyBtn,
}) {
    const total = rust.length + svelte.length;
    const all = [...rust, ...svelte].sort((a, b) => {
        if (a.line !== b.line) return a.line - b.line;
        return a.column - b.column;
    });

    const errors = all.filter((d) => d.severity === "error").length;
    const warnings = all.filter((d) => d.severity === "warning").length;

    lastSnapshot = { all, rust, svelte, sourceView, root };

    if (total === 0) {
        root.innerHTML = `<p class="diag-empty">No diagnostics — clean compile.</p>`;
        if (cleanBadge) cleanBadge.hidden = false;
        if (errorBadge) errorBadge.hidden = true;
        if (warnBadge) warnBadge.hidden = true;
        if (rustBadge) rustBadge.hidden = true;
        if (svelteBadge) svelteBadge.hidden = true;
        if (mobileBadge) mobileBadge.hidden = true;
        if (copyBtn) copyBtn.hidden = true;
        if (filterGroup) {
            filterGroup.querySelectorAll("[data-filter]").forEach((b) => {
                if (b.dataset.filter !== "all") b.hidden = true;
            });
            const allBtn = filterGroup.querySelector('[data-filter="all"]');
            if (allBtn) allBtn.querySelector("[data-count]").textContent = "0";
        }
        panelEl.dataset.collapsed = "true";
        return;
    }

    if (cleanBadge) cleanBadge.hidden = true;
    if (errorBadge) {
        errorBadge.hidden = errors === 0;
        errorBadge.textContent = `${errors} ${errors === 1 ? "error" : "errors"}`;
    }
    if (warnBadge) {
        warnBadge.hidden = warnings === 0;
        warnBadge.textContent = `${warnings} ${warnings === 1 ? "warning" : "warnings"}`;
    }
    if (rustBadge) {
        rustBadge.hidden = rust.length === 0;
        rustBadge.textContent = `${rust.length} rust`;
    }
    if (svelteBadge) {
        svelteBadge.hidden = svelte.length === 0;
        svelteBadge.textContent = `${svelte.length} svelte`;
    }
    if (mobileBadge) {
        mobileBadge.hidden = false;
        mobileBadge.textContent = String(total);
    }
    if (copyBtn) copyBtn.hidden = false;

    if (filterGroup) {
        const counts = { all: total, rust: rust.length, svelte: svelte.length };
        filterGroup.querySelectorAll("[data-filter]").forEach((btn) => {
            const id = btn.dataset.filter;
            const count = counts[id] ?? 0;
            btn.querySelector("[data-count]").textContent = String(count);
            btn.hidden = id !== "all" && count === 0;
        });
    }

    paintList();
}

function paintList() {
    const { all, sourceView, root } = lastSnapshot;
    if (!root) return;
    const filtered = activeFilter === "all" ? all : all.filter((d) => d.source === activeFilter);
    root.innerHTML = "";
    if (filtered.length === 0) {
        root.innerHTML = `<p class="diag-empty">No matches for filter.</p>`;
        return;
    }
    const frag = document.createDocumentFragment();
    for (const d of filtered) {
        const row = document.createElement("button");
        row.className = "diag-row";
        row.dataset.source = d.source;
        row.dataset.sev = d.severity;
        row.innerHTML = `
            <span class="diag-source">${d.source}</span>
            <span class="diag-severity" data-sev="${d.severity}" title="${d.severity}"></span>
            <span class="diag-body-text">
                ${d.code ? `<code class="diag-code">${escapeHtml(d.code)}</code>` : ""}
                <span class="diag-message">${escapeHtml(d.message)}</span>
            </span>
            <span class="diag-loc">${d.line}:${d.column}</span>
            <span class="diag-jump">Jump</span>
        `;
        row.addEventListener("click", () => jumpTo(sourceView, d.line, d.column));
        frag.appendChild(row);
    }
    root.appendChild(frag);
}

export function bindDiagnosticsFilters(filterGroup) {
    if (!filterGroup) return;
    filterGroup.addEventListener("click", (e) => {
        const btn = e.target.closest("[data-filter]");
        if (!btn) return;
        const id = btn.dataset.filter;
        if (id === activeFilter) return;
        activeFilter = id;
        filterGroup.querySelectorAll("[data-filter]").forEach((b) => {
            b.classList.toggle("active", b.dataset.filter === id);
        });
        paintList();
    });
}

export function bindCopyDiagnostics(btn) {
    if (!btn) return;
    btn.addEventListener("click", async () => {
        const { all } = lastSnapshot;
        if (!all.length) return;
        const text = all
            .map((d) => `[${d.source}/${d.severity}] ${d.code ? d.code + ": " : ""}${d.message} (${d.line}:${d.column})`)
            .join("\n");
        try {
            await navigator.clipboard.writeText(text);
            btn.dataset.state = "copied";
            const original = btn.textContent;
            btn.textContent = "Copied";
            setTimeout(() => {
                delete btn.dataset.state;
                btn.textContent = original;
            }, 1500);
        } catch {
            btn.dataset.state = "error";
            setTimeout(() => delete btn.dataset.state, 1500);
        }
    });
}

function escapeHtml(s) {
    return String(s)
        .replace(/&/g, "&amp;")
        .replace(/</g, "&lt;")
        .replace(/>/g, "&gt;")
        .replace(/"/g, "&quot;");
}
