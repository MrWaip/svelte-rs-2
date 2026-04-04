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

- [ ] `use:action`
- [ ] `transition:` / `in:` / `out:`
- [ ] `animate:`
- [ ] `{@attach}`

## Special Elements

- [ ] `<svelte:options>`
- [ ] `<svelte:head>` / `<title>`
- [ ] `<svelte:window>` / `<svelte:document>` / `<svelte:body>`
- [ ] `<svelte:element>`
- [ ] `<svelte:boundary>`

## CSS

- Shared spec for all CSS items: [specs/css-pipeline.md](specs/css-pipeline.md)
- [ ] CSS scoping pipeline (parse → hash → analyze → prune → transform) — [spec](specs/css-pipeline.md)
- [ ] CSS custom properties on elements & components
- [ ] Nested `<style>` elements (unscoped, global rules)

Самый большой standalone workstream — новый подсистема `svelte_css`. Pipeline: парсинг `<style>` в CSS AST (selectors, declarations, at-rules, nesting с `&`) → детерминистический hash (`svelte-{hash}` из filename) → анализ selectors (`:global()`, `:global { ... }`, `is_global_like` для `:root`/`:host`/`::view-transition-*`, keyframe collection) → pruning: backward matching selectors против template elements с обходом комбинаторов (descendant, child, adjacent, sibling), conservative matching для компонентов и сниппетов → трансформация: append `.svelte-HASH` class, удаление `:global()` синтаксиса, scoping `@keyframes`, pruning unused rules, минификация в prod. На стороне template codegen — injection `class="svelte-HASH"` для scoped элементов и поддержка `css: 'injected'` (embed в JS) / `css: 'external'` (отдельный файл).

Первый шаг — выбор CSS-стека: OXC css parser, `lightningcss`, `cssparser` (Servo), или свой парсер. Критерии: полнота CSS3 selectors, поддержка `:global()`/nesting, доступ к AST для мутаций, source maps.

## Validation & Diagnostics

- [ ] Diagnostic infrastructure — [spec](specs/diagnostics-infrastructure.md)
- [ ] Rune argument & placement validation
- [ ] Element & directive validation
- [ ] A11y warnings
- [ ] Unused selector warnings (depends on CSS)

## Dev Mode

- [ ] `$.tag()` / `$.tag_proxy()` rune tagging
- [ ] Strict equality transforms (`$.strict_equals` / `$.equals`)
- [ ] `$.apply()` + event handler naming
- [ ] Ownership validation
- [ ] Runtime validations (`$.validate_store`, console state logging, etc.)

## Compiler Infrastructure

- [ ] Module compilation (`.svelte.js` / `.svelte.ts`)
- [ ] WASM target
- [ ] Custom elements
- [ ] `discloseVersion` option
- [ ] `preserveComments` option
- [ ] Source maps (JS + CSS)
- [ ] HMR

## `<script module>` in Components — [spec](specs/script-module.md)

- [x] Analyze pass: scoping, rune detection, exports collection for module script body
- [x] Codegen: emit module script body as module-level output (separate from component function)
- [x] `export_undefined` diagnostic for unresolved module export specifiers
- [x] Interaction with instance script: module-scope bindings visible to instance, not vice versa

## Legacy Svelte 4

- [ ] `<slot>` + `let:`
- [ ] `<svelte:component>` / `<svelte:self>` / `<svelte:fragment>` 
- [ ] `export let` props
- [ ] `$:` reactive assignments
- [ ] `$$props` / `$$restProps` / `$$slots`
- [ ] `beforeUpdate` / `afterUpdate`
