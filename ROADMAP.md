# Roadmap: Svelte 5 Client Compiler in Rust

## Runes & Script

- [x] `$state` / `$state.raw` — [spec](specs/state-rune.md)
- [x] `$derived` / `$derived.by` — [spec](specs/derived-state.md)
- [x] `$props` / `$bindable` / `$props.id` — [spec](specs/props-bindable.md)
- [x] `$effect` / `$effect.pre` — [spec](specs/effect-runes.md)
- [x] `$inspect` / `$inspect.trace` — [spec](specs/inspect-runes.md)
- [x] `$host` — [spec](specs/host-rune.md)
- [x] `$store` subscriptions — [spec](specs/store-subscriptions.md)
- [x] Destructuring & class fields — [spec](specs/destructuring-class-fields.md)
- [x] `<script module>` — [spec](specs/script-module.md)

## Template

- [x] Element — [spec](specs/element.md)
- [x] `<Component>` / component — [spec](specs/component-node.md)
- [x] `{#if}` / `{:else}` — [spec](specs/if-block.md)
- [x] `{#each}` — [spec](specs/each-block.md)
- [x] `{#await}` — [spec](specs/await-block.md)
- [x] `{#key}` — [spec](specs/key-block.md)
- [x] `{#snippet}` / `{@render}` — [spec](specs/snippet-block.md)
- [x] `{@html}` — [spec](specs/html-tag.md)
- [x] `{@const}` — [spec](specs/const-tag.md)
- [x] `{@debug}` — [spec](specs/debug-tag.md)
- [x] Text / ExpressionTag — [spec](specs/text-expression-tag.md)
- [x] Experimental async — [spec](specs/experimental-async.md)

## Attributes & Spreads

- Shared spec for this bucket: [specs/attributes-spreads.md](specs/attributes-spreads.md)

- [x] Static & dynamic attributes
- [x] `style:prop` / `class:name` / `class` object/array
- [x] Spread attributes
- [x] Element attribute edge cases
- [x] Form element special handling

## Events

- [x] Svelte 5 event attributes — [spec](specs/events.md)
- [x] Event delegation — [spec](specs/events.md)
- [x] Event modifiers (capture, passive) — [spec](specs/events.md)
- [x] `on:event` legacy — [spec](specs/events.md)

## Bindings

- Shared spec for all `bind:*` items: [specs/bind-directives.md](specs/bind-directives.md)
- [x] `bind:value` / `bind:checked` / `bind:group` / `bind:files`
- [x] `bind:innerHTML` / `bind:innerText` / `bind:textContent`
- [x] `bind:clientWidth` / `bind:clientHeight` / `bind:offsetWidth` / `bind:offsetHeight`
- [x] `bind:this`
- [x] Media bindings
- [x] `bind:focused`

## Directives

- [x] `use:action` — [spec](specs/use-action.md)
- [x] `transition:` / `in:` / `out:` — [spec](specs/transitions.md)
- [x] `animate:` — [spec](specs/animate.md)
- [x] `{@attach}` — [spec](specs/attach-tag.md)

## Special Elements

- [x] [`<svelte:options>`](specs/svelte-options.md)
- [x] `<svelte:head>` / `<title>` — [spec](specs/svelte-head-title.md)
- [x] `<svelte:window>` / `<svelte:document>` / `<svelte:body>` — [spec](specs/svelte-window-document-body.md)
- [x] `<svelte:element>` — [spec](specs/svelte-element.md)
- [x] `<svelte:boundary>` ([spec](./specs/svelte-boundary.md))

## CSS

- [x] CSS analyze / transform / codegen [spec](specs/css-pipeline.md)

## Validation & Diagnostics

- [x] Diagnostic infrastructure — [spec](specs/diagnostics-infrastructure.md)

## A11y Warnings

- [x] A11y warnings — [spec](specs/a11y-warnings.md)

## Modules `.svelte.js` / `.svelte.ts`

- [ ] `.svelte.js` / `.svelte.ts`

## Custom Elements

- [x] Custom Elements — [spec](specs/custom-elements.md)

## Typescript

- [x] TypeScript script stripping — [spec](specs/typescript-script-stripping.md)

## Legacy Svelte 4

- [x] Legacy reactivity system: `let var = ''` — [spec](specs/legacy-reactivity-system.md)
- [x] `<slot>` + `let:` + `<svelte:fragment>` + `slot attribute` + `$$slots` — [spec](specs/legacy-slots.md)
- [ ] `<svelte:self>` — [spec](specs/svelte-self.md)
- [x] `<svelte:component>` — [spec](specs/svelte-component.md)
- [x] `export let` props / `$$props` / `$$restProps` — [spec](specs/legacy-export-let.md)
- [x] `$:` reactive assignments — [spec](specs/legacy-reactive-assignments.md)
- [x] `beforeUpdate` / `afterUpdate` — [spec](specs/before-update-after-update.md)

## Compiler Infrastructure

- [x] Filename-derived component naming — [spec](specs/filename-derived-component-name.md)
- [ ] `discloseVersion` option
- [ ] `preserveComments` option
- [ ] HMR

## Source Maps

- Shared spec for this bucket: [specs/source-maps.md](specs/source-maps.md)
- [ ] JS source maps
- [ ] CSS source maps
- [ ] Preprocessor / upstream map composition
- [ ] Source map validation fixtures

## Unknown / Triage

- [ ] Unknown or not-yet-owned repros — [spec](specs/unknown.md)
