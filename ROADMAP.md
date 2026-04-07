# Roadmap: Svelte 5 Client Compiler in Rust

Scope: client-side compilation only (no SSR, no legacy mode).
Details per feature live in `specs/` — run `/audit <feature>` to generate or update a spec.

---

## Runes & Script

- [x] `$state` / `$state.raw` — [spec](specs/state-rune.md)
- [x] `$derived` / `$derived.by` — [spec](specs/derived-state.md)
- [x] `$props` / `$bindable` / `$props.id` — [spec](specs/props-bindable.md)
- [x] `$effect` / `$effect.pre` — [spec](specs/effect-runes.md)
- [x] `$inspect` / `$inspect.trace`  — [spec](specs/inspect-runes.md)
- [x] `$host` — [spec](specs/host-rune.md)
- [x] `$store` subscriptions — [spec](specs/store-subscriptions.md)
- [x] Destructuring & class fields — [spec](specs/destructuring-class-fields.md)
- [x] `<script module>` — [spec](specs/script-module.md)

## Template

- [ ] Element — [spec](specs/element.md)
- [ ] `<Component>` / component — [spec](specs/component-node.md)
- [x] `{#if}` / `{:else}` — [spec](specs/if-block.md)
- [x] `{#each}` — [spec](specs/each-block.md)
- [x] `{#await}` — [spec](specs/await-block.md)
- [x] `{#key}` — [spec](specs/key-block.md)
- [x] `{#snippet}` / `{@render}` — [spec](specs/snippet-block.md)
- [ ] `{@html}` — [spec](specs/html-tag.md)
- [x] `{@const}` — [spec](specs/const-tag.md)
- [x] `{@debug}` — [spec](specs/debug-tag.md)
- [x] Text / ExpressionTag — [spec](specs/text-expression-tag.md)
- [ ] Experimental async — [spec](specs/experimental-async.md)

## Attributes & Spreads

- Shared spec for this bucket: [specs/attributes-spreads.md](specs/attributes-spreads.md)

- [ ] Static & dynamic attributes
- [ ] `style:prop` / `class:name` / `class` object/array
- [ ] Spread attributes
- [ ] Element attribute edge cases
- [ ] Form element special handling

## Events

- [ ] Svelte 5 event attributes — [spec](specs/events.md)
- [ ] Event delegation — [spec](specs/events.md)
- [ ] Event modifiers (capture, passive) — [spec](specs/events.md)
- [ ] `on:event` legacy — [spec](specs/events.md)

## Bindings

- Shared spec for all `bind:*` items: [specs/bind-directives.md](specs/bind-directives.md)
- [x] `bind:value` / `bind:checked` / `bind:group` / `bind:files`
- [x] `bind:innerHTML` / `bind:innerText` / `bind:textContent`
- [x] `bind:clientWidth` / `bind:clientHeight` / `bind:offsetWidth` / `bind:offsetHeight`
- [x] `bind:this`
- [x] Media bindings
- [x] `bind:focused`

## Directives

- [ ] `use:action` — [spec](specs/use-action.md)
- [ ] `transition:` / `in:` / `out:` — [spec](specs/transitions.md)
- [ ] `animate:` — [spec](specs/animate.md)
- [ ] `{@attach}` — [spec](specs/attach-tag.md)

## Special Elements

- [ ] [`<svelte:options>`](specs/svelte-options.md)
- [ ] `<svelte:head>` / `<title>` — [spec](specs/svelte-head-title.md)
- [ ] `<svelte:window>` / `<svelte:document>` / `<svelte:body>` — [spec](specs/svelte-window-document-body.md)
- [ ] `<svelte:element>` — [spec](specs/svelte-element.md)
- [ ] `<svelte:boundary>` ([spec](./specs/svelte-boundary.md))

## CSS

- Shared spec for all CSS items: [specs/css-pipeline.md](specs/css-pipeline.md)
- [ ] CSS scoping pipeline (parse → hash → analyze → prune → transform) — [spec](specs/css-pipeline.md)
- [ ] CSS custom properties on elements & components — [spec](specs/css-pipeline.md)
- [ ] Nested `<style>` elements (unscoped, global rules)

## Source Maps

- Shared spec for this bucket: [specs/source-maps.md](specs/source-maps.md)
- [ ] JS source maps
- [ ] CSS source maps
- [ ] Preprocessor / upstream map composition
- [ ] Source map validation fixtures

## Validation & Diagnostics

- [ ] Diagnostic infrastructure — [spec](specs/diagnostics-infrastructure.md)
- [ ] Rune argument & placement validation
- [ ] Element & directive validation

## A11y Warnings

- [ ] A11y warnings — [spec](specs/a11y-warnings.md)

## Modules `.svelte.js` / `.svelte.ts`

- [ ] `.svelte.js` / `.svelte.ts`

## Custom Elements

- [ ] Custom Elements

## Compiler Infrastructure

- [ ] `discloseVersion` option
- [ ] `preserveComments` option
- [ ] HMR

## Legacy Svelte 4

- [ ] `<slot>` + `let:`
- [ ] `<svelte:component>` / `<svelte:self>` / `<svelte:fragment>` 
- [ ] `export let` props
- [ ] `$:` reactive assignments
- [ ] `$$props` / `$$restProps` / `$$slots`
- [ ] `beforeUpdate` / `afterUpdate`
