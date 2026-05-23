<svelte:options runes={false} />
<script>
    export let items;
</script>

{#each items as item}
    {#each item.kids as kid}
        <p>{kid}</p>
    {/each}
{/each}
