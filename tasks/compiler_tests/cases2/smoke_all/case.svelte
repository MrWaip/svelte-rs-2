<script>
    import Panel from "./Panel.svelte";
    import { formatDate } from "./utils.js";
    import { onMount } from "svelte";

    let {
        title,
        theme = "light",
        editable = $bindable(),
        config = $bindable({}),
        multiplier = 2,
        ...extras
    } = $props();

    let count = $state(0);
    let query = $state("");
    let state = $state("");
    let counter = $state(0);
    let items = ["Задачи", "Settings", "🌞 Profile"];

    counter = 10;
    count += 1;

    let doubled = $derived(count * multiplier);

    $effect(() => {
        console.log("Title:", title, "Count:", count);
    });

    export const VERSION = "2.0";
    export const APP_VERSION = "1.0.0";

    export function reset() {
        count = 0;
    }

    export function formatTitle(prefix) {
        return prefix + ": " + title;
    }

    function increment() {
        count++;
    }
</script>

{#snippet row(item)}
    <li>{item} — {count}</li>
{/snippet}

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

<header id="top" data-theme={theme} title="Dashboard: {title}" {...extras}>
    <h1>{title} 🚀</h1>
    <input bind:value={query} />
    <button onclick={increment}>{count}</button>
</header>

{#if count > 0}
    <section>
        <p>Результат: {count} for {query}</p>

        {#each items as item}
            {@render row(item)}

            <div class="entry" data-q="q: {query}">
                {item}
            </div>
        {/each}
    </section>
{:else if editable}
    <Panel label="empty" {count}>
        <p>Nothing here yet</p>

        <Panel label="empty" {count}>
            Title
            <p>Nothing here yet</p>
        </Panel>
    </Panel>
{:else}
    <noscript>Enable JS</noscript>
    <p>{(count = 0)}</p>
{/if}

<Panel {count} label={title} />

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
        <p {...extras} data-index="chunk-0">{item}</p>
    {/each}

    <input bind:value={state} />

    {@render badge("chunk-0", "secondary")}
    {@render card(title, "Content for chunk 0")}
</div>
