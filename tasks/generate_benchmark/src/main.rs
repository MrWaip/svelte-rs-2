use std::{env, fmt::Write, fs, path::Path};

fn main() {
    let args: Vec<String> = env::args().collect();
    let legacy = args.iter().any(|arg| arg == "--legacy");
    let positional: Vec<&String> = args
        .iter()
        .skip(1)
        .filter(|arg| !arg.starts_with("--"))
        .collect();

    let default_name = if legacy { "big_legacy_v7" } else { "big_v7" };
    let name = positional
        .first()
        .map(|s| s.as_str())
        .unwrap_or(default_name);
    let n: usize = positional.get(1).and_then(|s| s.parse().ok()).unwrap_or(50);

    if legacy {
        generate_legacy(name, n);
        return;
    }

    let mut out = String::with_capacity(n * 3000);

    write_module_script(&mut out);
    write_script(&mut out);
    write_style(&mut out);
    write_svelte_head(&mut out);
    write_special_elements(&mut out);
    write_snippets(&mut out);

    for i in 0..n {
        write_chunk(&mut out, i);
    }

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let filename = format!("{name}.svelte");
    let path = Path::new(manifest_dir)
        .join("../benchmark/benches/compiler")
        .join(&filename);
    fs::write(&path, &out).expect("failed to write benchmark file");

    let lines = out.lines().count();
    println!("Generated {filename}: {lines} lines ({n} chunks)");

    // Also generate single-chunk example for docs/example.js
    let mut example = String::with_capacity(8000);
    write_module_script(&mut example);
    write_script(&mut example);
    write_style(&mut example);
    write_svelte_head(&mut example);
    write_special_elements(&mut example);
    write_snippets(&mut example);
    write_chunk(&mut example, 0);

    // Escape backticks and ${} for JS template literal
    let escaped = example
        .replace('\\', "\\\\")
        .replace('`', "\\`")
        .replace("${", "\\${");

    let example_js = format!("{EXAMPLE_HEADER}export const benchmarkExample = `{escaped}`;\n",);
    let example_path = Path::new(manifest_dir).join("../../docs/example.js");
    fs::write(&example_path, &example_js).expect("failed to write docs/example.js");
    println!("Updated docs/example.js");
}

fn generate_legacy(name: &str, n: usize) {
    let mut out = String::with_capacity(n * 3000);

    write_legacy_options(&mut out);
    write_legacy_module_script(&mut out);
    write_legacy_script(&mut out);
    write_style(&mut out);
    write_legacy_special_elements(&mut out);

    for i in 0..n {
        write_legacy_chunk(&mut out, i);
    }

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let filename = format!("{name}.svelte");
    let path = Path::new(manifest_dir)
        .join("../benchmark/benches/compiler")
        .join(&filename);
    fs::write(&path, &out).expect("failed to write benchmark file");

    let lines = out.lines().count();
    println!("Generated {filename}: {lines} lines ({n} chunks)");
}

const EXAMPLE_HEADER: &str = r#"export const example = [
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

"#;

fn write_module_script(out: &mut String) {
    out.push_str(
        r#"<script module>
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

"#,
    );
}

fn write_script(out: &mut String) {
    out.push_str(
        r#"<script>
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

"#,
    );
}

fn write_svelte_head(out: &mut String) {
    out.push_str(
        r#"<svelte:head>
    <title>{title} - Benchmark</title>
    <meta name="description" content="Benchmark component">
    <link rel="canonical" href="/benchmark">
</svelte:head>

"#,
    );
}

fn write_style(out: &mut String) {
    out.push_str(
        r#"<style>
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

"#,
    );
}

fn write_special_elements(out: &mut String) {
    out.push_str(
        r#"<svelte:options runes />

<svelte:window onscroll={handleClick} />
<svelte:document onvisibilitychange={handleClick} />
<svelte:body onmouseenter={handleClick} use:action={state} />

"#,
    );
}

fn write_snippets(out: &mut String) {
    out.push_str(
        r#"{#snippet badge(text, variant)}
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

"#,
    );
}

