<svelte:options runes={false} accessors={true} namespace="html" />

<script context="module">
    import { writable as makeStore } from "svelte/store";

    export const LEGACY_KIND = "compiler";
    export const LEGACY_SCALE = 3;

    export const sharedStore = makeStore(0);

    export function legacyLabel(name) {
        return LEGACY_KIND + ":" + name;
    }
</script>

<script>
    import { onMount, onDestroy, createEventDispatcher, tick } from "svelte";
    import { writable, derived as derivedStore, get as getStore } from "svelte/store";
    import { fade, fly, slide } from "svelte/transition";
    import { flip } from "svelte/animate";
    import ChildComponent from "./Child.svelte";

    export let title = "Default Title";
    export let count = 0;
    export let items = [];
    export let visible = false;
    export let config = {};
    export let multiplier = 2;

    let internalId = "legacy-0";

    export { internalId as id };

    export const LEGACY_VERSION = "1.0.0";

    export function formatTitle(prefix) {
        return prefix + ": " + title;
    }

    const dispatch = createEventDispatcher();

    let state = "";
    let counter = 0;
    let checked = false;
    let group = [];
    let selected = "opt-0";
    let selectedList = [];
    let rawData = { x: 1, y: 2 };
    let inputEl;
    let componentRef;
    let dynamicEl;

    let metrics = writable([1, 2, 3]);
    let labelStore = writable("ready");
    let metricsTotal = derivedStore(metrics, (values) => values.length);

    $: doubled = count * multiplier;
    $: computed = items.length * multiplier + counter;
    $: legacySummary = legacyLabel(title) + ":" + LEGACY_SCALE;
    $: storeSummary = $metrics.length + ":" + $labelStore + ":" + $metricsTotal;
    $: ({ x: rawX, y: rawY } = rawData);
    $: [firstItem = { name: "none" }] = items;
    $: propsKeys = Object.keys($$props).length + Object.keys($$restProps).length;

    $: if (counter > 5) {
        console.log("counter grew:", counter, rawX, rawY);
    }

    $: {
        const local = counter * 2;
        console.log("block reactive:", local, firstItem.name);
    }

    $: console.log("bare expression statement:", doubled);

    label: for (let i = 0; i < 1; i += 1) {
        if (i > 0) {
            break label;
        }
    }

    function handleClick(event) {
        counter += 1;
        dispatch("bump", { counter, title });
    }

    function handleKeydown(event) {
        if (event.key === "Enter") {
            counter -= 1;
        }
    }

    function getHandler() {
        return handleClick;
    }

    function handleError(error) {
        console.error(error);
    }

    function action(node, arg) {
        return {
            update(next) {},
            destroy() {}
        };
    }

    function addMetric() {
        $metrics = [...$metrics, counter];
        $labelStore = title;
        sharedStore.update((value) => value + 1);
    }

    onMount(() => {
        counter = 1;
        return () => {
            counter = 0;
        };
    });

    onDestroy(() => {
        console.log("destroyed", getStore(metrics));
    });

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
    <title>{title} - Legacy Benchmark</title>
    <meta name="description" content="Legacy benchmark component">
</svelte:head>

