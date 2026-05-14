<svelte:options runes={false} />
<script>
    export let rows;
    import Row from './Row.svelte';
</script>

{#each rows as row, index}
    <Row icon={index + 1} title={row.title} />
{/each}
