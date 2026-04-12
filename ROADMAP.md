# Roadmap: Svelte 5 Client Compiler in Rust

Scope: client-side compilation only (no SSR, no legacy mode).
Details per feature live in `specs/` — run `/audit <feature>` to generate or update a spec.

## Delivery Clusters

These roadmap buckets are grouped into larger delivery clusters so work can be prioritized as coherent milestones instead of isolated feature checkboxes.

### 1. Core Client Compiler Parity

- Runes & Script
- Template
- Attributes & Spreads
- Events
- Bindings
- Directives
- Special Elements
- CSS
- Custom Elements

Goal: match `svelte/compiler` for normal client-side component compilation before expanding the integration surface.

### 2. Validation Parity

- Validation & Diagnostics
- A11y Warnings

Goal: align warnings, errors, spans, and severity once the owning runtime/compiler behavior is in place.

### 3. Optimizations

- Benchmark-driven hot-path work in `svelte_component_semantics`, `svelte_analyze`, `svelte_transform`, and `svelte_codegen_client`
- Allocation cleanup, redundant traversal removal, and data-flow tightening in already-stable client paths
- Large-scale compile throughput and memory work after the main client parity slices stop churning

Goal: optimize the stabilized client compiler before expanding the tooling surface or starting SSR work.

### 4. Tooling And Integration Surface

- Source Maps
- Modules `.svelte.js` / `.svelte.ts`
- Compiler Infrastructure

Goal: stabilize the compiler contract for downstream tools after core compile parity is solid.

### 5. Triage And Deferred Compatibility

- Unknown / Triage
- Legacy Svelte 4

Goal: keep new repros tracked without diluting the main Svelte 5 client-compiler push.

SSR remains a separate future track. This roadmap stays client-only until these clusters are in better shape.

---

## Runes & Script

- [x] `$state` / `$state.raw` — [spec](specs/state-rune.md)
- [x] `$derived` / `$derived.by` — [spec](specs/derived-state.md)
- [ ] `$props` / `$bindable` / `$props.id` — [spec](specs/props-bindable.md)
- [x] `$effect` / `$effect.pre` — [spec](specs/effect-runes.md)
- [x] `$inspect` / `$inspect.trace`  — [spec](specs/inspect-runes.md)
- [ ] `$host` — [spec](specs/host-rune.md)
- [x] `$store` subscriptions — [spec](specs/store-subscriptions.md)
- [x] Destructuring & class fields — [spec](specs/destructuring-class-fields.md)
- [x] `<script module>` — [spec](specs/script-module.md)

## Template

- [ ] Element — [spec](specs/element.md)
- [ ] `<Component>` / component — [spec](specs/component-node.md)
- [x] `{#if}` / `{:else}` — [spec](specs/if-block.md)
- [ ] `{#each}` — [spec](specs/each-block.md)
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

- [x] Static & dynamic attributes
- [x] `style:prop` / `class:name` / `class` object/array
- [x] Spread attributes
- [x] Element attribute edge cases
- [x] Form element special handling

## Events

- [x] Svelte 5 event attributes — [spec](specs/events.md)
- [x] Event delegation — [spec](specs/events.md)
- [x] Event modifiers (capture, passive) — [spec](specs/events.md)
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

- [x] `use:action` — [spec](specs/use-action.md)
- [x] `transition:` / `in:` / `out:` — [spec](specs/transitions.md)
- [x] `animate:` — [spec](specs/animate.md)
- [x] `{@attach}` — [spec](specs/attach-tag.md)

## Special Elements

- [ ] [`<svelte:options>`](specs/svelte-options.md)
- [ ] `<svelte:head>` / `<title>` — [spec](specs/svelte-head-title.md)
- [ ] `<svelte:window>` / `<svelte:document>` / `<svelte:body>` — [spec](specs/svelte-window-document-body.md)
- [ ] `<svelte:element>` — [spec](specs/svelte-element.md)
- [ ] `<svelte:boundary>` ([spec](./specs/svelte-boundary.md))

## CSS

- Shared spec for all CSS items: [specs/css-pipeline.md](specs/css-pipeline.md)
- [x] CSS scoping pipeline (parse → hash → analyze → prune → transform) — [spec](specs/css-pipeline.md)
- [x] CSS custom properties on elements & components — [spec](specs/css-pipeline.md)
- [x] Nested `<style>` elements (unscoped, global rules)

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

- [x] A11y warnings — [spec](specs/a11y-warnings.md)

## Modules `.svelte.js` / `.svelte.ts`

- [ ] `.svelte.js` / `.svelte.ts`

## Custom Elements

- [ ] Custom Elements — [spec](specs/custom-elements.md)

## Compiler Infrastructure

- [x] Filename-derived component naming — [spec](specs/filename-derived-component-name.md)
- [ ] `discloseVersion` option
- [ ] `preserveComments` option
- [ ] HMR

## Unknown / Triage

- [ ] Unknown or not-yet-owned repros — [spec](specs/unknown.md)

## Legacy Svelte 4

- [ ] `<slot>` + `let:` + `<svelte:fragment>` + `slot attribute` — [spec](specs/legacy-slots.md)
- [ ] `<svelte:self>` — [spec](specs/svelte-self.md)
- [ ] `<svelte:component>` — [spec](specs/svelte-component.md)
- [ ] `export let` props — [spec](specs/legacy-export-let.md)
- [ ] `$:` reactive assignments — [spec](specs/legacy-reactive-assignments.md)
- [ ] `$$props` / `$$restProps` / `$$slots`
- [ ] `beforeUpdate` / `afterUpdate`
