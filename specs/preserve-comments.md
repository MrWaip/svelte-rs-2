# preserveComments

## Current state
- **Working**: 0/7 use cases
- **Tests**: 0/5 green
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

- [ ] `Node::Comment::value()` returns the inner comment text (currently returns the full `<!--data-->` slice including delimiters); add an inner-data accessor that strips `<!--`/`-->` and trailing whitespace. (parser/AST, quick fix)
- [ ] Thread `preserve_comments` from `CompileOptions` into `AnalyzeOptions`, store on `AnalysisData`, and expose via `CodegenView::preserve_comments()`. (analyze, quick fix)
- [ ] When `preserve_comments=true`, stop classifying `Node::Comment` as `HoistedKind::Comment` in `crates/svelte_codegen_client/src/codegen/fragment/prepare.rs` so that comment children flow into the regular child stream. (codegen, moderate)
- [ ] Emit preserved comments as `template.push_comment(Some(data))` in fragment processing so the static template HTML contains `<!--{data}-->`; verify whitespace trimming around comments matches reference. (codegen, moderate, test: `preserve_comments_basic`, `preserve_comments_only_child`)
- [ ] Sibling/anchor walk must count preserved comments as real DOM siblings: `make_sibling_expr` and ghost-skip arithmetic must not skip them in `process_children.rs`. (codegen, moderate, test: `preserve_comments_between_elements`)
- [ ] Preserved comments inside blocks (`{#if}`, `{#each}`, `{#await}`, `{#key}`, `{#snippet}`) emit into the block's child template, separately from the block-anchor `<!>` push. (codegen, moderate, test: `preserve_comments_in_block`)
- [ ] `<!-- svelte-ignore … -->` and `<!-- @component … -->` are still emitted into output when `preserve_comments=true` (analyzer continues to consume them for warnings independently). (validate, quick fix, test: `preserve_comments_svelte_ignore`)

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

- [ ] `preserve_comments_basic`
- [ ] `preserve_comments_only_child`
- [ ] `preserve_comments_between_elements`
- [ ] `preserve_comments_in_block`
- [ ] `preserve_comments_svelte_ignore`
