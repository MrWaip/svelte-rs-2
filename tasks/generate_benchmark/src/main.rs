use std::fmt::Write;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let name = args.get(1).map(|s| s.as_str()).unwrap_or("big_v6");
    let n: usize = args
        .get(2)
        .and_then(|s| s.parse().ok())
        .unwrap_or(50);

    let mut out = String::with_capacity(n * 3000);

    write_script(&mut out);
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
    write_script(&mut example);
    write_svelte_head(&mut example);
    write_special_elements(&mut example);
    write_snippets(&mut example);
    write_chunk(&mut example, 0);

    // Escape backticks and ${} for JS template literal
    let escaped = example
        .replace('\\', "\\\\")
        .replace('`', "\\`")
        .replace("${", "\\${");

    let example_js = format!(
        "{EXAMPLE_HEADER}export const benchmarkExample = `{escaped}`;\n",
    );
    let example_path = std::path::Path::new(manifest_dir)
        .join("../../docs/example.js");
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

fn write_script(out: &mut String) {
    out.push_str(
        r#"<script>
    import { onMount } from "svelte";
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
    let inputEl;
    let componentRef;

    /** @type {Function | undefined} */
    let show;

    counter = 10;

    let doubled = $derived(count * multiplier);
    let computed = $derived.by(() => {
        return items.length * multiplier + counter;
    });
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

"#,
    );
}

fn write_chunk(out: &mut String, i: usize) {
    let _ = write!(
        out,
        r#"<div>
    Chunk {i}: Lorem {{state}} + {{state}} = Ipsum;
    <p>Props: title={{title}}, count={{count}}, doubled={{doubled}}, computed={{computed}}</p>

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
        <p {{...rest}} data-index="chunk-{i}-{{idx}}" animate:flip>{{item.name}}</p>
    {{/each}}

    {{#await promise}}
        <p>Loading chunk {i}...</p>
    {{:then value}}
        <p>Resolved: {{value}}</p>
    {{:catch error}}
        <p>Error: {{error.message}}</p>
    {{/await}}

    <input bind:value={{state}} />
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

    <ChildComponent bind:this={{componentRef}} title={{title}} onclick={{getHandler()}} />

    {{@render badge("chunk-{i}", "secondary")}}
    {{@render card(title, "Content for chunk {i}")}}
    {{@render show?.()}}

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
