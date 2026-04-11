<script>
    let items = $state([{ selected: [] }]);
</script>
{#each items as { ...rest }}
    <input type="checkbox" bind:group={rest} value="x" />
{/each}
