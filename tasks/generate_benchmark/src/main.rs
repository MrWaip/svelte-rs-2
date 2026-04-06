use std::fmt::Write;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let name = args.get(1).map(|s| s.as_str()).unwrap_or("big_v6");
    let n: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(50);

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
    let path = std::path::Path::new(manifest_dir)
        .join("../benchmark/benches/compiler")
        .join(&filename);
    std::fs::write(&path, &out).expect("failed to write benchmark file");

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
    let example_path = std::path::Path::new(manifest_dir).join("../../docs/example.js");
    std::fs::write(&example_path, &example_js).expect("failed to write docs/example.js");
    println!("Updated docs/example.js");
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
    export const BENCHMARK_KIND = "compiler";
    export const MODULE_SCALE = 3;

    export function moduleLabel(name) {
        return `${BENCHMARK_KIND}:${name}`;
    }
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

    $inspect(counter, doubled);

    export const APP_VERSION = "1.0.0";

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

    .item-less {
        color: #7a4f2a;
    }

    [data-index] {
        color: var(--custom, #5c4634);
    }
</style>

"#,
    );
}

fn write_special_elements(out: &mut String) {
    out.push_str(
        r#"<svelte:window onscroll={handleClick} />
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
    <textarea bind:value={{state}} />
    <select bind:value={{selected}}>
        <option value="opt-0">Zero</option>
        <option value="opt-1">One</option>
    </select>
    <input type="checkbox" bind:checked={{checked}} />
    <input type="radio" bind:group={{group}} value="opt-{i}" />
    <div bind:this={{inputEl}} bind:clientWidth={{counter}} contenteditable bind:innerHTML={{state}}>editable</div>
    <video bind:volume={{volume}} bind:paused={{checked}}></video>

    <div use:action={{state}}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{{{ y: 200 }}}} out:fade>in/out target</div>
    <svelte:element this={{state ? "div" : "span"}} class="dynamic-{i}">
        Dynamic element chunk {i}: {{title}}
    </svelte:element>

    <ChildComponent bind:this={{componentRef}} title={{title}} onclick={{getHandler()}}>
        <strong>Inline child chunk {i}: {{title}}</strong>
        <div slot="footer">Footer chunk {i}: {{counter}}</div>
    </ChildComponent>

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
