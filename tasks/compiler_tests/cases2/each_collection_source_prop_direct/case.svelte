<script>
    let { items = [] } = $props();
</script>

{#each items as item}
    <span>{item}</span>
{/each}
