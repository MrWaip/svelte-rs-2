<script>
    export let rows;
</script>

{#each rows as row (row.key)}
    <p>{@html row.content}</p>
{/each}
