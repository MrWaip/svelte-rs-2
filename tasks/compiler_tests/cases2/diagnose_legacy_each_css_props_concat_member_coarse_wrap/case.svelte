<svelte:options runes={false} />
<script>
    export let rows;
    import Cell from './Cell.svelte';
</script>

{#each rows as row}
    <Cell --tone="prefix-{row.kind}-suffix" />
{/each}
