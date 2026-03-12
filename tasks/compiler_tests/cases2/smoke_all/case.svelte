<script>
    import Panel from "./Panel.svelte";
    import { formatDate } from "./utils.js";

    let {
        title,
        theme = "light",
        editable = $bindable(),
        ...extras
    } = $props();

    let count = $state(0);
    let query = $state("");
    let items = ["Задачи", "Settings", "🌞 Profile"];

    export const VERSION = "2.0";
    export function reset() {
        count = 0;
    }

    function increment() {
        count++;
    }

    count += 1;
</script>

{#snippet row(item)}
    <li>{item} — {count}</li>
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
