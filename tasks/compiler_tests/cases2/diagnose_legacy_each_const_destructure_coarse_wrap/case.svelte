<script>
    export let rows;
    const lookup = { a: { x: 1, y: 2 }, b: { x: 3, y: 4 } };
</script>

{#each rows as row (row.key)}
    {@const { x, y } = lookup[row.key]}
    <p>{x}:{y}</p>
{/each}
