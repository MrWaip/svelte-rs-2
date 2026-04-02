---
name: rust-lsp-workflow
description: Start and use the `rust-lsp` MCP server correctly for this repository. Use when navigating Rust code, looking up definitions or references, inspecting types with hover, checking diagnostics, renaming symbols, or when broad LSP symbol search returns nothing and you need the right fallback for workspace or dependency code.
---

# Rust Lsp Workflow

Use `rust-lsp` before text search for Rust code semantics. Start the server for the current workspace first; otherwise symbol and definition queries can silently return empty results.

## 1. Start Or Reuse The Server

Check server status before semantic queries:

```text
rust-lsp.lsp_server_status({"server_id":"rust"})
```

If no matching server is running for this repository, start one with the repo root as `workspace_root`:

```text
rust-lsp.lsp_start_server({
  "server_id":"rust",
  "workspace_root":"/Users/klobkov/personal-code/svelte-rs-2"
})
```

Use `server_id: "rust"`, not `rust-analyzer`.

If there are multiple running Rust servers, prefer the one whose `workspace_root` is the repository root instead of a nested crate.

## 2. Use LSP For Repository Rust Code

For symbols defined in this repository, use this order:

1. `workspace_symbols` when you know the symbol name.
2. `document_symbols` when you already know the file.
3. `definition`, `references`, `hover`, `rename`, or `diagnostics` once you have an exact position.

Do not conclude that a symbol does not exist just because `workspace_symbols` returned nothing. Empty results can mean the wrong server is running, the wrong workspace is selected, or symbol search is incomplete.

When broad symbol search fails, fall back to file discovery with `rg`, then return to LSP on the exact file and position.

## 3. Handle Dependency Types Differently

For types from external crates such as `oxc_semantic::Scoping`, do not rely on workspace-wide symbol search first. Those types may live outside the repository workspace.

Use this order instead:

1. Locate the source file in Cargo registry or git checkouts with `rg`.
2. Use `lsp_document_symbols` on the exact dependency file to get the type outline and method names.
3. Read the file directly to confirm the real type and nearby methods.
4. If needed, use `definition`, `hover`, or other exact-position LSP queries after the file is known.

For OXC visitor or semantic APIs, load `$oxc-analyze-api` when exact method signatures matter.

## 4. Failure Modes

- `servers: []`: start the Rust server before more LSP queries.
- `SERVER_NOT_FOUND`: use `server_id: "rust"` and confirm the `rust-lsp` MCP server is configured.
- Empty `workspace_symbols`: verify the running server matches the repository root, then fall back to `rg` plus exact-position LSP.
- Nested-crate server selected: prefer the server whose `workspace_root` is `/Users/klobkov/personal-code/svelte-rs-2`.

## 5. Working Rule

Use `rust-lsp` for semantics, not as a replacement for all search. Use `rg` for plain text, regexes, docs, generated files, and dependency file discovery. Return to LSP as soon as the exact Rust file and position are known.
