<script>
    export let rows;
    let refs = {};
</script>

{#each rows as row (row.key)}
    <div bind:this={refs[row.key]}></div>
{/each}