fn write_chunk(out: &mut String, i: usize) {
    let _ = write!(
        out,
        r#"<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-{i}">
    Chunk {i}: Lorem {{state}} + {{state}} = Ipsum;
    <p>Props: title={{title}}, count={{count}}, doubled={{doubled}}, computed={{computed}}</p>
    <p>Module: {{moduleSummary}} | Store: {{storeSummary}} | Label: {{$labelStore}}</p>

    {{@html "<b>raw html chunk {i}</b>"}}
    {{@debug counter, state}}

    <div
        class:state
        class:staticly={{true}}
        class:invinsible
        class:reactive={{counter}}
        class={{{{ active: checked, big: counter > 10 }}}}
        style:color={{state}}
        style:font-size="14px"
        style:opacity={{counter / 100}}
        style:--custom="value-{i}"
        onclick={{handleClick}}
        onscroll={{handleClick}}
        onclickcapture={{handleClick}}
        onfocus={{getHandler()}}
        bind:this={{dynamicEl}}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {{#if state}}
            {{@const localLen = state.length}}
            <span title="{{title}}: {{doubled}}" empty {{state}} {{counter}} count={{count}}>
                Duis aute irure dolor: {{localLen}}. Chunk {i}.
            </span>
        {{:else}}
            <div>
                <input {{title}} {{state}} value={{count}} />
            </div>

            {{#if counter > 30}}
                <h1 {{state}}>
                    Lorem ipsum dolor sit amet. Chunk {i}.
                </h1>
            {{:else if counter == 100}}
                Lorem ipsum dolor sit amet. Chunk {i}.
            {{:else}}
                <h2>EMPTY</h2>
            {{/if}}
        {{/if}}
    </div>

    {{#key counter}}
        <p transition:slide>Keyed content chunk {i}: {{counter}}</p>
    {{/key}}

    {{#each items as item, idx (item.id)}}
        {{@const itemLabel = `${{idx}}:${{item.name}}`}}
        <p {{...rest}} data-index="chunk-{i}-{{idx}}" animate:flip>{{itemLabel}}</p>
    {{/each}}

    {{#each items}}
        <span class="item-less">Repeated shell chunk {i}</span>
    {{/each}}

    {{#each items as item (item.id)}}
        <p animate:flip={{{{ duration: 200 }}}}>{{item.name}}</p>
    {{:else}}
        <p>No items in chunk {i}</p>
    {{/each}}

    {{#await promise}}
        <p>Loading chunk {i}...</p>
    {{:then value}}
        <p>Resolved: {{value}}</p>
    {{:catch error}}
        <p>Error: {{error.message}}</p>
    {{/await}}

    {{#await promise then quickValue}}
        <p>Quick resolved: {{quickValue}}</p>
    {{/await}}

    <input bind:value={{state}} />
    <textarea bind:value={{state}}></textarea>
    <select bind:value={{selected}}>
        <option value="opt-0">Zero</option>
        <option value="opt-1">One</option>
    </select>
    <select multiple bind:value={{selectedList}}>
        <option value="dog">Dog</option>
        <option value={{`opt-${{counter}}`}}>{{title}}</option>
    </select>
    <input type="checkbox" bind:checked={{checked}} />
    <input type="checkbox" bind:group={{group}} value="opt-{i}" />
    <input type="radio" bind:group={{group}} value="opt-{i}" />
    <input type="number" bind:value={{counter}} />
    <details bind:open={{checked}}><summary>More</summary>chunk {i}</details>
    <div bind:this={{inputEl}} bind:clientWidth={{counter}} contenteditable bind:innerHTML={{state}}>editable</div>
    <video bind:volume={{volume}} bind:paused={{checked}}></video>

    <div use:action={{state}}>action target</div>
    <div {{@attach (node) => {{ node.dataset.chunk = "{i}"; }}}}>attach target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{{{ y: 200 }}}} out:fade>in/out target</div>
    <p transition:slide|global>global transition chunk {i}</p>
    <p in:fly|local={{{{ y: 40 }}}}>local transition chunk {i}</p>

    <div style:opacity style:color={{state}}>style shorthand</div>
    <div class="slider chunk-{i} {{title}}" class:invinsible>literal class</div>
    <div class={{`chunk-${{i}}-${{counter}}`}}>template literal class</div>
    <span data-note="&copy;=value" title="&copy 1998" aria-label="a &amp; b">entities</span>

    <template id="chunk-tpl-{i}">
        {{@html "<b>templated</b>"}}
        {{#if counter}}<span>inside template</span>{{/if}}
    </template>

    <svg viewBox="0 0 10 10" class="chunk-svg">
        <clipPath id="clip-{i}"><rect width="4" height="4" /></clipPath>
        <linearGradient id="grad-{i}"><stop offset="0%" stop-color={{state}} /></linearGradient>
        <foreignObject width="10" height="10"><div>svg foreign chunk {i}</div></foreignObject>
    </svg>

    <math><mrow><mi>x</mi><mo>+</mo><mn>{{counter}}</mn></mrow></math>
    <svelte:element this={{state ? "div" : "span"}} class="dynamic-{i}">
        Dynamic element chunk {i}: {{title}}
    </svelte:element>

    <ChildComponent bind:this={{componentRef}} title={{title}} onclick={{getHandler()}}>
        <strong>Inline child chunk {i}: {{title}}</strong>
        <div slot="footer">Footer chunk {i}: {{counter}}</div>
    </ChildComponent>

    <ChildComponent --accent="var(--custom, #5c4634)" {{...rest}} bind:config>
        {{#snippet header(label)}}
            <em>Header snippet chunk {i}: {{label}}</em>
        {{/snippet}}
    </ChildComponent>

    <svelte:component this={{checked ? ChildComponent : null}} title={{pageHeading}} />

    {{#if depth > 0}}
        <svelte:self depth={{depth - 1}} title={{title}} />
    {{/if}}

    {{@render badge("chunk-{i}", "secondary")}}
    {{@render card(title, "Content for chunk {i}")}}
    {{@render metricSummary({{ label: title, values: [count, doubled, counter], meta: {{ id: propsId }} }})}}
    {{@render show?.()}}

    <button onclick={{addMetric}}>Update store</button>
    <p>Metric count: {{$metrics.length}}</p>

    <svelte:boundary onerror={{handleError}}>
        <p>Boundary chunk {i}: {{title}}</p>
        {{#snippet failed(error)}}
            <p>Error in chunk {i}: {{error.message}}</p>
        {{/snippet}}
    </svelte:boundary>
</div>

"#,
    );
}

fn write_legacy_options(out: &mut String) {
    out.push_str(
        r#"<svelte:options runes={false} accessors={true} namespace="html" />

"#,
    );
}

fn write_legacy_module_script(out: &mut String) {
    out.push_str(
        r#"<script context="module">
    import { writable as makeStore } from "svelte/store";

    export const LEGACY_KIND = "compiler";
    export const LEGACY_SCALE = 3;

    export const sharedStore = makeStore(0);

    export function legacyLabel(name) {
        return LEGACY_KIND + ":" + name;
    }
</script>

"#,
    );
}

fn write_legacy_script(out: &mut String) {
    out.push_str(
        r#"<script>
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

"#,
    );
}

fn write_legacy_special_elements(out: &mut String) {
    out.push_str(
        r#"<svelte:head>
    <title>{title} - Legacy Benchmark</title>
    <meta name="description" content="Legacy benchmark component">
</svelte:head>

<svelte:window on:scroll={handleClick} on:keydown|once={handleKeydown} bind:scrollY={counter} />
<svelte:document on:visibilitychange={handleClick} />
<svelte:body on:mouseenter={handleClick} use:action={state} />

"#,
    );
}

fn write_legacy_chunk(out: &mut String, i: usize) {
    let _ = write!(
        out,
        r#"<div class="chunk-shell benchmark-reset benchmark-host" data-kind="chunk-{i}">
    Chunk {i}: Lorem {{state}} + {{state}} = Ipsum;
    <p>Props: title={{title}}, count={{count}}, doubled={{doubled}}, computed={{computed}}</p>
    <p>Module: {{legacySummary}} | Store: {{storeSummary}} | Keys: {{propsKeys}}</p>
    <p>Destructured: {{rawX}}/{{rawY}} first={{firstItem.name}}</p>

    {{@html "<b>raw html chunk {i}</b>"}}
    {{@debug counter, state}}

    <div
        class:state
        class:staticly={{true}}
        class:reactive={{counter}}
        style:color={{state}}
        style:--custom="value-{i}"
        on:click|preventDefault|stopPropagation={{handleClick}}
        on:click|once|capture={{getHandler()}}
        on:keydown|trusted={{handleKeydown}}
        on:mouseenter={{() => (counter += 1)}}
        bind:this={{dynamicEl}}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit.

        {{#if state}}
            {{@const localLen = state.length}}
            <span title="{{title}}: {{doubled}}" empty {{state}} count={{count}}>
                Duis aute irure dolor: {{localLen}}. Chunk {i}.
            </span>
        {{:else if counter == 100}}
            <h1 {{state}}>Lorem ipsum dolor sit amet. Chunk {i}.</h1>
        {{:else}}
            <h2>EMPTY</h2>
        {{/if}}
    </div>

    {{#key counter}}
        <p transition:slide|global>Keyed content chunk {i}: {{counter}}</p>
    {{/key}}

    {{#each items as item, idx (item.id)}}
        {{@const itemLabel = idx + ":" + item.name}}
        <p {{...$$restProps}} data-index="chunk-{i}-{{idx}}" animate:flip={{{{ duration: 200 }}}}>
            {{itemLabel}}
        </p>
    {{:else}}
        <p>No items in chunk {i}</p>
    {{/each}}

    {{#each items as item, idx}}
        <input bind:value={{item.name}} />
        <input type="checkbox" bind:checked={{item.done}} />
        <span>{{idx}}: {{item.name}}</span>
    {{/each}}

    {{#each $metrics as metric, metricIdx}}
        <input bind:value={{$metrics[metricIdx]}} />
        <span class="item-less">Metric {{metric}}</span>
    {{/each}}

    {{#await promise}}
        <p>Loading chunk {i}...</p>
    {{:then value}}
        <p>Resolved: {{value}}</p>
    {{:catch error}}
        <p>Error: {{error.message}}</p>
    {{/await}}

    <input bind:value={{state}} />
    <textarea bind:value={{state}}></textarea>
    <select bind:value={{selected}}>
        <option value="opt-0">Zero</option>
        <option value={{"opt-" + counter}}>{{title}}</option>
    </select>
    <select multiple bind:value={{selectedList}}>
        <option value="dog">Dog</option>
    </select>
    <input type="checkbox" bind:group={{group}} value="opt-{i}" />
    <input type="radio" bind:group={{group}} value="opt-{i}" />
    <div bind:this={{inputEl}} bind:clientWidth={{counter}} contenteditable bind:innerHTML={{state}}>editable</div>

    <div use:action={{state}}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly|local={{{{ y: 200 }}}} out:fade>in/out target</div>
    <svelte:element this={{state ? "div" : "span"}} class="dynamic-{i}">
        Dynamic element chunk {i}: {{title}}
    </svelte:element>

    <slot name="header" {{counter}} label={{title}} />
    {{#if $$slots.footer}}
        <slot name="footer" {{counter}} />
    {{/if}}
    <slot>Default slot fallback chunk {i}</slot>

    <ChildComponent
        bind:this={{componentRef}}
        title={{title}}
        on:bump={{handleClick}}
        on:custom={{() => (counter += 1)}}
        let:item
        let:index={{childIndex}}
    >
        <strong>Inline child chunk {i}: {{item}} / {{childIndex}}</strong>
        <svelte:fragment slot="footer" let:value>
            <div>Footer chunk {i}: {{value}} / {{counter}}</div>
        </svelte:fragment>
    </ChildComponent>

    <svelte:component this={{checked ? ChildComponent : null}} title={{title}} />

    {{#if counter > 1000}}
        <svelte:self title={{title}} count={{count - 1}} />
    {{/if}}

    <button on:click={{addMetric}}>Update store</button>
    <p>Metric count: {{$metrics.length}}</p>

    <svelte:boundary onerror={{handleError}}>
        <p>Boundary chunk {i}: {{title}}</p>
    </svelte:boundary>
</div>

"#,
    );
}
