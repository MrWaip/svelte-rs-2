use std::fmt::Write;

fn main() {
    let n: usize = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(50);

    let mut out = String::with_capacity(n * 3000);

    write_script(&mut out);

    for i in 0..n {
        write_chunk(&mut out, i);
    }

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let path = std::path::Path::new(manifest_dir)
        .join("../benchmark/benches/compiler/big.svelte");
    std::fs::write(&path, &out).expect("failed to write big.svelte");

    let lines = out.lines().count();
    println!("Generated big.svelte: {lines} lines ({n} chunks)");
}

fn write_script(out: &mut String) {
    out.push_str(
        r#"<script>
    import { onMount } from "svelte";

    let {
        title = "Default Title",
        count = 0,
        items = [],
        config = $bindable({}),
        multiplier = 2,
        ...rest
    } = $props();

    let state = $state("");
    let counter = $state(0);

    counter = 10;

    let doubled = $derived(count * multiplier);

    $effect(() => {
        console.log("Title:", title, "Count:", count);
    });
</script>

"#,
    );
}

fn write_chunk(out: &mut String, i: usize) {
    let _ = write!(
        out,
        r#"<div>
    Chunk {i}: Lorem {{state}} + {{state}} = Ipsum;
    <p>Props: title={{title}}, count={{count}}, doubled={{doubled}}</p>
    <div
        class:state
        class:staticly={{true}}
        class:invinsible
        class:reactive={{counter}}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {{#if state}}
            <span title="{{title}}: {{doubled}}" empty {{state}} {{counter}} count={{count}}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk {i}.
            </span>
        {{:else}}
            <div>
                <input {{title}} {{state}} value={{count}} />
            </div>

            {{#if counter > 30}}
                <h1 {{state}}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk {i}.
                </h1>
            {{:else if counter == 100}}
                Lorem ipsum dolor sit amet. Chunk {i}.
            {{:else}}
                <h2>EMPTY</h2>
            {{/if}}
        {{/if}}
    </div>

    {{#each items as item}}
        <p {{...rest}} data-index="chunk-{i}">{{item}}</p>
    {{/each}}

    <input bind:value={{state}} />
</div>

"#,
    );
}
