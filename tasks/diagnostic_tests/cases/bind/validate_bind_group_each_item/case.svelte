<script>
    let items = $state(['a', 'b']);
</script>
{#each items as item}
    <input type="checkbox" bind:group={item} />
{/each}