<svelte:window on:scroll={handleClick} on:keydown|once={handleKeydown} bind:scrollY={counter} />
<svelte:document on:visibilitychange={handleClick} />
<svelte:body on:mouseenter={handleClick} use:action={state} />

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-0">
    Chunk 0: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {legacySummary} | Store: {storeSummary} | Keys: {propsKeys}</p>
    <p>Destructured: {rawX}/{rawY} first={firstItem.name}</p>

    {@html "<b>raw html chunk 0</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:reactive={counter}
        style:color={state}
        style:--custom="value-0"
        on:click|preventDefault|stopPropagation={handleClick}
        on:click|once|capture={getHandler()}
        on:keydown|trusted={handleKeydown}
        on:mouseenter={() => (counter += 1)}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 0.
            </span>
        {:else if counter == 100}
            <h1 {state}>Lorem ipsum dolor sit amet. Chunk 0.</h1>
        {:else}
            <h2>EMPTY</h2>
        {/if}
    </div>

    {#key counter}
        <p transition:slide|global>Keyed content chunk 0: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = idx + ":" + item.name}
        <p {...$$restProps} data-index="chunk-0-{idx}" animate:flip={{ duration: 200 }}>
            {itemLabel}
        </p>
    {:else}
        <p>No items in chunk 0</p>
    {/each}

    {#each items as item, idx}
        <input bind:value={item.name} />
        <input type="checkbox" bind:checked={item.done} />
        <span>{idx}: {item.name}</span>
    {/each}

    {#each $metrics as metric, metricIdx}
        <input bind:value={$metrics[metricIdx]} />
        <span class="item-less">Metric {metric}</span>
    {/each}

    {#await promise}
        <p>Loading chunk 0...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value={"opt-" + counter}>{title}</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
    </select>
    <input type="checkbox" bind:group={group} value="opt-0" />
    <input type="radio" bind:group={group} value="opt-0" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly|local={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-0">
        Dynamic element chunk 0: {title}
    </svelte:element>

    <slot name="header" {counter} label={title} />
    {#if $$slots.footer}
        <slot name="footer" {counter} />
    {/if}
    <slot>Default slot fallback chunk 0</slot>

    <ChildComponent
        bind:this={componentRef}
        title={title}
        on:bump={handleClick}
        on:custom={() => (counter += 1)}
        let:item
        let:index={childIndex}
    >
        <strong>Inline child chunk 0: {item} / {childIndex}</strong>
        <svelte:fragment slot="footer" let:value>
            <div>Footer chunk 0: {value} / {counter}</div>
        </svelte:fragment>
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={title} />

    {#if counter > 1000}
        <svelte:self title={title} count={count - 1} />
    {/if}

    <button on:click={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 0: {title}</p>
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-1">
    Chunk 1: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {legacySummary} | Store: {storeSummary} | Keys: {propsKeys}</p>
    <p>Destructured: {rawX}/{rawY} first={firstItem.name}</p>

    {@html "<b>raw html chunk 1</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:reactive={counter}
        style:color={state}
        style:--custom="value-1"
        on:click|preventDefault|stopPropagation={handleClick}
        on:click|once|capture={getHandler()}
        on:keydown|trusted={handleKeydown}
        on:mouseenter={() => (counter += 1)}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 1.
            </span>
        {:else if counter == 100}
            <h1 {state}>Lorem ipsum dolor sit amet. Chunk 1.</h1>
        {:else}
            <h2>EMPTY</h2>
        {/if}
    </div>

    {#key counter}
        <p transition:slide|global>Keyed content chunk 1: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = idx + ":" + item.name}
        <p {...$$restProps} data-index="chunk-1-{idx}" animate:flip={{ duration: 200 }}>
            {itemLabel}
        </p>
    {:else}
        <p>No items in chunk 1</p>
    {/each}

    {#each items as item, idx}
        <input bind:value={item.name} />
        <input type="checkbox" bind:checked={item.done} />
        <span>{idx}: {item.name}</span>
    {/each}

    {#each $metrics as metric, metricIdx}
        <input bind:value={$metrics[metricIdx]} />
        <span class="item-less">Metric {metric}</span>
    {/each}

    {#await promise}
        <p>Loading chunk 1...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value={"opt-" + counter}>{title}</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
    </select>
    <input type="checkbox" bind:group={group} value="opt-1" />
    <input type="radio" bind:group={group} value="opt-1" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly|local={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-1">
        Dynamic element chunk 1: {title}
    </svelte:element>

    <slot name="header" {counter} label={title} />
    {#if $$slots.footer}
        <slot name="footer" {counter} />
    {/if}
    <slot>Default slot fallback chunk 1</slot>

    <ChildComponent
        bind:this={componentRef}
        title={title}
        on:bump={handleClick}
        on:custom={() => (counter += 1)}
        let:item
        let:index={childIndex}
    >
        <strong>Inline child chunk 1: {item} / {childIndex}</strong>
        <svelte:fragment slot="footer" let:value>
            <div>Footer chunk 1: {value} / {counter}</div>
        </svelte:fragment>
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={title} />

    {#if counter > 1000}
        <svelte:self title={title} count={count - 1} />
    {/if}

    <button on:click={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 1: {title}</p>
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-2">
    Chunk 2: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {legacySummary} | Store: {storeSummary} | Keys: {propsKeys}</p>
    <p>Destructured: {rawX}/{rawY} first={firstItem.name}</p>

    {@html "<b>raw html chunk 2</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:reactive={counter}
        style:color={state}
        style:--custom="value-2"
        on:click|preventDefault|stopPropagation={handleClick}
        on:click|once|capture={getHandler()}
        on:keydown|trusted={handleKeydown}
        on:mouseenter={() => (counter += 1)}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 2.
            </span>
        {:else if counter == 100}
            <h1 {state}>Lorem ipsum dolor sit amet. Chunk 2.</h1>
        {:else}
            <h2>EMPTY</h2>
        {/if}
    </div>

    {#key counter}
        <p transition:slide|global>Keyed content chunk 2: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = idx + ":" + item.name}
        <p {...$$restProps} data-index="chunk-2-{idx}" animate:flip={{ duration: 200 }}>
            {itemLabel}
        </p>
    {:else}
        <p>No items in chunk 2</p>
    {/each}

    {#each items as item, idx}
        <input bind:value={item.name} />
        <input type="checkbox" bind:checked={item.done} />
        <span>{idx}: {item.name}</span>
    {/each}

    {#each $metrics as metric, metricIdx}
        <input bind:value={$metrics[metricIdx]} />
        <span class="item-less">Metric {metric}</span>
    {/each}

    {#await promise}
        <p>Loading chunk 2...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value={"opt-" + counter}>{title}</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
    </select>
    <input type="checkbox" bind:group={group} value="opt-2" />
    <input type="radio" bind:group={group} value="opt-2" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly|local={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-2">
        Dynamic element chunk 2: {title}
    </svelte:element>

    <slot name="header" {counter} label={title} />
    {#if $$slots.footer}
        <slot name="footer" {counter} />
    {/if}
    <slot>Default slot fallback chunk 2</slot>

    <ChildComponent
        bind:this={componentRef}
        title={title}
        on:bump={handleClick}
        on:custom={() => (counter += 1)}
        let:item
        let:index={childIndex}
    >
        <strong>Inline child chunk 2: {item} / {childIndex}</strong>
        <svelte:fragment slot="footer" let:value>
            <div>Footer chunk 2: {value} / {counter}</div>
        </svelte:fragment>
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={title} />

    {#if counter > 1000}
        <svelte:self title={title} count={count - 1} />
    {/if}

    <button on:click={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 2: {title}</p>
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-3">
    Chunk 3: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {legacySummary} | Store: {storeSummary} | Keys: {propsKeys}</p>
    <p>Destructured: {rawX}/{rawY} first={firstItem.name}</p>

    {@html "<b>raw html chunk 3</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:reactive={counter}
        style:color={state}
        style:--custom="value-3"
        on:click|preventDefault|stopPropagation={handleClick}
        on:click|once|capture={getHandler()}
        on:keydown|trusted={handleKeydown}
        on:mouseenter={() => (counter += 1)}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 3.
            </span>
        {:else if counter == 100}
            <h1 {state}>Lorem ipsum dolor sit amet. Chunk 3.</h1>
        {:else}
            <h2>EMPTY</h2>
        {/if}
    </div>

    {#key counter}
        <p transition:slide|global>Keyed content chunk 3: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = idx + ":" + item.name}
        <p {...$$restProps} data-index="chunk-3-{idx}" animate:flip={{ duration: 200 }}>
            {itemLabel}
        </p>
    {:else}
        <p>No items in chunk 3</p>
    {/each}

    {#each items as item, idx}
        <input bind:value={item.name} />
        <input type="checkbox" bind:checked={item.done} />
        <span>{idx}: {item.name}</span>
    {/each}

    {#each $metrics as metric, metricIdx}
        <input bind:value={$metrics[metricIdx]} />
        <span class="item-less">Metric {metric}</span>
    {/each}

    {#await promise}
        <p>Loading chunk 3...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value={"opt-" + counter}>{title}</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
    </select>
    <input type="checkbox" bind:group={group} value="opt-3" />
    <input type="radio" bind:group={group} value="opt-3" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly|local={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-3">
        Dynamic element chunk 3: {title}
    </svelte:element>

    <slot name="header" {counter} label={title} />
    {#if $$slots.footer}
        <slot name="footer" {counter} />
    {/if}
    <slot>Default slot fallback chunk 3</slot>

    <ChildComponent
        bind:this={componentRef}
        title={title}
        on:bump={handleClick}
        on:custom={() => (counter += 1)}
        let:item
        let:index={childIndex}
    >
        <strong>Inline child chunk 3: {item} / {childIndex}</strong>
        <svelte:fragment slot="footer" let:value>
            <div>Footer chunk 3: {value} / {counter}</div>
        </svelte:fragment>
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={title} />

    {#if counter > 1000}
        <svelte:self title={title} count={count - 1} />
    {/if}

    <button on:click={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 3: {title}</p>
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-4">
    Chunk 4: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {legacySummary} | Store: {storeSummary} | Keys: {propsKeys}</p>
    <p>Destructured: {rawX}/{rawY} first={firstItem.name}</p>

    {@html "<b>raw html chunk 4</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:reactive={counter}
        style:color={state}
        style:--custom="value-4"
        on:click|preventDefault|stopPropagation={handleClick}
        on:click|once|capture={getHandler()}
        on:keydown|trusted={handleKeydown}
        on:mouseenter={() => (counter += 1)}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 4.
            </span>
        {:else if counter == 100}
            <h1 {state}>Lorem ipsum dolor sit amet. Chunk 4.</h1>
        {:else}
            <h2>EMPTY</h2>
        {/if}
    </div>

    {#key counter}
        <p transition:slide|global>Keyed content chunk 4: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = idx + ":" + item.name}
        <p {...$$restProps} data-index="chunk-4-{idx}" animate:flip={{ duration: 200 }}>
            {itemLabel}
        </p>
    {:else}
        <p>No items in chunk 4</p>
    {/each}

    {#each items as item, idx}
        <input bind:value={item.name} />
        <input type="checkbox" bind:checked={item.done} />
        <span>{idx}: {item.name}</span>
    {/each}

    {#each $metrics as metric, metricIdx}
        <input bind:value={$metrics[metricIdx]} />
        <span class="item-less">Metric {metric}</span>
    {/each}

    {#await promise}
        <p>Loading chunk 4...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value={"opt-" + counter}>{title}</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
    </select>
    <input type="checkbox" bind:group={group} value="opt-4" />
    <input type="radio" bind:group={group} value="opt-4" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly|local={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-4">
        Dynamic element chunk 4: {title}
    </svelte:element>

    <slot name="header" {counter} label={title} />
    {#if $$slots.footer}
        <slot name="footer" {counter} />
    {/if}
    <slot>Default slot fallback chunk 4</slot>

    <ChildComponent
        bind:this={componentRef}
        title={title}
        on:bump={handleClick}
        on:custom={() => (counter += 1)}
        let:item
        let:index={childIndex}
    >
        <strong>Inline child chunk 4: {item} / {childIndex}</strong>
        <svelte:fragment slot="footer" let:value>
            <div>Footer chunk 4: {value} / {counter}</div>
        </svelte:fragment>
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={title} />

    {#if counter > 1000}
        <svelte:self title={title} count={count - 1} />
    {/if}

    <button on:click={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 4: {title}</p>
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-5">
    Chunk 5: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {legacySummary} | Store: {storeSummary} | Keys: {propsKeys}</p>
    <p>Destructured: {rawX}/{rawY} first={firstItem.name}</p>

    {@html "<b>raw html chunk 5</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:reactive={counter}
        style:color={state}
        style:--custom="value-5"
        on:click|preventDefault|stopPropagation={handleClick}
        on:click|once|capture={getHandler()}
        on:keydown|trusted={handleKeydown}
        on:mouseenter={() => (counter += 1)}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 5.
            </span>
        {:else if counter == 100}
            <h1 {state}>Lorem ipsum dolor sit amet. Chunk 5.</h1>
        {:else}
            <h2>EMPTY</h2>
        {/if}
    </div>

    {#key counter}
        <p transition:slide|global>Keyed content chunk 5: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = idx + ":" + item.name}
        <p {...$$restProps} data-index="chunk-5-{idx}" animate:flip={{ duration: 200 }}>
            {itemLabel}
        </p>
    {:else}
        <p>No items in chunk 5</p>
    {/each}

    {#each items as item, idx}
        <input bind:value={item.name} />
        <input type="checkbox" bind:checked={item.done} />
        <span>{idx}: {item.name}</span>
    {/each}

    {#each $metrics as metric, metricIdx}
        <input bind:value={$metrics[metricIdx]} />
        <span class="item-less">Metric {metric}</span>
    {/each}

    {#await promise}
        <p>Loading chunk 5...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value={"opt-" + counter}>{title}</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
    </select>
    <input type="checkbox" bind:group={group} value="opt-5" />
    <input type="radio" bind:group={group} value="opt-5" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly|local={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-5">
        Dynamic element chunk 5: {title}
    </svelte:element>

    <slot name="header" {counter} label={title} />
    {#if $$slots.footer}
        <slot name="footer" {counter} />
    {/if}
    <slot>Default slot fallback chunk 5</slot>

    <ChildComponent
        bind:this={componentRef}
        title={title}
        on:bump={handleClick}
        on:custom={() => (counter += 1)}
        let:item
        let:index={childIndex}
    >
        <strong>Inline child chunk 5: {item} / {childIndex}</strong>
        <svelte:fragment slot="footer" let:value>
            <div>Footer chunk 5: {value} / {counter}</div>
        </svelte:fragment>
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={title} />

    {#if counter > 1000}
        <svelte:self title={title} count={count - 1} />
    {/if}

    <button on:click={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 5: {title}</p>
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-6">
    Chunk 6: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {legacySummary} | Store: {storeSummary} | Keys: {propsKeys}</p>
    <p>Destructured: {rawX}/{rawY} first={firstItem.name}</p>

    {@html "<b>raw html chunk 6</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:reactive={counter}
        style:color={state}
        style:--custom="value-6"
        on:click|preventDefault|stopPropagation={handleClick}
        on:click|once|capture={getHandler()}
        on:keydown|trusted={handleKeydown}
        on:mouseenter={() => (counter += 1)}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 6.
            </span>
        {:else if counter == 100}
            <h1 {state}>Lorem ipsum dolor sit amet. Chunk 6.</h1>
        {:else}
            <h2>EMPTY</h2>
        {/if}
    </div>

    {#key counter}
        <p transition:slide|global>Keyed content chunk 6: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = idx + ":" + item.name}
        <p {...$$restProps} data-index="chunk-6-{idx}" animate:flip={{ duration: 200 }}>
            {itemLabel}
        </p>
    {:else}
        <p>No items in chunk 6</p>
    {/each}

    {#each items as item, idx}
        <input bind:value={item.name} />
        <input type="checkbox" bind:checked={item.done} />
        <span>{idx}: {item.name}</span>
    {/each}

    {#each $metrics as metric, metricIdx}
        <input bind:value={$metrics[metricIdx]} />
        <span class="item-less">Metric {metric}</span>
    {/each}

    {#await promise}
        <p>Loading chunk 6...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value={"opt-" + counter}>{title}</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
    </select>
    <input type="checkbox" bind:group={group} value="opt-6" />
    <input type="radio" bind:group={group} value="opt-6" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly|local={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-6">
        Dynamic element chunk 6: {title}
    </svelte:element>

    <slot name="header" {counter} label={title} />
    {#if $$slots.footer}
        <slot name="footer" {counter} />
    {/if}
    <slot>Default slot fallback chunk 6</slot>

    <ChildComponent
        bind:this={componentRef}
        title={title}
        on:bump={handleClick}
        on:custom={() => (counter += 1)}
        let:item
        let:index={childIndex}
    >
        <strong>Inline child chunk 6: {item} / {childIndex}</strong>
        <svelte:fragment slot="footer" let:value>
            <div>Footer chunk 6: {value} / {counter}</div>
        </svelte:fragment>
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={title} />

    {#if counter > 1000}
        <svelte:self title={title} count={count - 1} />
    {/if}

    <button on:click={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 6: {title}</p>
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-7">
    Chunk 7: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {legacySummary} | Store: {storeSummary} | Keys: {propsKeys}</p>
    <p>Destructured: {rawX}/{rawY} first={firstItem.name}</p>

    {@html "<b>raw html chunk 7</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:reactive={counter}
        style:color={state}
        style:--custom="value-7"
        on:click|preventDefault|stopPropagation={handleClick}
        on:click|once|capture={getHandler()}
        on:keydown|trusted={handleKeydown}
        on:mouseenter={() => (counter += 1)}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 7.
            </span>
        {:else if counter == 100}
            <h1 {state}>Lorem ipsum dolor sit amet. Chunk 7.</h1>
        {:else}
            <h2>EMPTY</h2>
        {/if}
    </div>

    {#key counter}
        <p transition:slide|global>Keyed content chunk 7: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = idx + ":" + item.name}
        <p {...$$restProps} data-index="chunk-7-{idx}" animate:flip={{ duration: 200 }}>
            {itemLabel}
        </p>
    {:else}
        <p>No items in chunk 7</p>
    {/each}

    {#each items as item, idx}
        <input bind:value={item.name} />
        <input type="checkbox" bind:checked={item.done} />
        <span>{idx}: {item.name}</span>
    {/each}

    {#each $metrics as metric, metricIdx}
        <input bind:value={$metrics[metricIdx]} />
        <span class="item-less">Metric {metric}</span>
    {/each}

    {#await promise}
        <p>Loading chunk 7...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value={"opt-" + counter}>{title}</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
    </select>
    <input type="checkbox" bind:group={group} value="opt-7" />
    <input type="radio" bind:group={group} value="opt-7" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly|local={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-7">
        Dynamic element chunk 7: {title}
    </svelte:element>

    <slot name="header" {counter} label={title} />
    {#if $$slots.footer}
        <slot name="footer" {counter} />
    {/if}
    <slot>Default slot fallback chunk 7</slot>

    <ChildComponent
        bind:this={componentRef}
        title={title}
        on:bump={handleClick}
        on:custom={() => (counter += 1)}
        let:item
        let:index={childIndex}
    >
        <strong>Inline child chunk 7: {item} / {childIndex}</strong>
        <svelte:fragment slot="footer" let:value>
            <div>Footer chunk 7: {value} / {counter}</div>
        </svelte:fragment>
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={title} />

    {#if counter > 1000}
        <svelte:self title={title} count={count - 1} />
    {/if}

    <button on:click={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 7: {title}</p>
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-8">
    Chunk 8: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {legacySummary} | Store: {storeSummary} | Keys: {propsKeys}</p>
    <p>Destructured: {rawX}/{rawY} first={firstItem.name}</p>

    {@html "<b>raw html chunk 8</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:reactive={counter}
        style:color={state}
        style:--custom="value-8"
        on:click|preventDefault|stopPropagation={handleClick}
        on:click|once|capture={getHandler()}
        on:keydown|trusted={handleKeydown}
        on:mouseenter={() => (counter += 1)}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 8.
            </span>
        {:else if counter == 100}
            <h1 {state}>Lorem ipsum dolor sit amet. Chunk 8.</h1>
        {:else}
            <h2>EMPTY</h2>
        {/if}
    </div>

    {#key counter}
        <p transition:slide|global>Keyed content chunk 8: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = idx + ":" + item.name}
        <p {...$$restProps} data-index="chunk-8-{idx}" animate:flip={{ duration: 200 }}>
            {itemLabel}
        </p>
    {:else}
        <p>No items in chunk 8</p>
    {/each}

    {#each items as item, idx}
        <input bind:value={item.name} />
        <input type="checkbox" bind:checked={item.done} />
        <span>{idx}: {item.name}</span>
    {/each}

    {#each $metrics as metric, metricIdx}
        <input bind:value={$metrics[metricIdx]} />
        <span class="item-less">Metric {metric}</span>
    {/each}

    {#await promise}
        <p>Loading chunk 8...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value={"opt-" + counter}>{title}</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
    </select>
    <input type="checkbox" bind:group={group} value="opt-8" />
    <input type="radio" bind:group={group} value="opt-8" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly|local={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-8">
        Dynamic element chunk 8: {title}
    </svelte:element>

    <slot name="header" {counter} label={title} />
    {#if $$slots.footer}
        <slot name="footer" {counter} />
    {/if}
    <slot>Default slot fallback chunk 8</slot>

    <ChildComponent
        bind:this={componentRef}
        title={title}
        on:bump={handleClick}
        on:custom={() => (counter += 1)}
        let:item
        let:index={childIndex}
    >
        <strong>Inline child chunk 8: {item} / {childIndex}</strong>
        <svelte:fragment slot="footer" let:value>
            <div>Footer chunk 8: {value} / {counter}</div>
        </svelte:fragment>
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={title} />

    {#if counter > 1000}
        <svelte:self title={title} count={count - 1} />
    {/if}

    <button on:click={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 8: {title}</p>
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-9">
    Chunk 9: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {legacySummary} | Store: {storeSummary} | Keys: {propsKeys}</p>
    <p>Destructured: {rawX}/{rawY} first={firstItem.name}</p>

    {@html "<b>raw html chunk 9</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:reactive={counter}
        style:color={state}
        style:--custom="value-9"
        on:click|preventDefault|stopPropagation={handleClick}
        on:click|once|capture={getHandler()}
        on:keydown|trusted={handleKeydown}
        on:mouseenter={() => (counter += 1)}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 9.
            </span>
        {:else if counter == 100}
            <h1 {state}>Lorem ipsum dolor sit amet. Chunk 9.</h1>
        {:else}
            <h2>EMPTY</h2>
        {/if}
    </div>

    {#key counter}
        <p transition:slide|global>Keyed content chunk 9: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = idx + ":" + item.name}
        <p {...$$restProps} data-index="chunk-9-{idx}" animate:flip={{ duration: 200 }}>
            {itemLabel}
        </p>
    {:else}
        <p>No items in chunk 9</p>
    {/each}

    {#each items as item, idx}
        <input bind:value={item.name} />
        <input type="checkbox" bind:checked={item.done} />
        <span>{idx}: {item.name}</span>
    {/each}

    {#each $metrics as metric, metricIdx}
        <input bind:value={$metrics[metricIdx]} />
        <span class="item-less">Metric {metric}</span>
    {/each}

    {#await promise}
        <p>Loading chunk 9...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value={"opt-" + counter}>{title}</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
    </select>
    <input type="checkbox" bind:group={group} value="opt-9" />
    <input type="radio" bind:group={group} value="opt-9" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly|local={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-9">
        Dynamic element chunk 9: {title}
    </svelte:element>

    <slot name="header" {counter} label={title} />
    {#if $$slots.footer}
        <slot name="footer" {counter} />
    {/if}
    <slot>Default slot fallback chunk 9</slot>

    <ChildComponent
        bind:this={componentRef}
        title={title}
        on:bump={handleClick}
        on:custom={() => (counter += 1)}
        let:item
        let:index={childIndex}
    >
        <strong>Inline child chunk 9: {item} / {childIndex}</strong>
        <svelte:fragment slot="footer" let:value>
            <div>Footer chunk 9: {value} / {counter}</div>
        </svelte:fragment>
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={title} />

    {#if counter > 1000}
        <svelte:self title={title} count={count - 1} />
    {/if}

    <button on:click={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 9: {title}</p>
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-10">
    Chunk 10: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {legacySummary} | Store: {storeSummary} | Keys: {propsKeys}</p>
    <p>Destructured: {rawX}/{rawY} first={firstItem.name}</p>

    {@html "<b>raw html chunk 10</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:reactive={counter}
        style:color={state}
        style:--custom="value-10"
        on:click|preventDefault|stopPropagation={handleClick}
        on:click|once|capture={getHandler()}
        on:keydown|trusted={handleKeydown}
        on:mouseenter={() => (counter += 1)}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 10.
            </span>
        {:else if counter == 100}
            <h1 {state}>Lorem ipsum dolor sit amet. Chunk 10.</h1>
        {:else}
            <h2>EMPTY</h2>
        {/if}
    </div>

    {#key counter}
        <p transition:slide|global>Keyed content chunk 10: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = idx + ":" + item.name}
        <p {...$$restProps} data-index="chunk-10-{idx}" animate:flip={{ duration: 200 }}>
            {itemLabel}
        </p>
    {:else}
        <p>No items in chunk 10</p>
    {/each}

    {#each items as item, idx}
        <input bind:value={item.name} />
        <input type="checkbox" bind:checked={item.done} />
        <span>{idx}: {item.name}</span>
    {/each}

    {#each $metrics as metric, metricIdx}
        <input bind:value={$metrics[metricIdx]} />
        <span class="item-less">Metric {metric}</span>
    {/each}

    {#await promise}
        <p>Loading chunk 10...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value={"opt-" + counter}>{title}</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
    </select>
    <input type="checkbox" bind:group={group} value="opt-10" />
    <input type="radio" bind:group={group} value="opt-10" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly|local={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-10">
        Dynamic element chunk 10: {title}
    </svelte:element>

    <slot name="header" {counter} label={title} />
    {#if $$slots.footer}
        <slot name="footer" {counter} />
    {/if}
    <slot>Default slot fallback chunk 10</slot>

    <ChildComponent
        bind:this={componentRef}
        title={title}
        on:bump={handleClick}
        on:custom={() => (counter += 1)}
        let:item
        let:index={childIndex}
    >
        <strong>Inline child chunk 10: {item} / {childIndex}</strong>
        <svelte:fragment slot="footer" let:value>
            <div>Footer chunk 10: {value} / {counter}</div>
        </svelte:fragment>
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={title} />

    {#if counter > 1000}
        <svelte:self title={title} count={count - 1} />
    {/if}

    <button on:click={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 10: {title}</p>
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-11">
    Chunk 11: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {legacySummary} | Store: {storeSummary} | Keys: {propsKeys}</p>
    <p>Destructured: {rawX}/{rawY} first={firstItem.name}</p>

    {@html "<b>raw html chunk 11</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:reactive={counter}
        style:color={state}
        style:--custom="value-11"
        on:click|preventDefault|stopPropagation={handleClick}
        on:click|once|capture={getHandler()}
        on:keydown|trusted={handleKeydown}
        on:mouseenter={() => (counter += 1)}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 11.
            </span>
        {:else if counter == 100}
            <h1 {state}>Lorem ipsum dolor sit amet. Chunk 11.</h1>
        {:else}
            <h2>EMPTY</h2>
        {/if}
    </div>

    {#key counter}
        <p transition:slide|global>Keyed content chunk 11: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = idx + ":" + item.name}
        <p {...$$restProps} data-index="chunk-11-{idx}" animate:flip={{ duration: 200 }}>
            {itemLabel}
        </p>
    {:else}
        <p>No items in chunk 11</p>
    {/each}

    {#each items as item, idx}
        <input bind:value={item.name} />
        <input type="checkbox" bind:checked={item.done} />
        <span>{idx}: {item.name}</span>
    {/each}

    {#each $metrics as metric, metricIdx}
        <input bind:value={$metrics[metricIdx]} />
        <span class="item-less">Metric {metric}</span>
    {/each}

    {#await promise}
        <p>Loading chunk 11...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value={"opt-" + counter}>{title}</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
    </select>
    <input type="checkbox" bind:group={group} value="opt-11" />
    <input type="radio" bind:group={group} value="opt-11" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly|local={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-11">
        Dynamic element chunk 11: {title}
    </svelte:element>

    <slot name="header" {counter} label={title} />
    {#if $$slots.footer}
        <slot name="footer" {counter} />
    {/if}
    <slot>Default slot fallback chunk 11</slot>

    <ChildComponent
        bind:this={componentRef}
        title={title}
        on:bump={handleClick}
        on:custom={() => (counter += 1)}
        let:item
        let:index={childIndex}
    >
        <strong>Inline child chunk 11: {item} / {childIndex}</strong>
        <svelte:fragment slot="footer" let:value>
            <div>Footer chunk 11: {value} / {counter}</div>
        </svelte:fragment>
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={title} />

    {#if counter > 1000}
        <svelte:self title={title} count={count - 1} />
    {/if}

    <button on:click={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 11: {title}</p>
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-12">
    Chunk 12: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {legacySummary} | Store: {storeSummary} | Keys: {propsKeys}</p>
    <p>Destructured: {rawX}/{rawY} first={firstItem.name}</p>

    {@html "<b>raw html chunk 12</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:reactive={counter}
        style:color={state}
        style:--custom="value-12"
        on:click|preventDefault|stopPropagation={handleClick}
        on:click|once|capture={getHandler()}
        on:keydown|trusted={handleKeydown}
        on:mouseenter={() => (counter += 1)}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 12.
            </span>
        {:else if counter == 100}
            <h1 {state}>Lorem ipsum dolor sit amet. Chunk 12.</h1>
        {:else}
            <h2>EMPTY</h2>
        {/if}
    </div>

    {#key counter}
        <p transition:slide|global>Keyed content chunk 12: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = idx + ":" + item.name}
        <p {...$$restProps} data-index="chunk-12-{idx}" animate:flip={{ duration: 200 }}>
            {itemLabel}
        </p>
    {:else}
        <p>No items in chunk 12</p>
    {/each}

    {#each items as item, idx}
        <input bind:value={item.name} />
        <input type="checkbox" bind:checked={item.done} />
        <span>{idx}: {item.name}</span>
    {/each}

    {#each $metrics as metric, metricIdx}
        <input bind:value={$metrics[metricIdx]} />
        <span class="item-less">Metric {metric}</span>
    {/each}

    {#await promise}
        <p>Loading chunk 12...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value={"opt-" + counter}>{title}</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
    </select>
    <input type="checkbox" bind:group={group} value="opt-12" />
    <input type="radio" bind:group={group} value="opt-12" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly|local={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-12">
        Dynamic element chunk 12: {title}
    </svelte:element>

    <slot name="header" {counter} label={title} />
    {#if $$slots.footer}
        <slot name="footer" {counter} />
    {/if}
    <slot>Default slot fallback chunk 12</slot>

    <ChildComponent
        bind:this={componentRef}
        title={title}
        on:bump={handleClick}
        on:custom={() => (counter += 1)}
        let:item
        let:index={childIndex}
    >
        <strong>Inline child chunk 12: {item} / {childIndex}</strong>
        <svelte:fragment slot="footer" let:value>
            <div>Footer chunk 12: {value} / {counter}</div>
        </svelte:fragment>
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={title} />

    {#if counter > 1000}
        <svelte:self title={title} count={count - 1} />
    {/if}

    <button on:click={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 12: {title}</p>
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-13">
    Chunk 13: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {legacySummary} | Store: {storeSummary} | Keys: {propsKeys}</p>
    <p>Destructured: {rawX}/{rawY} first={firstItem.name}</p>

    {@html "<b>raw html chunk 13</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:reactive={counter}
        style:color={state}
        style:--custom="value-13"
        on:click|preventDefault|stopPropagation={handleClick}
        on:click|once|capture={getHandler()}
        on:keydown|trusted={handleKeydown}
        on:mouseenter={() => (counter += 1)}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 13.
            </span>
        {:else if counter == 100}
            <h1 {state}>Lorem ipsum dolor sit amet. Chunk 13.</h1>
        {:else}
            <h2>EMPTY</h2>
        {/if}
    </div>

    {#key counter}
        <p transition:slide|global>Keyed content chunk 13: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = idx + ":" + item.name}
        <p {...$$restProps} data-index="chunk-13-{idx}" animate:flip={{ duration: 200 }}>
            {itemLabel}
        </p>
    {:else}
        <p>No items in chunk 13</p>
    {/each}

    {#each items as item, idx}
        <input bind:value={item.name} />
        <input type="checkbox" bind:checked={item.done} />
        <span>{idx}: {item.name}</span>
    {/each}

    {#each $metrics as metric, metricIdx}
        <input bind:value={$metrics[metricIdx]} />
        <span class="item-less">Metric {metric}</span>
    {/each}

    {#await promise}
        <p>Loading chunk 13...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value={"opt-" + counter}>{title}</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
    </select>
    <input type="checkbox" bind:group={group} value="opt-13" />
    <input type="radio" bind:group={group} value="opt-13" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly|local={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-13">
        Dynamic element chunk 13: {title}
    </svelte:element>

    <slot name="header" {counter} label={title} />
    {#if $$slots.footer}
        <slot name="footer" {counter} />
    {/if}
    <slot>Default slot fallback chunk 13</slot>

    <ChildComponent
        bind:this={componentRef}
        title={title}
        on:bump={handleClick}
        on:custom={() => (counter += 1)}
        let:item
        let:index={childIndex}
    >
        <strong>Inline child chunk 13: {item} / {childIndex}</strong>
        <svelte:fragment slot="footer" let:value>
            <div>Footer chunk 13: {value} / {counter}</div>
        </svelte:fragment>
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={title} />

    {#if counter > 1000}
        <svelte:self title={title} count={count - 1} />
    {/if}

    <button on:click={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 13: {title}</p>
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-14">
    Chunk 14: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {legacySummary} | Store: {storeSummary} | Keys: {propsKeys}</p>
    <p>Destructured: {rawX}/{rawY} first={firstItem.name}</p>

    {@html "<b>raw html chunk 14</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:reactive={counter}
        style:color={state}
        style:--custom="value-14"
        on:click|preventDefault|stopPropagation={handleClick}
        on:click|once|capture={getHandler()}
        on:keydown|trusted={handleKeydown}
        on:mouseenter={() => (counter += 1)}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 14.
            </span>
        {:else if counter == 100}
            <h1 {state}>Lorem ipsum dolor sit amet. Chunk 14.</h1>
        {:else}
            <h2>EMPTY</h2>
        {/if}
    </div>

    {#key counter}
        <p transition:slide|global>Keyed content chunk 14: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = idx + ":" + item.name}
        <p {...$$restProps} data-index="chunk-14-{idx}" animate:flip={{ duration: 200 }}>
            {itemLabel}
        </p>
    {:else}
        <p>No items in chunk 14</p>
    {/each}

    {#each items as item, idx}
        <input bind:value={item.name} />
        <input type="checkbox" bind:checked={item.done} />
        <span>{idx}: {item.name}</span>
    {/each}

    {#each $metrics as metric, metricIdx}
        <input bind:value={$metrics[metricIdx]} />
        <span class="item-less">Metric {metric}</span>
    {/each}

    {#await promise}
        <p>Loading chunk 14...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value={"opt-" + counter}>{title}</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
    </select>
    <input type="checkbox" bind:group={group} value="opt-14" />
    <input type="radio" bind:group={group} value="opt-14" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly|local={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-14">
        Dynamic element chunk 14: {title}
    </svelte:element>

    <slot name="header" {counter} label={title} />
    {#if $$slots.footer}
        <slot name="footer" {counter} />
    {/if}
    <slot>Default slot fallback chunk 14</slot>

    <ChildComponent
        bind:this={componentRef}
        title={title}
        on:bump={handleClick}
        on:custom={() => (counter += 1)}
        let:item
        let:index={childIndex}
    >
        <strong>Inline child chunk 14: {item} / {childIndex}</strong>
        <svelte:fragment slot="footer" let:value>
            <div>Footer chunk 14: {value} / {counter}</div>
        </svelte:fragment>
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={title} />

    {#if counter > 1000}
        <svelte:self title={title} count={count - 1} />
    {/if}

    <button on:click={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 14: {title}</p>
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-15">
    Chunk 15: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {legacySummary} | Store: {storeSummary} | Keys: {propsKeys}</p>
    <p>Destructured: {rawX}/{rawY} first={firstItem.name}</p>

    {@html "<b>raw html chunk 15</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:reactive={counter}
        style:color={state}
        style:--custom="value-15"
        on:click|preventDefault|stopPropagation={handleClick}
        on:click|once|capture={getHandler()}
        on:keydown|trusted={handleKeydown}
        on:mouseenter={() => (counter += 1)}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 15.
            </span>
        {:else if counter == 100}
            <h1 {state}>Lorem ipsum dolor sit amet. Chunk 15.</h1>
        {:else}
            <h2>EMPTY</h2>
        {/if}
    </div>

    {#key counter}
        <p transition:slide|global>Keyed content chunk 15: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = idx + ":" + item.name}
        <p {...$$restProps} data-index="chunk-15-{idx}" animate:flip={{ duration: 200 }}>
            {itemLabel}
        </p>
    {:else}
        <p>No items in chunk 15</p>
    {/each}

    {#each items as item, idx}
        <input bind:value={item.name} />
        <input type="checkbox" bind:checked={item.done} />
        <span>{idx}: {item.name}</span>
    {/each}

    {#each $metrics as metric, metricIdx}
        <input bind:value={$metrics[metricIdx]} />
        <span class="item-less">Metric {metric}</span>
    {/each}

    {#await promise}
        <p>Loading chunk 15...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value={"opt-" + counter}>{title}</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
    </select>
    <input type="checkbox" bind:group={group} value="opt-15" />
    <input type="radio" bind:group={group} value="opt-15" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly|local={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-15">
        Dynamic element chunk 15: {title}
    </svelte:element>

    <slot name="header" {counter} label={title} />
    {#if $$slots.footer}
        <slot name="footer" {counter} />
    {/if}
    <slot>Default slot fallback chunk 15</slot>

    <ChildComponent
        bind:this={componentRef}
        title={title}
        on:bump={handleClick}
        on:custom={() => (counter += 1)}
        let:item
        let:index={childIndex}
    >
        <strong>Inline child chunk 15: {item} / {childIndex}</strong>
        <svelte:fragment slot="footer" let:value>
            <div>Footer chunk 15: {value} / {counter}</div>
        </svelte:fragment>
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={title} />

    {#if counter > 1000}
        <svelte:self title={title} count={count - 1} />
    {/if}

    <button on:click={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 15: {title}</p>
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-16">
    Chunk 16: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {legacySummary} | Store: {storeSummary} | Keys: {propsKeys}</p>
    <p>Destructured: {rawX}/{rawY} first={firstItem.name}</p>

    {@html "<b>raw html chunk 16</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:reactive={counter}
        style:color={state}
        style:--custom="value-16"
        on:click|preventDefault|stopPropagation={handleClick}
        on:click|once|capture={getHandler()}
        on:keydown|trusted={handleKeydown}
        on:mouseenter={() => (counter += 1)}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 16.
            </span>
        {:else if counter == 100}
            <h1 {state}>Lorem ipsum dolor sit amet. Chunk 16.</h1>
        {:else}
            <h2>EMPTY</h2>
        {/if}
    </div>

    {#key counter}
        <p transition:slide|global>Keyed content chunk 16: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = idx + ":" + item.name}
        <p {...$$restProps} data-index="chunk-16-{idx}" animate:flip={{ duration: 200 }}>
            {itemLabel}
        </p>
    {:else}
        <p>No items in chunk 16</p>
    {/each}

    {#each items as item, idx}
        <input bind:value={item.name} />
        <input type="checkbox" bind:checked={item.done} />
        <span>{idx}: {item.name}</span>
    {/each}

    {#each $metrics as metric, metricIdx}
        <input bind:value={$metrics[metricIdx]} />
        <span class="item-less">Metric {metric}</span>
    {/each}

    {#await promise}
        <p>Loading chunk 16...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value={"opt-" + counter}>{title}</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
    </select>
    <input type="checkbox" bind:group={group} value="opt-16" />
    <input type="radio" bind:group={group} value="opt-16" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly|local={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-16">
        Dynamic element chunk 16: {title}
    </svelte:element>

    <slot name="header" {counter} label={title} />
    {#if $$slots.footer}
        <slot name="footer" {counter} />
    {/if}
    <slot>Default slot fallback chunk 16</slot>

    <ChildComponent
        bind:this={componentRef}
        title={title}
        on:bump={handleClick}
        on:custom={() => (counter += 1)}
        let:item
        let:index={childIndex}
    >
        <strong>Inline child chunk 16: {item} / {childIndex}</strong>
        <svelte:fragment slot="footer" let:value>
            <div>Footer chunk 16: {value} / {counter}</div>
        </svelte:fragment>
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={title} />

    {#if counter > 1000}
        <svelte:self title={title} count={count - 1} />
    {/if}

    <button on:click={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 16: {title}</p>
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-17">
    Chunk 17: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {legacySummary} | Store: {storeSummary} | Keys: {propsKeys}</p>
    <p>Destructured: {rawX}/{rawY} first={firstItem.name}</p>

    {@html "<b>raw html chunk 17</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:reactive={counter}
        style:color={state}
        style:--custom="value-17"
        on:click|preventDefault|stopPropagation={handleClick}
        on:click|once|capture={getHandler()}
        on:keydown|trusted={handleKeydown}
        on:mouseenter={() => (counter += 1)}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 17.
            </span>
        {:else if counter == 100}
            <h1 {state}>Lorem ipsum dolor sit amet. Chunk 17.</h1>
        {:else}
            <h2>EMPTY</h2>
        {/if}
    </div>

    {#key counter}
        <p transition:slide|global>Keyed content chunk 17: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = idx + ":" + item.name}
        <p {...$$restProps} data-index="chunk-17-{idx}" animate:flip={{ duration: 200 }}>
            {itemLabel}
        </p>
    {:else}
        <p>No items in chunk 17</p>
    {/each}

    {#each items as item, idx}
        <input bind:value={item.name} />
        <input type="checkbox" bind:checked={item.done} />
        <span>{idx}: {item.name}</span>
    {/each}

    {#each $metrics as metric, metricIdx}
        <input bind:value={$metrics[metricIdx]} />
        <span class="item-less">Metric {metric}</span>
    {/each}

    {#await promise}
        <p>Loading chunk 17...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value={"opt-" + counter}>{title}</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
    </select>
    <input type="checkbox" bind:group={group} value="opt-17" />
    <input type="radio" bind:group={group} value="opt-17" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly|local={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-17">
        Dynamic element chunk 17: {title}
    </svelte:element>

    <slot name="header" {counter} label={title} />
    {#if $$slots.footer}
        <slot name="footer" {counter} />
    {/if}
    <slot>Default slot fallback chunk 17</slot>

    <ChildComponent
        bind:this={componentRef}
        title={title}
        on:bump={handleClick}
        on:custom={() => (counter += 1)}
        let:item
        let:index={childIndex}
    >
        <strong>Inline child chunk 17: {item} / {childIndex}</strong>
        <svelte:fragment slot="footer" let:value>
            <div>Footer chunk 17: {value} / {counter}</div>
        </svelte:fragment>
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={title} />

    {#if counter > 1000}
        <svelte:self title={title} count={count - 1} />
    {/if}

    <button on:click={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 17: {title}</p>
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-18">
    Chunk 18: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {legacySummary} | Store: {storeSummary} | Keys: {propsKeys}</p>
    <p>Destructured: {rawX}/{rawY} first={firstItem.name}</p>

    {@html "<b>raw html chunk 18</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:reactive={counter}
        style:color={state}
        style:--custom="value-18"
        on:click|preventDefault|stopPropagation={handleClick}
        on:click|once|capture={getHandler()}
        on:keydown|trusted={handleKeydown}
        on:mouseenter={() => (counter += 1)}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 18.
            </span>
        {:else if counter == 100}
            <h1 {state}>Lorem ipsum dolor sit amet. Chunk 18.</h1>
        {:else}
            <h2>EMPTY</h2>
        {/if}
    </div>

    {#key counter}
        <p transition:slide|global>Keyed content chunk 18: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = idx + ":" + item.name}
        <p {...$$restProps} data-index="chunk-18-{idx}" animate:flip={{ duration: 200 }}>
            {itemLabel}
        </p>
    {:else}
        <p>No items in chunk 18</p>
    {/each}

    {#each items as item, idx}
        <input bind:value={item.name} />
        <input type="checkbox" bind:checked={item.done} />
        <span>{idx}: {item.name}</span>
    {/each}

    {#each $metrics as metric, metricIdx}
        <input bind:value={$metrics[metricIdx]} />
        <span class="item-less">Metric {metric}</span>
    {/each}

    {#await promise}
        <p>Loading chunk 18...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value={"opt-" + counter}>{title}</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
    </select>
    <input type="checkbox" bind:group={group} value="opt-18" />
    <input type="radio" bind:group={group} value="opt-18" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly|local={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-18">
        Dynamic element chunk 18: {title}
    </svelte:element>

    <slot name="header" {counter} label={title} />
    {#if $$slots.footer}
        <slot name="footer" {counter} />
    {/if}
    <slot>Default slot fallback chunk 18</slot>

    <ChildComponent
        bind:this={componentRef}
        title={title}
        on:bump={handleClick}
        on:custom={() => (counter += 1)}
        let:item
        let:index={childIndex}
    >
        <strong>Inline child chunk 18: {item} / {childIndex}</strong>
        <svelte:fragment slot="footer" let:value>
            <div>Footer chunk 18: {value} / {counter}</div>
        </svelte:fragment>
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={title} />

    {#if counter > 1000}
        <svelte:self title={title} count={count - 1} />
    {/if}

    <button on:click={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 18: {title}</p>
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-19">
    Chunk 19: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {legacySummary} | Store: {storeSummary} | Keys: {propsKeys}</p>
    <p>Destructured: {rawX}/{rawY} first={firstItem.name}</p>

    {@html "<b>raw html chunk 19</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:reactive={counter}
        style:color={state}
        style:--custom="value-19"
        on:click|preventDefault|stopPropagation={handleClick}
        on:click|once|capture={getHandler()}
        on:keydown|trusted={handleKeydown}
        on:mouseenter={() => (counter += 1)}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 19.
            </span>
        {:else if counter == 100}
            <h1 {state}>Lorem ipsum dolor sit amet. Chunk 19.</h1>
        {:else}
            <h2>EMPTY</h2>
        {/if}
    </div>

    {#key counter}
        <p transition:slide|global>Keyed content chunk 19: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = idx + ":" + item.name}
        <p {...$$restProps} data-index="chunk-19-{idx}" animate:flip={{ duration: 200 }}>
            {itemLabel}
        </p>
    {:else}
        <p>No items in chunk 19</p>
    {/each}

    {#each items as item, idx}
        <input bind:value={item.name} />
        <input type="checkbox" bind:checked={item.done} />
        <span>{idx}: {item.name}</span>
    {/each}

    {#each $metrics as metric, metricIdx}
        <input bind:value={$metrics[metricIdx]} />
        <span class="item-less">Metric {metric}</span>
    {/each}

    {#await promise}
        <p>Loading chunk 19...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value={"opt-" + counter}>{title}</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
    </select>
    <input type="checkbox" bind:group={group} value="opt-19" />
    <input type="radio" bind:group={group} value="opt-19" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly|local={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-19">
        Dynamic element chunk 19: {title}
    </svelte:element>

    <slot name="header" {counter} label={title} />
    {#if $$slots.footer}
        <slot name="footer" {counter} />
    {/if}
    <slot>Default slot fallback chunk 19</slot>

    <ChildComponent
        bind:this={componentRef}
        title={title}
        on:bump={handleClick}
        on:custom={() => (counter += 1)}
        let:item
        let:index={childIndex}
    >
        <strong>Inline child chunk 19: {item} / {childIndex}</strong>
        <svelte:fragment slot="footer" let:value>
            <div>Footer chunk 19: {value} / {counter}</div>
        </svelte:fragment>
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={title} />

    {#if counter > 1000}
        <svelte:self title={title} count={count - 1} />
    {/if}

    <button on:click={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 19: {title}</p>
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-20">
    Chunk 20: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {legacySummary} | Store: {storeSummary} | Keys: {propsKeys}</p>
    <p>Destructured: {rawX}/{rawY} first={firstItem.name}</p>

    {@html "<b>raw html chunk 20</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:reactive={counter}
        style:color={state}
        style:--custom="value-20"
        on:click|preventDefault|stopPropagation={handleClick}
        on:click|once|capture={getHandler()}
        on:keydown|trusted={handleKeydown}
        on:mouseenter={() => (counter += 1)}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 20.
            </span>
        {:else if counter == 100}
            <h1 {state}>Lorem ipsum dolor sit amet. Chunk 20.</h1>
        {:else}
            <h2>EMPTY</h2>
        {/if}
    </div>

    {#key counter}
        <p transition:slide|global>Keyed content chunk 20: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = idx + ":" + item.name}
        <p {...$$restProps} data-index="chunk-20-{idx}" animate:flip={{ duration: 200 }}>
            {itemLabel}
        </p>
    {:else}
        <p>No items in chunk 20</p>
    {/each}

    {#each items as item, idx}
        <input bind:value={item.name} />
        <input type="checkbox" bind:checked={item.done} />
        <span>{idx}: {item.name}</span>
    {/each}

    {#each $metrics as metric, metricIdx}
        <input bind:value={$metrics[metricIdx]} />
        <span class="item-less">Metric {metric}</span>
    {/each}

    {#await promise}
        <p>Loading chunk 20...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value={"opt-" + counter}>{title}</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
    </select>
    <input type="checkbox" bind:group={group} value="opt-20" />
    <input type="radio" bind:group={group} value="opt-20" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly|local={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-20">
        Dynamic element chunk 20: {title}
    </svelte:element>

    <slot name="header" {counter} label={title} />
    {#if $$slots.footer}
        <slot name="footer" {counter} />
    {/if}
    <slot>Default slot fallback chunk 20</slot>

    <ChildComponent
        bind:this={componentRef}
        title={title}
        on:bump={handleClick}
        on:custom={() => (counter += 1)}
        let:item
        let:index={childIndex}
    >
        <strong>Inline child chunk 20: {item} / {childIndex}</strong>
        <svelte:fragment slot="footer" let:value>
            <div>Footer chunk 20: {value} / {counter}</div>
        </svelte:fragment>
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={title} />

    {#if counter > 1000}
        <svelte:self title={title} count={count - 1} />
    {/if}

    <button on:click={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 20: {title}</p>
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-21">
    Chunk 21: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {legacySummary} | Store: {storeSummary} | Keys: {propsKeys}</p>
    <p>Destructured: {rawX}/{rawY} first={firstItem.name}</p>

    {@html "<b>raw html chunk 21</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:reactive={counter}
        style:color={state}
        style:--custom="value-21"
        on:click|preventDefault|stopPropagation={handleClick}
        on:click|once|capture={getHandler()}
        on:keydown|trusted={handleKeydown}
        on:mouseenter={() => (counter += 1)}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 21.
            </span>
        {:else if counter == 100}
            <h1 {state}>Lorem ipsum dolor sit amet. Chunk 21.</h1>
        {:else}
            <h2>EMPTY</h2>
        {/if}
    </div>

    {#key counter}
        <p transition:slide|global>Keyed content chunk 21: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = idx + ":" + item.name}
        <p {...$$restProps} data-index="chunk-21-{idx}" animate:flip={{ duration: 200 }}>
            {itemLabel}
        </p>
    {:else}
        <p>No items in chunk 21</p>
    {/each}

    {#each items as item, idx}
        <input bind:value={item.name} />
        <input type="checkbox" bind:checked={item.done} />
        <span>{idx}: {item.name}</span>
    {/each}

    {#each $metrics as metric, metricIdx}
        <input bind:value={$metrics[metricIdx]} />
        <span class="item-less">Metric {metric}</span>
    {/each}

    {#await promise}
        <p>Loading chunk 21...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value={"opt-" + counter}>{title}</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
    </select>
    <input type="checkbox" bind:group={group} value="opt-21" />
    <input type="radio" bind:group={group} value="opt-21" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly|local={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-21">
        Dynamic element chunk 21: {title}
    </svelte:element>

    <slot name="header" {counter} label={title} />
    {#if $$slots.footer}
        <slot name="footer" {counter} />
    {/if}
    <slot>Default slot fallback chunk 21</slot>

    <ChildComponent
        bind:this={componentRef}
        title={title}
        on:bump={handleClick}
        on:custom={() => (counter += 1)}
        let:item
        let:index={childIndex}
    >
        <strong>Inline child chunk 21: {item} / {childIndex}</strong>
        <svelte:fragment slot="footer" let:value>
            <div>Footer chunk 21: {value} / {counter}</div>
        </svelte:fragment>
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={title} />

    {#if counter > 1000}
        <svelte:self title={title} count={count - 1} />
    {/if}

    <button on:click={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 21: {title}</p>
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-22">
    Chunk 22: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {legacySummary} | Store: {storeSummary} | Keys: {propsKeys}</p>
    <p>Destructured: {rawX}/{rawY} first={firstItem.name}</p>

    {@html "<b>raw html chunk 22</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:reactive={counter}
        style:color={state}
        style:--custom="value-22"
        on:click|preventDefault|stopPropagation={handleClick}
        on:click|once|capture={getHandler()}
        on:keydown|trusted={handleKeydown}
        on:mouseenter={() => (counter += 1)}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 22.
            </span>
        {:else if counter == 100}
            <h1 {state}>Lorem ipsum dolor sit amet. Chunk 22.</h1>
        {:else}
            <h2>EMPTY</h2>
        {/if}
    </div>

    {#key counter}
        <p transition:slide|global>Keyed content chunk 22: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = idx + ":" + item.name}
        <p {...$$restProps} data-index="chunk-22-{idx}" animate:flip={{ duration: 200 }}>
            {itemLabel}
        </p>
    {:else}
        <p>No items in chunk 22</p>
    {/each}

    {#each items as item, idx}
        <input bind:value={item.name} />
        <input type="checkbox" bind:checked={item.done} />
        <span>{idx}: {item.name}</span>
    {/each}

    {#each $metrics as metric, metricIdx}
        <input bind:value={$metrics[metricIdx]} />
        <span class="item-less">Metric {metric}</span>
    {/each}

    {#await promise}
        <p>Loading chunk 22...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value={"opt-" + counter}>{title}</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
    </select>
    <input type="checkbox" bind:group={group} value="opt-22" />
    <input type="radio" bind:group={group} value="opt-22" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly|local={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-22">
        Dynamic element chunk 22: {title}
    </svelte:element>

    <slot name="header" {counter} label={title} />
    {#if $$slots.footer}
        <slot name="footer" {counter} />
    {/if}
    <slot>Default slot fallback chunk 22</slot>

    <ChildComponent
        bind:this={componentRef}
        title={title}
        on:bump={handleClick}
        on:custom={() => (counter += 1)}
        let:item
        let:index={childIndex}
    >
        <strong>Inline child chunk 22: {item} / {childIndex}</strong>
        <svelte:fragment slot="footer" let:value>
            <div>Footer chunk 22: {value} / {counter}</div>
        </svelte:fragment>
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={title} />

    {#if counter > 1000}
        <svelte:self title={title} count={count - 1} />
    {/if}

    <button on:click={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 22: {title}</p>
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-23">
    Chunk 23: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {legacySummary} | Store: {storeSummary} | Keys: {propsKeys}</p>
    <p>Destructured: {rawX}/{rawY} first={firstItem.name}</p>

    {@html "<b>raw html chunk 23</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:reactive={counter}
        style:color={state}
        style:--custom="value-23"
        on:click|preventDefault|stopPropagation={handleClick}
        on:click|once|capture={getHandler()}
        on:keydown|trusted={handleKeydown}
        on:mouseenter={() => (counter += 1)}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 23.
            </span>
        {:else if counter == 100}
            <h1 {state}>Lorem ipsum dolor sit amet. Chunk 23.</h1>
        {:else}
            <h2>EMPTY</h2>
        {/if}
    </div>

    {#key counter}
        <p transition:slide|global>Keyed content chunk 23: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = idx + ":" + item.name}
        <p {...$$restProps} data-index="chunk-23-{idx}" animate:flip={{ duration: 200 }}>
            {itemLabel}
        </p>
    {:else}
        <p>No items in chunk 23</p>
    {/each}

    {#each items as item, idx}
        <input bind:value={item.name} />
        <input type="checkbox" bind:checked={item.done} />
        <span>{idx}: {item.name}</span>
    {/each}

    {#each $metrics as metric, metricIdx}
        <input bind:value={$metrics[metricIdx]} />
        <span class="item-less">Metric {metric}</span>
    {/each}

    {#await promise}
        <p>Loading chunk 23...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value={"opt-" + counter}>{title}</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
    </select>
    <input type="checkbox" bind:group={group} value="opt-23" />
    <input type="radio" bind:group={group} value="opt-23" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly|local={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-23">
        Dynamic element chunk 23: {title}
    </svelte:element>

    <slot name="header" {counter} label={title} />
    {#if $$slots.footer}
        <slot name="footer" {counter} />
    {/if}
    <slot>Default slot fallback chunk 23</slot>

    <ChildComponent
        bind:this={componentRef}
        title={title}
        on:bump={handleClick}
        on:custom={() => (counter += 1)}
        let:item
        let:index={childIndex}
    >
        <strong>Inline child chunk 23: {item} / {childIndex}</strong>
        <svelte:fragment slot="footer" let:value>
            <div>Footer chunk 23: {value} / {counter}</div>
        </svelte:fragment>
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={title} />

    {#if counter > 1000}
        <svelte:self title={title} count={count - 1} />
    {/if}

    <button on:click={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 23: {title}</p>
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-24">
    Chunk 24: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {legacySummary} | Store: {storeSummary} | Keys: {propsKeys}</p>
    <p>Destructured: {rawX}/{rawY} first={firstItem.name}</p>

    {@html "<b>raw html chunk 24</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:reactive={counter}
        style:color={state}
        style:--custom="value-24"
        on:click|preventDefault|stopPropagation={handleClick}
        on:click|once|capture={getHandler()}
        on:keydown|trusted={handleKeydown}
        on:mouseenter={() => (counter += 1)}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 24.
            </span>
        {:else if counter == 100}
            <h1 {state}>Lorem ipsum dolor sit amet. Chunk 24.</h1>
        {:else}
            <h2>EMPTY</h2>
        {/if}
    </div>

    {#key counter}
        <p transition:slide|global>Keyed content chunk 24: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = idx + ":" + item.name}
        <p {...$$restProps} data-index="chunk-24-{idx}" animate:flip={{ duration: 200 }}>
            {itemLabel}
        </p>
    {:else}
        <p>No items in chunk 24</p>
    {/each}

    {#each items as item, idx}
        <input bind:value={item.name} />
        <input type="checkbox" bind:checked={item.done} />
        <span>{idx}: {item.name}</span>
    {/each}

    {#each $metrics as metric, metricIdx}
        <input bind:value={$metrics[metricIdx]} />
        <span class="item-less">Metric {metric}</span>
    {/each}

    {#await promise}
        <p>Loading chunk 24...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value={"opt-" + counter}>{title}</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
    </select>
    <input type="checkbox" bind:group={group} value="opt-24" />
    <input type="radio" bind:group={group} value="opt-24" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly|local={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-24">
        Dynamic element chunk 24: {title}
    </svelte:element>

    <slot name="header" {counter} label={title} />
    {#if $$slots.footer}
        <slot name="footer" {counter} />
    {/if}
    <slot>Default slot fallback chunk 24</slot>

    <ChildComponent
        bind:this={componentRef}
        title={title}
        on:bump={handleClick}
        on:custom={() => (counter += 1)}
        let:item
        let:index={childIndex}
    >
        <strong>Inline child chunk 24: {item} / {childIndex}</strong>
        <svelte:fragment slot="footer" let:value>
            <div>Footer chunk 24: {value} / {counter}</div>
        </svelte:fragment>
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={title} />

    {#if counter > 1000}
        <svelte:self title={title} count={count - 1} />
    {/if}

    <button on:click={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 24: {title}</p>
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-25">
    Chunk 25: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {legacySummary} | Store: {storeSummary} | Keys: {propsKeys}</p>
    <p>Destructured: {rawX}/{rawY} first={firstItem.name}</p>

    {@html "<b>raw html chunk 25</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:reactive={counter}
        style:color={state}
        style:--custom="value-25"
        on:click|preventDefault|stopPropagation={handleClick}
        on:click|once|capture={getHandler()}
        on:keydown|trusted={handleKeydown}
        on:mouseenter={() => (counter += 1)}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 25.
            </span>
        {:else if counter == 100}
            <h1 {state}>Lorem ipsum dolor sit amet. Chunk 25.</h1>
        {:else}
            <h2>EMPTY</h2>
        {/if}
    </div>

    {#key counter}
        <p transition:slide|global>Keyed content chunk 25: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = idx + ":" + item.name}
        <p {...$$restProps} data-index="chunk-25-{idx}" animate:flip={{ duration: 200 }}>
            {itemLabel}
        </p>
    {:else}
        <p>No items in chunk 25</p>
    {/each}

    {#each items as item, idx}
        <input bind:value={item.name} />
        <input type="checkbox" bind:checked={item.done} />
        <span>{idx}: {item.name}</span>
    {/each}

    {#each $metrics as metric, metricIdx}
        <input bind:value={$metrics[metricIdx]} />
        <span class="item-less">Metric {metric}</span>
    {/each}

    {#await promise}
        <p>Loading chunk 25...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value={"opt-" + counter}>{title}</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
    </select>
    <input type="checkbox" bind:group={group} value="opt-25" />
    <input type="radio" bind:group={group} value="opt-25" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly|local={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-25">
        Dynamic element chunk 25: {title}
    </svelte:element>

    <slot name="header" {counter} label={title} />
    {#if $$slots.footer}
        <slot name="footer" {counter} />
    {/if}
    <slot>Default slot fallback chunk 25</slot>

    <ChildComponent
        bind:this={componentRef}
        title={title}
        on:bump={handleClick}
        on:custom={() => (counter += 1)}
        let:item
        let:index={childIndex}
    >
        <strong>Inline child chunk 25: {item} / {childIndex}</strong>
        <svelte:fragment slot="footer" let:value>
            <div>Footer chunk 25: {value} / {counter}</div>
        </svelte:fragment>
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={title} />

    {#if counter > 1000}
        <svelte:self title={title} count={count - 1} />
    {/if}

    <button on:click={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 25: {title}</p>
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-26">
    Chunk 26: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {legacySummary} | Store: {storeSummary} | Keys: {propsKeys}</p>
    <p>Destructured: {rawX}/{rawY} first={firstItem.name}</p>

    {@html "<b>raw html chunk 26</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:reactive={counter}
        style:color={state}
        style:--custom="value-26"
        on:click|preventDefault|stopPropagation={handleClick}
        on:click|once|capture={getHandler()}
        on:keydown|trusted={handleKeydown}
        on:mouseenter={() => (counter += 1)}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 26.
            </span>
        {:else if counter == 100}
            <h1 {state}>Lorem ipsum dolor sit amet. Chunk 26.</h1>
        {:else}
            <h2>EMPTY</h2>
        {/if}
    </div>

    {#key counter}
        <p transition:slide|global>Keyed content chunk 26: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = idx + ":" + item.name}
        <p {...$$restProps} data-index="chunk-26-{idx}" animate:flip={{ duration: 200 }}>
            {itemLabel}
        </p>
    {:else}
        <p>No items in chunk 26</p>
    {/each}

    {#each items as item, idx}
        <input bind:value={item.name} />
        <input type="checkbox" bind:checked={item.done} />
        <span>{idx}: {item.name}</span>
    {/each}

    {#each $metrics as metric, metricIdx}
        <input bind:value={$metrics[metricIdx]} />
        <span class="item-less">Metric {metric}</span>
    {/each}

    {#await promise}
        <p>Loading chunk 26...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value={"opt-" + counter}>{title}</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
    </select>
    <input type="checkbox" bind:group={group} value="opt-26" />
    <input type="radio" bind:group={group} value="opt-26" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly|local={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-26">
        Dynamic element chunk 26: {title}
    </svelte:element>

    <slot name="header" {counter} label={title} />
    {#if $$slots.footer}
        <slot name="footer" {counter} />
    {/if}
    <slot>Default slot fallback chunk 26</slot>

    <ChildComponent
        bind:this={componentRef}
        title={title}
        on:bump={handleClick}
        on:custom={() => (counter += 1)}
        let:item
        let:index={childIndex}
    >
        <strong>Inline child chunk 26: {item} / {childIndex}</strong>
        <svelte:fragment slot="footer" let:value>
            <div>Footer chunk 26: {value} / {counter}</div>
        </svelte:fragment>
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={title} />

    {#if counter > 1000}
        <svelte:self title={title} count={count - 1} />
    {/if}

    <button on:click={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 26: {title}</p>
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-27">
    Chunk 27: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {legacySummary} | Store: {storeSummary} | Keys: {propsKeys}</p>
    <p>Destructured: {rawX}/{rawY} first={firstItem.name}</p>

    {@html "<b>raw html chunk 27</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:reactive={counter}
        style:color={state}
        style:--custom="value-27"
        on:click|preventDefault|stopPropagation={handleClick}
        on:click|once|capture={getHandler()}
        on:keydown|trusted={handleKeydown}
        on:mouseenter={() => (counter += 1)}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 27.
            </span>
        {:else if counter == 100}
            <h1 {state}>Lorem ipsum dolor sit amet. Chunk 27.</h1>
        {:else}
            <h2>EMPTY</h2>
        {/if}
    </div>

    {#key counter}
        <p transition:slide|global>Keyed content chunk 27: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = idx + ":" + item.name}
        <p {...$$restProps} data-index="chunk-27-{idx}" animate:flip={{ duration: 200 }}>
            {itemLabel}
        </p>
    {:else}
        <p>No items in chunk 27</p>
    {/each}

    {#each items as item, idx}
        <input bind:value={item.name} />
        <input type="checkbox" bind:checked={item.done} />
        <span>{idx}: {item.name}</span>
    {/each}

    {#each $metrics as metric, metricIdx}
        <input bind:value={$metrics[metricIdx]} />
        <span class="item-less">Metric {metric}</span>
    {/each}

    {#await promise}
        <p>Loading chunk 27...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value={"opt-" + counter}>{title}</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
    </select>
    <input type="checkbox" bind:group={group} value="opt-27" />
    <input type="radio" bind:group={group} value="opt-27" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly|local={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-27">
        Dynamic element chunk 27: {title}
    </svelte:element>

    <slot name="header" {counter} label={title} />
    {#if $$slots.footer}
        <slot name="footer" {counter} />
    {/if}
    <slot>Default slot fallback chunk 27</slot>

    <ChildComponent
        bind:this={componentRef}
        title={title}
        on:bump={handleClick}
        on:custom={() => (counter += 1)}
        let:item
        let:index={childIndex}
    >
        <strong>Inline child chunk 27: {item} / {childIndex}</strong>
        <svelte:fragment slot="footer" let:value>
            <div>Footer chunk 27: {value} / {counter}</div>
        </svelte:fragment>
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={title} />

    {#if counter > 1000}
        <svelte:self title={title} count={count - 1} />
    {/if}

    <button on:click={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 27: {title}</p>
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-28">
    Chunk 28: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {legacySummary} | Store: {storeSummary} | Keys: {propsKeys}</p>
    <p>Destructured: {rawX}/{rawY} first={firstItem.name}</p>

    {@html "<b>raw html chunk 28</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:reactive={counter}
        style:color={state}
        style:--custom="value-28"
        on:click|preventDefault|stopPropagation={handleClick}
        on:click|once|capture={getHandler()}
        on:keydown|trusted={handleKeydown}
        on:mouseenter={() => (counter += 1)}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 28.
            </span>
        {:else if counter == 100}
            <h1 {state}>Lorem ipsum dolor sit amet. Chunk 28.</h1>
        {:else}
            <h2>EMPTY</h2>
        {/if}
    </div>

    {#key counter}
        <p transition:slide|global>Keyed content chunk 28: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = idx + ":" + item.name}
        <p {...$$restProps} data-index="chunk-28-{idx}" animate:flip={{ duration: 200 }}>
            {itemLabel}
        </p>
    {:else}
        <p>No items in chunk 28</p>
    {/each}

    {#each items as item, idx}
        <input bind:value={item.name} />
        <input type="checkbox" bind:checked={item.done} />
        <span>{idx}: {item.name}</span>
    {/each}

    {#each $metrics as metric, metricIdx}
        <input bind:value={$metrics[metricIdx]} />
        <span class="item-less">Metric {metric}</span>
    {/each}

    {#await promise}
        <p>Loading chunk 28...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value={"opt-" + counter}>{title}</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
    </select>
    <input type="checkbox" bind:group={group} value="opt-28" />
    <input type="radio" bind:group={group} value="opt-28" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly|local={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-28">
        Dynamic element chunk 28: {title}
    </svelte:element>

    <slot name="header" {counter} label={title} />
    {#if $$slots.footer}
        <slot name="footer" {counter} />
    {/if}
    <slot>Default slot fallback chunk 28</slot>

    <ChildComponent
        bind:this={componentRef}
        title={title}
        on:bump={handleClick}
        on:custom={() => (counter += 1)}
        let:item
        let:index={childIndex}
    >
        <strong>Inline child chunk 28: {item} / {childIndex}</strong>
        <svelte:fragment slot="footer" let:value>
            <div>Footer chunk 28: {value} / {counter}</div>
        </svelte:fragment>
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={title} />

    {#if counter > 1000}
        <svelte:self title={title} count={count - 1} />
    {/if}

    <button on:click={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 28: {title}</p>
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-29">
    Chunk 29: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {legacySummary} | Store: {storeSummary} | Keys: {propsKeys}</p>
    <p>Destructured: {rawX}/{rawY} first={firstItem.name}</p>

    {@html "<b>raw html chunk 29</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:reactive={counter}
        style:color={state}
        style:--custom="value-29"
        on:click|preventDefault|stopPropagation={handleClick}
        on:click|once|capture={getHandler()}
        on:keydown|trusted={handleKeydown}
        on:mouseenter={() => (counter += 1)}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 29.
            </span>
        {:else if counter == 100}
            <h1 {state}>Lorem ipsum dolor sit amet. Chunk 29.</h1>
        {:else}
            <h2>EMPTY</h2>
        {/if}
    </div>

    {#key counter}
        <p transition:slide|global>Keyed content chunk 29: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = idx + ":" + item.name}
        <p {...$$restProps} data-index="chunk-29-{idx}" animate:flip={{ duration: 200 }}>
            {itemLabel}
        </p>
    {:else}
        <p>No items in chunk 29</p>
    {/each}

    {#each items as item, idx}
        <input bind:value={item.name} />
        <input type="checkbox" bind:checked={item.done} />
        <span>{idx}: {item.name}</span>
    {/each}

    {#each $metrics as metric, metricIdx}
        <input bind:value={$metrics[metricIdx]} />
        <span class="item-less">Metric {metric}</span>
    {/each}

    {#await promise}
        <p>Loading chunk 29...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value={"opt-" + counter}>{title}</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
    </select>
    <input type="checkbox" bind:group={group} value="opt-29" />
    <input type="radio" bind:group={group} value="opt-29" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly|local={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-29">
        Dynamic element chunk 29: {title}
    </svelte:element>

    <slot name="header" {counter} label={title} />
    {#if $$slots.footer}
        <slot name="footer" {counter} />
    {/if}
    <slot>Default slot fallback chunk 29</slot>

    <ChildComponent
        bind:this={componentRef}
        title={title}
        on:bump={handleClick}
        on:custom={() => (counter += 1)}
        let:item
        let:index={childIndex}
    >
        <strong>Inline child chunk 29: {item} / {childIndex}</strong>
        <svelte:fragment slot="footer" let:value>
            <div>Footer chunk 29: {value} / {counter}</div>
        </svelte:fragment>
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={title} />

    {#if counter > 1000}
        <svelte:self title={title} count={count - 1} />
    {/if}

    <button on:click={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 29: {title}</p>
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-30">
    Chunk 30: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {legacySummary} | Store: {storeSummary} | Keys: {propsKeys}</p>
    <p>Destructured: {rawX}/{rawY} first={firstItem.name}</p>

    {@html "<b>raw html chunk 30</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:reactive={counter}
        style:color={state}
        style:--custom="value-30"
        on:click|preventDefault|stopPropagation={handleClick}
        on:click|once|capture={getHandler()}
        on:keydown|trusted={handleKeydown}
        on:mouseenter={() => (counter += 1)}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 30.
            </span>
        {:else if counter == 100}
            <h1 {state}>Lorem ipsum dolor sit amet. Chunk 30.</h1>
        {:else}
            <h2>EMPTY</h2>
        {/if}
    </div>

    {#key counter}
        <p transition:slide|global>Keyed content chunk 30: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = idx + ":" + item.name}
        <p {...$$restProps} data-index="chunk-30-{idx}" animate:flip={{ duration: 200 }}>
            {itemLabel}
        </p>
    {:else}
        <p>No items in chunk 30</p>
    {/each}

    {#each items as item, idx}
        <input bind:value={item.name} />
        <input type="checkbox" bind:checked={item.done} />
        <span>{idx}: {item.name}</span>
    {/each}

    {#each $metrics as metric, metricIdx}
        <input bind:value={$metrics[metricIdx]} />
        <span class="item-less">Metric {metric}</span>
    {/each}

    {#await promise}
        <p>Loading chunk 30...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value={"opt-" + counter}>{title}</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
    </select>
    <input type="checkbox" bind:group={group} value="opt-30" />
    <input type="radio" bind:group={group} value="opt-30" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly|local={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-30">
        Dynamic element chunk 30: {title}
    </svelte:element>

    <slot name="header" {counter} label={title} />
    {#if $$slots.footer}
        <slot name="footer" {counter} />
    {/if}
    <slot>Default slot fallback chunk 30</slot>

    <ChildComponent
        bind:this={componentRef}
        title={title}
        on:bump={handleClick}
        on:custom={() => (counter += 1)}
        let:item
        let:index={childIndex}
    >
        <strong>Inline child chunk 30: {item} / {childIndex}</strong>
        <svelte:fragment slot="footer" let:value>
            <div>Footer chunk 30: {value} / {counter}</div>
        </svelte:fragment>
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={title} />

    {#if counter > 1000}
        <svelte:self title={title} count={count - 1} />
    {/if}

    <button on:click={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 30: {title}</p>
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-31">
    Chunk 31: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {legacySummary} | Store: {storeSummary} | Keys: {propsKeys}</p>
    <p>Destructured: {rawX}/{rawY} first={firstItem.name}</p>

    {@html "<b>raw html chunk 31</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:reactive={counter}
        style:color={state}
        style:--custom="value-31"
        on:click|preventDefault|stopPropagation={handleClick}
        on:click|once|capture={getHandler()}
        on:keydown|trusted={handleKeydown}
        on:mouseenter={() => (counter += 1)}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 31.
            </span>
        {:else if counter == 100}
            <h1 {state}>Lorem ipsum dolor sit amet. Chunk 31.</h1>
        {:else}
            <h2>EMPTY</h2>
        {/if}
    </div>

    {#key counter}
        <p transition:slide|global>Keyed content chunk 31: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = idx + ":" + item.name}
        <p {...$$restProps} data-index="chunk-31-{idx}" animate:flip={{ duration: 200 }}>
            {itemLabel}
        </p>
    {:else}
        <p>No items in chunk 31</p>
    {/each}

    {#each items as item, idx}
        <input bind:value={item.name} />
        <input type="checkbox" bind:checked={item.done} />
        <span>{idx}: {item.name}</span>
    {/each}

    {#each $metrics as metric, metricIdx}
        <input bind:value={$metrics[metricIdx]} />
        <span class="item-less">Metric {metric}</span>
    {/each}

    {#await promise}
        <p>Loading chunk 31...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value={"opt-" + counter}>{title}</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
    </select>
    <input type="checkbox" bind:group={group} value="opt-31" />
    <input type="radio" bind:group={group} value="opt-31" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly|local={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-31">
        Dynamic element chunk 31: {title}
    </svelte:element>

    <slot name="header" {counter} label={title} />
    {#if $$slots.footer}
        <slot name="footer" {counter} />
    {/if}
    <slot>Default slot fallback chunk 31</slot>

    <ChildComponent
        bind:this={componentRef}
        title={title}
        on:bump={handleClick}
        on:custom={() => (counter += 1)}
        let:item
        let:index={childIndex}
    >
        <strong>Inline child chunk 31: {item} / {childIndex}</strong>
        <svelte:fragment slot="footer" let:value>
            <div>Footer chunk 31: {value} / {counter}</div>
        </svelte:fragment>
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={title} />

    {#if counter > 1000}
        <svelte:self title={title} count={count - 1} />
    {/if}

    <button on:click={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 31: {title}</p>
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-32">
    Chunk 32: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {legacySummary} | Store: {storeSummary} | Keys: {propsKeys}</p>
    <p>Destructured: {rawX}/{rawY} first={firstItem.name}</p>

    {@html "<b>raw html chunk 32</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:reactive={counter}
        style:color={state}
        style:--custom="value-32"
        on:click|preventDefault|stopPropagation={handleClick}
        on:click|once|capture={getHandler()}
        on:keydown|trusted={handleKeydown}
        on:mouseenter={() => (counter += 1)}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 32.
            </span>
        {:else if counter == 100}
            <h1 {state}>Lorem ipsum dolor sit amet. Chunk 32.</h1>
        {:else}
            <h2>EMPTY</h2>
        {/if}
    </div>

    {#key counter}
        <p transition:slide|global>Keyed content chunk 32: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = idx + ":" + item.name}
        <p {...$$restProps} data-index="chunk-32-{idx}" animate:flip={{ duration: 200 }}>
            {itemLabel}
        </p>
    {:else}
        <p>No items in chunk 32</p>
    {/each}

    {#each items as item, idx}
        <input bind:value={item.name} />
        <input type="checkbox" bind:checked={item.done} />
        <span>{idx}: {item.name}</span>
    {/each}

    {#each $metrics as metric, metricIdx}
        <input bind:value={$metrics[metricIdx]} />
        <span class="item-less">Metric {metric}</span>
    {/each}

    {#await promise}
        <p>Loading chunk 32...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value={"opt-" + counter}>{title}</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
    </select>
    <input type="checkbox" bind:group={group} value="opt-32" />
    <input type="radio" bind:group={group} value="opt-32" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly|local={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-32">
        Dynamic element chunk 32: {title}
    </svelte:element>

    <slot name="header" {counter} label={title} />
    {#if $$slots.footer}
        <slot name="footer" {counter} />
    {/if}
    <slot>Default slot fallback chunk 32</slot>

    <ChildComponent
        bind:this={componentRef}
        title={title}
        on:bump={handleClick}
        on:custom={() => (counter += 1)}
        let:item
        let:index={childIndex}
    >
        <strong>Inline child chunk 32: {item} / {childIndex}</strong>
        <svelte:fragment slot="footer" let:value>
            <div>Footer chunk 32: {value} / {counter}</div>
        </svelte:fragment>
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={title} />

    {#if counter > 1000}
        <svelte:self title={title} count={count - 1} />
    {/if}

    <button on:click={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 32: {title}</p>
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-33">
    Chunk 33: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {legacySummary} | Store: {storeSummary} | Keys: {propsKeys}</p>
    <p>Destructured: {rawX}/{rawY} first={firstItem.name}</p>

    {@html "<b>raw html chunk 33</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:reactive={counter}
        style:color={state}
        style:--custom="value-33"
        on:click|preventDefault|stopPropagation={handleClick}
        on:click|once|capture={getHandler()}
        on:keydown|trusted={handleKeydown}
        on:mouseenter={() => (counter += 1)}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 33.
            </span>
        {:else if counter == 100}
            <h1 {state}>Lorem ipsum dolor sit amet. Chunk 33.</h1>
        {:else}
            <h2>EMPTY</h2>
        {/if}
    </div>

    {#key counter}
        <p transition:slide|global>Keyed content chunk 33: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = idx + ":" + item.name}
        <p {...$$restProps} data-index="chunk-33-{idx}" animate:flip={{ duration: 200 }}>
            {itemLabel}
        </p>
    {:else}
        <p>No items in chunk 33</p>
    {/each}

    {#each items as item, idx}
        <input bind:value={item.name} />
        <input type="checkbox" bind:checked={item.done} />
        <span>{idx}: {item.name}</span>
    {/each}

    {#each $metrics as metric, metricIdx}
        <input bind:value={$metrics[metricIdx]} />
        <span class="item-less">Metric {metric}</span>
    {/each}

    {#await promise}
        <p>Loading chunk 33...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value={"opt-" + counter}>{title}</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
    </select>
    <input type="checkbox" bind:group={group} value="opt-33" />
    <input type="radio" bind:group={group} value="opt-33" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly|local={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-33">
        Dynamic element chunk 33: {title}
    </svelte:element>

    <slot name="header" {counter} label={title} />
    {#if $$slots.footer}
        <slot name="footer" {counter} />
    {/if}
    <slot>Default slot fallback chunk 33</slot>

    <ChildComponent
        bind:this={componentRef}
        title={title}
        on:bump={handleClick}
        on:custom={() => (counter += 1)}
        let:item
        let:index={childIndex}
    >
        <strong>Inline child chunk 33: {item} / {childIndex}</strong>
        <svelte:fragment slot="footer" let:value>
            <div>Footer chunk 33: {value} / {counter}</div>
        </svelte:fragment>
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={title} />

    {#if counter > 1000}
        <svelte:self title={title} count={count - 1} />
    {/if}

    <button on:click={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 33: {title}</p>
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-34">
    Chunk 34: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {legacySummary} | Store: {storeSummary} | Keys: {propsKeys}</p>
    <p>Destructured: {rawX}/{rawY} first={firstItem.name}</p>

    {@html "<b>raw html chunk 34</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:reactive={counter}
        style:color={state}
        style:--custom="value-34"
        on:click|preventDefault|stopPropagation={handleClick}
        on:click|once|capture={getHandler()}
        on:keydown|trusted={handleKeydown}
        on:mouseenter={() => (counter += 1)}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 34.
            </span>
        {:else if counter == 100}
            <h1 {state}>Lorem ipsum dolor sit amet. Chunk 34.</h1>
        {:else}
            <h2>EMPTY</h2>
        {/if}
    </div>

    {#key counter}
        <p transition:slide|global>Keyed content chunk 34: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = idx + ":" + item.name}
        <p {...$$restProps} data-index="chunk-34-{idx}" animate:flip={{ duration: 200 }}>
            {itemLabel}
        </p>
    {:else}
        <p>No items in chunk 34</p>
    {/each}

    {#each items as item, idx}
        <input bind:value={item.name} />
        <input type="checkbox" bind:checked={item.done} />
        <span>{idx}: {item.name}</span>
    {/each}

    {#each $metrics as metric, metricIdx}
        <input bind:value={$metrics[metricIdx]} />
        <span class="item-less">Metric {metric}</span>
    {/each}

    {#await promise}
        <p>Loading chunk 34...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value={"opt-" + counter}>{title}</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
    </select>
    <input type="checkbox" bind:group={group} value="opt-34" />
    <input type="radio" bind:group={group} value="opt-34" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly|local={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-34">
        Dynamic element chunk 34: {title}
    </svelte:element>

    <slot name="header" {counter} label={title} />
    {#if $$slots.footer}
        <slot name="footer" {counter} />
    {/if}
    <slot>Default slot fallback chunk 34</slot>

    <ChildComponent
        bind:this={componentRef}
        title={title}
        on:bump={handleClick}
        on:custom={() => (counter += 1)}
        let:item
        let:index={childIndex}
    >
        <strong>Inline child chunk 34: {item} / {childIndex}</strong>
        <svelte:fragment slot="footer" let:value>
            <div>Footer chunk 34: {value} / {counter}</div>
        </svelte:fragment>
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={title} />

    {#if counter > 1000}
        <svelte:self title={title} count={count - 1} />
    {/if}

    <button on:click={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 34: {title}</p>
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-35">
    Chunk 35: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {legacySummary} | Store: {storeSummary} | Keys: {propsKeys}</p>
    <p>Destructured: {rawX}/{rawY} first={firstItem.name}</p>

    {@html "<b>raw html chunk 35</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:reactive={counter}
        style:color={state}
        style:--custom="value-35"
        on:click|preventDefault|stopPropagation={handleClick}
        on:click|once|capture={getHandler()}
        on:keydown|trusted={handleKeydown}
        on:mouseenter={() => (counter += 1)}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 35.
            </span>
        {:else if counter == 100}
            <h1 {state}>Lorem ipsum dolor sit amet. Chunk 35.</h1>
        {:else}
            <h2>EMPTY</h2>
        {/if}
    </div>

    {#key counter}
        <p transition:slide|global>Keyed content chunk 35: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = idx + ":" + item.name}
        <p {...$$restProps} data-index="chunk-35-{idx}" animate:flip={{ duration: 200 }}>
            {itemLabel}
        </p>
    {:else}
        <p>No items in chunk 35</p>
    {/each}

    {#each items as item, idx}
        <input bind:value={item.name} />
        <input type="checkbox" bind:checked={item.done} />
        <span>{idx}: {item.name}</span>
    {/each}

    {#each $metrics as metric, metricIdx}
        <input bind:value={$metrics[metricIdx]} />
        <span class="item-less">Metric {metric}</span>
    {/each}

    {#await promise}
        <p>Loading chunk 35...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value={"opt-" + counter}>{title}</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
    </select>
    <input type="checkbox" bind:group={group} value="opt-35" />
    <input type="radio" bind:group={group} value="opt-35" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly|local={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-35">
        Dynamic element chunk 35: {title}
    </svelte:element>

    <slot name="header" {counter} label={title} />
    {#if $$slots.footer}
        <slot name="footer" {counter} />
    {/if}
    <slot>Default slot fallback chunk 35</slot>

    <ChildComponent
        bind:this={componentRef}
        title={title}
        on:bump={handleClick}
        on:custom={() => (counter += 1)}
        let:item
        let:index={childIndex}
    >
        <strong>Inline child chunk 35: {item} / {childIndex}</strong>
        <svelte:fragment slot="footer" let:value>
            <div>Footer chunk 35: {value} / {counter}</div>
        </svelte:fragment>
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={title} />

    {#if counter > 1000}
        <svelte:self title={title} count={count - 1} />
    {/if}

    <button on:click={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 35: {title}</p>
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-36">
    Chunk 36: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {legacySummary} | Store: {storeSummary} | Keys: {propsKeys}</p>
    <p>Destructured: {rawX}/{rawY} first={firstItem.name}</p>

    {@html "<b>raw html chunk 36</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:reactive={counter}
        style:color={state}
        style:--custom="value-36"
        on:click|preventDefault|stopPropagation={handleClick}
        on:click|once|capture={getHandler()}
        on:keydown|trusted={handleKeydown}
        on:mouseenter={() => (counter += 1)}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 36.
            </span>
        {:else if counter == 100}
            <h1 {state}>Lorem ipsum dolor sit amet. Chunk 36.</h1>
        {:else}
            <h2>EMPTY</h2>
        {/if}
    </div>

    {#key counter}
        <p transition:slide|global>Keyed content chunk 36: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = idx + ":" + item.name}
        <p {...$$restProps} data-index="chunk-36-{idx}" animate:flip={{ duration: 200 }}>
            {itemLabel}
        </p>
    {:else}
        <p>No items in chunk 36</p>
    {/each}

    {#each items as item, idx}
        <input bind:value={item.name} />
        <input type="checkbox" bind:checked={item.done} />
        <span>{idx}: {item.name}</span>
    {/each}

    {#each $metrics as metric, metricIdx}
        <input bind:value={$metrics[metricIdx]} />
        <span class="item-less">Metric {metric}</span>
    {/each}

    {#await promise}
        <p>Loading chunk 36...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value={"opt-" + counter}>{title}</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
    </select>
    <input type="checkbox" bind:group={group} value="opt-36" />
    <input type="radio" bind:group={group} value="opt-36" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly|local={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-36">
        Dynamic element chunk 36: {title}
    </svelte:element>

    <slot name="header" {counter} label={title} />
    {#if $$slots.footer}
        <slot name="footer" {counter} />
    {/if}
    <slot>Default slot fallback chunk 36</slot>

    <ChildComponent
        bind:this={componentRef}
        title={title}
        on:bump={handleClick}
        on:custom={() => (counter += 1)}
        let:item
        let:index={childIndex}
    >
        <strong>Inline child chunk 36: {item} / {childIndex}</strong>
        <svelte:fragment slot="footer" let:value>
            <div>Footer chunk 36: {value} / {counter}</div>
        </svelte:fragment>
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={title} />

    {#if counter > 1000}
        <svelte:self title={title} count={count - 1} />
    {/if}

    <button on:click={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 36: {title}</p>
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-37">
    Chunk 37: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {legacySummary} | Store: {storeSummary} | Keys: {propsKeys}</p>
    <p>Destructured: {rawX}/{rawY} first={firstItem.name}</p>

    {@html "<b>raw html chunk 37</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:reactive={counter}
        style:color={state}
        style:--custom="value-37"
        on:click|preventDefault|stopPropagation={handleClick}
        on:click|once|capture={getHandler()}
        on:keydown|trusted={handleKeydown}
        on:mouseenter={() => (counter += 1)}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 37.
            </span>
        {:else if counter == 100}
            <h1 {state}>Lorem ipsum dolor sit amet. Chunk 37.</h1>
        {:else}
            <h2>EMPTY</h2>
        {/if}
    </div>

    {#key counter}
        <p transition:slide|global>Keyed content chunk 37: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = idx + ":" + item.name}
        <p {...$$restProps} data-index="chunk-37-{idx}" animate:flip={{ duration: 200 }}>
            {itemLabel}
        </p>
    {:else}
        <p>No items in chunk 37</p>
    {/each}

    {#each items as item, idx}
        <input bind:value={item.name} />
        <input type="checkbox" bind:checked={item.done} />
        <span>{idx}: {item.name}</span>
    {/each}

    {#each $metrics as metric, metricIdx}
        <input bind:value={$metrics[metricIdx]} />
        <span class="item-less">Metric {metric}</span>
    {/each}

    {#await promise}
        <p>Loading chunk 37...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value={"opt-" + counter}>{title}</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
    </select>
    <input type="checkbox" bind:group={group} value="opt-37" />
    <input type="radio" bind:group={group} value="opt-37" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly|local={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-37">
        Dynamic element chunk 37: {title}
    </svelte:element>

    <slot name="header" {counter} label={title} />
    {#if $$slots.footer}
        <slot name="footer" {counter} />
    {/if}
    <slot>Default slot fallback chunk 37</slot>

    <ChildComponent
        bind:this={componentRef}
        title={title}
        on:bump={handleClick}
        on:custom={() => (counter += 1)}
        let:item
        let:index={childIndex}
    >
        <strong>Inline child chunk 37: {item} / {childIndex}</strong>
        <svelte:fragment slot="footer" let:value>
            <div>Footer chunk 37: {value} / {counter}</div>
        </svelte:fragment>
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={title} />

    {#if counter > 1000}
        <svelte:self title={title} count={count - 1} />
    {/if}

    <button on:click={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 37: {title}</p>
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-38">
    Chunk 38: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {legacySummary} | Store: {storeSummary} | Keys: {propsKeys}</p>
    <p>Destructured: {rawX}/{rawY} first={firstItem.name}</p>

    {@html "<b>raw html chunk 38</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:reactive={counter}
        style:color={state}
        style:--custom="value-38"
        on:click|preventDefault|stopPropagation={handleClick}
        on:click|once|capture={getHandler()}
        on:keydown|trusted={handleKeydown}
        on:mouseenter={() => (counter += 1)}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 38.
            </span>
        {:else if counter == 100}
            <h1 {state}>Lorem ipsum dolor sit amet. Chunk 38.</h1>
        {:else}
            <h2>EMPTY</h2>
        {/if}
    </div>

    {#key counter}
        <p transition:slide|global>Keyed content chunk 38: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = idx + ":" + item.name}
        <p {...$$restProps} data-index="chunk-38-{idx}" animate:flip={{ duration: 200 }}>
            {itemLabel}
        </p>
    {:else}
        <p>No items in chunk 38</p>
    {/each}

    {#each items as item, idx}
        <input bind:value={item.name} />
        <input type="checkbox" bind:checked={item.done} />
        <span>{idx}: {item.name}</span>
    {/each}

    {#each $metrics as metric, metricIdx}
        <input bind:value={$metrics[metricIdx]} />
        <span class="item-less">Metric {metric}</span>
    {/each}

    {#await promise}
        <p>Loading chunk 38...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value={"opt-" + counter}>{title}</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
    </select>
    <input type="checkbox" bind:group={group} value="opt-38" />
    <input type="radio" bind:group={group} value="opt-38" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly|local={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-38">
        Dynamic element chunk 38: {title}
    </svelte:element>

    <slot name="header" {counter} label={title} />
    {#if $$slots.footer}
        <slot name="footer" {counter} />
    {/if}
    <slot>Default slot fallback chunk 38</slot>

    <ChildComponent
        bind:this={componentRef}
        title={title}
        on:bump={handleClick}
        on:custom={() => (counter += 1)}
        let:item
        let:index={childIndex}
    >
        <strong>Inline child chunk 38: {item} / {childIndex}</strong>
        <svelte:fragment slot="footer" let:value>
            <div>Footer chunk 38: {value} / {counter}</div>
        </svelte:fragment>
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={title} />

    {#if counter > 1000}
        <svelte:self title={title} count={count - 1} />
    {/if}

    <button on:click={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 38: {title}</p>
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-39">
    Chunk 39: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {legacySummary} | Store: {storeSummary} | Keys: {propsKeys}</p>
    <p>Destructured: {rawX}/{rawY} first={firstItem.name}</p>

    {@html "<b>raw html chunk 39</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:reactive={counter}
        style:color={state}
        style:--custom="value-39"
        on:click|preventDefault|stopPropagation={handleClick}
        on:click|once|capture={getHandler()}
        on:keydown|trusted={handleKeydown}
        on:mouseenter={() => (counter += 1)}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 39.
            </span>
        {:else if counter == 100}
            <h1 {state}>Lorem ipsum dolor sit amet. Chunk 39.</h1>
        {:else}
            <h2>EMPTY</h2>
        {/if}
    </div>

    {#key counter}
        <p transition:slide|global>Keyed content chunk 39: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = idx + ":" + item.name}
        <p {...$$restProps} data-index="chunk-39-{idx}" animate:flip={{ duration: 200 }}>
            {itemLabel}
        </p>
    {:else}
        <p>No items in chunk 39</p>
    {/each}

    {#each items as item, idx}
        <input bind:value={item.name} />
        <input type="checkbox" bind:checked={item.done} />
        <span>{idx}: {item.name}</span>
    {/each}

    {#each $metrics as metric, metricIdx}
        <input bind:value={$metrics[metricIdx]} />
        <span class="item-less">Metric {metric}</span>
    {/each}

    {#await promise}
        <p>Loading chunk 39...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value={"opt-" + counter}>{title}</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
    </select>
    <input type="checkbox" bind:group={group} value="opt-39" />
    <input type="radio" bind:group={group} value="opt-39" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly|local={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-39">
        Dynamic element chunk 39: {title}
    </svelte:element>

    <slot name="header" {counter} label={title} />
    {#if $$slots.footer}
        <slot name="footer" {counter} />
    {/if}
    <slot>Default slot fallback chunk 39</slot>

    <ChildComponent
        bind:this={componentRef}
        title={title}
        on:bump={handleClick}
        on:custom={() => (counter += 1)}
        let:item
        let:index={childIndex}
    >
        <strong>Inline child chunk 39: {item} / {childIndex}</strong>
        <svelte:fragment slot="footer" let:value>
            <div>Footer chunk 39: {value} / {counter}</div>
        </svelte:fragment>
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={title} />

    {#if counter > 1000}
        <svelte:self title={title} count={count - 1} />
    {/if}

    <button on:click={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 39: {title}</p>
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-40">
    Chunk 40: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {legacySummary} | Store: {storeSummary} | Keys: {propsKeys}</p>
    <p>Destructured: {rawX}/{rawY} first={firstItem.name}</p>

    {@html "<b>raw html chunk 40</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:reactive={counter}
        style:color={state}
        style:--custom="value-40"
        on:click|preventDefault|stopPropagation={handleClick}
        on:click|once|capture={getHandler()}
        on:keydown|trusted={handleKeydown}
        on:mouseenter={() => (counter += 1)}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 40.
            </span>
        {:else if counter == 100}
            <h1 {state}>Lorem ipsum dolor sit amet. Chunk 40.</h1>
        {:else}
            <h2>EMPTY</h2>
        {/if}
    </div>

    {#key counter}
        <p transition:slide|global>Keyed content chunk 40: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = idx + ":" + item.name}
        <p {...$$restProps} data-index="chunk-40-{idx}" animate:flip={{ duration: 200 }}>
            {itemLabel}
        </p>
    {:else}
        <p>No items in chunk 40</p>
    {/each}

    {#each items as item, idx}
        <input bind:value={item.name} />
        <input type="checkbox" bind:checked={item.done} />
        <span>{idx}: {item.name}</span>
    {/each}

    {#each $metrics as metric, metricIdx}
        <input bind:value={$metrics[metricIdx]} />
        <span class="item-less">Metric {metric}</span>
    {/each}

    {#await promise}
        <p>Loading chunk 40...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value={"opt-" + counter}>{title}</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
    </select>
    <input type="checkbox" bind:group={group} value="opt-40" />
    <input type="radio" bind:group={group} value="opt-40" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly|local={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-40">
        Dynamic element chunk 40: {title}
    </svelte:element>

    <slot name="header" {counter} label={title} />
    {#if $$slots.footer}
        <slot name="footer" {counter} />
    {/if}
    <slot>Default slot fallback chunk 40</slot>

    <ChildComponent
        bind:this={componentRef}
        title={title}
        on:bump={handleClick}
        on:custom={() => (counter += 1)}
        let:item
        let:index={childIndex}
    >
        <strong>Inline child chunk 40: {item} / {childIndex}</strong>
        <svelte:fragment slot="footer" let:value>
            <div>Footer chunk 40: {value} / {counter}</div>
        </svelte:fragment>
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={title} />

    {#if counter > 1000}
        <svelte:self title={title} count={count - 1} />
    {/if}

    <button on:click={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 40: {title}</p>
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-41">
    Chunk 41: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {legacySummary} | Store: {storeSummary} | Keys: {propsKeys}</p>
    <p>Destructured: {rawX}/{rawY} first={firstItem.name}</p>

    {@html "<b>raw html chunk 41</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:reactive={counter}
        style:color={state}
        style:--custom="value-41"
        on:click|preventDefault|stopPropagation={handleClick}
        on:click|once|capture={getHandler()}
        on:keydown|trusted={handleKeydown}
        on:mouseenter={() => (counter += 1)}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 41.
            </span>
        {:else if counter == 100}
            <h1 {state}>Lorem ipsum dolor sit amet. Chunk 41.</h1>
        {:else}
            <h2>EMPTY</h2>
        {/if}
    </div>

    {#key counter}
        <p transition:slide|global>Keyed content chunk 41: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = idx + ":" + item.name}
        <p {...$$restProps} data-index="chunk-41-{idx}" animate:flip={{ duration: 200 }}>
            {itemLabel}
        </p>
    {:else}
        <p>No items in chunk 41</p>
    {/each}

    {#each items as item, idx}
        <input bind:value={item.name} />
        <input type="checkbox" bind:checked={item.done} />
        <span>{idx}: {item.name}</span>
    {/each}

    {#each $metrics as metric, metricIdx}
        <input bind:value={$metrics[metricIdx]} />
        <span class="item-less">Metric {metric}</span>
    {/each}

    {#await promise}
        <p>Loading chunk 41...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value={"opt-" + counter}>{title}</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
    </select>
    <input type="checkbox" bind:group={group} value="opt-41" />
    <input type="radio" bind:group={group} value="opt-41" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly|local={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-41">
        Dynamic element chunk 41: {title}
    </svelte:element>

    <slot name="header" {counter} label={title} />
    {#if $$slots.footer}
        <slot name="footer" {counter} />
    {/if}
    <slot>Default slot fallback chunk 41</slot>

    <ChildComponent
        bind:this={componentRef}
        title={title}
        on:bump={handleClick}
        on:custom={() => (counter += 1)}
        let:item
        let:index={childIndex}
    >
        <strong>Inline child chunk 41: {item} / {childIndex}</strong>
        <svelte:fragment slot="footer" let:value>
            <div>Footer chunk 41: {value} / {counter}</div>
        </svelte:fragment>
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={title} />

    {#if counter > 1000}
        <svelte:self title={title} count={count - 1} />
    {/if}

    <button on:click={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 41: {title}</p>
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-42">
    Chunk 42: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {legacySummary} | Store: {storeSummary} | Keys: {propsKeys}</p>
    <p>Destructured: {rawX}/{rawY} first={firstItem.name}</p>

    {@html "<b>raw html chunk 42</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:reactive={counter}
        style:color={state}
        style:--custom="value-42"
        on:click|preventDefault|stopPropagation={handleClick}
        on:click|once|capture={getHandler()}
        on:keydown|trusted={handleKeydown}
        on:mouseenter={() => (counter += 1)}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 42.
            </span>
        {:else if counter == 100}
            <h1 {state}>Lorem ipsum dolor sit amet. Chunk 42.</h1>
        {:else}
            <h2>EMPTY</h2>
        {/if}
    </div>

    {#key counter}
        <p transition:slide|global>Keyed content chunk 42: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = idx + ":" + item.name}
        <p {...$$restProps} data-index="chunk-42-{idx}" animate:flip={{ duration: 200 }}>
            {itemLabel}
        </p>
    {:else}
        <p>No items in chunk 42</p>
    {/each}

    {#each items as item, idx}
        <input bind:value={item.name} />
        <input type="checkbox" bind:checked={item.done} />
        <span>{idx}: {item.name}</span>
    {/each}

    {#each $metrics as metric, metricIdx}
        <input bind:value={$metrics[metricIdx]} />
        <span class="item-less">Metric {metric}</span>
    {/each}

    {#await promise}
        <p>Loading chunk 42...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value={"opt-" + counter}>{title}</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
    </select>
    <input type="checkbox" bind:group={group} value="opt-42" />
    <input type="radio" bind:group={group} value="opt-42" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly|local={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-42">
        Dynamic element chunk 42: {title}
    </svelte:element>

    <slot name="header" {counter} label={title} />
    {#if $$slots.footer}
        <slot name="footer" {counter} />
    {/if}
    <slot>Default slot fallback chunk 42</slot>

    <ChildComponent
        bind:this={componentRef}
        title={title}
        on:bump={handleClick}
        on:custom={() => (counter += 1)}
        let:item
        let:index={childIndex}
    >
        <strong>Inline child chunk 42: {item} / {childIndex}</strong>
        <svelte:fragment slot="footer" let:value>
            <div>Footer chunk 42: {value} / {counter}</div>
        </svelte:fragment>
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={title} />

    {#if counter > 1000}
        <svelte:self title={title} count={count - 1} />
    {/if}

    <button on:click={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 42: {title}</p>
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-43">
    Chunk 43: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {legacySummary} | Store: {storeSummary} | Keys: {propsKeys}</p>
    <p>Destructured: {rawX}/{rawY} first={firstItem.name}</p>

    {@html "<b>raw html chunk 43</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:reactive={counter}
        style:color={state}
        style:--custom="value-43"
        on:click|preventDefault|stopPropagation={handleClick}
        on:click|once|capture={getHandler()}
        on:keydown|trusted={handleKeydown}
        on:mouseenter={() => (counter += 1)}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 43.
            </span>
        {:else if counter == 100}
            <h1 {state}>Lorem ipsum dolor sit amet. Chunk 43.</h1>
        {:else}
            <h2>EMPTY</h2>
        {/if}
    </div>

    {#key counter}
        <p transition:slide|global>Keyed content chunk 43: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = idx + ":" + item.name}
        <p {...$$restProps} data-index="chunk-43-{idx}" animate:flip={{ duration: 200 }}>
            {itemLabel}
        </p>
    {:else}
        <p>No items in chunk 43</p>
    {/each}

    {#each items as item, idx}
        <input bind:value={item.name} />
        <input type="checkbox" bind:checked={item.done} />
        <span>{idx}: {item.name}</span>
    {/each}

    {#each $metrics as metric, metricIdx}
        <input bind:value={$metrics[metricIdx]} />
        <span class="item-less">Metric {metric}</span>
    {/each}

    {#await promise}
        <p>Loading chunk 43...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value={"opt-" + counter}>{title}</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
    </select>
    <input type="checkbox" bind:group={group} value="opt-43" />
    <input type="radio" bind:group={group} value="opt-43" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly|local={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-43">
        Dynamic element chunk 43: {title}
    </svelte:element>

    <slot name="header" {counter} label={title} />
    {#if $$slots.footer}
        <slot name="footer" {counter} />
    {/if}
    <slot>Default slot fallback chunk 43</slot>

    <ChildComponent
        bind:this={componentRef}
        title={title}
        on:bump={handleClick}
        on:custom={() => (counter += 1)}
        let:item
        let:index={childIndex}
    >
        <strong>Inline child chunk 43: {item} / {childIndex}</strong>
        <svelte:fragment slot="footer" let:value>
            <div>Footer chunk 43: {value} / {counter}</div>
        </svelte:fragment>
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={title} />

    {#if counter > 1000}
        <svelte:self title={title} count={count - 1} />
    {/if}

    <button on:click={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 43: {title}</p>
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-44">
    Chunk 44: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {legacySummary} | Store: {storeSummary} | Keys: {propsKeys}</p>
    <p>Destructured: {rawX}/{rawY} first={firstItem.name}</p>

    {@html "<b>raw html chunk 44</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:reactive={counter}
        style:color={state}
        style:--custom="value-44"
        on:click|preventDefault|stopPropagation={handleClick}
        on:click|once|capture={getHandler()}
        on:keydown|trusted={handleKeydown}
        on:mouseenter={() => (counter += 1)}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 44.
            </span>
        {:else if counter == 100}
            <h1 {state}>Lorem ipsum dolor sit amet. Chunk 44.</h1>
        {:else}
            <h2>EMPTY</h2>
        {/if}
    </div>

    {#key counter}
        <p transition:slide|global>Keyed content chunk 44: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = idx + ":" + item.name}
        <p {...$$restProps} data-index="chunk-44-{idx}" animate:flip={{ duration: 200 }}>
            {itemLabel}
        </p>
    {:else}
        <p>No items in chunk 44</p>
    {/each}

    {#each items as item, idx}
        <input bind:value={item.name} />
        <input type="checkbox" bind:checked={item.done} />
        <span>{idx}: {item.name}</span>
    {/each}

    {#each $metrics as metric, metricIdx}
        <input bind:value={$metrics[metricIdx]} />
        <span class="item-less">Metric {metric}</span>
    {/each}

    {#await promise}
        <p>Loading chunk 44...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value={"opt-" + counter}>{title}</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
    </select>
    <input type="checkbox" bind:group={group} value="opt-44" />
    <input type="radio" bind:group={group} value="opt-44" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly|local={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-44">
        Dynamic element chunk 44: {title}
    </svelte:element>

    <slot name="header" {counter} label={title} />
    {#if $$slots.footer}
        <slot name="footer" {counter} />
    {/if}
    <slot>Default slot fallback chunk 44</slot>

    <ChildComponent
        bind:this={componentRef}
        title={title}
        on:bump={handleClick}
        on:custom={() => (counter += 1)}
        let:item
        let:index={childIndex}
    >
        <strong>Inline child chunk 44: {item} / {childIndex}</strong>
        <svelte:fragment slot="footer" let:value>
            <div>Footer chunk 44: {value} / {counter}</div>
        </svelte:fragment>
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={title} />

    {#if counter > 1000}
        <svelte:self title={title} count={count - 1} />
    {/if}

    <button on:click={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 44: {title}</p>
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-45">
    Chunk 45: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {legacySummary} | Store: {storeSummary} | Keys: {propsKeys}</p>
    <p>Destructured: {rawX}/{rawY} first={firstItem.name}</p>

    {@html "<b>raw html chunk 45</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:reactive={counter}
        style:color={state}
        style:--custom="value-45"
        on:click|preventDefault|stopPropagation={handleClick}
        on:click|once|capture={getHandler()}
        on:keydown|trusted={handleKeydown}
        on:mouseenter={() => (counter += 1)}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 45.
            </span>
        {:else if counter == 100}
            <h1 {state}>Lorem ipsum dolor sit amet. Chunk 45.</h1>
        {:else}
            <h2>EMPTY</h2>
        {/if}
    </div>

    {#key counter}
        <p transition:slide|global>Keyed content chunk 45: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = idx + ":" + item.name}
        <p {...$$restProps} data-index="chunk-45-{idx}" animate:flip={{ duration: 200 }}>
            {itemLabel}
        </p>
    {:else}
        <p>No items in chunk 45</p>
    {/each}

    {#each items as item, idx}
        <input bind:value={item.name} />
        <input type="checkbox" bind:checked={item.done} />
        <span>{idx}: {item.name}</span>
    {/each}

    {#each $metrics as metric, metricIdx}
        <input bind:value={$metrics[metricIdx]} />
        <span class="item-less">Metric {metric}</span>
    {/each}

    {#await promise}
        <p>Loading chunk 45...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value={"opt-" + counter}>{title}</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
    </select>
    <input type="checkbox" bind:group={group} value="opt-45" />
    <input type="radio" bind:group={group} value="opt-45" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly|local={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-45">
        Dynamic element chunk 45: {title}
    </svelte:element>

    <slot name="header" {counter} label={title} />
    {#if $$slots.footer}
        <slot name="footer" {counter} />
    {/if}
    <slot>Default slot fallback chunk 45</slot>

    <ChildComponent
        bind:this={componentRef}
        title={title}
        on:bump={handleClick}
        on:custom={() => (counter += 1)}
        let:item
        let:index={childIndex}
    >
        <strong>Inline child chunk 45: {item} / {childIndex}</strong>
        <svelte:fragment slot="footer" let:value>
            <div>Footer chunk 45: {value} / {counter}</div>
        </svelte:fragment>
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={title} />

    {#if counter > 1000}
        <svelte:self title={title} count={count - 1} />
    {/if}

    <button on:click={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 45: {title}</p>
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-46">
    Chunk 46: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {legacySummary} | Store: {storeSummary} | Keys: {propsKeys}</p>
    <p>Destructured: {rawX}/{rawY} first={firstItem.name}</p>

    {@html "<b>raw html chunk 46</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:reactive={counter}
        style:color={state}
        style:--custom="value-46"
        on:click|preventDefault|stopPropagation={handleClick}
        on:click|once|capture={getHandler()}
        on:keydown|trusted={handleKeydown}
        on:mouseenter={() => (counter += 1)}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 46.
            </span>
        {:else if counter == 100}
            <h1 {state}>Lorem ipsum dolor sit amet. Chunk 46.</h1>
        {:else}
            <h2>EMPTY</h2>
        {/if}
    </div>

    {#key counter}
        <p transition:slide|global>Keyed content chunk 46: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = idx + ":" + item.name}
        <p {...$$restProps} data-index="chunk-46-{idx}" animate:flip={{ duration: 200 }}>
            {itemLabel}
        </p>
    {:else}
        <p>No items in chunk 46</p>
    {/each}

    {#each items as item, idx}
        <input bind:value={item.name} />
        <input type="checkbox" bind:checked={item.done} />
        <span>{idx}: {item.name}</span>
    {/each}

    {#each $metrics as metric, metricIdx}
        <input bind:value={$metrics[metricIdx]} />
        <span class="item-less">Metric {metric}</span>
    {/each}

    {#await promise}
        <p>Loading chunk 46...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value={"opt-" + counter}>{title}</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
    </select>
    <input type="checkbox" bind:group={group} value="opt-46" />
    <input type="radio" bind:group={group} value="opt-46" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly|local={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-46">
        Dynamic element chunk 46: {title}
    </svelte:element>

    <slot name="header" {counter} label={title} />
    {#if $$slots.footer}
        <slot name="footer" {counter} />
    {/if}
    <slot>Default slot fallback chunk 46</slot>

    <ChildComponent
        bind:this={componentRef}
        title={title}
        on:bump={handleClick}
        on:custom={() => (counter += 1)}
        let:item
        let:index={childIndex}
    >
        <strong>Inline child chunk 46: {item} / {childIndex}</strong>
        <svelte:fragment slot="footer" let:value>
            <div>Footer chunk 46: {value} / {counter}</div>
        </svelte:fragment>
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={title} />

    {#if counter > 1000}
        <svelte:self title={title} count={count - 1} />
    {/if}

    <button on:click={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 46: {title}</p>
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-47">
    Chunk 47: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {legacySummary} | Store: {storeSummary} | Keys: {propsKeys}</p>
    <p>Destructured: {rawX}/{rawY} first={firstItem.name}</p>

    {@html "<b>raw html chunk 47</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:reactive={counter}
        style:color={state}
        style:--custom="value-47"
        on:click|preventDefault|stopPropagation={handleClick}
        on:click|once|capture={getHandler()}
        on:keydown|trusted={handleKeydown}
        on:mouseenter={() => (counter += 1)}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 47.
            </span>
        {:else if counter == 100}
            <h1 {state}>Lorem ipsum dolor sit amet. Chunk 47.</h1>
        {:else}
            <h2>EMPTY</h2>
        {/if}
    </div>

    {#key counter}
        <p transition:slide|global>Keyed content chunk 47: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = idx + ":" + item.name}
        <p {...$$restProps} data-index="chunk-47-{idx}" animate:flip={{ duration: 200 }}>
            {itemLabel}
        </p>
    {:else}
        <p>No items in chunk 47</p>
    {/each}

    {#each items as item, idx}
        <input bind:value={item.name} />
        <input type="checkbox" bind:checked={item.done} />
        <span>{idx}: {item.name}</span>
    {/each}

    {#each $metrics as metric, metricIdx}
        <input bind:value={$metrics[metricIdx]} />
        <span class="item-less">Metric {metric}</span>
    {/each}

    {#await promise}
        <p>Loading chunk 47...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value={"opt-" + counter}>{title}</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
    </select>
    <input type="checkbox" bind:group={group} value="opt-47" />
    <input type="radio" bind:group={group} value="opt-47" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly|local={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-47">
        Dynamic element chunk 47: {title}
    </svelte:element>

    <slot name="header" {counter} label={title} />
    {#if $$slots.footer}
        <slot name="footer" {counter} />
    {/if}
    <slot>Default slot fallback chunk 47</slot>

    <ChildComponent
        bind:this={componentRef}
        title={title}
        on:bump={handleClick}
        on:custom={() => (counter += 1)}
        let:item
        let:index={childIndex}
    >
        <strong>Inline child chunk 47: {item} / {childIndex}</strong>
        <svelte:fragment slot="footer" let:value>
            <div>Footer chunk 47: {value} / {counter}</div>
        </svelte:fragment>
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={title} />

    {#if counter > 1000}
        <svelte:self title={title} count={count - 1} />
    {/if}

    <button on:click={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 47: {title}</p>
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-48">
    Chunk 48: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {legacySummary} | Store: {storeSummary} | Keys: {propsKeys}</p>
    <p>Destructured: {rawX}/{rawY} first={firstItem.name}</p>

    {@html "<b>raw html chunk 48</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:reactive={counter}
        style:color={state}
        style:--custom="value-48"
        on:click|preventDefault|stopPropagation={handleClick}
        on:click|once|capture={getHandler()}
        on:keydown|trusted={handleKeydown}
        on:mouseenter={() => (counter += 1)}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 48.
            </span>
        {:else if counter == 100}
            <h1 {state}>Lorem ipsum dolor sit amet. Chunk 48.</h1>
        {:else}
            <h2>EMPTY</h2>
        {/if}
    </div>

    {#key counter}
        <p transition:slide|global>Keyed content chunk 48: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = idx + ":" + item.name}
        <p {...$$restProps} data-index="chunk-48-{idx}" animate:flip={{ duration: 200 }}>
            {itemLabel}
        </p>
    {:else}
        <p>No items in chunk 48</p>
    {/each}

    {#each items as item, idx}
        <input bind:value={item.name} />
        <input type="checkbox" bind:checked={item.done} />
        <span>{idx}: {item.name}</span>
    {/each}

    {#each $metrics as metric, metricIdx}
        <input bind:value={$metrics[metricIdx]} />
        <span class="item-less">Metric {metric}</span>
    {/each}

    {#await promise}
        <p>Loading chunk 48...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value={"opt-" + counter}>{title}</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
    </select>
    <input type="checkbox" bind:group={group} value="opt-48" />
    <input type="radio" bind:group={group} value="opt-48" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly|local={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-48">
        Dynamic element chunk 48: {title}
    </svelte:element>

    <slot name="header" {counter} label={title} />
    {#if $$slots.footer}
        <slot name="footer" {counter} />
    {/if}
    <slot>Default slot fallback chunk 48</slot>

    <ChildComponent
        bind:this={componentRef}
        title={title}
        on:bump={handleClick}
        on:custom={() => (counter += 1)}
        let:item
        let:index={childIndex}
    >
        <strong>Inline child chunk 48: {item} / {childIndex}</strong>
        <svelte:fragment slot="footer" let:value>
            <div>Footer chunk 48: {value} / {counter}</div>
        </svelte:fragment>
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={title} />

    {#if counter > 1000}
        <svelte:self title={title} count={count - 1} />
    {/if}

    <button on:click={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 48: {title}</p>
    </svelte:boundary>
</div>

<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-49">
    Chunk 49: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>
    <p>Module: {legacySummary} | Store: {storeSummary} | Keys: {propsKeys}</p>
    <p>Destructured: {rawX}/{rawY} first={firstItem.name}</p>

    {@html "<b>raw html chunk 49</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:reactive={counter}
        style:color={state}
        style:--custom="value-49"
        on:click|preventDefault|stopPropagation={handleClick}
        on:click|once|capture={getHandler()}
        on:keydown|trusted={handleKeydown}
        on:mouseenter={() => (counter += 1)}
        bind:this={dynamicEl}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 49.
            </span>
        {:else if counter == 100}
            <h1 {state}>Lorem ipsum dolor sit amet. Chunk 49.</h1>
        {:else}
            <h2>EMPTY</h2>
        {/if}
    </div>

    {#key counter}
        <p transition:slide|global>Keyed content chunk 49: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        {@const itemLabel = idx + ":" + item.name}
        <p {...$$restProps} data-index="chunk-49-{idx}" animate:flip={{ duration: 200 }}>
            {itemLabel}
        </p>
    {:else}
        <p>No items in chunk 49</p>
    {/each}

    {#each items as item, idx}
        <input bind:value={item.name} />
        <input type="checkbox" bind:checked={item.done} />
        <span>{idx}: {item.name}</span>
    {/each}

    {#each $metrics as metric, metricIdx}
        <input bind:value={$metrics[metricIdx]} />
        <span class="item-less">Metric {metric}</span>
    {/each}

    {#await promise}
        <p>Loading chunk 49...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <textarea bind:value={state}></textarea>
    <select bind:value={selected}>
        <option value="opt-0">Zero</option>
        <option value={"opt-" + counter}>{title}</option>
    </select>
    <select multiple bind:value={selectedList}>
        <option value="dog">Dog</option>
    </select>
    <input type="checkbox" bind:group={group} value="opt-49" />
    <input type="radio" bind:group={group} value="opt-49" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly|local={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-49">
        Dynamic element chunk 49: {title}
    </svelte:element>

    <slot name="header" {counter} label={title} />
    {#if $$slots.footer}
        <slot name="footer" {counter} />
    {/if}
    <slot>Default slot fallback chunk 49</slot>

    <ChildComponent
        bind:this={componentRef}
        title={title}
        on:bump={handleClick}
        on:custom={() => (counter += 1)}
        let:item
        let:index={childIndex}
    >
        <strong>Inline child chunk 49: {item} / {childIndex}</strong>
        <svelte:fragment slot="footer" let:value>
            <div>Footer chunk 49: {value} / {counter}</div>
        </svelte:fragment>
    </ChildComponent>

    <svelte:component this={checked ? ChildComponent : null} title={title} />

    {#if counter > 1000}
        <svelte:self title={title} count={count - 1} />
    {/if}

    <button on:click={addMetric}>Update store</button>
    <p>Metric count: {$metrics.length}</p>

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 49: {title}</p>
    </svelte:boundary>
</div>

