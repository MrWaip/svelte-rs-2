<svelte:options runes={false} />
<script>
    export let item;
</script>

{#each item.list as entry}
    {entry}
{/each}
