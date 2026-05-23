<svelte:options runes={false} />
<script>
    export let rows;
    import Row from './Row.svelte';
</script>

{#each rows as row}
    <Row --tone="red">
        <span slot="label">{row.title}</span>
    </Row>
{/each}
