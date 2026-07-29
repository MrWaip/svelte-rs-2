<script module>
    import { writable as makeStore, derived as derivedStore } from "svelte/store";

    export const BENCHMARK_KIND = "compiler";
    export const MODULE_SCALE = 3;

    export const moduleStore = makeStore(0);
    export const moduleDoubled = derivedStore(moduleStore, (value) => value * 2);

    let moduleCounter = $state(0);

    export function bumpModule() {
        moduleCounter += 1;
        moduleStore.set(moduleCounter);
        moduleStore.update((value) => value + 1);
    }

    export function moduleLabel(name) {
        return `${BENCHMARK_KIND}:${name}`;
    }

    class ModuleCounter {
        #count = $state(0);
        label = $state("module");
        doubled = $derived(this.#count * 2);

        increment() {
            this.#count += 1;
        }

        get count() {
            return this.#count;
        }
    }

    export const moduleCounterInstance = new ModuleCounter();
</script>

<script>
    import { onMount } from "svelte";
    import { writable } from "svelte/store";
    import { fade, fly, slide } from "svelte/transition";
    import { flip } from "svelte/animate";
    import ChildComponent from "./Child.svelte";

    let {
        title = "Default Title",
        count = 0,
        items = [],
        config = $bindable({}),
        multiplier = 2,
        visible = $bindable(false),
        heading: pageHeading = "Untitled",
        density: layoutDensity = "cozy",
        ...rest
    } = $props();

    const propsId = $props.id();

    let state = $state("");
    let counter = $state(0);
    let rawData = $state.raw({ x: 1, y: 2 });
    let checked = $state(false);
    let group = $state([]);
    let volume = $state(0.5);
    let selected = $state("opt-0");
    let selectedList = $state([]);
    let cache = $state({ value: 0, nested: { deep: 0 } });
    let plainA = 1, stateB = $state(2), plainC = "c";
    let depth = 0;
    let inputEl;
    let componentRef;
    let dynamicEl;

    let metrics = writable([1, 2, 3]);
    let labelStore = writable("ready");

    /** @type {Function | undefined} */
    let show;

    counter = 10;

    let doubled = $derived(count * multiplier);
    let computed = $derived.by(() => {
        return items.length * multiplier + counter;
    });
    let moduleSummary = $derived(moduleLabel(title) + ":" + MODULE_SCALE);
    let storeSummary = $derived($metrics.length + ":" + $labelStore);
    let snapshot = $state.snapshot(rawData);

    $effect(() => {
        console.log("Title:", title, "Count:", count);
    });

    $effect.pre(() => {
        console.log("Pre effect:", counter);
    });

    let tracking = $effect.tracking();

    let stopRoot = $effect.root(() => {
        let rootCount = $state(0);

        $effect(() => {
            console.log("root effect:", rootCount);
        });

        return () => {
            rootCount = 0;
        };
    });

    $inspect(counter, doubled);
    $inspect(state).with((type, value) => console.log(type, value));

    $effect(() => {
        $inspect.trace("counterEffect");
        console.log(counter, cache.value);
    });

    class Counter {
        #count = $state(0);
        label = $state("counter");
        doubled = $derived(this.#count * 2);

        increment() {
            this.#count += 1;
        }

        get count() {
            return this.#count;
        }
    }

    const counterInstance = new Counter();

    export const APP_VERSION = "1.0.0";

    const internalVersion = "1.0.1";

    export { internalVersion as PATCH_VERSION };

    async function fillCache() {
        cache.value ??= await Promise.resolve(counter);
        cache.nested.deep ||= await Promise.resolve(doubled);
        return cache;
    }

    export function formatTitle(prefix) {
        return prefix + ": " + title;
    }

    function addMetric() {
        $metrics = [...$metrics, counter];
        $labelStore = title;
    }

    function action(node, arg) {
        return { destroy() {} };
    }

    function handleClick(e) {
        counter++;
    }

    function getHandler() {
        return handleClick;
    }

    function handleError(error) {
        console.error(error);
    }

    let promise = Promise.resolve(42);
</script>

<style>
    :global(body) {
        margin: 0;
        font-family: "IBM Plex Sans", sans-serif;
        background: #f5f1e8;
    }

    :global(.benchmark-host) {
        color: #3f2a18;
    }

    :global {
        .benchmark-reset {
            box-sizing: border-box;
        }
    }

    @keyframes pulse {
        0% { opacity: 0.4; transform: scale(0.98); }
        100% { opacity: 1; transform: scale(1); }
    }

    @keyframes -global-marquee {
        from { transform: translateX(0); }
        to { transform: translateX(12px); }
    }

    .chunk-shell {
        padding: 16px;
        margin: 12px 0;
        border: 1px solid #d9c7ab;
        background: linear-gradient(180deg, #fffdf9 0%, #f4ead9 100%);
    }

    .chunk-shell :is(.badge, .card, .summary) {
        border-radius: 10px;
    }

    .chunk-shell.state .summary {
        animation: pulse 180ms ease-out;
    }

    .summary span {
        display: inline-block;
        margin-right: 8px;
    }

    .item-less{ color: #7a4f2a; }

    [data-index] {
        color: var(--custom, #5c4634);
    }

    [data-kind^="chunk-"] {
        font-weight: 600;
    }

    [data-id$="-summary"] {
        letter-spacing: 0.01em;
    }

    :root {
        --custom: #5c4634;
    }

    :root .chunk-shell {
        line-height: 1.5;
    }

    :host > .chunk-shell {
        color: inherit;
    }

    .chunk-shell:has(.badge) {
        outline: 1px dashed #d9c7ab;
    }

    .card:not(.badge, :global(.legacy)) {
        background: #fffdf9;
    }

    :where(.badge, .card) {
        line-height: 1.4;
    }

    .card h3 {
        margin: 0;
    }

    h1 + p {
        margin-top: 4px;
    }

    h2 ~ p {
        color: #7a4f2a;
    }

    .badge::before {
        content: "\2605";
    }

    .\32 5px-gap {
        gap: 25px;
    }

    .benchmark-unused-guard {
        color: red;
    }

    @media (min-width: 1px) {
        .chunk-shell {
            color: inherit;
        }

        .summary span {
            font-variant-numeric: tabular-nums;
        }
    }

    @supports (display: grid) {
        .card {
            display: grid;
        }
    }

    :global {
        .benchmark-legacy {
            &.active {
                color: green;
            }
        }

        .benchmark-fade-a {
            animation: pulse 1s;
        }

        .benchmark-fade-b {
            animation: pulse 2s;
        }
    }
</style>

<svelte:head>
    <title>{title} - Benchmark</title>
    <meta name="description" content="Benchmark component">
    <link rel="canonical" href="/benchmark">
</svelte:head>

<svelte:options runes />

<svelte:window onscroll={handleClick} />
<svelte:document onvisibilitychange={handleClick} />
<svelte:body onmouseenter={handleClick} use:action={state} />

{#snippet badge(text, variant)}
    <span class="badge" class:primary={variant === "primary"} class:secondary={variant === "secondary"}>
        {text}
    </span>
{/snippet}

{#snippet card(heading, body)}
    <div class="card">
        <h3>{heading}</h3>
        <p>{body}</p>
        {@render badge("new", "primary")}
    </div>
{/snippet}

{#snippet metricSummary({ label, values = [counter], meta: { id = propsId } = {} })}
    <section class="summary" data-id={id}>
        <h4>{label}</h4>
        {#each values as value, index}
            <span>{index}: {value}</span>
        {/each}
    </section>
{/snippet}

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-0">
    Chunk 0: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {moduleSummary} | Store: {storeSummary} | Label: {$labelStore}</p>

    {@html "<b>raw html chunk 0</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-0"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 0.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 0.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 0.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 0: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = `${idx}:${item.name}`}
        <p {...rest} data-index="chunk-0-{idx}" animate:flip>{itemLabel}</p>
    {/each}

    {#each items}
        <span class="item-less">Repeated shell chunk 0</span>
    {/each}

    {#each items as item (item.id)}
        <p animate:flip={{ duration: 200 }}>{item.name}</p>
    {:else}
        <p>No items in chunk 0</p>
    {/each}

    {#await promise}
        <p>Loading chunk 0...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    {#await promise then quickValue}
        <p>Quick resolved: {quickValue}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value="opt-1">One</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
        <option value={`opt-${counter}`}>{title}</option>
    </select>
    <input type="checkbox" bind:checked={checked} />
    <input type="checkbox" bind:group={group} value="opt-0" />
    <input type="radio" bind:group={group} value="opt-0" />
    <input type="number" bind:value={counter} />
    <details bind:open={checked}><summary>More</summary>chunk 0</details>
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div {@attach (node) => { node.dataset.chunk = "0"; }}>attach target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <p transition:slide|global>global transition chunk 0</p>
    <p in:fly|local={{ y: 40 }}>local transition chunk 0</p>

    <div style:opacity style:color={state}>style shorthand</div>
    <div class="slider chunk-0 {title}" class:invinsible>literal class</div>
    <div class={`chunk-${i}-${counter}`}>template literal class</div>
    <span data-note="&copy;=value" title="&copy 1998" aria-label="a &amp; b">entities</span>

    <template id="chunk-tpl-0">
        {@html "<b>templated</b>"}
        {#if counter}<span>inside template</span>{/if}
    </template>

    <svg viewBox="0 0 10 10" class="chunk-svg">
        <clipPath id="clip-0"><rect width="4" height="4" /></clipPath>
        <linearGradient id="grad-0"><stop offset="0%" stop-color={state} /></linearGradient>
        <foreignObject width="10" height="10"><div>svg foreign chunk 0</div></foreignObject>
    </svg>

    <math><mrow><mi>x</mi><mo>+</mo><mn>{counter}</mn></mrow></math>
    <svelte:element this={state ? "div" : "span"} class="dynamic-0">
        Dynamic element chunk 0: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()}>
        <strong>Inline child chunk 0: {title}</strong>
        <div slot="footer">Footer chunk 0: {counter}</div>
    </ChildComponent>

    <ChildComponent --accent="var(--custom, #5c4634)" {...rest} bind:config>
        {#snippet header(label)}
            <em>Header snippet chunk 0: {label}</em>
        {/snippet}
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={pageHeading} />

    {#if depth > 0}
        <svelte:self depth={depth - 1} title={title} />
    {/if}

    {@render badge("chunk-0", "secondary")}
    {@render card(title, "Content for chunk 0")}
    {@render metricSummary({ label: title, values: [count, doubled, counter], meta: { id: propsId } })}
    {@render show?.()}

    <button onclick={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 0: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 0: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-1">
    Chunk 1: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {moduleSummary} | Store: {storeSummary} | Label: {$labelStore}</p>

    {@html "<b>raw html chunk 1</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-1"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 1.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 1.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 1.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 1: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = `${idx}:${item.name}`}
        <p {...rest} data-index="chunk-1-{idx}" animate:flip>{itemLabel}</p>
    {/each}

    {#each items}
        <span class="item-less">Repeated shell chunk 1</span>
    {/each}

    {#each items as item (item.id)}
        <p animate:flip={{ duration: 200 }}>{item.name}</p>
    {:else}
        <p>No items in chunk 1</p>
    {/each}

    {#await promise}
        <p>Loading chunk 1...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    {#await promise then quickValue}
        <p>Quick resolved: {quickValue}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value="opt-1">One</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
        <option value={`opt-${counter}`}>{title}</option>
    </select>
    <input type="checkbox" bind:checked={checked} />
    <input type="checkbox" bind:group={group} value="opt-1" />
    <input type="radio" bind:group={group} value="opt-1" />
    <input type="number" bind:value={counter} />
    <details bind:open={checked}><summary>More</summary>chunk 1</details>
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div {@attach (node) => { node.dataset.chunk = "1"; }}>attach target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <p transition:slide|global>global transition chunk 1</p>
    <p in:fly|local={{ y: 40 }}>local transition chunk 1</p>

    <div style:opacity style:color={state}>style shorthand</div>
    <div class="slider chunk-1 {title}" class:invinsible>literal class</div>
    <div class={`chunk-${i}-${counter}`}>template literal class</div>
    <span data-note="&copy;=value" title="&copy 1998" aria-label="a &amp; b">entities</span>

    <template id="chunk-tpl-1">
        {@html "<b>templated</b>"}
        {#if counter}<span>inside template</span>{/if}
    </template>

    <svg viewBox="0 0 10 10" class="chunk-svg">
        <clipPath id="clip-1"><rect width="4" height="4" /></clipPath>
        <linearGradient id="grad-1"><stop offset="0%" stop-color={state} /></linearGradient>
        <foreignObject width="10" height="10"><div>svg foreign chunk 1</div></foreignObject>
    </svg>

    <math><mrow><mi>x</mi><mo>+</mo><mn>{counter}</mn></mrow></math>
    <svelte:element this={state ? "div" : "span"} class="dynamic-1">
        Dynamic element chunk 1: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()}>
        <strong>Inline child chunk 1: {title}</strong>
        <div slot="footer">Footer chunk 1: {counter}</div>
    </ChildComponent>

    <ChildComponent --accent="var(--custom, #5c4634)" {...rest} bind:config>
        {#snippet header(label)}
            <em>Header snippet chunk 1: {label}</em>
        {/snippet}
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={pageHeading} />

    {#if depth > 0}
        <svelte:self depth={depth - 1} title={title} />
    {/if}

    {@render badge("chunk-1", "secondary")}
    {@render card(title, "Content for chunk 1")}
    {@render metricSummary({ label: title, values: [count, doubled, counter], meta: { id: propsId } })}
    {@render show?.()}

    <button onclick={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 1: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 1: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-2">
    Chunk 2: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {moduleSummary} | Store: {storeSummary} | Label: {$labelStore}</p>

    {@html "<b>raw html chunk 2</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-2"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 2.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 2.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 2.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 2: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = `${idx}:${item.name}`}
        <p {...rest} data-index="chunk-2-{idx}" animate:flip>{itemLabel}</p>
    {/each}

    {#each items}
        <span class="item-less">Repeated shell chunk 2</span>
    {/each}

    {#each items as item (item.id)}
        <p animate:flip={{ duration: 200 }}>{item.name}</p>
    {:else}
        <p>No items in chunk 2</p>
    {/each}

    {#await promise}
        <p>Loading chunk 2...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    {#await promise then quickValue}
        <p>Quick resolved: {quickValue}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value="opt-1">One</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
        <option value={`opt-${counter}`}>{title}</option>
    </select>
    <input type="checkbox" bind:checked={checked} />
    <input type="checkbox" bind:group={group} value="opt-2" />
    <input type="radio" bind:group={group} value="opt-2" />
    <input type="number" bind:value={counter} />
    <details bind:open={checked}><summary>More</summary>chunk 2</details>
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div {@attach (node) => { node.dataset.chunk = "2"; }}>attach target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <p transition:slide|global>global transition chunk 2</p>
    <p in:fly|local={{ y: 40 }}>local transition chunk 2</p>

    <div style:opacity style:color={state}>style shorthand</div>
    <div class="slider chunk-2 {title}" class:invinsible>literal class</div>
    <div class={`chunk-${i}-${counter}`}>template literal class</div>
    <span data-note="&copy;=value" title="&copy 1998" aria-label="a &amp; b">entities</span>

    <template id="chunk-tpl-2">
        {@html "<b>templated</b>"}
        {#if counter}<span>inside template</span>{/if}
    </template>

    <svg viewBox="0 0 10 10" class="chunk-svg">
        <clipPath id="clip-2"><rect width="4" height="4" /></clipPath>
        <linearGradient id="grad-2"><stop offset="0%" stop-color={state} /></linearGradient>
        <foreignObject width="10" height="10"><div>svg foreign chunk 2</div></foreignObject>
    </svg>

    <math><mrow><mi>x</mi><mo>+</mo><mn>{counter}</mn></mrow></math>
    <svelte:element this={state ? "div" : "span"} class="dynamic-2">
        Dynamic element chunk 2: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()}>
        <strong>Inline child chunk 2: {title}</strong>
        <div slot="footer">Footer chunk 2: {counter}</div>
    </ChildComponent>

    <ChildComponent --accent="var(--custom, #5c4634)" {...rest} bind:config>
        {#snippet header(label)}
            <em>Header snippet chunk 2: {label}</em>
        {/snippet}
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={pageHeading} />

    {#if depth > 0}
        <svelte:self depth={depth - 1} title={title} />
    {/if}

    {@render badge("chunk-2", "secondary")}
    {@render card(title, "Content for chunk 2")}
    {@render metricSummary({ label: title, values: [count, doubled, counter], meta: { id: propsId } })}
    {@render show?.()}

    <button onclick={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 2: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 2: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-3">
    Chunk 3: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {moduleSummary} | Store: {storeSummary} | Label: {$labelStore}</p>

    {@html "<b>raw html chunk 3</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-3"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 3.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 3.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 3.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 3: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = `${idx}:${item.name}`}
        <p {...rest} data-index="chunk-3-{idx}" animate:flip>{itemLabel}</p>
    {/each}

    {#each items}
        <span class="item-less">Repeated shell chunk 3</span>
    {/each}

    {#each items as item (item.id)}
        <p animate:flip={{ duration: 200 }}>{item.name}</p>
    {:else}
        <p>No items in chunk 3</p>
    {/each}

    {#await promise}
        <p>Loading chunk 3...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    {#await promise then quickValue}
        <p>Quick resolved: {quickValue}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value="opt-1">One</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
        <option value={`opt-${counter}`}>{title}</option>
    </select>
    <input type="checkbox" bind:checked={checked} />
    <input type="checkbox" bind:group={group} value="opt-3" />
    <input type="radio" bind:group={group} value="opt-3" />
    <input type="number" bind:value={counter} />
    <details bind:open={checked}><summary>More</summary>chunk 3</details>
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div {@attach (node) => { node.dataset.chunk = "3"; }}>attach target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <p transition:slide|global>global transition chunk 3</p>
    <p in:fly|local={{ y: 40 }}>local transition chunk 3</p>

    <div style:opacity style:color={state}>style shorthand</div>
    <div class="slider chunk-3 {title}" class:invinsible>literal class</div>
    <div class={`chunk-${i}-${counter}`}>template literal class</div>
    <span data-note="&copy;=value" title="&copy 1998" aria-label="a &amp; b">entities</span>

    <template id="chunk-tpl-3">
        {@html "<b>templated</b>"}
        {#if counter}<span>inside template</span>{/if}
    </template>

    <svg viewBox="0 0 10 10" class="chunk-svg">
        <clipPath id="clip-3"><rect width="4" height="4" /></clipPath>
        <linearGradient id="grad-3"><stop offset="0%" stop-color={state} /></linearGradient>
        <foreignObject width="10" height="10"><div>svg foreign chunk 3</div></foreignObject>
    </svg>

    <math><mrow><mi>x</mi><mo>+</mo><mn>{counter}</mn></mrow></math>
    <svelte:element this={state ? "div" : "span"} class="dynamic-3">
        Dynamic element chunk 3: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()}>
        <strong>Inline child chunk 3: {title}</strong>
        <div slot="footer">Footer chunk 3: {counter}</div>
    </ChildComponent>

    <ChildComponent --accent="var(--custom, #5c4634)" {...rest} bind:config>
        {#snippet header(label)}
            <em>Header snippet chunk 3: {label}</em>
        {/snippet}
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={pageHeading} />

    {#if depth > 0}
        <svelte:self depth={depth - 1} title={title} />
    {/if}

    {@render badge("chunk-3", "secondary")}
    {@render card(title, "Content for chunk 3")}
    {@render metricSummary({ label: title, values: [count, doubled, counter], meta: { id: propsId } })}
    {@render show?.()}

    <button onclick={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 3: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 3: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-4">
    Chunk 4: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {moduleSummary} | Store: {storeSummary} | Label: {$labelStore}</p>

    {@html "<b>raw html chunk 4</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-4"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 4.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 4.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 4.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 4: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = `${idx}:${item.name}`}
        <p {...rest} data-index="chunk-4-{idx}" animate:flip>{itemLabel}</p>
    {/each}

    {#each items}
        <span class="item-less">Repeated shell chunk 4</span>
    {/each}

    {#each items as item (item.id)}
        <p animate:flip={{ duration: 200 }}>{item.name}</p>
    {:else}
        <p>No items in chunk 4</p>
    {/each}

    {#await promise}
        <p>Loading chunk 4...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    {#await promise then quickValue}
        <p>Quick resolved: {quickValue}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value="opt-1">One</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
        <option value={`opt-${counter}`}>{title}</option>
    </select>
    <input type="checkbox" bind:checked={checked} />
    <input type="checkbox" bind:group={group} value="opt-4" />
    <input type="radio" bind:group={group} value="opt-4" />
    <input type="number" bind:value={counter} />
    <details bind:open={checked}><summary>More</summary>chunk 4</details>
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div {@attach (node) => { node.dataset.chunk = "4"; }}>attach target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <p transition:slide|global>global transition chunk 4</p>
    <p in:fly|local={{ y: 40 }}>local transition chunk 4</p>

    <div style:opacity style:color={state}>style shorthand</div>
    <div class="slider chunk-4 {title}" class:invinsible>literal class</div>
    <div class={`chunk-${i}-${counter}`}>template literal class</div>
    <span data-note="&copy;=value" title="&copy 1998" aria-label="a &amp; b">entities</span>

    <template id="chunk-tpl-4">
        {@html "<b>templated</b>"}
        {#if counter}<span>inside template</span>{/if}
    </template>

    <svg viewBox="0 0 10 10" class="chunk-svg">
        <clipPath id="clip-4"><rect width="4" height="4" /></clipPath>
        <linearGradient id="grad-4"><stop offset="0%" stop-color={state} /></linearGradient>
        <foreignObject width="10" height="10"><div>svg foreign chunk 4</div></foreignObject>
    </svg>

    <math><mrow><mi>x</mi><mo>+</mo><mn>{counter}</mn></mrow></math>
    <svelte:element this={state ? "div" : "span"} class="dynamic-4">
        Dynamic element chunk 4: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()}>
        <strong>Inline child chunk 4: {title}</strong>
        <div slot="footer">Footer chunk 4: {counter}</div>
    </ChildComponent>

    <ChildComponent --accent="var(--custom, #5c4634)" {...rest} bind:config>
        {#snippet header(label)}
            <em>Header snippet chunk 4: {label}</em>
        {/snippet}
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={pageHeading} />

    {#if depth > 0}
        <svelte:self depth={depth - 1} title={title} />
    {/if}

    {@render badge("chunk-4", "secondary")}
    {@render card(title, "Content for chunk 4")}
    {@render metricSummary({ label: title, values: [count, doubled, counter], meta: { id: propsId } })}
    {@render show?.()}

    <button onclick={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 4: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 4: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-5">
    Chunk 5: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {moduleSummary} | Store: {storeSummary} | Label: {$labelStore}</p>

    {@html "<b>raw html chunk 5</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-5"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 5.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 5.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 5.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 5: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = `${idx}:${item.name}`}
        <p {...rest} data-index="chunk-5-{idx}" animate:flip>{itemLabel}</p>
    {/each}

    {#each items}
        <span class="item-less">Repeated shell chunk 5</span>
    {/each}

    {#each items as item (item.id)}
        <p animate:flip={{ duration: 200 }}>{item.name}</p>
    {:else}
        <p>No items in chunk 5</p>
    {/each}

    {#await promise}
        <p>Loading chunk 5...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    {#await promise then quickValue}
        <p>Quick resolved: {quickValue}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value="opt-1">One</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
        <option value={`opt-${counter}`}>{title}</option>
    </select>
    <input type="checkbox" bind:checked={checked} />
    <input type="checkbox" bind:group={group} value="opt-5" />
    <input type="radio" bind:group={group} value="opt-5" />
    <input type="number" bind:value={counter} />
    <details bind:open={checked}><summary>More</summary>chunk 5</details>
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div {@attach (node) => { node.dataset.chunk = "5"; }}>attach target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <p transition:slide|global>global transition chunk 5</p>
    <p in:fly|local={{ y: 40 }}>local transition chunk 5</p>

    <div style:opacity style:color={state}>style shorthand</div>
    <div class="slider chunk-5 {title}" class:invinsible>literal class</div>
    <div class={`chunk-${i}-${counter}`}>template literal class</div>
    <span data-note="&copy;=value" title="&copy 1998" aria-label="a &amp; b">entities</span>

    <template id="chunk-tpl-5">
        {@html "<b>templated</b>"}
        {#if counter}<span>inside template</span>{/if}
    </template>

    <svg viewBox="0 0 10 10" class="chunk-svg">
        <clipPath id="clip-5"><rect width="4" height="4" /></clipPath>
        <linearGradient id="grad-5"><stop offset="0%" stop-color={state} /></linearGradient>
        <foreignObject width="10" height="10"><div>svg foreign chunk 5</div></foreignObject>
    </svg>

    <math><mrow><mi>x</mi><mo>+</mo><mn>{counter}</mn></mrow></math>
    <svelte:element this={state ? "div" : "span"} class="dynamic-5">
        Dynamic element chunk 5: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()}>
        <strong>Inline child chunk 5: {title}</strong>
        <div slot="footer">Footer chunk 5: {counter}</div>
    </ChildComponent>

    <ChildComponent --accent="var(--custom, #5c4634)" {...rest} bind:config>
        {#snippet header(label)}
            <em>Header snippet chunk 5: {label}</em>
        {/snippet}
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={pageHeading} />

    {#if depth > 0}
        <svelte:self depth={depth - 1} title={title} />
    {/if}

    {@render badge("chunk-5", "secondary")}
    {@render card(title, "Content for chunk 5")}
    {@render metricSummary({ label: title, values: [count, doubled, counter], meta: { id: propsId } })}
    {@render show?.()}

    <button onclick={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 5: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 5: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-6">
    Chunk 6: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {moduleSummary} | Store: {storeSummary} | Label: {$labelStore}</p>

    {@html "<b>raw html chunk 6</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-6"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 6.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 6.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 6.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 6: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = `${idx}:${item.name}`}
        <p {...rest} data-index="chunk-6-{idx}" animate:flip>{itemLabel}</p>
    {/each}

    {#each items}
        <span class="item-less">Repeated shell chunk 6</span>
    {/each}

    {#each items as item (item.id)}
        <p animate:flip={{ duration: 200 }}>{item.name}</p>
    {:else}
        <p>No items in chunk 6</p>
    {/each}

    {#await promise}
        <p>Loading chunk 6...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    {#await promise then quickValue}
        <p>Quick resolved: {quickValue}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value="opt-1">One</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
        <option value={`opt-${counter}`}>{title}</option>
    </select>
    <input type="checkbox" bind:checked={checked} />
    <input type="checkbox" bind:group={group} value="opt-6" />
    <input type="radio" bind:group={group} value="opt-6" />
    <input type="number" bind:value={counter} />
    <details bind:open={checked}><summary>More</summary>chunk 6</details>
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div {@attach (node) => { node.dataset.chunk = "6"; }}>attach target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <p transition:slide|global>global transition chunk 6</p>
    <p in:fly|local={{ y: 40 }}>local transition chunk 6</p>

    <div style:opacity style:color={state}>style shorthand</div>
    <div class="slider chunk-6 {title}" class:invinsible>literal class</div>
    <div class={`chunk-${i}-${counter}`}>template literal class</div>
    <span data-note="&copy;=value" title="&copy 1998" aria-label="a &amp; b">entities</span>

    <template id="chunk-tpl-6">
        {@html "<b>templated</b>"}
        {#if counter}<span>inside template</span>{/if}
    </template>

    <svg viewBox="0 0 10 10" class="chunk-svg">
        <clipPath id="clip-6"><rect width="4" height="4" /></clipPath>
        <linearGradient id="grad-6"><stop offset="0%" stop-color={state} /></linearGradient>
        <foreignObject width="10" height="10"><div>svg foreign chunk 6</div></foreignObject>
    </svg>

    <math><mrow><mi>x</mi><mo>+</mo><mn>{counter}</mn></mrow></math>
    <svelte:element this={state ? "div" : "span"} class="dynamic-6">
        Dynamic element chunk 6: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()}>
        <strong>Inline child chunk 6: {title}</strong>
        <div slot="footer">Footer chunk 6: {counter}</div>
    </ChildComponent>

    <ChildComponent --accent="var(--custom, #5c4634)" {...rest} bind:config>
        {#snippet header(label)}
            <em>Header snippet chunk 6: {label}</em>
        {/snippet}
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={pageHeading} />

    {#if depth > 0}
        <svelte:self depth={depth - 1} title={title} />
    {/if}

    {@render badge("chunk-6", "secondary")}
    {@render card(title, "Content for chunk 6")}
    {@render metricSummary({ label: title, values: [count, doubled, counter], meta: { id: propsId } })}
    {@render show?.()}

    <button onclick={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 6: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 6: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-7">
    Chunk 7: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {moduleSummary} | Store: {storeSummary} | Label: {$labelStore}</p>

    {@html "<b>raw html chunk 7</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-7"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 7.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 7.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 7.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 7: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = `${idx}:${item.name}`}
        <p {...rest} data-index="chunk-7-{idx}" animate:flip>{itemLabel}</p>
    {/each}

    {#each items}
        <span class="item-less">Repeated shell chunk 7</span>
    {/each}

    {#each items as item (item.id)}
        <p animate:flip={{ duration: 200 }}>{item.name}</p>
    {:else}
        <p>No items in chunk 7</p>
    {/each}

    {#await promise}
        <p>Loading chunk 7...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    {#await promise then quickValue}
        <p>Quick resolved: {quickValue}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value="opt-1">One</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
        <option value={`opt-${counter}`}>{title}</option>
    </select>
    <input type="checkbox" bind:checked={checked} />
    <input type="checkbox" bind:group={group} value="opt-7" />
    <input type="radio" bind:group={group} value="opt-7" />
    <input type="number" bind:value={counter} />
    <details bind:open={checked}><summary>More</summary>chunk 7</details>
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div {@attach (node) => { node.dataset.chunk = "7"; }}>attach target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <p transition:slide|global>global transition chunk 7</p>
    <p in:fly|local={{ y: 40 }}>local transition chunk 7</p>

    <div style:opacity style:color={state}>style shorthand</div>
    <div class="slider chunk-7 {title}" class:invinsible>literal class</div>
    <div class={`chunk-${i}-${counter}`}>template literal class</div>
    <span data-note="&copy;=value" title="&copy 1998" aria-label="a &amp; b">entities</span>

    <template id="chunk-tpl-7">
        {@html "<b>templated</b>"}
        {#if counter}<span>inside template</span>{/if}
    </template>

    <svg viewBox="0 0 10 10" class="chunk-svg">
        <clipPath id="clip-7"><rect width="4" height="4" /></clipPath>
        <linearGradient id="grad-7"><stop offset="0%" stop-color={state} /></linearGradient>
        <foreignObject width="10" height="10"><div>svg foreign chunk 7</div></foreignObject>
    </svg>

    <math><mrow><mi>x</mi><mo>+</mo><mn>{counter}</mn></mrow></math>
    <svelte:element this={state ? "div" : "span"} class="dynamic-7">
        Dynamic element chunk 7: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()}>
        <strong>Inline child chunk 7: {title}</strong>
        <div slot="footer">Footer chunk 7: {counter}</div>
    </ChildComponent>

    <ChildComponent --accent="var(--custom, #5c4634)" {...rest} bind:config>
        {#snippet header(label)}
            <em>Header snippet chunk 7: {label}</em>
        {/snippet}
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={pageHeading} />

    {#if depth > 0}
        <svelte:self depth={depth - 1} title={title} />
    {/if}

    {@render badge("chunk-7", "secondary")}
    {@render card(title, "Content for chunk 7")}
    {@render metricSummary({ label: title, values: [count, doubled, counter], meta: { id: propsId } })}
    {@render show?.()}

    <button onclick={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 7: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 7: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-8">
    Chunk 8: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {moduleSummary} | Store: {storeSummary} | Label: {$labelStore}</p>

    {@html "<b>raw html chunk 8</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-8"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 8.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 8.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 8.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 8: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = `${idx}:${item.name}`}
        <p {...rest} data-index="chunk-8-{idx}" animate:flip>{itemLabel}</p>
    {/each}

    {#each items}
        <span class="item-less">Repeated shell chunk 8</span>
    {/each}

    {#each items as item (item.id)}
        <p animate:flip={{ duration: 200 }}>{item.name}</p>
    {:else}
        <p>No items in chunk 8</p>
    {/each}

    {#await promise}
        <p>Loading chunk 8...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    {#await promise then quickValue}
        <p>Quick resolved: {quickValue}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value="opt-1">One</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
        <option value={`opt-${counter}`}>{title}</option>
    </select>
    <input type="checkbox" bind:checked={checked} />
    <input type="checkbox" bind:group={group} value="opt-8" />
    <input type="radio" bind:group={group} value="opt-8" />
    <input type="number" bind:value={counter} />
    <details bind:open={checked}><summary>More</summary>chunk 8</details>
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div {@attach (node) => { node.dataset.chunk = "8"; }}>attach target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <p transition:slide|global>global transition chunk 8</p>
    <p in:fly|local={{ y: 40 }}>local transition chunk 8</p>

    <div style:opacity style:color={state}>style shorthand</div>
    <div class="slider chunk-8 {title}" class:invinsible>literal class</div>
    <div class={`chunk-${i}-${counter}`}>template literal class</div>
    <span data-note="&copy;=value" title="&copy 1998" aria-label="a &amp; b">entities</span>

    <template id="chunk-tpl-8">
        {@html "<b>templated</b>"}
        {#if counter}<span>inside template</span>{/if}
    </template>

    <svg viewBox="0 0 10 10" class="chunk-svg">
        <clipPath id="clip-8"><rect width="4" height="4" /></clipPath>
        <linearGradient id="grad-8"><stop offset="0%" stop-color={state} /></linearGradient>
        <foreignObject width="10" height="10"><div>svg foreign chunk 8</div></foreignObject>
    </svg>

    <math><mrow><mi>x</mi><mo>+</mo><mn>{counter}</mn></mrow></math>
    <svelte:element this={state ? "div" : "span"} class="dynamic-8">
        Dynamic element chunk 8: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()}>
        <strong>Inline child chunk 8: {title}</strong>
        <div slot="footer">Footer chunk 8: {counter}</div>
    </ChildComponent>

    <ChildComponent --accent="var(--custom, #5c4634)" {...rest} bind:config>
        {#snippet header(label)}
            <em>Header snippet chunk 8: {label}</em>
        {/snippet}
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={pageHeading} />

    {#if depth > 0}
        <svelte:self depth={depth - 1} title={title} />
    {/if}

    {@render badge("chunk-8", "secondary")}
    {@render card(title, "Content for chunk 8")}
    {@render metricSummary({ label: title, values: [count, doubled, counter], meta: { id: propsId } })}
    {@render show?.()}

    <button onclick={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 8: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 8: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-9">
    Chunk 9: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {moduleSummary} | Store: {storeSummary} | Label: {$labelStore}</p>

    {@html "<b>raw html chunk 9</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-9"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 9.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 9.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 9.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 9: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = `${idx}:${item.name}`}
        <p {...rest} data-index="chunk-9-{idx}" animate:flip>{itemLabel}</p>
    {/each}

    {#each items}
        <span class="item-less">Repeated shell chunk 9</span>
    {/each}

    {#each items as item (item.id)}
        <p animate:flip={{ duration: 200 }}>{item.name}</p>
    {:else}
        <p>No items in chunk 9</p>
    {/each}

    {#await promise}
        <p>Loading chunk 9...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    {#await promise then quickValue}
        <p>Quick resolved: {quickValue}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value="opt-1">One</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
        <option value={`opt-${counter}`}>{title}</option>
    </select>
    <input type="checkbox" bind:checked={checked} />
    <input type="checkbox" bind:group={group} value="opt-9" />
    <input type="radio" bind:group={group} value="opt-9" />
    <input type="number" bind:value={counter} />
    <details bind:open={checked}><summary>More</summary>chunk 9</details>
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div {@attach (node) => { node.dataset.chunk = "9"; }}>attach target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <p transition:slide|global>global transition chunk 9</p>
    <p in:fly|local={{ y: 40 }}>local transition chunk 9</p>

    <div style:opacity style:color={state}>style shorthand</div>
    <div class="slider chunk-9 {title}" class:invinsible>literal class</div>
    <div class={`chunk-${i}-${counter}`}>template literal class</div>
    <span data-note="&copy;=value" title="&copy 1998" aria-label="a &amp; b">entities</span>

    <template id="chunk-tpl-9">
        {@html "<b>templated</b>"}
        {#if counter}<span>inside template</span>{/if}
    </template>

    <svg viewBox="0 0 10 10" class="chunk-svg">
        <clipPath id="clip-9"><rect width="4" height="4" /></clipPath>
        <linearGradient id="grad-9"><stop offset="0%" stop-color={state} /></linearGradient>
        <foreignObject width="10" height="10"><div>svg foreign chunk 9</div></foreignObject>
    </svg>

    <math><mrow><mi>x</mi><mo>+</mo><mn>{counter}</mn></mrow></math>
    <svelte:element this={state ? "div" : "span"} class="dynamic-9">
        Dynamic element chunk 9: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()}>
        <strong>Inline child chunk 9: {title}</strong>
        <div slot="footer">Footer chunk 9: {counter}</div>
    </ChildComponent>

    <ChildComponent --accent="var(--custom, #5c4634)" {...rest} bind:config>
        {#snippet header(label)}
            <em>Header snippet chunk 9: {label}</em>
        {/snippet}
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={pageHeading} />

    {#if depth > 0}
        <svelte:self depth={depth - 1} title={title} />
    {/if}

    {@render badge("chunk-9", "secondary")}
    {@render card(title, "Content for chunk 9")}
    {@render metricSummary({ label: title, values: [count, doubled, counter], meta: { id: propsId } })}
    {@render show?.()}

    <button onclick={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 9: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 9: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-10">
    Chunk 10: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {moduleSummary} | Store: {storeSummary} | Label: {$labelStore}</p>

    {@html "<b>raw html chunk 10</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-10"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 10.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 10.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 10.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 10: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = `${idx}:${item.name}`}
        <p {...rest} data-index="chunk-10-{idx}" animate:flip>{itemLabel}</p>
    {/each}

    {#each items}
        <span class="item-less">Repeated shell chunk 10</span>
    {/each}

    {#each items as item (item.id)}
        <p animate:flip={{ duration: 200 }}>{item.name}</p>
    {:else}
        <p>No items in chunk 10</p>
    {/each}

    {#await promise}
        <p>Loading chunk 10...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    {#await promise then quickValue}
        <p>Quick resolved: {quickValue}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value="opt-1">One</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
        <option value={`opt-${counter}`}>{title}</option>
    </select>
    <input type="checkbox" bind:checked={checked} />
    <input type="checkbox" bind:group={group} value="opt-10" />
    <input type="radio" bind:group={group} value="opt-10" />
    <input type="number" bind:value={counter} />
    <details bind:open={checked}><summary>More</summary>chunk 10</details>
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div {@attach (node) => { node.dataset.chunk = "10"; }}>attach target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <p transition:slide|global>global transition chunk 10</p>
    <p in:fly|local={{ y: 40 }}>local transition chunk 10</p>

    <div style:opacity style:color={state}>style shorthand</div>
    <div class="slider chunk-10 {title}" class:invinsible>literal class</div>
    <div class={`chunk-${i}-${counter}`}>template literal class</div>
    <span data-note="&copy;=value" title="&copy 1998" aria-label="a &amp; b">entities</span>

    <template id="chunk-tpl-10">
        {@html "<b>templated</b>"}
        {#if counter}<span>inside template</span>{/if}
    </template>

    <svg viewBox="0 0 10 10" class="chunk-svg">
        <clipPath id="clip-10"><rect width="4" height="4" /></clipPath>
        <linearGradient id="grad-10"><stop offset="0%" stop-color={state} /></linearGradient>
        <foreignObject width="10" height="10"><div>svg foreign chunk 10</div></foreignObject>
    </svg>

    <math><mrow><mi>x</mi><mo>+</mo><mn>{counter}</mn></mrow></math>
    <svelte:element this={state ? "div" : "span"} class="dynamic-10">
        Dynamic element chunk 10: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()}>
        <strong>Inline child chunk 10: {title}</strong>
        <div slot="footer">Footer chunk 10: {counter}</div>
    </ChildComponent>

    <ChildComponent --accent="var(--custom, #5c4634)" {...rest} bind:config>
        {#snippet header(label)}
            <em>Header snippet chunk 10: {label}</em>
        {/snippet}
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={pageHeading} />

    {#if depth > 0}
        <svelte:self depth={depth - 1} title={title} />
    {/if}

    {@render badge("chunk-10", "secondary")}
    {@render card(title, "Content for chunk 10")}
    {@render metricSummary({ label: title, values: [count, doubled, counter], meta: { id: propsId } })}
    {@render show?.()}

    <button onclick={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 10: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 10: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-11">
    Chunk 11: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {moduleSummary} | Store: {storeSummary} | Label: {$labelStore}</p>

    {@html "<b>raw html chunk 11</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-11"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 11.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 11.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 11.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 11: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = `${idx}:${item.name}`}
        <p {...rest} data-index="chunk-11-{idx}" animate:flip>{itemLabel}</p>
    {/each}

    {#each items}
        <span class="item-less">Repeated shell chunk 11</span>
    {/each}

    {#each items as item (item.id)}
        <p animate:flip={{ duration: 200 }}>{item.name}</p>
    {:else}
        <p>No items in chunk 11</p>
    {/each}

    {#await promise}
        <p>Loading chunk 11...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    {#await promise then quickValue}
        <p>Quick resolved: {quickValue}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value="opt-1">One</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
        <option value={`opt-${counter}`}>{title}</option>
    </select>
    <input type="checkbox" bind:checked={checked} />
    <input type="checkbox" bind:group={group} value="opt-11" />
    <input type="radio" bind:group={group} value="opt-11" />
    <input type="number" bind:value={counter} />
    <details bind:open={checked}><summary>More</summary>chunk 11</details>
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div {@attach (node) => { node.dataset.chunk = "11"; }}>attach target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <p transition:slide|global>global transition chunk 11</p>
    <p in:fly|local={{ y: 40 }}>local transition chunk 11</p>

    <div style:opacity style:color={state}>style shorthand</div>
    <div class="slider chunk-11 {title}" class:invinsible>literal class</div>
    <div class={`chunk-${i}-${counter}`}>template literal class</div>
    <span data-note="&copy;=value" title="&copy 1998" aria-label="a &amp; b">entities</span>

    <template id="chunk-tpl-11">
        {@html "<b>templated</b>"}
        {#if counter}<span>inside template</span>{/if}
    </template>

    <svg viewBox="0 0 10 10" class="chunk-svg">
        <clipPath id="clip-11"><rect width="4" height="4" /></clipPath>
        <linearGradient id="grad-11"><stop offset="0%" stop-color={state} /></linearGradient>
        <foreignObject width="10" height="10"><div>svg foreign chunk 11</div></foreignObject>
    </svg>

    <math><mrow><mi>x</mi><mo>+</mo><mn>{counter}</mn></mrow></math>
    <svelte:element this={state ? "div" : "span"} class="dynamic-11">
        Dynamic element chunk 11: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()}>
        <strong>Inline child chunk 11: {title}</strong>
        <div slot="footer">Footer chunk 11: {counter}</div>
    </ChildComponent>

    <ChildComponent --accent="var(--custom, #5c4634)" {...rest} bind:config>
        {#snippet header(label)}
            <em>Header snippet chunk 11: {label}</em>
        {/snippet}
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={pageHeading} />

    {#if depth > 0}
        <svelte:self depth={depth - 1} title={title} />
    {/if}

    {@render badge("chunk-11", "secondary")}
    {@render card(title, "Content for chunk 11")}
    {@render metricSummary({ label: title, values: [count, doubled, counter], meta: { id: propsId } })}
    {@render show?.()}

    <button onclick={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 11: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 11: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-12">
    Chunk 12: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {moduleSummary} | Store: {storeSummary} | Label: {$labelStore}</p>

    {@html "<b>raw html chunk 12</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-12"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 12.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 12.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 12.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 12: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = `${idx}:${item.name}`}
        <p {...rest} data-index="chunk-12-{idx}" animate:flip>{itemLabel}</p>
    {/each}

    {#each items}
        <span class="item-less">Repeated shell chunk 12</span>
    {/each}

    {#each items as item (item.id)}
        <p animate:flip={{ duration: 200 }}>{item.name}</p>
    {:else}
        <p>No items in chunk 12</p>
    {/each}

    {#await promise}
        <p>Loading chunk 12...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    {#await promise then quickValue}
        <p>Quick resolved: {quickValue}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value="opt-1">One</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
        <option value={`opt-${counter}`}>{title}</option>
    </select>
    <input type="checkbox" bind:checked={checked} />
    <input type="checkbox" bind:group={group} value="opt-12" />
    <input type="radio" bind:group={group} value="opt-12" />
    <input type="number" bind:value={counter} />
    <details bind:open={checked}><summary>More</summary>chunk 12</details>
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div {@attach (node) => { node.dataset.chunk = "12"; }}>attach target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <p transition:slide|global>global transition chunk 12</p>
    <p in:fly|local={{ y: 40 }}>local transition chunk 12</p>

    <div style:opacity style:color={state}>style shorthand</div>
    <div class="slider chunk-12 {title}" class:invinsible>literal class</div>
    <div class={`chunk-${i}-${counter}`}>template literal class</div>
    <span data-note="&copy;=value" title="&copy 1998" aria-label="a &amp; b">entities</span>

    <template id="chunk-tpl-12">
        {@html "<b>templated</b>"}
        {#if counter}<span>inside template</span>{/if}
    </template>

    <svg viewBox="0 0 10 10" class="chunk-svg">
        <clipPath id="clip-12"><rect width="4" height="4" /></clipPath>
        <linearGradient id="grad-12"><stop offset="0%" stop-color={state} /></linearGradient>
        <foreignObject width="10" height="10"><div>svg foreign chunk 12</div></foreignObject>
    </svg>

    <math><mrow><mi>x</mi><mo>+</mo><mn>{counter}</mn></mrow></math>
    <svelte:element this={state ? "div" : "span"} class="dynamic-12">
        Dynamic element chunk 12: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()}>
        <strong>Inline child chunk 12: {title}</strong>
        <div slot="footer">Footer chunk 12: {counter}</div>
    </ChildComponent>

    <ChildComponent --accent="var(--custom, #5c4634)" {...rest} bind:config>
        {#snippet header(label)}
            <em>Header snippet chunk 12: {label}</em>
        {/snippet}
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={pageHeading} />

    {#if depth > 0}
        <svelte:self depth={depth - 1} title={title} />
    {/if}

    {@render badge("chunk-12", "secondary")}
    {@render card(title, "Content for chunk 12")}
    {@render metricSummary({ label: title, values: [count, doubled, counter], meta: { id: propsId } })}
    {@render show?.()}

    <button onclick={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 12: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 12: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-13">
    Chunk 13: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {moduleSummary} | Store: {storeSummary} | Label: {$labelStore}</p>

    {@html "<b>raw html chunk 13</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-13"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 13.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 13.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 13.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 13: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = `${idx}:${item.name}`}
        <p {...rest} data-index="chunk-13-{idx}" animate:flip>{itemLabel}</p>
    {/each}

    {#each items}
        <span class="item-less">Repeated shell chunk 13</span>
    {/each}

    {#each items as item (item.id)}
        <p animate:flip={{ duration: 200 }}>{item.name}</p>
    {:else}
        <p>No items in chunk 13</p>
    {/each}

    {#await promise}
        <p>Loading chunk 13...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    {#await promise then quickValue}
        <p>Quick resolved: {quickValue}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value="opt-1">One</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
        <option value={`opt-${counter}`}>{title}</option>
    </select>
    <input type="checkbox" bind:checked={checked} />
    <input type="checkbox" bind:group={group} value="opt-13" />
    <input type="radio" bind:group={group} value="opt-13" />
    <input type="number" bind:value={counter} />
    <details bind:open={checked}><summary>More</summary>chunk 13</details>
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div {@attach (node) => { node.dataset.chunk = "13"; }}>attach target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <p transition:slide|global>global transition chunk 13</p>
    <p in:fly|local={{ y: 40 }}>local transition chunk 13</p>

    <div style:opacity style:color={state}>style shorthand</div>
    <div class="slider chunk-13 {title}" class:invinsible>literal class</div>
    <div class={`chunk-${i}-${counter}`}>template literal class</div>
    <span data-note="&copy;=value" title="&copy 1998" aria-label="a &amp; b">entities</span>

    <template id="chunk-tpl-13">
        {@html "<b>templated</b>"}
        {#if counter}<span>inside template</span>{/if}
    </template>

    <svg viewBox="0 0 10 10" class="chunk-svg">
        <clipPath id="clip-13"><rect width="4" height="4" /></clipPath>
        <linearGradient id="grad-13"><stop offset="0%" stop-color={state} /></linearGradient>
        <foreignObject width="10" height="10"><div>svg foreign chunk 13</div></foreignObject>
    </svg>

    <math><mrow><mi>x</mi><mo>+</mo><mn>{counter}</mn></mrow></math>
    <svelte:element this={state ? "div" : "span"} class="dynamic-13">
        Dynamic element chunk 13: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()}>
        <strong>Inline child chunk 13: {title}</strong>
        <div slot="footer">Footer chunk 13: {counter}</div>
    </ChildComponent>

    <ChildComponent --accent="var(--custom, #5c4634)" {...rest} bind:config>
        {#snippet header(label)}
            <em>Header snippet chunk 13: {label}</em>
        {/snippet}
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={pageHeading} />

    {#if depth > 0}
        <svelte:self depth={depth - 1} title={title} />
    {/if}

    {@render badge("chunk-13", "secondary")}
    {@render card(title, "Content for chunk 13")}
    {@render metricSummary({ label: title, values: [count, doubled, counter], meta: { id: propsId } })}
    {@render show?.()}

    <button onclick={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 13: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 13: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-14">
    Chunk 14: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {moduleSummary} | Store: {storeSummary} | Label: {$labelStore}</p>

    {@html "<b>raw html chunk 14</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-14"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 14.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 14.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 14.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 14: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = `${idx}:${item.name}`}
        <p {...rest} data-index="chunk-14-{idx}" animate:flip>{itemLabel}</p>
    {/each}

    {#each items}
        <span class="item-less">Repeated shell chunk 14</span>
    {/each}

    {#each items as item (item.id)}
        <p animate:flip={{ duration: 200 }}>{item.name}</p>
    {:else}
        <p>No items in chunk 14</p>
    {/each}

    {#await promise}
        <p>Loading chunk 14...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    {#await promise then quickValue}
        <p>Quick resolved: {quickValue}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value="opt-1">One</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
        <option value={`opt-${counter}`}>{title}</option>
    </select>
    <input type="checkbox" bind:checked={checked} />
    <input type="checkbox" bind:group={group} value="opt-14" />
    <input type="radio" bind:group={group} value="opt-14" />
    <input type="number" bind:value={counter} />
    <details bind:open={checked}><summary>More</summary>chunk 14</details>
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div {@attach (node) => { node.dataset.chunk = "14"; }}>attach target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <p transition:slide|global>global transition chunk 14</p>
    <p in:fly|local={{ y: 40 }}>local transition chunk 14</p>

    <div style:opacity style:color={state}>style shorthand</div>
    <div class="slider chunk-14 {title}" class:invinsible>literal class</div>
    <div class={`chunk-${i}-${counter}`}>template literal class</div>
    <span data-note="&copy;=value" title="&copy 1998" aria-label="a &amp; b">entities</span>

    <template id="chunk-tpl-14">
        {@html "<b>templated</b>"}
        {#if counter}<span>inside template</span>{/if}
    </template>

    <svg viewBox="0 0 10 10" class="chunk-svg">
        <clipPath id="clip-14"><rect width="4" height="4" /></clipPath>
        <linearGradient id="grad-14"><stop offset="0%" stop-color={state} /></linearGradient>
        <foreignObject width="10" height="10"><div>svg foreign chunk 14</div></foreignObject>
    </svg>

    <math><mrow><mi>x</mi><mo>+</mo><mn>{counter}</mn></mrow></math>
    <svelte:element this={state ? "div" : "span"} class="dynamic-14">
        Dynamic element chunk 14: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()}>
        <strong>Inline child chunk 14: {title}</strong>
        <div slot="footer">Footer chunk 14: {counter}</div>
    </ChildComponent>

    <ChildComponent --accent="var(--custom, #5c4634)" {...rest} bind:config>
        {#snippet header(label)}
            <em>Header snippet chunk 14: {label}</em>
        {/snippet}
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={pageHeading} />

    {#if depth > 0}
        <svelte:self depth={depth - 1} title={title} />
    {/if}

    {@render badge("chunk-14", "secondary")}
    {@render card(title, "Content for chunk 14")}
    {@render metricSummary({ label: title, values: [count, doubled, counter], meta: { id: propsId } })}
    {@render show?.()}

    <button onclick={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 14: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 14: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-15">
    Chunk 15: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {moduleSummary} | Store: {storeSummary} | Label: {$labelStore}</p>

    {@html "<b>raw html chunk 15</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-15"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 15.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 15.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 15.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 15: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = `${idx}:${item.name}`}
        <p {...rest} data-index="chunk-15-{idx}" animate:flip>{itemLabel}</p>
    {/each}

    {#each items}
        <span class="item-less">Repeated shell chunk 15</span>
    {/each}

    {#each items as item (item.id)}
        <p animate:flip={{ duration: 200 }}>{item.name}</p>
    {:else}
        <p>No items in chunk 15</p>
    {/each}

    {#await promise}
        <p>Loading chunk 15...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    {#await promise then quickValue}
        <p>Quick resolved: {quickValue}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value="opt-1">One</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
        <option value={`opt-${counter}`}>{title}</option>
    </select>
    <input type="checkbox" bind:checked={checked} />
    <input type="checkbox" bind:group={group} value="opt-15" />
    <input type="radio" bind:group={group} value="opt-15" />
    <input type="number" bind:value={counter} />
    <details bind:open={checked}><summary>More</summary>chunk 15</details>
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div {@attach (node) => { node.dataset.chunk = "15"; }}>attach target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <p transition:slide|global>global transition chunk 15</p>
    <p in:fly|local={{ y: 40 }}>local transition chunk 15</p>

    <div style:opacity style:color={state}>style shorthand</div>
    <div class="slider chunk-15 {title}" class:invinsible>literal class</div>
    <div class={`chunk-${i}-${counter}`}>template literal class</div>
    <span data-note="&copy;=value" title="&copy 1998" aria-label="a &amp; b">entities</span>

    <template id="chunk-tpl-15">
        {@html "<b>templated</b>"}
        {#if counter}<span>inside template</span>{/if}
    </template>

    <svg viewBox="0 0 10 10" class="chunk-svg">
        <clipPath id="clip-15"><rect width="4" height="4" /></clipPath>
        <linearGradient id="grad-15"><stop offset="0%" stop-color={state} /></linearGradient>
        <foreignObject width="10" height="10"><div>svg foreign chunk 15</div></foreignObject>
    </svg>

    <math><mrow><mi>x</mi><mo>+</mo><mn>{counter}</mn></mrow></math>
    <svelte:element this={state ? "div" : "span"} class="dynamic-15">
        Dynamic element chunk 15: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()}>
        <strong>Inline child chunk 15: {title}</strong>
        <div slot="footer">Footer chunk 15: {counter}</div>
    </ChildComponent>

    <ChildComponent --accent="var(--custom, #5c4634)" {...rest} bind:config>
        {#snippet header(label)}
            <em>Header snippet chunk 15: {label}</em>
        {/snippet}
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={pageHeading} />

    {#if depth > 0}
        <svelte:self depth={depth - 1} title={title} />
    {/if}

    {@render badge("chunk-15", "secondary")}
    {@render card(title, "Content for chunk 15")}
    {@render metricSummary({ label: title, values: [count, doubled, counter], meta: { id: propsId } })}
    {@render show?.()}

    <button onclick={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 15: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 15: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-16">
    Chunk 16: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {moduleSummary} | Store: {storeSummary} | Label: {$labelStore}</p>

    {@html "<b>raw html chunk 16</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-16"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 16.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 16.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 16.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 16: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = `${idx}:${item.name}`}
        <p {...rest} data-index="chunk-16-{idx}" animate:flip>{itemLabel}</p>
    {/each}

    {#each items}
        <span class="item-less">Repeated shell chunk 16</span>
    {/each}

    {#each items as item (item.id)}
        <p animate:flip={{ duration: 200 }}>{item.name}</p>
    {:else}
        <p>No items in chunk 16</p>
    {/each}

    {#await promise}
        <p>Loading chunk 16...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    {#await promise then quickValue}
        <p>Quick resolved: {quickValue}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value="opt-1">One</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
        <option value={`opt-${counter}`}>{title}</option>
    </select>
    <input type="checkbox" bind:checked={checked} />
    <input type="checkbox" bind:group={group} value="opt-16" />
    <input type="radio" bind:group={group} value="opt-16" />
    <input type="number" bind:value={counter} />
    <details bind:open={checked}><summary>More</summary>chunk 16</details>
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div {@attach (node) => { node.dataset.chunk = "16"; }}>attach target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <p transition:slide|global>global transition chunk 16</p>
    <p in:fly|local={{ y: 40 }}>local transition chunk 16</p>

    <div style:opacity style:color={state}>style shorthand</div>
    <div class="slider chunk-16 {title}" class:invinsible>literal class</div>
    <div class={`chunk-${i}-${counter}`}>template literal class</div>
    <span data-note="&copy;=value" title="&copy 1998" aria-label="a &amp; b">entities</span>

    <template id="chunk-tpl-16">
        {@html "<b>templated</b>"}
        {#if counter}<span>inside template</span>{/if}
    </template>

    <svg viewBox="0 0 10 10" class="chunk-svg">
        <clipPath id="clip-16"><rect width="4" height="4" /></clipPath>
        <linearGradient id="grad-16"><stop offset="0%" stop-color={state} /></linearGradient>
        <foreignObject width="10" height="10"><div>svg foreign chunk 16</div></foreignObject>
    </svg>

    <math><mrow><mi>x</mi><mo>+</mo><mn>{counter}</mn></mrow></math>
    <svelte:element this={state ? "div" : "span"} class="dynamic-16">
        Dynamic element chunk 16: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()}>
        <strong>Inline child chunk 16: {title}</strong>
        <div slot="footer">Footer chunk 16: {counter}</div>
    </ChildComponent>

    <ChildComponent --accent="var(--custom, #5c4634)" {...rest} bind:config>
        {#snippet header(label)}
            <em>Header snippet chunk 16: {label}</em>
        {/snippet}
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={pageHeading} />

    {#if depth > 0}
        <svelte:self depth={depth - 1} title={title} />
    {/if}

    {@render badge("chunk-16", "secondary")}
    {@render card(title, "Content for chunk 16")}
    {@render metricSummary({ label: title, values: [count, doubled, counter], meta: { id: propsId } })}
    {@render show?.()}

    <button onclick={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 16: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 16: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-17">
    Chunk 17: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {moduleSummary} | Store: {storeSummary} | Label: {$labelStore}</p>

    {@html "<b>raw html chunk 17</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-17"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 17.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 17.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 17.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 17: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = `${idx}:${item.name}`}
        <p {...rest} data-index="chunk-17-{idx}" animate:flip>{itemLabel}</p>
    {/each}

    {#each items}
        <span class="item-less">Repeated shell chunk 17</span>
    {/each}

    {#each items as item (item.id)}
        <p animate:flip={{ duration: 200 }}>{item.name}</p>
    {:else}
        <p>No items in chunk 17</p>
    {/each}

    {#await promise}
        <p>Loading chunk 17...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    {#await promise then quickValue}
        <p>Quick resolved: {quickValue}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value="opt-1">One</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
        <option value={`opt-${counter}`}>{title}</option>
    </select>
    <input type="checkbox" bind:checked={checked} />
    <input type="checkbox" bind:group={group} value="opt-17" />
    <input type="radio" bind:group={group} value="opt-17" />
    <input type="number" bind:value={counter} />
    <details bind:open={checked}><summary>More</summary>chunk 17</details>
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div {@attach (node) => { node.dataset.chunk = "17"; }}>attach target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <p transition:slide|global>global transition chunk 17</p>
    <p in:fly|local={{ y: 40 }}>local transition chunk 17</p>

    <div style:opacity style:color={state}>style shorthand</div>
    <div class="slider chunk-17 {title}" class:invinsible>literal class</div>
    <div class={`chunk-${i}-${counter}`}>template literal class</div>
    <span data-note="&copy;=value" title="&copy 1998" aria-label="a &amp; b">entities</span>

    <template id="chunk-tpl-17">
        {@html "<b>templated</b>"}
        {#if counter}<span>inside template</span>{/if}
    </template>

    <svg viewBox="0 0 10 10" class="chunk-svg">
        <clipPath id="clip-17"><rect width="4" height="4" /></clipPath>
        <linearGradient id="grad-17"><stop offset="0%" stop-color={state} /></linearGradient>
        <foreignObject width="10" height="10"><div>svg foreign chunk 17</div></foreignObject>
    </svg>

    <math><mrow><mi>x</mi><mo>+</mo><mn>{counter}</mn></mrow></math>
    <svelte:element this={state ? "div" : "span"} class="dynamic-17">
        Dynamic element chunk 17: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()}>
        <strong>Inline child chunk 17: {title}</strong>
        <div slot="footer">Footer chunk 17: {counter}</div>
    </ChildComponent>

    <ChildComponent --accent="var(--custom, #5c4634)" {...rest} bind:config>
        {#snippet header(label)}
            <em>Header snippet chunk 17: {label}</em>
        {/snippet}
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={pageHeading} />

    {#if depth > 0}
        <svelte:self depth={depth - 1} title={title} />
    {/if}

    {@render badge("chunk-17", "secondary")}
    {@render card(title, "Content for chunk 17")}
    {@render metricSummary({ label: title, values: [count, doubled, counter], meta: { id: propsId } })}
    {@render show?.()}

    <button onclick={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 17: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 17: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-18">
    Chunk 18: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {moduleSummary} | Store: {storeSummary} | Label: {$labelStore}</p>

    {@html "<b>raw html chunk 18</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-18"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 18.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 18.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 18.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 18: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = `${idx}:${item.name}`}
        <p {...rest} data-index="chunk-18-{idx}" animate:flip>{itemLabel}</p>
    {/each}

    {#each items}
        <span class="item-less">Repeated shell chunk 18</span>
    {/each}

    {#each items as item (item.id)}
        <p animate:flip={{ duration: 200 }}>{item.name}</p>
    {:else}
        <p>No items in chunk 18</p>
    {/each}

    {#await promise}
        <p>Loading chunk 18...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    {#await promise then quickValue}
        <p>Quick resolved: {quickValue}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value="opt-1">One</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
        <option value={`opt-${counter}`}>{title}</option>
    </select>
    <input type="checkbox" bind:checked={checked} />
    <input type="checkbox" bind:group={group} value="opt-18" />
    <input type="radio" bind:group={group} value="opt-18" />
    <input type="number" bind:value={counter} />
    <details bind:open={checked}><summary>More</summary>chunk 18</details>
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div {@attach (node) => { node.dataset.chunk = "18"; }}>attach target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <p transition:slide|global>global transition chunk 18</p>
    <p in:fly|local={{ y: 40 }}>local transition chunk 18</p>

    <div style:opacity style:color={state}>style shorthand</div>
    <div class="slider chunk-18 {title}" class:invinsible>literal class</div>
    <div class={`chunk-${i}-${counter}`}>template literal class</div>
    <span data-note="&copy;=value" title="&copy 1998" aria-label="a &amp; b">entities</span>

    <template id="chunk-tpl-18">
        {@html "<b>templated</b>"}
        {#if counter}<span>inside template</span>{/if}
    </template>

    <svg viewBox="0 0 10 10" class="chunk-svg">
        <clipPath id="clip-18"><rect width="4" height="4" /></clipPath>
        <linearGradient id="grad-18"><stop offset="0%" stop-color={state} /></linearGradient>
        <foreignObject width="10" height="10"><div>svg foreign chunk 18</div></foreignObject>
    </svg>

    <math><mrow><mi>x</mi><mo>+</mo><mn>{counter}</mn></mrow></math>
    <svelte:element this={state ? "div" : "span"} class="dynamic-18">
        Dynamic element chunk 18: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()}>
        <strong>Inline child chunk 18: {title}</strong>
        <div slot="footer">Footer chunk 18: {counter}</div>
    </ChildComponent>

    <ChildComponent --accent="var(--custom, #5c4634)" {...rest} bind:config>
        {#snippet header(label)}
            <em>Header snippet chunk 18: {label}</em>
        {/snippet}
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={pageHeading} />

    {#if depth > 0}
        <svelte:self depth={depth - 1} title={title} />
    {/if}

    {@render badge("chunk-18", "secondary")}
    {@render card(title, "Content for chunk 18")}
    {@render metricSummary({ label: title, values: [count, doubled, counter], meta: { id: propsId } })}
    {@render show?.()}

    <button onclick={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 18: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 18: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-19">
    Chunk 19: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {moduleSummary} | Store: {storeSummary} | Label: {$labelStore}</p>

    {@html "<b>raw html chunk 19</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-19"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 19.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 19.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 19.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 19: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = `${idx}:${item.name}`}
        <p {...rest} data-index="chunk-19-{idx}" animate:flip>{itemLabel}</p>
    {/each}

    {#each items}
        <span class="item-less">Repeated shell chunk 19</span>
    {/each}

    {#each items as item (item.id)}
        <p animate:flip={{ duration: 200 }}>{item.name}</p>
    {:else}
        <p>No items in chunk 19</p>
    {/each}

    {#await promise}
        <p>Loading chunk 19...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    {#await promise then quickValue}
        <p>Quick resolved: {quickValue}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value="opt-1">One</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
        <option value={`opt-${counter}`}>{title}</option>
    </select>
    <input type="checkbox" bind:checked={checked} />
    <input type="checkbox" bind:group={group} value="opt-19" />
    <input type="radio" bind:group={group} value="opt-19" />
    <input type="number" bind:value={counter} />
    <details bind:open={checked}><summary>More</summary>chunk 19</details>
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div {@attach (node) => { node.dataset.chunk = "19"; }}>attach target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <p transition:slide|global>global transition chunk 19</p>
    <p in:fly|local={{ y: 40 }}>local transition chunk 19</p>

    <div style:opacity style:color={state}>style shorthand</div>
    <div class="slider chunk-19 {title}" class:invinsible>literal class</div>
    <div class={`chunk-${i}-${counter}`}>template literal class</div>
    <span data-note="&copy;=value" title="&copy 1998" aria-label="a &amp; b">entities</span>

    <template id="chunk-tpl-19">
        {@html "<b>templated</b>"}
        {#if counter}<span>inside template</span>{/if}
    </template>

    <svg viewBox="0 0 10 10" class="chunk-svg">
        <clipPath id="clip-19"><rect width="4" height="4" /></clipPath>
        <linearGradient id="grad-19"><stop offset="0%" stop-color={state} /></linearGradient>
        <foreignObject width="10" height="10"><div>svg foreign chunk 19</div></foreignObject>
    </svg>

    <math><mrow><mi>x</mi><mo>+</mo><mn>{counter}</mn></mrow></math>
    <svelte:element this={state ? "div" : "span"} class="dynamic-19">
        Dynamic element chunk 19: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()}>
        <strong>Inline child chunk 19: {title}</strong>
        <div slot="footer">Footer chunk 19: {counter}</div>
    </ChildComponent>

    <ChildComponent --accent="var(--custom, #5c4634)" {...rest} bind:config>
        {#snippet header(label)}
            <em>Header snippet chunk 19: {label}</em>
        {/snippet}
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={pageHeading} />

    {#if depth > 0}
        <svelte:self depth={depth - 1} title={title} />
    {/if}

    {@render badge("chunk-19", "secondary")}
    {@render card(title, "Content for chunk 19")}
    {@render metricSummary({ label: title, values: [count, doubled, counter], meta: { id: propsId } })}
    {@render show?.()}

    <button onclick={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 19: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 19: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-20">
    Chunk 20: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {moduleSummary} | Store: {storeSummary} | Label: {$labelStore}</p>

    {@html "<b>raw html chunk 20</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-20"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 20.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 20.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 20.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 20: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = `${idx}:${item.name}`}
        <p {...rest} data-index="chunk-20-{idx}" animate:flip>{itemLabel}</p>
    {/each}

    {#each items}
        <span class="item-less">Repeated shell chunk 20</span>
    {/each}

    {#each items as item (item.id)}
        <p animate:flip={{ duration: 200 }}>{item.name}</p>
    {:else}
        <p>No items in chunk 20</p>
    {/each}

    {#await promise}
        <p>Loading chunk 20...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    {#await promise then quickValue}
        <p>Quick resolved: {quickValue}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value="opt-1">One</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
        <option value={`opt-${counter}`}>{title}</option>
    </select>
    <input type="checkbox" bind:checked={checked} />
    <input type="checkbox" bind:group={group} value="opt-20" />
    <input type="radio" bind:group={group} value="opt-20" />
    <input type="number" bind:value={counter} />
    <details bind:open={checked}><summary>More</summary>chunk 20</details>
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div {@attach (node) => { node.dataset.chunk = "20"; }}>attach target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <p transition:slide|global>global transition chunk 20</p>
    <p in:fly|local={{ y: 40 }}>local transition chunk 20</p>

    <div style:opacity style:color={state}>style shorthand</div>
    <div class="slider chunk-20 {title}" class:invinsible>literal class</div>
    <div class={`chunk-${i}-${counter}`}>template literal class</div>
    <span data-note="&copy;=value" title="&copy 1998" aria-label="a &amp; b">entities</span>

    <template id="chunk-tpl-20">
        {@html "<b>templated</b>"}
        {#if counter}<span>inside template</span>{/if}
    </template>

    <svg viewBox="0 0 10 10" class="chunk-svg">
        <clipPath id="clip-20"><rect width="4" height="4" /></clipPath>
        <linearGradient id="grad-20"><stop offset="0%" stop-color={state} /></linearGradient>
        <foreignObject width="10" height="10"><div>svg foreign chunk 20</div></foreignObject>
    </svg>

    <math><mrow><mi>x</mi><mo>+</mo><mn>{counter}</mn></mrow></math>
    <svelte:element this={state ? "div" : "span"} class="dynamic-20">
        Dynamic element chunk 20: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()}>
        <strong>Inline child chunk 20: {title}</strong>
        <div slot="footer">Footer chunk 20: {counter}</div>
    </ChildComponent>

    <ChildComponent --accent="var(--custom, #5c4634)" {...rest} bind:config>
        {#snippet header(label)}
            <em>Header snippet chunk 20: {label}</em>
        {/snippet}
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={pageHeading} />

    {#if depth > 0}
        <svelte:self depth={depth - 1} title={title} />
    {/if}

    {@render badge("chunk-20", "secondary")}
    {@render card(title, "Content for chunk 20")}
    {@render metricSummary({ label: title, values: [count, doubled, counter], meta: { id: propsId } })}
    {@render show?.()}

    <button onclick={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 20: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 20: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-21">
    Chunk 21: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {moduleSummary} | Store: {storeSummary} | Label: {$labelStore}</p>

    {@html "<b>raw html chunk 21</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-21"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 21.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 21.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 21.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 21: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = `${idx}:${item.name}`}
        <p {...rest} data-index="chunk-21-{idx}" animate:flip>{itemLabel}</p>
    {/each}

    {#each items}
        <span class="item-less">Repeated shell chunk 21</span>
    {/each}

    {#each items as item (item.id)}
        <p animate:flip={{ duration: 200 }}>{item.name}</p>
    {:else}
        <p>No items in chunk 21</p>
    {/each}

    {#await promise}
        <p>Loading chunk 21...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    {#await promise then quickValue}
        <p>Quick resolved: {quickValue}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value="opt-1">One</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
        <option value={`opt-${counter}`}>{title}</option>
    </select>
    <input type="checkbox" bind:checked={checked} />
    <input type="checkbox" bind:group={group} value="opt-21" />
    <input type="radio" bind:group={group} value="opt-21" />
    <input type="number" bind:value={counter} />
    <details bind:open={checked}><summary>More</summary>chunk 21</details>
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div {@attach (node) => { node.dataset.chunk = "21"; }}>attach target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <p transition:slide|global>global transition chunk 21</p>
    <p in:fly|local={{ y: 40 }}>local transition chunk 21</p>

    <div style:opacity style:color={state}>style shorthand</div>
    <div class="slider chunk-21 {title}" class:invinsible>literal class</div>
    <div class={`chunk-${i}-${counter}`}>template literal class</div>
    <span data-note="&copy;=value" title="&copy 1998" aria-label="a &amp; b">entities</span>

    <template id="chunk-tpl-21">
        {@html "<b>templated</b>"}
        {#if counter}<span>inside template</span>{/if}
    </template>

    <svg viewBox="0 0 10 10" class="chunk-svg">
        <clipPath id="clip-21"><rect width="4" height="4" /></clipPath>
        <linearGradient id="grad-21"><stop offset="0%" stop-color={state} /></linearGradient>
        <foreignObject width="10" height="10"><div>svg foreign chunk 21</div></foreignObject>
    </svg>

    <math><mrow><mi>x</mi><mo>+</mo><mn>{counter}</mn></mrow></math>
    <svelte:element this={state ? "div" : "span"} class="dynamic-21">
        Dynamic element chunk 21: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()}>
        <strong>Inline child chunk 21: {title}</strong>
        <div slot="footer">Footer chunk 21: {counter}</div>
    </ChildComponent>

    <ChildComponent --accent="var(--custom, #5c4634)" {...rest} bind:config>
        {#snippet header(label)}
            <em>Header snippet chunk 21: {label}</em>
        {/snippet}
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={pageHeading} />

    {#if depth > 0}
        <svelte:self depth={depth - 1} title={title} />
    {/if}

    {@render badge("chunk-21", "secondary")}
    {@render card(title, "Content for chunk 21")}
    {@render metricSummary({ label: title, values: [count, doubled, counter], meta: { id: propsId } })}
    {@render show?.()}

    <button onclick={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 21: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 21: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-22">
    Chunk 22: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {moduleSummary} | Store: {storeSummary} | Label: {$labelStore}</p>

    {@html "<b>raw html chunk 22</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-22"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 22.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 22.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 22.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 22: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = `${idx}:${item.name}`}
        <p {...rest} data-index="chunk-22-{idx}" animate:flip>{itemLabel}</p>
    {/each}

    {#each items}
        <span class="item-less">Repeated shell chunk 22</span>
    {/each}

    {#each items as item (item.id)}
        <p animate:flip={{ duration: 200 }}>{item.name}</p>
    {:else}
        <p>No items in chunk 22</p>
    {/each}

    {#await promise}
        <p>Loading chunk 22...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    {#await promise then quickValue}
        <p>Quick resolved: {quickValue}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value="opt-1">One</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
        <option value={`opt-${counter}`}>{title}</option>
    </select>
    <input type="checkbox" bind:checked={checked} />
    <input type="checkbox" bind:group={group} value="opt-22" />
    <input type="radio" bind:group={group} value="opt-22" />
    <input type="number" bind:value={counter} />
    <details bind:open={checked}><summary>More</summary>chunk 22</details>
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div {@attach (node) => { node.dataset.chunk = "22"; }}>attach target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <p transition:slide|global>global transition chunk 22</p>
    <p in:fly|local={{ y: 40 }}>local transition chunk 22</p>

    <div style:opacity style:color={state}>style shorthand</div>
    <div class="slider chunk-22 {title}" class:invinsible>literal class</div>
    <div class={`chunk-${i}-${counter}`}>template literal class</div>
    <span data-note="&copy;=value" title="&copy 1998" aria-label="a &amp; b">entities</span>

    <template id="chunk-tpl-22">
        {@html "<b>templated</b>"}
        {#if counter}<span>inside template</span>{/if}
    </template>

    <svg viewBox="0 0 10 10" class="chunk-svg">
        <clipPath id="clip-22"><rect width="4" height="4" /></clipPath>
        <linearGradient id="grad-22"><stop offset="0%" stop-color={state} /></linearGradient>
        <foreignObject width="10" height="10"><div>svg foreign chunk 22</div></foreignObject>
    </svg>

    <math><mrow><mi>x</mi><mo>+</mo><mn>{counter}</mn></mrow></math>
    <svelte:element this={state ? "div" : "span"} class="dynamic-22">
        Dynamic element chunk 22: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()}>
        <strong>Inline child chunk 22: {title}</strong>
        <div slot="footer">Footer chunk 22: {counter}</div>
    </ChildComponent>

    <ChildComponent --accent="var(--custom, #5c4634)" {...rest} bind:config>
        {#snippet header(label)}
            <em>Header snippet chunk 22: {label}</em>
        {/snippet}
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={pageHeading} />

    {#if depth > 0}
        <svelte:self depth={depth - 1} title={title} />
    {/if}

    {@render badge("chunk-22", "secondary")}
    {@render card(title, "Content for chunk 22")}
    {@render metricSummary({ label: title, values: [count, doubled, counter], meta: { id: propsId } })}
    {@render show?.()}

    <button onclick={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 22: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 22: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-23">
    Chunk 23: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {moduleSummary} | Store: {storeSummary} | Label: {$labelStore}</p>

    {@html "<b>raw html chunk 23</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-23"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 23.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 23.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 23.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 23: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = `${idx}:${item.name}`}
        <p {...rest} data-index="chunk-23-{idx}" animate:flip>{itemLabel}</p>
    {/each}

    {#each items}
        <span class="item-less">Repeated shell chunk 23</span>
    {/each}

    {#each items as item (item.id)}
        <p animate:flip={{ duration: 200 }}>{item.name}</p>
    {:else}
        <p>No items in chunk 23</p>
    {/each}

    {#await promise}
        <p>Loading chunk 23...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    {#await promise then quickValue}
        <p>Quick resolved: {quickValue}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value="opt-1">One</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
        <option value={`opt-${counter}`}>{title}</option>
    </select>
    <input type="checkbox" bind:checked={checked} />
    <input type="checkbox" bind:group={group} value="opt-23" />
    <input type="radio" bind:group={group} value="opt-23" />
    <input type="number" bind:value={counter} />
    <details bind:open={checked}><summary>More</summary>chunk 23</details>
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div {@attach (node) => { node.dataset.chunk = "23"; }}>attach target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <p transition:slide|global>global transition chunk 23</p>
    <p in:fly|local={{ y: 40 }}>local transition chunk 23</p>

    <div style:opacity style:color={state}>style shorthand</div>
    <div class="slider chunk-23 {title}" class:invinsible>literal class</div>
    <div class={`chunk-${i}-${counter}`}>template literal class</div>
    <span data-note="&copy;=value" title="&copy 1998" aria-label="a &amp; b">entities</span>

    <template id="chunk-tpl-23">
        {@html "<b>templated</b>"}
        {#if counter}<span>inside template</span>{/if}
    </template>

    <svg viewBox="0 0 10 10" class="chunk-svg">
        <clipPath id="clip-23"><rect width="4" height="4" /></clipPath>
        <linearGradient id="grad-23"><stop offset="0%" stop-color={state} /></linearGradient>
        <foreignObject width="10" height="10"><div>svg foreign chunk 23</div></foreignObject>
    </svg>

    <math><mrow><mi>x</mi><mo>+</mo><mn>{counter}</mn></mrow></math>
    <svelte:element this={state ? "div" : "span"} class="dynamic-23">
        Dynamic element chunk 23: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()}>
        <strong>Inline child chunk 23: {title}</strong>
        <div slot="footer">Footer chunk 23: {counter}</div>
    </ChildComponent>

    <ChildComponent --accent="var(--custom, #5c4634)" {...rest} bind:config>
        {#snippet header(label)}
            <em>Header snippet chunk 23: {label}</em>
        {/snippet}
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={pageHeading} />

    {#if depth > 0}
        <svelte:self depth={depth - 1} title={title} />
    {/if}

    {@render badge("chunk-23", "secondary")}
    {@render card(title, "Content for chunk 23")}
    {@render metricSummary({ label: title, values: [count, doubled, counter], meta: { id: propsId } })}
    {@render show?.()}

    <button onclick={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 23: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 23: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-24">
    Chunk 24: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {moduleSummary} | Store: {storeSummary} | Label: {$labelStore}</p>

    {@html "<b>raw html chunk 24</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-24"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 24.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 24.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 24.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 24: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = `${idx}:${item.name}`}
        <p {...rest} data-index="chunk-24-{idx}" animate:flip>{itemLabel}</p>
    {/each}

    {#each items}
        <span class="item-less">Repeated shell chunk 24</span>
    {/each}

    {#each items as item (item.id)}
        <p animate:flip={{ duration: 200 }}>{item.name}</p>
    {:else}
        <p>No items in chunk 24</p>
    {/each}

    {#await promise}
        <p>Loading chunk 24...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    {#await promise then quickValue}
        <p>Quick resolved: {quickValue}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value="opt-1">One</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
        <option value={`opt-${counter}`}>{title}</option>
    </select>
    <input type="checkbox" bind:checked={checked} />
    <input type="checkbox" bind:group={group} value="opt-24" />
    <input type="radio" bind:group={group} value="opt-24" />
    <input type="number" bind:value={counter} />
    <details bind:open={checked}><summary>More</summary>chunk 24</details>
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div {@attach (node) => { node.dataset.chunk = "24"; }}>attach target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <p transition:slide|global>global transition chunk 24</p>
    <p in:fly|local={{ y: 40 }}>local transition chunk 24</p>

    <div style:opacity style:color={state}>style shorthand</div>
    <div class="slider chunk-24 {title}" class:invinsible>literal class</div>
    <div class={`chunk-${i}-${counter}`}>template literal class</div>
    <span data-note="&copy;=value" title="&copy 1998" aria-label="a &amp; b">entities</span>

    <template id="chunk-tpl-24">
        {@html "<b>templated</b>"}
        {#if counter}<span>inside template</span>{/if}
    </template>

    <svg viewBox="0 0 10 10" class="chunk-svg">
        <clipPath id="clip-24"><rect width="4" height="4" /></clipPath>
        <linearGradient id="grad-24"><stop offset="0%" stop-color={state} /></linearGradient>
        <foreignObject width="10" height="10"><div>svg foreign chunk 24</div></foreignObject>
    </svg>

    <math><mrow><mi>x</mi><mo>+</mo><mn>{counter}</mn></mrow></math>
    <svelte:element this={state ? "div" : "span"} class="dynamic-24">
        Dynamic element chunk 24: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()}>
        <strong>Inline child chunk 24: {title}</strong>
        <div slot="footer">Footer chunk 24: {counter}</div>
    </ChildComponent>

    <ChildComponent --accent="var(--custom, #5c4634)" {...rest} bind:config>
        {#snippet header(label)}
            <em>Header snippet chunk 24: {label}</em>
        {/snippet}
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={pageHeading} />

    {#if depth > 0}
        <svelte:self depth={depth - 1} title={title} />
    {/if}

    {@render badge("chunk-24", "secondary")}
    {@render card(title, "Content for chunk 24")}
    {@render metricSummary({ label: title, values: [count, doubled, counter], meta: { id: propsId } })}
    {@render show?.()}

    <button onclick={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 24: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 24: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-25">
    Chunk 25: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {moduleSummary} | Store: {storeSummary} | Label: {$labelStore}</p>

    {@html "<b>raw html chunk 25</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-25"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 25.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 25.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 25.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 25: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = `${idx}:${item.name}`}
        <p {...rest} data-index="chunk-25-{idx}" animate:flip>{itemLabel}</p>
    {/each}

    {#each items}
        <span class="item-less">Repeated shell chunk 25</span>
    {/each}

    {#each items as item (item.id)}
        <p animate:flip={{ duration: 200 }}>{item.name}</p>
    {:else}
        <p>No items in chunk 25</p>
    {/each}

    {#await promise}
        <p>Loading chunk 25...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    {#await promise then quickValue}
        <p>Quick resolved: {quickValue}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value="opt-1">One</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
        <option value={`opt-${counter}`}>{title}</option>
    </select>
    <input type="checkbox" bind:checked={checked} />
    <input type="checkbox" bind:group={group} value="opt-25" />
    <input type="radio" bind:group={group} value="opt-25" />
    <input type="number" bind:value={counter} />
    <details bind:open={checked}><summary>More</summary>chunk 25</details>
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div {@attach (node) => { node.dataset.chunk = "25"; }}>attach target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <p transition:slide|global>global transition chunk 25</p>
    <p in:fly|local={{ y: 40 }}>local transition chunk 25</p>

    <div style:opacity style:color={state}>style shorthand</div>
    <div class="slider chunk-25 {title}" class:invinsible>literal class</div>
    <div class={`chunk-${i}-${counter}`}>template literal class</div>
    <span data-note="&copy;=value" title="&copy 1998" aria-label="a &amp; b">entities</span>

    <template id="chunk-tpl-25">
        {@html "<b>templated</b>"}
        {#if counter}<span>inside template</span>{/if}
    </template>

    <svg viewBox="0 0 10 10" class="chunk-svg">
        <clipPath id="clip-25"><rect width="4" height="4" /></clipPath>
        <linearGradient id="grad-25"><stop offset="0%" stop-color={state} /></linearGradient>
        <foreignObject width="10" height="10"><div>svg foreign chunk 25</div></foreignObject>
    </svg>

    <math><mrow><mi>x</mi><mo>+</mo><mn>{counter}</mn></mrow></math>
    <svelte:element this={state ? "div" : "span"} class="dynamic-25">
        Dynamic element chunk 25: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()}>
        <strong>Inline child chunk 25: {title}</strong>
        <div slot="footer">Footer chunk 25: {counter}</div>
    </ChildComponent>

    <ChildComponent --accent="var(--custom, #5c4634)" {...rest} bind:config>
        {#snippet header(label)}
            <em>Header snippet chunk 25: {label}</em>
        {/snippet}
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={pageHeading} />

    {#if depth > 0}
        <svelte:self depth={depth - 1} title={title} />
    {/if}

    {@render badge("chunk-25", "secondary")}
    {@render card(title, "Content for chunk 25")}
    {@render metricSummary({ label: title, values: [count, doubled, counter], meta: { id: propsId } })}
    {@render show?.()}

    <button onclick={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 25: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 25: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-26">
    Chunk 26: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {moduleSummary} | Store: {storeSummary} | Label: {$labelStore}</p>

    {@html "<b>raw html chunk 26</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-26"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 26.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 26.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 26.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 26: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = `${idx}:${item.name}`}
        <p {...rest} data-index="chunk-26-{idx}" animate:flip>{itemLabel}</p>
    {/each}

    {#each items}
        <span class="item-less">Repeated shell chunk 26</span>
    {/each}

    {#each items as item (item.id)}
        <p animate:flip={{ duration: 200 }}>{item.name}</p>
    {:else}
        <p>No items in chunk 26</p>
    {/each}

    {#await promise}
        <p>Loading chunk 26...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    {#await promise then quickValue}
        <p>Quick resolved: {quickValue}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value="opt-1">One</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
        <option value={`opt-${counter}`}>{title}</option>
    </select>
    <input type="checkbox" bind:checked={checked} />
    <input type="checkbox" bind:group={group} value="opt-26" />
    <input type="radio" bind:group={group} value="opt-26" />
    <input type="number" bind:value={counter} />
    <details bind:open={checked}><summary>More</summary>chunk 26</details>
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div {@attach (node) => { node.dataset.chunk = "26"; }}>attach target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <p transition:slide|global>global transition chunk 26</p>
    <p in:fly|local={{ y: 40 }}>local transition chunk 26</p>

    <div style:opacity style:color={state}>style shorthand</div>
    <div class="slider chunk-26 {title}" class:invinsible>literal class</div>
    <div class={`chunk-${i}-${counter}`}>template literal class</div>
    <span data-note="&copy;=value" title="&copy 1998" aria-label="a &amp; b">entities</span>

    <template id="chunk-tpl-26">
        {@html "<b>templated</b>"}
        {#if counter}<span>inside template</span>{/if}
    </template>

    <svg viewBox="0 0 10 10" class="chunk-svg">
        <clipPath id="clip-26"><rect width="4" height="4" /></clipPath>
        <linearGradient id="grad-26"><stop offset="0%" stop-color={state} /></linearGradient>
        <foreignObject width="10" height="10"><div>svg foreign chunk 26</div></foreignObject>
    </svg>

    <math><mrow><mi>x</mi><mo>+</mo><mn>{counter}</mn></mrow></math>
    <svelte:element this={state ? "div" : "span"} class="dynamic-26">
        Dynamic element chunk 26: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()}>
        <strong>Inline child chunk 26: {title}</strong>
        <div slot="footer">Footer chunk 26: {counter}</div>
    </ChildComponent>

    <ChildComponent --accent="var(--custom, #5c4634)" {...rest} bind:config>
        {#snippet header(label)}
            <em>Header snippet chunk 26: {label}</em>
        {/snippet}
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={pageHeading} />

    {#if depth > 0}
        <svelte:self depth={depth - 1} title={title} />
    {/if}

    {@render badge("chunk-26", "secondary")}
    {@render card(title, "Content for chunk 26")}
    {@render metricSummary({ label: title, values: [count, doubled, counter], meta: { id: propsId } })}
    {@render show?.()}

    <button onclick={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 26: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 26: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-27">
    Chunk 27: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {moduleSummary} | Store: {storeSummary} | Label: {$labelStore}</p>

    {@html "<b>raw html chunk 27</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-27"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 27.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 27.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 27.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 27: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = `${idx}:${item.name}`}
        <p {...rest} data-index="chunk-27-{idx}" animate:flip>{itemLabel}</p>
    {/each}

    {#each items}
        <span class="item-less">Repeated shell chunk 27</span>
    {/each}

    {#each items as item (item.id)}
        <p animate:flip={{ duration: 200 }}>{item.name}</p>
    {:else}
        <p>No items in chunk 27</p>
    {/each}

    {#await promise}
        <p>Loading chunk 27...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    {#await promise then quickValue}
        <p>Quick resolved: {quickValue}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value="opt-1">One</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
        <option value={`opt-${counter}`}>{title}</option>
    </select>
    <input type="checkbox" bind:checked={checked} />
    <input type="checkbox" bind:group={group} value="opt-27" />
    <input type="radio" bind:group={group} value="opt-27" />
    <input type="number" bind:value={counter} />
    <details bind:open={checked}><summary>More</summary>chunk 27</details>
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div {@attach (node) => { node.dataset.chunk = "27"; }}>attach target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <p transition:slide|global>global transition chunk 27</p>
    <p in:fly|local={{ y: 40 }}>local transition chunk 27</p>

    <div style:opacity style:color={state}>style shorthand</div>
    <div class="slider chunk-27 {title}" class:invinsible>literal class</div>
    <div class={`chunk-${i}-${counter}`}>template literal class</div>
    <span data-note="&copy;=value" title="&copy 1998" aria-label="a &amp; b">entities</span>

    <template id="chunk-tpl-27">
        {@html "<b>templated</b>"}
        {#if counter}<span>inside template</span>{/if}
    </template>

    <svg viewBox="0 0 10 10" class="chunk-svg">
        <clipPath id="clip-27"><rect width="4" height="4" /></clipPath>
        <linearGradient id="grad-27"><stop offset="0%" stop-color={state} /></linearGradient>
        <foreignObject width="10" height="10"><div>svg foreign chunk 27</div></foreignObject>
    </svg>

    <math><mrow><mi>x</mi><mo>+</mo><mn>{counter}</mn></mrow></math>
    <svelte:element this={state ? "div" : "span"} class="dynamic-27">
        Dynamic element chunk 27: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()}>
        <strong>Inline child chunk 27: {title}</strong>
        <div slot="footer">Footer chunk 27: {counter}</div>
    </ChildComponent>

    <ChildComponent --accent="var(--custom, #5c4634)" {...rest} bind:config>
        {#snippet header(label)}
            <em>Header snippet chunk 27: {label}</em>
        {/snippet}
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={pageHeading} />

    {#if depth > 0}
        <svelte:self depth={depth - 1} title={title} />
    {/if}

    {@render badge("chunk-27", "secondary")}
    {@render card(title, "Content for chunk 27")}
    {@render metricSummary({ label: title, values: [count, doubled, counter], meta: { id: propsId } })}
    {@render show?.()}

    <button onclick={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 27: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 27: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-28">
    Chunk 28: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {moduleSummary} | Store: {storeSummary} | Label: {$labelStore}</p>

    {@html "<b>raw html chunk 28</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-28"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 28.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 28.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 28.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 28: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = `${idx}:${item.name}`}
        <p {...rest} data-index="chunk-28-{idx}" animate:flip>{itemLabel}</p>
    {/each}

    {#each items}
        <span class="item-less">Repeated shell chunk 28</span>
    {/each}

    {#each items as item (item.id)}
        <p animate:flip={{ duration: 200 }}>{item.name}</p>
    {:else}
        <p>No items in chunk 28</p>
    {/each}

    {#await promise}
        <p>Loading chunk 28...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    {#await promise then quickValue}
        <p>Quick resolved: {quickValue}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value="opt-1">One</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
        <option value={`opt-${counter}`}>{title}</option>
    </select>
    <input type="checkbox" bind:checked={checked} />
    <input type="checkbox" bind:group={group} value="opt-28" />
    <input type="radio" bind:group={group} value="opt-28" />
    <input type="number" bind:value={counter} />
    <details bind:open={checked}><summary>More</summary>chunk 28</details>
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div {@attach (node) => { node.dataset.chunk = "28"; }}>attach target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <p transition:slide|global>global transition chunk 28</p>
    <p in:fly|local={{ y: 40 }}>local transition chunk 28</p>

    <div style:opacity style:color={state}>style shorthand</div>
    <div class="slider chunk-28 {title}" class:invinsible>literal class</div>
    <div class={`chunk-${i}-${counter}`}>template literal class</div>
    <span data-note="&copy;=value" title="&copy 1998" aria-label="a &amp; b">entities</span>

    <template id="chunk-tpl-28">
        {@html "<b>templated</b>"}
        {#if counter}<span>inside template</span>{/if}
    </template>

    <svg viewBox="0 0 10 10" class="chunk-svg">
        <clipPath id="clip-28"><rect width="4" height="4" /></clipPath>
        <linearGradient id="grad-28"><stop offset="0%" stop-color={state} /></linearGradient>
        <foreignObject width="10" height="10"><div>svg foreign chunk 28</div></foreignObject>
    </svg>

    <math><mrow><mi>x</mi><mo>+</mo><mn>{counter}</mn></mrow></math>
    <svelte:element this={state ? "div" : "span"} class="dynamic-28">
        Dynamic element chunk 28: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()}>
        <strong>Inline child chunk 28: {title}</strong>
        <div slot="footer">Footer chunk 28: {counter}</div>
    </ChildComponent>

    <ChildComponent --accent="var(--custom, #5c4634)" {...rest} bind:config>
        {#snippet header(label)}
            <em>Header snippet chunk 28: {label}</em>
        {/snippet}
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={pageHeading} />

    {#if depth > 0}
        <svelte:self depth={depth - 1} title={title} />
    {/if}

    {@render badge("chunk-28", "secondary")}
    {@render card(title, "Content for chunk 28")}
    {@render metricSummary({ label: title, values: [count, doubled, counter], meta: { id: propsId } })}
    {@render show?.()}

    <button onclick={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 28: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 28: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-29">
    Chunk 29: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {moduleSummary} | Store: {storeSummary} | Label: {$labelStore}</p>

    {@html "<b>raw html chunk 29</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-29"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 29.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 29.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 29.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 29: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = `${idx}:${item.name}`}
        <p {...rest} data-index="chunk-29-{idx}" animate:flip>{itemLabel}</p>
    {/each}

    {#each items}
        <span class="item-less">Repeated shell chunk 29</span>
    {/each}

    {#each items as item (item.id)}
        <p animate:flip={{ duration: 200 }}>{item.name}</p>
    {:else}
        <p>No items in chunk 29</p>
    {/each}

    {#await promise}
        <p>Loading chunk 29...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    {#await promise then quickValue}
        <p>Quick resolved: {quickValue}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value="opt-1">One</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
        <option value={`opt-${counter}`}>{title}</option>
    </select>
    <input type="checkbox" bind:checked={checked} />
    <input type="checkbox" bind:group={group} value="opt-29" />
    <input type="radio" bind:group={group} value="opt-29" />
    <input type="number" bind:value={counter} />
    <details bind:open={checked}><summary>More</summary>chunk 29</details>
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div {@attach (node) => { node.dataset.chunk = "29"; }}>attach target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <p transition:slide|global>global transition chunk 29</p>
    <p in:fly|local={{ y: 40 }}>local transition chunk 29</p>

    <div style:opacity style:color={state}>style shorthand</div>
    <div class="slider chunk-29 {title}" class:invinsible>literal class</div>
    <div class={`chunk-${i}-${counter}`}>template literal class</div>
    <span data-note="&copy;=value" title="&copy 1998" aria-label="a &amp; b">entities</span>

    <template id="chunk-tpl-29">
        {@html "<b>templated</b>"}
        {#if counter}<span>inside template</span>{/if}
    </template>

    <svg viewBox="0 0 10 10" class="chunk-svg">
        <clipPath id="clip-29"><rect width="4" height="4" /></clipPath>
        <linearGradient id="grad-29"><stop offset="0%" stop-color={state} /></linearGradient>
        <foreignObject width="10" height="10"><div>svg foreign chunk 29</div></foreignObject>
    </svg>

    <math><mrow><mi>x</mi><mo>+</mo><mn>{counter}</mn></mrow></math>
    <svelte:element this={state ? "div" : "span"} class="dynamic-29">
        Dynamic element chunk 29: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()}>
        <strong>Inline child chunk 29: {title}</strong>
        <div slot="footer">Footer chunk 29: {counter}</div>
    </ChildComponent>

    <ChildComponent --accent="var(--custom, #5c4634)" {...rest} bind:config>
        {#snippet header(label)}
            <em>Header snippet chunk 29: {label}</em>
        {/snippet}
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={pageHeading} />

    {#if depth > 0}
        <svelte:self depth={depth - 1} title={title} />
    {/if}

    {@render badge("chunk-29", "secondary")}
    {@render card(title, "Content for chunk 29")}
    {@render metricSummary({ label: title, values: [count, doubled, counter], meta: { id: propsId } })}
    {@render show?.()}

    <button onclick={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 29: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 29: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-30">
    Chunk 30: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {moduleSummary} | Store: {storeSummary} | Label: {$labelStore}</p>

    {@html "<b>raw html chunk 30</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-30"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 30.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 30.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 30.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 30: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = `${idx}:${item.name}`}
        <p {...rest} data-index="chunk-30-{idx}" animate:flip>{itemLabel}</p>
    {/each}

    {#each items}
        <span class="item-less">Repeated shell chunk 30</span>
    {/each}

    {#each items as item (item.id)}
        <p animate:flip={{ duration: 200 }}>{item.name}</p>
    {:else}
        <p>No items in chunk 30</p>
    {/each}

    {#await promise}
        <p>Loading chunk 30...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    {#await promise then quickValue}
        <p>Quick resolved: {quickValue}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value="opt-1">One</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
        <option value={`opt-${counter}`}>{title}</option>
    </select>
    <input type="checkbox" bind:checked={checked} />
    <input type="checkbox" bind:group={group} value="opt-30" />
    <input type="radio" bind:group={group} value="opt-30" />
    <input type="number" bind:value={counter} />
    <details bind:open={checked}><summary>More</summary>chunk 30</details>
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div {@attach (node) => { node.dataset.chunk = "30"; }}>attach target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <p transition:slide|global>global transition chunk 30</p>
    <p in:fly|local={{ y: 40 }}>local transition chunk 30</p>

    <div style:opacity style:color={state}>style shorthand</div>
    <div class="slider chunk-30 {title}" class:invinsible>literal class</div>
    <div class={`chunk-${i}-${counter}`}>template literal class</div>
    <span data-note="&copy;=value" title="&copy 1998" aria-label="a &amp; b">entities</span>

    <template id="chunk-tpl-30">
        {@html "<b>templated</b>"}
        {#if counter}<span>inside template</span>{/if}
    </template>

    <svg viewBox="0 0 10 10" class="chunk-svg">
        <clipPath id="clip-30"><rect width="4" height="4" /></clipPath>
        <linearGradient id="grad-30"><stop offset="0%" stop-color={state} /></linearGradient>
        <foreignObject width="10" height="10"><div>svg foreign chunk 30</div></foreignObject>
    </svg>

    <math><mrow><mi>x</mi><mo>+</mo><mn>{counter}</mn></mrow></math>
    <svelte:element this={state ? "div" : "span"} class="dynamic-30">
        Dynamic element chunk 30: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()}>
        <strong>Inline child chunk 30: {title}</strong>
        <div slot="footer">Footer chunk 30: {counter}</div>
    </ChildComponent>

    <ChildComponent --accent="var(--custom, #5c4634)" {...rest} bind:config>
        {#snippet header(label)}
            <em>Header snippet chunk 30: {label}</em>
        {/snippet}
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={pageHeading} />

    {#if depth > 0}
        <svelte:self depth={depth - 1} title={title} />
    {/if}

    {@render badge("chunk-30", "secondary")}
    {@render card(title, "Content for chunk 30")}
    {@render metricSummary({ label: title, values: [count, doubled, counter], meta: { id: propsId } })}
    {@render show?.()}

    <button onclick={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 30: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 30: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-31">
    Chunk 31: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {moduleSummary} | Store: {storeSummary} | Label: {$labelStore}</p>

    {@html "<b>raw html chunk 31</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-31"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 31.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 31.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 31.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 31: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = `${idx}:${item.name}`}
        <p {...rest} data-index="chunk-31-{idx}" animate:flip>{itemLabel}</p>
    {/each}

    {#each items}
        <span class="item-less">Repeated shell chunk 31</span>
    {/each}

    {#each items as item (item.id)}
        <p animate:flip={{ duration: 200 }}>{item.name}</p>
    {:else}
        <p>No items in chunk 31</p>
    {/each}

    {#await promise}
        <p>Loading chunk 31...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    {#await promise then quickValue}
        <p>Quick resolved: {quickValue}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value="opt-1">One</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
        <option value={`opt-${counter}`}>{title}</option>
    </select>
    <input type="checkbox" bind:checked={checked} />
    <input type="checkbox" bind:group={group} value="opt-31" />
    <input type="radio" bind:group={group} value="opt-31" />
    <input type="number" bind:value={counter} />
    <details bind:open={checked}><summary>More</summary>chunk 31</details>
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div {@attach (node) => { node.dataset.chunk = "31"; }}>attach target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <p transition:slide|global>global transition chunk 31</p>
    <p in:fly|local={{ y: 40 }}>local transition chunk 31</p>

    <div style:opacity style:color={state}>style shorthand</div>
    <div class="slider chunk-31 {title}" class:invinsible>literal class</div>
    <div class={`chunk-${i}-${counter}`}>template literal class</div>
    <span data-note="&copy;=value" title="&copy 1998" aria-label="a &amp; b">entities</span>

    <template id="chunk-tpl-31">
        {@html "<b>templated</b>"}
        {#if counter}<span>inside template</span>{/if}
    </template>

    <svg viewBox="0 0 10 10" class="chunk-svg">
        <clipPath id="clip-31"><rect width="4" height="4" /></clipPath>
        <linearGradient id="grad-31"><stop offset="0%" stop-color={state} /></linearGradient>
        <foreignObject width="10" height="10"><div>svg foreign chunk 31</div></foreignObject>
    </svg>

    <math><mrow><mi>x</mi><mo>+</mo><mn>{counter}</mn></mrow></math>
    <svelte:element this={state ? "div" : "span"} class="dynamic-31">
        Dynamic element chunk 31: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()}>
        <strong>Inline child chunk 31: {title}</strong>
        <div slot="footer">Footer chunk 31: {counter}</div>
    </ChildComponent>

    <ChildComponent --accent="var(--custom, #5c4634)" {...rest} bind:config>
        {#snippet header(label)}
            <em>Header snippet chunk 31: {label}</em>
        {/snippet}
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={pageHeading} />

    {#if depth > 0}
        <svelte:self depth={depth - 1} title={title} />
    {/if}

    {@render badge("chunk-31", "secondary")}
    {@render card(title, "Content for chunk 31")}
    {@render metricSummary({ label: title, values: [count, doubled, counter], meta: { id: propsId } })}
    {@render show?.()}

    <button onclick={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 31: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 31: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-32">
    Chunk 32: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {moduleSummary} | Store: {storeSummary} | Label: {$labelStore}</p>

    {@html "<b>raw html chunk 32</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-32"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 32.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 32.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 32.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 32: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = `${idx}:${item.name}`}
        <p {...rest} data-index="chunk-32-{idx}" animate:flip>{itemLabel}</p>
    {/each}

    {#each items}
        <span class="item-less">Repeated shell chunk 32</span>
    {/each}

    {#each items as item (item.id)}
        <p animate:flip={{ duration: 200 }}>{item.name}</p>
    {:else}
        <p>No items in chunk 32</p>
    {/each}

    {#await promise}
        <p>Loading chunk 32...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    {#await promise then quickValue}
        <p>Quick resolved: {quickValue}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value="opt-1">One</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
        <option value={`opt-${counter}`}>{title}</option>
    </select>
    <input type="checkbox" bind:checked={checked} />
    <input type="checkbox" bind:group={group} value="opt-32" />
    <input type="radio" bind:group={group} value="opt-32" />
    <input type="number" bind:value={counter} />
    <details bind:open={checked}><summary>More</summary>chunk 32</details>
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div {@attach (node) => { node.dataset.chunk = "32"; }}>attach target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <p transition:slide|global>global transition chunk 32</p>
    <p in:fly|local={{ y: 40 }}>local transition chunk 32</p>

    <div style:opacity style:color={state}>style shorthand</div>
    <div class="slider chunk-32 {title}" class:invinsible>literal class</div>
    <div class={`chunk-${i}-${counter}`}>template literal class</div>
    <span data-note="&copy;=value" title="&copy 1998" aria-label="a &amp; b">entities</span>

    <template id="chunk-tpl-32">
        {@html "<b>templated</b>"}
        {#if counter}<span>inside template</span>{/if}
    </template>

    <svg viewBox="0 0 10 10" class="chunk-svg">
        <clipPath id="clip-32"><rect width="4" height="4" /></clipPath>
        <linearGradient id="grad-32"><stop offset="0%" stop-color={state} /></linearGradient>
        <foreignObject width="10" height="10"><div>svg foreign chunk 32</div></foreignObject>
    </svg>

    <math><mrow><mi>x</mi><mo>+</mo><mn>{counter}</mn></mrow></math>
    <svelte:element this={state ? "div" : "span"} class="dynamic-32">
        Dynamic element chunk 32: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()}>
        <strong>Inline child chunk 32: {title}</strong>
        <div slot="footer">Footer chunk 32: {counter}</div>
    </ChildComponent>

    <ChildComponent --accent="var(--custom, #5c4634)" {...rest} bind:config>
        {#snippet header(label)}
            <em>Header snippet chunk 32: {label}</em>
        {/snippet}
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={pageHeading} />

    {#if depth > 0}
        <svelte:self depth={depth - 1} title={title} />
    {/if}

    {@render badge("chunk-32", "secondary")}
    {@render card(title, "Content for chunk 32")}
    {@render metricSummary({ label: title, values: [count, doubled, counter], meta: { id: propsId } })}
    {@render show?.()}

    <button onclick={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 32: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 32: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-33">
    Chunk 33: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {moduleSummary} | Store: {storeSummary} | Label: {$labelStore}</p>

    {@html "<b>raw html chunk 33</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-33"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 33.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 33.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 33.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 33: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = `${idx}:${item.name}`}
        <p {...rest} data-index="chunk-33-{idx}" animate:flip>{itemLabel}</p>
    {/each}

    {#each items}
        <span class="item-less">Repeated shell chunk 33</span>
    {/each}

    {#each items as item (item.id)}
        <p animate:flip={{ duration: 200 }}>{item.name}</p>
    {:else}
        <p>No items in chunk 33</p>
    {/each}

    {#await promise}
        <p>Loading chunk 33...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    {#await promise then quickValue}
        <p>Quick resolved: {quickValue}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value="opt-1">One</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
        <option value={`opt-${counter}`}>{title}</option>
    </select>
    <input type="checkbox" bind:checked={checked} />
    <input type="checkbox" bind:group={group} value="opt-33" />
    <input type="radio" bind:group={group} value="opt-33" />
    <input type="number" bind:value={counter} />
    <details bind:open={checked}><summary>More</summary>chunk 33</details>
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div {@attach (node) => { node.dataset.chunk = "33"; }}>attach target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <p transition:slide|global>global transition chunk 33</p>
    <p in:fly|local={{ y: 40 }}>local transition chunk 33</p>

    <div style:opacity style:color={state}>style shorthand</div>
    <div class="slider chunk-33 {title}" class:invinsible>literal class</div>
    <div class={`chunk-${i}-${counter}`}>template literal class</div>
    <span data-note="&copy;=value" title="&copy 1998" aria-label="a &amp; b">entities</span>

    <template id="chunk-tpl-33">
        {@html "<b>templated</b>"}
        {#if counter}<span>inside template</span>{/if}
    </template>

    <svg viewBox="0 0 10 10" class="chunk-svg">
        <clipPath id="clip-33"><rect width="4" height="4" /></clipPath>
        <linearGradient id="grad-33"><stop offset="0%" stop-color={state} /></linearGradient>
        <foreignObject width="10" height="10"><div>svg foreign chunk 33</div></foreignObject>
    </svg>

    <math><mrow><mi>x</mi><mo>+</mo><mn>{counter}</mn></mrow></math>
    <svelte:element this={state ? "div" : "span"} class="dynamic-33">
        Dynamic element chunk 33: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()}>
        <strong>Inline child chunk 33: {title}</strong>
        <div slot="footer">Footer chunk 33: {counter}</div>
    </ChildComponent>

    <ChildComponent --accent="var(--custom, #5c4634)" {...rest} bind:config>
        {#snippet header(label)}
            <em>Header snippet chunk 33: {label}</em>
        {/snippet}
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={pageHeading} />

    {#if depth > 0}
        <svelte:self depth={depth - 1} title={title} />
    {/if}

    {@render badge("chunk-33", "secondary")}
    {@render card(title, "Content for chunk 33")}
    {@render metricSummary({ label: title, values: [count, doubled, counter], meta: { id: propsId } })}
    {@render show?.()}

    <button onclick={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 33: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 33: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-34">
    Chunk 34: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {moduleSummary} | Store: {storeSummary} | Label: {$labelStore}</p>

    {@html "<b>raw html chunk 34</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-34"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 34.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 34.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 34.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 34: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = `${idx}:${item.name}`}
        <p {...rest} data-index="chunk-34-{idx}" animate:flip>{itemLabel}</p>
    {/each}

    {#each items}
        <span class="item-less">Repeated shell chunk 34</span>
    {/each}

    {#each items as item (item.id)}
        <p animate:flip={{ duration: 200 }}>{item.name}</p>
    {:else}
        <p>No items in chunk 34</p>
    {/each}

    {#await promise}
        <p>Loading chunk 34...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    {#await promise then quickValue}
        <p>Quick resolved: {quickValue}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value="opt-1">One</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
        <option value={`opt-${counter}`}>{title}</option>
    </select>
    <input type="checkbox" bind:checked={checked} />
    <input type="checkbox" bind:group={group} value="opt-34" />
    <input type="radio" bind:group={group} value="opt-34" />
    <input type="number" bind:value={counter} />
    <details bind:open={checked}><summary>More</summary>chunk 34</details>
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div {@attach (node) => { node.dataset.chunk = "34"; }}>attach target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <p transition:slide|global>global transition chunk 34</p>
    <p in:fly|local={{ y: 40 }}>local transition chunk 34</p>

    <div style:opacity style:color={state}>style shorthand</div>
    <div class="slider chunk-34 {title}" class:invinsible>literal class</div>
    <div class={`chunk-${i}-${counter}`}>template literal class</div>
    <span data-note="&copy;=value" title="&copy 1998" aria-label="a &amp; b">entities</span>

    <template id="chunk-tpl-34">
        {@html "<b>templated</b>"}
        {#if counter}<span>inside template</span>{/if}
    </template>

    <svg viewBox="0 0 10 10" class="chunk-svg">
        <clipPath id="clip-34"><rect width="4" height="4" /></clipPath>
        <linearGradient id="grad-34"><stop offset="0%" stop-color={state} /></linearGradient>
        <foreignObject width="10" height="10"><div>svg foreign chunk 34</div></foreignObject>
    </svg>

    <math><mrow><mi>x</mi><mo>+</mo><mn>{counter}</mn></mrow></math>
    <svelte:element this={state ? "div" : "span"} class="dynamic-34">
        Dynamic element chunk 34: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()}>
        <strong>Inline child chunk 34: {title}</strong>
        <div slot="footer">Footer chunk 34: {counter}</div>
    </ChildComponent>

    <ChildComponent --accent="var(--custom, #5c4634)" {...rest} bind:config>
        {#snippet header(label)}
            <em>Header snippet chunk 34: {label}</em>
        {/snippet}
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={pageHeading} />

    {#if depth > 0}
        <svelte:self depth={depth - 1} title={title} />
    {/if}

    {@render badge("chunk-34", "secondary")}
    {@render card(title, "Content for chunk 34")}
    {@render metricSummary({ label: title, values: [count, doubled, counter], meta: { id: propsId } })}
    {@render show?.()}

    <button onclick={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 34: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 34: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-35">
    Chunk 35: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {moduleSummary} | Store: {storeSummary} | Label: {$labelStore}</p>

    {@html "<b>raw html chunk 35</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-35"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 35.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 35.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 35.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 35: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = `${idx}:${item.name}`}
        <p {...rest} data-index="chunk-35-{idx}" animate:flip>{itemLabel}</p>
    {/each}

    {#each items}
        <span class="item-less">Repeated shell chunk 35</span>
    {/each}

    {#each items as item (item.id)}
        <p animate:flip={{ duration: 200 }}>{item.name}</p>
    {:else}
        <p>No items in chunk 35</p>
    {/each}

    {#await promise}
        <p>Loading chunk 35...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    {#await promise then quickValue}
        <p>Quick resolved: {quickValue}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value="opt-1">One</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
        <option value={`opt-${counter}`}>{title}</option>
    </select>
    <input type="checkbox" bind:checked={checked} />
    <input type="checkbox" bind:group={group} value="opt-35" />
    <input type="radio" bind:group={group} value="opt-35" />
    <input type="number" bind:value={counter} />
    <details bind:open={checked}><summary>More</summary>chunk 35</details>
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div {@attach (node) => { node.dataset.chunk = "35"; }}>attach target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <p transition:slide|global>global transition chunk 35</p>
    <p in:fly|local={{ y: 40 }}>local transition chunk 35</p>

    <div style:opacity style:color={state}>style shorthand</div>
    <div class="slider chunk-35 {title}" class:invinsible>literal class</div>
    <div class={`chunk-${i}-${counter}`}>template literal class</div>
    <span data-note="&copy;=value" title="&copy 1998" aria-label="a &amp; b">entities</span>

    <template id="chunk-tpl-35">
        {@html "<b>templated</b>"}
        {#if counter}<span>inside template</span>{/if}
    </template>

    <svg viewBox="0 0 10 10" class="chunk-svg">
        <clipPath id="clip-35"><rect width="4" height="4" /></clipPath>
        <linearGradient id="grad-35"><stop offset="0%" stop-color={state} /></linearGradient>
        <foreignObject width="10" height="10"><div>svg foreign chunk 35</div></foreignObject>
    </svg>

    <math><mrow><mi>x</mi><mo>+</mo><mn>{counter}</mn></mrow></math>
    <svelte:element this={state ? "div" : "span"} class="dynamic-35">
        Dynamic element chunk 35: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()}>
        <strong>Inline child chunk 35: {title}</strong>
        <div slot="footer">Footer chunk 35: {counter}</div>
    </ChildComponent>

    <ChildComponent --accent="var(--custom, #5c4634)" {...rest} bind:config>
        {#snippet header(label)}
            <em>Header snippet chunk 35: {label}</em>
        {/snippet}
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={pageHeading} />

    {#if depth > 0}
        <svelte:self depth={depth - 1} title={title} />
    {/if}

    {@render badge("chunk-35", "secondary")}
    {@render card(title, "Content for chunk 35")}
    {@render metricSummary({ label: title, values: [count, doubled, counter], meta: { id: propsId } })}
    {@render show?.()}

    <button onclick={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 35: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 35: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-36">
    Chunk 36: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {moduleSummary} | Store: {storeSummary} | Label: {$labelStore}</p>

    {@html "<b>raw html chunk 36</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-36"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 36.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 36.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 36.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 36: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = `${idx}:${item.name}`}
        <p {...rest} data-index="chunk-36-{idx}" animate:flip>{itemLabel}</p>
    {/each}

    {#each items}
        <span class="item-less">Repeated shell chunk 36</span>
    {/each}

    {#each items as item (item.id)}
        <p animate:flip={{ duration: 200 }}>{item.name}</p>
    {:else}
        <p>No items in chunk 36</p>
    {/each}

    {#await promise}
        <p>Loading chunk 36...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    {#await promise then quickValue}
        <p>Quick resolved: {quickValue}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value="opt-1">One</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
        <option value={`opt-${counter}`}>{title}</option>
    </select>
    <input type="checkbox" bind:checked={checked} />
    <input type="checkbox" bind:group={group} value="opt-36" />
    <input type="radio" bind:group={group} value="opt-36" />
    <input type="number" bind:value={counter} />
    <details bind:open={checked}><summary>More</summary>chunk 36</details>
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div {@attach (node) => { node.dataset.chunk = "36"; }}>attach target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <p transition:slide|global>global transition chunk 36</p>
    <p in:fly|local={{ y: 40 }}>local transition chunk 36</p>

    <div style:opacity style:color={state}>style shorthand</div>
    <div class="slider chunk-36 {title}" class:invinsible>literal class</div>
    <div class={`chunk-${i}-${counter}`}>template literal class</div>
    <span data-note="&copy;=value" title="&copy 1998" aria-label="a &amp; b">entities</span>

    <template id="chunk-tpl-36">
        {@html "<b>templated</b>"}
        {#if counter}<span>inside template</span>{/if}
    </template>

    <svg viewBox="0 0 10 10" class="chunk-svg">
        <clipPath id="clip-36"><rect width="4" height="4" /></clipPath>
        <linearGradient id="grad-36"><stop offset="0%" stop-color={state} /></linearGradient>
        <foreignObject width="10" height="10"><div>svg foreign chunk 36</div></foreignObject>
    </svg>

    <math><mrow><mi>x</mi><mo>+</mo><mn>{counter}</mn></mrow></math>
    <svelte:element this={state ? "div" : "span"} class="dynamic-36">
        Dynamic element chunk 36: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()}>
        <strong>Inline child chunk 36: {title}</strong>
        <div slot="footer">Footer chunk 36: {counter}</div>
    </ChildComponent>

    <ChildComponent --accent="var(--custom, #5c4634)" {...rest} bind:config>
        {#snippet header(label)}
            <em>Header snippet chunk 36: {label}</em>
        {/snippet}
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={pageHeading} />

    {#if depth > 0}
        <svelte:self depth={depth - 1} title={title} />
    {/if}

    {@render badge("chunk-36", "secondary")}
    {@render card(title, "Content for chunk 36")}
    {@render metricSummary({ label: title, values: [count, doubled, counter], meta: { id: propsId } })}
    {@render show?.()}

    <button onclick={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 36: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 36: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-37">
    Chunk 37: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {moduleSummary} | Store: {storeSummary} | Label: {$labelStore}</p>

    {@html "<b>raw html chunk 37</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-37"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 37.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 37.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 37.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 37: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = `${idx}:${item.name}`}
        <p {...rest} data-index="chunk-37-{idx}" animate:flip>{itemLabel}</p>
    {/each}

    {#each items}
        <span class="item-less">Repeated shell chunk 37</span>
    {/each}

    {#each items as item (item.id)}
        <p animate:flip={{ duration: 200 }}>{item.name}</p>
    {:else}
        <p>No items in chunk 37</p>
    {/each}

    {#await promise}
        <p>Loading chunk 37...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    {#await promise then quickValue}
        <p>Quick resolved: {quickValue}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value="opt-1">One</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
        <option value={`opt-${counter}`}>{title}</option>
    </select>
    <input type="checkbox" bind:checked={checked} />
    <input type="checkbox" bind:group={group} value="opt-37" />
    <input type="radio" bind:group={group} value="opt-37" />
    <input type="number" bind:value={counter} />
    <details bind:open={checked}><summary>More</summary>chunk 37</details>
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div {@attach (node) => { node.dataset.chunk = "37"; }}>attach target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <p transition:slide|global>global transition chunk 37</p>
    <p in:fly|local={{ y: 40 }}>local transition chunk 37</p>

    <div style:opacity style:color={state}>style shorthand</div>
    <div class="slider chunk-37 {title}" class:invinsible>literal class</div>
    <div class={`chunk-${i}-${counter}`}>template literal class</div>
    <span data-note="&copy;=value" title="&copy 1998" aria-label="a &amp; b">entities</span>

    <template id="chunk-tpl-37">
        {@html "<b>templated</b>"}
        {#if counter}<span>inside template</span>{/if}
    </template>

    <svg viewBox="0 0 10 10" class="chunk-svg">
        <clipPath id="clip-37"><rect width="4" height="4" /></clipPath>
        <linearGradient id="grad-37"><stop offset="0%" stop-color={state} /></linearGradient>
        <foreignObject width="10" height="10"><div>svg foreign chunk 37</div></foreignObject>
    </svg>

    <math><mrow><mi>x</mi><mo>+</mo><mn>{counter}</mn></mrow></math>
    <svelte:element this={state ? "div" : "span"} class="dynamic-37">
        Dynamic element chunk 37: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()}>
        <strong>Inline child chunk 37: {title}</strong>
        <div slot="footer">Footer chunk 37: {counter}</div>
    </ChildComponent>

    <ChildComponent --accent="var(--custom, #5c4634)" {...rest} bind:config>
        {#snippet header(label)}
            <em>Header snippet chunk 37: {label}</em>
        {/snippet}
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={pageHeading} />

    {#if depth > 0}
        <svelte:self depth={depth - 1} title={title} />
    {/if}

    {@render badge("chunk-37", "secondary")}
    {@render card(title, "Content for chunk 37")}
    {@render metricSummary({ label: title, values: [count, doubled, counter], meta: { id: propsId } })}
    {@render show?.()}

    <button onclick={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 37: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 37: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-38">
    Chunk 38: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {moduleSummary} | Store: {storeSummary} | Label: {$labelStore}</p>

    {@html "<b>raw html chunk 38</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-38"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 38.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 38.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 38.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 38: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = `${idx}:${item.name}`}
        <p {...rest} data-index="chunk-38-{idx}" animate:flip>{itemLabel}</p>
    {/each}

    {#each items}
        <span class="item-less">Repeated shell chunk 38</span>
    {/each}

    {#each items as item (item.id)}
        <p animate:flip={{ duration: 200 }}>{item.name}</p>
    {:else}
        <p>No items in chunk 38</p>
    {/each}

    {#await promise}
        <p>Loading chunk 38...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    {#await promise then quickValue}
        <p>Quick resolved: {quickValue}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value="opt-1">One</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
        <option value={`opt-${counter}`}>{title}</option>
    </select>
    <input type="checkbox" bind:checked={checked} />
    <input type="checkbox" bind:group={group} value="opt-38" />
    <input type="radio" bind:group={group} value="opt-38" />
    <input type="number" bind:value={counter} />
    <details bind:open={checked}><summary>More</summary>chunk 38</details>
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div {@attach (node) => { node.dataset.chunk = "38"; }}>attach target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <p transition:slide|global>global transition chunk 38</p>
    <p in:fly|local={{ y: 40 }}>local transition chunk 38</p>

    <div style:opacity style:color={state}>style shorthand</div>
    <div class="slider chunk-38 {title}" class:invinsible>literal class</div>
    <div class={`chunk-${i}-${counter}`}>template literal class</div>
    <span data-note="&copy;=value" title="&copy 1998" aria-label="a &amp; b">entities</span>

    <template id="chunk-tpl-38">
        {@html "<b>templated</b>"}
        {#if counter}<span>inside template</span>{/if}
    </template>

    <svg viewBox="0 0 10 10" class="chunk-svg">
        <clipPath id="clip-38"><rect width="4" height="4" /></clipPath>
        <linearGradient id="grad-38"><stop offset="0%" stop-color={state} /></linearGradient>
        <foreignObject width="10" height="10"><div>svg foreign chunk 38</div></foreignObject>
    </svg>

    <math><mrow><mi>x</mi><mo>+</mo><mn>{counter}</mn></mrow></math>
    <svelte:element this={state ? "div" : "span"} class="dynamic-38">
        Dynamic element chunk 38: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()}>
        <strong>Inline child chunk 38: {title}</strong>
        <div slot="footer">Footer chunk 38: {counter}</div>
    </ChildComponent>

    <ChildComponent --accent="var(--custom, #5c4634)" {...rest} bind:config>
        {#snippet header(label)}
            <em>Header snippet chunk 38: {label}</em>
        {/snippet}
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={pageHeading} />

    {#if depth > 0}
        <svelte:self depth={depth - 1} title={title} />
    {/if}

    {@render badge("chunk-38", "secondary")}
    {@render card(title, "Content for chunk 38")}
    {@render metricSummary({ label: title, values: [count, doubled, counter], meta: { id: propsId } })}
    {@render show?.()}

    <button onclick={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 38: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 38: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-39">
    Chunk 39: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {moduleSummary} | Store: {storeSummary} | Label: {$labelStore}</p>

    {@html "<b>raw html chunk 39</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-39"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 39.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 39.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 39.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 39: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = `${idx}:${item.name}`}
        <p {...rest} data-index="chunk-39-{idx}" animate:flip>{itemLabel}</p>
    {/each}

    {#each items}
        <span class="item-less">Repeated shell chunk 39</span>
    {/each}

    {#each items as item (item.id)}
        <p animate:flip={{ duration: 200 }}>{item.name}</p>
    {:else}
        <p>No items in chunk 39</p>
    {/each}

    {#await promise}
        <p>Loading chunk 39...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    {#await promise then quickValue}
        <p>Quick resolved: {quickValue}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value="opt-1">One</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
        <option value={`opt-${counter}`}>{title}</option>
    </select>
    <input type="checkbox" bind:checked={checked} />
    <input type="checkbox" bind:group={group} value="opt-39" />
    <input type="radio" bind:group={group} value="opt-39" />
    <input type="number" bind:value={counter} />
    <details bind:open={checked}><summary>More</summary>chunk 39</details>
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div {@attach (node) => { node.dataset.chunk = "39"; }}>attach target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <p transition:slide|global>global transition chunk 39</p>
    <p in:fly|local={{ y: 40 }}>local transition chunk 39</p>

    <div style:opacity style:color={state}>style shorthand</div>
    <div class="slider chunk-39 {title}" class:invinsible>literal class</div>
    <div class={`chunk-${i}-${counter}`}>template literal class</div>
    <span data-note="&copy;=value" title="&copy 1998" aria-label="a &amp; b">entities</span>

    <template id="chunk-tpl-39">
        {@html "<b>templated</b>"}
        {#if counter}<span>inside template</span>{/if}
    </template>

    <svg viewBox="0 0 10 10" class="chunk-svg">
        <clipPath id="clip-39"><rect width="4" height="4" /></clipPath>
        <linearGradient id="grad-39"><stop offset="0%" stop-color={state} /></linearGradient>
        <foreignObject width="10" height="10"><div>svg foreign chunk 39</div></foreignObject>
    </svg>

    <math><mrow><mi>x</mi><mo>+</mo><mn>{counter}</mn></mrow></math>
    <svelte:element this={state ? "div" : "span"} class="dynamic-39">
        Dynamic element chunk 39: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()}>
        <strong>Inline child chunk 39: {title}</strong>
        <div slot="footer">Footer chunk 39: {counter}</div>
    </ChildComponent>

    <ChildComponent --accent="var(--custom, #5c4634)" {...rest} bind:config>
        {#snippet header(label)}
            <em>Header snippet chunk 39: {label}</em>
        {/snippet}
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={pageHeading} />

    {#if depth > 0}
        <svelte:self depth={depth - 1} title={title} />
    {/if}

    {@render badge("chunk-39", "secondary")}
    {@render card(title, "Content for chunk 39")}
    {@render metricSummary({ label: title, values: [count, doubled, counter], meta: { id: propsId } })}
    {@render show?.()}

    <button onclick={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 39: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 39: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-40">
    Chunk 40: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {moduleSummary} | Store: {storeSummary} | Label: {$labelStore}</p>

    {@html "<b>raw html chunk 40</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-40"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 40.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 40.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 40.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 40: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = `${idx}:${item.name}`}
        <p {...rest} data-index="chunk-40-{idx}" animate:flip>{itemLabel}</p>
    {/each}

    {#each items}
        <span class="item-less">Repeated shell chunk 40</span>
    {/each}

    {#each items as item (item.id)}
        <p animate:flip={{ duration: 200 }}>{item.name}</p>
    {:else}
        <p>No items in chunk 40</p>
    {/each}

    {#await promise}
        <p>Loading chunk 40...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    {#await promise then quickValue}
        <p>Quick resolved: {quickValue}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value="opt-1">One</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
        <option value={`opt-${counter}`}>{title}</option>
    </select>
    <input type="checkbox" bind:checked={checked} />
    <input type="checkbox" bind:group={group} value="opt-40" />
    <input type="radio" bind:group={group} value="opt-40" />
    <input type="number" bind:value={counter} />
    <details bind:open={checked}><summary>More</summary>chunk 40</details>
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div {@attach (node) => { node.dataset.chunk = "40"; }}>attach target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <p transition:slide|global>global transition chunk 40</p>
    <p in:fly|local={{ y: 40 }}>local transition chunk 40</p>

    <div style:opacity style:color={state}>style shorthand</div>
    <div class="slider chunk-40 {title}" class:invinsible>literal class</div>
    <div class={`chunk-${i}-${counter}`}>template literal class</div>
    <span data-note="&copy;=value" title="&copy 1998" aria-label="a &amp; b">entities</span>

    <template id="chunk-tpl-40">
        {@html "<b>templated</b>"}
        {#if counter}<span>inside template</span>{/if}
    </template>

    <svg viewBox="0 0 10 10" class="chunk-svg">
        <clipPath id="clip-40"><rect width="4" height="4" /></clipPath>
        <linearGradient id="grad-40"><stop offset="0%" stop-color={state} /></linearGradient>
        <foreignObject width="10" height="10"><div>svg foreign chunk 40</div></foreignObject>
    </svg>

    <math><mrow><mi>x</mi><mo>+</mo><mn>{counter}</mn></mrow></math>
    <svelte:element this={state ? "div" : "span"} class="dynamic-40">
        Dynamic element chunk 40: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()}>
        <strong>Inline child chunk 40: {title}</strong>
        <div slot="footer">Footer chunk 40: {counter}</div>
    </ChildComponent>

    <ChildComponent --accent="var(--custom, #5c4634)" {...rest} bind:config>
        {#snippet header(label)}
            <em>Header snippet chunk 40: {label}</em>
        {/snippet}
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={pageHeading} />

    {#if depth > 0}
        <svelte:self depth={depth - 1} title={title} />
    {/if}

    {@render badge("chunk-40", "secondary")}
    {@render card(title, "Content for chunk 40")}
    {@render metricSummary({ label: title, values: [count, doubled, counter], meta: { id: propsId } })}
    {@render show?.()}

    <button onclick={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 40: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 40: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-41">
    Chunk 41: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {moduleSummary} | Store: {storeSummary} | Label: {$labelStore}</p>

    {@html "<b>raw html chunk 41</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-41"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 41.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 41.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 41.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 41: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = `${idx}:${item.name}`}
        <p {...rest} data-index="chunk-41-{idx}" animate:flip>{itemLabel}</p>
    {/each}

    {#each items}
        <span class="item-less">Repeated shell chunk 41</span>
    {/each}

    {#each items as item (item.id)}
        <p animate:flip={{ duration: 200 }}>{item.name}</p>
    {:else}
        <p>No items in chunk 41</p>
    {/each}

    {#await promise}
        <p>Loading chunk 41...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    {#await promise then quickValue}
        <p>Quick resolved: {quickValue}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value="opt-1">One</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
        <option value={`opt-${counter}`}>{title}</option>
    </select>
    <input type="checkbox" bind:checked={checked} />
    <input type="checkbox" bind:group={group} value="opt-41" />
    <input type="radio" bind:group={group} value="opt-41" />
    <input type="number" bind:value={counter} />
    <details bind:open={checked}><summary>More</summary>chunk 41</details>
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div {@attach (node) => { node.dataset.chunk = "41"; }}>attach target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <p transition:slide|global>global transition chunk 41</p>
    <p in:fly|local={{ y: 40 }}>local transition chunk 41</p>

    <div style:opacity style:color={state}>style shorthand</div>
    <div class="slider chunk-41 {title}" class:invinsible>literal class</div>
    <div class={`chunk-${i}-${counter}`}>template literal class</div>
    <span data-note="&copy;=value" title="&copy 1998" aria-label="a &amp; b">entities</span>

    <template id="chunk-tpl-41">
        {@html "<b>templated</b>"}
        {#if counter}<span>inside template</span>{/if}
    </template>

    <svg viewBox="0 0 10 10" class="chunk-svg">
        <clipPath id="clip-41"><rect width="4" height="4" /></clipPath>
        <linearGradient id="grad-41"><stop offset="0%" stop-color={state} /></linearGradient>
        <foreignObject width="10" height="10"><div>svg foreign chunk 41</div></foreignObject>
    </svg>

    <math><mrow><mi>x</mi><mo>+</mo><mn>{counter}</mn></mrow></math>
    <svelte:element this={state ? "div" : "span"} class="dynamic-41">
        Dynamic element chunk 41: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()}>
        <strong>Inline child chunk 41: {title}</strong>
        <div slot="footer">Footer chunk 41: {counter}</div>
    </ChildComponent>

    <ChildComponent --accent="var(--custom, #5c4634)" {...rest} bind:config>
        {#snippet header(label)}
            <em>Header snippet chunk 41: {label}</em>
        {/snippet}
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={pageHeading} />

    {#if depth > 0}
        <svelte:self depth={depth - 1} title={title} />
    {/if}

    {@render badge("chunk-41", "secondary")}
    {@render card(title, "Content for chunk 41")}
    {@render metricSummary({ label: title, values: [count, doubled, counter], meta: { id: propsId } })}
    {@render show?.()}

    <button onclick={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 41: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 41: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-42">
    Chunk 42: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {moduleSummary} | Store: {storeSummary} | Label: {$labelStore}</p>

    {@html "<b>raw html chunk 42</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-42"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 42.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 42.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 42.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 42: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = `${idx}:${item.name}`}
        <p {...rest} data-index="chunk-42-{idx}" animate:flip>{itemLabel}</p>
    {/each}

    {#each items}
        <span class="item-less">Repeated shell chunk 42</span>
    {/each}

    {#each items as item (item.id)}
        <p animate:flip={{ duration: 200 }}>{item.name}</p>
    {:else}
        <p>No items in chunk 42</p>
    {/each}

    {#await promise}
        <p>Loading chunk 42...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    {#await promise then quickValue}
        <p>Quick resolved: {quickValue}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value="opt-1">One</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
        <option value={`opt-${counter}`}>{title}</option>
    </select>
    <input type="checkbox" bind:checked={checked} />
    <input type="checkbox" bind:group={group} value="opt-42" />
    <input type="radio" bind:group={group} value="opt-42" />
    <input type="number" bind:value={counter} />
    <details bind:open={checked}><summary>More</summary>chunk 42</details>
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div {@attach (node) => { node.dataset.chunk = "42"; }}>attach target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <p transition:slide|global>global transition chunk 42</p>
    <p in:fly|local={{ y: 40 }}>local transition chunk 42</p>

    <div style:opacity style:color={state}>style shorthand</div>
    <div class="slider chunk-42 {title}" class:invinsible>literal class</div>
    <div class={`chunk-${i}-${counter}`}>template literal class</div>
    <span data-note="&copy;=value" title="&copy 1998" aria-label="a &amp; b">entities</span>

    <template id="chunk-tpl-42">
        {@html "<b>templated</b>"}
        {#if counter}<span>inside template</span>{/if}
    </template>

    <svg viewBox="0 0 10 10" class="chunk-svg">
        <clipPath id="clip-42"><rect width="4" height="4" /></clipPath>
        <linearGradient id="grad-42"><stop offset="0%" stop-color={state} /></linearGradient>
        <foreignObject width="10" height="10"><div>svg foreign chunk 42</div></foreignObject>
    </svg>

    <math><mrow><mi>x</mi><mo>+</mo><mn>{counter}</mn></mrow></math>
    <svelte:element this={state ? "div" : "span"} class="dynamic-42">
        Dynamic element chunk 42: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()}>
        <strong>Inline child chunk 42: {title}</strong>
        <div slot="footer">Footer chunk 42: {counter}</div>
    </ChildComponent>

    <ChildComponent --accent="var(--custom, #5c4634)" {...rest} bind:config>
        {#snippet header(label)}
            <em>Header snippet chunk 42: {label}</em>
        {/snippet}
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={pageHeading} />

    {#if depth > 0}
        <svelte:self depth={depth - 1} title={title} />
    {/if}

    {@render badge("chunk-42", "secondary")}
    {@render card(title, "Content for chunk 42")}
    {@render metricSummary({ label: title, values: [count, doubled, counter], meta: { id: propsId } })}
    {@render show?.()}

    <button onclick={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 42: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 42: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-43">
    Chunk 43: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {moduleSummary} | Store: {storeSummary} | Label: {$labelStore}</p>

    {@html "<b>raw html chunk 43</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-43"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 43.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 43.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 43.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 43: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = `${idx}:${item.name}`}
        <p {...rest} data-index="chunk-43-{idx}" animate:flip>{itemLabel}</p>
    {/each}

    {#each items}
        <span class="item-less">Repeated shell chunk 43</span>
    {/each}

    {#each items as item (item.id)}
        <p animate:flip={{ duration: 200 }}>{item.name}</p>
    {:else}
        <p>No items in chunk 43</p>
    {/each}

    {#await promise}
        <p>Loading chunk 43...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    {#await promise then quickValue}
        <p>Quick resolved: {quickValue}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value="opt-1">One</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
        <option value={`opt-${counter}`}>{title}</option>
    </select>
    <input type="checkbox" bind:checked={checked} />
    <input type="checkbox" bind:group={group} value="opt-43" />
    <input type="radio" bind:group={group} value="opt-43" />
    <input type="number" bind:value={counter} />
    <details bind:open={checked}><summary>More</summary>chunk 43</details>
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div {@attach (node) => { node.dataset.chunk = "43"; }}>attach target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <p transition:slide|global>global transition chunk 43</p>
    <p in:fly|local={{ y: 40 }}>local transition chunk 43</p>

    <div style:opacity style:color={state}>style shorthand</div>
    <div class="slider chunk-43 {title}" class:invinsible>literal class</div>
    <div class={`chunk-${i}-${counter}`}>template literal class</div>
    <span data-note="&copy;=value" title="&copy 1998" aria-label="a &amp; b">entities</span>

    <template id="chunk-tpl-43">
        {@html "<b>templated</b>"}
        {#if counter}<span>inside template</span>{/if}
    </template>

    <svg viewBox="0 0 10 10" class="chunk-svg">
        <clipPath id="clip-43"><rect width="4" height="4" /></clipPath>
        <linearGradient id="grad-43"><stop offset="0%" stop-color={state} /></linearGradient>
        <foreignObject width="10" height="10"><div>svg foreign chunk 43</div></foreignObject>
    </svg>

    <math><mrow><mi>x</mi><mo>+</mo><mn>{counter}</mn></mrow></math>
    <svelte:element this={state ? "div" : "span"} class="dynamic-43">
        Dynamic element chunk 43: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()}>
        <strong>Inline child chunk 43: {title}</strong>
        <div slot="footer">Footer chunk 43: {counter}</div>
    </ChildComponent>

    <ChildComponent --accent="var(--custom, #5c4634)" {...rest} bind:config>
        {#snippet header(label)}
            <em>Header snippet chunk 43: {label}</em>
        {/snippet}
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={pageHeading} />

    {#if depth > 0}
        <svelte:self depth={depth - 1} title={title} />
    {/if}

    {@render badge("chunk-43", "secondary")}
    {@render card(title, "Content for chunk 43")}
    {@render metricSummary({ label: title, values: [count, doubled, counter], meta: { id: propsId } })}
    {@render show?.()}

    <button onclick={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 43: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 43: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-44">
    Chunk 44: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {moduleSummary} | Store: {storeSummary} | Label: {$labelStore}</p>

    {@html "<b>raw html chunk 44</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-44"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 44.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 44.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 44.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 44: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = `${idx}:${item.name}`}
        <p {...rest} data-index="chunk-44-{idx}" animate:flip>{itemLabel}</p>
    {/each}

    {#each items}
        <span class="item-less">Repeated shell chunk 44</span>
    {/each}

    {#each items as item (item.id)}
        <p animate:flip={{ duration: 200 }}>{item.name}</p>
    {:else}
        <p>No items in chunk 44</p>
    {/each}

    {#await promise}
        <p>Loading chunk 44...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    {#await promise then quickValue}
        <p>Quick resolved: {quickValue}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value="opt-1">One</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
        <option value={`opt-${counter}`}>{title}</option>
    </select>
    <input type="checkbox" bind:checked={checked} />
    <input type="checkbox" bind:group={group} value="opt-44" />
    <input type="radio" bind:group={group} value="opt-44" />
    <input type="number" bind:value={counter} />
    <details bind:open={checked}><summary>More</summary>chunk 44</details>
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div {@attach (node) => { node.dataset.chunk = "44"; }}>attach target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <p transition:slide|global>global transition chunk 44</p>
    <p in:fly|local={{ y: 40 }}>local transition chunk 44</p>

    <div style:opacity style:color={state}>style shorthand</div>
    <div class="slider chunk-44 {title}" class:invinsible>literal class</div>
    <div class={`chunk-${i}-${counter}`}>template literal class</div>
    <span data-note="&copy;=value" title="&copy 1998" aria-label="a &amp; b">entities</span>

    <template id="chunk-tpl-44">
        {@html "<b>templated</b>"}
        {#if counter}<span>inside template</span>{/if}
    </template>

    <svg viewBox="0 0 10 10" class="chunk-svg">
        <clipPath id="clip-44"><rect width="4" height="4" /></clipPath>
        <linearGradient id="grad-44"><stop offset="0%" stop-color={state} /></linearGradient>
        <foreignObject width="10" height="10"><div>svg foreign chunk 44</div></foreignObject>
    </svg>

    <math><mrow><mi>x</mi><mo>+</mo><mn>{counter}</mn></mrow></math>
    <svelte:element this={state ? "div" : "span"} class="dynamic-44">
        Dynamic element chunk 44: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()}>
        <strong>Inline child chunk 44: {title}</strong>
        <div slot="footer">Footer chunk 44: {counter}</div>
    </ChildComponent>

    <ChildComponent --accent="var(--custom, #5c4634)" {...rest} bind:config>
        {#snippet header(label)}
            <em>Header snippet chunk 44: {label}</em>
        {/snippet}
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={pageHeading} />

    {#if depth > 0}
        <svelte:self depth={depth - 1} title={title} />
    {/if}

    {@render badge("chunk-44", "secondary")}
    {@render card(title, "Content for chunk 44")}
    {@render metricSummary({ label: title, values: [count, doubled, counter], meta: { id: propsId } })}
    {@render show?.()}

    <button onclick={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 44: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 44: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-45">
    Chunk 45: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {moduleSummary} | Store: {storeSummary} | Label: {$labelStore}</p>

    {@html "<b>raw html chunk 45</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-45"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 45.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 45.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 45.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 45: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = `${idx}:${item.name}`}
        <p {...rest} data-index="chunk-45-{idx}" animate:flip>{itemLabel}</p>
    {/each}

    {#each items}
        <span class="item-less">Repeated shell chunk 45</span>
    {/each}

    {#each items as item (item.id)}
        <p animate:flip={{ duration: 200 }}>{item.name}</p>
    {:else}
        <p>No items in chunk 45</p>
    {/each}

    {#await promise}
        <p>Loading chunk 45...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    {#await promise then quickValue}
        <p>Quick resolved: {quickValue}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value="opt-1">One</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
        <option value={`opt-${counter}`}>{title}</option>
    </select>
    <input type="checkbox" bind:checked={checked} />
    <input type="checkbox" bind:group={group} value="opt-45" />
    <input type="radio" bind:group={group} value="opt-45" />
    <input type="number" bind:value={counter} />
    <details bind:open={checked}><summary>More</summary>chunk 45</details>
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div {@attach (node) => { node.dataset.chunk = "45"; }}>attach target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <p transition:slide|global>global transition chunk 45</p>
    <p in:fly|local={{ y: 40 }}>local transition chunk 45</p>

    <div style:opacity style:color={state}>style shorthand</div>
    <div class="slider chunk-45 {title}" class:invinsible>literal class</div>
    <div class={`chunk-${i}-${counter}`}>template literal class</div>
    <span data-note="&copy;=value" title="&copy 1998" aria-label="a &amp; b">entities</span>

    <template id="chunk-tpl-45">
        {@html "<b>templated</b>"}
        {#if counter}<span>inside template</span>{/if}
    </template>

    <svg viewBox="0 0 10 10" class="chunk-svg">
        <clipPath id="clip-45"><rect width="4" height="4" /></clipPath>
        <linearGradient id="grad-45"><stop offset="0%" stop-color={state} /></linearGradient>
        <foreignObject width="10" height="10"><div>svg foreign chunk 45</div></foreignObject>
    </svg>

    <math><mrow><mi>x</mi><mo>+</mo><mn>{counter}</mn></mrow></math>
    <svelte:element this={state ? "div" : "span"} class="dynamic-45">
        Dynamic element chunk 45: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()}>
        <strong>Inline child chunk 45: {title}</strong>
        <div slot="footer">Footer chunk 45: {counter}</div>
    </ChildComponent>

    <ChildComponent --accent="var(--custom, #5c4634)" {...rest} bind:config>
        {#snippet header(label)}
            <em>Header snippet chunk 45: {label}</em>
        {/snippet}
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={pageHeading} />

    {#if depth > 0}
        <svelte:self depth={depth - 1} title={title} />
    {/if}

    {@render badge("chunk-45", "secondary")}
    {@render card(title, "Content for chunk 45")}
    {@render metricSummary({ label: title, values: [count, doubled, counter], meta: { id: propsId } })}
    {@render show?.()}

    <button onclick={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 45: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 45: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-46">
    Chunk 46: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {moduleSummary} | Store: {storeSummary} | Label: {$labelStore}</p>

    {@html "<b>raw html chunk 46</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-46"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 46.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 46.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 46.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 46: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = `${idx}:${item.name}`}
        <p {...rest} data-index="chunk-46-{idx}" animate:flip>{itemLabel}</p>
    {/each}

    {#each items}
        <span class="item-less">Repeated shell chunk 46</span>
    {/each}

    {#each items as item (item.id)}
        <p animate:flip={{ duration: 200 }}>{item.name}</p>
    {:else}
        <p>No items in chunk 46</p>
    {/each}

    {#await promise}
        <p>Loading chunk 46...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    {#await promise then quickValue}
        <p>Quick resolved: {quickValue}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value="opt-1">One</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
        <option value={`opt-${counter}`}>{title}</option>
    </select>
    <input type="checkbox" bind:checked={checked} />
    <input type="checkbox" bind:group={group} value="opt-46" />
    <input type="radio" bind:group={group} value="opt-46" />
    <input type="number" bind:value={counter} />
    <details bind:open={checked}><summary>More</summary>chunk 46</details>
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div {@attach (node) => { node.dataset.chunk = "46"; }}>attach target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <p transition:slide|global>global transition chunk 46</p>
    <p in:fly|local={{ y: 40 }}>local transition chunk 46</p>

    <div style:opacity style:color={state}>style shorthand</div>
    <div class="slider chunk-46 {title}" class:invinsible>literal class</div>
    <div class={`chunk-${i}-${counter}`}>template literal class</div>
    <span data-note="&copy;=value" title="&copy 1998" aria-label="a &amp; b">entities</span>

    <template id="chunk-tpl-46">
        {@html "<b>templated</b>"}
        {#if counter}<span>inside template</span>{/if}
    </template>

    <svg viewBox="0 0 10 10" class="chunk-svg">
        <clipPath id="clip-46"><rect width="4" height="4" /></clipPath>
        <linearGradient id="grad-46"><stop offset="0%" stop-color={state} /></linearGradient>
        <foreignObject width="10" height="10"><div>svg foreign chunk 46</div></foreignObject>
    </svg>

    <math><mrow><mi>x</mi><mo>+</mo><mn>{counter}</mn></mrow></math>
    <svelte:element this={state ? "div" : "span"} class="dynamic-46">
        Dynamic element chunk 46: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()}>
        <strong>Inline child chunk 46: {title}</strong>
        <div slot="footer">Footer chunk 46: {counter}</div>
    </ChildComponent>

    <ChildComponent --accent="var(--custom, #5c4634)" {...rest} bind:config>
        {#snippet header(label)}
            <em>Header snippet chunk 46: {label}</em>
        {/snippet}
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={pageHeading} />

    {#if depth > 0}
        <svelte:self depth={depth - 1} title={title} />
    {/if}

    {@render badge("chunk-46", "secondary")}
    {@render card(title, "Content for chunk 46")}
    {@render metricSummary({ label: title, values: [count, doubled, counter], meta: { id: propsId } })}
    {@render show?.()}

    <button onclick={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 46: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 46: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-47">
    Chunk 47: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {moduleSummary} | Store: {storeSummary} | Label: {$labelStore}</p>

    {@html "<b>raw html chunk 47</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-47"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 47.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 47.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 47.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 47: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = `${idx}:${item.name}`}
        <p {...rest} data-index="chunk-47-{idx}" animate:flip>{itemLabel}</p>
    {/each}

    {#each items}
        <span class="item-less">Repeated shell chunk 47</span>
    {/each}

    {#each items as item (item.id)}
        <p animate:flip={{ duration: 200 }}>{item.name}</p>
    {:else}
        <p>No items in chunk 47</p>
    {/each}

    {#await promise}
        <p>Loading chunk 47...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    {#await promise then quickValue}
        <p>Quick resolved: {quickValue}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value="opt-1">One</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
        <option value={`opt-${counter}`}>{title}</option>
    </select>
    <input type="checkbox" bind:checked={checked} />
    <input type="checkbox" bind:group={group} value="opt-47" />
    <input type="radio" bind:group={group} value="opt-47" />
    <input type="number" bind:value={counter} />
    <details bind:open={checked}><summary>More</summary>chunk 47</details>
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div {@attach (node) => { node.dataset.chunk = "47"; }}>attach target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <p transition:slide|global>global transition chunk 47</p>
    <p in:fly|local={{ y: 40 }}>local transition chunk 47</p>

    <div style:opacity style:color={state}>style shorthand</div>
    <div class="slider chunk-47 {title}" class:invinsible>literal class</div>
    <div class={`chunk-${i}-${counter}`}>template literal class</div>
    <span data-note="&copy;=value" title="&copy 1998" aria-label="a &amp; b">entities</span>

    <template id="chunk-tpl-47">
        {@html "<b>templated</b>"}
        {#if counter}<span>inside template</span>{/if}
    </template>

    <svg viewBox="0 0 10 10" class="chunk-svg">
        <clipPath id="clip-47"><rect width="4" height="4" /></clipPath>
        <linearGradient id="grad-47"><stop offset="0%" stop-color={state} /></linearGradient>
        <foreignObject width="10" height="10"><div>svg foreign chunk 47</div></foreignObject>
    </svg>

    <math><mrow><mi>x</mi><mo>+</mo><mn>{counter}</mn></mrow></math>
    <svelte:element this={state ? "div" : "span"} class="dynamic-47">
        Dynamic element chunk 47: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()}>
        <strong>Inline child chunk 47: {title}</strong>
        <div slot="footer">Footer chunk 47: {counter}</div>
    </ChildComponent>

    <ChildComponent --accent="var(--custom, #5c4634)" {...rest} bind:config>
        {#snippet header(label)}
            <em>Header snippet chunk 47: {label}</em>
        {/snippet}
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={pageHeading} />

    {#if depth > 0}
        <svelte:self depth={depth - 1} title={title} />
    {/if}

    {@render badge("chunk-47", "secondary")}
    {@render card(title, "Content for chunk 47")}
    {@render metricSummary({ label: title, values: [count, doubled, counter], meta: { id: propsId } })}
    {@render show?.()}

    <button onclick={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 47: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 47: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-48">
    Chunk 48: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {moduleSummary} | Store: {storeSummary} | Label: {$labelStore}</p>

    {@html "<b>raw html chunk 48</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-48"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 48.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 48.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 48.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 48: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = `${idx}:${item.name}`}
        <p {...rest} data-index="chunk-48-{idx}" animate:flip>{itemLabel}</p>
    {/each}

    {#each items}
        <span class="item-less">Repeated shell chunk 48</span>
    {/each}

    {#each items as item (item.id)}
        <p animate:flip={{ duration: 200 }}>{item.name}</p>
    {:else}
        <p>No items in chunk 48</p>
    {/each}

    {#await promise}
        <p>Loading chunk 48...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    {#await promise then quickValue}
        <p>Quick resolved: {quickValue}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value="opt-1">One</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
        <option value={`opt-${counter}`}>{title}</option>
    </select>
    <input type="checkbox" bind:checked={checked} />
    <input type="checkbox" bind:group={group} value="opt-48" />
    <input type="radio" bind:group={group} value="opt-48" />
    <input type="number" bind:value={counter} />
    <details bind:open={checked}><summary>More</summary>chunk 48</details>
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div {@attach (node) => { node.dataset.chunk = "48"; }}>attach target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <p transition:slide|global>global transition chunk 48</p>
    <p in:fly|local={{ y: 40 }}>local transition chunk 48</p>

    <div style:opacity style:color={state}>style shorthand</div>
    <div class="slider chunk-48 {title}" class:invinsible>literal class</div>
    <div class={`chunk-${i}-${counter}`}>template literal class</div>
    <span data-note="&copy;=value" title="&copy 1998" aria-label="a &amp; b">entities</span>

    <template id="chunk-tpl-48">
        {@html "<b>templated</b>"}
        {#if counter}<span>inside template</span>{/if}
    </template>

    <svg viewBox="0 0 10 10" class="chunk-svg">
        <clipPath id="clip-48"><rect width="4" height="4" /></clipPath>
        <linearGradient id="grad-48"><stop offset="0%" stop-color={state} /></linearGradient>
        <foreignObject width="10" height="10"><div>svg foreign chunk 48</div></foreignObject>
    </svg>

    <math><mrow><mi>x</mi><mo>+</mo><mn>{counter}</mn></mrow></math>
    <svelte:element this={state ? "div" : "span"} class="dynamic-48">
        Dynamic element chunk 48: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()}>
        <strong>Inline child chunk 48: {title}</strong>
        <div slot="footer">Footer chunk 48: {counter}</div>
    </ChildComponent>

    <ChildComponent --accent="var(--custom, #5c4634)" {...rest} bind:config>
        {#snippet header(label)}
            <em>Header snippet chunk 48: {label}</em>
        {/snippet}
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={pageHeading} />

    {#if depth > 0}
        <svelte:self depth={depth - 1} title={title} />
    {/if}

    {@render badge("chunk-48", "secondary")}
    {@render card(title, "Content for chunk 48")}
    {@render metricSummary({ label: title, values: [count, doubled, counter], meta: { id: propsId } })}
    {@render show?.()}

    <button onclick={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 48: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 48: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-49">
    Chunk 49: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {moduleSummary} | Store: {storeSummary} | Label: {$labelStore}</p>

    {@html "<b>raw html chunk 49</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-49"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 49.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 49.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 49.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 49: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = `${idx}:${item.name}`}
        <p {...rest} data-index="chunk-49-{idx}" animate:flip>{itemLabel}</p>
    {/each}

    {#each items}
        <span class="item-less">Repeated shell chunk 49</span>
    {/each}

    {#each items as item (item.id)}
        <p animate:flip={{ duration: 200 }}>{item.name}</p>
    {:else}
        <p>No items in chunk 49</p>
    {/each}

    {#await promise}
        <p>Loading chunk 49...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    {#await promise then quickValue}
        <p>Quick resolved: {quickValue}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value="opt-1">One</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
        <option value={`opt-${counter}`}>{title}</option>
    </select>
    <input type="checkbox" bind:checked={checked} />
    <input type="checkbox" bind:group={group} value="opt-49" />
    <input type="radio" bind:group={group} value="opt-49" />
    <input type="number" bind:value={counter} />
    <details bind:open={checked}><summary>More</summary>chunk 49</details>
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div {@attach (node) => { node.dataset.chunk = "49"; }}>attach target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <p transition:slide|global>global transition chunk 49</p>
    <p in:fly|local={{ y: 40 }}>local transition chunk 49</p>

    <div style:opacity style:color={state}>style shorthand</div>
    <div class="slider chunk-49 {title}" class:invinsible>literal class</div>
    <div class={`chunk-${i}-${counter}`}>template literal class</div>
    <span data-note="&copy;=value" title="&copy 1998" aria-label="a &amp; b">entities</span>

    <template id="chunk-tpl-49">
        {@html "<b>templated</b>"}
        {#if counter}<span>inside template</span>{/if}
    </template>

    <svg viewBox="0 0 10 10" class="chunk-svg">
        <clipPath id="clip-49"><rect width="4" height="4" /></clipPath>
        <linearGradient id="grad-49"><stop offset="0%" stop-color={state} /></linearGradient>
        <foreignObject width="10" height="10"><div>svg foreign chunk 49</div></foreignObject>
    </svg>

    <math><mrow><mi>x</mi><mo>+</mo><mn>{counter}</mn></mrow></math>
    <svelte:element this={state ? "div" : "span"} class="dynamic-49">
        Dynamic element chunk 49: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()}>
        <strong>Inline child chunk 49: {title}</strong>
        <div slot="footer">Footer chunk 49: {counter}</div>
    </ChildComponent>

    <ChildComponent --accent="var(--custom, #5c4634)" {...rest} bind:config>
        {#snippet header(label)}
            <em>Header snippet chunk 49: {label}</em>
        {/snippet}
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={pageHeading} />

    {#if depth > 0}
        <svelte:self depth={depth - 1} title={title} />
    {/if}

    {@render badge("chunk-49", "secondary")}
    {@render card(title, "Content for chunk 49")}
    {@render metricSummary({ label: title, values: [count, doubled, counter], meta: { id: propsId } })}
    {@render show?.()}

    <button onclick={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 49: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 49: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

