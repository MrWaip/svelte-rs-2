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

export const benchmarkExample = `<script>
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

    export const APP_VERSION = "1.0.0";

    export function formatTitle(prefix) {
        return prefix + ": " + title;
    }
</script>

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

<div>
    Chunk 0: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 0.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 0.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 0.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-0">{item}</p>
    {/each}

    <input bind:value={state} />

    {@render badge("chunk-0", "secondary")}
    {@render card(title, "Content for chunk 0")}
</div>
`;
