export const example = [
  "<script>",
  '	let name = $state("world")',
  "</script>",
  "",
  "<h1>Hello {name}!</h1>",
  "",
].join("\n");

export const moduleExample = `let count = $state(0);
const doubled = $derived(count * 2);

export function increment() {
    count++;
}

export function getCount() {
    return count;
}

export function getDoubled() {
    return doubled;
}

$effect(() => {
    console.log("count changed:", count);
});
`;

export const benchmarkExample = `<script module>
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
        return \`\${BENCHMARK_KIND}:\${name}\`;
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
        content: "\\2605";
    }

    .\\32 5px-gap {
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
        {@const itemLabel = \`\${idx}:\${item.name}\`}
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
        <option value={\`opt-\${counter}\`}>{title}</option>
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
    <div class={\`chunk-\${i}-\${counter}\`}>template literal class</div>
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

`;
