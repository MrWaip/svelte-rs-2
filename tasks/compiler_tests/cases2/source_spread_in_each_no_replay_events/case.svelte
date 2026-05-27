<script>
    let { sources } = $props();
</script>

<picture>
    {#each sources as source}
        <source {...source} />
    {/each}
</picture>
