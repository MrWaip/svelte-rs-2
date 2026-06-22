<script>
    export let items;
</script>

{#each items as item, idx (item.id)}
    {@const label = `${idx}:${item.name}`}
    <p>{label}</p>
{/each}
