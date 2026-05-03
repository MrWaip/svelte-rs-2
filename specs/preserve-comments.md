# preserveComments

## Current state
- **Working**: 7/7 use cases
- **Tests**: 9/9 green
- Last updated: 2026-05-03

## Source

- User request: `/audit preserveComments`
- Reference option: `compilerOptions.preserveComments` (default `false`)

## How It Works

`preserveComments` is a compile-time option (only on `compilerOptions`, not on `<svelte:options>`). When `false` (default), HTML comments in component templates are stripped before any DOM/SSR output is produced. When `true`, the original comment text is emitted into the runtime template as `<!--{data}-->`, becoming a real DOM `Comment` node at runtime — which means the codegen sibling/anchor walk must treat preserved comments as real DOM siblings.

The `svelte-ignore` directive on a comment is consumed by the analyzer regardless of `preserveComments`. The `@component` and `@hmr:keep` comments are also independent of this flag — they are tooling/HMR concerns, but for the *output template* they are governed by `preserveComments` like any other comment.

`Comment` AST nodes carry a `data` slice which is the inner text between `<!--` and `-->`. Anchor comments (`<!>`) emitted by block lowering are unrelated to user comments and remain regardless of the flag.

## Syntax variants

```svelte
<!-- single line -->
<!--
  multi
  line
-->
<!---->
<!-- svelte-ignore a11y_autofocus -->
<!-- @component
description
-->
<div><!-- inside element --></div>
{#if cond}<!-- inside block -->{/if}
<svg><!-- inside svg --></svg>
```

## Use cases

- [x] `Node::Comment::data()` accessor returns inner comment text (strips `<!--`/`-->`); `value()` retained as raw-slice form for callers that need the full span.
- [x] `preserve_comments` threaded from `CompileOptions` into `AnalyzeOptions::preserve_comments`, stored on `AnalysisData::script.preserve_comments`, exposed via `CodegenView::preserve_comments()` and `FragmentCtx::preserve_comments`.
- [x] When `preserve_comments=true`, `prepare.rs` does not hoist `Node::Comment` (`HoistedKind::Comment` guarded by the flag); Comment nodes flow into the regular child stream as `Child::Comment(String)` (test: `preserve_comments_basic`).
- [x] Preserved comments emit as `template.push_comment(Some(data))` so the static template HTML contains `<!--{data}-->`. Single-Comment fragments take the `$.comment()` runtime helper path via `emit_static_comment_anchor` (test: `preserve_comments_basic`, `preserve_comments_only_child`).
- [x] Sibling/anchor walk: in `process_children.rs`, `Child::Comment` is treated as a static template child when inside a `FragmentAnchor::Child` parent and no later child needs an anchor; otherwise it gets a `var node = ...` extraction via `flush_sibling_var` so subsequent dynamic children compute the right `$.sibling(node, n)` (test: `preserve_comments_between_elements`).
- [x] Preserved comments inside blocks (`{#if}` consequent/alternate, `{#each}` body, `{#snippet}`, `{#await}`, `{#key}`) emit into the block's inner fragment with the same logic; block-anchor `<!>` for the block itself (`push_comment(None)`) remains untouched (test: `preserve_comments_in_block`).
- [x] `<!-- svelte-ignore … -->` and `<!-- @component … -->` retain analyzer consumption for warnings while also being emitted into the runtime template when `preserve_comments=true` (test: `preserve_comments_svelte_ignore`).

## Out of scope

- SSR / server codegen for preserved comments — no `svelte_codegen_server` crate exists yet.
- `<svelte:options preserveComments={…} />` — reference compiler does not parse `preserveComments` from `<svelte:options>`, only from `compilerOptions`.
- HMR `@hmr:keep` / `@hmr:keep-all` runtime semantics beyond literal preservation.
- Validation of malformed comments (`-->` inside data, unclosed comments) — orthogonal to preservation.

## Reference

### Svelte
- `reference/compiler/types/index.d.ts` (option type)
- `reference/compiler/validate-options.js` (option validation)
- `reference/compiler/phases/1-parse/state/element.js` (Comment AST node creation, leadingComments for script/style)
- `reference/compiler/phases/3-transform/utils.js` `clean_nodes` (filter when `!preserve_comments`)
- `reference/compiler/phases/3-transform/client/visitors/Comment.js` (push_comment(data))
- `reference/compiler/phases/3-transform/client/visitors/Fragment.js` (passes preserveComments to clean_nodes)
- `reference/compiler/phases/3-transform/client/visitors/RegularElement.js` (passes preserveComments to clean_nodes for children)
- `reference/compiler/phases/3-transform/client/transform-template/template.js` `push_comment` (emits `<!--data-->` in HTML, `[\`// data\`]` in tree)

### Our code
- `crates/svelte_compiler/src/options.rs` `CompileOptions::preserve_comments`
- `crates/svelte_compiler/src/lib.rs` (option resolution; missing `resolved_preserve_comments_option` plumbing into `AnalyzeOptions`)
- `crates/svelte_analyze/src/lib.rs` `AnalyzeOptions` (no `preserve_comments` field yet)
- `crates/svelte_analyze/src/types/data/analysis.rs` (no `preserve_comments` storage)
- `crates/svelte_analyze/src/types/data/codegen_view.rs` (no `preserve_comments()` accessor)
- `crates/svelte_ast/src/lib.rs` `Comment` and `Comment::value()` (returns full `<!--data-->` slice; needs inner accessor)
- `crates/svelte_parser/src/lib.rs` (Comment node creation)
- `crates/svelte_codegen_client/src/codegen/fragment/prepare.rs` (`HoistedKind::Comment` always filters)
- `crates/svelte_codegen_client/src/codegen/fragment/process_children.rs` (no Comment branch)
- `crates/svelte_codegen_client/src/codegen/data_structures/template.rs` `Template::push_comment` (already supports `Some(data)` payload)

## Test cases

- [x] `preserve_comments_basic`
- [x] `preserve_comments_only_child`
- [x] `preserve_comments_between_elements`
- [x] `preserve_comments_in_block`
- [x] `preserve_comments_svelte_ignore`
- [x] `preserve_comments_only_in_block`
- [x] `preserve_comments_consecutive`
- [x] `preserve_comments_empty`
- [x] `preserve_comments_in_each`
